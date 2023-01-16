use crate::ast::*;

#[derive(Clone, Debug)]
pub enum Arity {
    Literal(String),
    Inf,
}
impl Arity {
    pub fn n(n: usize) -> Arity {
        Arity::Literal(n.to_string())
    }
}
impl std::ops::Add for Arity {
    type Output = Arity;
    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Arity::Literal(a), Arity::Literal(b)) => Arity::Literal(format!("({a} + {b})")),
            (_, Arity::Inf) => Arity::Inf,
            (Arity::Inf, _) => Arity::Inf,
        }
    }
}
impl std::ops::Sub for Arity {
    type Output = Arity;
    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Arity::Literal(a), Arity::Literal(b)) => Arity::Literal(format!("({a} - {b})")),
            _ => unreachable!(),
        }
    }
}
#[test]
fn test_arity() {
    let a = Arity::n(10);
    let b = Arity::Literal("m".to_string());
    let c = Arity::n(20);
    dbg!(a.clone() - b.clone());
    dbg!(a.clone() - c.clone());
    dbg!(a.clone() + b - c);
}

pub trait GetArity {
    fn arity(&self) -> Arity;
}

impl GetArity for UnitType {
    fn arity(&self) -> Arity {
        Arity::n(1)
    }
}
impl GetArity for Array {
    fn arity(&self) -> Arity {
        let len = &self.1;
        Arity::Literal(len.0.to_owned())
    }
}
impl GetArity for List {
    fn arity(&self) -> Arity {
        Arity::Inf
    }
}
impl GetArity for TupleElem {
    fn arity(&self) -> Arity {
        match self {
            TupleElem::UnitType(_) => Arity::n(1),
            TupleElem::Array(x) => Arity::Literal(x.1 .0.clone()),
            TupleElem::List(_) => Arity::Inf,
        }
    }
}
impl GetArity for Tuple {
    fn arity(&self) -> Arity {
        let mut sum = Arity::n(0);
        for e in &self.0 {
            sum = sum + e.arity();
        }
        sum
    }
}
impl GetArity for TupleLike {
    fn arity(&self) -> Arity {
        match self {
            TupleLike::Array(x) => x.arity(),
            TupleLike::Tuple(x) => x.arity(),
            TupleLike::List(x) => x.arity(),
        }
    }
}
impl GetArity for Type {
    fn arity(&self) -> Arity {
        match self {
            Type::UnitType(x) => x.arity(),
            Type::TupleLike(x) => x.arity(),
            _ => unreachable!(),
        }
    }
}
