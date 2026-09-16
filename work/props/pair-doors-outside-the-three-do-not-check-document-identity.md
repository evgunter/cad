---
id: pair-doors-outside-the-three-do-not-check-document-identity
kind: unit
title: Beyond product/assemble/placement, (document, evaluation) doors do not check the pairing, and the three that do spell the predicate three ways
status: review
opened: 2026-09-04
refs: [1808]
branch: edit/pair-apply-names
pr: 2723
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

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/edit/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): EDIT is DOCM's successor on the document-model ground (persist, the edit vocabulary, the node and resolver doors). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

(At DOCM's exit sweep, `refs` names the PRs `DOCM-4` stood for: `DOCM-4` = #1808 — the unit rows left the tracker with `work/docm/`; `docs/DOC-LEDGER.md` sweep 14.)

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/pair-apply-names`. EDIT builds the one remaining door on
its ground and re-homes the rest.

1. `resolve::apply_with_names(doc, edit, eval, tol)`
   (`crates/editor-core/src/resolve/mod.rs`) runs `ident::mispaired(doc.id(),
   eval.document)` before any name is checked (`Evaluation.document`
   exists, `eval/mod.rs`) and refuses typed through a new `EditError`
   arm `EvaluationOfAnotherDocument { expected, found }` — the per-door
   vocabulary pattern the row records (`ProductError`, `MateFault`,
   `ChecksError` each carry their own arm over the one predicate).
   `Display` in prose, F6-shaped; the `pncad-py` tag row for the new
   arm (LIB's file, mechanical, said in the PR).
2. The red probe first: `origin/docm/4-review-r2` still exists; fetch
   it and lift R2's probe (two documents of one recipe, ids colliding
   by construction, `apply_with_names` against the twin's evaluation —
   a spurious `NameUnresolvedInEvaluation` or a false admission).
   Adopt it authorship-preserving if it is fit, else write your own,
   pinning BOTH directions (the twin's evaluation refused; the own
   evaluation still admitted).
3. Re-home the row: after this door the remaining entries (`stackup`
   ×3, `drive::certifying`) are all PROPS's (`work.py territory`).
   `git mv` the file to `work/props/` with a `## Re-homed` section
   saying EDIT's door landed and what remains is PROPS's, keeping the
   id; one-file-one-item says the move is how a finding reaches its
   owner (`work/README.md`). Say in the PR that PROPS was not asked.

## Built (2026-09-16)

`resolve::apply_with_names` is the fourth pairing door. It runs
`ident::mispaired(doc.id(), eval.document)` before it reads the edit at
all and refuses `EditError::EvaluationOfAnotherDocument { expected,
found }` — its own arm over the one predicate, the pattern
`ProductError`, `MateFault` and `ChecksError` set. `Display` is F6's
prose with no category prefix (this enum's own rule), and the
`pncad-py` tag, inner-variant and payload rows follow it; the two ids
take no payload attribute, the message states both, which is the
`ProductError` arm's precedent one door over.

The red row is `edit_pair_apply_names::
apply_with_names_refuses_an_evaluation_of_another_document` — review
lane R2's DOCM-4 probe, adopted and widened. Twins of one recipe that
differ in ONE thing the tables can see (a square prism has a fourth
rim edge, a triangular one does not), so BOTH wrong answers are
reachable and both are pinned: the false admission (a name the twin
carries and the edited document does not) and the spurious
`NameUnresolvedInEvaluation` (a name the edited document carries and
the twin does not). The two own-evaluation answers are pinned beside
them as the premises, and a name-free edit refused on the pairing pins
that the check is the door's rather than the name loop's.

The prose that enumerated the doors moved with it: `ASSEMBLY.md`'s A2a
is the one list (it was already a door behind — DOCM-5's `run_checks`
door landed without it), and `ident.rs`'s `Mispaired` and
`Evaluation::document` now point there instead of carrying a second
copy that rots.

Not built: `stackup::sensitivities`, `stackup::stackup` (whose
`pair_record` ties `paired` by node set and content key, both satisfied
by a twin) and `drive::certifying_vector`. They are PROPS's, below.

## Re-homed (2026-09-16)

Moved from `work/edit/` to `work/props/` in the PR that built EDIT's
door. What remains is the three doors above, all of them on PROPS's
paths (`work.py territory`: `crates/editor-core/src/stackup.rs` and
`crates/editor-core/src/drive.rs`). The id, the finding and the
`## Spec` above are unchanged; the directory is the claim
(`work/README.md`). PROPS was not asked — a lane does not need the
owner's permission to put a finding where it belongs.

`status: review` and `pr` are this PR's. Once it merges the row is
PROPS's open work again: the door EDIT owed is built, the three that
remain have no spec yet.
