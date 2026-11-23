---
id: the-sector-machinerys-defects-found-while-weighing-what-a-bound-holds
kind: issue
title: Defects in the sector machinery found by reading it for the sector-bound question: a Zero acted on at a lever from below, one lever where every sign is acted on, two corner predicates, a point placed on a tangent ray
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Found on 2026-11-23 by the two designers weighing what a sector bound holds (the question in front of the records unit; `analysis/design-fork/sector-bound-records`), each reading the tree at PR 7628's head `63d0479875`. Every item is BY INSPECTION: nothing was built or run, and no pose shows a wrong answer. They are listed here so that they outlive the designers' reports. The unit that builds the answer to that question closes the ones its design removes; any left when it is cut gets its own row.

**A wrong bound, by the README's lever bullet** ("a verdict … that acts on its Zero reads it from above, and one that acts on every sign reads both"):

- `topo::sector_shape` `apex_extrema_inside` acts on its Zero ("no extremum inside" means the sector is not cut) and is levered at the bare `s.arm.length()`, a length read from below.
- `geom_brep::sector_shape` `corner_shape` and `topo::sector_shape` `apex_corner_shape` act on a Zero at `CornerBounds::SlitTip` (`reads_slit_tip_turn` reads it as a whole turn), at an arm read from below. Benign while a slit tip's two edges coincide by construction; a build that reads both bounds must keep an agreed Zero apart from a straddle there (round 2 of the fork).
- `topo::bound_side` `first_order_as` (under `bound_side::read`) acts on every sign (its Zero is a `Tie` consumers key on) and reads a curved bound or a divider at one lever.
- The curved bound's lever itself: `BoundSupport::Conic.extent` and `ConicDeparture.extent` are `edge_extent_inf`, a bound from below on the edge's diameter, where the departure is read at the vertex and its lever is the edge's far distance from the vertex (the class of `diameter-levers-of-a-datum-read-at-a-point`). On a nearly closed flat ellipse leaving an end of its minor axis the diameter is about twice that distance, on the side that claims a definite sign.

**Two implementations of one logic:**

- `topo::validate::wedge_holds_named` and `topo::boolean::fate::corner_holds` both answer "is this direction inside that corner", levered differently (the second direction's length; the lesser of the two reaches read at both bounds).
- "How an edge leaves a vertex" has four forms: `star::walk::Bound` with `BoundSupport` (a diameter from below), `fate::Leaving` (the reach from the vertex, both bounds), `validate::Departure` (the vector scaled at both bounds), and `link`'s scaled vectors (`TangentFan::Corner`, `Horn.ends`, `ApexLobe.ends`). `fate::leaving` calls `star::departure` and discards the star's lever, so one bound has two levers in the tree.

**A point that is no point of the body, or a length that is no lever:**

- `topo::splitting::rules` `edge_wedge`: `p + along` hands `classify_dihedral` a point on a curved edge's tangent ray, which lies on neither face. Whether any row reaches it with a curved edge at a cone's apex is not known.
- `topo::link::second_order::reach` reads a scaled vector's norm through the point door (`ExtentBall::point(apex + d).lever_from(apex)`): a fabricated point entering the consumed-points door, which neither lever gate sees.
- `topo::link` `fan` sets its rim at `reach · 1.01 / cos(π/8)`, "a hundredth to spare for the rounding": hand-written slack in a construction.
- More sites that make the packed vector than the filed row lists: `census.rs` at `edge_wedge_shape(own * arm, next * arm, …)`; `second_order::thorns`' `along(d, l)`.

- `side_reader` places a ray's start at `reach.length() * −2`: a lever's length as a parameter (round 3).

**A key wider than what it marks** (round 3, not confirmed reachable):

- `SlitTips` keys by `(vertex, face)` and `SlitTips::of` inserts both ends of a shared segment, so every corner of that face at that vertex reads as `CornerBounds::SlitTip`, not only the one between the two slit edges. A slit rooted on a boundary vertex would mark the face's ordinary corners there, and a spike among them would read as a whole turn where it should refuse.

**A refusal spelled as a decision:**

- `geom_brep::conic_bend_side`'s `BendBounds::Unbounded` arm decides `0 − |slope · extent|` under `decide_positive`, which can never pass.

**On another slate, noted there too:** `topo::boolean::vtxfac::held_reads` hands `one_carrier` a ball of the pierce's shortest arm where `ConsumedExtent`'s doc requires a ball enclosing both faces; that under-states in the wrong-answer direction for that ladder (`work/heldrecords/pierce-and-rim-balls-are-built-from-an-arm.md`, on PR 7628's branch until it merges).

P1 by `work/README.md` Priority (two implementations of one underlying logic; the wrong bounds are latent, with no wrong answer shown).
