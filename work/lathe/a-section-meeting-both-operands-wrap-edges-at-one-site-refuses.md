---
id: a-section-meeting-both-operands-wrap-edges-at-one-site-refuses
kind: issue
title: A section loop that meets both operands' wrap edges at one site refuses SingleSiteSectionLoop, so the snowman union and coaxial sphere booleans refuse once a full revolve is one band
status: closed
opened: 2026-11-22
priority: P1
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
cost: H
closed: 2026-11-26
pr: 7476
---

Found by the LATHE one-band lane (2026-11-22, PR 7414), measured on its
branch `lathe/full-revolve-one-band`.

## What

A full revolve's curved wall is now one band, its meridian the face's
wrap edge (DESIGN `[topology]`'s seam clause). Every revolved sphere about
one axis carries its seam in the same half-plane, so a section circle of
two coaxial spheres (the snowman's waist, a lens against a ball) now meets
BOTH operands' wrap edges at ONE point. With two π-bands it met the
meridians at azimuths 0 and π, two sites, and joined as an ordinary
two-chord loop. `crates/topo/src/boolean/join.rs` `one_site_segments`
leaves "a site on both operands' wrap edges" to the refusal, so each such
boolean refuses `Join(BooleanJoin(SingleSiteSectionLoop { count: 1 }))`.
A tilted pair whose section passes through a pole and the seam meets the
same refusal.

Failing on the branch (`cargo nextest run -p sweep --profile ci`), each
with that payload: `snowman::` the_snowman_builds_under_every_boolean,
the_snowman_builds_at_the_interval_scalar, the_snowman_waist_fillets,
a_near_tangent_snowman_builds, the_near_tangent_family_builds_to_1e_6_and_escalates_at_1e_8,
the_near_tangent_family_builds_to_1e_6_at_the_finer_bands,
a_hemisphere_against_a_ball_crossing_its_cap_and_dome_builds,
a_lens_against_a_ball_crossing_both_its_caps_builds,
a_lens_beside_a_slab_its_trimmed_sphere_crosses_builds,
a_millimetre_lens_inside_a_ball_builds_at_every_tolerance,
a_closed_join_through_the_door_leaves_a_conventional_vertex,
a_corner_at_a_caps_conventional_vertex_reads_the_circles_interior,
a_plane_through_a_caps_conventional_vertex_cuts_as_elsewhere,
a_slab_tilted_across_the_lens_seams_builds,
the_fallback_assembly_carries_the_kept_operands_certificates;
`pierce_tangent_off_face::` (7 rows, every lens row);
`extent_scan_off_face_tangency::` a_ball_touching_the_lens_sphere_where_it_is_trimmed_builds,
a_ball_touching_the_snowman_sphere_where_it_is_trimmed_builds,
a_brick_whose_plane_touches_the_snowman_sphere_off_its_face_builds;
`tilted_sphere_pair::` the_section_nearest_a_pole_splits_at_the_poles_joint,
a_flipped_remnant_with_a_pole_off_its_loop_is_refused_by_name;
`verbs_sphsph_opening::seam_crossing_pairs_build`. 28 rows.

## Ground

`crates/topo/src/boolean/join.rs` (`one_site_segments`,
`closes_at_its_site`) and `crates/topo/src/chord_join.rs`
(`SingleSiteSectionLoop`): `work.py territory` names cleavetail,
heldrecords, hone, honejoin, honeone, onecopy, onedoor, onehome,
partframes, parting, seal, sect*, splice, splitdoor and tang. Claimed by
LATHE because it gates the one-band item and needs its branch as base.

## Open

Whether the one site on two wrap edges closes as two self-loop chords
(one per operand, each the loop walked from the shared site) or the
join must split one operand's loop off its seam first.
`one_site_segments`' doc lists what closes at one site today; a site on
both operands' wrap edges is the shape it leaves out.
