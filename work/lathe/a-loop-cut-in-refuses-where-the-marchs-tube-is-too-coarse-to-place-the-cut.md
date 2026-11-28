---
id: a-loop-cut-in-refuses-where-the-marchs-tube-is-too-coarse-to-place-the-cut
kind: issue
title: A section loop inside a sphere face and a cylinder face refuses R-loop where the march's tube boxes are too coarse to place the cut-in's meridian
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [a-section-loop-inside-two-one-band-curved-faces-refuses-at-the-section-certificate]
---



Found by the LATHE two-curved-interior-loop lane (2026-11-23, branch
`lathe/two-curved-interior-loop`).

## What

A section loop the section certificate places inside a sphere face and
a curved face (R-loop) is cut in along a meridian of the sphere's chart
(`crates/topo/src/boolean/ops/interior_cuts.rs`, `loop_cut_in`), and
`apply_cut_ins` (`crates/topo/src/boolean/ops.rs`, the
`read_loop_tube` and `height_against_tube`) reads where the loop lies off
the march's tube:
every piece in the cut's half-plane, a piece wholly on each side of the
meridian's plane, and each boundary hit on the half-meridian above or
below every piece. Where a piece's box straddles one of those, the cut
is not made and the op answers the certificate's R-loop refusal.

`sweep` `verbs_cylsph_opening::a_ball_scraping_the_wall_refuses_at_the_section_pass`
is such a pose: the ball of radius 0.5 at `(0, 1.8, 0)`, poled along
`z`, against the cylinder of radius 2. Its oval lies about 0.2 off the
ball's centre and comes within about 0.04 of both poles, and the march
lists it in 1121 tube pieces each about 0.25 across. Directly, a piece
straddles the half-plane (`the section loop is not read inside one half
of a meridian plane`); on the re-posed twin, a pole lies between
the pieces' heights (`the sphere face's boundary meets the cut's
meridian inside the section`). Both refuse typed, as the row asserts.

## Ground

The reading of the loop is the march's tube, whose boxes enclose the
loop but do not tighten with it. A tighter enclosure of the component
(the tube's pieces split, or the carrier's own spans with the pad)
would place the cut on these poses too. The torus scrape and the
`mesh` ball against a cylinder wall build.

- 2026-11-28 — The same coarseness reaches a cone partner (PR 7839's review, LATHE). A ball whose pole is tilted toward the cone's loop refuses the loop cut-in at `read_loop_tube`'s half check (`LoopTubeRefusal::OffHalf`) with the loop about 7.5° from the pole: the tube's boxes, not the loop, reach across the axis. It is typed and loud, ahead of the pole placement PR 7839 adds, and it is this row's class: a finer tube near the pole would place the cut. Filed here rather than as its own row.
