---
id: a-sphere-remnant-that-keeps-its-seam-refuses-to-mesh-as-seam-twice
kind: issue
title: A boolean's sphere remnant that keeps its one seam refuses to mesh, SphereTrimShape SeamTwice
status: open
opened: 2026-11-22
priority: P1
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
cost: M
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

## Ground

`crates/mesh/src/trimmed.rs`: tesslane and tesslie. Claimed by LATHE.
