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

**via** is a scripting language that **refuses to run invalid programs**. It isn't *just* "x with nicer syntax" or "x but faster", but rather a language that aims to make invalid/erroneous programs fundamentally unrepresentable- with the goal of eliminating an entire class of bugs.

If _[insert dynamic language]_ has ever failed you in production, you're in the right place!

<details>
<summary><strong>A (humble) comparison to Lua</strong></summary>

### No compile-time checks

In Lua, there is no way to guarantee _anything_ about parameters, variables, etc. at compile time:

```lua
function foo(n, f)
    return f(someglobal) / n
end

-- Every one of these is "legal", but all of them will fail at runtime:
foo() -- okay
foo(32) -- ok
foo(nil, nil) -- alright
foo(foo, foo) -- go for it!
```

There are half a dozen possible ways in which this function can fail, inherently undetectable at compile time. This is an unfavorable tradeoff for the programmer because of the trivial nature of this function. You can imagine the permutations of potential failure in larger, more complex code.

And no, **Luau** does *not* fix this. It has an _optional_ type safety mechanism with `--!strict` which means the core of the problem is still there. This happens because Lua is fully dynamic which makes its type system fundamentally incompatible with compile-time invariants, which in this case are:

- `someglobal` **exists**
- `n` **is a** `number`
- `n` **is not** `0`
- `f` **is a** `function`
- `f` **does not throw an error**
- `f` **returns** `number`

None of which can be truly validated without explicit runtime checks.

Now the same function in via:

```rust
fn foo(n: float, f: fn(float) -> float) -> float
    raise DivisionByZero
{
    f(n) / n
}
```

### Dangers of catch-all data structures

In Lua, a table (being the primary structurization organ of the language) can store absolutely _anything_. Even itself. They can also mutate their type as the program unfolds. This makes them _practically impossible_ to statically analyze, and outsources type-safety into runtime once again:

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
let mut t = [10];
t[1] = "hello!!";        // error: cannot assign value of type `string` to `&mut int`

t["self"] = t;           // error: cannot index value of type `[int]` with `string`
                        // |- note: type `[int]` does not implement `Index<string>`

t["foo"] = fn "bar";     // error: <same as above>
```

Structural type polymorphism simply doesn't exist in via. Arrays and maps are split, and every index assignment must satisfy the type system. There is no silent type mutation, no runtime surprises, and no creeping complexity.

### Problems of `unknown`

In Lua, `unknown` types are allowed (and are everywhere), replacing type inference with runtime uncertainty:

```lua
local t = {}
local first = t[0] -- type: nil | unknown
```

Here is what it would look like in via:

```rust
let t = []              // [$0]
let first = t.first()   // error: type annotations needed
```

> [!NOTE]
> Unfortunately, the error message isn't exactly helpful. But that's due to a good reason, the compiler isn't there to determine intent.

As you can see, this does not compile. Because via does not have an `unknown` type. When the array is declared, the solver infers its type as `[$0]` by assigning a metavariable to the inner type to be solved later.

But because `first` does not introduce any new information about `$0`, it is typed as `$0 raise OutOfRange`. And since there is never enough information to infer `$0`, the compiler throws an error.

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

There is **absolutely no way of knowing whether if this function throws or not**. The only way to find out is to `pcall` it, meaning you have to outsource it to runtime.

Luau does have a `never` type, however it is opaque and also ambiguous between other non-continous control flow like infinite loops.

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

let result = foo(...)?;
let n = result * 2; // error: cannot multiply type `int | Oops` with `int`
                    //  note: type `int | Oops` does not implement trait `Mul<int>`

let raw = foo(a); // error: cannot propagate raise alternative `Oops` in callsite
                  //  help: explicitly handle the error by inserting a `?` after the function call
```

</details>

#### Why does it matter though?

After all, can't we just write better code?

Yes, you _absolutely can_. But there are _undeniable_ tradeoffs:

- **Surprise errors**: Would you rather your script not compile or die in production? via is in favor of the *former*.
- **Code duplication**: End up sprinkling `assert` to check the type of every parameter, making your code cluttered, repetitive, and fragile.
- **Performance**: Runtime type-safety is *expensive*. Enforcing this invariant during compile-time completely eliminates this overhead.
- **Developer experience**: Dynamic type systems often produce verbose, confusing, and opaque diagnostics that are hard to trace, and tooling struggles to provide reliable feedback. via eliminates this chaos by providing first class tooling and enforcing correctness at compile-time, letting tools actually help you instead of fighting you.
- **Human nature**: Even the most careful programmer _will_ inevitably make mistakes. Modern compilers are capable of catching invariant violations one can't even conceptualize.

Having to write better code as a counterweight to the flaws of a language is not good practice, to say the least.

## Features

> [!NOTE]
> via is constantly evolving, therefore putting a full list of features here would be premature. It should be noted that this is only a list of **core features**.

- [**Compile-time invariance:**](#but-what-are-invariants) No more debugging `expected number, got nil` in prod like it's still the '90s.
- **Modern, clean and sane syntax and standard library:** via takes syntax and design inspiration from beloved languages like **Rust** and **TypeScript**.
- **Powerful type system and metaprogramming:** via comes with a powerful set of builtin types, a [type-trait system](https://en.wikipedia.org/wiki/Trait_(computer_programming)), and [hygenic macros](https://en.wikipedia.org/wiki/Hygienic_macro) inspired by Rust.
- **Multi-paradigm design:** via supports multiple programming paradigms, including object-oriented and functional programming.
- **High performance:** Static typing opens the door for a multitude of non-trivial optimizations that dynamic languages simply cannot implement.
- **Platform independence:** via uses Rust as the compatibility layer; if your device can run Rust, it can run via.
- **No garbage collection:** via uses a combination of [reference counting](https://en.wikipedia.org/wiki/Reference_counting) and manual* [garbage collection](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)) to achieve efficient resource management.

## Installation

via currently doesn't have a release, therefore you must build it from source:

```sh
git clone https://github.com/via-lang/via.git
cd via
cargo build --release
```

## Credits

- **@xnlogical** – Lead developer/maintainer
- **Kasen L. Daniels** – Name and banner design
