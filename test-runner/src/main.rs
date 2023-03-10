use anyhow::Result;
use colored::*;
use procon_input_compiler as Compiler;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Lang {
    template: PathBuf,
    compile: PathBuf,
    runner: PathBuf,
}

#[derive(Debug)]
struct TestCase {
    input: PathBuf,
    parser: PathBuf,
}

struct ExecInfo {
    compile_time: Duration,
    run_time: Duration,
}
#[derive(Debug)]
struct TestTask<'a> {
    kind: &'a str,
    lang_name: &'a str,
    case_idx: u64,
    // lang/python/template
    template: &'a Path,
    compile: &'a Path,
    // lang/python/runner
    runner: &'a Path,
    // case/1
    case: &'a TestCase,
    // checker/python/1
    checker: Option<&'a Path>,
    // target
    target: &'a Path,
}
#[derive(serde::Serialize)]
struct Context {
    parser: String,
    checker: String,
}
impl TestTask<'_> {
    fn exec(self) -> Result<ExecInfo> {
        let parser = {
            let parser = read(&self.case.parser)?;
            match self.lang_name.as_ref() {
                "python" => Compiler::compile(Compiler::Lang::Python, &parser)?,
                "cpp" => Compiler::compile(Compiler::Lang::Cpp, &parser)?,
                "cpp-stream" => Compiler::compile(Compiler::Lang::CppStream, &parser)?,
                "nim" => Compiler::compile(Compiler::Lang::Nim, &parser)?,
                "ruby" => Compiler::compile(Compiler::Lang::Ruby, &parser)?,
                "java" => Compiler::compile(Compiler::Lang::Java, &parser)?,
                "java-stream" => Compiler::compile(Compiler::Lang::JavaStream, &parser)?,
                "csharp" => Compiler::compile(Compiler::Lang::CSharp, &parser)?,
                "rust" => Compiler::compile(Compiler::Lang::Rust, &parser)?,
                "kotlin" => Compiler::compile(Compiler::Lang::Kotlin, &parser)?,
                "go-stream" => Compiler::compile(Compiler::Lang::GoStream, &parser)?,
                "swift" => Compiler::compile(Compiler::Lang::Swift, &parser)?,
                _ => unreachable!(),
            }
        };
        let checker = match self.checker {
            Some(path) => read(path)?,
            None => "".to_string(),
        };
        let exec_content = {
            let mut engine = tinytemplate::TinyTemplate::new();

            engine.set_default_formatter(&tinytemplate::format_unescaped);

            let template = read(self.template)?;
            engine.add_template(self.lang_name, &template)?;

            let ctx = Context { parser, checker };
            engine.render(self.lang_name, &ctx)?
        };
        let exec_file = self.target.join(format!(
            "{}-{}-{}",
            self.kind, self.lang_name, self.case_idx
        ));
        write(&exec_file, exec_content)?;

        let mut command = Command::new("sh");
        command.arg(self.compile);
        command.arg(exec_file);
        let t = Instant::now();
        let r = command.status()?;
        let compile_time = t.elapsed();
        anyhow::ensure!(r.success(), "failed to compile.");

        let input = File::open(&self.case.input)?;
        let mut command = Command::new("sh");
        command.arg(self.runner);
        command.stdin(input);
        let t = Instant::now();
        let r = command.status()?;
        let run_time = t.elapsed();
        anyhow::ensure!(r.success(), "failed to run.");

        Ok(ExecInfo {
            compile_time,
            run_time,
        })
    }
}

struct BenchTask<'a> {
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

use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
struct Opts {
    #[arg(long)]
    lang: Option<String>,
    #[command(subcommand)]
    sub: Sub,
}
#[derive(Debug, Subcommand)]
enum Sub {
    Test,
    Bench,
    MakeBench,
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
                    template: path.join("template"),
                    compile: path.join("compile"),
                    runner: path.join("runner"),
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

