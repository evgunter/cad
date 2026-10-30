---
id: loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk
kind: issue
title: props, tier 3's check 6, the mesh walk and the boolean's sectors each walk a face's loop and derive a corner's turn on their own; nothing shares the walk
status: open
opened: 2026-10-29
priority: P1
cost: H
---


Found by FOREIGN's designer pair on `import-normalizes-the-rim-only-cap` and its probe (`probe/foreign-wraptip`, 2026-10-29). A face's loop is walked, and the turn at each corner read, separately in `geom_brep::props::curved` (`sphere_loop`, `cone_loop_winding`), tier 3's check 6 (which reads through the same props reader, so the two cannot disagree when that reader is wrong), `mesh::walk::loop_polygon`, and `topo::boolean::sectors` (`SlitTips`, `build_sectors`). Teaching the kernel one new corner (a same-edge reversal at a wrap tip) took four separate edits (the probe's E2, E4, E6, E7), and one of them, written by hand, gave a volume of 22.81 against a true 23.86 that passed tier 3 because validation and measurement share the reader.

This is the class two-implementations-of-one-logic (P1): one loop walk that reads each corner's turn once (the same-edge reversal by Ev's 2026-10-14 ruling, fork row 135, as `topo::chord_join`'s pole branch already does), which every reader cites. Owner undecided: the readers span PROPS (geom-brep props), TOPO (validate, boolean sectors) and TESS (mesh). Independent of `[ev]` PR 6014's pole decision; under the wrap tip it would be a precondition.

Signed: (FOREIGN orchestrator)

## First in the wrap-tip sequence (Ev, `[ev]` PR 6014, 2026-10-29)

Ev approved the wrap tip "if the complexity it adds is tolerable", asking that keeping it down be investigated. This row is that investigation's answer: one loop walk that reads each corner's turn once, the tip's included (a walk out and back along one edge contributes nothing to a loop integral: row 135's structural reading, which the designers expect to equal the probe's corrected slit-base turn; re-run the probe's tilted-sphere union and slab-minus-pip rows against it before any reader cites it), cited by props' sphere and cone loops (and so check 6), the mesh walk and the boolean sectors. The CARVE, TOPO, SPLITDOOR, TESSWALK and FOREIGN wrap-tip rows read the tip through it. If the walk cannot be made one home, say so on Ev's channel before the readers each learn the tip. Raised to the first step; still P1. (FOREIGN orchestrator)

## One more walker (RESTCURVE, PR 6054, 2026-10-30)

`geom_brep::chart_singularity::walk_loop` walks a face's loop through
its chart to read which side of it a pole or apex lies on (tier 3's
chart-singularity arm and step-import's normalization), for the loops
props' closed form does not read. It reads a corner only as the azimuth
step across it. Beside it, `chart_singularity::without_spurs` takes a
loop's out-and-back pairs out before either reading (a spur bounds no
area): the structural reading of a same-edge reversal this row's walk
would own. Both are this row's to absorb. (RESTCURVE implementer)
