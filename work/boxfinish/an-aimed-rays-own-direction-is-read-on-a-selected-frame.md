---
id: an-aimed-rays-own-direction-is-read-on-a-selected-frame
kind: issue
title: An aimed ray's own direction is read on a selected frame, so on a seam normal it is the hull of two frames
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found by the review of the ray-readings unit (PR 7480).

**What is there.** `side_reader::aimed_ray` turns its ray off the
face's normal along a tangent built on `normal.orthonormal_basis()`
(`aim_turn`). That basis is a selection between two frames
(`Vec3::select` on `|n_z| − max(|n_x|, |n_y|)/2`), and an enclosure of a
normal on that seam selects the hull of both. So over a box, or at
plain `Interval`, a face whose normal sits on the seam aims a ray whose
own direction is as wide as the two frames are apart.

**What the unit did and did not do.** The unit's first form read the
loops' turn on the same basis and lost four `Interval` reads on walls
square to `(2/3, 2/3, 1/3)` (`sweep`
`review_m2_pr4_interval::fixed_interval_rotated_placement_builds_tier_valid`);
its final form reads the turn with no frame
(`solid_contain::planar_cross_dot`), so a ray CROSSING a seam face reads
it. The ray AIMED at a seam face still takes its direction from the
selection, as it did before the unit. That body reads by its other
faces' rays
(`an_aimed_ray_over_boxed_door_bodies::a_prism_with_walls_on_the_frame_seam_reads_by_its_rays_at_interval`).

**Not measured:** how wide that ray's direction is, and whether any
body's every face sits on the seam.

**What closes it.** The aimed ray's tangent taken from something the
face holds and nothing selects (its plane's stored `u_ref`), or the
seam face's ray shown to be set aside and never read wrong, with a row
at `Interval` aiming at a seam face.
