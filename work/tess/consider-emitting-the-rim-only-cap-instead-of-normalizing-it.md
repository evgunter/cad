---
id: consider-emitting-the-rim-only-cap-instead-of-normalizing-it
kind: issue
title: TABLED (Ev): consider (E) — teach the lanes an interior pole — instead of (N)'s one seamed form
status: deferred
opened: 2026-09-18
refs: [rim-only-sphere-cap-panics-at-census]
priority: P1
cost: D
---


**Deliberately tabled (Ev, in-chat, 2026-09-18, deciding `[ev]` PR
2850).** The ratification this `deferred` row cites is that ruling: (N)
for now, entertained rather than enshrined — Ev kept it OUT of
DESIGN.md on purpose (same conversation: "too much weight"), so the
rule lives on the rows that carry its work and, once they land, in the
docs of the two doors that enforce it.

## What is tabled

(E): a sphere face with a pole in its interior — one latitude circle,
no meridian, no pole vertex — is a face, and each lane learns it. For
`mesh` that is a synthesized pole row and one interior seam column
whose two sides share vertex ids, reading which pole and that the rim
closes from props' structural predicates; every other consumer of a
sphere face (booleans, fillet, shell, offset, export) owes its own
answer. The two options and their costs are argued in
`rim-only-sphere-cap-panics-at-census` §The question for Ev.

## Where (E)'s existing half lives

(N) retires props' rim-only arm — PR 2741's `rim_interior_side`,
`sphere_rim_only_pole_level`, `require_rim_only_closed`
(`props_rim_only_closed`, `props_rim_only_extent`) in
`crates/geom-brep/src/props/curved.rs`, with its rows in
`crates/topo/tests/props_sphere_cap_door.rs` — correct, exact work
that (E) would want back. **It is whole at `6a1f6d60d`** (main,
2026-09-18). When the retiring commit lands, this line is repointed at
that commit's parent. The mesh-side measurements (the zero-height
polygon, the fixtures, the STEP statements) are on branch
`tess/rim-only-cap-diag` at `83833e586`.

## What would reopen it

A seam-free exporter's caps mattering enough that round-tripping them
with two extra faces is a cost someone names; or a native verb wanting
to MINT the full-wrap face (`merge_coplanar_faces` skips it today as
`PeriodClosure`).
