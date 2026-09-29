---
id: chart-region-ray-exhausted-says-read-at-a-tighter-eps
kind: issue
title: topo: ChartRegionError::RayExhausted's Display offers an unconditional, unvalued 'read the pair at a tighter ε'
status: open
opened: 2026-09-29
---


(CONTACT-10 implementer, from the §5 sweep of `contact/10-contain-endings`;
the rule is D4 ¶1 (i) in `docs/DESIGN.md`.)

## What

`impl Display for ChartRegionError`, the `RayExhausted` arm
(`crates/topo/src/chart_region.rs`), ends "move the point off the
boundary it grazes, or read the pair at a tighter ε". That is the
pre-ruling shape: an unconditional, unvalued tighten. A literal grep for
"lower the tolerance" does not find it. D4 ¶1 (i) offers a tighter
tolerance only on a band-decided arm of a decision that passes on a
nonzero sign, only conditionally, and only with the value the margin
gives. An exhausted schedule carries no margin. `contfp`'s own
`RayExhausted` now ends in the lever alone ("Recourse: move the geometry
clear of the boundary").

`topo::validate::classify_chart_region` renders the at-rest ending for
this arm through `too_close`, the coincidence menu. PR 3398's
`no_validate_ending_says_lower_the_tolerance` exempts it as one of the
menu's composing arms.

## Repair shape

End the arm in its lever, with a `Recourse:` marker. If the census's
chart-region lane is a declarable coincidence, keep the menu at rest and
say so in the arm.
