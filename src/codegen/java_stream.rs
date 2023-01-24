use super::*;

use super::java::typing;

pub struct JavaStream;
impl stream::Lang for JavaStream {
    fn unit_type(bind: Bind, ast: &ast::UnitType) -> Code {
        let mut code = vec![];
        code.append(&mut scan_unit_type(bind, &ast));
        code
    }
    fn array(bind: Bind, ast: &ast::Array) -> Code {
        let mut code = vec![];
        let n = Index(ast.1 .0.clone());
        let ty = typing::array(&ast);
        code.push(format!("var {bind} = new {ty}();"));
        let k = new_var();
        code.push(format!("for (int {k}=0; {k}<{n}; {k}++) {{"));

        let mut inner_code = vec![];
        let x = new_var();
        inner_code.append(&mut scan_unit_type(x.clone(), &ast.0));
        inner_code.push(format!("{bind}.add({x});"));
        append_code(&mut code, "\t", inner_code);

        code.push(format!("}}"));
        code
    }
    fn list(bind: Bind, ast: &ast::List) -> Code {
        let mut code = vec![];

        let n = Bind(ast.1 .0.to_owned());
        code.push(format!("var {n} = input.nextInt();"));

        let ty = typing::list(&ast);
        code.push(format!("var {bind} = new {ty}();"));
        let k = new_var();
        code.push(format!("for (int {k}=0; {k}<{n}; {k}++) {{"));

        let mut inner_code = vec![];
        let x = new_var();
        inner_code.append(&mut scan_unit_type(x.clone(), &ast.0));
        inner_code.push(format!("{bind}.add({x});"));
        append_code(&mut code, "\t", inner_code);

        code.push(format!("}}"));
        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, Error> {
        let mut code = vec![];
        let n = Index(ast.1 .0.clone());
        let ty = format!("ArrayList<{}>", typing::tuple_like(&ast.0)?);
        code.push(format!("var {bind} = new {ty}();"));
        let k = new_var();
        code.push(format!("for (int {k}=0; {k}<{n}; {k}++) {{"));

        let mut inner_code = vec![];
        let tuple = new_var();
        inner_code.append(&mut Self::tuple_like(tuple.clone(), &ast.0)?);
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
        code.push(format!("var {bind} = {};", e.1));
        Ok(code)
    }
}
fn scan_unit_type(bind: Bind, ty: &ast::UnitType) -> Code {
    let code = match ty {
        ast::UnitType::Int => {
            format!("var {bind} = input.nextInt();")
        }
        ast::UnitType::Float => {
            format!("var {bind} = input.nextDouble();")
        }
        ast::UnitType::Int0 => {
            format!("var {bind} = (input.nextInt()-1);")
        }
        ast::UnitType::Str => {
            format!("var {bind} = input.next();")
        }
    };
    vec![code]
}
