use std::io::Read;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut input)?;
    let out = procon_input::compile(&input)?;
    println!("{out}");
    Ok(())
}
