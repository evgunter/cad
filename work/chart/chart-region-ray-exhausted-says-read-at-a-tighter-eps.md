---
id: chart-region-ray-exhausted-says-read-at-a-tighter-eps
kind: issue
title: topo: ChartRegionError's RayExhausted and TouchingBoundary Displays offer an unconditional, unvalued 'read the pair at a tighter ε'
status: open
opened: 2026-09-29
---


(CONTACT-10 implementer, from the §5 sweep of `contact/10-contain-endings`;
the rule is D4 ¶1 (i) in `docs/DESIGN.md`.)

## What

`impl Display for ChartRegionError` (`crates/topo/src/chart_region.rs`)
has two arms in the pre-ruling shape, an unconditional, unvalued
tighten:

- `RayExhausted` ends "move the point off the boundary it grazes, or
  read the pair at a tighter ε";
- `TouchingBoundary` ends "Move the boundaries definitely apart or
  definitely across each other … or read the pair at a tighter ε".

A literal grep for "lower the tolerance" does not find either. D4 ¶1
(i) offers a tighter tolerance only on a band-decided arm of a decision
that passes on a nonzero sign, only conditionally, and only with the
value the margin gives. Neither arm carries a margin. `contfp`'s own
`RayExhausted` now ends in its ray decision's lever alone.

`topo::validate::classify_chart_region` renders `TouchingBoundary` at
rest in its lever alone. It renders `RayExhausted` through `too_close`,
the coincidence menu, which PR 3398's
`no_validate_ending_says_lower_the_tolerance` exempts as one of the
menu's composing arms.

## Repair shape

End each arm in its lever, with a `Recourse:` marker. If the census's
chart-region lane is a declarable coincidence, keep the menu at rest and
say so in the arm.
