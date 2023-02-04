# procon-input-compiler

This is a compiler project behind [Procon Input](https://akiradeveloper.github.io/procon-input/).

## Overview

If you play competitive programming,
you should be bothered parsing the input.

The syntax of the input is usually something like this:

```
N
A1 A2 A3 ... AN
```

You may be parsing the input by writing boilarplate code but
imagine what if you can parse the input from the syntax itself.

```
n: int
a: [int; n]
```

This is what Procon Input does.

![スクリーンショット 2023-01-22 17 45 45](https://user-images.githubusercontent.com/785824/213907536-c5381fb4-2854-4208-8962-8f9d3859da5a.png)
![スクリーンショット 2023-01-22 17 45 19](https://user-images.githubusercontent.com/785824/213907534-ae6c317e-d319-4868-9bad-687bb6d45285.png)

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
### Example 1: Matrix

```
4 3
1 2 3
4 5 6
7 8 9
10 11 12
```

```
n: int, m: int
mat: [[int;m]; n]
```

### Example 2: Jagged Array

`int0` interprets 1-indexed number to a 0-indexed number.

```
4
2 2 3
1 4
0
0
```

```
n: int
g: [[int0]; n]
```

```mermaid
graph TD
  v1(1) --> v2(2)
  v1 --> v3(3)
  v2 --> v4(4)
```

### Example 3: Weighted Graph

```
4 3
1 2 1.0
1 3 1.5
2 4 2.5
```

```
n_v: int, n_e: int
e: [(int0, int0, float); n_e]
```

```mermaid
graph TD
  v1(1) -->|1.0| v2(2)
  v1 -->|1.5| v3(3)
  v2 -->|2.5| v4(4)
```

### Example 4: Vertical Array

Single element tuple `(A)` is interpreted as `A`.

```
3
10
20
40
```

```
n: int
a: [(int); n]
```

## Supported Languages

- Supported languages: Python, C++, Nim, Ruby, Java, C#, Rust, Kotlin, Go, Swift

### Mapping

|name | syntax | Python | C++ | Nim | Ruby | Java | C# | Rust | Kotlin | Go | Swift |
|-|-|-|-|-|-|-|-|-|-|-|-|
|integer number|int|`int`|`int`|`int`|`Integer`|`Integer`|`int`|`i32`|`Int`|`int`|`Int`|
|floating number|float|`float`|`double`|`float`|`Float`|`Double`|`double`|`f64`|`Double`|`float64`|`Double`|
|string|str|`str`|`string`|`string`|`String`|`String`|`string`|`String`|`String`|`string`|`String`|
|tuple|(A,B)|`(A,B)`|`tuple<A,B>`|`(A,B)`|`[A,B]`|Not Supported|`ValueTuple<A,B>`|`(A,B)`|Not Supported|Not Supported|`(A,B)`|
|array|[A;n]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|`ArrayList<A>`|`List<A>`|`Vec<A>`|`ArrayList<A>`|`[]A`|`[A]`|
|list|[A]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|`ArrayList<A>`|`List<A>`|`Vec<A>`|`ArrayList<A>`|`[]A`|`[A]`|
|matrix|[A;n]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|`ArrayList<A>`|`List<A>`|`Vec<A>`|`ArrayList<A>`|`[]A`|`[A]`|

### Performance (ms)

| Bench# | Python | C++ | C++ (Stream) | Nim | Ruby | Java | Java (Stream) | C#  | Rust | Kotlin | Go (Stream) | Swift |
|--------|--------|-----|--------------|-----|------|------|---------------|-----|------|--------|-------------|-------|
| 1      | 48     | 24  | 24           | 35  | 94   | 134  | 622           | 71  | 16   | 364    | 21          | 51    |
| 2      | 123    | 53  | 28           | 77  | 189  | 168  | 312           | 118 | 30   | 336    | 32          | 86    |
| 3      | 26     | 11  | 10           | 19  | 63   | 116  | 142           | 41  | 8    | 312    | 19          | 58    |


- bench-case
  - [1](https://github.com/akiradeveloper/procon-input-compiler/blob/master/test-runner/data/bench-case/1/parser) (n=100000): Large Array
  - [2](https://github.com/akiradeveloper/procon-input-compiler/blob/master/test-runner/data/bench-case/2/parser) (n=100000): Large Graph
  - [3](https://github.com/akiradeveloper/procon-input-compiler/blob/master/test-runner/data/bench-case/3/parser) (n=1000): Large Matrix

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

