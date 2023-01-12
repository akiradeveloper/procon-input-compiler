use std::io::Read;

use procon_input_compiler as Compiler;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut input)?;
    let out = Compiler::compile(Compiler::Lang::Python3, &input)?;
    println!("{out}");
    Ok(())
}
