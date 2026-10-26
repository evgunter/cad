---
id: a-glue-verdict-is-not-bound-to-the-bodies-it-judged
kind: issue
title: A GlueVerdict is not bound to the bodies it judged: a caller can re-key its A face or hand it to another pair
status: open
opened: 2026-10-13
priority: P2
cost: M
design: true
needs_ev: true
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

## Design

Two designers weighed the row (2026-10-26) and agree on the final state,
and both reject its framing: a verdict is not a token to bind to its
bodies. `given` is a decision handed to the kernel from outside, and it
exists because DM4's n-ary union and intersect have no kernel door. The
kernel exposes pair booleans, so `editor-core` rebuilds the operator
(judge every meeting member pair once, fold in the author's order, read
each verdict through face lineage) around them and hands the kernel's
own verdicts back in through a public channel it cannot check. The
lineage is already a function of kernel output alone (`names::Fold`
builds it from `topo::BooleanNaming` through `FaceDescent`).

The answer: `topo` gets an n-ary union and intersect over the members in
the author's order. It judges the pairs, folds, advances its own lineage
and carries each step's records, and returns each judgement's and each
step's body with the lineage `editor-core` names from.
`BooleanInputs::given`, `GlueVerdict::given_at` and `GluedPair` become
crate-private, so no public kernel input is a decision and a glue no
record states cannot be written. The fold's half of
`work/contacthold/a-booleans-records-are-complete-only-relative-to-the-records-passed-in.md`
and `work/contacthold/a-fold-steps-restated-tangency-takes-one-of-several-verdicts-margins.md`
close with it.

Rejected: a verdict carrying an identity of the bodies it judged (a stamp
the kernel carries, which `[coincidence.door]` rules out, and it leaves
the channel public); re-checking the given pair at the step (ruled out by
the one-verdict-per-carrier-pair rule PR 4323 ratified, and it records
nothing); leaving it documented (a sequencing choice, not a design).

Open within the answer: which member is a judgement's operand A. Today
`editor-core` picks the lesser read (PR 5303). The kernel door either
takes a per-member rank, or judges in list order with the margin made
symmetric where the asymmetry starts.
