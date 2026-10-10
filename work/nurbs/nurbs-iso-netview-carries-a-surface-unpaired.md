---
id: nurbs-iso-netview-carries-a-surface-unpaired
kind: issue
title: nurbs_iso's NetView borrows a surface's vectors, net and weights unpaired, and re-checks the counts
status: closed
opened: 2026-10-10
priority: P3
cost: E
branch: nurbs/netview-reads-the-surface
closed: 2026-10-10
pr: 4535
---


Found by the `coefficient-vector-pairing-survivors` sweep.

`crates/geom-brep/src/nurbs_iso.rs` `NetView` borrows `knots_u`,
`knots_v`, `control` and `weights` side by side from a `NurbsSurface`
(`NetView::of`), and the iso doors re-check the counts — the test
`a_net_whose_length_disagrees_with_its_knots_refuses_typed` builds a
short view by struct update to reach that refusal. The surface already
holds the relation; the view unpairs it.

Disposition: read through the surface (or a rational tensor pair, the
type `work/quad/patch-doors-take-a-control-net-beside-two-vectors.md`
asks for), deleting the re-check and the test that reaches it.

## Closed (2026-10-10, PR 4535)

The iso doors (`boundary_iso_u`, `boundary_iso_v`, `interior_iso_u`)
read the surface itself: counts from `NurbsSurface::control_counts`,
the net and weights from its accessors. `NetView`, its count
re-check, the struct-update test that reached it, and
`NurbsSurface::check_net_counts` (whose only caller it was) are
deleted; the count-mismatch arm no longer appears in any door's
`# Errors`. The weight refusals `NurbsCurve3::new` can still name
stay typed. The outputs are bit-identical to main's over a 3000-surface
randomized comparison at `f64` and `Interval` (PR body).
