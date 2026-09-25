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

## Phase 2

**The change.** `swept::turned_span(turn, bulge)` returns
`4·atan(σ·b)`, where `σ·b` is `0 − b` on a turn `turn_negates` reads as
clockwise and `b` otherwise. `placed_segment_spec` spells the carrier's
`param_end` through it. `arc_span` is renamed `span_magnitude`, keeping
its one caller, revolve's `axis_arc_span` margin. `register_span_identity`'s
proof reads the span as `4·atan(σ·b)`, with `σ` the decided sign of that
`b`; it named `|b|` before. The crate docs say the same.

**The row.** `swept::tests::the_carriers_span_meets_the_pushforwards_at_a_parameter_bulge_of_either_sign`
lowers an arc through `placed_segment_spec` at `Sym<f64>` with the bulge a
parameter at `+0.7`, at `−0.7`, and as the reversal `0 − b` of `+0.7`.
Nine samples each, and at every one the carrier's cosine and turn-signed
sine against the pushforward's `4·atan b` are THEOREMS. With the call
spelled `span_magnitude` again, it reds on its first arc: every sine
but `s = 0` is numeric (run locally, reverted).

**Re-baselined, before → after** (dev, all three ε rows, identical at each):
- `decide_3_no_predicate_loses_a_decision`, R2's link, G and the read
  off → shipped:
  - `carrier_on_surface_2` `([98, 0, 0, 10], [82, 0, 6, 20])` →
    `([88, 0, 0, 20], [84, 0, 8, 16])`;
  - `carrier_matches_mapped_source` was not pinned (both sides
    `[108, 0, 40, 32]`) and is now pinned at
    `([108, 0, 60, 12], [108, 0, 50, 22])`.
- `m10_9_no_registrant_lies_on_any_measured_document`, the link:
  `registered` 96 → 108, `symbolic_zero` 541 → 545. The other four
  documents are unmoved.
- `sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`, the link:
  - without the ladder `[541, 0, 96, 465]` → `[545, 0, 108, 449]`;
  - with it `[553, 0, 96, 453]` → `[549, 0, 120, 433]`;
  - retried 12 → 16;
  - `carrier_on_surface_2` `[92, 0, 6, 10]` → `[88, 0, 8, 12]`.
- `the_forms_the_walks_build_are_pinned_per_eps_row`, the plate's
  ledger: five form counts and five digests. Every call and frozen count
  is identical, and so is the receipt.
- `m10_bulge_renders.txt`: the parameter control's section.

Route B's numbers are reproduced exactly, the control's ceiling
included. The item's DECIDE-5 section has the table and the costs, and
`rule-g-trades-sixteen-of-the-links-carrier-on-surface-2` renders the
link's four.

**Gates widened.** `sym_9_retry_interval` and `m10_sym_profile_interval`
were `gated_to!` paths that did not include `crates/sweep/src/swept.rs`,
and this change to it moves both. The path is added to both.
