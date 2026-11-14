---
id: nightly-reds-the-1e-6-rows
kind: unit
title: Nightly reds: the rows red at ε 1e-6
status: review
opened: 2026-11-14
priority: P1
cost: M
branch: green/eps-1e-6
pr: 6934
---

From nightly 43371638371 (main `e36675c1`, 2026-11-13), each leg's junit read whole. GREEN's rule holds (`work/green/program.md`): a red is greened here, not fixed. A move a merge explains is re-blessed, naming the merge; anything else is pinned at what main produces, or `#[ignore]`d naming the row it waits on, and the fix row goes on the program that owns the code.

**Difficulty letter:** M. **Review tier:** single review, STYLE — A pin can be read against what main produces; the rows that build a wrong volume are the ones to read hardest. Both logged before any draw (2026-11-14).

## What is red

`sweep::all`, 1e-6 only:
- `a_curved_keel_in_the_plane::a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur` and `::a_round_keel_in_the_plane_reads_two_annuli_today`
- `a_plane_moved_to_a_cone_apex::a_half_space_flush_with_a_frustum_wedge_cap_builds_the_union`
- `a_vertex_piercing_a_lens_wall::a_vertex_piercing_a_lens_wall_mints_its_ring_and_reaches_the_chords_certificate` (new on 11-13)
- `band_strip_mouth_cups::a_cups_mouth_a_hair_across_the_trimline_crosses_and_one_inside_goes`
- `germ_plane_torus_frame::a_nearest_tie_in_band_on_a_bar_across_the_hole_escalates_by_name`
- `m5_s11_concave_sense::loft_concave_arc_walls_face_out_and_a_flip_is_invisible_below`
- `near_tangent_pierce_poses::near_tangent_pierce_poses_build_their_closed_form_or_refuse_typed`
- `parallel_cylinder_join::one_face_walls_read_their_axes_as_an_angle` and `::seams_lying_on_the_walls_read_the_axes_across_the_region` (both `OK BAD`: a built body with a wrong volume)
- `tilted_sphere_pair::a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed`
- `verbs_germarms2::a_rod_through_a_drum_wall_builds_at_its_sliced_volume` and `::the_fenced_poses_keep_their_own_outcomes` (both `OK BAD`)

1e-6 and 1e-12:
- `editor-core::all revalue_corpus::revalue_boss_union`
- `sweep::all verbs_cylcyl_r1_review_probes::every_reachable_crossing_pose_refuses_typed_or_answers_correctly` (`OK BAD` at 1e-6; `GermSectionUnbuilt` at 1e-12, 782 s)

This slate's rows: `main-red-a-round-keel-in-the-plane-reads-off-annuli-at-1e-6`, `main-red-revalue-boss-union-reads-two-period-near-mark-flips-at-1e-6`. Elsewhere (unchecked): `work/onehome/a-closed-keel-in-the-section-plane-divides-the-s…`, `work/band/a-cups-fixture-boolean-is-unfinished-at-eps-1e-6`, `work/onecopy/main-red-a-nearest-tie-in-band-on-a-bar-across-t…`, `work/quad/lofted-concave-prism-pad-pin-moved-at-1e-6`, `work/restread/main-red-near-tangent-pierce-poses-w60-a4-census…`, `work/quadtail/chorded-walls-reach-the-work-wall-short-of-the-p…`, `work/germ/tilted-section-through-a-chart-pole-is-not-split…`, `work/spiricarm/a-carried-cylinder-pair-marches-for-minutes-befo…`.

## 2026-11-14: greened on `green/eps-1e-6` (PR 6934)

All fifteen reproduced red on main `b363f5fa1` at ε 1e-6, and the two this unit lists at 1e-12 reproduced there. One is re-blessed, fourteen are pinned; the PR body has the table. Corrections to the text above, checked against the tree:

- **`OK BAD` is not always a wrong volume.** `verbs_germarms2`'s two rows and `verbs_cylcyl_r1_review_probes` at 1e-6 hold their closed form (`v` and `want` agree to nine places); the line is `BAD` because the enclosure's pad is over `differential.rs`' fixed `VOLUME_PAD_BUDGET` of 1e-7. Fix row: `work/spiricarm/main-red-three-cylinder-pair-volume-pads-pass-the-fixed-budget-at-1e-6.md` (PR 6478). `parallel_cylinder_join`'s two rows do miss their closed form, by 1.2e-7 to 2.8e-7.
- **`parallel_cylinder_join`'s two rows, the keel's split, and the cone cap's union went red at one merge**, PR 6505 (`onedoor/one-surface-record`), with the cups fixture: `work/onedoor/main-red-four-sweep-rows-hold-volumes-the-one-surface-record-moves-at-1e-6.md` and `work/band/a-cups-fixture-boolean-is-unfinished-at-eps-1e-6.md`.
- **`a_vertex_piercing_a_lens_wall` is a move, not a red**: PR 6891 gave a spline wall's ring its winding, so the 1e-6 run passes the join and stops at containment's missing spline arm. Its 1e-6 stop is re-pinned there.
- **`tilted_sphere_pair`'s row is `work/gauge/the-in-arc-pairing-turns-the-pole-arcs-lane-gap-into-an-indeterminate-at-eps-1e-6.md`** (PR 6832, re-bisected), not `work/germ/tilted-section-through-a-chart-pole-is-not-split-at-the-pole.md`.
- **`verbs_cylcyl_r1_review_probes` at 1e-12 is a different stop** from its 1e-6 one, and had no row: `work/ssiarith/main-red-a-tall-thin-cylinder-pairs-section-reads-limb-2-in-band-at-1e-12.md`. `work/spiricarm/a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses.md` and `work/quadtail/chorded-walls-reach-the-work-wall-short-of-the-pinned-resolution-at-1e-12.md` are about cost and the 1e-12 resolution, and cover neither.
- `work/restread/main-red-near-tangent-pierce-poses-pinned-finding-moved-at-1e-6.md` is closed and is another pose; the open one is `…-w60-a4-census-undecidable-at-1e-6.md`.
