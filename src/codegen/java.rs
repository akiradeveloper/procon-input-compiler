use super::*;

pub struct Java;
impl readline::Lang for Java {
    fn read_line(bind: Bind) -> (Code, Index) {
        let mut code = vec![];
        code.push(format!("var {bind} = input.readLine().split(\" \");"));
        let n = new_var();
        code.push(format!("var {n} = {bind}.length;"));
        (code, Index(n.0))
    }
    fn unit_type(bind: Bind, ast: &ast::UnitType, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let v = format!("{xs}[{i}]");
        code.push(format!("var {bind} = {};", unit_type_convert(&ast, &v)));
        code
    }
    fn array(bind: Bind, ast: &ast::Array, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        let ty = typing::array(&ast);
        code.push(format!("var {bind} = new {ty}();"));
        let k = new_var();
        code.push(format!("for (int {k}={i}; {k}<{j}; {k}++) {{"));
        let x = new_var();
        let v = format!("{xs}[{k}]");
        let v = unit_type_convert(&ast.0, &v);
        code.push(format!("\tvar {x} = {v}; {bind}.add({x});"));
        code.push(format!("}}"));
        code
    }
    fn list(bind: Bind, ast: &ast::List, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;

        let n = Bind(ast.1 .0.to_owned());
        code.push(format!("var {n} = Integer.parseInt({xs}[{i}]);"));

        let ty = typing::list(&ast);
        code.push(format!("var {bind} = new {ty}();"));
        let k = new_var();
        code.push(format!("for (int {k}={i}+1; {k}<{j}; {k}++) {{"));
        let x = new_var();
        let v = format!("{xs}[{k}]");
        let v = unit_type_convert(&ast.0, &v);
        code.push(format!("\tvar {x} = {v}; {bind}.add({x});"));
        code.push(format!("}}"));
        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, Error> {
        let mut code = vec![];
        let ty = format!("ArrayList<{}>", typing::tuple_like(&ast.0)?);
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
        code.push(format!("\t{bind}.add({tuple});"));

        code.push(format!("}}"));
        Ok(code)
    }
    fn tuple(bind: Bind, mut elems: Vec<(&ast::TupleElem, Bind)>) -> Result<Code, Error> {
        let n = elems.len();
        if n > 1 {
            return Err(Error::TupleNotSupported);
        }
        let e = elems.pop().unwrap();
        let mut code = vec![];
        code.push(format!("var {bind} = {};", e.1));
        Ok(code)
    }
}
type Type = String;
pub mod typing {
    use super::*;
    pub fn unit_type(ty: &ast::UnitType) -> Type {
        match ty {
            ast::UnitType::Int => "Integer".to_string(),
            ast::UnitType::Int0 => "Integer".to_string(),
            ast::UnitType::Float => "Double".to_string(),
            ast::UnitType::Str => "String".to_string(),
        }
    }
    pub fn array(ty: &ast::Array) -> Type {
        let inner = unit_type(&ty.0);
        format!("ArrayList<{inner}>")
    }
    pub fn list(ty: &ast::List) -> Type {
        let inner = unit_type(&ty.0);
        format!("ArrayList<{inner}>")
    }
    pub fn tuple_like(ty: &ast::TupleLike) -> Result<Type, Error> {
        match ty {
            ast::TupleLike::Array(x) => Ok(array(x)),
            ast::TupleLike::List(x) => Ok(list(x)),
            ast::TupleLike::Tuple(x) => tuple(x),
        }
    }
    pub fn tuple(ty: &ast::Tuple) -> Result<Type, Error> {
        let n = ty.0.len();
        if n > 1 {
            return Err(Error::TupleNotSupported);
        }
        let ty = &ty.0[0];
        let ty = match ty {
            TupleElem::Array(x) => array(x),
            TupleElem::List(x) => list(x),
            TupleElem::UnitType(x) => unit_type(x),
        };
        Ok(ty)
    }
}
fn unit_type_convert(ty: &ast::UnitType, v: &str) -> String {
    match ty {
        ast::UnitType::Int => {
            format!("Integer.parseInt({v})")
        }
        ast::UnitType::Float => {
            format!("Double.parseDouble({v})")
        }
        ast::UnitType::Int0 => {
            format!("(Integer.parseInt({v})-1)")
        }
        ast::UnitType::Str => v.to_string(),
    }
}
