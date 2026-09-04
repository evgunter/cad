---
id: load-path-stringifies-structured-refusals
kind: issue
title: The load path stringifies structured kernel refusals, contradicting the bindings' never-strings contract
status: open
opened: 2026-08-20
github: 694
refs: [561, 689, S107]
---

## From GitHub issue 694

Opened 2026-08-20; 0 comments.

Found by the style review of #689 (SMELL-SCAN Track B, row B5), reached **by execution** rather than by reading. Filed rather than fixed because `crates/editor-core/src/persist/` is outside that lane's scope column.

## The defect

`crates/editor-core/src/persist/wire.rs:155-160` — `impl Deserialize for Expr` takes the kernel's *structured* `DimensionError` and does:

```rust
format!("ill-dimensioned expression refused: {e:?}")
```

delivering it as a serde message. `crates/pncad-py/src/errors.rs:4` promises the opposite: *"typed exceptions carrying the structured error, never strings."*

So an ill-dimensioned expression in a save file surfaces in Python as `PersistError` with `variant="parse"` and the structured refusal Debug-formatted into the message. It is **not** a parse failure — the file parsed fine; the expression is dimensionally wrong — and the structure that would let a caller act on it is destroyed at the boundary.

## Reproduction

Executed against the built extension module at #689's head, using a hand-edited save file:

```
Mismatch { op: "add", left: Length, right: Angle }   -> PersistError, variant "parse"
MulNeedsScalar { left: Length, right: Length }       -> PersistError, variant "parse"
TrigNeedsAngle { op: "sin", found: Length }          -> PersistError, variant "parse"
UnknownDisplayUnit { symbol: "furlong" }             -> PersistError, variant "parse"
```

No new binding is needed; `pncad.load(text)` is the door.

## Why it went unnoticed, which is the more useful half

`WireExpr::rebuild()` (`wire.rs:115`) calls `Expr::add`, `sub`, `mul`, `div`, `sin`, `cos`, `tan`, `atan2`, `min`, `max` and `literal_with_unit` — **the operator builders**. #689 had argued, and shipped on the published Python surface, that the kernel's `DimensionError` reaches Python through exactly one door (`Expr::literal`) because *"no bound door binds the operator builders"*. That is true of the **authoring** doors and false of the **deserialization** doors.

The general shape, which is the class worth naming: **reachability argued from the authoring doors while ignoring the deserialization doors.** Every `Deserialize` impl in `wire.rs` re-runs a smart constructor, so every kernel refusal type reachable from a smart constructor is reachable from `load`.

## Scope of the class

Two sweeps a fix should run rather than fixing this one site:

1. Every `format!("{err:?}")` (or equivalent) used as a user-facing message. Known instances: `crates/pncad-py/src/py/value.rs:203` and `:710`, `crates/pncad-py/src/py/flush.rs:166`, plus every `serde` `Error::custom` in `crates/editor-core/src/persist/`.
2. Every tag function in `crates/pncad-py/src/tags.rs` — `persist_error_tag`, `edit_error_tag`, `path_error_tag` — should be asked the same reachability question that `literal_refusals_*` was asked, from the load path as well as the authoring path.

## Relationship to #689

#689 is correcting its own false reachability statements and re-premising its trigger test as part of its fix pass; that is the half inside its scope. The *decision* it made — not to rename the published Python `DimensionError` — survives on narrower grounds, since no bound door raises the Python `LiteralError` class for a genuine dimension mismatch today. This issue is the remaining half: the load path's refusals are misrouted and destringified nowhere.

Refs #561 (the Python refusal-tag values pinned nowhere), which is adjacent but not the same defect — no tag spelling is wrong here; a structured value is being discarded.

## Home

LIB: the contract broken is the bindings' own (`crates/pncad-py/src/errors.rs`), the reproduction is through `pncad.load`, and the LIB register fold placed this issue in its **category B** (bindings-parity items the audit test structurally cannot see).
