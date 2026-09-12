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
