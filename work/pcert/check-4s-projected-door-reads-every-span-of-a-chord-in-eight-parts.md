---
id: check-4s-projected-door-reads-every-span-of-a-chord-in-eight-parts
kind: issue
title: Check 4's projected door reads every span of a 625-span chord in eight parts, now the larger part of a cylinder pair's edge certification
status: closed
closed: 2026-11-12
opened: 2026-11-11
priority: P1
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

## Re-banded P1 (2026-11-11)
By SPIRICARM's orchestrator, under the blocker rule (`work/README.md`, Priority): `spiricarm/a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses` (P1, about 10 s per ordinary cylinder boolean in release) is parked on this row and ONEHOME's/PCERT's sibling, which are the remainder of its cost after PCERT's chord hulls (re-measured there).

## Closed

Branch `pcert/check4-parts-by-turn`. Check 4 reads a curved chart's net
first with each knot span in the parts its own turn needs
(`pcurve_cache::Parts::ByTurn`, the parts limb 2's first reading
takes), under a `k_stats::detached` recording
(`projected::run_projected_checks`). Each part's hull bounds `|f|` at
any count, so a first-reading envelope that passes is the check's, and
its recording is spliced in. Anything else is set aside with its
recording, and the envelope is read whole again at `INCIDENCE_CUTS`
parts a span. So every refusal, escalation and sector or lever ending
of check 4 is read on main's parts, and the sliver below the band is
main's: `projected-check4-incidence-can-falsely-refuse-in-the-sliver-below-the-band`
stays open, since the halving it asks for is not taken here.

On the cylinder pair (`cylinder_pair_cost_probe`, release, one op a
process, two runs), check 4's envelope takes 0.36–0.46 s per op, down
from 1.75–2.15 s, and reads 10,024–10,640 parts, down from
80,192–85,120. Every chord passes on its first reading. The ops take
4.7–7.3 s, down from 6.1–8.7 s, with identical volumes.

Rows (`pcurve_cache/by_turn_rows.rs`):
- the 625-span chord's one composite of 625 parts;
- a first reading's bound over a span bulging inside its part, red
  under mutant `by-turn-sampled`;
- a first reading that does not pass, decided by the eight parts' second
  composite.

The dominance fuzz reads both schedules.
