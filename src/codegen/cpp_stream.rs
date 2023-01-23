use super::cpp::typing;
use super::*;

pub struct CppStream;
impl stream::Lang for CppStream {
    fn unit_type(bind: Bind, ast: &ast::UnitType) -> Code {
        let mut code = vec![];
        code.append(&mut scan_unit_type(bind, &ast));
        code
    }
    fn array(bind: Bind, ast: &ast::Array) -> Code {
        let mut code = vec![];
        let ty = typing::array(&ast);
        let n = Index(ast.1 .0.clone());
        code.push(format!("{ty} {bind};"));
        code.push(format!("{bind}.reserve({n});"));

        let k = new_var();
        code.push(format!("for (int {k}=0; {k}<{n}; ++{k}) {{"));

        let mut inner_code = vec![];
        let unit_val = new_var();
        inner_code.append(&mut scan_unit_type(unit_val.clone(), &ast.0));
        inner_code.push(format!("{bind}.push_back({unit_val});"));
        append_code(&mut code, "\t", inner_code);

        code.push(format!("}}"));
        code
    }
    fn list(bind: Bind, ast: &ast::List) -> Code {
        let mut code = vec![];
        let ty = typing::list(&ast);

        let n = Index(ast.1 .0.clone());
        code.push(format!("int {n}; std::cin >> {n};"));

        code.push(format!("{ty} {bind};"));
        code.push(format!("{bind}.reserve({n});"));

        let k = new_var();
        code.push(format!("for (int {k}=0; {k}<{n}; ++{k}) {{"));

        let mut inner_code = vec![];
        let unit_val = new_var();
        inner_code.append(&mut scan_unit_type(unit_val.clone(), &ast.0));
        inner_code.push(format!("{bind}.push_back({unit_val});"));
        append_code(&mut code, "\t", inner_code);

        code.push(format!("}}"));
        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, Error> {
        let mut code = vec![];
        let ty = format!("std::vector<{}>", typing::tuple_like(&ast.0));
        let n = Index(ast.1 .0.clone());
        code.push(format!("{ty} {bind};"));
        code.push(format!("{bind}.reserve({n});"));
        let k = new_var();
        code.push(format!("for (int {k}=0; {k}<{n}; ++{k}) {{"));

        let tuple = new_var();
        let inner_code = Self::tuple_like(tuple.clone(), &ast.0)?;
        append_code(&mut code, "\t", inner_code);
        code.push(format!("\t{bind}.push_back({tuple});"));

        code.push(format!("}}"));
        Ok(code)
    }
    fn tuple(bind: Bind, elems: Vec<(&ast::TupleElem, Bind)>) -> Result<Code, Error> {
        let mut code = vec![];
        let mut inner = vec![];
        let n = elems.len();
        for (_, e) in elems {
            inner.push(e.0);
        }
        let inner = inner.join(", ");
        if n == 1 {
            code.push(format!("auto {bind} = {inner};"));
        } else {
            code.push(format!("auto {bind} = std::make_tuple({inner});"));
        }
        Ok(code)
    }
}

fn scan_unit_type(bind: Bind, ast: &ast::UnitType) -> Code {
    let mut code = vec![];
    let ty = typing::unit_type(&ast);
    code.push(format!("{ty} {bind};"));
    match ast {
        ast::UnitType::Int => {
            code.push(format!("std::cin >> {bind};"));
        }
        ast::UnitType::Int0 => {
            code.push(format!("std::cin >> {bind};"));
            code.push(format!("{bind}--;"));
        }
        ast::UnitType::Float => {
            code.push(format!("std::cin >> {bind};"));
        }
        ast::UnitType::Str => {
            code.push(format!("std::cin >> {bind};"));
        }
    }
    code
}
