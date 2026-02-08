 <h1 align="center">
  <a href="https://github.com/XnLogicaL/via-lang">
    <img src="https://i.imgur.com/9WjzQ98.png" alt="via Language Logo"/>
  </a>
</h1>

<p align="center">
  <img src="https://img.shields.io/github/license/XnLogicaL/via-lang" alt="License">
  <img src="https://img.shields.io/github/languages/top/XnLogicaL/via-lang" alt="Top Language">
  <img src="https://github.com/XnLogicaL/via-lang/actions/workflows/ci.yml/badge.svg" alt="CI">
</p>

<p align="center">
  This repository contains the <strong>via Programming Language</strong> source code, runtime, and tooling.
</p>

<p align="center">
  <a href="#introduction">Introduction</a>
    &nbsp;&bull;&nbsp;
  <a href="#features">Features</a>
    &nbsp;&bull;&nbsp;
  <a href="#installation">Installation</a>
    &nbsp;&bull;&nbsp;
  <a href="#credits">Credits</a>
</p>

> [!WARNING]
> This is an **experimental** project and currently not production-ready. Most features are under conception/development and may be incomplete/unstable. Implementations are subject to change as the project evolves. That said, by no means is it a non-serious project.

## Introduction

**via** is a modern, performant, multi-paradigm scripting language designed for **correctness**, **embeddability**, and **performance**.

## Yet another toy scripting language?

_Not quite_.

via isn't just _x with nicer syntax_ or _x but faster_, it is a language that aims to have **fully sealed compile-time invariants**.

A fair way to compared it to Lua would be: _via is to Lua what Rust is to C_.

### But what are invariants?

An **invariant** is a guarantee by the compiler that a **contract within the program will never be breached**. That is still quite vague, so it is demonstrated in the following examples:

In Lua, there is no way to guarantee _anything_ about parameters passed to functions at **compile-time**:

```lua
function foo(n, f)
    return f(n) / n
end

-- Every one of these is "legal", but all of them will fail at runtime:
foo() -- okay
foo(nil, nil) -- alright
foo(foo, foo) -- go for it!
```

There are **five possible ways in which this function can fail, completely undetectable at compile-time**. This is completely unacceptable because of the trivial nature of this function. You can imagine the possibilities of failure in larger, more complex code.

And no, **Luau does not fix this**. All it does is bolt on an half-baked type system that fails to resolve trivial types most of the time. This happens because Lua is fully-dynamic which makes its type system fundamentally incompatible with compile-time invariants, which in this case are:

- `n` **is a** `number`
- `n` **is not** `0`
- `f` **is a** `function`
- `f` **does not throw an error**
- `f` **returns** `number`

None of which can be truly validated without explicit runtime-checks.

Now the same function in via:

```rust
fn foo(
    n: float + !0,          // `n` is a float that is not equal to 0
    f: fn(float) -> float   // `f` is a function that takes no arguments and returns a float
) -> float {
    f(n) / n
}
```

---

In Lua, a **table** (the primary structurization mechanism of the language) can store absolutely _anything_. Even itself. They can also mutate their type as the program unfolds. This makes it an _absolute nightmare_ to statically analyze, and out sources type-safety to runtime once again:

```lua
local t = { 10 }            -- type: {number}
t[2] = "hello!!"            -- type: {number | string}
t["self"] = t               -- type: {[number]: number | string, self: <cycle>}
t["foo"] = function()       -- type: {[number]: number | string, self: <cycle>, foo: () -> string}
    return "bar"
end
```

As you can see, the type keeps getting more complex with each addition. There is absolutely no limit to this, which is problematic for many reasons.

If you were to try the same thing in via:

```zig
var t = [10]
t[1] = "hello!!"        // error: cannot assign value of type `string` to `[int]`
                        // |- help: type `[int]` does not implement trait `IndexAssign<I = int, string>`

t["self"] = t           // error: cannot index value of type `[int]` with `string`
                        // |- help: type `[int]` does not implement super-trait `Index<I = string>` required by trait `IndexAssign<I = string, [int]>`

t["foo"] = fn "bar"     // error: <same as above>
```

