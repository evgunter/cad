---
id: boolean-given-pairs-glue-unmeasured-for-any-rust-caller
kind: issue
title: BooleanInputs::given is a public field: any Rust caller of boolean_op_with glues its pairs unmeasured
status: closed
opened: 2026-10-11
priority: P2
cost: E
branch: flush/given-gate
pr: 5002
closed: 2026-10-14
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

**Owed is narrowed.** The row asks for a pair "only `editor_core`'s
union fold can mint". What lands is a pair only made from a verdict
*some boolean* decided. Rust cannot restrict a constructor to one
calling crate, and a kernel-held mint that knows the fold's members
and lineage is the refactor the row's other shape names (the fold's
step behind a kernel door). So any `boolean_op_with` caller can still
re-key or reuse a minted verdict. Through that route the three
measurements above reproduce on this branch (review probes on
`flush/given-gate-review`): a `Continuation` verdict re-keyed onto the
rest touch builds with no row; a verdict reused on faces 0.5 apart
builds; unrelated pairs refuse only at the backstop. That residue is
`a-glue-verdict-is-not-bound-to-the-bodies-it-judged`. What is closed
is a pair stated from nothing: a relation the caller writes down.

**The change.** The given pair is `GluedPair`, the glue's own pair
type, now public with crate-private fields; its one public mint is
`GlueVerdict::given_at`. A `GlueVerdict` is minted only by the kernel,
from the face-pair rows of a boolean's own coincidences
(`GlueVerdict::of_rows`, `crates/topo/src/boolean/mod.rs`, reading
`Coincidence::face_pair`), and handed out on `BooleanBody::verdicts`.
`verbs` carries the list on `VerbRecord::Boolean`. The union fold
(`crates/editor-core/src/eval/wire.rs`, `pair_verdicts` and
`given_pairs`) reads the judgement's verdicts there rather than
re-deriving them from the rows, and re-keys each verdict's A face
through its lineage. `Glue::of` is infallible, because a verdict's
class always glues.

**The pin** is a source census,
`a_given_pair_is_made_from_a_kernel_verdict::the_one_public_mint_of_a_given_pair_is_given_at`:
no public function in `topo`'s source other than `given_at` returns
either type, and neither has a public field or a minting derive. A
public `of_rows`, a public `GluedPair::new`, or a public field each
turn it red (each tried). The same module's
`a_rest_touch_is_the_unions_one_verdict_and_glues_as_given` checks
that a rest touch is the union's one verdict and that, handed back, it
builds the decided body with no row of its own.
