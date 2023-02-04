use super::*;

pub struct Swift;
impl readline::Lang for Swift {
    fn read_line(bind: Bind) -> (Code, Index) {
        let mut code = vec![];
        code.push(format!("let {bind} = readLine()!.split(separator: \" \")"));
        let n = new_var();
        code.push(format!("let {n} = {bind}.count"));
        (code, Index(n.0))
    }
    fn unit_type(bind: Bind, ast: &ast::UnitType, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let v = format!("{xs}[{i}]");
        code.push(format!("let {bind} = {}", unit_type_convert(ast, &v)));
        code
    }
    fn array(bind: Bind, ast: &ast::Array, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        code.push(format!("var {bind}: {} = []", typing::array(ast)));
        code.push(format!("for i in {i} ..< {j} {{"));
        let v = format!("{xs}[i]");
        code.push(format!(
            "\t{bind}.append({});",
            unit_type_convert(&ast.0, &v)
        ));
        code.push(format!("}}"));
        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, Error> {
        let mut code = vec![];
        let inner_ty = typing::tuple_like(&ast.0);
        let n = &ast.1;
        let n = Index(n.0.clone());
        code.push(format!("var {bind}: [{inner_ty}] = []"));
        code.push(format!("for _ in 0..<{n} {{"));

        let mut inner_code = vec![];
        let line = new_var();
        inner_code.push(format!("let {line} = readLine()!.split(separator: \" \")"));
        let t = new_var();
        let m = new_var();
        inner_code.push(format!("let {m} = {line}.count"));
        let mut e = Self::tuple_like(
            t.clone(),
            &ast.0,
            Slice(line, Range(Index::zero(), Index(format!("{m}")))),
        )?;
        inner_code.append(&mut e);
        inner_code.push(format!("{bind}.append({t})"));

        append_code(&mut code, "    ", inner_code);
        code.push(format!("}}"));

        Ok(code)
    }
    fn tuple(bind: Bind, elems: Vec<(&ast::TupleElem, Bind)>) -> Result<Code, Error> {
        let mut inner = vec![];
        for (_, e) in elems {
            inner.push(e.0);
        }
        let inner = inner.join(",");
        let code = format!("let {bind} = ({inner});");
        Ok(vec![code])
    }
}
fn unit_type_convert(ty: &ast::UnitType, v: &str) -> String {
    match ty {
        ast::UnitType::Int => {
            format!("Int({v})!")
        }
        ast::UnitType::Int0 => {
            format!("(Int({v})! - 1)")
        }
        ast::UnitType::Float => {
            format!("Double({v})!")
        }
        ast::UnitType::Str => {
            format!("String({v})")
        }
    }
}
type Type = String;
mod typing {
    use super::*;
    pub fn unit_type(ty: &ast::UnitType) -> Type {
        match ty {
            ast::UnitType::Int => "Int".to_string(),
            ast::UnitType::Int0 => "Int".to_string(),
            ast::UnitType::Float => "Double".to_string(),
            ast::UnitType::Str => "String".to_string(),
        }
    }
    pub fn array(ty: &ast::Array) -> Type {
        let inner = &ty.0;
        let inner = unit_type(inner);
        format!("[{inner}]")
    }
    pub fn list(ty: &ast::List) -> Type {
        let inner = &ty.0;
        let inner = unit_type(inner);
        format!("[{inner}]")
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
        let inner = inner.join(", ");
        if n == 1 {
            format!("{inner}")
        } else {
            format!("({inner})")
        }
    }
    pub fn tuple_like(ty: &ast::TupleLike) -> Type {
        match ty {
            ast::TupleLike::Array(x) => array(x),
            ast::TupleLike::List(x) => list(x),
            ast::TupleLike::Tuple(x) => tuple(x),
        }
    }
}
