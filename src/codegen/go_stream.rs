use super::*;

pub struct GoStream;
impl stream::Lang for GoStream {
    fn unit_type(bind: Bind, ast: &ast::UnitType) -> Code {
        let mut code = vec![];
        code.append(&mut scan_unit_type(bind, &ast));
        code
    }
    fn array(bind: Bind, ast: &ast::Array) -> Code {
        let mut code = vec![];
        let inner_ty = typing::unit_type(&ast.0);
        let n = Index(ast.1 .0.clone());
        code.push(format!("{bind} := make([]{inner_ty}, 0, {n})"));
        code.push(format!("for i := 0; i < {n}; i++ {{"));

        let mut inner_code = vec![];
        let v = new_var();
        inner_code.append(&mut scan_unit_type(v.clone(), &ast.0));
        inner_code.push(format!("{bind} = append({bind}, {v})"));

        append_code(&mut code, "\t", inner_code);
        code.push(format!("}}"));
        code
    }
    fn list(bind: Bind, ast: &ast::List) -> Code {
        let mut code = vec![];
        let inner_ty = typing::unit_type(&ast.0);

        let n = Bind(ast.1 .0.to_owned());
        code.push(format!("input.Scan()"));
        code.push(format!("{n}, _ := strconv.Atoi(input.Text())"));

        code.push(format!("{bind} := make([]{inner_ty}, 0, {n})"));
        code.push(format!("for i := 0; i < {n}; i++ {{"));

        let mut inner_code = vec![];
        let v = new_var();
        inner_code.append(&mut scan_unit_type(v.clone(), &ast.0));
        inner_code.push(format!("{bind} = append({bind}, {v})"));

        append_code(&mut code, "\t", inner_code);
        code.push(format!("}}"));
        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, Error> {
        let mut code = vec![];
        let n = Index(ast.1 .0.clone());
        let ty = format!("[]{}", typing::tuple_like(&ast.0)?);
        code.push(format!("{bind} := make({ty}, 0, {n})"));
        let k = new_var();
        code.push(format!("for {k} := 0; {k}<{n}; {k}++ {{"));

        let mut inner_code = vec![];
        let tuple = new_var();
        inner_code.append(&mut Self::tuple_like(tuple.clone(), &ast.0)?);
        inner_code.push(format!("{bind} = append({bind}, {tuple})"));

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
        code.push(format!("{bind} := {}", e.1));
        Ok(code)
    }
}
fn scan_unit_type(bind: Bind, ty: &ast::UnitType) -> Code {
    let mut code = vec![];
    code.push(format!("input.Scan()"));
    code.push(match ty {
        ast::UnitType::Int => {
            format!("{bind}, _ := strconv.Atoi(input.Text())")
        }
        ast::UnitType::Int0 => {
            format!("{bind}, _ := strconv.Atoi(input.Text()); {bind}--")
        }
        ast::UnitType::Float => {
            format!("{bind}, _ := strconv.ParseFloat(input.Text(), 64)")
        }
        ast::UnitType::Str => {
            format!("{bind} := input.Text()")
        }
    });
    code
}
type Type = String;
pub mod typing {
    use super::*;
    pub fn unit_type(ty: &ast::UnitType) -> Type {
        match ty {
            ast::UnitType::Int => "int".to_string(),
            ast::UnitType::Int0 => "int".to_string(),
            ast::UnitType::Float => "float64".to_string(),
            ast::UnitType::Str => "string".to_string(),
        }
    }
    pub fn array(ty: &ast::Array) -> Type {
        let inner = unit_type(&ty.0);
        format!("[]{inner}")
    }
    pub fn list(ty: &ast::List) -> Type {
        let inner = unit_type(&ty.0);
        format!("[]{inner}")
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
