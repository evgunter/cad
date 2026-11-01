---
id: the-pass-refuses-a-tier-3-clean-loop-anchored-past-a-pole-slit
kind: issue
title: At a zero-lever pole joint the pass drops the whole-period bookkeeping, so it refuses LoopNotClosed on a loop tier 3 accepts, depending on which half-edge is first
status: closed
opened: 2026-10-04
priority: P3
cost: M
refs: [a-kill-that-re-anchors-a-loops-first-leaves-its-rows-a-period-off-the-pass]
closed: 2026-10-04
---


(TOPO orchestrator, from the probe of kill-kept rows against tier 3. Its report
is `probe-report.md` on `analysis/probe/topo-kill-rows-tier3`, base `0edbbecb`.)

## What

At a pole or apex joint the joint arm is zero, and `pin_branch` skips the
period shift there (`crates/topo/src/pcurves.rs:3626-3630`,
`Ok(Sign::Zero) | Err(_) => Ok(T::zero())`). The skip also drops the
whole-period bookkeeping. A half-edge after the pole can therefore stay a
period off the chain. Continuity cannot see that, because the lever is zero.
Closure can: `loop_closes` (`pcurves.rs:1782`) admits only ±1 period of net
span.

Tier 3 accepts any u gap at a zero-lever joint (`pcurves.rs:4233`, the arm at
`prev.y`), while the pass's closure counts it. So the pass's verdict on a loop
depends on which half-edge is `first`. It can refuse a row set that tier 3
accepts, and the reverse.

## Seen live (once, sweep `ci`)

`snowman::the_snowman_builds_under_every_boolean`: after a `kef_minting` on
the sphere cap face, `validate_pcurves` is clean, and
`mint_pcurves_of(&mut body.clone(), &[cap])` refuses `LoopNotClosed`.

- The loop is: a rim arc, a meridian slit to the pole (both halves of one edge,
  u = π), back down, then a rim arc.
- Walking from the anchor that `kef` re-set, the walk runs the arc over
  u ∈ [−2π, −π] and goes up to the pole at u = −π.
- The down half keeps its base branch at u = +π: a free 2π jump with zero lever.
- The walk ends at 2π, a net span of 4π.

The stored rows, lifted from the old anchor, wrap once and validate clean.
Today the producer's closing pass does not re-mint that face from the new
anchor, so the build does not fail.

## Synthetic recipe

A sphere cap whose loop is two rim arcs plus a meridian slit to a pole vertex,
anchored on the rim arc whose derived base sits a period below the slit's.

## Relation to the re-anchor ruling

`a-kill-that-re-anchors-a-loops-first-leaves-its-rows-a-period-off-the-pass`
(R, Ev, PR 4024) gives a pole joint a reset marker instead of a group element.
That is where this has to be decided:
- if a reset marker means "the next image starts on its principal branch", the
  closure sum has to skip the pole's free period the way tier 3 does;
- if it does not, tier 3 has to count it the way the pass does.

Either way the two must agree, and must not depend on `first`. Build it with R,
or before R as a one-place fix that makes `loop_closes` and tier 3 read
zero-lever joints alike.

## Closed

PR #4037 (R). A pole joint's element is the reset marker
(`JointElement::Reset`). The pass's winding (`Winding::closes`) and tier
3's both count no azimuth across a reset, and both read the same elements
from every member, so neither verdict depends on `first`.

`pcurves::pole_slit_tests::a_pole_slit_loop_re_mints_from_every_anchor`
pins it. That row is the snowman's shape: a cap whose loop is a rim arc
(its parameter a period below), a meridian slit to the pole and back, and
a rim arc. It mints from each of the four anchors with the same rows and
reads tier-3 clean. On `main` the pass refused `LoopNotClosed` from the
arc's anchor.
