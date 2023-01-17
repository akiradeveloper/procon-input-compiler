use super::*;

pub struct Cpp11;

impl Lang for Cpp11 {
    fn read_line(bind: Bind) -> (Code, Index) {
        let mut code = vec![];
        let line = new_var();
        let s = new_var();
        let ss = new_var();
        let n = new_var();
        code.push(format!("std::vector<std::string> {bind};"));
        code.push(format!(
            "std::string {line}; std::getline(std::cin, {line});"
        ));
        code.push(format!("std::istringstream {ss}({line}); std::string {s};"));
        code.push(format!(
            "while (std::getline({ss}, {s}, ' ')) {{ {bind}.push_back({s}); }}"
        ));
        code.push(format!("int {n} = {bind}.size();"));
        let size = Index(n.0);
        (code, size)
    }
    fn unit_type(bind: Bind, ast: &ast::UnitType, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let ss = new_var();
        let ty = typing::unit_type(&ast);
        code.push(format!("{ty} {bind};"));
        code.push(format!("std::istringstream {ss}({xs}[{i}]);"));
        code.push(format!("{ss} >> {bind};"));
        match ast {
            ast::UnitType::Int0 => {
                code.push(format!("{bind}--;"));
            }
            _ => {}
        }
        code
    }
    fn array(bind: Bind, ast: &ast::Array, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        let ty1 = typing::array(&ast);
        let ty2 = typing::unit_type(&ast.0);
        code.push(format!("{ty1} {bind};"));
        let k = new_var();
        code.push(format!("for (int {k}={i}; {k}<{j}; {k}++) {{"));
        code.push(format!(
            "\t{ty2} s; std::istringstream ss({xs}[{k}]); ss >> s; {bind}.push_back(s);"
        ));
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
        code.push(format!("{ty1} {bind};"));
        let k = new_var();
        code.push(format!("for (int {k}={i}; {k}<{j}; {k}++) {{"));
        code.push(format!(
            "\t{ty2} s; std::istringstream ss({xs}[{k}]); ss >> s; {bind}.push_back(s);"
        ));
        code.push(format!("}}"));
        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, super::Error> {
        let mut code = vec![];
        let ty = format!("std::vector<{}>", typing::tuple_like(&ast.0));
        let n = &ast.1;
        let n = Index(n.0.clone());
        code.push(format!("{ty} {bind};"));
        let k = new_var();
        code.push(format!("for (int {k}=0; {k}<{n}; {k}++) {{"));

        let line = new_var();
        let (read_line, m) = Self::read_line(line.clone());
        append_code(&mut code, "\t", read_line);

        let tuple = new_var();
        let slice = Slice(line, Range(Index::zero(), m));
        let inner_code = Self::tuple_like(tuple.clone(), &ast.0, slice)?;
        append_code(&mut code, "\t", inner_code);
        code.push(format!("\t{bind}.push_back({tuple});"));

        code.push(format!("}}"));
        Ok(code)
    }
    fn tuple(bind: Bind, elems: Vec<(&ast::TupleElem, Bind)>) -> Result<Code, super::Error> {
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

type Type = String;
mod typing {
    use super::*;
    pub fn unit_type(ty: &ast::UnitType) -> Type {
        match ty {
            ast::UnitType::Int => "int".to_string(),
            ast::UnitType::Int0 => "int".to_string(),
            ast::UnitType::Float => "double".to_string(),
            ast::UnitType::Str => "std::string".to_string(),
        }
    }
    pub fn array(ty: &ast::Array) -> Type {
        let inner = unit_type(&ty.0);
        format!("std::vector<{inner}>")
    }
    pub fn list(ty: &ast::List) -> Type {
        let inner = unit_type(&ty.0);
        format!("std::vector<{inner}>")
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
            format!("std::tuple<{inner}>")
        }
    }
}
