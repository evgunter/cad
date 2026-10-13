---
id: a-glue-verdict-is-not-bound-to-the-bodies-it-judged
kind: issue
title: A GlueVerdict is not bound to the bodies it judged: a caller can re-key its A face or hand it to another pair
status: open
opened: 2026-10-13
priority: P3
cost: M
design: true
---


The residue of `boolean-given-pairs-glue-unmeasured-for-any-rust-caller`
(`flush/given-gate`).

## What

A `GivenPair` is now minted only from a `GlueVerdict`
(`crates/topo/src/boolean/mod.rs`, `GlueVerdict::given_at`), and a
`GlueVerdict` only by the kernel from its own face-pair rows
(`BooleanBody::verdicts`). So a pair's relation is always a decision
some boolean made. What the verdict does not carry is which bodies it
judged:

- `given_at(a)` takes the A face from the caller. The union fold
  passes the accumulation faces whose lineage holds the judged face
  (`given_pairs` in `crates/editor-core/src/eval/wire.rs`), and the
  kernel cannot check that claim: the lineage lives in
  `editor_core::names` (`Fold::member_faces`).
- A verdict's keys are arena keys, so a Rust caller can take one from
  one boolean and hand it to a step over two other bodies, where its B
  key names whatever face holds that key there. The door checks only
  that the keys are live and planar.

## Owed

Bind a verdict to what it judged without measuring again: either the
kernel carries the fold's lineage itself (the fold's step moved behind
a kernel door that holds the members and advances its own lineage from
each step's naming), or a verdict carries an identity of the judged
bodies the door can compare. Weigh whether a determined forger is
worth either: the façade never sets `given`, and `pncad-py` does not
expose it.
