use super::*;

fn index_repr(x: &Arity) -> String {
    match x {
        Arity::Literal(x) => x.to_owned(),
        Arity::Inf => "1<<63".to_string(),
    }
}

pub fn emit(root: Root) -> String {
    let mut out = vec![];
    for line in root.0 {
        out.push("xs = input().split()".to_string());
        let mut head = Arity::n(0);
        for Definition(var, typ) in &line.0 {
            match typ {
                Type::UnitType(x) => {
                    let last = head.clone() + x.arity();
                    let slice = format!("xs[{}:{}]", index_repr(&head), index_repr(&last));
                    out.push(format!("{} = {}", var.0, typ.emit(&slice)));
                    head = last;
                }
                Type::TupleLike(x) => {
                    let last = head.clone() + x.arity();
                    let slice = format!("xs[{}:{}]", index_repr(&head), index_repr(&last));
                    out.push(format!("{} = {}", var.0, typ.emit(&slice)));
                    head = last;
                }
                Type::Matrix(Matrix(tuple_like, len)) => {
                    out.remove(out.len() - 1);
                    let rep = &len.0;
                    out.push(format!("{} = []", var.0));
                    out.push(format!("for _ in range({rep}):"));
                    out.push(format!("\txs = input().split()"));
                    out.push(format!("\t{}.append({})", var.0, tuple_like.emit("xs")));
                }
            }
        }
    }
    out.join("\n")
}

trait Emit {
    fn emit(&self, slice: &str) -> String;
}
impl Emit for Type {
    fn emit(&self, slice: &str) -> String {
        match self {
            Type::UnitType(x) => x.emit(slice),
            Type::TupleLike(x) => x.emit(slice),
            _ => todo!(),
        }
    }
}
impl Emit for TupleLike {
    fn emit(&self, slice: &str) -> String {
        match self {
            TupleLike::Array(Array(ty, _)) => {
                format!("[{} for x in {slice}]", ty.emit("[x]"))
            }
            TupleLike::List(List(ty)) => {
                format!("[{} for x in {slice}]", ty.emit("[x]"))
            }
            TupleLike::Tuple(tuple) => {
                let mut inner = vec![];
                let mut head = Arity::n(0);
                for e in &tuple.0 {
                    match e {
                        TupleElem::UnitType(x) => {
                            let last = head.clone() + x.arity();
                            let slice =
                                format!("{slice}[{}:{}]", index_repr(&head), index_repr(&last));
                            inner.push(x.emit(&slice));
                            head = last;
                        }
                        TupleElem::Array(x) => {
                            let last = head.clone() + x.arity();
                            let slice =
                                format!("{slice}[{}:{}]", index_repr(&head), index_repr(&last));
                            inner.push(x.emit(&slice));
                            head = last;
                        }
                        TupleElem::List(x) => {
                            let last = head.clone() + x.arity();
                            let slice =
                                format!("{slice}[{}:{}]", index_repr(&head), index_repr(&last));
                            inner.push(x.emit(&slice));
                            head = last;
                        }
                    }
                }
                format!("({})", inner.join(","))
            }
        }
    }
}
impl Emit for List {
    fn emit(&self, slice: &str) -> String {
        let ty = &self.0;
        format!("[{} for x in {slice}]", ty.emit("[x]"))
    }
}
impl Emit for Array {
    fn emit(&self, slice: &str) -> String {
        let ty = &self.0;
        format!("[{} for x in {slice}]", ty.emit("[x]"))
    }
}
impl Emit for UnitType {
    fn emit(&self, slice: &str) -> String {
        let val = format!("{slice}[0]");
        match self {
            UnitType::Int => format!("int({val})"),
            UnitType::Int0 => format!("(int({val}) - 1)"),
            UnitType::Float => format!("float({val})"),
            UnitType::Str => format!("{val}"),
        }
    }
}
