use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending, none_of};
use nom::combinator::{all_consuming, map};
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

use crate::ast::*;

mod generic;
use generic::*;

fn parse_unit_type(i: &str) -> IResult<&str, UnitType> {
    let p = alt((tag("int0"), tag("int"), tag("float"), tag("str")));
    map(p, |s| match s {
        "int" => UnitType::Int,
        "int0" => UnitType::Int0,
        "float" => UnitType::Float,
        "str" => UnitType::Str,
        _ => unreachable!(),
    })(i)
}
fn parse_var(i: &str) -> IResult<&str, Var> {
    map(parse_indent, |x| Var(x.to_string()))(i)
}
fn parse_len(i: &str) -> IResult<&str, Len> {
    let p = many1(none_of("]"));
    map(p, |cs| {
        let mut s = String::new();
        for c in cs {
            s.push(c);
        }
        Len(s)
    })(i)
}
fn parse_array(i: &str) -> IResult<&str, Array> {
    let p = separated_pair(ws(parse_unit_type), char(';'), ws(parse_len));
    let p = map(p, |(fi, la)| Array(fi, la));
    delimited(char('['), p, char(']'))(i)
}
fn parse_list(i: &str) -> IResult<&str, List> {
    let p = map(ws(parse_unit_type), List);
    delimited(char('['), p, char(']'))(i)
}
fn parse_tuple(i: &str) -> IResult<&str, Tuple> {
    let unit_type = map(parse_unit_type, TupleElem::UnitType);
    let array = map(parse_array, TupleElem::Array);
    let list = map(parse_list, TupleElem::List);
    let p = alt((unit_type, array, list));
    let p = separated_list1(char(','), ws(p));
    let p = delimited(char('('), p, char(')'));
    map(p, |x| Tuple(x))(i)
}
fn parse_tuple_like(i: &str) -> IResult<&str, TupleLike> {
    let tuple = map(parse_tuple, TupleLike::Tuple);
    let array = map(parse_array, TupleLike::Array);
    let list = map(parse_list, TupleLike::List);
    alt((tuple, array, list))(i)
}
fn parse_matrix(i: &str) -> IResult<&str, Matrix> {
    let p = separated_pair(ws(parse_tuple_like), char(';'), ws(parse_len));
    let p = map(p, |(fi, la)| Matrix(fi, la));
    delimited(char('['), p, char(']'))(i)
}
fn parse_type(i: &str) -> IResult<&str, Type> {
    let unit_type = map(parse_unit_type, Type::UnitType);
    let tuple_like = map(parse_tuple_like, Type::TupleLike);
    let matrix = map(parse_matrix, Type::Matrix);
    alt((unit_type, tuple_like, matrix))(i)
}
fn parse_definition(i: &str) -> IResult<&str, Definition> {
    let p = separated_pair(ws(parse_var), char(':'), ws(parse_type));
    map(p, |(fi, la)| Definition(fi, la))(i)
}
fn parse_line(i: &str) -> IResult<&str, Line> {
    let p = separated_list0(char(','), ws(parse_definition));
    map(p, Line)(i)
}
fn parse_root(i: &str) -> IResult<&str, Root> {
    let p = separated_list1(line_ending, ws(parse_line));
    map(p, Root)(i)
}
pub fn parse(i: &str) -> IResult<&str, Root> {
    all_consuming(parse_root)(i)
}

mod tests {
    use super::*;

    macro_rules! ok {
        ($p: expr, $data: expr) => {
            assert!(dbg!(all_consuming($p)($data)).is_ok())
        };
    }
    macro_rules! err {
        ($p: expr, $data: expr) => {
            assert!(dbg!($p($data)).is_err())
        };
    }

    #[test]
    fn test_parse() {
        ok!(parse, "");
        ok!(parse, "n: int");
        ok!(parse, "n: int\nm: int");
    }
    #[test]
    fn test_line() {
        ok!(parse_line, "n: int");
        ok!(parse_line, "n: int, m: int");
        ok!(parse_line, "n: int, m: [int; 4]");
        ok!(parse_line, " n: int , m: int ");
    }
    #[test]
    fn test_array() {
        ok!(parse_array, "[int;10]");
        ok!(parse_array, "[int;m]");
        ok!(parse_array, "[ int;m ]");
        ok!(parse_array, "[int; 10]");
        ok!(parse_array, "[int; n+1]");
        ok!(parse_array, "[int; n-1]");
        ok!(parse_array, "[int; n+m]");
        ok!(parse_array, "[int; 2*n]");
        ok!(parse_array, "[float; n]")
    }
    #[test]
    fn test_list() {
        ok!(parse_list, "[int]");
    }
    #[test]
    fn test_var() {
        ok!(parse_var, "x01");
        ok!(parse_var, "Aa01");
        err!(parse_var, "01x");
    }
    #[test]
    fn test_len() {
        ok!(parse_len, "10");
        ok!(parse_len, "m");
        ok!(parse_len, "M");
        ok!(parse_len, "n+1");
    }
    #[test]
    fn test_unit_type() {
        ok!(parse_unit_type, "int");
        ok!(parse_unit_type, "int0");
        ok!(parse_unit_type, "str");
        ok!(parse_unit_type, "float");
    }
    #[test]
    fn test_tuple() {
        ok!(parse_tuple, "(int, str)");
        ok!(parse_tuple, "( int, str,   int)");
        ok!(parse_tuple, "(int, [int])");
        ok!(parse_tuple, "([int;3], [int;2])");
    }
    #[test]
    fn test_matrix() {
        ok!(parse_matrix, "[[int; 4]; 5]");
        ok!(parse_matrix, "[[int]; 4]");
    }
    #[test]
    fn test_definition() {
        ok!(parse_definition, "n: int");
        ok!(parse_definition, "e: [(int0, int0); m]");
    }
}
