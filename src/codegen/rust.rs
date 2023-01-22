use super::*;
pub struct Rust;
impl Lang for Rust {
    fn read_line(bind: Bind) -> (Code, Index) {
        let mut code = vec![];
        let buf = new_var();
        code.push(format!("let mut {buf} = String::new();"));
        code.push(format!("input.read_line(&mut {buf}).unwrap();"));
        code.push(format!(
            "let {bind}: Vec<&str> = {buf}.trim().split(' ').collect();"
        ));
        let n = new_var();
        code.push(format!("let {n} = {bind}.len();"));
        (code, Index(n.0))
    }
    fn unit_type(bind: Bind, ast: &ast::UnitType, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let v = format!("{xs}[{i}]");
        code.push(format!("let {bind} = {};", unit_type_convert(&ast, &v)));
        code
    }
    fn array(bind: Bind, ast: &ast::Array, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        code.push(format!("let mut {bind} = vec![];"));
        code.push(format!("for i in {i} as usize..{j} as usize {{"));
        let v = format!("{xs}[i]");
        code.push(format!("\t{bind}.push({});", unit_type_convert(&ast.0, &v)));
        code.push(format!("}}"));
        code
    }
    fn list(bind: Bind, ast: &ast::List, source: Slice) -> Code {
        let mut code = vec![];
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;

        let n = Bind(ast.1 .0.to_owned());
        code.push(format!("let {n} = ({xs}[{i}]).parse::<i32>().unwrap();"));

        code.push(format!("let mut {bind} = vec![];"));
        code.push(format!("for i in (({i}+1) as usize)..(({j}) as usize) {{"));
        let v = format!("{xs}[i]");
        code.push(format!("\t{bind}.push({});", unit_type_convert(&ast.0, &v)));
        code.push(format!("}}"));
        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, Error> {
        let mut code = vec![];
        code.push(format!("let mut {bind} = vec![];"));
        let n = &ast.1;
        let n = Index(n.0.clone());
        code.push(format!("for i in 0..(({n}) as usize) {{"));

        let line = new_var();
        let (read_line, m) = Self::read_line(line.clone());
        append_code(&mut code, "\t", read_line);

        let tuple = new_var();
        let slice = Slice(line, Range(Index::zero(), m));
        let inner_code = Self::tuple_like(tuple.clone(), &ast.0, slice)?;
        append_code(&mut code, "\t", inner_code);
        code.push(format!("\t{bind}.push({tuple});"));

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
// In Rust, the default types for integer number is i32 and floating number is f64.
// https://github.com/rust-lang/rfcs/blob/master/text/0212-restore-int-fallback.md
fn unit_type_convert(ty: &ast::UnitType, v: &str) -> String {
    match ty {
        ast::UnitType::Int => {
            format!("{v}.parse::<i32>().unwrap()")
        }
        ast::UnitType::Int0 => {
            format!("({v}.parse::<i32>().unwrap() - 1)")
        }
        ast::UnitType::Float => {
            format!("{v}.parse::<f64>().unwrap()")
        }
        ast::UnitType::Str => {
            format!("{v}.to_owned()")
        }
    }
}
