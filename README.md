# procon-input-compiler

This is a compiler project behind [Procon Input](https://akiradeveloper.github.io/procon-input/).

## Overview

If you play competitive programming,
you should be bothered parsing the input.

The syntax of the input is usually something like this:

```
N M
X1 Y1
X2 Y2
...
XM YM
```

You may be parsing the input by writing boilarplate code but
imagine what if you can parse the input from the syntax itself.

```
n: int, m: int
xy: [(int,int); m]
```

This is what Procon Input does.


![スクリーンショット 2023-01-18 20 32 16](https://user-images.githubusercontent.com/785824/213160943-deb3fd95-5fe4-4dfa-9185-4403efeb10d3.png)
![スクリーンショット 2023-01-18 20 32 43](https://user-images.githubusercontent.com/785824/213160989-e81783db-aa34-4758-8d0c-11ed4a2671a3.png)

## Supported Languages

- Supported languages: Python, C++, Nim, Ruby, Java, C#, Rust

### Mapping

|name | syntax | Python | C++ | Nim | Ruby | Java | C# | Rust |
|-|-|-|-|-|-|-|-|-|
|integer number|int|`int`|`int`|`int`|`Integer`|`Integer`|`int`|`i32`|
|floating number|float|`float`|`double`|`float`|`Float`|`Float`|`double`|`f64`|
|string|str|`str`|`string`|`string`|`String`|`String`|`string`|`String`|
|tuple|(A,B)|`(A,B)`|`tuple<A,B>`|`(A,B)`|`[A,B]`|Not Supported|`ValueTuple<A,B>`|`(A,B)`|
|array|[A;n]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|`ArrayList<A>`|`List<A>`|`Vec<A>`|
|list|[A]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|`ArrayList<A>`|`List<A>`|`Vec<A>`|
|matrix|[A;n]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|`ArrayList<A>`|`List<A>`|`Vec<A>`|

### Performance

| bench_no | python | cpp | nim | ruby | java | csharp | rust |
|----------|--------|-----|-----|------|------|--------|------|
| 1        | 48ms     | 59  | 138 | 76   | 156  | 81     | 30   |
| 2        | 128    | 186 | 620 | 187  | 159  | 124    | 118  |
| 3        | 28     | 12  | 53  | 55   | 110  | 41     | 11   |

- [bench 1](https://github.com/akiradeveloper/procon-input-compiler/blob/master/test-runner/data/bench-case/1/parser) n=100000
- [bench 2](https://github.com/akiradeveloper/procon-input-compiler/blob/master/test-runner/data/bench-case/2/parser) m=100000
- [bench 3](https://github.com/akiradeveloper/procon-input-compiler/blob/master/test-runner/data/bench-case/3/parser) n=1000

## Syntax

```
Root := Line in-between ‘\n’
Line := Definition in-between ‘,‘
Definition := Var : Type

UnitType := int | int0 | float | str
Array := [UnitType; Len]
List := [UnitType]

TupleElem := UnitType | Array | List
Tuple := (TupleElem in-between ‘,’)
TupleLike := Array | List | Tuple

Matrix := [TupleLike; Len]
Type := UnitType | TupleLike | Matrix
```

## Architecuture

```mermaid
graph LR
  P(Parser)
  subgraph codegen
    EC(Common Layer)
    EL(Language Specific)
  end
  Input -->|Text| P -->|AST| EC --> EL -->|Text| Output
```

## Related Works

- [proconio](https://github.com/statiolake/proconio-rs)
- [proconIO.jl](https://github.com/lucifer1004/ProconIO.jl)

## Development

Use `test-runner` command in dev container.

### Test

```
$ ./dev
# cargo run --package test-runner -- test
```

### Benchmark

```
$ ./dev
# cargo run --package test-runner -- bench
```

## Author

Akira Hayakawa (@akiradeveloper)

