# Expression

## Literals

### None

`none` is a monomorphic literal used to represent "nothing". There is only one way to write it:

```
none
```

### Boolean

Boolean literals have two states (`true`, `false`) that can be written as:

```lua
true
false
```

### Integer

via has three types of integer literals, which are: `decimal`, `hexadecimal` and `binary`.

The default `int` type is a signed 64-bit integer. It has a maximum width of `±2^63-1` and all literals wider than this limit are considered **semantically invalid**.

Decimal integer literals can be written using `[0-9]` range of characters.
```ts
1
900000
123456789
```

Hexadecimal integer literals are prefixed with `0x` and can be written using `[A-Ea-f0-9]` range of characters.
```ts
0xae3
0xCAFEBABE
0xDe4dB3Ef
```

Finally, binary integer literals are prefixed with `0b` and can be written using `[0-1]` range of characters. They are encoded in the host platform's endianness.
```ts
0b001
0b11111
0b11011011
```

### Float

Floating-point literals represent real numbers with a fractional component.

via currently supports **decimal floating-point literals only**.

A floating-point literal is written using:

* A decimal integer part
* A decimal point (`.`)
* A fractional part

```ts
3.14
0.5
42.0
```

Scientific notation is also supported using `e` or `E`:

```ts
1e3
2.5e-4
6.022E23
```

The exact precision and rounding behavior follow IEEE-754 double-precision semantics.

> [!NOTE]
> There is currently no implicit conversion between integer and floating-point types. Any such conversion must be explicit.

### String

String literals represent immutable sequences of UTF-8 encoded characters.

A string literal is written using double quotes:

```ts
"hello"
"via"
"this is a string"
```

Strings may span multiple characters and lines as long as they are terminated correctly.

#### Escape sequences

The following escape sequences are supported inside string literals:

| Escape | Meaning         |
| -----: | --------------- |
|   `\\` | Backslash       |
|   `\"` | Double quote    |
|   `\n` | Newline         |
|   `\t` | Tab             |

Example:

```ts
"hello\nworld"
"\"quoted text\""
```

## Unary

Unary expressions are composed of an operator component that is followed by an expression.

### Operators

| Operator | Function |
| -------- | -------- |
| `-` (Negation) | Invokes `neg` implemented by the [Arith](traits.md#arith) trait |
| `!` (Logical NOT) | Invokes `not` implemented by the [Logic](traits.md#logic) trait |
| `~` (Bitwise NOT) | Invokes `not` implemented by the [Bitwise](traits.md#bitwise) trait |
| `&` (Reference) | Creates a reference to an expression with an identity (See [reference expressions](#reference) for more information)
| `try` (Protected call) | Evaluates an expression (that may raise an error) in protected mode (See [try expressions](#try) for more information)

## Binary

Binary expressions are composed of two expressions and an operator in between.

### Operators

| Operator | Function |
| -------- | -------- |
| `+` (Addition) | Invokes `add` implemented by the [Arith](traits.md#arith) trait |
| `-` (Subtraction) | Invokes `sub` implemented by the [Arith](traits.md#arith) trait |
| `*` (Multiplication) | Invokes `mul` implemented by the [Arith](traits.md#arith) trait |
| `/` (Division) | Invokes `div` implemented by the [Arith](traits.md#arith) trait |
| `**` (Exponentiation) | Invokes `pow` implemented by the [Arith](traits.md#arith) trait |
| `%` (Modulo) | Invokes `mod` implemented by the [Arith](traits.md#arith) trait |
| `&` (Bitwise AND) | Invokes `and` implemented by the [Bitwise](traits.md#bitwise) trait |
| `\|` (Bitwise OR) | Invokes `or` implemented by the [Bitwise](traits.md#bitwise) trait | 
| `&&` (Logical AND) | Invokes `and` implemented by the [Logic](traits.md#logic) trait |
| `\|\|` (Logical OR) | Invokes `or` implemented by the [Logic](traits.md#logic) trait |

## Postfix

Postfix expressions are composed of an two expression with an operator in between. They have lower precedence compared to [Binary](#binary) expressions.

### Operators

| Operator | Function |
| -------- | -------- |
| `.` (Dynamic access) | Access into a value |
| `::` (Static access) | Access into a static record |
| `()` (Function call) | Invokes `call` implemented by the `Function` trait (See [call expressions](#call) for more information) |
| `::<>` (Template instantiation) | Access a specific instantiation (See [instantiation access expressions](#instantiation-access) for more information) |

## Reference

Reference expressions are a way to create a reference to an expression that has [identity]().

```rust
&x // OK
&x.foo // OK
&10 // ERROR
```

The reference may be modified using the `mut` keyword and the `'` pseudo-operator.

```rust
&'x // strong
&mut x // weak mutable
&mut'x // strong mutable
```

## as

`as` (typecast) expressions are composed of an expression and a type with the `as` keyword in between.

```ts
10 as string
```

The resulting type is always the right-hand side type passed into the `as` expression.

Depending on the nature of the typecast, this expression may raise an error.

```ts
"10" as int // OK
"abc" as int // ERROR
```

Both marked `int raise CastError<string, int>` in the given example.

# Notes

- Numeric literals ([int](#integer) and [float](#float)) are an exception to the [Arith](traits.md#arith) and [Bitwise](traits.md#bitwise) traits, so you can use appropriate [unary](#operators) and [binary](#operators-1) operators without writing any trait implementations.

- [none](types.md#none), [bool](types.md#bool), and [option](types.md#option) types are an exception to the [Logic](traits.md#logic) trait, so you can use appropriate [unary](#operators) and [binary](#operators-1) operators without writing any trait implementations.
