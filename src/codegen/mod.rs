use crate::ast;
use crate::ast::*;
use crate::new_id;

pub mod readline {
    use super::*;

    pub trait GetArity {
        fn arity(&self) -> Index;
    }

    impl GetArity for UnitType {
        fn arity(&self) -> Index {
            Index::n(1)
        }
    }
    impl GetArity for Array {
        fn arity(&self) -> Index {
            let len = &self.1;
            Index(len.0.to_owned())
        }
    }
    impl GetArity for List {
        fn arity(&self) -> Index {
            let n = &self.1;
            Index::n(1) + Index(n.0.to_owned())
        }
    }
    impl GetArity for TupleElem {
        fn arity(&self) -> Index {
            match self {
                TupleElem::UnitType(x) => x.arity(),
                TupleElem::Array(x) => x.arity(),
                TupleElem::List(x) => x.arity(),
            }
        }
    }
    impl GetArity for Tuple {
        fn arity(&self) -> Index {
            let mut sum = Index::n(0);
            for e in &self.0 {
                sum = sum + e.arity();
            }
            sum
        }
    }
    impl GetArity for TupleLike {
        fn arity(&self) -> Index {
            match self {
                TupleLike::Array(x) => x.arity(),
                TupleLike::Tuple(x) => x.arity(),
                TupleLike::List(x) => x.arity(),
            }
        }
    }
    impl GetArity for Type {
        fn arity(&self) -> Index {
            match self {
                Type::UnitType(x) => x.arity(),
                Type::TupleLike(x) => x.arity(),
                _ => unreachable!(),
            }
        }
    }

    pub trait Lang {
        fn read_line(bind: Bind) -> (Code, Index);
        fn unit_type(bind: Bind, ast: &ast::UnitType, source: Slice) -> Code;
        fn array(bind: Bind, ast: &ast::Array, source: Slice) -> Code;
        fn list(bind: Bind, ast: &ast::List, source: Slice) -> Code;
        fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, Error>;
        fn tuple(bind: Bind, elems: Vec<(&ast::TupleElem, Bind)>) -> Result<Code, Error>;
        fn tuple_like(bind: Bind, ast: &ast::TupleLike, source: Slice) -> Result<Code, Error> {
            match ast {
                ast::TupleLike::Array(ast) => Ok(Self::array(bind, ast, source)),
                ast::TupleLike::List(ast) => Ok(Self::list(bind, ast, source)),
                ast::TupleLike::Tuple(ast::Tuple(elems)) => {
                    let Slice(line_name, Range(fi, la)) = source;
                    let mut out = vec![];
                    let mut inner = vec![];
                    let mut head = fi;
                    for elem in elems {
                        match &elem {
                            TupleElem::UnitType(x) => {
                                let last = head.clone() + x.arity();
                                let ran = Range(head, last.clone());
                                let var = new_var();
                                let mut code =
                                    Self::unit_type(var.clone(), x, Slice(line_name.clone(), ran));
                                out.append(&mut code);
                                inner.push((elem, var));
                                head = last;
                            }
                            TupleElem::Array(x) => {
                                let last = head.clone() + x.arity();
                                let ran = Range(head, last.clone());
                                let var = new_var();
                                let mut code =
                                    Self::array(var.clone(), x, Slice(line_name.clone(), ran));
                                out.append(&mut code);
                                inner.push((elem, var));
                                head = last;
                            }
                            TupleElem::List(x) => {
                                let last = head.clone() + x.arity();
                                let ran = Range(head, last.clone());
                                let var = new_var();
                                let mut code =
                                    Self::list(var.clone(), x, Slice(line_name.clone(), ran));
                                out.append(&mut code);
                                inner.push((elem, var));
                                head = last;
                            }
                        }
                    }
                    let mut code = Self::tuple(bind, inner)?;
                    out.append(&mut code);
                    Ok(out)
                }
            }
        }
    }

