---
id: sphere-seam-in-a-plane-face-loses-the-fallback-recut-to-the-tilted-section-refusal
kind: issue
title: A sphere whose seam meridian lies in a plane face now takes the crossings path and refuses the tilted plane×sphere section, where the no-crossings re-cut answered
status: open
opened: 2026-09-28
priority: P2
cost: M
refs: [coplanar-conic-edge-skips-endpoint-treatment-in-the-sweep]
---


## What

A revolved ball's seam meridian and both its poles lie in a plane face
of the other operand when the ball is poled along an axis lying in that
plane: `cube(1) ∖ ball` with the ball `r = 0.3` poled along `y` and
centred at `(0.5, 0.5, 1)`, on the cube's top face.

Before `coplanar-conic-edge-skips-endpoint-treatment-in-the-sweep`, the
sweep's conic × plane lane skipped a conic lying in the face's plane, so
this pair had no events: the reduction found no crossings,
`boolean/ops.rs`'s sphere extent scan saw the ball escape through the top
face, re-charted it about that normal, and the re-entered op answered
`1 − (2/3)π·0.3³` exactly (valid body, `point_in_solid` right in the
dimple, below it and above it). The sweep now records both poles on the
top face, so the reduction takes the crossings path, the re-cut never
runs, and the join refuses `Join(SectionInvariant)` in
`chord_join.rs` — "plane×sphere section tilted against the sphere
chart's polar axis". Every op of the pair refuses there.

The events are right: the poles lie in the face, and premise S of the
section certificate needs them recorded. What is lost is the answer the
fallback's re-chart gave, which the crossings path has no counterpart
for.

Held, refuse-or-answer-correctly, by
`crates/sweep/tests/germ_coplanar_conic.rs`
`every_op_refuses_or_answers_its_closed_form` ("y-poled pip on the
cube's top face"); `m5_pr9c_sphere_doors.rs`
`the_die_pips_shape_stops_typed_at_its_section_roles` pins PR 9c's
smoke shape, the same class, at the join's refusal.

## What the taker owes

The crossings path re-charts a sphere group whose section with a plane
face is tilted against its polar axis (as the fallback's re-cut does),
or the join's tilted-section arm learns the section; either way the
y-poled pip answers its closed form.

## Evidence (2026-10-03, `reach/arc-from-pairing`)

The join learns the section: a chord takes its arc from the germs it
joins and reads no chart, so the tilted plane×sphere section no longer
refuses. Every op of the y-poled pip, and of PR 9c's die-pips shape,
now stops one step later, at the role read: the section's segments run
along the ball's seam edges, and every witness of both hemispheres lies
on the cube's (the slab's) face, so neither section loop reads a side
(`Join(SectionLoopUndecided)`,
`work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior.md`).
Re-pinned at that door.
