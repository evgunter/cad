---
id: mixfraction-has-two-constructors-where-one-would-do
kind: issue
title: MixFraction's private literal constructor is redundant now that new is a const fn
status: open
opened: 2026-09-20
---


Found by the VIEW orchestrator while merging #2808, after that PR's own
review round had already made `MixFraction::new` a `const fn`.

## The claim the type now carries, and why it is false

`theme.rs` gives `MixFraction` two constructors and says why:

> A `Result` cannot be unwrapped in a `const`, and that is the whole
> reason there are two.

That was true when `new` returned an `Option` from a non-`const` fn.
It is **not true now**. `Option::unwrap` has been const-stable since
Rust 1.83 and this workspace pins 1.97, so a `const` item may call
`new(x).unwrap()` and a bad weight fails **at build time**, which is
exactly what `literal`'s `assert!` buys.

Executed under this toolchain rather than reasoned about:

```rust
struct F(f32);
impl F {
    const fn new(x: f32) -> Option<Self> {
        if x >= 0.0 && x <= 1.0 { Some(F(x)) } else { None }
    }
}
const GOOD: F = F::new(1.5).unwrap();
```

```
error[E0080]: evaluation panicked: called `Option::unwrap()` on a `None` value
 --> const GOOD: F = F::new(1.5).unwrap();
```

Same class of build error, from the one public door.

## What removing `literal` would collect

Three findings the review of #2808 raised against `literal`
specifically, all of which stop existing rather than being answered:

- **Its `assert!` is held by nothing.** Deleting the `assert!` with the
  registry untouched reds **no row** — 18/18 green, exit 0, measured.
  The compile-time property is asserted in three doc comments and the
  PR body and guarded by nothing mechanical.
- **It cannot be guarded the usual way.** A `compile_fail` doctest
  cannot reach a private item, and `literal` is private on purpose, so
  the repo's one mechanism for pinning a build error is closed to it.
- **Its doc asserts an invariant nothing enforces** — *"The argument is
  a literal, never an input: every call sits in a `const` item below."*
  Nothing checks either half, and because `literal` is a `const fn`, a
  later call from a non-`const` body inside `theme.rs` compiles and
  turns the advertised build error into a runtime panic, which is the
  one failure mode the whole design is written against.

## The friction a taker owes an answer to

The workspace denies the `unwrap`/`expect`/`panic` clippy family under
CI's `-D warnings`, so the registry would need an
`#[allow(clippy::unwrap_used, reason = …)]`. The orchestrator's read is
that one allow with an honest reason — *a const-evaluated `unwrap` is a
build error, not a runtime one* — is a clearly better trade than a
private second constructor, an unguarded `assert!`, three doc
paragraphs and two claims nothing enforces. **But that read was made
from outside the file**, and three things could defeat it: a registry
site that is not a `const` item at all and would take a real runtime
`unwrap`; an allow that has to sit somewhere broader than the registry
constants; or another row keying on `literal` by name.

Establish which, if any, applies. An honest *"tried it, here is what
bit"* closes this row as well as the deletion does.

## Not done in #2808, and why

That PR sat three days while `main` moved past a hundred merges. The
simplification is an improvement, not a repair — the type is sound
either way — so it was filed rather than allowed to hold a correct
change out of the tree.

**Where**: `crates/viewer/src/theme.rs`, `MixFraction::new` and
`MixFraction::literal`, and the three registry constants below them.
