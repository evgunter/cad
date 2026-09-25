---
id: node-error-kind-renders-the-slot-id-through-debug
kind: issue
title: NodeErrorKind's two slot renderings dump SlotId where SlotId::label is the prose spelling
status: closed
opened: 2026-09-16
priority: P1
cost: E
closed: 2026-09-24
pr: 3141
---


Filed by EDIT's `edit/error-prose` unit, which swept the
`{binding:?}`-inside-`impl Display` class across `editor-core/src` and
repaired every site on its own fence. `crates/editor-core/src/eval/mod.rs`
is WIRE's by its path list, so these two are named here rather than
edited there.

## The finding

`NodeErrorKind`'s `Display` renders a `SlotId` through `Debug` twice:

```rust
"the expression at slot {slot:?} failed to evaluate: {source}"
"internal: the wiring expected slot {slot:?}, which is absent"
```

`SlotId` has a prose spelling of its own — `SlotId::label`
(`crates/editor-core/src/node.rs`), which reads a component slot as its
family plus its axis (`origin x`) and a profile slot as its address plus
its role (`loop 0 step 2 · centre x`). Through `Debug` the same two
sentences read `slot Origin(X)` and `slot Profile { loop_: 0, step: 2,
arg: CenterX }`: the variant identifier and, for the profile arm, the
struct braces — the two fingerprints
`crates/editor-core/tests/display_contract.rs`'s header names.

The brace half is not cosmetic. `crates/pncad-py/src/prose_census.rs`'s
`KNOWN_BRACED` carries this site (count 2), and its module docs say what
a braced payload costs at the binding: `crate::errors::reads_as_prose`
rejects `" { "` and `crate::py::typed_err` asserts it live under release,
so a `NodeErrorKind::Expr` or `MissingSlot` over a profile slot **panics the binding
at the arm meant to refuse gracefully**.

## What the repair is

The same one EDIT's unit applied at five sites — `slot.label()` in place
of `{slot:?}` — plus striking this row's entry from `KNOWN_BRACED`, which
compares in both directions and so reds until it is struck. The unit left
`crates/editor-core/src/eval/mod.rs` alone and the roster entry now cites
this file as the reason it stands.

The sibling sites it did repair, for the shape: `EditError` (six arms),
`ProgramFault`, `ProgramRefusal` and `resolve::Diagnosis`.

## Closed

PR 3141. Both arms of `NodeErrorKind`'s `Display` (`Expr`, `MissingSlot`,
`crates/editor-core/src/eval/mod.rs`) render the slot through
`SlotId::label`, so a profile slot reads `loop 0 step 2 · centre x`. The
`KNOWN_BRACED` row (`crates/pncad-py/src/prose_census.rs`) is struck, and
the census's two-way comparison is green without it. The guard added is
`display_contract::a_node_refusal_names_its_slot_by_its_label`, which
builds both arms over a `SlotId::Profile` and bans its variant words and
braces. Under the old `{slot:?}` spelling it goes red.

Sweep: `:?}` inside `NodeErrorKind`'s `Display` impl. Three hits: the two
fixed here, and `SeedPinnedSection`'s `{:?}` over `param.0`, which is a
`String` and is quoted on purpose. That one is not this shape.
