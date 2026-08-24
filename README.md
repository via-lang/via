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
  This repository contains the Via Programming Language source code, runtime, and tooling.
</p>

<p align="center">
  <a href="#introduction">Introduction</a>
    &nbsp;&bull;&nbsp;
  <a href="#roadmap">Roadmap</a>
    &nbsp;&bull;&nbsp;
  <a href="#installation">Installation</a>
    &nbsp;&bull;&nbsp;
  <a href="#credits">Credits</a>
</p>

> [!WARNING]
> This project has been paused until further notice. (due to personal reasons)

> [!WARNING]
> This is a **highly experimental** project. Most features are a WIP; they may be incomplete, missing, unstable, or broken. 
>
> * **High Instability:** Code is provided as-is, if it compiles/works at all.
> * **Breaking Changes:** Implementations are subject to radical change without notice as everything evolves.
> * **Incomplete Design:** Significant architectural and design questions are still completely unanswered.
>
> While this is a serious & dedicated project, it is currently *not* production-ready. Proceed with extreme caution.

## Introduction

**via** is a modern scripting language designed for complex, high-performance sandbox environments where correctness and speed are the absolute top priority.
It completely eliminates runtime type checking overhead, GC pauses, and excessive memory usage by providing a powerful, flexible static type system - while maintaining a footprint lightweight enough to embed.
Although it isn't designed for standalone use, it still comes with batteries included! 🔋⚡️

