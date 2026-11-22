---
id: the-seg-differential-agrees-with-the-plane-instance
kind: unit
title: a differential of profile::seg against the plane instance agrees over profile's rejection rows, pair-contact rows and reach fuzz
status: parked
opened: 2026-11-22
priority: P1
cost: H
blocked_on: [two-curves-in-one-plane-meet-by-the-in-plane-arms, a-touchs-crossing-or-side-is-re-decided-from-tangents-at-its-one-point]
---


PLANAR's fifth unit: the safety net the ruling puts before any deletion
in `profile`.

## The ruled text

PR 7290 (Ev, 2026-11-21): *"`profile::seg`'s pair and ray readers are
deleted only after a differential of `seg`, kept as the test oracle,
agrees over profile's rejection rows, pair-contact rows and reach
fuzz."* Each disagreement is fixed in the reading, not in `seg`.

## What it builds

- `profile` gains `geom-brep` as a dependency (the Layering row's
  `profile` row already says it reads through `geom-brep`'s curve-pair
  reading). The lowering from profile's `Segment` to
  `geom_brep::SketchSegment` is one function (`Line { a, b }`,
  `Arc { a, b, arc }`, the stored vertices verbatim).
- A differential that, for every segment pair `seg::pair_contacts`
  reads and every ray `seg::ray_crossings` counts, reads the same pair
  or ray through the instance and compares: no contact against no
  meeting; a contact's point and kind (`Crossing`, `Touch`, `Tangency`,
  `SharedVertex`) against a meeting's point, place and kind; an
  `Escalation` against a refusal; a ray's count or graze.
- Over three corpora: `crates/profile/tests/rejections.rs` (an
  integration suite, through the public door), `seg.rs`'
  `pair_contact_tests` (in-crate), and `seg_reach_fuzz.rs` (in-crate,
  `gated_to!`, on the EFFORT dial); at `f64`, `Interval` and
  `Sym<Interval>` (profile's `sym-revalue-testing` rows).

## Done when

- The differential runs in the default suite (the fuzz at EFFORT 1) and
  reports zero disagreements, or each remaining one is a named row in
  this unit's PR with a ruling of which side is right and a filed item
  where the instance is wrong and the fix is not this unit's.
- The cost is measured and in the PR: the pair pass's time through the
  instance against `seg` on the largest profile in each corpus, at each
  scalar, and the change in `profile`'s CI test time from the new
  dependency's build.
- `seg`'s pair and ray readers are unchanged in production (this unit
  deletes nothing).

## First check, and what to do if it fails

**Three that every designer left unverified:**
1. **The shared vertex.** PR 7264 (`profile/pen-rectangle-over-a-box`,
   not yet merged) gives `seg::pair_contacts` an adjacent pair's shared
   ends (`Option<SharedEnds>`) and a `CKind::SharedVertex` decided by
   construction, because at plain `Interval` a boxed vertex read against
   itself (`|v − v|`) straddles. The instance's spans keep their ends as
   given, but C5 says *"The reader is given no topology: two edges joined
   at a vertex meet there because their ends meet"*. Run 7264's
   `a_boxed_rectangle_validates.rs` square at ±0.01 through the instance
   first: if its shared vertices escalate, **stop and report** (the
   cut's PR carries this as a question for Ev).
2. **`Sym` discharges.** Profile's symbolic tier proves margins zero
   that `Interval` alone straddles (the D-tab's apothem, the shared
   vertex's chord). The instance spells its margins in 3-D (unit
   normalisation, cross products). Run `recorded_directions` and the
   `Sym<Interval>` rows first; where a margin `seg` discharged lands
   numeric through the instance, list it.
3. **Cost.** If the instance's pair pass costs more than the CI budget
   allows on the corpora (`docs/prompts/implementer-discipline.md`: about
   15 minutes per PR), report the measurement before building further.

## Leaves for later

- Moving the pair pass onto the instance and deleting `seg`'s pair
  reader: `profile-validation-reads-its-pairs-through-the-plane-instance`.
- The ray walk: `both-planar-ray-walks-share-one-driver-beside-the-reader`.
- Ellipse pairs: `seg` has none, so no differential covers them; the
  instance's walk oracle does (unit 2).

## Rows

Waits on ONECOPY's `a-touchs-crossing-or-side-is-re-decided-from-tangents-at-its-one-point`,
which puts the kind on `curve_pair::Meeting` that the comparison reads.

## Territory

`crates/profile/*` and `Cargo.toml`: PROFILE (announce on its log: its
CI time moves with the dependency). `crates/geom-brep/*` only where a
disagreement is fixed in the reader: CPAIR and ONECOPY.

## Review

**Single FULL**, difficulty **H**: the harness is plain, but each
disagreement it finds is a geometry judgement, and the cost and `Sym`
measurements decide whether unit 6 can land at all.
