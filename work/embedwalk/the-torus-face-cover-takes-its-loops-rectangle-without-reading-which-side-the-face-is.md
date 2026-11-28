---
id: the-torus-face-cover-takes-its-loops-rectangle-without-reading-which-side-the-face-is
kind: issue
title: The torus face cover takes its loop's chart rectangle without reading which side of the loop the face lies on
status: closed
opened: 2026-11-25
priority: P2
cost: C
branch: embedwalk/two-bands
pr: 7786
closed: 2026-11-25
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

## Closed

Closed by PR 7786 (`embedwalk/two-bands`). It is the same question as the
embedding walk's: one closing loop bounds the rectangle's side and its
complement. Both readers now take one test, `cover::bounded_side_loops`
(the loop's signed chart area against the face's sense, band-free), through
`cover::bounded_torus_window`, which the face cover's torus arm reads.
`clearance::seated_windows::a_torus_windows_side_reads_alike_reverted_and_across_the_seam`
pins the four sense-and-direction quadrants and a lift across the seam;
the complement keeps the whole tube.
