 <h1 align="center">
  <a href="https://github.com/via-lang/via">
    <img src="https://i.imgur.com/9WjzQ98.png" alt="via Language Logo"/>
  </a>
</h1>

<p align="center">
  <img src="https://img.shields.io/github/license/via-lang/via" alt="License">
  <img src="https://img.shields.io/github/languages/top/via-lang/via" alt="Top Language">
  <img src="https://github.com/via-lang/via/actions/workflows/ci.yml/badge.svg" alt="CI">
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

via isn't just "x with nicer syntax" or "x but faster", it is a language that aims to have **full compile-time safety and correctness using well-defined invariants**.

A fair way to compared it to Lua would be: _via is to Lua what Rust is to C_.

### But what are invariants?

An **invariant** is a guarantee by the compiler that a **contract within the program will never be breached**. That is still quite vague, so it is demonstrated by comparison to [Lua](https://lua.org)/[Luau](https://luau.org) in the following points paragraphs.

### No compile-time checks

In Lua, there is no way to guarantee _anything_ about parameters passed to functions at **compile-time**:

```lua
function foo(n, f)
    return f(n) / n
end

-- Every one of these is "legal", but all of them will fail at runtime:
foo() -- okay
foo(32) -- ok
foo(nil, nil) -- alright
foo(foo, foo) -- go for it!
```

There are **five possible ways in which this function can fail, inherently undetectable at compile-time**. This is unacceptable because of the trivial nature of this function. You can imagine the permutations of potential failure in larger, more complex code.

And no, **Luau does not fix this**. All it does is bolt on an half-baked type system that fails to resolve trivial types most of the time. This happens because Lua is fully dynamic which makes its type system fundamentally incompatible with compile-time invariants, which in this case are:

- `n` **is a** `number`
- `n` **is not** `0`
- `f` **is a** `function`
- `f` **does not throw an error**
- `f` **returns** `number`

None of which can be truly validated without explicit runtime checks.

Now the same function in via:

```rust
fn foo(n: float, f: fn(float) -> float) -> float {
    assert n != 0; // this is technically a runtime check, but it is explicit and has intent
    f(n) / n // since we asserted that `n != 0`, the type solver can safely assume division by zero is impossible
}
```

### Dangers of catch-all data structures

In Lua, a **table** (the primary structurization mechanism of the language) can store absolutely _anything_. Even itself. They can also mutate their type as the program unfolds. This makes it **practically impossible** to properly statically analyze, and outsources type-safety into the language runtime once again:

```lua
local t = { 10 }            -- type: {number}
t[2] = "hello!!"            -- type: {number | string}
t["self"] = t               -- type: {[number]: number | string, self: <cycle>}
t["foo"] = function()       -- type: {[number]: number | string, self: <cycle>, foo: () -> string}
    return "bar"
end
```

As you can see, the type keeps getting more complex with each addition. There is absolutely no limit to this, which can get out of control extremely fast.

If you were to try the same thing in via:

```zig
var t = [10]
t[1] = "hello!!";        // error: cannot assign value of type `string` to `[int]`
                        // |- help: type `[int]` does not implement trait `IndexAssign<I = int, string>`

t["self"] = t;           // error: cannot index value of type `[int]` with `string`
                        // |- help: type `[int]` does not implement super-trait `Index<I = string>` required by trait `IndexAssign<I = string, [int]>`

t["foo"] = fn "bar";     // error: <same as above>
```

**via doesn’t even have “catch-all” tables**. Arrays and maps are split, and every index assignment must satisfy the type system. There is no silent type mutation, no runtime surprises, and no creeping complexity.

### Problems of `unknown`

In Lua, `unknown` types are allowed (and are everywhere), replacing type inference with runtime uncertainty:

```lua
local t = {}
local first = t[0] -- type: nil | unknown
```

Here is what it would look like in via:

```rust
let t = []
let first = t.first()   // error: type annotations needed
```

> [!NOTE]
> Unfortunately, the error message isn't exactly helpful. But that's due to a good reason, as the compiler cannot exactly determine intent here.

As you can see, this does not compile. Because via does not have an `unknown` type. When the array is declared, the compiler infers its type as `['0]` (array with unknown inner type) and **continues under the assumption that it can solve its inner type later on in the code**.

But since `first` does not introduce any new information about the inner type of the array, it is typed as `'0 raise OutOfRange` (unknown type that may raise `OutOfRange`). And since there is never enough information to infer the underlying type of metavar `'0`, the compiler throws an error because the **"no unknown types"** invariant is violated.

### Unmarked erroneous behavior

In Lua, functions signatures have no reflection of the functions erroneous behavior policy:

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

There is **absolutely no way of knowing whether if this function throws or not**. The only way to find out is to `pcall` it, which is not the right tool for the job.

Some Lua forks  like Luau do have a `never` type, but that still results in opaque error types, and is ambiguous between other non-continous control flow operations like _infinite loops_.

The problem this creates is the fact that you can never know if any function anywhere in your code base will fail or not, and due to that only way to enforce the **"errors must always be handled"** invariant becomes sprinkling `pcall` everywhere; making your code repetitive, cluttered, and fragile.

In via, the same function would look like:

```rust
struct Oops;

fn foo(a: int) -> int raise Oops {
    if a == 1 {
        raise Oops
    } else if a == 2 {
        raise 10    // error: cannot raise type `int` in callsite with raise type `Oops`
                    // |- help: add `int` to the raise type by changing raise clause to: `raise Oops | int`
    } else {
        return a + a
    }
}

let result = foo(...)?    // type: int | Oops

let n = result * 2;     // error: cannot multiply type `int | Oops` with `int`
                        // |- help: type `int | Oops` does not implement trait `Mul<int>`
                        //          explicitly handle variants of this union: type `int` and type `Oops`

let raw = foo(a)        // error: cannot propagate raise alternative `Oops` in callsite
                        // |- help: explicitly handle the error by inserting a `?` after the function call
```

> [!NOTE]
> These are just a few of many places in which Lua (and by extension other dynamic languages) completely disregard guaranteed compile-time correctness by design.

### Why does it matter though?

After all, can't we just write better code?

_Yes, you absolutely can._ But there are **expensive tradeoffs**:

- **Surprise errors**: Would you rather your script not compile or die in production? via is in favor of the **former**.
- **Code duplication**: End up sprinkling `assert` to check the type of every parameter, making your code cluttered, repetitive, and fragile.
- **Performance**: Runtime type-safety is **expensive**. Enforcing this invariant during compile-time completely eliminates this overhead.
- **Developer experience**: Dynamic type systems often produce verbose, confusing, and opaque diagnostics that are hard to trace, and tooling struggles to provide reliable feedback. via eliminates this chaos by providing first class tooling and enforcing correctness at compile-time, letting tools actually help you instead of fighting you.
- **Human nature**: Even the most careful programmer **will inevitably make mistakes**. The compiler is capable of catching invariant violations we can't even conceptualize.

In short; via shifts the burden of correctness from **runtime** to **compile time**, letting you focus on the logic, not the bugs.

## Features

> [!NOTE]
> via is constantly evolving, therefore putting a full list of features here would be foolish. It should be noted that this is only a list of **core features**.

- [**Compile-time invariance:**](#but-what-are-invariants)
    - No more debugging `expected number, got nil` in prod
- **Modern, clean and sane syntax and standard library**
    - via takes syntax and design inspiration from beloved languages like **Rust** and **TypeScript**.
- **Powerful type system and metaprogramming**
    - via comes with a powerful set of builtin types, a [type-trait system](https://en.wikipedia.org/wiki/Trait_(computer_programming)), and [hygenic macros](https://en.wikipedia.org/wiki/Hygienic_macro) inspired by Rust.
- **Multi-paradigm design:**
    - via supports multiple programming paradigms, including object-oriented and functional programming.
- **High performance:**
    - Static typing opens the door for a multitude of non-trivial optimizations that dynamic languages simply cannot implement.
- **Platform independence:**
    - via uses Rust as the compatibility layer, if your device can run Rust, it can run via.
- **No garbage collection:**
    - via uses a combination of [RC](https://en.wikipedia.org/wiki/Reference_counting) and fully manual [garbage collection](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)) to ensure proper resource management.

## Installation

via currently doesn't have a release, therefore you must build it from source:

```bash
git clone https://github.com/via-lang/via.git
cd via
cargo build --release
```

## Credits

- **@xnlogical** – Lead developer/maintainer
- **Kasen L. Daniels** – Name and banner design
