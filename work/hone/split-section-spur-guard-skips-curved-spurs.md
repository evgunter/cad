---
id: split-section-spur-guard-skips-curved-spurs
kind: issue
title: The split join's spur guard decides straight tips only; a curved out-and-back spur passes
status: open
opened: 2026-09-24
priority: P3
cost: M
---


## What

`Sweep::refuse_section_spur` (`crates/topo/src/splitting/join.rs`)
refuses a completed section polygon whose loop runs out along an edge
and straight back (`SplitJoinError::SectionSpur`). It decides only at
tips where BOTH incident edges are straight.

A curved out-and-back has the same defect: the loop runs out along an
arc and back along the same arc. Its two conic excesses in
`certify_section_area` cancel, so the spur bounds nothing and the
polygon's net area is the real section's. Two DIFFERENT arcs between
one pair of points (complementary arcs of one circle, or arcs of two
circles) do bound area. Telling the two cases apart needs a carrier
comparison, same carrier traversed in reverse, and the guard does not
make it.

## Why it matters

Latent. No measured document reaches a curved spur. The straight case
reached one through `split`'s pinch lane: a one-sided tangency along a
straight edge, rerun mirrored, where the plane also cut another part
of the solid. The curved analogue would be a plane tangent along a
circular edge that also cuts elsewhere, and whether that reaches
`split` has not been measured.

## Found by

PR 3133's review (m1).

## Evidence (CLEAVE, PR 4098, 2026-10-06)

The guard has no producer left. PR 3726 (rule (b)'s convex edge) and
PR 4098 (rule (a) refusing a wall that bends away from its material,
`SplitReduceError::KnifeEdge`) removed the two contacts known to mint a
null edge of their own. An instrumented run over topo's and sweep's
suites, CLEAVE DR-4098's probes and 22 mirrored reruns reached it 0
times. PR 4098 turned the straight-tip check into an invariant
(`Sweep::assert_no_spur`, which panics on an exact spur) and retired
`SplitJoinError::SectionSpur`. The curved blind spot this row names is
now a blind spot of that invariant check, not of a refusal.
