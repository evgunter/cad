# SCALAR log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/scalar/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Seven rows moved in by
`git mv`, all seven from `work/code-quality/` — this is the one track
the cut built entirely out of that directory.

It is small on purpose. The scalar rows that already had a fix written
(`placement-lifts-its-affine-by-hand-beside-affine3-map`, the two
profile lift doors) went to the seat's successor WIRE and to DOOR, where
their class puts them; what is left here is what actually needs the
substrate decided first.

No branch exists yet. The first act is an `[ev]` PR carrying `D6`,
`D283` and the unit-vector question as one conversation.

<<<<<<< HEAD
## The first `[ev]` sitting: D6, D283, the unit vector (2026-09-12)

Three surveys of the tree, read by the orchestrator before the
recommendations were written; each row's `## Put to Ev` section carries
the decision and the rejected alternative, this entry the evidence.

**D6.** No `SenseSign` exists. `OutwardNormal<T>` (`geom-brep/src/enters.rs`)
wraps the product and takes the bit; `ReferenceNormal` is its sibling.
`Face::sense_sign<T>()` (`topo/src/entity.rs`) is the only mint of the
±1. Bare-`T` doors: `geom_brep::classify_material_pairing`
(`dihedral.rs`; callers `census.rs` `ee_cross_backed`, `validate.rs`
check 4, `rim_wedge.rs`), `material_kappa_rel` (`dihedral.rs`),
`rim_wedge::classify_shared_rim` (caller `boolean/mod.rs`
`verify_tangent_declaration`), `curved_face` and its private `sphere`
(`props/curved.rs`; caller `topo/src/props.rs` `face_flux`). Hand
multiplies `normal * face.sense_sign()`: `emit_topo.rs` `face_plane`,
`blend/build.rs` `outward_of`, `blend/battery.rs` `outward`,
`boolean/join.rs` `ring_run_ccw`, `boolean/rest.rs` `face_carrier`,
`boolean/solid_contain.rs` `face_plane` and `face_geo`, `validate.rs`
check 6, `merge_faces.rs` `merged_outline_ring` and
`planes_declared_equal`, `mesh/walk.rs` `loop_polygon` (`f64`, an
area). No comparison on the ±1 anywhere; every comparison is on the
bit. Test-side hand-multiplies: `sweep/tests/common/orient.rs` and
three blend suites; `dihedral.rs`'s own tests pass `1.0`/`-1.0`
literals into the bare doors. `sphere`'s `s_f` is also minted from
`t_sign(linear_rim_side(..))`, a decided `Sign` that can be zero.
Defect filed: `work/bool/missing-face-in-verify-tangent-declaration-reads-as-sense-true.md`.

**D283.** `Exhaustiveness` and its writer `sweep` in
`geom-brep/src/ssi/exhaust.rs`; the lanes `account_r3` and
`account_chart_plane`, called from `ssi.rs` with `domain.floor(band)`
and `domain.floor(band) / speed`; `speed` from `NurbsBoxes::deriv_box`
via `nan_propagating_max`, a local. `SsiOutcome` carries no domain,
chart, lane or speed. Readers: `Display` (`ssi.rs`), a wildcard match in
`pcurve_cache.rs`, and `geom-brep/tests/m5_pr7_ssi.rs` with its helper
`assert_floor_is_the_meters_floor_over_the_chart_speed` and the pinned
`WALL_CHART_SPEED`. The guard for #762 is `!speed.is_finite()` and
`speed <= 0.0`; `work/curved/ssi-chart-speed-usability-boundary.md`
holds the finite-but-huge case. Precedents: `Margin`'s constructor doors
(`predicate.rs`, `docs/predicate-dimension-audit.md`), `Tol`
(`tolerance.rs`), `quantity::Length` (boundary-only, restates D6),
`PatchRegularity` (`offset_meters.rs`: `floor`, `speed_u`, `speed_v`,
`thinness()`), `SsiLimb` (`ssi/certify.rs`). Blast radius of the tag:
the two `Exhaustiveness` literals in `sweep`, the two chart-lane
signatures and call sites, a manual `Default`, `Display`; every test
destructure compiles as is.

