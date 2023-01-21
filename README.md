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
![スクリーンショット 2023-01-20 21 03 33](https://user-images.githubusercontent.com/785824/213690840-7bd3c1b3-7754-4769-a08b-b73a3f623654.png)


## Supported Languages

- Supported languages: Python, C++, Nim, Ruby, Java, C#, Rust, Kotlin

### Mapping

|name | syntax | Python | C++ | Nim | Ruby | Java | C# | Rust | Kotlin |
|-|-|-|-|-|-|-|-|-|-|
|integer number|int|`int`|`int`|`int`|`Integer`|`Integer`|`int`|`i32`|`Int`|
|floating number|float|`float`|`double`|`float`|`Float`|`Double`|`double`|`f64`|`Double`|
|string|str|`str`|`string`|`string`|`String`|`String`|`string`|`String`|`String`|
|tuple|(A,B)|`(A,B)`|`tuple<A,B>`|`(A,B)`|`[A,B]`|Not Supported|`ValueTuple<A,B>`|`(A,B)`|Not Supported|
|array|[A;n]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|`ArrayList<A>`|`List<A>`|`Vec<A>`|`ArrayList<A>`|
|list|[A]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|`ArrayList<A>`|`List<A>`|`Vec<A>`|`ArrayList<A>`|
|matrix|[A;n]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|`ArrayList<A>`|`List<A>`|`Vec<A>`|`ArrayList<A>`|

### Performance (ms)

| Bench# | Python | C++ | Nim | Ruby | Java | C#  | Rust | Kotlin |
|--------|--------|-----|-----|------|------|-----|------|--------|
| 1      | 35     | 15  | 24  | 74   | 117  | 65  | 8    | 300    |
| 2      | 123    | 65  | 68  | 199  | 126  | 135 | 24   | 273    |
| 3      | 15     | 4   | 8   | 52   | 180  | 32  | 4    | 252    |

- bench-case
  - [1](https://github.com/akiradeveloper/procon-input-compiler/blob/master/test-runner/data/bench-case/1/parser) (n=100000): Large Array
  - [2](https://github.com/akiradeveloper/procon-input-compiler/blob/master/test-runner/data/bench-case/2/parser) (m=100000): Large Graph
  - [3](https://github.com/akiradeveloper/procon-input-compiler/blob/master/test-runner/data/bench-case/3/parser) (n=1000, m=1000): Large Matrix

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

