---
id: closed-form-limb-2-escalates-at-eps-1e-12-on-ordinary-geometry
kind: issue
title: Limb 2 for every closed-form carrier escalates at eps 1e-12 on ordinary geometry: the composite's own rounding at the part's scale lands in the band
status: closed
opened: 2026-10-26
priority: P0
cost: M
parent: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
pr: 5728
closed: 2026-10-26
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

At `eps 1e-6` the same escalation reaches two rows:
`sym11_far_placement_rows::sym11_the_bare_lifts_answer_typed_at_every_placement`
(a cylinder at a far placement, 3.3e-6) and
`split_edge_on_faces_poses::an_edge_at_the_zero_bands_edge_refuses_typed_and_its_lever_builds`
(a plane, at the band's edge by construction). `mesh::all
perf1_torus_sizing::torus_sizing_sweep_is_sound_tight_and_on_the_ideal`
refuses at 1e-12 (a torus limb bound of 1.75e-11). One row refuses outright: `review_m3_pr2::r5_crossing_vertex_on_is_declared_not_measured`,
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

**Weighed (fork row 203, 2026-10-26).** Both designers say these refusals are not the certificate's honest limit:
- **The cause.** A closed form's net is enclosed in world coordinates, so the bound rounds at the part's distance from the origin.
- **The fix.** One limb-2 reader serves every carrier: the carrier's net, framed at each operand in certification arithmetic once. `closed_form_offset_sup` goes, and so does the spline path's unenclosed `f64` framing.
- **The verdict.** A bound that stalls above the band refuses as the kernel approximation limit `[tolerance.recourse]` names, carrying the bound, the extent and the carrier's enclosure width. It is never an escalation and never a definite miss.

Ev's ruling is asked on PR 5728.

## Closed

Ev approved the designers' answer on PR 5728 ("this seems good",
2026-10-26), and the same PR builds it.

- **One limb-2 reader for every carrier**,
  `crates/geom-brep/src/pcurve_cache.rs` `limb2_offset_sup`.
  - It reads the carrier's net through
    `crates/geom-brep/src/carrier_net.rs` `framed_carrier_net`, framed at
    each operand in certification arithmetic. The frame is applied to the
    carrier's own data before anything is combined: a closed form's
    centre and axes, a line's points at the breaks, a spline's controls.
  - `closed_form_offset_sup` and `projected::net_offset_sup` are gone.
  - The spline path's `f64` framing, which was unenclosed, is gone with
    them. The row `carrier_net::tests::a_spline_framed_at_an_operand_encloses_its_exact_framed_controls`
    pins its replacement.
- **The verdict.** Limb 2's upper bound either clears the band or refuses
  as `AnalyticRung3Refusal::KernelLimit`.
  - The refusal carries the bound, the net's width and the net's extent,
    plus the classifier's terminal bit where the bound lay in the band.
  - Its one story reads which limit the data show: the edge's extent, the
    carrier's radius, or the composite's conditioning.
  - `Limb { HullSup }` is removed, and `Limb` is now the measured miss
    alone.
- **Rows**: far placements now certify at ε = 1e-12. What still refuses
  is listed on PR 5728 and filed as four follow-ups:
  - `the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding`
  - `the-closed-form-net-builder-rounds-at-a-km-radius-not-at-the-span`
  - `the-session-box-is-not-enforced-at-construction`
  - `eps-min-leaves-no-certification-headroom-at-the-box-edge`
