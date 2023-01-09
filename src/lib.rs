mod ast;
mod emit;
mod parse;

pub fn compile(input: impl AsRef<str>) -> anyhow::Result<String> {
    // https://github.com/rust-bakery/nom/issues/1571#issuecomment-1359257249
    let out = parse::parse(input.as_ref()).map_err(|e| e.to_owned())?.1;
    let out = emit::python::emit(out);
    Ok(out)
}
