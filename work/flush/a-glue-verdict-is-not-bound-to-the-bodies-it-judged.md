---
id: a-glue-verdict-is-not-bound-to-the-bodies-it-judged
kind: issue
title: A GlueVerdict is not bound to the bodies it judged: a caller can re-key its A face or hand it to another pair
status: open
opened: 2026-10-13
priority: P2
cost: M
design: true
---

The residue of `boolean-given-pairs-glue-unmeasured-for-any-rust-caller`
(`flush/given-gate`, PR 5002), banded at its parent's P2 because the
harm it leaves is the parent's own: a glue no record states.

## What

A given pair (`GluedPair`) is made only by `GlueVerdict::given_at`,
and a `GlueVerdict` only by the kernel from its own face-pair rows
(`BooleanBody::verdicts`), so a given pair's class is always one some
boolean decided. What the verdict does not carry is which bodies it
judged:

- `given_at(held, a)` takes the A face from the caller, and since
  PR 5303 also `held`, which side of the verdict that face stands for
  (a union's judged pair may have the joining member as operand A).
  The union fold
  passes the accumulation faces whose lineage holds the judged face
  (`given_pairs` in `crates/editor-core/src/eval/wire.rs`), and the
  kernel cannot check that claim: the lineage lives in
  `editor_core::names` (`Fold::member_faces`).
- A verdict's B face is a bare arena key, so a Rust caller can take a
  verdict from one boolean and hand it to a step over two other
  bodies, where the key names whatever face holds it there. The door
  checks only that the keys are live and planar.

## Measured

PR 5002's review reproduced the parent's three measurements on the
gated branch through reuse (probes in
`crates/topo/tests/given_gate_review_probes.rs` on
`flush/given-gate-review` @ `e1abf0a4dd`):

- a `Continuation` verdict re-keyed onto an opposed rest touch builds,
  with 0 coincidence rows;
- a verdict reused on faces 0.5 apart builds;
- unrelated pairs refuse only at the door's unglued-coincidence
  backstop.

Over 648 runs (6 verdicts × 6 A faces × 3 ops × 6 B placements) and a
small-gap sweep, no forged step built a body that differs from the
decided run. What is lost is the record: a forged step glues with no
row, so D10's one door where a value-decided coincidence is recorded,
and the `unproven-coincidence` lint that reads it, never see the glue.

## Owed

Bind a verdict to what it judged without measuring again: either the
kernel carries the fold's lineage itself (the fold's step moved behind
a kernel door that holds the members and advances its own lineage from
each step's naming), or a verdict carries an identity of the judged
bodies that the door can compare. The façade never sets `given`, and
`pncad-py` does not expose it, so the route is a Rust caller of
`boolean_op_with`.
