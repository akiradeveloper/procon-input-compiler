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

#[derive(PartialEq, Clone, Copy)]
pub enum Lang {
    Python,
    Cpp,
    CppStream,
    Nim,
    Ruby,
    Java,
    JavaStream,
    CSharp,
    Rust,
    Kotlin,
    GoStream,
}

pub fn compile(lang: Lang, input: impl AsRef<str>) -> anyhow::Result<String> {
    COUNTER.store(0, Ordering::SeqCst);

    // https://github.com/rust-bakery/nom/issues/1571#issuecomment-1359257249
    let out = parse::parse(input.as_ref()).map_err(|e| e.to_owned())?.1;
    let out = match lang {
        Lang::Python => codegen::readline::emit::<codegen::python::Python>(out),
        Lang::Cpp => codegen::readline::emit::<codegen::cpp::Cpp>(out),
        Lang::CppStream => codegen::stream::emit::<codegen::cpp_stream::CppStream>(out),
        Lang::Nim => codegen::readline::emit::<codegen::nim::Nim>(out),
        Lang::Ruby => codegen::readline::emit::<codegen::ruby::Ruby>(out),
        Lang::Java => codegen::readline::emit::<codegen::java::Java>(out),
        Lang::JavaStream => codegen::stream::emit::<codegen::java_stream::JavaStream>(out),
        Lang::CSharp => codegen::readline::emit::<codegen::csharp::CSharp>(out),
        Lang::Rust => codegen::readline::emit::<codegen::rust::Rust>(out),
        Lang::Kotlin => codegen::readline::emit::<codegen::kotlin::Kotlin>(out),
        Lang::GoStream => codegen::stream::emit::<codegen::go_stream::GoStream>(out),
    }?;
    Ok(out)
}
