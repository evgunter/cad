---
id: pair-doors-outside-the-three-do-not-check-document-identity
kind: issue
title: Beyond product/assemble/placement, (document, evaluation) doors do not check the pairing, and the three that do spell the predicate three ways
status: open
opened: 2026-09-04
refs: [DOCM-4]
---


## What

Found by DOCM-4's dual review (PR 1808), both lanes, with two red
probes on the reviewers' branches (`docm/4-review-r1`,
`docm/4-review-r2`). DI3 puts the pairing check at `product`, `assemble`
and `SolvedPoses::placement`. Every other door that takes a document
plus a value that must be OF that document still answers about a
foreign one when the ids collide, which two documents of one recipe do
by construction:

- `checks::run_checks` (`crates/editor-core/src/checks.rs:544`) — with
  `separation: Advisory::Off`, `connectedness` reads `ev.value(root)`
  first and returns `Ok` (red probe in both lanes); with the default
  config the gather's refusal arrives, but as `ChecksError::Product {
  reason: String }`, untyped. The spec forbade a second check there.
- `resolve::apply_with_names` (`resolve/mod.rs:1132`) — the
  forward-reference carve-out is SATISFIED on a twin, so the name is
  checked against the wrong table: a spurious
  `NameUnresolvedInEvaluation` or a false admission (red probe, R2).
- `stackup::sensitivities`, `stackup::stackup` (`stackup.rs:421`,
  `:1625`) and `pair_record` (`:638`), which ties `paired` by node set
  and content key — both satisfied by a twin; `stackup` already refuses
  a mispaired `analyzed` box (`StackupRefusal::ForeignBox`), so the
  module knows the move for one of its two foreign-input channels.
- `drive::certifying` (`drive.rs:471`) — M10's lane, by announced seam.

And the three doors that do check spell the predicate three times with
three payloads (`PriorIgnored`, `ProductError::EvaluationOfAnotherDocument`,
`MateFault::PosesOfAnotherDocument`; `eval/mod.rs:1763`,
`product.rs:464`, `mate/solve.rs:142`). The fix is one predicate with
one payload struct, each door wrapping it in its own error vocabulary,
applied to every door above — a class, not five patches. The
`run_checks` case wants the registry's subject decision
(`check-registry-gathers-product-twice`) since a resident handed a
subject never reads the evaluation itself.

## Where it stands

DOCM's slate; the `drive.rs` door is edited by announced seam to M10's
successor. Not a unit until `check-registry-gathers-product-twice` is
ruled, since the two share the registry's shape.
