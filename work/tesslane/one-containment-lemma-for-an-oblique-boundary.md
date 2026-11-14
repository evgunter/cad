---
id: one-containment-lemma-for-an-oblique-boundary
kind: unit
title: one containment lemma for a boundary run that crosses lattice lines, and UNPROVED_HEADROOM goes
status: open
opened: 2026-11-14
priority: P2
cost: H
refs: [the-trimmed-cylinder-lane-has-no-worst-triangle-theorem, the-trimmed-sphere-arm-has-no-worst-triangle-theorem, a-cone-face-with-a-split-side-has-no-worst-triangle-theorem]
---


Unit 5 of the chart lane (`work/tesslane/plan.md`, The chart lane). Independent of units 2–4.

## What it is

`[mesh.sizing]`'s theorem factors the way the lane does. The **cell bound** is per chart: a triangle inside a chart box one lattice step wide deviates at most δ; it is proved and pinned for every chart today. **Containment** is per triangulation: how far past one cell a triangle can reach. For a boundary run on a chart iso line it is proved (the strip argument at `sizing::sagitta_step`, the aspect lemma at `sphere_grid_steps`). For a run that crosses lattice lines it is not, on any chart, and that one missing lemma is what three rows see from three charts:

- `the-trimmed-cylinder-lane-has-no-worst-triangle-theorem`
- `the-trimmed-sphere-arm-has-no-worst-triangle-theorem`
- `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`

The lemma lives in the triangulation's plane, in the lattice metric, so it is the same for every chart: a bound on the circumradius of a constrained Delaunay triangle beside a boundary chord, in terms of the lattice cell and the chord's chart step. With it, a face with an oblique run sizes from the lemma solved for the step, and `UNPROVED_HEADROOM` is deleted.

## What is known and what is not

- `nurbs_cert::LATTICE_SHARE`'s doc claims its sliver lemma covers adjacent chord points of a slanted trim edge; its pin (`a_strip_lattice_off_row_sliver_stays_inside_the_box_bound`) samples one constraint with lattice on both sides. The two debt rows say the same triangle is unbounded. One of them is wrong, and this unit starts by settling which.
- Both designers sketched a bound and both mark it unsure. Neither sketch is a proof; hidden-circumcentre triangles and the sphere's one-latitude metric are not worked through.
- It starts with the measurement the debt rows ask for: the worst certificate against δ over oblique faces at the current headroom.
