#[derive(Debug)]
pub struct Root(pub Vec<Line>);
#[derive(Debug)]
pub struct Line(pub Vec<Definition>);
#[derive(Debug)]
pub struct Var(pub String);
#[derive(Debug)]
pub struct Definition(pub Var, pub Type);
#[derive(Debug)]
pub enum UnitType {
    Int,
    Int0,
    Float,
    Str,
}
#[derive(Debug)]
pub struct Array(pub UnitType, pub Len);
#[derive(Debug)]
pub struct List(pub UnitType);
#[derive(Debug)]
pub enum TupleElem {
    UnitType(UnitType),
    Array(Array),
    List(List),
}
#[derive(Debug)]
pub struct Tuple(pub Vec<TupleElem>);
#[derive(Debug)]
pub enum TupleLike {
    Tuple(Tuple),
    Array(Array),
    List(List),
}
#[derive(Debug)]
pub struct ConstNum(pub usize);
#[derive(Debug)]
pub struct Len(pub String);
#[derive(Debug)]
pub struct Matrix(pub TupleLike, pub Len);
#[derive(Debug)]
pub enum Type {
    UnitType(UnitType),
    TupleLike(TupleLike),
    Matrix(Matrix),
}
