---
id: direction-normalization-two-doors-one-home
kind: issue
title: Direction normalization has two doors (eval_direction_norm / datum_unit_norm) and three direction spellings — decide the family's one home
status: closed
opened: 2026-09-02
github: 1570
refs: [1564, 1372, 1527]
closed: 2026-09-05
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

## Ruled (Ev, PR 1902, 2026-09-05): (B) — one door, two names ratified

The decide/normalize/refuse body lives ONCE, in the kernel seat
(`topo::query`), and takes the funnel site as a parameter; both names
stay — `eval_direction_norm` for the directions `editor-core` owns
(transform axes, linear-pattern directions, the mate solve's re-read of
both rule kinds) and `datum_unit_norm` for every datum normal/axis —
so no K-REPORT row moves and no pinned census count re-baselines. The
mate re-read's two-name split becomes ratified rather than tolerated,
stated at `mate/solve.rs` and in `unit()`'s successor. `Dir::from_unit`
gets issue-1527's treatment. The options and the state measured at
main `097a8ea5` that this ruling answered: see this file's history at
the `[ev]` PR. Unit: SEAT-DN (`docs/SEAT-DN-SPEC.md`), block SEAT-B3.

## Closed by SEAT-DN (2026-09-05)

The ruling above is executed in `work/seat/SEAT-DN.md`'s PR.

- **One body.** `topo::query::unit_direction` is the workspace's only
  decide/normalize/refuse for a 3-D direction length: finiteness
  through `is_finite_length`, then `Margin::norm3(v)` against the band
  at a funnel site the CALLER passes as a `&'static str`, then the
  normalized ray or a typed `UnitVec3Error`. `UnitVec3::new` is that
  call under `DATUM_UNIT_NORM`; `editor-core::eval::wire::unit` is
  that call under `EVAL_DIRECTION_NORM`, mapping the kernel refusal
  onto its three `NodeErrorKind` arms with the role word preserved.
- **Two names, ratified where they are read.** No K-REPORT row moved
  and no census count re-baselined; the funnel-site census rows
  (`m4_pr2_wire`, `m10_3_driver_interval`) are green untouched. The
  prose that read "two doors, not one" / "tolerated" / "issue 1570 is
  where this is homed" now states the ratified reason — the layer that
  owns a value is the layer whose telemetry names its length decision
  — at `unit()`'s successor, at `mate/solve.rs`'s two-road paragraph,
  at `DATUM_AXIS_ROLE`, and in `docs/K-REPORT.md`. MATE-1's collapse
  of `mate_pattern_direction_norm` onto `eval_direction_norm` is
  restated as holding.
- **`Dir::from_unit`** (DN-3): callers measured; every ray reaching it
  was already decided at the door that built it, or is an exact
  negation, and none holds an angle — so no re-spelling through the
  angle door and no new K carrier row. Stated at the constructor and
  pinned. The residue that measurement DID find is a CLASS, filed with
  its five members and three executed reproductions as
  `work/seat/two-d-director-doors-skip-the-finiteness-question`:
  direction doors that classify a length's sign without first asking
  whether it is finite (`geom-core`'s `definitely_positive`, `sweep`'s
  revolve axis, `topo`'s `sector_shape`, `profile`'s two director
  doors) — each in another program's territory, and four of them
  unlocked at once by FIX's
  `is-finite-length-homed-in-the-query-seat`.
