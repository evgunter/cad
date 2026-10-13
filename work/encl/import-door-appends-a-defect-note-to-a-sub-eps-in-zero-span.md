---
id: import-door-appends-a-defect-note-to-a-sub-eps-in-zero-span
kind: issue
title: geom-brep: at the import door, a ParamSpan Zero arm whose enclosure straddles zero appends the at-rest 'kernel defect or a damaged file' note right after 'the file does not state it'
status: open
opened: 2026-10-08
priority: P3
cost: M
design: true
---


(Filed by the ENCL fix-pass implementer on PR 4331, `encl/adoption-at-rest-eps-in`, on the orchestrator's ruling. That PR leaves this case unchanged.)

## What

At the STEP import door, `SizedDecision::recourse_in_file` (`crates/geom-brep/src/recourse.rs`:326–330) ends a sized decision's Zero arm through `MarginDiag::sized_recourse_in_file`, with `SizedWords::otherwise` set to the decision's `at_zero.stored` note. That note is appended where no smaller tolerance decides the margin.

`ParamSpan`'s note (`crates/geom-brep/src/certify.rs`:752–757) is "an edge of no length, or a reversed one, is one no kernel construction mints, so this is a kernel defect or a damaged file, worth reporting".

At the `Interval` lane, a span's Zero verdict can carry an enclosure that straddles zero, for example `[-2e-12, 6e-12]`:
- `tightens_below` gives no value for it, because both ends must lie on the passing side;
- so the door reads: "This length is below the file's declared coincidence distance ε_in = …, so the file does not state it. Recourse: move the geometry so this edge is not vanishingly short; an edge of no length … is a kernel defect or a damaged file".

That attaches a defect claim to what may be sound geometry below ε_in. poleband_eps12's span is such geometry: a sphere band truncated 9e-11 rad below the pole, and at f64 a positive point margin, so the door offers the value. The note's premise, "an edge of no length", is not what a straddling enclosure establishes.

## Repair shape

Which words the door owes a band-decided arm that has no value and an `at_zero` note is a design question:
- the lever alone;
- a door-specific note saying the file does not resolve the span's sign;
- or `at_zero` notes split by what they claim.

Decide it with D4 ¶1 (i)'s one-recourse-per-decision rule in view. Pin the answer with an Interval-lane row in `certify.rs`'s `the_import_door_reads_at_rest_with_its_eps_in_words`, which today asserts that wording (its `reaches_zero` arm).

## Also on the dihedral arm (review of PR 4366)

- **An exact zero contradicts its own note.** For an exact-zero arm
  the import door (`MarginDiag::sized_recourse_in_file`) prefixes
  "below ε_in … the file does not state it" even where the zero is
  sound geometry the file states exactly: a cone apex, which
  `geom_brep::DIHEDRAL_ARM`'s zero note then names ("a face curving to
  a point as a cone does, leaves no angle to measure"). The two
  sentences disagree about whether the file stated the size.
- **No budget applies at this door.** The import door's renderings of
  the arm (`CertCheck::TransversalityArm`, `CertifyError::ArmCollapsed`)
  run 80–101 words, as the review measured them, and no row holds the
  door's text to the 75-word refusal budget
  (`test_utils::refusal::BUDGET`) the way editor-core's
  `refusal_concision_at_rest` holds the at-rest badges.

- 2026-10-12 (ENCL orchestrator): PR 4855 (merge `5903a8c5a2`) widened this row's reach. `SizedDecision::recourse`'s Undecided arm now takes `at_zero` for a gate's rejected Zero, so on the Interval lane a rejected-Zero enclosure that straddles zero inside the zero band takes the note on every Undecided-routed reader of `DIHEDRAL_ARM` and offset_meters, not only on the Zero arm. The note's premise ('an edge of no length…') is no more established by a straddling enclosure there than here. Also, `passes.passes_zero()`'s early return guards only the Zero arm (recourse.rs about L437). A decision that passes zero and has `at_zero` would end differently by route. None exists today: every `at_zero: Some` decision is Positive.

## 2026-10-13: the ε floor widens what the notes speak for (ENCL, PR 4925 review)

PR 4925 gives ε a floor, `geom_core::tolerance::EPS_MIN` ≈ 2.2e-13 m (D4 ¶4), and `MarginDiag::tightens_below` withholds any offer at or below it. A decision's `at_zero` note was written for a zero verdict with no size to tighten below. It now also ends every nonzero zero-band margin whose |m|/K ≤ EPS_MIN.

At a run with ε < K·EPS_MIN (2.2e-12 at K = 10, so CI's 1e-12 row), every zero-band margin satisfies |m| ≤ ε < K·EPS_MIN. Every zero-band offer therefore vanishes, and each note speaks for any sub-floor size, not only an exact or proved zero.

Notes whose wording claims a defect or a non-mintable shape, and so want this audit:
- `CertCheck::ParamSpan`, the "kernel defect or a damaged file" note this row is about (`crates/geom-brep/src/certify.rs`).
- `CertCheck::ParamSpanMeter` (`certify.rs`, `SpanMeterCollapsed`).
- `DIHEDRAL_ARM` (`crates/geom-brep/src/dihedral.rs`). Its `a_gates_rejected_zero_ends_in_the_arms_zero_note` row now ends a rounded 1.39e-17 m zero in the note.
- `offset_meters`' `Meter::NormalFloor` (`crates/geom-brep/src/offset_meters.rs`, read in `topo/src/validate.rs`).

PR 4925 changes no note's wording. The audit is this row's.
