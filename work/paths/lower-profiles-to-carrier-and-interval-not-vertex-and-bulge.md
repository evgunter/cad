---
id: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
kind: issue
title: A profile lowers to vertex + bulge, which cannot express a full turn, so every circle is split in two and the lowered pieces diverge from the authored path; lower to one canonical carrier + interval form instead
status: open
opened: 2026-09-25
priority: P1
cost: H
---


## Where this came from

EMIT filed this row from Ev's thread on #3202 (2026-09-25). Ev asked:
"we already have lots of ways of expressing arcs in the path algebra; is
this another case of divergence from the lower-level description? …
maybe fixable … by switching to some other single canonical
representation that can express this case". Ev also asked for it to go
to an orchestrator who can give it full attention. No PATHS
orchestrator was active on 2026-09-25.

## The finding

A profile lowers every segment to vertex + bulge. Bulge is tan(θ/4),
which diverges at θ = 2π (`crates/profile/src/path.rs`,
`CircleSplitCount`), so no segment can be a full turn:
- `circle_kernel` splits a circle into two semicircles at ±x;
- `validate` refuses loops with fewer than two vertices
  (`build_loop_segs`).

This is ratified as the "M2 closed-carrier precedent" in
`docs/PATHS-DESIGN.md` §5.1.

The B-rep has no such limit. A full revolve's rims are full-period
self-loop edges (`sweep/src/revolve/mod.rs`), and STEP import builds
them too. Certification, mesh and naming all handle them. Nothing in D1
or topo requires two edges.

What the split forces:
- **Artefacts:**
  - a seam vertex pair and a strut on every cylinder at ±x;
  - the naming anchor's n = 2 orientation case (`eval/anchor.rs`);
  - demos using `circle_split(3)` to dodge semicircle trouble in
    booleans (`demos/tour/src/bossplate.rs`, `twopeg.rs`, `lily.rs`).
- **A divergence:** the lowered pieces no longer line up one-to-one with
  what the author wrote, beyond the merges the geometry itself makes.

## Direction

Lower to one canonical segment form that can express every authored
arc: a carrier (line, or circle with centre and radius) plus a
parameter interval, where a sweep of 2π is legal. Bulge becomes a
derived view rather than the storage.

It needs a survey first. Bulge is read deeply: area sums, joins,
merge_faces, validation and canonicalization. A periodic wall still
needs one seam edge to cut its parameterization (`gate_maximal_faces`,
`boolean/reduce.rs`). Extrude, loft and sweep would build that wall.
The size is roughly 15–25 files across `profile`, `sweep` and
`editor-core`, plus persistence and Python.

This is a design change to ratified PATHS-DESIGN §5.1, so it goes to Ev
as an `[ev]` PR once surveyed.

## Coupling

EMIT's step-id names (`work/emit/profile-pieces-are-named-by-minted-step-ids.md`)
spell a circle as `Piece(0)`/`Piece(1)`. When this lands, a circle is
one `Carrier`, `Piece(1)` vanishes, and the saved format breaks a
second time.
