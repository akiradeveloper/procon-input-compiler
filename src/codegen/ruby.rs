use super::*;

pub struct Ruby;
impl Lang for Ruby {
    fn read_line(bind: Bind) -> (Code, Index) {
        let mut code = vec![];
        code.push(format!("{bind} = gets.chomp.split"));
        (code, Index(format!("{bind}.size")))
    }
    fn unit_type(bind: Bind, ast: &ast::UnitType, source: Slice) -> Code {
        let Slice(xs, range) = source;
        let i = range.0;
        let v = format!("{xs}[{i}]");
        let code = format!("{bind} = {}", unit_type_convert(ast, &v));
        vec![code]
    }
    fn array(bind: Bind, ast: &ast::Array, source: Slice) -> Code {
        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;
        let ty = &ast.0;
        let v = format!(
            "{xs}[{i}...{j}].map {{ |x| {} }}",
            unit_type_convert(&ty, "x")
        );
        let code = format!("{bind} = {v}");
        vec![code]
    }
    fn list(bind: Bind, ast: &ast::List, source: Slice) -> Code {
        let mut code = vec![];

        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;

        let n = Bind(ast.1 .0.to_owned());
        code.push(format!("{n} = {xs}[{i}].to_i"));

        let ty = &ast.0;
        let v = format!(
            "{xs}[({i}+1)...{j}].map {{ |x| {} }}",
            unit_type_convert(&ty, "x")
        );
        code.push(format!("{bind} = {v}"));

        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, super::Error> {
        let ty = &ast.0;
        let len = &ast.1;
        let rep = &len.0;
        let mut code = vec![];
        code.push(format!("{bind} = []"));
        code.push(format!("{rep}.times do"));

        let mut inner_code = vec![];
        let xs = new_var();
        inner_code.push(format!("{xs} = gets.chomp.split"));
        let n = new_var();
        inner_code.push(format!("{n} = {xs}.size"));
        let slice = Slice(xs.clone(), Range(Index::zero(), Index(n.0)));
        let t = new_var();
        inner_code.append(&mut Self::tuple_like(t.clone(), &ty, slice)?);
        inner_code.push(format!("{bind} << {t}"));

        append_code(&mut code, "  ", inner_code);
        code.push(format!("end"));

        Ok(code)
    }
    fn tuple(bind: Bind, elems: Vec<(&ast::TupleElem, Bind)>) -> Result<Code, super::Error> {
        let mut inner = vec![];
        let n = elems.len();
        for (_, e) in elems {
            inner.push(e.0);
        }
        let inner = inner.join(", ");
        let code = if n == 1 {
            format!("{bind} = {inner}")
        } else {
            format!("{bind} = [{inner}]")
        };
        Ok(vec![code])
    }
}
fn unit_type_convert(ty: &ast::UnitType, v: &str) -> String {
    match ty {
        ast::UnitType::Int => {
            format!("{v}.to_i")
        }
        ast::UnitType::Int0 => {
            format!("({v}.to_i - 1)")
        }
        ast::UnitType::Float => {
            format!("{v}.to_f")
        }
        ast::UnitType::Str => v.to_string(),
    }
}
