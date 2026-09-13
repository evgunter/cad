---
id: fillet-escalation-site-has-no-producer
kind: unit
title: The six profile fillet recourse sentences are dead: EscalationSite::Fillet has no producer and PathError::Escalated has no fillet arm
status: closed
closed: 2026-09-13
opened: 2026-09-04
refs: [S11]
branch: blend/12-fillet-recourse-arm
pr: 2508
---

## The class

All six `FILLET_*_RECOURSE` sentences in `crates/profile/src/validate.rs`
(`FILLET_TURN_INBAND_RECOURSE`, `FILLET_NO_CORNER_RECOURSE`,
`FILLET_OFFSET_LEVER_RECOURSE`, `FILLET_ENCLOSING_RECOURSE`,
`FILLET_FIT_RECOURSE`, `FILLET_LEG_EXTENT_RECOURSE`) are rendered by
exactly one Display arm — `ProfileError::Escalated { site:
EscalationSite::Fillet, .. }`, dispatched on the escalation's predicate
name (`crates/profile/src/validate.rs:640`). Nothing in `crates/*/src`
constructs that value:

- every `ProfileError::Escalated` mint in `validate.rs` (`:1287`,
  `:1315`, `:1411`, `:1503`, `:1544`, `:1607`) carries a `Segment`,
  `SegmentPair` or `Loop` site;
- `EscalationSite::Fillet` appears in `src` only at the Display arm;
  the other two mentions are hand-built test values
  (`crates/profile/tests/rejections.rs:539`,
  `crates/profile/tests/fillet_recourse_followability.rs:202`).

The gates themselves fire — the nine `fillet_*` predicate names are
decided in `crates/profile/src/sugar.rs` — but an in-band verdict
leaves as `PathError::Escalated`, whose Display
(`crates/profile/src/path.rs:1505`) has no fillet arm and appends the
shared `COINCIDENCE_RECOURSE`. The tailored sentence reaches nobody.

## What is already known, and where

This is not new to PR 1753, which narrates it as its headline finding:
`crates/profile/tests/rejections.rs:501` (commit 38cb556f, 2026-09-02)
already says "`EscalationSite::Fillet` has no producer in the kernel
today", and `work/code-quality/S11.md` carries the same neighbourhood
("`ProfileError`'s five fillet variants … constructible only from
`test_support.rs`", later "now fully orphaned"). Neither recorded the
six sentences as a dead-recourse instance, and no item owned it. This
file is that home.

## What is pinned

`crates/profile/tests/fillet_recourse_followability.rs` (PR 1753)
asserts, per sentence, that the public door's refusal carries none of
the six — so those rows go red the day a producer lands — and that the
request each sentence endorses builds anyway. The rows are
characterizations, not proof the sentences are worth keeping.

## The decision owed

One of: give `PathError::Escalated` a fillet arm keyed on the predicate
name (the sentences then reach the caller the gates were written for);
route the `sugar` fillet escalations through `ProfileError` at the
`Fillet` site; or retire the six constants and the arm as machinery
with no producer. A door change either way — input to the FILLET
program's H units.

## Pointer (FILLET sweep, 2026-09-06)

Former `refs` `recourse-sentences-owe-followability-pin` named FILLET items now deleted with that program's directory: `recourse-sentences-owe-followability-pin` — FILLET E2, closed (PR 1753). Recoverable at the sweep SHA in `docs/DOC-LEDGER.md` (sweep 7).

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/blend/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). `crates/profile/src/validate.rs` and `path.rs` are S-BOOL's glob for the PATHS lattice; the fillet door's arms are edited here by announced seam, as FILLET did.

## Landed (BLEND-12, PR 2508)

`PathError::Escalated`'s `Display` asks `validate::fillet_recourse_for`
first — the crate's ONE map from the nine `fillet_*` names to the six
`FILLET_*_RECOURSE` sentences — and renders the site it was resolving
with that sentence and no coincidence tail. `EscalationSite::Fillet` had
no producer in `crates/*/src` (all seven `ProfileError::Escalated` mints
carry `Segment`, `SegmentPair` or `Loop`) and was retired with its arm
and both hand-built test values.

**The reachability map, as measured at the landed head** — six of the
nine take an in-band verdict from a request a caller can author, not the
four the unit first reported:

| driveable in band through the public door | how |
| --- | --- |
| `fillet_enclosing_carrier` | radius within the band of a leg's carrier radius |
| `fillet_offset_line_circle` | offset line tangent to the offset circle |
| `fillet_offset_circles_external` | offset circles externally tangent |
| `fillet_offset_circles_internal` | mixed-winding corner, offsets `R ± r` |
| `fillet_corner_turn` | the margin is LEVERED (`sin φ · arm`), so a short leg at a real angle lands in the band with no length-shaped gate answering first |
| `fillet_offset_lever` | the threshold grows with the corner's squared scale, so a large enough scene meets `\|ρ\|` with both clearances definite |

| not driveable | why, and how strong the claim is |
| --- | --- |
| `fillet_corner_arm` | **shadowed by magnitude**: its margin IS the lever arm, so every way of putting it in the band puts a length-shaped path gate (`path_corner_advance`, `path_corner_reach_arc`, `path_arc_center_radius`) in the band on the same request. A family of witnesses, not a proof. |
| `fillet_leg_fit`, `fillet_leg_reach` | **unreachable at `f64`, and only there**: the exact-order band `(from_bits(1), from_bits(2))` admits no representable `f64`. Scalar-scoped — `tests/interval_lane.rs` already drives `fillet_leg_fit` in band under `--features interval`. |

Two sentences were corrected under the A3-2 rule because they were false
at sites that turned out reachable: `FILLET_TURN_INBAND_RECOURSE` now
names the leg extent as a lever (it asserted the corner was degenerate,
at a corner that is not), and `FILLET_OFFSET_LEVER_RECOURSE` now names
the bound on its own window (its request reverses past a narrow window
on a scene the offset radius dominates).

**Still open, filed:**
`work/blend/the-guided-replay-door-renders-the-shared-recourse-for-a-fillet-gate.md`
(a second mouth of the public door the arm does not cover),
`work/blend/fillet-inband-recourse-drops-the-tolerance-lever.md`,
`work/issues/the-levered-turn-margin-conflates-a-short-arm-with-a-small-turn.md`
(the definite arm of the turn gate refuses a real corner as already
tangent — the root of the sentence repair above),
`work/issues/dead-work-citations-from-shipped-code-and-docs.md`.

## Closed (2026-09-13, PR 2508)

The nine `fillet_*` names route through one map in `validate.rs` to the
six sentences, dispatched first in the path door's Display; the
producerless `EscalationSite::Fillet` arm and variant are retired. Six
of the nine gates are driven in band through the public door and pinned
by row; the two exact-order gates are pre-empted at scalar `f64` and
paired to their sentence by an independent doc line. Residues on the
slate from this unit: the guided-replay door rendering the shared
recourse for a fillet gate; the tolerance-lever residue with its premise
corrected; in `work/issues/`, the levered turn margin conflating a short
arm with a small turn, and dead `work/` citations from shipped code.
Unit 15 takes the unknown-name arm and the roster from here.
