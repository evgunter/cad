---
id: boolean-given-pairs-glue-unmeasured-for-any-rust-caller
kind: issue
title: BooleanInputs::given is a public field: any Rust caller of boolean_op_with glues its pairs unmeasured
status: review
opened: 2026-10-11
priority: P2
cost: E
branch: flush/given-gate
pr: 5002
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

## Measured

On main `3ad2c1bf`, from `topo`'s integration tests (an outside
crate): `A = [0,1]³`, `B = [0.25,0.75]² × [1,2]` resting on `A`'s top.
A `BooleanInputs` built by hand with one `GivenPair` went through
`topo::boolean_op_with` unrefused:

- `(A top, B bottom, SameOriented)`, the wrong relation for an opposed
  touch: union and subtract built (the intersection is empty either
  way), and neither recorded a coincidence row for the glue (the
  decided run records one).
- `(A top at z = 1, B bottom at z = 0.5)`, two planes 0.5 apart: all
  three ops built.
- A pair whose faces do not touch while the true touch went unlisted
  (and `given: Some(vec![])`) refused, but only through the door's
  unglued-coincidence backstop
  (`Escalated … carrier_unglued_coincidence`), not by checking the
  given list.

## Fix

A given pair can no longer be stated. `GivenPair`'s fields are private
and its one constructor is `GlueVerdict::given_at`. A `GlueVerdict` is
minted only by the kernel, from the face-pair rows of a boolean's own
coincidences (`GlueVerdict::of_rows`, `crates/topo/src/boolean/mod.rs`),
and handed out on `BooleanBody::verdicts`. `verbs` carries the list on
`VerbRecord::Boolean`. The union fold
(`crates/editor-core/src/eval/wire.rs`, `pair_verdicts` and
`given_pairs`) reads the judgement's verdicts there rather than
re-deriving them from the rows, and re-keys each verdict's A face
through its lineage. `Glue::of` is infallible, because a verdict's
relation always glues.

The pin is `GivenPair`'s `compile_fail` doc-test, which tries the
measurement's spelling. `given_pairs_are_the_folds_alone.rs` adds a
second pin: a rest touch is the union's one verdict, and handed back,
it builds the decided body with no row of its own.

What stays open: a verdict is not bound to the bodies it judged
(`a-glue-verdict-is-not-bound-to-the-bodies-it-judged`).
