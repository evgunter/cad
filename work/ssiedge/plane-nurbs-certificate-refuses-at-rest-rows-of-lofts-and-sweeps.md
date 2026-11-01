---
id: plane-nurbs-certificate-refuses-at-rest-rows-of-lofts-and-sweeps
kind: issue
title: the plane x NURBS certificate refuses the at-rest boundary rows of 44 lofts and sweeps, so their cap rims cannot be stated as Intersection (D2's prefer-intrinsic on NURBS-adjacent edges)
status: open
opened: 2026-10-08
priority: P2
cost: H
refs: [plane-nurbs-limb-two-refuses-a-non-row-section, plane-nurbs-certificate-bound-does-not-refine-with-eps, ssi-limb-three-takes-the-widest-one-arc-rungs-band-verdict, shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam]
---


Filed by SHELL's `shell/oblique-corner-derives` lane (unit 9). The
unit's design decided that the loft states its cap rims as
`Intersection { cap, wall }` at rest, its carrier the wall's own
boundary row (the extrude's Phase-6 idiom; D2's NURBS-adjacent
exemption waives the demand, not the preference). Measured, that
upgrade cannot land: the plane × NURBS certificate refuses the at-rest
rows of bodies that build today, so the step was dropped.

## Measured

The upgrade as built: for every cap rim whose wall is a polynomial
spline, at a scalar holding a NURBS lane, `set_edge_curve` with
`Intersection { cap, wall }` and the carrier `boundary_iso_v(wall, end)`
run with the rim. `cargo nextest run -p sweep`, release, default ε:
**44 rows that build today refuse**, against 10 with the upgrade
disabled (all 10 unrelated to it). The certificate's refusals:

- `PlaneNurbs(TubeNotOneArc { rungs: 20, cause: Undecided(Indeterminate
  { margin: Invalid … }) })` — limb 3 cannot decide; the most common:
  helical sweeps (`m8_14_long_turn_sweep`, 5 rows), ducts and curls
  (`turning_orientation`, 3; `bool6_per_slab_stacking`, 4;
  `bool6_r2_probes`, `bool6r1_probes`), arc-spine lofts past π;
- `TubeNotOneArc { cause: Count { solutions } }` (46 on a twisted duct,
  0 on `transform_nurbs_walls`' rational-walled body);
- `PlaneNurbs(FootPointInconclusive { sample, last_distance ≈ 1e-13 })`
  — the strip loft, the translated arc prism;
- at `Interval`: `PlaneNurbs(Unsupported { "the analytic operand's
  structural parameters are not exact at this scalar" })`
  (`bool6r1_probes_interval`, 2 rows; `r1_area_gauge_probes`);
- knock-ons once the rims change description: `offc_r1_probes` and
  `verbs_offc_consumer` (8), `r1_p2_probes` (3), `m8_4_intersection_iso`
  (3), `m5_s10_face_sense` (3), `tcost_k3_certificate` (2) and about ten
  singles reading pcurves, tier 3, census or mass properties.

## What it is

Each refusal is the certificate's, on a row that lies on both surfaces
by construction (the row IS the wall's boundary and the segment the rim
was swept from). So this is the certificate's defect for these walls,
not the upgrade's shape. Until it certifies them, D2's preference for
an intrinsic description on a NURBS-adjacent cap rim is not met: the
loft rests its rims as images in the cap's chart, declared by the
placed segment, and the offset door derives them when a move tilts
them.
