---
id: the-patch-bound-refuses-an-integral-chart-with-a-crease-anywhere
kind: issue
title: offset_meters::chart_arm refuses an integral chart with a C0 crease anywhere, so no arm is read at a point however far the crease lies
status: open
opened: 2026-10-31
priority: P3
cost: M
---


Filed by `splice/a-vertex-piercing-a-spline-face-reads-no-outward-normal`'s
lane (PR 6152, review fix pass). Owned here because
`crates/geom-brep/src/offset_meters.rs` is on ENCL's paths.

## What

`geom_brep::offset_meters::chart_arm` is the certified lower bound on a
spline chart's length scale at a point, over a reach. On an integral
chart (`constant_weight().is_some()`) it reads its cells through
`patch_bound::patch_cells_refined`, which refuses a chart with a C⁰
crease (a knot run of multiplicity at least the degree) anywhere in
it. `chart_arm` then answers `ChartArmRefusal::PatchBound`, and no arm
is read at any point of the chart, however far the crease lies. A
rational chart takes `patch_cells_across_creases` and is refused only
where a crease bounds a cell within the reach
(`ChartArmRefusal::CreaseWithinReach`), which is the reading the
integral chart lacks.

Witness: the ridge chart in `topo`'s
`boolean/vtxfac_review_rows.rs` is degree 2 in `u` with a double knot
at `u = ½`. A pierce at `u = ¼`, 0.25 of chart from the crease and
beyond every sector's reach (about 0.21), refuses
`BooleanError::PierceLeverUnread { cause: PatchBound }`
(`rv_a_pierce_far_from_a_crease_reads_its_sides`, which pins that
refusal until this row lands).

Readers that pay for it:
- the boolean's spline pierce lever (`boolean::vtxfac::classify_vertex_on_face`,
  `PierceLever::Within`);
- the seam dihedral's lower arm (`geom_brep::classify_seam`, through
  `AtRestPolicy::spline_chart_arm`), which reads the slack as an
  escalation.

## What it needs

An integral chart's cells read across its creases, as a rational
chart's are, so that a crease outside the reach does not refuse the
arm. A crease within the reach still refuses, as `CreaseWithinReach`.
