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

![スクリーンショット 2023-02-09 19 26 00](https://user-images.githubusercontent.com/785824/217786742-a4a89d30-79d8-45cb-829c-d21719fb623c.png)
![スクリーンショット 2023-02-09 19 26 09](https://user-images.githubusercontent.com/785824/217786757-7c27192a-0f4f-4dc6-bdb4-8a60126133ff.png)

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

|name | type | Python | C++ | Nim | Ruby | Java | C# | Rust | Kotlin | Go | Swift |
|-|-|-|-|-|-|-|-|-|-|-|-|
|integer number|int|`int`|`int`|`int`|`Integer`|`Integer`|`int`|`i32`|`Int`|`int`|`Int`|
|floating number|float|`float`|`double`|`float`|`Float`|`Double`|`double`|`f64`|`Double`|`float64`|`Double`|
|string|str|`str`|`string`|`string`|`String`|`String`|`string`|`String`|`String`|`string`|`String`|
|tuple|(A,B)|`(A,B)`|`tuple<A,B>`|`(A,B)`|`[A,B]`|Not Supported|`ValueTuple<A,B>`|`(A,B)`|Not Supported|Not Supported|`(A,B)`|
|array|[A;n]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|`ArrayList<A>`|`List<A>`|`Vec<A>`|`ArrayList<A>`|`[]A`|`[A]`|

### Performance (ms)

| Bench# | Python | C++ | C++ (Stream) | Nim | Ruby | Java | Java (Stream) | C#  | Rust | Kotlin | Go (Stream) | Swift |
|--------|--------|-----|--------------|-----|------|------|---------------|-----|------|--------|-------------|-------|
| 1      | 64     | 14  | 14           | 26  | 127  | 137  | 458           | 66  | 14   | 305    | 11          | 51    |
| 2      | 124    | 63  | 22           | 73  | 208  | 141  | 277           | 143 | 24   | 314    | 14          | 113   |
| 3      | 18     | 4   | 4            | 8   | 56   | 109  | 135           | 33  | 4    | 299    | 9           | 72    |

#### bench 1

```
a: [float; 100000]
```

#### bench 2

```
e: [[int; 2]; 100000]
```

#### bench 3

```
a: [(str); 1000]
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

