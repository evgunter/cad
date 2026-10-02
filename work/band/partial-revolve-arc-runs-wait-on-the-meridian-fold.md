---
id: partial-revolve-arc-runs-wait-on-the-meridian-fold
kind: issue
title: revolve: a partial revolve still sweeps one wall per arc of a cocircular run, because one wall would carry a meridian in pieces mass properties do not fold
status: open
opened: 2026-10-02
priority: P1
cost: M
blocked_on: [sphere-wedge-arm-does-not-fold-split-meridians-by-lineage]
refs: [swept-cocircular-arc-runs-build-one-wall]
---


Split out of `swept-cocircular-arc-runs-build-one-wall`. Extrude and the
full revolve build one wall per run of cocircular arcs; the partial
revolve still builds one wall per arc, on the run's one surface key
(`crates/sweep/src/swept.rs::CurvedRuns::Split`, passed by
`revolve/partial.rs::sweep_loop`). What `crates/sweep/README.md`
("Walls: one per run") says is not yet true of it.

## Why it waits

Built whole, a partial revolve's arc-run wall keeps the station on both
wedge caps, so its sphere or torus face carries each cap's meridian as
two or more edges. `validate_geometric` then refuses every such body at
check 7 (`VolumeUncomputable`), measured over a D of k = 2..4 cocircular
arcs, both revolve directions, three loop starts:

- sphere wall (the D revolved about its diameter): `NotIsoRectangle {
  what: "the wedge arm reads a two-edge boundary; a meridian in pieces
  is not folded on the sphere" }`
  (`crates/geom-brep/src/props/curved.rs::sphere_wedge_azimuth`);
- torus wall (the D beside the axis): `NotIsoRectangle { what:
  "props_rim_level" }`.

The k = 1 bodies, and the same bodies split per arc, validate. Neither
fold can take these pieces today even with the sphere arm's lineage fold
(`work/props/sphere-wedge-arm-does-not-fold-split-meridians-by-lineage.md`):
the run's cap meridians are minted one edge per segment, so they carry
no split lineage (`fold_torus_meridians` folds by `CarrierId` identity,
never by stored geometry).

## The row

- PROPS folds a sphere meridian in lineage pieces (the blocker).
- The partial revolve mints a curved run's wedge-cap meridians so the
  pieces share lineage — one edge per run, split at its stations — or
  props gains a structural fold that reads a station chain some other
  way; then it passes `CurvedRuns::Whole`, and `CurvedRuns` goes.
- Re-check the torus arm's `props_rim_level` refusal on the same body
  once the meridians fold; it may be a second blocker.
- The fixtures that rely on the split then move to hand-cut bodies, as
  `common::latitude_seam::two_arc_sphere` did for the full revolve:
  `torax_axial::torax_a_two_arc_lune_re_authors_its_equator_seam_and_reaches_the_props_door`
  (the seam a station would strike is cut as
  `common::latitude_seam::latitude_arc` cuts the full revolve's), and
  the partial rows of `run_walls_built::arc_runs_build_one_wall_each`
  flip to one wall.
