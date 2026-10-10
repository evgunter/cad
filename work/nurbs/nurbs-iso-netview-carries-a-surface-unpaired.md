---
id: nurbs-iso-netview-carries-a-surface-unpaired
kind: issue
title: nurbs_iso's NetView borrows a surface's vectors, net and weights unpaired, and re-checks the counts
status: open
opened: 2026-10-10
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
