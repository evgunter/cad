---
id: array-doors-are-not-yet-adopted-by-the-test-suites
kind: issue
title: the array doors are adopted by every non-test site, and about two hundred test-suite lowerings still spell the components by hand
status: open
opened: 2026-10-01
priority: P4
cost: E
---


The residue of `geom-core-linalg-has-no-array-doors`, disclosed by the
lane that landed the doors (`linalg/doors`) so it has a home rather
than a PR-body line.

## What landed

`Vec2`/`Vec3`/`Point2`/`Point3::from_array`/`to_array`,
`Mat3::from_cols_array`/`to_cols_array`/`cols`, and `Affine3::cols`/
`components`, each binding its type's fields by pattern. Every
NON-TEST hit of the sweep below was retired into them in that PR,
plus the test helpers the parent row named.

## What is left

Test-suite and test-module lowerings, swept at that PR's merge base
with four shapes over `crates`, `demos`, `tools` and `benches`:
`(Vec3|Point3)::new(a[0], a[1], a[2])`, `[v.x, v.y, v.z]`, and their
2-D twins, with `#[cfg(test)]` module extents brace-matched. Lowering
a test fixture by hand is not a defect — nothing drifts, every one is
a literal transcription — so this is adoption, not repair, and it
rides with whichever lane next edits a file below.

The column and twelve-scalar readouts (`[a.linear.c0, a.linear.c1,
a.linear.c2, a.translation]`, `c0.x, c0.y, …`) left in tests are a
second, smaller shape: `editor-core`'s `pinned_lift_validates_once.rs`
and `scalar_frame_r1_probes.rs`, `profile`'s `validated_map.rs`,
`sweep`'s `review_blend_k_rk_probes.rs`, `s393_start_frame_door.rs`
and `turning_orientation.rs`, `geom-core`'s `cert3r1_probes.rs`,
`r2_cert3_probes.rs` and `review_m0_pr6.rs`. Those inside
`geom-core/src/linalg/` (`mat.rs`, `affine.rs`, `frame.rs`,
`ortho_frame.rs` tests) and `editor-core`'s
`placement::tests::affine_at_f64_carries_the_stored_bits` are KEPT on
purpose: they are independent statements of where each component
belongs, which a row checking a door cannot take from the door.

**What the patterns cannot match**: a lift through an intermediate
binding other than an array destructure (the destructure shape was
swept separately and found `viewer`'s `combine.rs`, retired), a slice
or variable-indexed lift in a loop, a sub-range of a longer state
(`geom-brep`'s ℝ⁴ `Point2::new(s[0], s[1])`, not this door's shape),
tuple lowerings `(p.x, p.y, p.z)`, and lowerings with a cast per
component (`[p.x as f32, …]`, VGEOM's narrowing rows).

## The hit list (test side)

210 hits in 88 files
- `crates/bvh`: `src/test_support.rs` (2)
- `crates/editor-core`: `src/names/emit_topo.rs` (2), `src/placement.rs` (2), `src/resolve/pick.rs` (2), `tests/asm2a_instantiate.rs` (2), `tests/edit_placement_type.rs` (2), `tests/emit_seam_junction.rs` (1), `tests/emit_split_edge_lineage.rs` (1), `tests/emit_union_borders.rs` (2), `tests/fixture/digest.rs` (1), `tests/gui1_pick.rs` (2), `tests/m10_p_fence.rs` (3), `tests/m10_p_lift.rs` (1), `tests/msolve8_levered_clash.rs` (2), `tests/msolve9_from_face.rs` (4), `tests/pick3_early_out.rs` (3), `tests/pinned_lift_validates_once.rs` (1), `tests/review_gui1_r1.rs` (1), `tests/scalar_frame_r1_probes.rs` (1)
- `crates/geom`: `src/test_support.rs` (2), `tests/curves/fit_certify.rs` (1), `tests/curves/hull_circle_rehearsal.rs` (3), `tests/curves/nurbs_differential.rs` (6), `tests/curves/review_m5_pr2_e2e.rs` (1), `tests/curves/review_m5_pr4_adversarial.rs` (1)
- `crates/geom-brep`: `src/pcurve_cache.rs` (1), `src/ssi/enclose.rs` (1), `src/ssi/march.rs` (1), `tests/cert10r2_probes.rs` (5), `tests/offb_r2_probes.rs` (1)
- `crates/geom-core`: `src/linalg/affine.rs` (3), `src/linalg/mat.rs` (2), `src/linalg/ortho_frame.rs` (8), `tests/cert3_evidence.rs` (2), `tests/cert3r1_probes.rs` (1), `tests/props1_review_rows.rs` (3), `tests/r2_cert3_probes.rs` (7)
- `crates/mesh`: `src/nurbs_cert.rs` (2), `tests/r2_bytes.rs` (1)
- `crates/pncad`: `tests/all.rs` (1)
- `crates/profile`: `tests/validated_map.rs` (1)
- `crates/step-export`: `tests/m5_pr13_curved.rs` (20), `tests/onb_wall_normal_census.rs` (2)
- `crates/step-import`: `tests/freecad.rs` (1)
- `crates/sweep`: `tests/blend3_r2_probes.rs` (2), `tests/blend4_r1_probes.rs` (1), `tests/common/cavity.rs` (1), `tests/common/poses.rs` (1), `tests/pis_arc_capped_poses.rs` (2), `tests/r2_probe_cert8.rs` (1), `tests/r2_rim_corpus_probes.rs` (2), `tests/reach_volume_backstop.rs` (1), `tests/review_blend3_r1_probes.rs` (1), `tests/review_m5_pr10.rs` (2), `tests/revolve_determinism.rs` (1), `tests/sf2a_r2_probes.rs` (1), `tests/shell5_r2_probes.rs` (1), `tests/wire_loft_end_profile_lift.rs` (1)
- `crates/topo`: `src/attach.rs` (2), `src/boolean/carrier_eq.rs` (5), `src/boolean/circle_sphere.rs` (1), `src/boolean/circle_torus.rs` (7), `src/boolean/contact_verify.rs` (4), `src/boolean/plane_eq.rs` (2), `src/boolean/recl.rs` (3), `src/boolean/rim_wedge.rs` (2), `src/boolean/sectors.rs` (5), `src/census.rs` (2), `src/euler_kill.rs` (1), `src/merge_faces.rs` (1), `src/replace_face.rs` (1), `src/test_support_fixtures.rs` (1), `tests/bool4r1_probes.rs` (1), `tests/contact5_gate_and_beam.rs` (1), `tests/contact7_touch_sweeps.rs` (2), `tests/contact9_side_codes.rs` (1), `tests/review_m3_pr55.rs` (1), `tests/review_m9_1_probes.rs` (5), `tests/review_m9_1_r2_probes.rs` (4)
- `crates/viewer`: `src/gpu.rs` (5), `tests/common/asm.rs` (3), `tests/creation_ops.rs` (5), `tests/datum_draw.rs` (7), `tests/display_budget.rs` (1), `tests/index_memo.rs` (2), `tests/profile_draw.rs` (1), `tests/review_gui4_r1.rs` (3)
- `tools/tess-meter`: `tests/mesh5_r2probe.rs` (1)
