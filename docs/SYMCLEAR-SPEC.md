# SYMCLEAR — the symbolic tier runs the clearance engine

Unit: `work/clear/symbolic-tier-and-clearance-engine.md`. Read its
`## Decided` and `## Scope added` sections; they are this spec.
Riders: `clearance-replays-a-leaf-off-the-verdicts-lane.md` and
`needs-enclosure-names-its-door-by-a-moved-path.md`. Program: CLEAR.
Branch: `clear/symclear`. Review: single FULL review.

## What is true when this lands

1. **One door value.**
   - `AtRestPolicy::clearance_lane() -> Option<ClearanceLane<Self>>`
     answers `Some` at `Interval` and at `Sym<T: CertifiedBounds>`, and
     `None` at `f64`, `Probe` and `Dual`.
   - It reaches `topo::clearance`'s generic door, which SHELL-3 built.
   - `editor_core::measure::MinClearanceLane` is deleted, together with
     every bound naming it (`EvalScalar`, `eval/wire.rs`), the pncad
     lane-seam list and the "trio" docs.
2. **The bracket re-enters `T`** as the hull of `T::from_computed(lo)`
   and `T::from_computed(hi)`.
   - At `Interval` that gives the same bits as today.
   - At `Sym` it gives a fresh unknown: two clearance measures with
     equal brackets are never proved equal.
3. **The up-front refusal is deleted.** `DriveRefusal::SymbolicClearanceUnsupported`
   and the drive's check both go. The M10-6 suites stop naming
   `SymbolicDials::off()`, and so does `m10_6_ci_rows_interval`'s
   refusal fallback.
   `the_symbolic_tier_refuses_a_clearance_measure_by_its_spoken_node`
   becomes a positive pin: the dumbbell certifies with the tier on, and
   its bracket is bit-identical to the numeric run.
4. **`clearance_with` replays each leaf on the verdict's lane**
   (`ParamBoxVerdict::lane()`), as `report.rs`'s `replay_leaf` does, and
   hands the generic door that `Body<T>`.
   - A row that drives a box with the tier on, wide enough for the lane
     to matter, and asks the clearance query must go red on the old
     replay.
   - If no box makes the two replays differ, say so with the widths you
     tried.
5. **`NodeErrorKind::MeasureClearanceRefused` carries only what the
   measure door can raise.** Narrow it.
6. **The `NeedsEnclosure` door string names the door the `f64` lane
   actually lacks**, and its five pins move together.
7. **A row runs the generic door at a `T` other than `Interval`.** This
   is `Sym<Interval>`, and the row checks that a computed station
   enters as `from_computed`.
8. **Measure the per-leaf cost of running at `Sym`** on the M10-6
   fixtures with default dials, and report it.

## Ground

- `measure.rs`, `drive.rs`, `eval/{mod,wire,measure}.rs`, `clearance.rs`
  (the editor-core half), `topo::props`'s `AtRestPolicy`, pncad and
  pncad-py.
- Announce any seam with `python3 scripts/work.py territory --base origin/main`.
- A public signature changes, so run `demos/tour` and `demos/wild`
  clippy.
- Read `docs/prompts/implementer-discipline.md` in full first.
- Never arm a waiter or a background chain. Anything that outlives
  600 s runs `setsid`-detached and is polled in the foreground.
- CI is the verification of record. This diff seeds `topo`, so allow
  25 min.

## Report

≤150 lines: what was deleted, the replay row and the widths it needed,
the measured cost, the goldens moved, and the PR and its CI.
