---
id: a-section-circle-through-both-chart-poles-leaves-a-cut-face-with-no-pole-to-read
kind: issue
title: A section circle through both of a sphere's chart poles leaves a crossings-path cut face with no pole to read, so its closing cannot decide which part keeps the outline (the die's slab against a y-poled ball)
status: open
opened: 2026-11-27
priority: P1
cost: H
refs: [close-cuts-is-not-total-on-crossings-path-cut-ins]
---

## What

The die's slab `[−2, 2]² × [0, 0.7]` against the `y`-poled unit ball
(`m5_pr9c_sphere_doors::the_die_slab_cutting_the_ball_in_its_top_builds_where_its_closing_reads_a_pole`).
- The slab's top cuts the ball's one face in the interior circle `z = 0.7`, which the crossings path cuts in (PR 7440).
- Its bottom cuts the face in the circle `z = 0`, a first-pass crossing that passes through both chart poles `(0, ±1, 0)`.

With the cut made, the output stage's closing (`topo::boolean::slit::close_cuts`) parts the cut face's loop at the cut's piece (`part_cut`). Both parts are bounded by section circles alone, so it reads which part keeps the outline off the chart's pole (`pole_beyond`): the part with the pole beyond one of its circles. Here the pole lies on the `z = 0` circle's plane and on the `z = 0.7` circle's near side, so no part reads it beyond, and the closing answers `CutOutlineUndecided`.

The crossings path then leaves the op uncut. The interior-loop guard's refusal (`CurvedPairUnsupported`, `InteriorLoopGuard`) stands in all six ops, in either order (`a_crossings_path_cut_in_is_made_only_where_its_closing_is_total`). Measured with the closing's pole premise read before the re-entry removed: ∩ in both orders and ∖ with the ball second raised `JoinDesync` before that refusal was typed. ∪ in both orders and ∖ with the ball first built tier-3′ sound.

## Why it is not a closing question alone

The `z = 0` circle puts both poles in the face's closure, and DESIGN `[topology]` makes a chart singularity "a vertex of every face whose closure holds it". So the face the closing reads is itself owed two pole vertices on a section curve, which the closing does not mint. A reading of the outline by another point than the pole (a pole of a turned chart, or a sphere point the face decidedly excludes) would part the loop, but the result would still lack those vertices.

## Done when

The pole-form owners decide what a section curve through a chart singularity mints, and the closing decides the outline for the face that results. Both are outside LATHE's crossings-path cut-in (ORBIT's `slit.rs`, and the pole form). The die's slab then builds in all six ops in either order, and its rows flip.

