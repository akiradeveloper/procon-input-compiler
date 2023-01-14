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
    fn unit_type(bind: Bind, ast: ast::UnitType, source: Slice) -> Code {
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
    fn array(bind: Bind, ast: ast::Array, source: Slice) -> Code {
        Self::list(bind, ast::List(ast.0), source)
    }
    fn list(bind: Bind, ast: ast::List, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        let ty1 = typing::list(&ast);
        let ty2 = typing::unit_type(&ast.0);
        code.push(format!("{ty1} {bind};"));
        code.push(format!("for (int i={i}; i<{j}; i++) {{ {ty2} s; std::istringstream ss({xs}[i]); ss >> s; {bind}.push_back(s); }}"));
        code
    }
    fn matrix(bind: Bind, ast: ast::Matrix) -> Code {
        let mut code = vec![];
        let ty = format!("std::vector<{}>", typing::tuple_like(&ast.0));
        let n = ast.1;
        let n = Index(n.0);
        code.push(format!("{ty} {bind};"));
        code.push(format!("for (int i=0; i<{n}; i++) {{"));

        let line = new_var();
        let (mut read_line, m) = Self::read_line(line.clone());
        code.append(&mut read_line);

        let tuple = new_var();
        let slice = Slice(line, Range(Index::zero(), m));
        let mut inner_code = Self::tuple_like(tuple.clone(), ast.0, slice);
        code.append(&mut inner_code);
        code.push(format!("{bind}.push_back({tuple});"));

        code.push(format!("}}"));
        code
    }
    fn tuple(bind: Bind, elems: Vec<Bind>) -> Code {
        let mut code = vec![];
        let mut inner = vec![];
        for e in elems {
            inner.push(e.0);
        }
        let inner = inner.join(", ");
        code.push(format!("auto {bind} = std::make_tuple({inner});"));
        code
    }
}

type Type = String;
mod typing {
    use super::*;
    pub fn unit_type(ty: &ast::UnitType) -> Type {
        match ty {
            ast::UnitType::Int => "int".to_string(),
            ast::UnitType::Int0 => "int".to_string(),
            ast::UnitType::Float => "float".to_string(),
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
        for e in &ty.0 {
            let ty = match e {
                TupleElem::Array(x) => array(x),
                TupleElem::List(x) => list(x),
                TupleElem::UnitType(x) => unit_type(x),
            };
            inner.push(ty);
        }
        let inner = inner.join(",");
        format!("std::tuple<{inner}>")
    }
}