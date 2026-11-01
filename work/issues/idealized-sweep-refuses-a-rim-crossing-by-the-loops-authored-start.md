---
id: idealized-sweep-refuses-a-rim-crossing-by-the-loops-authored-start
kind: issue
title: The Idealized sweep refuses a plate crossing a cylinder's rim with CurvedPierceUnsupported or accepts it, depending only on which vertex the cylinder's loop was authored from
status: open
opened: 2026-09-24
priority: P3
cost: D
---


## Finding

Found by EMIT's `loft-section-correspondence-is-authored`, which made
the canonical start of every profile loop its authored vertex 0. Before
that change the start was the lexicographic minimum.

`crates/sweep/tests/s16_box_soundness.rs`, `conic_pruning_never_loses_an_accepted_pair`:
the three-arc cylinder is `cylinder_from(0.0, 1.0, first)`, with vertices at
0°, 120° and 240°, extruded. The plate `rim_plate(-0.499)` crosses its
bottom rim at the rim's x-extreme, 180°, which lies mid-arc between the
120° and 240° vertices. `top_rim_x_plate(-0.499)` crosses the top rim at
the same place.

- Authored from 0°: `sweep_traces(.., SweepStrategy::Idealized, ..)`
  refuses `CurvedPierceUnsupported { operand: B, face: FaceKey(5v1),
  edge: EdgeKey(13v1), .. }` for both plates.
- Authored from 240°, the same point set and the canonical form the
  lexicographic-minimum start used to produce: the reference accepts
  events for both plates.

The realized sweep answers both authorings. The only difference between
the two operands is the order in which their rims, walls and struts were
minted. The frontier door in `crates/topo/src/boolean/reduce.rs`
(the curved sweep arm, `frontier`) is reached for one order and not the
other, so some decision on that path depends on arena or minting order
rather than on geometry.

The test now carries both authorings. The pin binds on the 240° pair
and records the 0° pair as a reference refusal (the file's own rule
for a class the exact lanes refuse typed).

**Confidence:** measured, both authorings, at the default ε.
**Where:** `crates/topo/src/boolean/reduce.rs`, the curved sweep arm;
the fixture is `crates/sweep/tests/s16_box_soundness.rs`,
`cylinder_from`.
