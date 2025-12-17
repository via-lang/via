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
0xcafebabe
0xde4db3ef
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

- A decimal integer part
- A decimal point (`.`)
- A fractional part

```ts
3.14
0.5
42.0
```

Scientific notation is also supported using `e` or `E`:

```ts
1e3
2.5e-4
6.022e23
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

| Escape | Meaning      |
| -----: | ------------ |
|   `\\` | Backslash    |
|   `\"` | Double quote |
|   `\n` | Newline      |
|   `\t` | Tab          |

Example:

```ts
"hello\nworld";
"\"quoted text\"";
```

## Array

Array expressions are a way to initialize arrays with an arbitrary amount of elements (other expressions) inline.

```rust
[1, 2, 3] // [int]
```

All elements of an array must have the exact same type. Failure to satisfy this condition is going to cause type deduction to fail, and result in a **semantic compiler error**.

```rust
[1, 2, 3] // OK
[1, 2, "3"] // ERROR
```

## Map

Map expressions are a way to initialize maps with an arbitrary amount of key-value expression pairs inline.

```rust
{ "foo": 1, "bar": 2 } // {string: int}
```

All keys and elements must have the exact same type between their respective sides of the pair type. Failure to satisfy this condition is going to cause type deduction to fail, and result in a **semantic compiler error**.

```rust
{ "foo": 1, "bar": 2 } // OK
{ "foo": 1, "bar": false } // ERROR
```

## Symbol

Symbol expressions are a way to reference anything with an identity.

```zig
var foo = 10
foo // <- Symbol expression
```

## Unary

Unary expressions are composed of an operator component that is followed by an expression.

### Operators

