use anyhow::Result;
use colored::*;
use procon_input_compiler as Compiler;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
struct Case {
    input: PathBuf,
    parser: PathBuf,
}

#[derive(Debug)]
struct Lang {
    runner: PathBuf,
    template: PathBuf,
    checker: BTreeMap<u64, PathBuf>,
}

#[derive(Debug)]
struct Task<'a> {
    name: &'a str,
    idx: u64,
    // case/1
    case: &'a Case,
    // lang/python/template
    template: &'a Path,
    // lang/python/checker/1
    checker: &'a Path,
    // lang/python/runner
    runner: &'a Path,
    // target
    target: &'a Path,
}
#[derive(serde::Serialize)]
struct Context {
    parser: String,
    checker: String,
}
impl Task<'_> {
    fn exec(self) -> Result<()> {
        let parser = {
            let parser = read(&self.case.parser)?;
            match self.name.as_ref() {
                "python" => Compiler::compile(Compiler::Lang::Python3, &parser)?,
                "cpp" => Compiler::compile(Compiler::Lang::Cpp11, &parser)?,
                _ => unreachable!(),
            }
        };
        let checker = { read(self.checker)? };
        let exec_content = {
            let mut engine = tinytemplate::TinyTemplate::new();

            engine.set_default_formatter(&tinytemplate::format_unescaped);

            let template = read(self.template)?;
            engine.add_template(self.name, &template)?;

            let ctx = Context { parser, checker };
            engine.render(self.name, &ctx)?
        };
        let exec_file = self.target.join(format!("{}-{}", self.name, self.idx));
        write(&exec_file, exec_content)?;

        let input = File::open(&self.case.input)?;

        let mut command = Command::new("sh");
        command.arg(self.runner);
        command.arg(exec_file);
        command.stdin(input);
        let r = command.status()?;
        anyhow::ensure!(r.success());

        Ok(())
    }
}

fn read(path: &Path) -> Result<String> {
    let out = std::fs::read_to_string(path)?;
    Ok(out)
}
fn write(path: &Path, data: String) -> Result<()> {
    let will_write = if path.exists() {
        let old = read(path)?;
        data != old
    } else {
        true
    };
    if will_write {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;
        file.write_all(data.as_bytes())?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut cur = std::env::current_dir()?;
    cur.push("test-runner");
    cur.push("data");

    let mut case: BTreeMap<u64, Case> = BTreeMap::new();
    cur.push("case");
    for ent in std::fs::read_dir(&cur)? {
        let ent = ent?;
        let path = ent.path();
        let idx: u64 = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .parse()?;
        case.insert(
            idx,
            Case {
                input: path.join("input"),
                parser: path.join("parser"),
            },
        );
    }
    cur.pop();

    let mut lang: BTreeMap<String, Lang> = BTreeMap::new();
    cur.push("lang");
    for dir in std::fs::read_dir(&cur)? {
        let dir = dir?;
        let path = dir.path();
        let name: String = path.file_name().unwrap().to_str().unwrap().to_string();

        let checker: BTreeMap<u64, PathBuf> = {
            let mut out = BTreeMap::new();
            let path = path.join("checker");
            for ent in std::fs::read_dir(&path)? {
                let ent = ent?;
                let path = ent.path();
                let idx: u64 = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .parse()?;
                out.insert(idx, path);
            }
            out
        };
        lang.insert(
            name,
            Lang {
                runner: path.join("runner"),
                template: path.join("template"),
                checker,
            },
        );
    }
    cur.pop();

    let target = cur.join("target");
    std::fs::create_dir(&target).ok();

    let mut tasks = BTreeMap::new();
    for name in lang.keys() {
        for idx in case.keys() {
            let lang = lang.get(name).unwrap();
            if let Some(checker) = lang.checker.get(idx) {
                let task = Task {
                    name,
                    idx: *idx,
                    case: case.get(idx).unwrap(),
                    runner: &lang.runner,
                    template: &lang.template,
                    checker,
                    target: &target,
                };
                tasks.insert((name.to_owned(), *idx), task);
            }
        }
    }

    for ((name, idx), task) in tasks {
        let res = task.exec();
        match res {
            Ok(_) => {
                let ok = "OK".green();
                println!("{name}-{idx} {ok}")
            }
            Err(e) => {
                let err = "ERR".red();
                println!("{name}-{idx} {err}\n{e}")
            }
        }
    }
    Ok(())
}
