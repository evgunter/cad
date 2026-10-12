---
id: boolean-given-pairs-glue-unmeasured-for-any-rust-caller
kind: issue
title: "BooleanInputs::given is a public field: any Rust caller of boolean_op_with glues its pairs unmeasured"
status: open
opened: 2026-10-11
priority: P2
cost: E
---

## What

`BooleanInputs::given` (crates/topo/src/boolean/mod.rs:1047) is the union
fold's channel: each fold step hands the kernel the verdicts the
pairwise pass decided, and under F-2(A) the kernel seeds its glue from
them unmeasured (`given_verdicts`, crates/topo/src/boolean/mod.rs:5285).
The field is public, and `topo::boolean_op_with`
(crates/topo/src/boolean/ops.rs:482) is public, so any Rust caller can
hand in face pairs no pass decided and have them glue with nothing
measured. What remains is `Glue::of`'s check that each relation glues
and the op door's key check.

Before stage 4 F a given pair was re-verified at the op
(`verify_declared_contacts`); F-2(A) is what makes the open field a
fail-loud gap. Its doc says the façade never sets it; nothing enforces
that.

Found by stage 4 F's review (M4).

## Owed

Gate the channel to its one caller: a constructor-gated type or token
that only `editor_core`'s union fold can mint (or the fold's step moved
behind a kernel door that takes the pass's verdicts itself), so a pair
boolean cannot be handed unmeasured glue.
