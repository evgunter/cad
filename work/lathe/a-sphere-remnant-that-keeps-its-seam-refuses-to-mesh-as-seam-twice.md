---
id: a-sphere-remnant-that-keeps-its-seam-refuses-to-mesh-as-seam-twice
kind: issue
title: A boolean's sphere remnant that keeps its one seam refuses to mesh, SphereTrimShape SeamTwice
status: closed
opened: 2026-11-22
priority: P1
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
cost: M
closed: 2026-11-26
pr: 7473
---

Found by the LATHE one-band lane (2026-11-22, PR 7414), measured on its
branch `lathe/full-revolve-one-band`.

## What

A boolean's sphere remnant from a one-band ball keeps its seam, the face's
wrap edge, walked both ways in its loop. The trimmed sphere lane
(`crates/mesh/src/trimmed.rs`) refuses it
`SphereTrimShape { cause: SeamTwice }` ("the loop visits one mesh vertex at
two chart locations off a singular row: a seam traversed twice",
`crates/mesh/src/types.rs` `SphereTrimCause::SeamTwice`). With two π-bands
no remnant carried both halves of a seam. The whole one-face ball meshes
(`mesh` `the_one_face_ball`); a TRIMMED one whose loop keeps the seam does not.

Failing on the branch: `sweep` `snowman::a_bar_through_a_ball_builds_under_every_boolean`
(ball ∪ bar), `tilted_sphere_pair::a_pierce_off_the_seam_plane_builds_under_every_boolean`,
`tilted_sphere_pair::a_plane_tilted_against_the_balls_chart_builds_under_every_boolean`,
`tilted_sphere_pair::a_section_passing_near_a_pole_or_through_it_builds`;
`mesh` `trimmed::tests::a_lattice_laid_without_clearance_cuts_a_triangle_the_lane_refuses`,
`trimmed::tests::an_over_delta_sphere_arm_triangle_refuses_typed`,
`route_rows::a_section_tilted_within_the_band_meshes_to_mains_bytes`.
Each reports `SphereTrimShape { .., cause: SeamTwice }`.

**More once a sphere cut-in's meridian is glued away** (LATHE
interior-loop lane, PR 7440, 2026-11-23). These unions meshed on the
base only because the transient cut survived the output merge (each
base result carries 2 to 8 edges dividing two faces of one sphere
record); with the cut glued away the remnant keeps its one seam and
meets this row: `spheres_crossing_off_every_edge::{two_trimmed_spheres_build,
a_trimmed_face_cut_beside_its_pole_builds, a_ball_crossed_by_two_partners_builds}`
and `snowman::{a_slab_cutting_a_cap_off_a_banded_ball_builds,
a_circle_across_an_earlier_cut_takes_none_of_its_own}`. The trimmed
operands of `two_trimmed_spheres_build` refuse `SeamTwice` on their own.

## Ground

`crates/mesh/src/trimmed.rs`: tesslane and tesslie. Claimed by LATHE.

## Evidence (LATHE one-site lane, 2026-11-23)

Once a one-site loop on both operands' wrap edges joins
(`lathe/one-site-wrap-edges`), `sweep`
`tilted_sphere_pair::a_flipped_remnant_with_a_pole_off_its_loop_is_refused_by_name`
builds `A ∪ B` (validated, its volume to the lens closed form) and then
refuses here, `SphereTrimShape { cause: SeamTwice }` at δ 0.01. The join
is pinned on its own by `the_pair_swallowing_a_pole_joins_under_every_op`.
