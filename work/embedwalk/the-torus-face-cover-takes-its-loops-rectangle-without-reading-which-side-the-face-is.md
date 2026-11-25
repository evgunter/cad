---
id: the-torus-face-cover-takes-its-loops-rectangle-without-reading-which-side-the-face-is
kind: issue
title: The torus face cover takes its loop's chart rectangle without reading which side of the loop the face lies on
status: open
opened: 2026-11-25
priority: P2
cost: C
---


## Finding

Read off the code; not measured. `cover::face_cover_in`'s torus arm
meets the tube with `cover::torus_chart_window`'s rectangle on any ring
torus, and `TorusChartWindow`'s docs argue that rectangle a superset of
the face because "the face's region is the planar region its loops'
chart images bound". One loop that closes in the chart bounds two
regions: the rectangle's side and its complement on the tube. The
complement is a face the kernel can hold:
`boxes::tests::torus_wall_clockwise` builds it, the patch's own loop
under `sense: true`, "the face is then the rest of the torus". Its
cover would be the patch's, which misses nearly all of the face, and
every reader of the cover (boxes, the BVH's pruning, the containment
doors' reaches) would then prune a real contact.

The sphere arm guards the same ambiguity
(`cover::sphere_window`: `boundary_material_sign` against the face's
sense). The embedding walk's window guards it band-free
(`clearance::bounded_side_steps`: the loop's certified chart shoelace,
its sign against the sense). The torus cover arm should take one of the
two. No kernel door mints a torus face on the complement's side today,
because every torus constructor seams its walls, so the hole is latent.
