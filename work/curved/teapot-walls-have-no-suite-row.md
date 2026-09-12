---
id: teapot-walls-have-no-suite-row
kind: issue
title: The teapot's boolean walls run only in the tour binary and the render lanes - no suite row exercises them
status: open
opened: 2026-09-05
refs: [VERBS-C5ARMS, 1864]
---


## What

Found by the C5ARMS PR-2 dual (R2, NOTE-2). `walls::wall`
(`demos/tour/src/walls.rs`) panics on a moved wall, but `teapot::stops`
is walked only by the tour binary (`demos/tour/src/main.rs`); no suite
test calls it. The klein has `every_klein_wall_is_attempted_by_the_suite`
(`klein.rs`); the teapot has none — `r2_the_two_union_walls_on_my_operands`
only prints. So "teapot walls bit-identical" is carried today by the
render lanes' `scene inputs + uv sheet + wild montage` job (where walls
2/3 refused typed on #1864's head), not by the `demos tour suite` step
PRs cite. A PR that moves a teapot wall is caught only by a render
re-baseline, one job over from where every reader looks.

## Fix

A suite row mirroring the klein's: every teapot stop is attempted and
each wall's refusal (or success) is asserted by variant, so a moved
wall reds in the suite. E; one file.

## Home

CURVED — the teapot walls are curved-boolean frontier rows on this
program's register (`docs/KERNEL-VERBS.md` scope limits).
