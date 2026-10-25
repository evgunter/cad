---
id: a-moved-lofts-top-rim-refuses-its-at-rest-certificate
kind: issue
title: A shelled loft's top rim fails its at-rest plane x fit certificate at 1e-6: the degree-1 33-foot chart image's chord error is limb 2
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [a-wall-seam-between-two-fits-has-no-section]
---

Found by SHELL's NURBS × NURBS section unit (`a-wall-seam-between-two-fits-has-no-section`).
Once the twisted loft's wall–wall seams derive, `topo::shell` of
`twisted_loft(0.3)` at `THICKNESS` (`crates/sweep/tests/encl_curved_loft_shell.rs`,
`shelling_the_curved_loft_refuses_at_a_walls_fit`) at `CAD_TOLERANCE_EPS=1e-6`
refuses at a top-cap rim (cap plane `z = 0.95` × a moved wall's fit),
re-described on the moved charts:

`RechartFalsifies { door: SetFaceSurfacesDescribing, edge: EdgeKey(12v1), error:
Escalated { check: PlaneNurbsHull, cause: Indeterminate { margin:
1.9785634851144333e-6, band: (1e-6, 1e-5), predicate: "ssi_hull_sup_chart" } } }`

Re-measured 2026-10-11 on main's refined limb 2 (`encl/hull-bound-refine`): the
same rim, `Escalated { check: PlaneNurbsHullValue, cause: Indeterminate {
margin: 1.0167785956182296e-6, .. } }`, limb 2's value at a break of
its composite. The 257-sample measurement below predates that
refinement.

At the default ε 1e-9 the rim was unreached until SHELL's PR 4672 let
the walls' offset fits certify there. It now refuses there outright,
past the band: `RechartFalsifies { door: SetFaceSurfacesDescribing,
edge: EdgeKey(12v1), error: PlaneNurbs(Limb { limb: HullValueChart,
margin: Value(1.599441575127619e-6) }) }` (1.6378e-6 before main's 2026-10-11 rim changes). It is pinned per ε in the
same row.

The raising site is `edge_nurbs::plane_nurbs_limbs`, whose chart image
is `chart_image`'s degree-1 interpolant through `PXN_FIT_SAMPLES` (33)
foot points. Measured, not argued: with the constant raised to 257 on a
scratch build the same rim certifies, and the shell proceeds past
every edge to tier 3, where it stops at
`work/quad/a-fitted-cap-cut-by-planes-has-a-sub-range-trim-image.md`'s
refusal. So the image's own chord error on this curved rim is what limb
2 bounds. The section's own certificate (the march's traced pcurve)
passed at derivation.

Owed: an at-rest image whose own error does not dominate the edge's
limb 2 on a curved rim: a higher-degree or adaptive image of the
carrier on the wall, keeping the one-producer rule (`chart_image` is
also the pcurve mint's producer).
