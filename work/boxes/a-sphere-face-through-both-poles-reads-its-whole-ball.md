---
id: a-sphere-face-through-both-poles-reads-its-whole-ball
kind: issue
title: A sphere face reads no chart window unless its side is encoded and every boundary edge is a latitude rim or a meridian arc, so a ball a boolean cuts, or a wedge running pole to pole, covers its whole ball and the session box refuses as a limit a body in the box whose ball pokes out
status: open
opened: 2026-11-21
priority: P2
cost: M
---

Found by the r2 review of PR 7225 (MAJOR 1), measured by that PR's lane. The class was widened on both reviewers' second delta reads (r1 MINOR 1, r2 E1).

**What.** `topo::cover::face_cover`'s sphere arm meets the ball with the face's chart window (`sphere_window`) only when two things hold:
- the face's outer loop encodes its side (`geom_brep::props::boundary_material_sign` reads `Encoded`, agreeing with the face's sense);
- `solid_contain::sphere_chart_trim` answers. Its class is no ring, and every boundary edge either a latitude rim (a circle about the polar axis) or an arc of a meridian great circle with no pole strictly inside it.

The window is also not read for:
- an outer loop that is a lone vertex or carries null scaffolding;
- a trim that escalates;
- an axis not decided unit;
- a radius not positive.

Wherever the window is not read, the cover is the whole ball. Since PR 7225 the session box then refuses, as the limit `SessionBoxUnshown`, a body in the box whose ball pokes out of it.

**The common members.**
- **A ball a boolean cuts.** A wall's section is a small circle, or a circle tilted to the poles; neither is a latitude rim nor a meridian great circle.
- **A wedge running pole to pole.** Its loop reads `Unencoded`: the chart image drops the loop's polar tips, and what is left encodes no side.

A ball cut square to its poles has a latitude rim and reads its window. So do a dome, a cap and a partial wedge.

**Measured** (each a `sweep session_box_reach` row):
- `a_ball_a_box_cuts_reads_its_whole_ball`: a ball of radius 1.5 cut by the brick `[−1, 1]² × [0, 3]`, moved 998.7 m. It reaches 999.7 m and is refused 0.2 m past.
- `a_ball_cut_by_a_plane_tilted_to_its_poles_reads_its_whole_ball`: a ball of radius 50 cut by a plane turned 0.4 rad, lifted 960 m. It reaches 988.3 m and is refused 10 m past. Cut square to its poles, it is admitted.
- `a_ball_wedge_in_the_box_whose_ball_pokes_out_refuses_as_a_limit`: the wedge of radius 30, its axis at `x = 990`, refused 20 m past.
- In r2's 756-placement differential, 15 of the 85 in-box bodies refused as a limit are pole-to-pole wedges. No sampled profile is a boolean's cut.

The overshoot is up to the ball's diameter.

**The shape of a fix.** A window for a face outside the trim's class: the azimuth and latitude ranges of each boundary circle's chart image, as the torus walk reads a harmonic image's box. For the wedge, a side for a loop through both poles read another way: its two meridians bound an azimuth window whatever the tips. Not a few lines. The side reading is the material sign's, and a tilted circle's chart image is not an iso-line.

**Why P2.** The session box (PR 7225) refuses a valid body that main admits, wherever such a face's whole ball reaches past the box. That needs a face whose ball stands within its own diameter of a box face. Every ball cut by a box near the wall is such a face.
