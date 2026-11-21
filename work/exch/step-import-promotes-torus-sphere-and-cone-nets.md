---
id: step-import-promotes-torus-sphere-and-cone-nets
kind: issue
title: STEP surface promotion reaches torus, sphere and cone (and open circular arcs for curves) on the whole patch before the C0-knot split
status: closed
opened: 2026-10-29
priority: P0
cost: H
closed: 2026-11-21
pr: 7170
---


Ev ruled on `[ev]` PR 6014 (2026-10-29, fork row 216, option (i)): the C⁰-knot split at the import door stands, exact; its cost on foreign rational conics is met by promoting them first, and by per-span quadrature schedules (`work/quad/the-at-rest-gates-cost-grows-with-the-face-count-of-a-split-conic-patch.md`). No spline-to-spline rewrite. Ev on promotion's ε_in: "yes, since in general we're allowed to move geometry by up to eps_in on import" (D7: ε_in governs interpretation).

Owed: `recognize.rs`'s surface promotion reaches torus, sphere and cone, and curve recognition open circular arcs (today's named exclusion), on the WHOLE patch, before `normalize_shell`'s C⁰-knot split, so a promoted carrier has no knots and is never cut. The QUAD acceptance quarter torus (`a_conic_quarter_torus_measures_a_bracket_at_the_wall.rs`, a 9×9-span rational conic patch) is the headline: 162 walls today on FOREIGN's draft PR 6013, two analytic faces after. Two cautions from the designers: the curved-kind certificate is a sampled grid plus envelope that refuses at fine ε even on exactly cylindrical geometry (`recognize.rs` module docs), so a torus track likely wants the tensor-product form of `geom_core::spline::compose`'s hull certificate; and once the fixture promotes, the QUAD test needs a conic that cannot be promoted (a revolved spline profile) to keep exercising the rational lane.

Signed: (FOREIGN orchestrator)

## Closed

The surface half landed in PR 7172 (2026-11-20). Cone, sphere and torus promote on the whole patch, and every kind, the plane and cylinder included, is decided by the one Interval tensor-product certificate `geom_brep::patch_distance_sup`. The QUAD quarter torus imports as two torus walls. Its rational-lane role passes to an elliptic tube that no kind promotes.

The curve half landed in PR 7170 (2026-11-21). Open circular arcs promote whole-carrier, before the C⁰-knot split, with a certified covered interval, and the edge's trim must lie inside that interval. Promotion owes the locus, never the timing: the orchestrator's decision of 2026-11-20 in `log.md`.

Residue has its own rows: `curved-promotion-leaves-apex-cones-and-fat-tori-as-nurbs`, and `solid-adoption-reads-the-printed-carriers-kind-not-the-edges-description` (P1).
