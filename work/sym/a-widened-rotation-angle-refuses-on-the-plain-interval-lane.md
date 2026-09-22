---
id: a-widened-rotation-angle-refuses-on-the-plain-interval-lane
kind: issue
title: a widened rotation angle refuses on the plain Interval lane at the first Node::Transform: cos^2+sin^2 is a bracket around 1 and transform_rigid_col0_unit is what notices
status: open
opened: 2026-09-22
priority: P1
cost: D
refs: [SYM-14]
---



## What

**Specifically requested by Ev, 2026-09-22** — found by SYM-14, the
chain demo Ev asked for, measuring the certified lane on it
(`demos/tour/src/chaintol.rs`, the cell's own CI row).

One leaf over the whole declared box, plain `Interval`, on a chain of
ONE link whose single joint carries σ = 0.01 rad:

```
node 7 — the transform op refused: transform: the map's linear part is
not an isometry at tolerance — predicate transform_rigid_col0_unit
refused, definitely or in-band
```

and the same at two, three and four links, always at the FIRST
transform. MEASURED, and pinned per link count by `chaintol.rs`'s
`the_certified_table_says_what_the_header_says`.

**The mechanism below is a reading of that refusal, not a second
measurement**, and is written as such: `eval::wire`'s `transform_map`
builds `Mat3::rotation_about(axis, angle)`, whose columns are built out
of `cos(angle)` and `sin(angle)`; on an interval angle those are two
independent brackets, so `cos² + sin²` would be a bracket AROUND 1
rather than 1, and that is what a column-unit check would refuse on.
It fits the predicate's name and the map's construction, and nobody has
yet read the column norm's own enclosure at the refusal to confirm it.
What IS established without it: the refusal is at the first transform,
at every link count, and it is not a geometry error — the same document
certifies whole on the symbolic lane at one link.

**The symbolic tier discharges exactly that**, and that is the whole
difference between the two lanes on this document: the same one-link
chain CERTIFIES whole at `Sym<Interval>` in 0.16 s, with
`symbolic_zero: 1760`. So the finding is not "the kernel is wrong", it
is that **the plain interval lane cannot carry a rotation by a widened
angle at all**, at any width, and every consumer of a parametric
placement is on the symbolic tier or nowhere.

Worth stating because the tier is a *performance and reach* dial
elsewhere (`DriveConfig::symbolic`, `SymBudget::none()`) — on this
construction it is load-bearing for the answer existing.

## What answers it

First, cheaply: the column norm's enclosure at the refusal, read out —
which turns the reading above into a measurement or replaces it.

Then, either an interval-lane rotation that keeps the Pythagorean identity
(the rotation built so the column norms are exactly 1 by construction,
rather than recomputed from two independent brackets), or the
statement, where `transform_map` is written, that a widened
`SlotId::RotationAngle` is a symbolic-tier-only construction and the
plain lane's refusal is the designed answer.

## Home

SYM — the tier's reach on a construction Ev asked for.
