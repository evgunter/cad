---
id: closed-form-limb-2-escalates-at-eps-1e-12-on-ordinary-geometry
kind: issue
title: Limb 2 for every closed-form carrier escalates at eps 1e-12 on ordinary geometry: the composite's own rounding at the part's scale lands in the band
status: open
opened: 2026-10-26
priority: P0
cost: M
parent: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
design: true
---



Found by conic unit 1's PR 5728 (branch `offset/limb2-for-every-carrier`),
on its PR gate's `eps 1e-12` rows (run on head `75a2e1cd34`, 2026-10-26).
It blocks that PR's gate.

**What fails.** About 45 rows in `sweep::all` and `topo` (lib and `all`)
refuse at `eps 1e-12` with limb 2's escalation,
`CertifyError::AnalyticRung3(AnalyticRung3Refusal::Escalated { limb: HullSup, .. })`:
the closed-form carrier's certified distance bound lands inside the
ambiguity band `(1e-12, 1e-11)`. The bounds read:

- plane operands at coordinates near 1 km: 1.0e-12 to 3.5e-12
  (`long_edges_on_one_line::*`, the six-kilometre bar;
  `section_end_poses::*` at `z0 = 1000`;
  `a_seamless_pipe_cut_by_an_oblique_plane_measures`;
  `verbs_tubewall_r1_probes::*`; `split_edge_on_faces_poses`);
- tori with `R = 10`: 1.1e-12 to 9.6e-12 (`verbs_tubewall_r2_*`,
  `mate7a_torus_rest::*`, `germ_plane_torus_frame`,
  `boolean::section_cert::section_cert_rows::*`, `boolean::boxes`'
  torus rows);
- cylinders, cones and spheres at unit scale: 1.1e-12 to 9.7e-12
  (`boolean::boxes`' conic rows, `chord_join`, `boolean::join`,
  `tier3_tests::a_far_thin_curved_body_is_read_by_its_exact_volume`,
  `near_circular_sections`, `reach_cone_split`, the sphere zone row).

One row refuses outright: `review_m3_pr2::r5_crossing_vertex_on_is_declared_not_measured`,
a plane limb bound of 1.47e-11. One census moves:
`shallow_arc_extrude_grid_interval` goes from `(24, 21, 15, 0)` to
`(22, 21, 17, 0)`. The cone rows that refused `NoOffsetBound` at the
apex are fixed on the branch (the apex root, `Meters::apex_root`).

**Why.** The bound is the canonical composite's enclosure converted to
metres (`pcurve_cache::closed_form_offset_sup`,
`pcurve_cache/projected.rs` `offset_rounds` and `net_incidence`), in
`f64`-endpoint interval arithmetic. Its width has a floor set by the
composite's own rounding at the part's scale, and halving does not lower
that floor. A plane at 1 km has `|p|·u ≈ 1e-13` per coordinate, carried
through the frame's lift. A quartic torus composite with `R = 10` has
terms near `R⁴ = 1e4`, so `1e4·u ≈ 2e-12` before the metres factor
divides it. At the default `eps` (1e-9) that floor sits three orders
under the band; at 1e-12 it sits inside it. The sampled check on `main`
measured the same carriers at points, in `f64`, where the residual of a
line on a plane is usually exactly zero.

**The question.** C2 (ratified on PR 5652, restated on PR 5689) owes
limb 2 by every carrier "in certification arithmetic". At `eps 1e-12`,
`f64` interval arithmetic cannot state a bound under `ε` for ordinary
closed-form geometry beyond unit scale, so the literal reading refuses
these bodies. The candidates, none weighed yet:

1. Accept the refusals at 1e-12 as the certificate's honest limit, and
   re-pin the rows.
2. State the closed forms' limb 2 exactly where the residual has finite
   degree, so the bound's floor is the rounding of a few terms rather
   than of a composite. A line against a plane, cylinder, cone or sphere
   is a quadratic in `t`, and a circle or ellipse against a quadric is a
   degree-2 trigonometric polynomial (`harmonic::harmonic_sup`, the cost
   row's candidate cut). This lowers the floor; it does not remove it.
3. Read the frame at the carrier, not at the surface's origin (a plane's
   `origin` far from the edge adds its distance to every term).

This is a design question for Ev, through the designer pair (`[ev]`).
