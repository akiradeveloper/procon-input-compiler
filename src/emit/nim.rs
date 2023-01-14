use super::*;

pub struct Nim;
impl Lang for Nim {
    fn read_line(bind: Bind) -> (Code, Index) {
        let mut code = vec![];
        code.push(format!("let {bind} = readLine(stdin).split(' ')"));
        let n = new_var();
        code.push(format!("let {n} = len({bind})"));
        (code, Index(n.0))
    }
    fn unit_type(bind: Bind, ast: &ast::UnitType, source: Slice) -> Code {
        let Slice(xs, range) = source;
        let i = range.0;
        let mut code = vec![];
        match ast {
            ast::UnitType::Int => {
                code.push(format!("let {bind} = {xs}[{i}].parseInt"));
            }
            ast::UnitType::Int0 => {
                code.push(format!("let {bind} = ({xs}[{i}].parseInt - 1)"));
            }
            ast::UnitType::Float => {
                code.push(format!("let {bind} = ({xs}[{i}].parseFloat)"));
            }
            ast::UnitType::Str => {
                code.push(format!("let {bind} = {xs}[{i}]"));
            }
        }
        code
    }
    fn array(bind: Bind, ast: &ast::Array, source: Slice) -> Code {
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        let mut code = vec![];
        let mapper = unit_type_mapper(&ast.0);
        code.push(format!("let {bind} = {xs}[{i}..<{j}].map({mapper})"));
        code
    }
    fn list(bind: Bind, ast: &ast::List, source: Slice) -> Code {
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        let mut code = vec![];
        let mapper = unit_type_mapper(&ast.0);
        code.push(format!("let {bind} = {xs}[{i}..<{j}].map({mapper})"));
        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Code {
        let mut code = vec![];
        let inner_ty = typing::tuple_like(&ast.0);
        let len = &ast.1 .0;
        code.push(format!("var {bind}: seq[{inner_ty}];"));
        code.push(format!("for i in 0..<{len}:"));

        let mut inner_code = vec![];
        let line = new_var();
        inner_code.push(format!("let {line} = readLine(stdin).split(' ')"));
        let t = new_var();
        let n = new_var();
        inner_code.push(format!("let {n} = len({line})"));
        let mut e = Self::tuple_like(
            t.clone(),
            &ast.0,
            Slice(line, Range(Index::zero(), Index(format!("{n}")))),
        );
        inner_code.append(&mut e);
        inner_code.push(format!("{bind}.add({t})"));

        append_code(&mut code, "    ", inner_code);
        code
    }
    fn tuple(bind: Bind, elems: Vec<(&ast::TupleElem, Bind)>) -> Code {
        let mut code = vec![];
        let mut inner = vec![];
        for (_, var) in elems {
            inner.push(var.0);
        }
        let inner = inner.join(",");
        code.push(format!("let {bind} = ({inner})"));
        code
    }
}
type Type = String;
mod typing {
    use super::*;
    fn unit_type(ty: &ast::UnitType) -> Type {
        match ty {
            ast::UnitType::Int => "int".to_string(),
            ast::UnitType::Int0 => "int".to_string(),
            ast::UnitType::Float => "float".to_string(),
            ast::UnitType::Str => "string".to_string(),
        }
    }
    fn array(ty: &ast::Array) -> Type {
        let inner = &ty.0;
        let inner = unit_type(inner);
        format!("seq[{inner}]")
    }
    fn list(ty: &ast::List) -> Type {
        let inner = &ty.0;
        let inner = unit_type(inner);
        format!("seq[{inner}]")
    }
    fn tuple(ty: &ast::Tuple) -> Type {
        let mut inner = vec![];
        for e in &ty.0 {
            let ty = match e {
                TupleElem::Array(x) => array(x),
                TupleElem::List(x) => list(x),
                TupleElem::UnitType(x) => unit_type(x),
            };
            inner.push(ty);
        }
        let inner = inner.join(", ");
        format!("({inner})")
    }
    pub fn tuple_like(ty: &ast::TupleLike) -> Type {
        match ty {
            ast::TupleLike::Array(x) => array(x),
            ast::TupleLike::List(x) => list(x),
            ast::TupleLike::Tuple(x) => tuple(x),
        }
    }
}
fn unit_type_mapper(ty: &ast::UnitType) -> &str {
    match ty {
        ast::UnitType::Int => "proc (x: string): int = x.parseInt",
        ast::UnitType::Int0 => "proc (x: string): int = (x.parseInt - 1)",
        ast::UnitType::Float => "proc (x: string): float = x.parseFloat",
        ast::UnitType::Str => "proc (x: string): string = x",
    }
}
