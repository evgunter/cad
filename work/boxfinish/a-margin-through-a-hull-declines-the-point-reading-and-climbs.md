---
id: a-margin-through-a-hull-declines-the-point-reading-and-climbs
kind: issue
title: A margin whose graph reaches a hull declines the point reading and climbs: the carrier gap's 3,208 climbs, 44 s of the box-arm-off tour drive
status: open
opened: 2026-11-24
---


Found by the refute-first lane's acceptance run (the tour's first drive,
`demo-tour certified`, box arm off, on `boxfinish/probe-switch-off`
with `boxfinish/refute-first` merged in locally).

The point reading (`crates/geom-core/src/sym/point.rs`, `read`)
declines on a `SymOp::Hull` node: a hull is a function of its operands'
enclosures, not of the reals they stand for, so no point value of it is
read. Every undecided ask whose margin reaches one still climbs the
whole ladder. Over the drive, after the change:

| predicate | asks | refuted | climbed (all through a hull) | ladder time |
|---|---|---|---|---|
| `curve_carrier_gap` | 4,816 | 1,608 | 3,208 | 43.9 s |
| `curve_carrier_clear` | 12,536 | 9,488 | 3,048 | 3.0 s |
| `azimuth_near_cut_side` | 5,452 | 4,812 | 640 | 1.5 s |
| `period_near_mark` | 5,448 | 2,240 | 3,208 | 0.3 s |

No hull-declined ask was ever proven `Zero` (measured before the
change: the ladder run on all of them returned no `Zero`), and the
value each declined reading stopped at was a hull (the measurement
recorded the first decline a reading met, so whether a hull's operands
would also decline is not measured). The gap's climbs cost more than the
same climbs did before the change (11.4 s), because the forms they
share were built by asks the reading now refutes.

What would close it is the designers' second reading (B,
`probe/briefs/fork-the-climb/B.md` on `boxfinish/briefs`): a value the
normal form treats as a free unknown may be read at any one point of
its own enclosure. A `Hull` is `Poly::indet(indet_hull(..))` in the
form (`sym.rs`, `combine`), an unconstrained unknown, so a zero form
is zero whatever value it takes. The node table stores no value, so the
hull's enclosure would have to be recorded at mint
(`Sym::enclosure_hull`). Not built: the brief chose the decline, and
the class was measured small before the build (~17 s of 1,013 s).
(boxfinish/refute-first)
