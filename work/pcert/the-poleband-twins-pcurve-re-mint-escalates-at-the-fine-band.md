---
id: the-poleband-twins-pcurve-re-mint-escalates-at-the-fine-band
kind: issue
title: "The poleguard twins' 0.9e-9 m cap reads a definite corner at the farther face's lever, and only the pcurve re-mint's map residual refuses it at 1e-12"
status: open
opened: 2026-10-16
priority: P3
cost: M
design: true
---

`step-import`'s poleguard twins (`tests/fixtures/poleguard/poleband.step` and `polefrustum.step`, an R = 10 mm sphere truncated 9e-8 rad below its north pole, so their near-pole vertices sit 0.9e-9 m from an undeclared pole) refuse at ambient 1e-12 at the pcurve re-mint on the adopted body:

> pcurve re-mint on the adopted body: the pcurve of half-edge … failed certification: pcurve certification: MapResidual at sample 0 escalated: margin 5.9303273403088124e-12 lies inside the ambiguity band (1e-12, 1e-11). Recourse: loosen the tolerance, as a last resort; this refusal may indicate a kernel bug worth reporting

Until TANG's U-lever (PR 5204) adoption refused first, at the rim/sphere near-tangency's second order read at the edge's own length (margin 1.62e-16 m). At the faces' extent that reads determinate, adoption passes, and the import reaches this door. The refusal is loud and typed (`step-import/tests/tier_gate.rs`'s `POLEBAND_PCURVE_REMINT` pins it), but its message asks for a report: a chart image whose map residual near a pole is 5.9e-12 m, in the band at 1e-12, is either the pcurve fit losing conditioning near the pole (where the sphere chart's `u` runs fast) or a real residual of the file's geometry. To close: find which. If the fit, certify the image near the pole (or refuse it typed as a pole feature with a recourse that names it); if the file, say so in the row and the message.

## The lever question this rests on (design; U-lever review F1, PR 5204)

This is not only a pcurve question. The edge the re-mint meters bounds a 0.9e-9 m cap beside an R = 10 mm sphere, meeting it at α ≈ 9e-8 rad.
- **On main** its second-order wedge was read at the edge's own length: 9e-8 × 1.8e-9 = 1.62e-16 m, which refused at adoption.
- **Since U-lever** it is read at the FARTHER face's extent (`topo::wedge_reach`, the approved "min(the faces' curvature radii, the farther face's extent)"): about 9e-10 m, which reads Transverse at the 1e-12 band.
- **But the locus is fixed only to about ε/α ≈ 1e-5 m**, some 10⁴ times the cap's own size. The faces open a definite corner only over a length the smaller face does not have.

Whether the lever should be the NEARER face's extent, or the reading should refuse where ε/α exceeds the smaller face, is a design fork for a later unit; this PR implements the approved text as written. **This item must NOT close by certifying the chart image near the pole** until that question is decided: today the map residual is the only door that refuses this cap, and certifying it would ship an edge whose locus the faces do not fix.


## The import route no longer reaches this door (FOREIGN implementer, 2026-10-29)

`import-normalizes-the-rim-only-cap` gives import a reading that runs before adoption: every sphere pole or cone apex in a face's closure is made a vertex of it, read on, inside or off the boundary at the coarser of the file's band and the run's (`crates/step-import/src/normalize/singularity.rs`). Both twins' near-pole vertices sit 0.9e-9 m from the undeclared pole, inside that band at ambient 1e-12 (the file's ε_in = 1e-10 m, K·ε_in = 1e-9 m), so the import now refuses them there typed, as a pole neither on the boundary nor off it, before the pcurve re-mint is asked: `tier_gate.rs`'s two 1e-12 cells read `POLE_IN_BAND`, not `POLEBAND_PCURVE_REMINT`. The default and 1e-6 cells are unchanged (the vertices read AT the pole there). So the map residual is no longer the only door refusing this cap on the import route; the question above stands for any route that builds the cap without import, and the import fixtures no longer witness the re-mint's refusal.
