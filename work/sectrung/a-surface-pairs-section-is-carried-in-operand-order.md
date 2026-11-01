---
id: a-surface-pairs-section-is-carried-in-operand-order
kind: issue
title: A surface pair's section is carried in operand order, so A∪B and B∪A march different carriers and cut different chord bits from one pose
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Found by the SECT orchestrator's delta read of step 1b's fix pass
(PR 5894, 2026-10-28).

## What

The section table keys a pair by A's surface then B's
(`topo::boolean::section_table`'s `PairKey`) and reads it with
`geom_brep::pair_section(sa, sb, band)` in that order. The cylinder ×
sphere pose keeps the order (`CylinderSpherePose::decide`,
`cylinder_first`), and the general rung marches in it
(`ssi::march_cylinder_sphere`, `ImplicitPairR3 { a, b }`). So `A ∪ B`
and `B ∪ A` on one pose march two different fitted loops. Their seats
are canonical within an op (`join::carried_seats`), but the chords cut
from them are different bits, and so are the edges and pcurves each op
builds. No pinned run moves door between the two orders today
(`crates/sweep/tests/cylinder_sphere_frame.rs`), but once the bodies
build they will differ by the two fits.

## Design note

The section is a property of the unordered pair. The value can be
built once per unordered pair, with its operands in a canonical order
(by kind, then a key-independent rule), and read by either operand
order through that one carrier. The march, the seat and the chords then
see the same bits whichever operand is A. Only the readers that name a
side (the germ's branch, the foot cache's operand) keep the operand.
Not built here.

## What the second order costs (SECT `sect-seamslow`, 2026-10-30)

In the seam-region door rows a ball's ∪, ∩ and A ∖ B with a cylinder
cut the same chord arcs, so after the first operation limb 2's rounds
and the projected rows' span hulls are memo hits; B ∖ A carries the
section in the other order, cuts other bits and computes them all
again. At the default ε `unit b-a rod` costs 6.3 s against `unit i rod`'s
3.5 s and `unit a-b rod`'s 2.5 s (opt-level 2, with
`sect/seam-region-speed`'s span memo). A canonical order would make B ∖ A
a reader of the same memo entries.
