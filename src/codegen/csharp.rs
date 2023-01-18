use super::*;

pub struct CSharp;

impl Lang for CSharp {
    fn read_line(bind: Bind) -> (Code, Index) {
        let mut code = vec![];
        code.push(format!("var {bind} = Console.ReadLine().Split(' ');"));
        let n = new_var();
        code.push(format!("var {n} = {bind}.Length;"));
        (code, Index(n.0))
    }
    fn unit_type(bind: Bind, ast: &ast::UnitType, source: Slice) -> Code {
        let Slice(xs, range) = source;
        let i = range.0;
        let v = format!("{xs}[{i}]");
        let rhs = format!("{}", unit_type_convert(ast, &v));
        let ty = typing::unit_type(&ast);
        let code = format!("{ty} {bind} = {rhs};");
        vec![code]
    }
    fn array(bind: Bind, ast: &ast::Array, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        let ty1 = typing::array(&ast);
        let ty2 = typing::unit_type(&ast.0);

        code.push(format!("var {bind} = new {ty1}();"));
        let k = new_var();
        code.push(format!("for (int {k}={i}; {k}<{j}; {k}++) {{"));
        let v = format!("{xs}[{k}]");
        let x = new_var();
        code.push(format!("\t{ty2} {x} = {};", unit_type_convert(&ast.0, &v)));
        code.push(format!("\t{bind}.Add({x});"));
        code.push(format!("}}"));

        code
    }
    fn list(bind: Bind, ast: &ast::List, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        let ty1 = typing::list(&ast);
        let ty2 = typing::unit_type(&ast.0);

        code.push(format!("var {bind} = new {ty1}();"));
        let k = new_var();
        code.push(format!("for (int {k}={i}; {k}<{j}; {k}++) {{"));
        let v = format!("{xs}[{k}]");
        let x = new_var();
        code.push(format!("\t{ty2} {x} = {};", unit_type_convert(&ast.0, &v)));
        code.push(format!("\t{bind}.Add({x});"));
        code.push(format!("}}"));

        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, Error> {
        let mut code = vec![];
        let ty = format!("List<{}>", typing::tuple_like(&ast.0));
        let n = &ast.1;
        let n = Index(n.0.clone());
        code.push(format!("var {bind} = new {ty}();"));
        let k = new_var();
        code.push(format!("for (int {k}=0; {k}<{n}; {k}++) {{"));

        let line = new_var();
        let (read_line, m) = Self::read_line(line.clone());
        append_code(&mut code, "\t", read_line);

        let tuple = new_var();
        let slice = Slice(line, Range(Index::zero(), m));
        let inner_code = Self::tuple_like(tuple.clone(), &ast.0, slice)?;
        append_code(&mut code, "\t", inner_code);
        code.push(format!("\t{bind}.Add({tuple});"));

        code.push(format!("}}"));
        Ok(code)
    }
    fn tuple(bind: Bind, elems: Vec<(&ast::TupleElem, Bind)>) -> Result<Code, Error> {
        let mut inner = vec![];
        for (_, e) in elems {
            inner.push(e.0);
        }
        let inner = inner.join(",");
        let code = format!("var {bind} = ({inner});");
        Ok(vec![code])
    }
}
fn unit_type_convert(ty: &ast::UnitType, v: &str) -> String {
    match ty {
        ast::UnitType::Int => {
            format!("Convert.ToInt32({v})")
        }
        ast::UnitType::Int0 => {
            format!("(Convert.ToInt32({v})-1)")
        }
        ast::UnitType::Float => {
            format!("Convert.ToDouble({v})")
        }
        ast::UnitType::Str => v.to_string(),
    }
}

type Type = String;
mod typing {
    use super::*;
    pub fn unit_type(ty: &ast::UnitType) -> Type {
        match ty {
            ast::UnitType::Int => "int".to_string(),
            ast::UnitType::Int0 => "int".to_string(),
            ast::UnitType::Float => "double".to_string(),
            ast::UnitType::Str => "string".to_string(),
        }
    }
    pub fn array(ty: &ast::Array) -> Type {
        let inner = unit_type(&ty.0);
        format!("List<{inner}>")
    }
    pub fn list(ty: &ast::List) -> Type {
        let inner = unit_type(&ty.0);
        format!("List<{inner}>")
    }
    pub fn tuple_like(ty: &ast::TupleLike) -> Type {
        match ty {
            ast::TupleLike::Array(x) => array(x),
            ast::TupleLike::List(x) => list(x),
            ast::TupleLike::Tuple(x) => tuple(x),
        }
    }
    pub fn tuple(ty: &ast::Tuple) -> Type {
        let mut inner = vec![];
        let n = ty.0.len();
        for e in &ty.0 {
            let ty = match e {
                TupleElem::Array(x) => array(x),
                TupleElem::List(x) => list(x),
                TupleElem::UnitType(x) => unit_type(x),
            };
            inner.push(ty);
        }
        let inner = inner.join(",");
        if n == 1 {
            format!("{inner}")
        } else {
            format!("ValueTuple<{inner}>")
        }
    }
}