**Unit vector.** `topo::query::UnitVec3<T>` (`new(v, band)` over
`decide_unit_direction`, funnel `DATUM_UNIT_NORM`, `get()`), consumers
`DatumValue::{Plane, Axis, Frame, AxisInPlane}`, `eval/wire.rs`
`frame_axes`, `AxisFrame`, `tube_args`, `frame_plane_lane`.
`profile::path::Dir<T>` (`from_unit` does not re-decide; mints
`unit_from_components`, `arc_fillet::carrier_tangent`, `reversed`).
The ladder: `Vec3::normalize` is a bare divide (poison on zero,
overflow/underflow rows open on PROPS' and FIX's slates);
`frame::definitely_positive` then `normalize` in `point_at`,
`path_start_frame`, `mirror_across_plane`; `sweep::revolve::axis`,
`profile::path::unit_from_components` decide first; `enters_material`
normalizes without the length question (FIX's row). Unchecked prose
preconditions: `Affine3::from_frame` (callers `wire.rs` ×2,
`anchor.rs`, `emit_topo.rs` ×4, `viewer/sketch.rs`, `pncad-py/doc.rs`
×2), `Vec3::orthonormal_basis` (`newell.rs`, `step-import/recognize.rs`),
`frame_from_unit_aim` (private; fed), the geom carrier fields under
`geom/src/lib.rs`'s at-rest rule, `SplitPlane.normal`, `slab_extent`,
`axial_radial`, `tangent::perp`, `blend/arms.rs` `perp_unit`,
`mesh/cert.rs` `dist_line_triangle`, `mate/coset.rs` `Subgroup`
fields, `measure.rs` `Carrier`. The opposite posture exists once:
`revolve/tube.rs` refuses a non-unit axis at tolerance. At `Interval`,
`UnitVec3::new` decides `‖u‖ − 1` to `Zero` for tight inputs and stays
sound on an overflowed enclosure (`query.rs` tests). `cross_len` is a
carried witness for `perp`'s length only; nothing carries `aim`'s.

The `[ev]` PR is `scalar/ev-newtypes`; `needs_ev` is set on all three.
=======
## Orchestrator seated; the eighth row on the table (2026-09-12)

A SCALAR orchestrator is seated (remote box; branch prefix `scalar/`,
orchestrator branch `scalar/orchestrator`). DOOR re-homed
`curve3-eval-and-deriv-at-one-t-run-two-basis-passes` here on
2026-09-12 after the plan's slate table was written; the table now
carries it at class **M** beside `S393`, and the program text counts
eight rows. Sequencing, per the plan and Ev in-chat (2026-09-12):
the `[ev]` PR carrying `D6`, `D283` and the unit-vector question is
drafted first; `D290` dispatches beside it rather than behind it;
`H5`'s own questions (Q1, RingInterval) go to a SECOND `[ev]` sitting
once the door rows are in and its decomposition is cut.

## D290 and S393 dispatched; the seams announced (2026-09-12)

Both door rows are in flight on their own branches with a spec each
(`docs/D290-SPEC.md`, `docs/S393-SPEC.md`), block SCALAR-B1 slots 0 and
1; the block record is branch-side per the A/B log's redaction shape.

**Seams, announced here and on each PR when it opens.** `D290` reaches
PROPS' `crates/geom-core/src/spline/knots.rs`, `crates/geom/src/curves/nurbs.rs`
and `crates/geom-brep/src/offset_fit.rs`, and TRIM's
`crates/geom-brep/src/edge_nurbs.rs` — one `KnotVector` rescale door
with exact pinned ends, replacing the private `offset_fit::rescaled_knots`
and the inline map in `edge_nurbs::on_carrier_domain` (which does not
pin its ends, so its image domain can sit an ulp off the carrier
interval — the one behaviour change, argued in the spec). `S393`
reaches S-TCOST's and S-TINT's `crates/sweep/tests/*`, BLEND's
`crates/sweep/src/skin.rs` (docs only) and `demos/tour/src/skinned.rs`.

**S393's premise corrected before dispatch.** The row says no public
door hands out the start frame. `geom_core::linalg::frame::path_start_frame`
does, is public, and is already bound into Python; the unit is the two
copies going onto it, with the one semantic difference (a hard 0.9
helper cone against the door's decided reference ladder) measured
fixture by fixture. Class corrects M → E in the plan table via the PR.

**The third door row waits on D290.** The v-reversal door on
`NurbsSurface` needs the same exact-ends argument for a REFLECTED knot
vector (`k ↦ lo + hi − k`) that D290 makes for a rescaled one; the test
that rebuilds the net carries the knots verbatim, which is the same
point set only when the v knots are symmetric — a door has to say what
it does when they are not.
>>>>>>> origin/main

## The rate census, for the `[ev]` sitting (2026-09-12)

Ev asked on PR 2457 whether a kernel-wide `Rate` would improve
uniformity and semantic correctness. Every parameter ↔ meters crossing
in `crates/*/src`, by shape (sup = certified upper bound on a speed,
inf = certified lower, pt = pointwise sampled, exact = closed form):

- **Through `Margin::metered` (span · rate)**: `certify.rs`
  `nurbs_span_meter`/`interval_span_forward` (inf,
  `speed_lower_bound`); `pcurve_cache.rs` `param_rate` and its four
  consumers (line 1 exact / nurbs inf / circle radius / ellipse minor);
  `pcurve_cache.rs` `trim_containment` (SUP, `chart_stretch_sup`);
  the `pcurve_iso_*` slack meters (SUP, `nurbs_stretch_bounds`);
  `topo/pcurves.rs` `pcurve_loop_continuity` v-channel (`v_meter`:
  exact polar arm or SUP); `topo/split.rs`
  `split_edge_param_interior` (inf per kind); `splitting/classify.rs`
  conic roots; `chord_join.rs` `split_tangent_chord_forward` (exact).
- **Through `Margin::levered` (angle · arm)**: `pcurve_loop_continuity`
  u-channel (`azimuth_arm`: exact per analytic kind, SUP on splines);
  `pcurve_azimuth_period`, `chart_windings` (`azimuth_lever`);
  `chart_bound.rs` `assembled`.
- **Through `levered_inv` (÷ rate)**: `ssi/certify.rs`
  `ssi_foot_orthogonality` (pt jet norms); two curvature uses
  (`kappa_rel`, 1/m) and one length use — not rates.
- **Through `over_lever`**: `offset_meters.rs` `offset_normal_floor`
  and `PatchRegularity::thinness` (inf area rate ÷ max SUP speed; the
  only struct storing named speeds).
- **By hand beside a predicate**: `ssi.rs` `plane_nurbs_ssi` (SUP,
  u/v max-folded, three divisions: seed floor, pad, floor);
  `ssi/certify.rs` limb-3 tube (SUP per axis) and transversality
  (`stretch`); `chart_region.rs` `certified_arms` (inf per kind,
  gated through `Margin::of`); `pcurve_cache.rs` slack products
  outside the door; the sphere `polar_rate` composition debt.
- **By hand outside the decide seam, by design**: the SSI march
  (`coordinate_scale`, `tangent_speed`, pt); `offset_fit.rs`
  `directional_mark` (structure selection); `coherence.rs`
  `gap_is_noise` and `mesh/walk.rs` (lever 0 = every gap noise, on
  purpose); `mesh/sizing.rs` `sagitta_step`/`curvature_step`/
  `ellipse_step`/`torus_grid_steps` and `mesh/nurbs_cert.rs`
  `split_steps` (second-order, m/param²); `mesh/chords.rs`
  `nurbs_tighten` (chart units per `t`); the projection Newton
  acceptances (pt); `enters_material_order2` (pt, squared).
- **Hazards a single type would blur**: bound direction (the
  `metered` doc promises inf, three sites pass SUP — PROPS' row
  `metered-margin-doc-promises-an-inf-bound-three-sites-pass-a-sup`);
  per-axis vs max-folded (`ssi.rs` vs limb 3 — TRIM's row
  `ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis`);
  the u/v door asymmetry in one predicate (TRIM's row
  `loop-continuity-meters-u-through-levered-and-v-through-metered`);
  certified vs sampled; second-order and param→param rates; 0 and ∞
  meaning different things per site (the #762 guard, `thinness`
  unguarded by contract, `gap_is_noise`'s zero lever); the NaN-fold
  divergence `nan_propagating_max` vs plain `f64::max`
  (`work/curved/ssi-lever-arm-min-fold-hides-poison.md`).

Counts: door-mediated 20, hand-spelled beside a predicate 8, outside
the seam 9, non-rate `levered_inv` uses 3. The recommendation put to
Ev: `SupSpeed`/`InfSpeed` in `geom-core` beside `Margin`, scoped to the
linear m-per-parameter crossing; everything in the last bullet stays
out.
