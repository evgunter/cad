---
id: mixfraction-has-two-constructors-where-one-would-do
kind: issue
title: MixFraction's private literal constructor is redundant now that new is a const fn
status: review
opened: 2026-09-20
branch: vgeom/deletions
pr: 3027
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

## Closed

Taken by `vgeom/deletions`. **`MixFraction::literal` is gone and the
fifteen registry weights go through `MixFraction::new(w).unwrap()`.**
None of the three things the row named as able to defeat it applies,
and that was established by compiling rather than by reading.

### The three defeaters, tried

**A registry site that is not a `const` item** — none. All fifteen
calls sit inside three `const Theme` items (`DARK_NEUTRAL`,
`LIGHT_NEUTRAL`, `COLORBLIND_SAFE`), and the compiler says so rather
than the reader: with one weight changed to `1.5`,

```
error[E0080]: evaluation panicked: called `Option::unwrap()` on a `None` value
   --> crates/viewer/src/theme.rs:371:14
    |
371 |     ambient: MixFraction::new(1.5).unwrap(),
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `theme::DARK_NEUTRAL` failed here
```

`cargo check -p viewer --features app` exits **101**. `f32::NAN` in
the same slot gives the identical error. A site that was not
const-evaluated would have compiled instead, so this one command
answers the whole question.

**An allow that has to sit somewhere broader** — no allow is needed
anywhere. `cargo clippy -p viewer --features app --all-targets --
-D warnings` exits **0** over all fifteen, with nothing added:
`clippy::unwrap_used` does not fire on a const-evaluated `unwrap`.
The control matters more than the pass, because a lint that was off
would give the same green: a throwaway
`pub fn probe_runtime(x: f32) -> Self { Self::new(x).unwrap() }`
added to the same `impl` produces

```
warning: used `unwrap()` on an `Option` value
  --> crates/viewer/src/theme.rs:139:9
```

so the lint is live in this file and const-exempt.

**Another row keying on `literal` by name** — one, and it is
**closed**: `the-shader-encodes-a-mark-strength-nothing-bounds`
(#2808) names `literal` in its `## Closed` prose. A closed item is a
record of what a PR did, not a guard over the tree, so it is left as
written.

### What the change buys beyond one fewer door

The row's three findings against `literal` stop existing rather than
being answered, and the third turns into a mechanism:

- **The `assert!` held by nothing** — re-measured on the merge base
  with the `assert!` deleted and the registry untouched:
  `cargo nextest run -p viewer --features app --no-fail-fast
  -E 'test(theme)'` is **22/22 green, exit 0**. (The row recorded
  18/18; the `theme::` population is 22 today. The count moved, the
  finding did not.)
- **A `compile_fail` doctest cannot reach a private item** — moot;
  the surviving door is `pub`, and the build error above is now
  produced at the registry rather than inside a private helper.
- **`literal`'s doc asserted an invariant nothing enforced** — that
  *"every call sits in a `const` item"*, whose violation turned an
  advertised build error into a runtime panic. That is now the lint's
  job: a `MixFraction::new(x).unwrap()` written into a non-`const`
  body is `clippy::unwrap_used` under CI's `-D warnings`, measured
  above. An unenforced sentence became a mechanical guard, which is
  the part of this change that is not merely a deletion.

What is lost is the `assert!`'s message, *"a mix fraction is in
[0, 1]"*, in exchange for `E0080` naming the line and the constant.
The message was held by nothing and the diagnostic is not.

**Where**: `crates/viewer/src/theme.rs` — `MixFraction::literal`
deleted, the type's two-doors paragraph and `new`'s `const` paragraph
rewritten to the one door, fifteen registry weights re-spelled.