    pub fn emit<L: Lang>(root: ast::Root) -> anyhow::Result<String> {
        let mut out: Vec<String> = vec![];
        for line in root.0 {
            let mut n = 0;
            for Definition(_, typ) in &line.0 {
                match typ {
                    Type::UnitType(_) => {
                        n += 1;
                    }
                    Type::TupleLike(_) => {
                        n += 1;
                    }
                    Type::Matrix(_) => {}
                }
            }
            if n > 0 {
                let line_var = new_var();
                let (mut code, len) = L::read_line(line_var.clone());
                out.append(&mut code);
                let mut head = Index::zero();
                for Definition(var, typ) in line.0 {
                    let var = Bind(var.0);
                    match &typ {
                        Type::UnitType(x) => {
                            let last = head.clone() + x.arity();
                            let ran = Range(head, last.clone());
                            let mut code = L::unit_type(var, x, Slice(line_var.clone(), ran));
                            out.append(&mut code);
                            head = last;
                        }
                        Type::TupleLike(x) => {
                            let last = head.clone() + x.arity();
                            let ran = Range(head, last.clone());
                            let mut code = L::tuple_like(var, x, Slice(line_var.clone(), ran))?;
                            out.append(&mut code);
                            head = last;
                        }
                        _ => unreachable!(),
                    }
                }
            } else {
                for Definition(var, typ) in line.0 {
                    let var = Bind(var.0);
                    match &typ {
                        Type::Matrix(x) => {
                            let mut code = L::matrix(var, x)?;
                            out.append(&mut code);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        Ok(out.join("\n"))
    }
}

pub mod stream {
    use super::*;
    pub trait Lang {
        fn unit_type(bind: Bind, ast: &ast::UnitType) -> Code;
        fn array(bind: Bind, ast: &ast::Array) -> Code;
        fn list(bind: Bind, ast: &ast::List) -> Code;
        fn matrix(bind: Bind, ast: &ast::Matrix) -> Result<Code, Error>;
        fn tuple(bind: Bind, elems: Vec<(&ast::TupleElem, Bind)>) -> Result<Code, Error>;
        fn tuple_like(bind: Bind, ast: &ast::TupleLike) -> Result<Code, Error> {
            match ast {
                ast::TupleLike::Array(ast) => Ok(Self::array(bind, ast)),
                ast::TupleLike::List(ast) => Ok(Self::list(bind, ast)),
                ast::TupleLike::Tuple(ast::Tuple(elems)) => {
                    let mut out = vec![];
                    let mut inner = vec![];
                    for elem in elems {
                        match &elem {
                            TupleElem::UnitType(x) => {
                                let var = new_var();
                                let mut code = Self::unit_type(var.clone(), x);
                                out.append(&mut code);
                                inner.push((elem, var));
                            }
                            TupleElem::Array(x) => {
                                let var = new_var();
                                let mut code = Self::array(var.clone(), x);
                                out.append(&mut code);
                                inner.push((elem, var));
                            }
                            TupleElem::List(x) => {
                                let var = new_var();
                                let mut code = Self::list(var.clone(), x);
                                out.append(&mut code);
                                inner.push((elem, var));
                            }
                        }
                    }
                    let mut code = Self::tuple(bind, inner)?;
                    out.append(&mut code);
                    Ok(out)
                }
            }
        }
    }
    pub fn emit<L: Lang>(root: ast::Root) -> anyhow::Result<String> {
        let mut out: Code = vec![];
        for line in root.0 {
            for Definition(var, typ) in line.0 {
                let var = Bind(var.0);
                match &typ {
                    Type::UnitType(x) => {
                        let mut code = L::unit_type(var, x);
                        out.append(&mut code);
                    }
                    Type::TupleLike(x) => {
                        let mut code = L::tuple_like(var, x)?;
                        out.append(&mut code);
                    }
                    Type::Matrix(x) => {
                        let mut code = L::matrix(var, x)?;
                        out.append(&mut code);
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(out.join("\n"))
    }
}

pub mod cpp;
pub mod cpp_stream;
pub mod csharp;
pub mod go_stream;
pub mod java;
pub mod java_stream;
pub mod kotlin;
pub mod nim;
pub mod python;
pub mod ruby;
pub mod rust;

#[derive(Clone)]
pub struct Bind(pub String);
impl std::fmt::Display for Bind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
struct Index(String);
impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Index {
    fn zero() -> Index {
        Index("0".to_string())
    }
    fn n(n: u64) -> Index {
        Index(format!("{n}"))
    }
}
impl std::ops::Add for Index {
    type Output = Index;
    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Index(a), Index(b)) => Index(format!("({a} + {b})")),
        }
    }
}
impl std::ops::Sub for Index {
    type Output = Index;
    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Index(a), Index(b)) => Index(format!("({a} - {b})")),
        }
    }
}

struct Range(pub Index, pub Index);
struct Slice(pub Bind, pub Range);

pub fn new_var() -> Bind {
    Bind(new_id())
}

pub type Code = Vec<String>;
fn append_code(dest: &mut Code, indent: &str, src: Code) {
    for line in src {
        dest.push(format!("{indent}{line}"));
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("Tuple isn't supported.")]
    TupleNotSupported,
}
