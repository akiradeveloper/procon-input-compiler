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

![スクリーンショット 2023-01-13 20 59 19](https://user-images.githubusercontent.com/785824/212315530-2e6c2873-5135-440b-95aa-cd68a592102a.png)

## Supported Languages

- Supported languages: Python3, C++11, Nim, Ruby

|name | syntax | Python3 | C++11 | Nim | Ruby | 
|-|-|-|-|-|-|
|integer number|int|`int`|`int`|`int`|`Integer`|
|floating number|float|`float`|`double`|`float`|`Float`|
|string|str|`str`|`string`|`string`|`String`|
|tuple|(A,B)|`(A,B)`|`tuple<A,B>`|`(A,B)`|`[A,B]`|
|array|[A;n]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|
|list|[A]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|
|matrix|[A;n]|`[A]`|`vector<A>`|`seq[A]`|`[A]`|

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
  subgraph emit
    EC(Common Layer)
    EL(Language Specific)
  end
  Input -->|Text| P -->|AST| EC --> EL -->|Text| Output
```

## Author

Akira Hayakawa (@akiradeveloper)

