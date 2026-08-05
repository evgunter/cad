# Predicate-comparand dimensional audit (M7, rim-dimensional unit)

Every decision-boundary comparand in `geom-brep` and `topo` (every
`classify`/`require_zero`/`require_extent`/`decide` funnel call and
every raw `sign_within` use), audited for the ratified ε semantics
(D4): **a margin classified against the linear band must be a LENGTH
in meters — the point deviation from specified geometry**. Angles and
dimensionless quantities meter through a named lever arm (θ·r);
squared quantities are rooted (or divided by a length, the `/2r`
linearization) before comparison; a product of two lengths is an
area-dimensioned defect (the class of the fixed `du_of_rims` bug and
the #89 in-band landing).

Trigger and method: the `props_rim_level_group` defect (fixed in this
unit — `crates/geom-brep/src/props/curved.rs`, the `RimLevel` enum)
metered a cone's already-length rim-level difference by `× arm`,
manufacturing an area. This document is the systematic sweep for the
rest of the family, and the input to the typed-margin (Length-typed
classify seam) design conversation. Status column: OK (dimension
verified), FIXED (corrected — in the originating rim-dimensional unit
unless the row names the follow-up unit that did it), FLAG (defect or
concern found — disposition in the findings section).

**Living document.** A row and its disposition entry must never
disagree; retiring a finding updates both. Retired so far: F1 plus the
eight inline class-(c) sites (rim-dimensional unit, #197); F5 and most
of F6 (M6-3, #192); F3 and F4 (the F3+F4 dimensional unit — F3 was the
tree's last funnel bypass, so predicate-name attribution in the K
telemetry is now complete).

Factor conventions used throughout (verified against definitions):
`Curve3::Line.dir` unit ⇒ line parameter is arc length (m);
`Circle`/`Ellipse` parameters are radians with radii/semi-axes in m;
all stored surface axes/normals/`u_ref` unit; `implicit_residual` is
`/2r`-normalized to meters; `implicit_gradient` unit on-locus;
`curvature_lever_arm` meters; `TangentJet.kappa_rel` 1/m;
`speed_lower_bound()` meters per parameter unit.

## geom-brep

| site | predicate | comparand | dim | status |
|---|---|---|---|---|
| dihedral.rs:140 | dihedral_arm | min(curvature arms, extent) | m | OK |
| dihedral.rs:151 | dihedral_wedge | sinθ (unit-gradient cross) × arm | m | OK |
| enters.rs:84 | enters_material_arm | caller arm (contract: m) | m | OK |
| enters.rs:95 | enters_material | cos(unit,unit) × arm | m | OK |
| enters.rs:141 | tangent_sector_order2_arm | caller arm | m | OK |
| enters.rs:153 | tangent_sector_order2 | normal curvature (1/m) × arm²/2 | m | OK |
| newell.rs:165 | newell_plane_residual | (p−centroid)·n̂ | m | OK |
| certify.rs:849/858 | interval_span_forward/winding (Circle) | span·radius / (τ−span)·radius | m | OK |
| certify.rs:872/877 | interval_span_forward/winding (Ellipse) | span·minor (conservative) | m | OK |
| certify.rs:883 | interval_span_forward (Line) | span (t IS arc length) | m | OK |
| certify.rs:897 | nurbs_span_meter | speed_lower_bound() | m/param | FLAG F7 |
| certify.rs:908 | interval_span_forward (Nurbs) | span × (m/param) | m | OK |
| certify.rs:917/925 | carrier_endpoint_start/end | point distance | m | OK |
| certify.rs:950/958/992/1000 | carrier/tangent_on_surface_1/2 | implicit_residual (/2r-normalized) | m | OK |
| certify.rs:1015 | tangent_second_order | κ_rel·arm²/2 | m | OK |
| certify.rs:1035 | tangent_normal_parallel | sinθ / κ_rel (arm = 1/κ_rel, the D4 ¶1 tangency lever) | m | OK (note N4) |
| certify.rs:1047 | carrier_matches_mapped_source | point distance | m | OK |
| certify.rs:1057/1068/1076 | carrier_on_seam_* | residual / radial·unit | m | OK |
| certify.rs:1103/1112 | tangent_hull_sup / tube_margin | m residual sums; κ·arm² | m | OK |
| certify.rs:1143/1151/1162 | witness_* | residuals / point distance | m | OK |
| intersect.rs:554/560/591 | pc_axis_plane_parallel / parallel_gap / rim_alignment | sin×extent; r−gap; sin×r | m | OK |
| intersect.rs:694 | ps_frame_seam | (sin−0.5)·r — deterministic frame tie-break, not a coincidence question | m | OK (note N5) |
| intersect.rs:705 | ps_center_gap | r − center-plane distance | m | OK |
| intersect.rs:834–882 | cc_* (radius eq, axes parallel, coaxial, gap, coplanar) | lengths / sin×extent / common-perpendicular | m | OK |
| intersect.rs:1000/1006/1043 | pn_apex_*, pn_axis_normal | m·unit; trig diff×extent; sin×rim r | m | OK |
| pcurve_cache.rs:1225/1233 | pcurve_chart_azimuth_affine / winding | (rad coeff)×radius | m | OK |
| pcurve_cache.rs:1268 | pcurve_map_residual | mapped point distance | m | OK |
| pcurve_cache.rs:1315 | pcurve_interval_forward (harmonic) | span × param_rate | m | OK |
| pcurve_cache.rs:1327 | pcurve_azimuth_period | (τ−span)·radius | m | OK |
| pcurve_cache.rs:1471 | pcurve_trim_containment | chart-param overhang × chart_arms | mixed | FLAG F6 |
| pcurve_cache.rs:1528 | pcurve_interval_forward (fitted) | NURBS param span × 1 | dimensionless | FLAG F6 |
| pcurve_cache.rs:1542 | pcurve_azimuth_period (fitted) | rad headroom × u_arm (1 for cone/torus) | mixed | FLAG F6 |
| pcurve_cache.rs:1664 | pcurve_chart_radial_moving | Σ m-norms BARE (amplitude is metres) | m | FIXED (M6-3) |
| pcurve_cache.rs:1680/1772/1791 | pcurve_chart_orientation / sphere meridian | m² ÷ radius | m | OK |
| pcurve_cache.rs:1752 | pcurve_sphere_chart_frame | m at :1770, dimensionless at :1836 (tie-break) | mixed | FLAG (note N5) |
| pcurve_cache.rs:1759–1829 | pcurve_sphere_chart_* | m-scaled coefficients / rooted | m | OK |
| props/curved.rs:145/150 | props_rim_axis_parallel / center_on_axis | sin×r_c; perpendicular offset | m | OK |
| props/curved.rs:208 | props_rim_level_group (Length) | level difference BARE (v is arc length) | m | FIXED (this unit) |
| props/curved.rs:211–212 | props_rim_level_group (Unit) | Δ(sin,cos) × arm | m | OK (note N1) |
| props/curved.rs:250 | props_rim_dir_group | (±1 diff) × arm ∈ {0, ±2·arm} | m | OK (note N2) |
| props/curved.rs:263 | props_du_consistent | Δu (rad) × arm | m | OK |
| props/curved.rs:286 | props_rim_side | per-kind: bare (Length) / ×arm (Unit) | m | FIXED (this unit) |
| props/curved.rs:313/421 | props_meridian_axial / generator | sin (or cos-diff) × parameter span (m for lines) | m | OK |
| props/curved.rs:323/343/451/553/677 | props_meridian_on_surface / rim_fit (all kinds) | residuals; sphere/torus fits ROOTED before compare | m | OK |
| props/curved.rs:337/444/549/671 | props_circle_axis_class | cos × r_c | m | OK (note N3) |
| props/curved.rs:372/484/589/740 | props_face_extent | m levels; sin-levels ×R; dt×minor | m | OK |
| props/curved.rs:430 | props_meridian_apex | apex-line distance | m | OK |
| props/curved.rs:487/488 | props_cone_nappe | slant levels (m) bare | m | OK |
| props/curved.rs:576/598/695/706/728 | sphere/torus meridian checks | lengths / sin×R / cos×minor | m | OK |
| props/curved.rs:777 | props_rim_level | rooted (sin,cos) chord × minor | m | OK (note N1) |
| props/quad.rs:453 | props_quad_converged | ε·F − flux-width(m³)/(3·area(m²)) | m | OK |
| props/quad.rs:461 | props_quad_face_extent | area/perimeter (mean width) | m | OK |
| ssi.rs:645 | ssi_cs_tangency | radius/axis distance differences | m | OK |
| ssi/certify.rs:366–524 | ssi_on_locus / hull_sup / foot / chart | residuals, /2R linearizations, foot distances | m | OK |
| ssi/certify.rs:836 | ssi_tube_transversality | sin (unit triple product) × arm | m | OK |
| ssi/march.rs:295/310 | ssi_transversality_arm / ssi_transversality | arm (m); sin × arm | m | OK |
| ssi/march.rs:420/447/478 | ssi_step_progress / branch_open_end / closure_return | state × (m/state); scaled domain margins | m | OK |
| ssi/march.rs:484 | ssi_closure_tangent | cos(unit tangents) × whole-branch arc length | m | FLAG F9 |

## topo

| site | predicate | comparand | dim | status |
|---|---|---|---|---|
| boolean/contain.rs:83–115 | bool_contact_vertex/edge_span/edge | point/span/perpendicular distances | m | OK |
| boolean/insert.rs:197 | bool_strut_order | (unit germ dir diff)·(unit e_dir) × min sector arm | m | FIXED (was dimensionless); verified CODE-READ + suites-green only — the rare germ-fan lane fires in none of the unit's live twin/probe configs (review MINOR-2, stated) |
| boolean/insert.rs:262 | bool_germ_line | sin(n̂_a,n̂_b) × min sector arm | m | OK |
| boolean/join.rs:575/611/808/822 | bool_join_nearest | point distances / differences | m | OK |
| boolean/join.rs:743/744 | bool_join_facing | unit germ dir · chord (cos × separation) | m | FIXED (was bare cosine, `/dist`) |
| boolean/join.rs:750/751 | bool_join_arc_facing | axis·((p−c)×dir) — radius-metered sine | m | OK |
| boolean/join.rs:1093 | bool_ring_run_winding | (n̂ · Newell sum) / run perimeter — 2A/P, the run's mean width | m | FIXED (F4; was a bare **m² AREA**) |
| boolean/ops.rs:663/679 | volume_backstop_operand / volume_backstop | V/A; ΔV/(A_got + A_bound) — mean boundary displacement | m | FIXED (F3; was **m³ VOLUME** through the tree's one raw `sign_within`) |
| boolean/ops.rs:1194–1480 | bool_sphere_* | radius/gap differences; sin × radius | m | OK |
| boolean/plane_eq.rs:174/233 | bool_plane_parallel | sin(n̂1,n̂2) × arm | m | OK |
| boolean/plane_eq.rs:190/252 | bool_plane_orient | cos(n̂1,n̂2) × arm | m | FIXED (was bare cosine) |
| boolean/plane_eq.rs:203/265 | bool_plane_offset | signed-offset difference | m | OK |
| boolean/recl.rs:224–748 | side_code / bool_dir_same / bool_ee_collinear | cos/sin × sector arms | m | OK |
| boolean/reduce.rs:548–802 | bool_vertex_face_side / circle & line clearances | plane residuals, /2r residual extremes, sagitta dips | m | OK |
| boolean/rest.rs:399/413 | bool_join_nearest | distances | m | OK |
| boolean/rest.rs:405/406 | bool_join_facing | unit dir · chord | m | FIXED (was bare cosine) |
| boolean/sectors.rs:147–432 | bool_sector_* / bool_dir_* / bool_faces_parallel / side_code | sin/cos × sector arm (arm = shorter bounding chord, m; every caller passes unit dirs — verified) | m | OK |
| boolean/solid_contain.rs:438 | bool_wall_trim_period | (τ−width)·radius | m | OK |
| boolean/solid_contain.rs:462 | bool_wall_trim (cone term) | (cosΔ−cos h)·radius — effective arm sin(h)·r, collapses for narrow windows | m | FLAG F8 |
| boolean/solid_contain.rs:538/562/587 | bool_point_in_solid_plane | plane residual; /2r linearizations | m | OK |
| boolean/solid_contain.rs:645/655 | bool_point_in_solid_advance/order | ray parameters (m, unit dir) | m | OK |
| boolean/solid_contain.rs:675 | bool_point_in_solid_denom (plane) | cos(unit,unit), no arm | dimensionless | FLAG F2 |
| boolean/solid_contain.rs:720 | bool_point_in_solid_denom (cylinder) | sin²/2r | **1/m** | FLAG F2 |
| boolean/solid_contain.rs:731 | bool_ray_cylinder_disc | disc/(2r)² (self-documented, F3 of PR 9c) | dimensionless | FLAG F2 |
| boolean/solid_contain.rs:753 | bool_point_in_solid_denom (sphere) | (unit·radial)/radius | dimensionless | FLAG F2 |
| boolean/solid_contain.rs:804/856 | bool_ray_sphere_disc / at_infinity | disc/2r; volume/area | m | OK |
| boolean/vtxfac.rs:106/113/453 | side_code / bool_sector_coplanar / bool_germ_line | cos/sin × sector arm | m | OK |
| census.rs:313–599 | pm_census_vv/ve/vf/ef gaps, spans, residuals | point/line/plane distances and spans (unit dirs verified) | m | OK |
| census.rs:614–746 | pm_census_span_* / ee_gap / ee_span / ee_overlap | span arithmetic (m) | m | OK |
| census.rs:666 | pm_census_ee_parallel | sin(unit dirs) × min(edge lengths) | m | FIXED (was bare sine) |
| census.rs:812/831 | pm_census_confirm_* | distances / residuals | m | OK |
| merge_faces.rs:924 | bool_ring_run_winding | (n̂ · Newell sum) / loop perimeter | m | FIXED (F4) |
| pcurves.rs:508–717 | pcurve_loop_continuity / closure(_height) | Δu(rad)×azimuth_arm; Δv (m on cylinder charts) | m | OK today; FLAG F6 (non-cylinder fallback arm = 1) |
| split.rs:197 | split_edge_param_interior | param spans × per-kind rate (1 / radius / minor / speed bound) | m | OK |
| transform.rs:139 | transform_rigid_* (7 residuals) | unit-column/orthogonality/det residuals, no arm | dimensionless | FLAG F10 |
| transform.rs:155 | transform_rigid_trans_finite_* | t·0 poison probe (0 or NaN by construction) | — | OK |
| validate.rs:1662/1847 | planar_face/boundary_residual | plane residuals | m | OK |
| validate.rs:1795 | tangent_second_order | κ_rel × arm²/2 | m | OK |
| validate.rs:2030 | bool_ring_run_winding | (outward · Newell sum) / loop perimeter | m | FIXED (F4) |
| validate.rs:2014 | positive_volume | volume/surface-area (the documented dimensional fix) | m | OK |
| splitting/classify.rs:81–286 | split_vertex_side / conic lane | plane residual; rooted amplitude; (rad)×minor semi-axis | m | OK |
| splitting/containment.rs:179/192/219/233 | point_in_loop boundary/side/advance | distances; m²/m advance | m | OK |
| splitting/containment.rs:203 | point_in_loop_arm | sin(member, plane normal) × loop extent (the member's in-plane fraction) | m | FIXED (was dimensionless schedule norm) |
| splitting/neighborhood.rs:226–324 | split_conic_departure / sector reflex/straight / bisector | tangent×extent projections; sin/cos × arm | m | OK |
| splitting/order.rs:73 | split_join_frame_arm | sin(member, plane normal) × points' spread (the member's in-plane fraction) | m | FIXED (was dimensionless schedule norm) |
| splitting/order.rs:111 | split_join_order_u/v | coordinate difference (m) vs the EXACT bit-level band (deliberate total-order device, documented) | m | OK (note N6) |
| splitting/rules.rs:130/149/197 | split_sector_extent / coplanar / enters arm | extent; sin×extent | m | OK |
| splitting/rules.rs:174 | tangent_sector_osculation | κ(1/m) × face-extent²/2 | m | FLAG F11 |
| splitting/join.rs:795/1261 | split_sphere_section_polar | sin(axes) × sphere radius | m | OK |
| splitting/join.rs:883 | split_tangent_chord_forward | dimensionless param diff × ‖dir‖ | m | OK |
| splitting/join.rs:973/1366 | split_arc_window (×5 each) | azimuth (rad) × chart radius | m | OK for cylinder; FLAG F8 for the sphere wall (arm R vs local R·cos lat) |
| splitting/join.rs:1040/1386 | split_arc_chart_orientation | cos × semi-major (= r for the plane×cyl ellipse) | m | OK |
| splitting/join.rs:1549 | split_conic_inplane_mid | plane residual at midpoint | m | OK |
| splitting/join.rs:1597 | bool_between_arc_window | (cosΔ−cos h)·r_c — quadratic in the angular deviation for narrow windows | m | FLAG F8 |
| splitting/join.rs:1619 | split_chart_azimuth_frame | radial·u_ref (m) — branch selection | m | OK (note N5) |
| splitting/join.rs:1750/1756 | split_sphere_window_pole(_side) | radius − axial distance | m | OK |
| splitting/join.rs:2251 | split_section_area | 2·\|A\|/P mean width | m | FIXED (factor-2 doc/code mismatch; dimension was already m) |
| splitting/finish.rs:414 | classify_dihedral arm | edge extents (m) | m | OK |

Funnel bypasses found: **boolean/ops.rs:634/649** (`sign_within`
called directly on volume margins — was FLAG F3, **FIXED**: both
gates now route through `k_stats::decide` under
`volume_backstop_operand` / `volume_backstop`). **The tree has no
funnel bypass left** — every shipped decision in geom-brep and topo
goes through `k_stats::decide`, so every margin the recorder sees is
attributed to the predicate that actually decided it. Raw ε reads
outside decisions: solver
tolerances and step-size control in ssi (documented structure
parameters), `props.rs` trig pad (ε/radius, an enclosure pad, not a
decision), test fixtures.

## Findings (dispositions)

Fixed in this unit. Live-pin coverage, honestly (review MINOR-2):

- **F1 (the unit's trigger)** `props_rim_level_group`/`props_rim_side`
  — per-kind metering via the `RimLevel` enum; the #89 in-band landing
  retired (margin now the true rim separation, scale-linear). Pinned
  by `crates/geom-brep/tests/rim_dim_scale_twins.rs` and the
  grouping-flip probes (`rim_dim_review_probes.rs`, adopted by merge).
- `bool_join_facing` (×2 files), `pm_census_ee_parallel`,
  `point_in_loop_arm` — live in the boolean scale-twin pin
  (`crates/topo/tests/rim_dim_boolean_twins.rs`).
- `bool_plane_orient` (×2 rungs), `split_join_frame_arm`,
  `split_section_area` (factor-2 aligned to its documented spec) —
  live in the adopted review probes' flush-subtract + oblique-split
  linearity test (`crates/topo/tests/rim_dim_review_probes.rs`).
- `bool_strut_order` — CODE-READ + suites-green only; the rare
  germ-fan lane fires in none of the above configs.

Fixed by the **F3+F4 dimensional unit** (the follow-up unit this
audit's F3/F4 rows banked; both findings were EXECUTED, not
speculative — each row below states what it measured):

- **F3** `volume_backstop` (ops.rs): the tree's ONE funnel bypass is
  gone. Both gates decide through `k_stats::decide` — the bound check
  under `volume_backstop` (margin `ΔV / (A_got + A_bound)`: a boundary
  displaced by δ moves the volume it encloses by ≈ δ·A, so the summed
  surface area of the two compared bodies is the whole boundary that
  could have produced the defect, and the quotient is the mean boundary
  displacement the violation corresponds to) and the operand-bounded
  test under `volume_backstop_operand` (`V/A`, verbatim the
  validate.rs `positive_volume` precedent). Both quotients are exactly
  zero when the volumes agree exactly, so the gate's non-strict pass
  direction is unmoved. The attribution defect this closes was measured
  in `rim_dim_boolean_twins` at ε = 1e-12: the operand/result VOLUME
  set {1, 1, 3, 8, 8, 16} m³ logged under certify's
  `witness_at_mid_parameter` (cubic, ×1e-9 between the twins) and
  perturbing that predicate's sample COUNT (102 vs 103). Post-fix the
  volumes appear under their own names and scale ×1000, and
  `witness_at_mid_parameter`'s decisive list is EMPTY at both scales —
  its real samples are coincident residuals, exactly as the old
  allowlist comment claimed.
  **Third executed consequence, found by this unit and NOT previously
  known: the cubic comparand was silently switching the backstop
  OFF at mm scale.** `bounded` classified a raw m³ volume against the
  linear band, so a 2 mm cube's 8e-9 m³ landed inside
  `Band{1e-9, 1e-8}` at the default ε — indeterminate — and the code
  reads an indeterminate operand as "not certifiably bounded" and
  SKIPS its bound. The whole `vol(A∖B) ≤ vol(A)` check was therefore
  vacuous on that boolean. Measured as a one-line census delta on the
  twin fixtures (`witness_at_mid_parameter` 123 samples/5 nonzero →
  118/0, plus `volume_backstop` 2/2 and `volume_backstop_operand` 4/4;
  every one of the other 49 predicates byte-identical): five volume
  decisions became six, the sixth being the restored check, which
  passes. Metered as `V/A` the same operand answers 3.3e-4 m —
  decisively bounded at every ε in the matrix. No verdict flipped; a
  gate that had been skipping now runs.
- **F4** `bool_ring_run_winding` (join.rs, merge_faces.rs,
  validate.rs — one predicate, three sites, all three moved together):
  the Newell AREA is divided by the region's boundary PERIMETER, giving
  `2A/P` — the ring's MEAN WIDTH, the distance the boundary would have
  to move to sweep the enclosed region away, and the same quantity
  `split_section_area` already meters. The canonical derivation lives
  at `boolean::join::ring_run_ccw`; the other two sites cross-reference
  it. In the join's ring-run lane the perimeter is arc-aware (conics
  contribute `|Δ|·semi-major` — exact for a circle, an upper bound for
  an ellipse, and an over-large P escalates rather than decides) and
  includes the chord that closes the open run. This retires an
  EXECUTED in-band refusal: at ε = 1e-6 the mm pocket-subtract twin
  refused typed on a 2e-6 m² margin inside Band{1e-6, 1e-5}; the same
  decisions now carry 5e-4 / 7.5e-4 / 1e-3 m and compute on every ε row
  in the hosted matrix. `rim_dim_boolean_twins`'s three-outcome F4
  signature match is deleted, and the predicate is pinned LINEAR.

Flagged, NOT fixed here (dispositions):

- **F2** `solid_contain.rs` ray-caster denominators (675/720/731/753):
  dimensionless and 1/m comparands. The cylinder-disc site carries an
  in-tree admission earmarking a re-pin unit (PR 9c review F3). One
  coordinated unit should meter all four (the sphere-disc form at :804
  is the model). Reported, deferred to that unit.
- **F3** — **FIXED by the F3+F4 dimensional unit** (see the fixed
  list above).
- **F4** — **FIXED by the F3+F4 dimensional unit** (see the fixed
  list above).
- **F5** `pcurve_chart_radial_moving` — **FIXED by M6-3** (the
  loft-assembly unit, PR #192): the amplitude is compared BARE (it is
  already a displacement in metres; the ×radius factor made it an
  area). The predicted retirement executed exactly: the freecad
  `CORPUS_EPS_CEILING` moved 1e-8 → 1e-5 (the 1e-7/1e-6 refusals were
  this comparand's artifact; at 1e-4 the attachment/span gates refuse
  at the corpus's true feature scale — table re-measured in
  step-import/tests/freecad.rs, composing #197's F-row retirement).
  In-band amplitudes take the meridian arm as a D9 tie-break, the
  discarded drift carried by check 4's envelope in metres.
- **F6** pcurve chart arms — **mostly closed by M6-3** alongside the
  chart completion: `chart_arms` now answers (r, r) for spheres and
  (R+r, r) for tori, the cone's azimuth arm comes from the
  containment check's own boxes (`chart_arms_at`, v_sup·sin α), and
  `pcurves.rs::azimuth_arm` is the LOCAL lever (r·cos v etc. — zero
  at poles/apex, which the walk exploits). Still bare: the
  fitted-lane `pcurve_interval_forward` NURBS param span (a
  reparametrization-sensitive rate) — fold into F7's typed-margin
  design.
- **F7** `nurbs_span_meter` (certify.rs:897): a RATE (m/param) gated
  against the linear band — reparametrization-sensitive. Fold into the
  typed-margin design. (The certify.rs collision claim is STALE as of
  the F3+F4 unit — the loft-assembly lane merged; deferred on its own
  merits now, not on a file conflict.)
- **F8** window/cosine family: `bool_between_arc_window` (cosΔ−cos h,
  quadratic near narrow/full windows), `bool_wall_trim` cone term
  (same shape, conservative direction), sphere-wall `split_arc_window`
  arm (R vs local parallel radius R·cos lat — over-generous near
  poles, the anti-conservative direction). One family, one fix shape
  (linearized angular margin × honest local arm); own unit.
- **F9** `ssi_closure_tangent` (march.rs:484): cos × whole-branch arc
  length — an unbounded arm (nothing folds in `ctx.extent`).
  Arm-policy question for the design conversation.
- **F10** `transform_rigid_*` (transform.rs:139): dimensionless
  rigidity residuals of the linear map against the metre band; the
  natural arm is the model/session-box extent. (The transform.rs
  collision claim is STALE as of the F3+F4 unit — the loft-assembly
  lane merged. Deferred on the arm question alone, which is a design
  input, not a conflict.)
- **F11** `tangent_sector_osculation` (rules.rs:174): sagitta model
  κ·L²/2 metered at the WHOLE-FACE extent, squared, and invalid for
  κ·L ≳ 1 — over-refusal direction. Arm-policy question; own unit.

Notes (verified honest, kept for the design conversation):

- **N1** Torus `props_rim_level_group` levers Δ(sin v, cos v) at
  `major`, while the induced point deviation levers at `minor` (the
  sibling `props_rim_level` uses × minor). Overstates by R/r —
  conservative (escalates a truly-coincident pair, never merges a
  distinct one). Lever-magnitude question, deferred to typed-margin.
- **N2** `props_rim_dir_group` compares a structural ±1 through the
  numeric funnel (margin 0 or ±2·arm). Guarded upstream: a rim with
  arm ≲ K·ε cannot reach it (`props_circle_axis_class` escalates
  first, cos·r_c in-band).
- **N3** The cone's `du_of_rims` arm is the FIRST rim's radius
  |v|·sinα (bounded below ≳ K·ε by the same axis-class guard); the
  `T::one()` fallback is unreachable (empty-rims refusal precedes).
- **N4** `tangent_normal_parallel`'s arm 1/κ_rel is the ratified D4 ¶1
  tangency lever ("normal-parallel within θ ⟺ within ε of the locus");
  unbounded only in the refusing direction, and the second-order gate
  fires first.
- **N5** `ps_frame_seam`, `pcurve_sphere_chart_frame`,
  `split_chart_azimuth_frame`, and `split_conic_phase_frame` are
  deterministic BRANCH/frame selections whose arms are all documented
  as verdict-neutral; their margins pollute per-predicate K telemetry
  with non-coincidence semantics (frame threshold at 0.5, mixed
  dimensions). Candidate for a separate "structure-selection" funnel
  tag in the typed-margin design.
- **N6** `split_join_order_u/v` deliberately classify against the
  bit-level exact band (total-order device), not ε — documented
  contract, excluded from the length rule by design.
- **N7** (review MINOR-3) The sphere's `Unit(sin v, 0) × R` grouping
  margin meters the AXIAL separation `R·|Δsin v|`, which degenerates
  toward the poles (∝ cos v̄ → 0): two distinct near-polar latitude
  rims can group as coincident although their true point separation is
  ~`R·Δv`. Dimensionally honest (the margin IS a length, and it is the
  quantity the area formula consumes), but the LEVER understates the
   3-D deviation near the poles — the same lever-magnitude family as
  N1, opposite direction (merges rather than escalates). Cone/cylinder
  bare levels and the torus two-component pair do not share this.
  Typed-margin conversation input.
