---
id: gate-on-the-type-in-prose-outside-geom-core
kind: issue
title: three prose sites outside geom-core still put the interval gate on the TYPE
status: open
opened: 2026-09-21
priority: P4
cost: E
refs: [ring-1-interval-type-ungated, H5]
---


## What

RING-1 ungated `geom_core::interval`: the type, its arithmetic and its
`Real`/`Decide`/`Bounds`/`SpanLocate` impls compile in every build, and
the `interval` feature now gates only the kernel's instantiation at the
scalar. Three prose sites outside that unit's fence still say the gate
is on the TYPE, and each is false as written:

- `crates/geom-brep/README.md`, clause **C9** ("the evaluation scalar
  `geom_core::Interval` (behind the `interval` feature, backend the
  in-repo `interval-transcendentals` crate) is a `Real` instantiation
  for replay", `:258`). A ratified design page; `work/scalar/H5.md`
  ruling 1 (iii) already schedules C9's re-wording for the cut that
  drops the feature, so this row is the record that the clause is
  stale from RING-1's merge and not from RING-3's.
- `interval-transcendentals/README.md` ("`geom-core`'s `interval`
  feature depends on this crate", `:11`) — `geom-core` depends on it
  unconditionally now; no feature activates the edge.
- `interval-transcendentals/docs/inventory.md` ("the interval scalar
  … `crates/geom-core/src/interval.rs`, behind the `interval`
  feature", `:4`).

Not hits, checked in the same sweep: `docs/DESIGN.md`'s crate-landscape
row for `interval-transcendentals` (the backend of the interval scalar,
which is what it says) and `docs/CI-MINUTES-2026-08.md` (a dated
measurement record, correct as of its date). Every other
site in the tree that names the gate is about the instantiation — a lane impl, a gated suite, a demo cell, the wheel's
uncertified half — and stays true.

## Why not fixed where it was found

`docs/RING-1-SPEC.md` §5 fences that unit to `geom-core`, `ci.yml`,
`scripts/gates/*`, `docs/DESIGN.md` and `docs/GENERICS-BUILD-COST.md`;
these three sit outside it, and C9 belongs to a ruling that already
named the PR that re-words it.
