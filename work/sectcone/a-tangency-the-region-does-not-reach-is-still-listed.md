---
id: a-tangency-the-region-does-not-reach-is-still-listed
kind: issue
title: A cone pair's tangency the region does not reach is still recorded as the section's tangency
status: open
opened: 2026-11-22
---


Found in PR 7357's review. PR 7354's lane is filing the same class for plane × cylinder and the cylinder pair ("a tangency the region does not reach is still listed"). If that row lands, merge this evidence into it.

`pair_section/cones.rs`, `cone_ruled`, the `Sheets` arm: every double root the sweep decides a tangency (`ruling::Touch`) pushes a `ZeroVerdict { Tangency(ruling::TANGENCY) }` into `decided`, including a touch at a point the region does not hold. Only the `Point` component is filtered by `held`, and the crossing node by the sweep's `crossings`. So `PairSection::tangency()` reports a tangency for a section whose listed components do not reach it. Step-import (`build.rs`, `served_pair`) reads that flag to take its tangent path; it now refuses cone pairs earlier, by `pair_section::uncarried`. But any later reader of `tangency()` on a cone pair would read a tangency the operation cannot use.

The fix shape is to push the verdict only for a touch the region holds, or to record it beside the placement, as the cylinder pair's `placed` does.
