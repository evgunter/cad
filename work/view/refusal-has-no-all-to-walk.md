---
id: refusal-has-no-all-to-walk
kind: issue
title: Refusal has no ALL value, so every property over the vocabulary is a hand-maintained list
status: open
opened: 2026-09-05
---


Named by `refusal-edit-arm-doubles-a-prefix-and-splits-one-mistake` as
"its own small design question" and left there; filed here because the
unit that closed that item ran straight into it.

## What happens

`crates/viewer/tests/panel_edits.rs`'s `refusals_render_as_sentences`
asserted a property over the whole `Refusal` vocabulary — renders as
prose, never as a debug dump — while exercising ONE arm. It stayed
green for as long as `Refusal::Edit` rendered a `{:?}`-quoted parameter
name into the status line, because the arm it walked was `Io`.

That row now walks five arms, each through a real op. It is still a
HAND-MAINTAINED list: `Refusal` has no `ALL`, so a new arm joins the
property by someone remembering to add it, which is the same shape as
the clearing walk `session-clearing-walk-is-hand-maintained-three-times`
closed one layer down.

## Why it is not simply "add an ALL"

Every other vocabulary in this crate that has an exhaustiveness guard
gets it from a `match` the compiler checks —
`SessionOp::permitted_during_value_gesture` is the model: a fortieth
operation cannot be added without answering for it, because the table
is an exhaustive match rather than a list. `Refusal`'s arms carry
PAYLOADS (a node id, a name, a boxed `EditError`), so an `ALL` would
have to mint a representative value per arm, and a representative
payload is a fixture decision, not a fact about the type.

Two shapes worth costing before either is built:

- an exhaustive `match` in the test that maps each arm to a sample —
  the compiler then refuses a new arm until it is sampled, and the
  samples stay where the property is asserted;
- a `Refusal::sample_of_each()` behind `cfg(test)` or a test-support
  feature, which puts the same match in the crate and lets more than
  one suite walk it.

The first is cheaper and keeps fixtures out of the shipped type; the
second is what `viewer-const-all-tables-have-no-exhaustiveness-guard`
is about more generally, so the two should be decided together.
