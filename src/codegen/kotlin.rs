use super::*;

pub struct Kotlin;
impl Lang for Kotlin {
    fn read_line(bind: Bind) -> (Code, Index) {
        let mut code = vec![];
        code.push(format!("val {bind} = readLine()!!.split(' ');"));
        let n = new_var();
        code.push(format!("val {n} = {bind}.size;"));
        (code, Index(n.0))
    }
    fn unit_type(bind: Bind, ast: &ast::UnitType, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let v = format!("{xs}[{i}]");
        code.append(&mut bind_unit_type(bind, &ast, &v));
        code
    }
    fn array(bind: Bind, ast: &ast::Array, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        let ty = typing::array(&ast);
        code.push(format!("val {bind} = {ty}();"));
        let k = new_var();
        code.push(format!("for ({k} in {i} until {j}) {{"));
        let x = new_var();
        let inner_code = bind_unit_type(x, &ast.0, &format!("{xs}[{k}]"));
        append_code(&mut code, "\t", inner_code);
        code.push(format!("}}"));
        code
    }
    fn list(bind: Bind, ast: &ast::List, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        let ty = typing::list(&ast);
        code.push(format!("val {bind} = {ty}();"));
        let k = new_var();
        code.push(format!("for ({k} in {i} until {j}) {{"));
        let x = new_var();
        let inner_code = bind_unit_type(x, &ast.0, &format!("{xs}[{k}]"));
        append_code(&mut code, "\t", inner_code);
        code.push(format!("}}"));
        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, Error> {
        let mut code = vec![];
        let ty = format!("ArrayList<{}>", typing::tuple_like(&ast.0)?);
        let n = &ast.1;
        let n = Index(n.0.clone());
        code.push(format!("val {bind} = {ty}();"));
        let k = new_var();
        code.push(format!("for ({k} in 0 until {n}) {{"));

        let mut inner_code = vec![];

        let line = new_var();
        let (mut read_line, m) = Self::read_line(line.clone());
        inner_code.append(&mut read_line);

        let tuple = new_var();
        let slice = Slice(line, Range(Index::zero(), m));
        inner_code.append(&mut Self::tuple_like(tuple.clone(), &ast.0, slice)?);
        inner_code.push(format!("{bind}.add({tuple});"));

        append_code(&mut code, "\t", inner_code);
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
        code.push(format!("val {bind} = {};", e.1));
        Ok(code)
    }
}
fn bind_unit_type(bind: Bind, ast: &ast::UnitType, s: &str) -> Code {
    let mut code = vec![];
    let ty = typing::unit_type(&ast);
    match ast {
        ast::UnitType::Int => {
            code.push(format!("val {bind}: {ty} = {s}.toInt();"));
        }
        ast::UnitType::Int0 => {
            code.push(format!("val {bind}: {ty} = ({s}.toInt() - 1);"));
        }
        ast::UnitType::Float => {
            code.push(format!("val {bind}: {ty} = {s}.toDouble();"));
        }
        ast::UnitType::Str => {
            code.push(format!("val {bind}: {ty} = {s};"));
        }
    }
    code
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
