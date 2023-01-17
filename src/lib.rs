mod ast;
mod codegen;
mod parse;

pub enum Lang {
    Python3,
    Cpp11,
    Nim,
    Ruby,
}

pub fn compile(lang: Lang, input: impl AsRef<str>) -> anyhow::Result<String> {
    // https://github.com/rust-bakery/nom/issues/1571#issuecomment-1359257249
    let out = parse::parse(input.as_ref()).map_err(|e| e.to_owned())?.1;
    let out = match lang {
        Lang::Python3 => codegen::emit::<codegen::python3::Python3>(out),
        Lang::Cpp11 => codegen::emit::<codegen::cpp11::Cpp11>(out),
        Lang::Nim => codegen::emit::<codegen::nim::Nim>(out),
        Lang::Ruby => codegen::emit::<codegen::ruby::Ruby>(out),
    }?;
    Ok(out)
}
