# Types

## Builtin Types

Builtin types are global types that require no generics and form the foundation of the type system.

- **`none`** — Represents the absence of a value.
- **`bool`** — Boolean type with two possible values: `true` or `false`.
- **`int`** — 64-bit signed integer.
- **`float`** — 64-bit (typically) IEEE-754 floating-point number.
- **`string`** — Dynamic sequence of characters representing text.

> [!WARNING]
> [Options](#option), [Arrays](#array) and [maps](#map) containing `none` values or keys, such as `none?`, `[none]`, or `{none: none}` are **not allowed**. These types have no practical use and will produce a **compiler error**. The `none` type is only meaningful on its own, e.g., as a function return type.

## Option

Option types are a way to represent a union between a concrete type and `none`. The type of an option is written by adding a question mark to the end of another type.

```c
int?
float?
```

> [!NOTE]
> `T?` is **not** equivalent to `T | none`.

## Array

Arrays are ordered collections of elements of the same type. The type of an array is written by enclosing the element type in square brackets.

```c
[int]
[float?]
[[int]] // Multi-dimensional array
```

Array types implement the [Access trait](traits.md#access) by default. This means that the [subscript operator](expressions.md#subscript) can be used without explicit trait implementation with the `int` type.

```zig
var arr = ["1", "2", "3"]
```

## Map

Maps are unordered collections of key-value pairs with O(1) lookup. The type of a map is written by denoting the key type and value type with a colon in the middle all in between two curly brackets.

```cpp
{float: int?}
{string: bool}
```

## Function

Functions are callable values 

## Union

Unions are superstate-ful types that hold one out of two or more types.

```rust
use number = int | float
number | string | bool
```
