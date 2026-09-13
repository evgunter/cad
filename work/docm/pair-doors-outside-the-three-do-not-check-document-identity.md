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

- ~~`checks::run_checks`~~ — **SETTLED at DOCM-5** (PR 1871). The
  registry's doors now check the pairing themselves:
  `run_checks_on` runs `ident::mispaired` over `(doc, ev)` AND over
  the `DocumentId` a `Subject::Product` carries, before any resident
  runs, and refuses
  `ChecksError::EvaluationOfAnotherDocument { expected, found }` —
  typed, mirroring `ProductError`'s arm rather than a `String`.
  `run_checks` inherits it, being the wrapper. Both directions of the
  original probe are pinned in
  `docm5_subject::the_subject_door_refuses_an_evaluation_or_a_subject_of_another_document`.

  **The premise this file gave for deferring it was wrong**, and the
  correction is the reason the fix had to be a door check rather than
  a subject decision: *"a resident handed a subject never reads the
  evaluation itself"* is false for `connectedness`, which reads
  `doc.roots()` against `ev.value(root)` and needs no product at all.
  So handing residents a subject moved the gather (and its DI3 door)
  off the path entirely for a config with `separation: Advisory::Off`
  — the `Off` probe both DOCM-4 lanes wrote, still green after the
  subject door, and a NEW hole with the default config, since
  `run_checks_on` is public and takes the pair. Measured by DOCM-5's
  R2 lane: one separation finding for a document whose solids are
  metres apart, computed from a twin's product.
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

And the doors that DO check spell the predicate with four payloads now
(`PriorIgnored`, `ProductError::EvaluationOfAnotherDocument`,
`MateFault::PosesOfAnotherDocument`,
`ChecksError::EvaluationOfAnotherDocument`; `eval/mod.rs:1763`,
`product.rs:464`, `mate/solve.rs:142`, `checks.rs`'s new arm). All four
go through the ONE predicate `ident::mispaired`, which is the half of
the fix that held: what differs is the error vocabulary each door
wraps it in, and that is each door's own — a typed arm a caller can
match beats a shared type a caller must import. DOCM-5 adding a fourth
arm rather than a fourth predicate is the pattern for the rest.

## What remains

`resolve::apply_with_names`, the three `stackup` doors, and
`drive::certifying` (M10's, by announced seam). Three of the five
original entries; the `run_checks` entry is closed above, and the
"spell it three ways" half is answered — one predicate, per-door
vocabularies, by design.

## Where it stands

DOCM's slate; the `drive.rs` door is edited by announced seam to M10's
successor. No longer blocked on `check-registry-gathers-product-twice`,
which DOCM-5 closed.
