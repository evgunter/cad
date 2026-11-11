---
id: check-4s-projected-door-reads-every-span-of-a-chord-in-eight-parts
kind: issue
title: Check 4's projected door reads every span of a 625-span chord in eight parts, now the larger part of a cylinder pair's edge certification
status: open
opened: 2026-11-11
priority: P2
cost: M
refs: [a-chorded-edges-certification-hulls-every-span-of-a-625-span-chord, projected-check4-incidence-can-falsely-refuse-in-the-sliver-below-the-band]
---


## What

Filed by the lane that closed
`a-chorded-edges-certification-hulls-every-span-of-a-625-span-chord`
(branch `pcert/chord-hulls-by-span-geometry`, 2026-11-11). That row's
fix reads limb 2's first composite of a net with each span in the
parts its own turn needs (`pcurve_cache::Parts::ByTurn`), and leaves
the rounds at `projected::INCIDENCE_CUTS` (8) parts a span, so a
625-span chord's limb 2 now composes 625 parts where it composed
5,000.

The other reader of the same span hulls still reads eight parts a span:
check 4's projected door, `pcurve_cache::projected_hull_lane` (its
`twin_span_hulls(…, Parts::Each(projected::INCIDENCE_CUTS), …)`), reached
from `projected::projected_envelope`'s `Net` arm through
`FittedLane::projected_hull`. On the cylinder pair
(`cargo run -p sweep --release --example cylinder_pair_cost_probe`,
release, one op a process) its eight composites a boolean, of
40,096–42,560 parts in all, take 2.4–2.8 s, against 0.5–0.6 s for limb
2's sixteen first readings. It is now the larger part of the
certification's cost.

## Why it was not cut with limb 2's

Limb 2 halves: a first reading that does not clear hands the limb to its
rounds, so a coarser first reading costs work and never a decision.
Check 4 reads its incidence term once and never halves it
(`projected-check4-incidence-can-falsely-refuse-in-the-sliver-below-the-band`),
and adds it to the envelope's other terms before one verdict. A span
read in one part reads up to 10.6× wider than in eight on a long span
(a quarter turn round a cylinder, `by_turn_rows`), 1.2× by a pole, and
alike far from both; so the schedule alone would widen check 4's
false-refusal sliver from about 1% of the band to whatever a span's
one-part reading over-states.

## What a fix could be

Halve check 4's incidence term (the sliver row's fix), then start its
rounds at `Parts::ByTurn` as limb 2's are. Or read the term first at
`Parts::ByTurn` and re-read at eight parts a span only where the
envelope does not clear with it, without taking a verdict on the first
reading (its terms are summed before one decision). Measure with the
probe above, and keep the sliver row's census within its 1%.
