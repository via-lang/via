# Traits

## Arith

The `Arith` trait is a default trait that is used to overload [arithmetic operators](expression.md#operators-1) when implemented.

```rust
trait Arith {
    fn add(other: &Self) -> Self
    fn sub(other: &Self) -> Self
    fn mul(other: &Self) -> Self
    fn div(other: &Self) -> Self
    fn pow(other: &Self) -> Self
    fn mod(other: &Self) -> Self
}
```

## Bitwise

The `Bitwise` trait is a default trait that is used to overload [bitwise operators](expression.md#operators-1) when implemented.

```rust
trait Bitwise {
    fn not() -> Self
    fn and(other: &Self) -> Self
    fn or(other: &Self) -> Self
    fn xor(other: &Self) -> Self
}
```

## Logic

The `Logic` trait is a default trait that is used to overload [logic operators](expression.md#operators-1) when implemented.

```rust
trait Logic {
    fn not() -> Self
    fn and(other: &Self) -> Self
    fn or(other: &Self) -> Self
}
```

## Conv

The `Conv` trait (and similarly the `ConvFallable` trait) is a default trait that is used to overload [type coercion operations](expression.md#as) when implemented.

```rust
trait Conv<T: T != Self> {
    fn to() -> T
}

trait ConvFallable<T: T != Self> {
    fn to() -> T raise CastError<Self, T>
}
```

## Function

The `Function` trait is a default trait that is used to overload the [function call operator](expression.md#operators-2) when implemented.

```rust
trait Function<T, ...A> {
    fn call(args: A...) -> T
}
```
