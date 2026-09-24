---
id: teapot-walls-have-no-suite-row
kind: issue
title: The teapot's boolean walls run only in the tour binary and the render lanes - no suite row exercises them
status: closed
opened: 2026-09-05
refs: [VERBS-C5ARMS, 1864]
closed: 2026-09-15
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

## Closed (2026-09-15, PR 2626)

**Closed by a SUITE unit, not by CURVED**: `D403` on SUITE's slate is
this same defect, found independently and filed two days earlier
(2026-09-03) from the lane that closed `#1434`'s two halves. Two
programs held one row; this is the one that was not dispatched, so it
is closed here rather than worked twice.

What landed is the fix this row asked for — *"every teapot stop is
attempted and each wall's refusal (or success) is asserted by
variant"*:

- `teapot::wall_probes` now carries walls 2 and 3, called by
  `teapot::stops` and by an in-bin `#[cfg(test)]` test,
  `every_teapot_wall_is_attempted_by_the_suite`. It takes the scene's
  `Evaluation` rather than building one, because the unions refuse at
  `evaluate` and the panel's note quotes the same payload the probe
  pins.
- The sweep that unit ran found the class had a second live member,
  `torusvessel`'s wall 1, and closed it in the same PR, so **every**
  `walls::wall` call site in the tour is now suite-driven.
- The by-variant half is real and not merely the attempted half:
  flipping the pinned `other_kind: SurfaceKind::Sphere` to `Cone` turns
  the teapot row FAILED, and flipping torusvessel's pinned `what`
  string turns its row FAILED at `walls::wall`'s *"refuses, but NOT
  with the refusal it pins"* arm. Both go green when restored.

Evidence, read off the step rather than a job name — hosted run
34940253650, job 104287198853, step *"demos tour suite (the #99 ε pin +
the tour's own probes)"*:

```
test klein::wall_probes_run_here::every_klein_wall_is_attempted_by_the_suite ... ok
test lily::review_probes::the_wall_list_still_stands ... ok
test teapot::wall_probes_run_here::every_teapot_wall_is_attempted_by_the_suite ... ok
test torusvessel::wall_probes_run_here::the_sectioned_vessels_wall_is_attempted_by_the_suite ... ok
```

So a PR that moves a teapot wall now reds in the `demos tour suite`
step PRs cite, not only in a render re-baseline one job over.

**One thing this row named is NOT closed**, and it has its own file:
`r2_the_two_union_walls_on_my_operands` still only prints — see
`work/curved/r2-union-wall-probe-only-prints.md`. The render lanes'
bit-identity claim is left where it is: that job's subject is the
rendered output, not the frontier, and the frontier now has its own
row.