**via doesn’t even have “catch-all” tables**. Arrays and maps are split, and every index assignment must satisfy the type system. There is no silent type mutation, no runtime surprises, and no creeping complexity.

---

In Lua, `unknown` types are _allowed_ (and are everywhere), replacing _type-inference_ with _runtime uncertainty_:

```lua
local t = {}
local first = t[0] -- type: nil | unknown
```

Here is what it would look like in via:

```rust
let t = []
let first = t.first()   // error: cannot infer type of `t`
                        // |- help: explictily annotate `t` as `[T]`
```

As you can see, this does not compile. Because via does not have an `unknown` type. When the array is declared, the compiler infers its type as `[_]` (array with unknown inner type) and **continues under the assumption that it can solve its inner type later on in the code**. But since `first` does not introduce any new information about the inner type of the array, it is typed as `_ raise OutOfRange` (unknown type that may raise `OutOfRange`). And since there is never enough information to infer `_`, the compiler throws an error because the **"no unknown types"** invariant is violated.

---

In Lua, functions signatures have no reflection of the functions error invariant:

```lua
function foo(a)
    if a == 1 then
        error("oops")
    elseif a == 2 then
        error(10)
    else
        return a + a
    end
end

local result = foo(...) -- uhh error maybe???
```

There is **absolutely no way of knowing whether if this function throws or not**. The only way to find out is to `pcall` it, which is extremely inefficient. Now some Lua forks like Luau have a `never` type, but that still results in opaque error types, and is ambiguous between other non-continous control flow operations like _infinite loops_.

The problem this creates is the fact that you can never know if _any_ function _anywhere_ in your code base will fail or not, and due to that only way to enforce the **"errors must always be handled"** invariant becomes sprinkling `pcall` everywhere; making your code repetitive, cluttered, and fragile.

In via, the same function would look like:

```rust
struct Oops;

fn foo(a: int) -> int raise Oops {
    if a == 1 {
        raise Oops
    } else if a == 2 {
        raise 10    // error: cannot raise type `int` in function with raise type `Oops`
                    // |- help: add `int` to the raise type by changing raise clause to: `raise Oops | int`
    } else {
        return a + a
    }
}

let result = foo(...)?    // type: int | Oops

let n = result * 2;     // error: cannot multiply type `int | Oops` with `int`
                        // |- help: type `int | Oops` does not implement trait `Mul<int>`
                        //          explicitly handle variants of this union: type `int` and type `Oops`

let raw = foo(a)        // error: cannot propagate raise alternative `Oops` here
                        // |- help: explicitly handle the error by inserting a `?` after the function call
```

---

_These are just a few examples where Lua completely ignores compile-time invariants by design..._

### Why does it matter though?

After all, can't we just write better code?

_Yes, you absolutely can._ But there are **expensive tradeoffs**:

- **Surprise errors**: Would you rather your script _not compile_ or _die in production_? via is in favor of the _former_.
- **Code duplication**: End up sprinkling `assert` to check the type of every parameter, making your code cluttered, repetitive, and fragile.
- **Performance**: Runtime type-safety is **expensive**. Enforcing this invariant during compile-time **completely eliminates** this overhead.
- **Developer experience**: Dynamic type systems often produce confusing, opaque errors that are hard to trace, and tooling struggles to provide reliable feedback. via eliminates this chaos by enforcing correctness at **compile-time**, letting tools actually help you instead of fighting you.
- **Human nature**: Even the most careful programmer _will make mistakes_. Compile-time invariants catch errors humans _can't forsee_.

**In short**; via shifts the burden of correctness from **runtime** to **compile time**, letting you focus on the logic, not the bugs.

## Features

- Sealed invariants [\*](#but-what-are-invariants)
- Modern, clean and sane syntax and standard library
- Built-in types for strings, arrays, maps, tuples, optionals, unions, etc.
- Powerful type system and metaprogramming
- Advanced compiler hints & intrinsics
- Multi-paradigm design, including object-oriented and functional programming
- High performance
- Platform independence
- Rich Rust API
- No garbage collection

## Installation

### Cargo (recommended)

```sh
cargo install via
```

## Credits

- **@xnlogical** – Lead developer/maintainer
- **Kasen L. Daniels** – Name and banner design
