use super::*;

pub struct Python;

impl readline::Lang for Python {
    fn read_line(bind: Bind) -> (Code, Index) {
        let mut out = vec![];
        let code = format!("{bind} = input().split()");
        let len = Index(format!("len({})", bind));
        out.push(code);
        (out, len)
    }
    fn unit_type(bind: Bind, ast: &ast::UnitType, source: Slice) -> Code {
        let Slice(slice_name, range) = source;
        let v = format!("{}[{}:{}][0]", slice_name, range.0, range.1);
        let rhs = format!("{}", unit_type_convert(ast, &v));
        let code = format!("{bind} = {rhs}");
        vec![code]
    }
    fn array(bind: Bind, ast: &ast::Array, source: Slice) -> Code {
        let Slice(slice_name, range) = source;
        let slice = format!("{}[{}:{}]", slice_name, range.0, range.1);
        let ty = &ast.0;
        let rhs = format!("[{} for x in {slice}]", unit_type_convert(ty, "x"));
        let code = format!("{bind} = {rhs}");
        vec![code]
    }
    fn list(bind: Bind, ast: &ast::List, source: Slice) -> Code {
        let mut code = vec![];

        let Slice(xs, range) = source;
        let i = range.0;
        let j = range.1;

        let n = Bind(ast.1 .0.to_owned());
        code.push(format!("{n} = int({xs}[{i}])"));

        let slice = format!("{}[({}+1):{}]", xs, i, j);
        let ty = &ast.0;
        let rhs = format!("[{} for x in {slice}]", unit_type_convert(ty, "x"));
        code.push(format!("{bind} = {rhs}"));
        code
    }
    fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, super::Error> {
        let ty = &ast.0;
        let len = &ast.1;
        let rep = &len.0;
        let mut out = vec![];
        out.push(format!("{bind} = []"));
        out.push(format!("for _ in range({rep}):"));
        out.push(format!("\txs = input().split()"));
        let n = format!("len(xs)");
        let ran = Range(Index::zero(), Index(n));
        let slice = Slice(Bind("xs".to_string()), ran);
        let eval_var = new_var();
        let eval_code = Self::tuple_like(eval_var.clone(), ty, slice)?;
        for e in eval_code {
            out.push(format!("\t{e}"));
        }
        out.push(format!("\t{bind}.append({eval_var})"));
        Ok(out)
    }
    fn tuple(bind: Bind, elems: Vec<(&ast::TupleElem, Bind)>) -> Result<Code, super::Error> {
        let mut inner = vec![];
        for (_, e) in elems {
            inner.push(e.0);
        }
        let inner = inner.join(",");
        let code = format!("{bind} = ({inner})");
        Ok(vec![code])
    }
}

fn unit_type_convert(ty: &ast::UnitType, v: &str) -> String {
    match ty {
        ast::UnitType::Int => {
            format!("int({v})")
        }
        ast::UnitType::Float => {
            format!("float({v})")
        }
        ast::UnitType::Int0 => {
            format!("(int({v})-1)")
        }
        ast::UnitType::Str => v.to_string(),
    }
}
