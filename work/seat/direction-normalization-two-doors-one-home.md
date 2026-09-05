---
id: direction-normalization-two-doors-one-home
kind: issue
title: Direction normalization has two doors (eval_direction_norm / datum_unit_norm) and three direction spellings — decide the family's one home
status: open
opened: 2026-09-02
github: 1570
refs: [1564, 1372, 1527]
needs_ev: true
---

## From GitHub issue 1570

Opened 2026-09-02; 0 comments.

(SEAT orchestrator) Class finding from SEAT-DV's dual review (PR #1564), filed per the durable-home rule; both reviewers converged on the shape.

SEAT-DV moved the datum length decision into `topo::query::UnitVec3::new` under the new funnel site `datum_unit_norm`, while `editor-core`'s `unit()` (`eval/wire.rs`) keeps `eval_direction_norm` for transform/pattern/mate directions. Consequences the reviews measured:

- `unit()` and `UnitVec3::new` are the same six-line decide/normalize/refuse body in two crates under two names — and the prose that recorded MATE-1's deliberate collapse of `mate_pattern_direction_norm` ONTO `eval_direction_norm` ("one door… wherever it is read") was the only record of that ratified decision; SEAT-DV's re-split is defensible (it crossed a crate boundary) but the family now has no single home.
- One datum direction is decided under TWO names depending on the road: evaluation decides it under `datum_unit_norm`; the mate solve's circular-pattern re-read of the same `Node::Datum` decides the same triple under `eval_direction_norm` (`mate/solve.rs:260-266`). Values bit-identical; the funnel telemetry splits.
- A third spelling exists at `profile::path::Dir<T>` (2-D, and its `from_unit` does not validate).
- The PR body itself asks "why doesn't the revolve axis / face normal / transform axis use `UnitVec3`" and declines the migration — correctly, for that unit — with no home for the question.

The unit this issue wants: migrate the direction-taking sites onto the one validated type (or record per-site why not), collapse the two funnel doors back to one name or ratify the two-name split with the mate-path telemetry consequence stated, and give `Dir<T>`'s unvalidated `from_unit` the same treatment issue 1527 gave `DatumValue`.

## Home

`work/seat/` — the new door lives in `crates/topo/src/query.rs`, a SEAT territory glob, and the direction/datum family is SEAT-DV's own residue in the verb-seat program.

## Decision for Ev (2026-09-05; the `[ev]` PR carries it)

**State today** (re-measured at main `097a8ea5`): the same
decide/normalize/refuse body lives twice — `editor-core::eval::wire::unit`
under `eval_direction_norm` (transform axes, linear-pattern directions,
and the mate solve's re-read of BOTH rule kinds, `mate/solve.rs:343,352`)
and `topo::query::UnitVec3::new` under `datum_unit_norm` (every datum
normal/axis). The FIX program's PR 1738 put the finiteness gate in front
of both through one kernel predicate (`is_finite_length`), so the two
bodies now agree by construction on the gate and by copy on the rest.
`unit()`'s doc and `mate/solve.rs` both state the two-name split as
deliberate ("split by which layer owns the value"); K-REPORT records
that ONE datum triple is emitted under both names depending on the
road. `profile::path::Dir::from_unit` (2-D) stores its ray unvalidated.

**The question: one door, and under how many names?**

- **(A) One door, one name.** `topo::query` owns the family:
  `unit_direction(v, band)` deciding under ONE predicate for every
  direction length in the workspace; `unit()` becomes a call; the
  mate-path telemetry split disappears. Cost: `eval_direction_norm`
  retires — K-REPORT rows move, the census counts pinned in
  `m4_pr2_wire.rs` / `m10_3_driver_interval.rs` re-baseline, and
  MATE-1's collapse record is re-stated under the new name. Cleanest
  definition ("a direction length is one decision"); the largest
  telemetry edit.
- **(B) Ratify the two-name split, one body.** The kernel door takes
  the site as a parameter (`unit_direction(v, site, band)`), both
  names stay, the duplicated body in `editor-core` is deleted. No
  K-REPORT row moves; the mate re-read keeps deciding one triple under
  two names, now stated as ratified rather than tolerated. Smallest
  observable change; keeps the "which layer owns the value" reading
  the code already asserts.
- **(C) Ratify as is** — two bodies, two names, prose only. Rejected
  by the orchestrator: the twin is the finding.

Either way: `Dir::from_unit` gets issue-1527's treatment (validate or
make the unvalidated spelling unrepresentable — it is `fn` private, so
the cheaper answer is a debug-free derivation from the angle door).

**Recommendation: (B).** The split IS a crate boundary and the telemetry
names are a commitment the K census pins; (A) buys one name at the
price of a census re-baseline nobody has asked for. If you would rather
have one name, say so and the unit takes (A) — the difference is one
parameter and the K rows.

Ev's ruling lands here in place; the unit (SEAT-DN, block SEAT-B3)
dispatches on it.
