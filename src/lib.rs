mod ast;
mod codegen;
mod parse;

use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);
pub fn new_id() -> String {
    let i = COUNTER.fetch_add(1, Ordering::SeqCst);
    let name = format!("v{i}");
    name
}

pub enum Lang {
    Python,
    Cpp,
    Nim,
    Ruby,
    Java,
    CSharp,
    Rust,
    Kotlin,
}

pub fn compile(lang: Lang, input: impl AsRef<str>) -> anyhow::Result<String> {
    // https://github.com/rust-bakery/nom/issues/1571#issuecomment-1359257249
    let out = parse::parse(input.as_ref()).map_err(|e| e.to_owned())?.1;
    let out = match lang {
        Lang::Python => codegen::emit::<codegen::python::Python>(out),
        Lang::Cpp => codegen::emit::<codegen::cpp::Cpp>(out),
        Lang::Nim => codegen::emit::<codegen::nim::Nim>(out),
        Lang::Ruby => codegen::emit::<codegen::ruby::Ruby>(out),
        Lang::Java => codegen::emit::<codegen::java::Java>(out),
        Lang::CSharp => codegen::emit::<codegen::csharp::CSharp>(out),
        Lang::Rust => codegen::emit::<codegen::rust::Rust>(out),
        Lang::Kotlin => codegen::emit::<codegen::kotlin::Kotlin>(out),
    }?;
    Ok(out)
}
