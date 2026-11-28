---
id: a-section-circle-through-both-chart-poles-leaves-a-cut-face-with-no-pole-to-read
kind: issue
title: A section circle through both of a sphere's chart poles leaves a crossings-path cut face with no pole to read, so its closing cannot decide which part keeps the outline (the die's slab against a y-poled ball)
status: open
opened: 2026-11-27
priority: P2
cost: H
refs: [close-cuts-is-not-total-on-crossings-path-cut-ins]
---

## What

The die's slab `[−2, 2]² × [0, 0.7]` against the `y`-poled unit ball
(`m5_pr9c_sphere_doors::the_die_slab_cutting_the_ball_in_its_top_builds_where_its_closing_reads_a_pole`).
- The slab's top cuts the ball's one face in the interior circle `z = 0.7`, which the crossings path cuts in (PR 7440).
- Its bottom cuts the face in the circle `z = 0`, a first-pass crossing that passes through both chart poles `(0, ±1, 0)`.

With the cut made, the output stage's closing (`topo::boolean::slit::close_cuts`) parts the cut face's loop at the cut's piece (`part_cut`). Both parts are bounded by section circles alone, so it reads which part keeps the outline off the chart's pole (`pole_beyond`): the part with the pole beyond one of its circles. Here the pole lies on the `z = 0` circle's plane and on the `z = 0.7` circle's near side, so no part reads it beyond, and the closing answers `CutOutlineUndecided`.

The crossings path then leaves the op uncut, and the interior-loop guard's refusal (`CurvedPairUnsupported`, `InteriorLoopGuard`) stands. That is so in three of the six runs: ∩ in both orders, and the slab less the ball. The union in both orders and the ball less the slab keep no part bounded by section circles alone that reads no pole, so they build (`a_crossings_path_cut_in_is_made_only_where_its_closing_is_total::the_die_slab_builds_where_its_closing_reads_a_pole_and_refuses_where_none`). Measured with the closing's pole premise read before the re-entry removed: ∩ in both orders and ∖ with the ball second raised `JoinDesync` before that refusal was typed. ∪ in both orders and ∖ with the ball first built tier-3′ sound.

## Why it is not a closing question alone

The `z = 0` circle puts both poles in the face's closure, and DESIGN `[topology]` makes a chart singularity "a vertex of every face whose closure holds it". So the face the closing reads is itself owed two pole vertices on a section curve, which the closing does not mint. The bodies that build hold the clause: the union in both orders and the ball less the slab each carry one vertex at each pole `(0, ±1, 0)`, on the `z = 0` circle, and tier 3′, whose chart-singularity arm reads that form, passes. The slab row asserts the vertices (2026-11-28). A reading of the outline by another point than the pole (a pole of a turned chart, or a sphere point the face decidedly excludes) would part the loop, but the result would still lack those vertices.

## Done when

The pole-form owners decide what a section curve through a chart singularity mints, and the closing decides the outline for the face that results. Both are outside LATHE's crossings-path cut-in (ORBIT's `slit.rs`, and the pole form). The die's slab then builds in all six ops in either order, and its rows flip.


## 2026-11-28: root cause, options, re-banded P1 → P2 (LATHE scoping, main `4b2903db2f`)

**Root cause.** `slit::pole_beyond` (`crates/topo/src/boolean/slit.rs:1278`) reads which part of a face bounded by section circles alone keeps the outline off the chart's pole. Its premise is that the face holds no edge of its own boundary there, so the pole lies outside it, beyond one loop. The `z = 0` circle puts both poles on a circle's plane: the pole's side reads `Zero` against it, so neither part reads the pole beyond it, and `part_cut` answers `CutOutlineUndecided` (`slit.rs:1192`). The premise is false here, since the poles lie on the face's boundary.

**Options.** Each is a change to ORBIT's closing in `slit.rs`. LATHE does not decide it.

- **(a) A reference point decidedly outside the face**, read through the face's own point-in-face door, refusing where it is read in band. General, and it fails loud. Cost M. *LATHE's lean.*
- **(b) The pole of a chart turned off every circle's plane.** Cost S–M, but whether the turned pole lies outside the face is its own unproven premise, which a row would have to hold.
- **(c) Pole vertices minted on the section circle before the closing, which then decides by the vertex.** This is the pole-form question on a section curve. The bodies that build already carry those vertices (above), so (c) is a pole-form fork rather than a closing fix. Cost M–H.

**Re-banded P1 → P2.** It is a typed refusal, never a wrong answer, and not a regression. Main before the one-band stack (`84588ee6fa`) refused all four of the die slab's ops at the interior-loop guard (`m5_pr9c_sphere_doors::the_die_slab_cutting_the_ball_in_its_top_refuses_at_the_interior_loop` there). Head builds two of the four.