| Operator               | Function                                                                                                               |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `-` (Negation)         | Invokes `neg` implemented by the [Arith](traits.md#arith) trait                                                        |
| `!` (Logical NOT)      | Invokes `not` implemented by the [Logic](traits.md#logic) trait                                                        |
| `~` (Bitwise NOT)      | Invokes `not` implemented by the [Bitwise](traits.md#bitwise) trait                                                    |
| `&` (Reference)        | Creates a reference to an expression with an identity (See [reference expressions](#reference) for more information)   |
| `try` (Protected call) | Evaluates an expression (that may raise an error) in protected mode (See [try expressions](#try) for more information) |

## Binary

Binary expressions are composed of two expressions and an operator in between.

### Operators

| Operator              | Function                                                            |
| --------------------- | ------------------------------------------------------------------- |
| `+` (Addition)        | Invokes `add` implemented by the [Arith](traits.md#arith) trait     |
| `-` (Subtraction)     | Invokes `sub` implemented by the [Arith](traits.md#arith) trait     |
| `*` (Multiplication)  | Invokes `mul` implemented by the [Arith](traits.md#arith) trait     |
| `/` (Division)        | Invokes `div` implemented by the [Arith](traits.md#arith) trait     |
| `**` (Exponentiation) | Invokes `pow` implemented by the [Arith](traits.md#arith) trait     |
| `%` (Modulo)          | Invokes `mod` implemented by the [Arith](traits.md#arith) trait     |
| `&` (Bitwise AND)     | Invokes `and` implemented by the [Bitwise](traits.md#bitwise) trait |
| `\|` (Bitwise OR)     | Invokes `or` implemented by the [Bitwise](traits.md#bitwise) trait  |
| `^` (Bitwise XOR)     | Invokes `xor` implemented by the [Bitwise](traits.md#bitwise) trait  |
| `&&` (Logical AND)    | Invokes `and` implemented by the [Logic](traits.md#logic) trait     |
| `\|\|` (Logical OR)   | Invokes `or` implemented by the [Logic](traits.md#logic) trait      |

## Postfix

Postfix expressions are composed of an two expression with an operator in between. They have lower precedence compared to [Binary](#binary) expressions.

### Operators

| Operator | Function |
| --- | --- |
| `.` (Dynamic access) | Access into a value |
| `::` (Static access) | Access into a  record |
| `as` (Type cast) | Invokes `to` implemented by the `Conv` trait (See [as expressions](#as) for more information) |
| `if..else` (Ternary) | Inline variance (See [ternary expressions](#ternary) for more information)
| `()` (Function call) | Invokes `call` implemented by the `Function` trait (See [call expressions](#function-call) for more information) |
| `[]` (Subscript) | Invokes `subs` implemented by the `Access` trait (See [subscript expressions](#subscript) for more information) |
| `::<>` (Instantiation access) | Access a specific instantiation (See [instantiation access expressions](#instantiation-access) for more information) |

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

## try

A `try` expression evaluates another expression in **protected mode**. It is used to "catch" errors raised by the enclosed expression. In which case no stack unwinding occurs. Errors are converted into values.

```zig
var result = try foo(...)
```

If the expression completes successfully, its value is returned.
If the expression raises an error, the raised value is returned instead.

A `try` expression always produces **a single value**.
If the expression has type `T raise E`, then the result of `try` has type `T | E`.

```ts
var bar = try "foo" as int // Stack frame panics without `try`
```

`try` has the **lowest precedence of all unary operators**, so it applies to the entire expression that follows it.

## as

`as` (typecast) expressions are used to coerce an expression into a specific type. It invokes `to` implemented by the `Conv` trait.

```ts
10 as string // string("10")
```

The resulting type is always the specified type, along with a `raise` extension if the coercion is potentially erroneous.

```ts
"10" as int // int(10)
"abc" as int // ERROR
```

Both expressions typed `int raise CastError<string, int>` in the given example.

This unsafe behavior can be made safe using [try expressions](#try).

```ts
var result = try "abc" as int
```

## Ternary

A ternary expression selects between two expressions based on a boolean condition, allowing simple conditional logic to be written inline.

```zig
var foo = true
var answer = "yes" if foo else "no"
```

Both arms of the ternary expression must have the same type. Failure to satisfy this condition will result in a **semantic compilation error**.

```zig
"yes" if ... else "no" // OK
1 if ... else false // ERROR
```

> [!WARNING] 
> Ternary expressions **only** evaluate the condition and the resulting expression. This may cause **subtle and devastating** bugs if stateful behavior is attached to either arm of the ternary expression.

## Lambda

Lambda expressions are a way to construct anonymous functions inline.

```zig
fn (x: int): x + 10 
```

This example uses special syntax, where if the inline-scope syntax (denoted by `:`) is terminated by an [expression statement](), it behaves as a return statement that encloses the expression.

It is possible to define more complex lambdas with C-style bracket terminated bodies. It is also possible to specify a return type in a similar way to [function declaration statements]().

```zig
fn (a: int, b: int) -> int {
    var x = a * b
    var y = a % b
    return x / y
}
```

## Function call

A function call expression is used to call another expression with a list of arguments.

```rust
foo()
bar(10)
```

Any expression that implements the `Function` trait can be called directly without having to be wrapped with an identity.

```rust
10() // ERROR
fn (foo: int) { ... }(10) // OK
```

## Subscript

A subscript expression accesses an element of a value, typically a collection, using C-style square-bracket notation.

```zig
var foo = [1, 2, 3]
foo[0] // 1
```

Semantically, a subscript expression invokes the `subs` method implemented by the `Access` trait on the left-hand operand.

## Instantiation access

An instantiation access expression is a way to explicitly select a specific instantiation of a [template]().

They can be used for explicit dispatch, or when type deduction fails on certain parameters with complex types.

```rust
println::<int>(10)
```

> [!NOTE]
> In this specific example, the explicit instantiation access is **not** required as the parameter type is trivial.

If the selected instantiation does not exist, it will result in a **semantic compilation error**.

Assuming `foo` is only instantiated for `int`:

```rust
foo::<int>() // OK
foo::<string>() // ERROR
```

# Notes

- Numeric literals ([int](#integer) and [float](#float)) are an exception to the [Arith](traits.md#arith) and [Bitwise](traits.md#bitwise) traits, so you can use appropriate [unary](#operators) and [binary](#operators-1) operators without writing any trait implementations.

- [none](types.md#none), [bool](types.md#bool), and [option](types.md#option) types are an exception to the [Logic](traits.md#logic) trait, so you can use appropriate [unary](#operators) and [binary](#operators-1) operators without writing any trait implementations.
