use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use crate::ast;
use crate::ast::*;

mod arity;
use arity::*;

pub mod cpp;
pub mod python;
pub mod python3;

#[derive(Clone)]
pub struct Bind(pub String);
impl std::fmt::Display for Bind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
struct Range(pub Index, pub Index);
struct Slice(pub Bind, pub Range);
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
}
impl std::ops::Add for Index {
    type Output = Index;
    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Index(x), Index(y)) => Index(format!("({x} + {y})")),
        }
    }
}
fn unwrap_or(x: Arity, or: Index) -> Index {
    match x {
        Arity::Literal(x) => Index(x),
        Arity::Inf => or,
    }
}

static COUNTER: AtomicU64 = AtomicU64::new(0);
pub fn new_var() -> Bind {
    let i = COUNTER.fetch_add(1, Ordering::SeqCst);
    let name = format!("v{i}");
    Bind(name)
}

pub type Code = Vec<String>;

pub trait Lang {
    fn read_line(bind: Bind) -> (Code, Index);
    fn unit_type(bind: Bind, ast: ast::UnitType, source: Slice) -> Code;
    fn array(bind: Bind, ast: ast::Array, source: Slice) -> Code;
    fn list(bind: Bind, ast: ast::List, source: Slice) -> Code;
    fn matrix(bind: Bind, ast: ast::Matrix) -> Code;
    fn tuple(bind: Bind, elems: Vec<Bind>) -> Code;
    fn tuple_like(bind: Bind, ast: ast::TupleLike, source: Slice) -> Code {
        match ast {
            ast::TupleLike::Array(ast) => Self::array(bind, ast, source),
            ast::TupleLike::List(ast) => Self::list(bind, ast, source),
            ast::TupleLike::Tuple(ast::Tuple(elems)) => {
                let Slice(line_name, Range(fi, la)) = source;
                let mut out = vec![];
                let mut inner = vec![];
                let mut head = fi;
                for elem in elems {
                    match elem {
                        TupleElem::UnitType(x) => {
                            let last = head.clone() + unwrap_or(x.arity(), la.clone());
                            let ran = Range(head, last.clone());
                            let var = new_var();
                            let mut code =
                                Self::unit_type(var.clone(), x, Slice(line_name.clone(), ran));
                            out.append(&mut code);
                            inner.push(var);
                            head = last;
                        }
                        TupleElem::Array(x) => {
                            let last = head.clone() + unwrap_or(x.arity(), la.clone());
                            let ran = Range(head, last.clone());
                            let var = new_var();
                            let mut code =
                                Self::array(var.clone(), x, Slice(line_name.clone(), ran));
                            out.append(&mut code);
                            inner.push(var);
                            head = last;
                        }
                        TupleElem::List(x) => {
                            let last = head.clone() + unwrap_or(x.arity(), la.clone());
                            let ran = Range(head, last.clone());
                            let var = new_var();
                            let mut code =
                                Self::list(var.clone(), x, Slice(line_name.clone(), ran));
                            out.append(&mut code);
                            inner.push(var);
                            head = last;
                        }
                    }
                }
                let mut code = Self::tuple(bind, inner);
                out.append(&mut code);
                out
            }
        }
    }
}

pub fn emit<L: Lang>(root: ast::Root) -> String {
    COUNTER.store(0, Ordering::SeqCst);

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
                match typ {
                    Type::UnitType(x) => {
                        let last = head.clone() + unwrap_or(x.arity(), len.clone());
                        let ran = Range(head, last.clone());
                        let mut code = L::unit_type(var, x, Slice(line_var.clone(), ran));
                        out.append(&mut code);
                        head = last;
                    }
                    Type::TupleLike(x) => {
                        let last = head.clone() + unwrap_or(x.arity(), len.clone());
                        let ran = Range(head, last.clone());
                        let mut code = L::tuple_like(var, x, Slice(line_var.clone(), ran));
                        out.append(&mut code);
                        head = last;
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            for Definition(var, typ) in line.0 {
                let var = Bind(var.0);
                match typ {
                    Type::Matrix(x) => {
                        let mut code = L::matrix(var, x);
                        out.append(&mut code);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    out.join("\n")
}
