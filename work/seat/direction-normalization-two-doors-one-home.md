---
id: direction-normalization-two-doors-one-home
kind: issue
title: Direction normalization has two doors (eval_direction_norm / datum_unit_norm) and three direction spellings — decide the family's one home
status: open
opened: 2026-09-02
github: 1570
refs: [1564, 1372, 1527]
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