    match opts.sub {
        Sub::Test => {
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
                            kind: "test",
                            lang_name,
                            case_idx,
                            case: case,
                            template: &lang.template,
                            compile: &lang.compile,
                            runner: &lang.runner,
                            checker: Some(checker_file),
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
        }
        Sub::Bench => {
            cur.push("bench-case");
            let mut cases: BTreeMap<u64, TestCase> = BTreeMap::new();
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
                cases.insert(
                    idx,
                    TestCase {
                        input: path.join("input"),
                        parser: path.join("parser"),
                    },
                );
            }
            cur.pop();

            let target = cur.join("target");
            std::fs::create_dir(&target).ok();
            let mut test_tasks = BTreeMap::new();
            for (lang_name, lang) in &langs {
                for (&case_idx, case) in &cases {
                    let task = TestTask {
                        kind: "bench",
                        lang_name,
                        case_idx,
                        case,
                        template: &lang.template,
                        compile: &lang.compile,
                        runner: &lang.runner,
                        checker: None,
                        target: &target,
                    };
                    test_tasks.insert((lang_name.to_owned(), case_idx), task);
                }
            }
            let mut result: BTreeMap<u64, BTreeMap<String, Duration>> = BTreeMap::new();
            for ((lang_name, idx), test_task) in test_tasks {
                match test_task.exec() {
                    Ok(info) => {
                        result
                            .entry(idx)
                            .or_default()
                            .insert(lang_name.to_owned(), info.run_time);
                        println!("{lang_name}-{idx}: {:?}", info.run_time);
                    }
                    Err(_) => {
                        let err = "ERR".red();
                        // No benchmark should fail.
                        panic!("{lang_name}-{idx} {err}");
                    }
                }
            }
            println!("{}", make_table(result));
        }
        Sub::MakeBench => {
            cur.push("bench-case");
            write(&cur.join("1").join("input"), bench_1());
            write(&cur.join("2").join("input"), bench_2());
            write(&cur.join("3").join("input"), bench_3());
            cur.pop();
        }
    }

    Ok(())
}
use tabled::{Style, Table, Tabled};
#[derive(Tabled, Default)]
struct BenchResult {
    #[tabled(rename = "Bench#")]
    bench_no: u64,
    // values are in ms
    #[tabled(rename = "Python")]
    python: u64,
    #[tabled(rename = "C++")]
    cpp: u64,
    #[tabled(rename = "C++ (Stream)")]
    cpp_stream: u64,
    #[tabled(rename = "Nim")]
    nim: u64,
    #[tabled(rename = "Ruby")]
    ruby: u64,
    #[tabled(rename = "Java")]
    java: u64,
    #[tabled(rename = "Java (Stream)")]
    java_stream: u64,
    #[tabled(rename = "C#")]
    csharp: u64,
    #[tabled(rename = "Rust")]
    rust: u64,
    #[tabled(rename = "Kotlin")]
    kotlin: u64,
    #[tabled(rename = "Go (Stream)")]
    go_stream: u64,
    #[tabled(rename = "Swift")]
    swift: u64,
}
fn make_table(result: BTreeMap<u64, BTreeMap<String, Duration>>) -> String {
    let mut rows: Vec<BenchResult> = vec![];
    for (bench_no, result_map) in result {
        let mut row = BenchResult::default();
        row.bench_no = bench_no;
        for (lang, du) in result_map {
            let du = du.as_millis() as u64;
            match lang.as_str() {
                "python" => row.python = du,
                "cpp" => row.cpp = du,
                "cpp-stream" => row.cpp_stream = du,
                "nim" => row.nim = du,
                "ruby" => row.ruby = du,
                "java" => row.java = du,
                "java-stream" => row.java_stream = du,
                "csharp" => row.csharp = du,
                "rust" => row.rust = du,
                "kotlin" => row.kotlin = du,
                "go-stream" => row.go_stream = du,
                "swift" => row.swift = du,
                _ => unreachable!(),
            }
        }
        rows.push(row);
    }
    let mut tbl = Table::new(rows);
    tbl.with(Style::markdown());
    tbl.to_string()
}
fn bench_1() -> String {
    let mut out = String::new();
    out.push_str("100000\n");
    let a = vec!["1.0"; 100000];
    out.push_str(&a.join(" "));
    out
}
fn bench_2() -> String {
    let mut out = String::new();
    out.push_str("100000\n");
    let e = vec!["1 1"; 100000];
    out.push_str(&e.join("\n"));
    out
}
fn bench_3() -> String {
    let mut out = String::new();
    out.push_str("1000\n");
    let s = vec!["a"; 1000].join("");
    let a = [s.as_str(); 1000];
    out.push_str(&a.join("\n"));
    out
}
