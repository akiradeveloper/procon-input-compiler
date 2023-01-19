use anyhow::Result;
use colored::*;
use procon_input_compiler as Compiler;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
struct TestCase {
    input: PathBuf,
    parser: PathBuf,
}

#[derive(Debug)]
struct Lang {
    runner: PathBuf,
    template: PathBuf,
}

#[derive(Debug)]
struct TestTask<'a> {
    lang_name: &'a str,
    case_idx: u64,
    // lang/python/template
    template: &'a Path,
    // lang/python/runner
    runner: &'a Path,
    // case/1
    case: &'a TestCase,
    // checker/python/1
    checker: &'a Path,
    // target
    target: &'a Path,
}
#[derive(serde::Serialize)]
struct Context {
    parser: String,
    checker: String,
}
impl TestTask<'_> {
    fn exec(self) -> Result<()> {
        let parser = {
            let parser = read(&self.case.parser)?;
            match self.lang_name.as_ref() {
                "python" => Compiler::compile(Compiler::Lang::Python, &parser)?,
                "cpp" => Compiler::compile(Compiler::Lang::Cpp, &parser)?,
                "nim" => Compiler::compile(Compiler::Lang::Nim, &parser)?,
                "ruby" => Compiler::compile(Compiler::Lang::Ruby, &parser)?,
                "java" => Compiler::compile(Compiler::Lang::Java, &parser)?,
                "csharp" => Compiler::compile(Compiler::Lang::CSharp, &parser)?,
                "rust" => Compiler::compile(Compiler::Lang::Rust, &parser)?,
                _ => unreachable!(),
            }
        };
        let checker = { read(self.checker)? };
        let exec_content = {
            let mut engine = tinytemplate::TinyTemplate::new();

            engine.set_default_formatter(&tinytemplate::format_unescaped);

            let template = read(self.template)?;
            engine.add_template(self.lang_name, &template)?;

            let ctx = Context { parser, checker };
            engine.render(self.lang_name, &ctx)?
        };
        let exec_file = self
            .target
            .join(format!("{}-{}", self.lang_name, self.case_idx));
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

use clap::Parser;
#[derive(Parser, Debug)]
struct Opts {
    #[arg(long)]
    lang: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let mut cur = std::env::current_dir()?;
    cur.push("test-runner");
    cur.push("data");

    cur.push("lang");
    let mut langs: BTreeMap<String, Lang> = BTreeMap::new();
    for dir in std::fs::read_dir(&cur)? {
        let dir = dir?;
        let path = dir.path();
        let lang_name: String = path.file_name().unwrap().to_str().unwrap().to_string();

        let mut do_insert = || {
            let lang_name = lang_name.clone();
            langs.insert(
                lang_name,
                Lang {
                    runner: path.join("runner"),
                    template: path.join("template"),
                },
            );
        };
        if let Some(lang) = &opts.lang {
            if &lang_name == lang {
                do_insert();
            }
        } else {
            do_insert();
        }
    }
    cur.pop();

    cur.push("test-case");
    let mut test_cases: BTreeMap<u64, TestCase> = BTreeMap::new();
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
        test_cases.insert(
            idx,
            TestCase {
                input: path.join("input"),
                parser: path.join("parser"),
            },
        );
    }
    cur.pop();

    cur.push("test-checker");
    let mut test_checkers = BTreeMap::new();
    for dir in std::fs::read_dir(&cur)? {
        let dir = dir?;
        let path = dir.path();
        let lang_name: String = path.file_name().unwrap().to_str().unwrap().to_string();

        let checker: BTreeMap<u64, PathBuf> = {
            let mut out = BTreeMap::new();
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
        test_checkers.insert(lang_name, checker);
    }
    cur.pop();

    let target = cur.join("target");
    std::fs::create_dir(&target).ok();

    let mut test_tasks = BTreeMap::new();
    for (lang_name, lang) in &langs {
        for (&case_idx, case) in &test_cases {
            if let Some(checker_file) = test_checkers
                .get(lang_name)
                .and_then(|checker| checker.get(&case_idx))
            {
                let task = TestTask {
                    lang_name,
                    case_idx,
                    case: case,
                    runner: &lang.runner,
                    template: &lang.template,
                    checker: checker_file,
                    target: &target,
                };
                test_tasks.insert((lang_name.to_owned(), case_idx), task);
            }
        }
    }

    let mut failures = vec![];
    for ((lang_name, idx), test_task) in test_tasks {
        let res = test_task.exec();
        match res {
            Ok(_) => {
                let ok = "OK".green();
                println!("{lang_name}-{idx} {ok}")
            }
            Err(e) => {
                let err = "ERR".red();
                println!("{lang_name}-{idx} {err}\n{e}");
                failures.push((lang_name, idx));
            }
        }
    }
    if failures.len() == 0 {
        let passed = "passed".green();
        println!("All test {passed}.");
    } else {
        for (name, idx) in &failures {
            let err = "ERR".red();
            println!("{name}-{idx} {err}");
        }
        let n = failures.len();
        let failed = "failed".red();
        println!("{n} tests {failed}.");
    }
    Ok(())
}
