---
id: DECIDE-5
kind: unit
title: the arc's span from the turn the profile decided: the sweep spells 4·atan(σ·b), not 4·atan|b| (Ev's route B, #3186)
status: dispatched
opened: 2026-09-25
priority: P1
cost: D
branch: decide/5-span-from-the-turn
refs: [rule-d-reaches-the-unit-bulge-only, 3186]
---

## What

Ev's Decision 1 on `[ev]` #3186. The sweep's `placed_segment_spec`
spells the arc carrier's span `4·atan(σ·b)` from the turn `σ` the
profile program already decided (`path_arc_bulge`), in place of
`arc_span`'s `4·atan|b|`. Where `σ = sign(b)`, the value channel is the
same bits. The carrier and the pushforward then mint one atom for the
span.

DECIDE-4's local patch measured what it takes: the `0.5` parameter
control's sign-blocked decisions and 22 on R2's link. It also measured
what it costs: four of the link's `carrier_on_surface_2` theorems, which
are re-baselined and said.

Spec: `docs/DECIDE-5-SPEC.md`. Opus implementer. Review tier: single
FULL review (`work/decide/log.md`, 2026-09-25).

## Phase 1 (on `bd2bf0c85` + the lane's spec commit)

**1. Bit-identity: no value moves.** A local probe patch (not
committed) spelled the span `4·atan(σ·b)` in `placed_segment_spec`
(`σ·b` as `0 − b` on a `Negative` turn, route B's spelling) and, beside
it, computed `arc_span`'s `4·atan|b|` and compared the two through
`Debug` with the `Sym` node id stripped: the exact bits of an `f64`, both
endpoints and the decoration of an `Interval`, value and tangent of a
`Dual`, the value channel of a `Sym`. Every mismatch was appended to a
log, and the first comparison of each (scalar, turn) pair was logged as
coverage. Runs:

- `cargo test -p sweep --features interval`: 1602 + 17 + 2 passed, 0
  failed.
- `cargo test -p editor-core --features interval` (the whole suite,
  every golden and value-channel digest in it):
  1891 passed, 6 failed, 94 ignored.
- Coverage: `f64`, `Interval`, `Dual<f64>`, `Dual<Interval>` and
  `Sym<Interval>` each at BOTH turns; `Sym<f64>` at `Positive`.
  **Mismatches: 0.** (`Probe`, the K-telemetry scalar, is behind the
  `probe` feature and did not run.)

The six failures are none of them a value:
- four pin FORMS or tier counts, and are Phase 2's re-baselines:
  `decide_3_no_predicate_loses_a_decision`,
  `m10_9_no_registrant_lies_on_any_measured_document`,
  `the_forms_the_walks_build_are_pinned_per_eps_row` (the plate's walk
  ledger: form counts and digests, every `SymCounts` column unmoved),
  and `sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`;
- two are `msolve8_levered_clash::c4_band_refuses_every_mate_…_{empty,overflow}`,
  which panic on the process-global tolerance commit
  (`AlreadyInitialized`) when run in a process another test already
  committed. That is the harness, not this patch: the rows commit a
  tolerance of their own and need a process to themselves.

**2. `Sign::Zero`: one helper, total, the positive arm.**
`swept::turn_negates` is the one reading of a turn. `turn_axis` and
`centre_on_material_side` read it now, and Phase 2's span will too. Each
of those two used to spell the convention itself. `Zero` stays total
rather than loud for three reasons:
- it is unreachable (the profile mints an arc's turn from a certified
  non-zero `segment_straightness` sign, `SegmentKind::Arc` documents
  never-`Zero`, and `revolve::tube` hand-mints only `Positive` and
  `Negative`);
- the two shipped consumers already chose total;
- with one reading, the axis, the span and the material side cannot
  part on it, which is what a second spelling risked.

A loud arm would be an assertion no reachable input breaks.

**3. `revolve/axis.rs`'s caller keeps `abs`.** The margin
`π − arc_span(b)` (`axis_arc_span`) asks whether a sphere-class arc's
extent passes a half-turn. That is a question about the size of the
span, whatever the traversal direction. No second spelling of that
quantity is met against it: it is a sign decided at the classifier, not
an identity residual. Once the carrier's span moves to the turn,
`arc_span` has that one caller. It stays in `swept.rs`, renamed for
what it then is (Phase 2).