The compiler is built with [salsa](https://salsa-rs.github.io), an incremental computation engine;
along with [rowan](https://github.com/rust-analyzer/rowan), a lossless syntax tree library - the same foundations behind [rust-analyzer](https://rust-analyzer.github.io/) & similar to the ones behind [rustc](https://github.com/rust-lang/rust).
The result is a compiler designed to recheck only what changed, keeping iteration fast even as a project grows.
The language server is also fused with the compiler, making total integration as smooth as can be! 

### What's different?

> [!NOTE]
> The following comparisons are limited to statically-typed languages only,
> given the statically-typed vs dynamically-typed language debate has been largely settled in favor of statically-typed languages.

<details>
<summary>Luau comparison</summary>

### Dynamic structural typing

Tables (being the primary data structure of the language) can take absolutely _any shape_.
They can also mutate their type as the program unfolds:

```luau
local t = { 10 }            -- type: {number}
t[2] = "hello!!"            -- type: {number | string}
t["self"] = t               -- type: {[number]: number | string, self: <cycle>}
t["foo"] = function()       -- type: {[number]: number | string, self: <cycle>, foo: () -> string}
    return "bar"
end
```

As you can see, the type keeps getting more complex with each addition.

If you were to try the same thing in via:

```rust
let mut t = [10];
t[1] = "hello, world";  // error: cannot assign value of type `String` to `&mut Int`

t["self"] = t;          // error: cannot index value of type `[Int]` with `string`
                        // |- note: type `[Int]` does not implement `Index<String>`
```

via strictly seperates dictionaries and sturctural types - which are immutable, compile-time objects - and rejects such dynamic behavior.

### Metatables

Metatables are a way to immitate inheritance hierarchies.
They link two tables together at runtime:

```luau
local Point2D = {}
Point2D.__index = Point2D

function Point2D:__tostring()
  return `Point2D({self.x}, {self.y})`
end

function Point2D:__call(x: number, y: number)
  return setmetatable({ x=x, y=y }, Point2D)
end

function Point2D.is(object: any)
  return getmetatable(object) == Point2D
end

tostring(Point2D(0, 1)) // "Point2D(0, 1)"
Point2D.is({}) // false
```

This pattern may look innocent - or even useful - but in reality it is practically impossible to analyze statically.
This leads to various issues, including but not limited to: LSP performance degradation, poor intellisense, and overall reduced runtime performance.

It can also easily be tricked into producing false positives when it comes to the validity of a given object, as demonstrated below:

```luau
Point2D.is(setmetatable({}, Point2D)) -- true !!
```

### any

`any` can be thought of as a union between all types except `nil` and `unknown`.
Any and all types can implicitly convert to `any`:

```luau
function foo(x: any)
end

foo(1) -- Valid
foo("hi") -- Valid
foo({}) -- Valid
```

Although it exists largely for backwards compatability, it is still a major source of unsoundness - as it actively destroys type information, with no way to get it back during compile time.
The only remedy to this is `typeof()`, which is runtime-only and retrieves partial type information at best.

### unknown

`unknown` is a placeholder type for types that the solver cannot determine:

```luau
local t = {}
local first = t[0] -- type: nil | unknown
```

Here is what it would look like in via:

```rust
let t = [];              // [$0]
let first = t.first();   // error: type annotations needed
```

As you can see, this does not compile because via rejects indeducable [metavariables](https://en.wikipedia.org/wiki/Metavariable) - or in other words, it does not have an `unknown` type. When the array is declared, the solver assigns it the type `[$0]` - a metavariable that infects any site where the array is used - until there is enough information for it to be substituted with a concrete type.

Because `first` does not introduce any new information about `$0`, it is typed as `$0 raise OutOfRange`.
And since there is never enough information to infer `$0`, the compiler rejects this code.

### Opaque errors

Function signatures have no reflection of the function's error behavior:

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

local result = foo(...) -- error, maybe?
```

There is **absolutely no way of knowing whether if this function throws or not**.

The `never` type is a partial substitute, but it still is not sufficient,
as it is still opaque between various infinite-yield control flow cases and actual errors.

The problem this creates is the fact that you can never know if any function anywhere in your code base will fail - without vetting each function individually -
and due to that only way to handle all errors becomes sprinkling `pcall` everywhere; making your code repetitive, cluttered, and fragile.

In via, the same function would look like:

```rust
struct Error;

fn foo(a: Int) -> Int raise Error {
    match a {
        1 => raise Error,
        2 => raise -1, // error: cannot raise type `Int` here
        _ => 0,
    }
}

fn main() {
    let result = foo(...)?;
    let n = result * 2; // error: cannot multiply type `Int | Oops` with `Int`
                        //  note: type `Int | Oops` does not implement trait `Mul<Int>`
    
    let raw = foo(a); // error: cannot propagate error `Oops` in callsite
                      //  help: explicitly handle the error by inserting a `?` after the function call
}
```

</details>

<details>
<summary>TypeScript comparison</summary>

### Absence of nominal typing

TypeScript completely lacks the mechanisms for nominal typing, as demonstrated below:

```ts
interface Point2D {
  x: number;
  y: number;
}

interface Vector2D {
  x: number;
  y: number;
}

function translate(point: Point2D, velocity: Vector2D): Point2D {
  return {
    x: point.x + velocity.x,
    y: point.y + velocity.y
  };
}

const location: Point2D = { x: 10, y: 20 };
const wind: Vector2D = { x: 5, y: -2 };

// OK!! Accidental mix-up compiles successfully
// We passed a Point where a Vector was expected, and vice versa
const incorrectTransform = translate(wind, location);
```

Instead, the language relies on marker fields to distinguish otherwise structurally similar types:

```ts
interface Point2D {
  x: number;
  y: number;
  _nominal_Point2D: unique symbol;
}

interface Vector2D {
  x: number;
  y: number;
  _nominal_Vector2D: unqiue symbol;
}
```

This approach, however, is still not 100% sound, harmful to ergonomics, and is usually memory inefficient as the runtime must tag each instance of these objects - which are often just dictionaries, leading to further memory inefficiency.

via fixes this by making all structural types nominal by default:

```rust
struct Point2D {
  pub x: Float,
  pub y: Float
}

struct Vector2D {
  pub x: Float,
  pub y: Float
}

fn translate(point: Point2D, velocity: Vector2D) -> Point2D {
  Point2D {
    x: point.x + velocity.x,
    y: point.y + velocity.y
  }
}

fn main() {
  let location = Point2D { x: 3, y: 4 };
  let wind = Vector2D { x: -2, y: 1 };
  
  // error: expected Point2D, got Vector2D as argument 0
  let result = translate(wind, location);
}
```

</details>

#### Why does it matter?
##### After all, can't we just write better code?

Yes, you _absolutely can_. But there are major tradeoffs:

- **Surprise errors**: Would you rather your program not compile or crash in production? via is in favor of the _former_.
- **Code duplication**: End up sprinkling `assert` to check the type of every parameter; making your code cluttered, repetitive, and fragile.
- **Performance**: Runtime type safety is _expensive_. Checking types entirely during compile time eliminates this overhead entirely.
- **Developer experience**: Dynamic type systems often produce verbose, confusing, and opaque diagnostics that are hard to trace, and tooling struggles to provide reliable feedback. This chaos is eliminated by first class tooling and compile time correctness, letting tools actually help instead fight.
- **Human nature**: Even the most talented, careful, and experienced programmer will inevitably make a mistake - computers are fundamentally advantaged when it comes to bookkeeping invariants.

Having to write better code to counterweigh the flaws of a language is simply feeding into the problem itself. It's not a solution but rather simply technical debt.

### Roadmap

## Installation

```console
# CLI
cargo install via_cli
# Library
cargo add via
```

## Contribution

> [!WARNING]
> A quick heads-up if you are looking to contribute: the author (I, @xnlogical) frequently hoards large, incomplete changes locally before pushing them. The project (as I deem) is not mature enough to warrant proper version control practices.
>
> Because of the aforementioned reasons, upstream development is largely asynchronous - meaning contributors should expect massive merge conflicts, or more broadly: _a life of pain_.
>
> If you are still brave enough to navigate the mess, all kinds of contributions are more than welcome!

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## AI

AI generation within this project is strictly limited to macro implementations (`via_macros`), planning, and general advisory.

## Credits

- **@xnlogical** – Lead developer/maintainer
- **Kasen L. Daniels** – Name and banner design
