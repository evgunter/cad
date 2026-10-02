---
id: lily-lanceolate-blade-sections
kind: unit
title: the lily's blades get arc-margined lanceolate sections, and its lantern caps merge
status: open
opened: 2026-10-02
priority: P3
cost: M
---

## What

Two scene changes the kernel unblocked, one lane because both live in
`lily.rs`:

1. **Lanceolate blades.** Every blade section is straight lines today;
   `demos/README.md` and `lily.rs`'s blade notes call arc-margined
   sections "outstanding work… no longer gated on the kernel", on the
   span meter's rational arm (`crates/sweep/tests/cert5_offgrid_knot_rational.rs`,
   issue 453). Give the swept and lofted blades lanceolate sections
   (one arc per margin, each under a half turn — a C⁰ crease between
   two arcs is `tess/lofted-circle-sections-are-unmeshable…`'s
   ground). **The cited row `the_lily_crescent_blade_certifies` is not
   on main** (the opening PR rewrote the citation), and expect what the teapot's spout
   got — rational walls take the quadrature lane, so at ε = 1e-12 the
   volume may be a certified bracket rather than a number. That is
   fine and is reported, as the teapot reports it; the blades' mesh
   Pappus band carries over.
2. **Lantern caps merge.** Wall 13 is retired: the lantern's two
   axis-touching caps merge (10 → 8 faces, 18 → 14 edges). Adopt it in
   the scene through the public merge door if a user building the
   lantern would, and drop the retired probe.

The lily's other walls (1, 2, 7, 8, 12) are still live and stay pinned;
wall 12's prose blames the sphere, whose half is fixed — SHOW's
opening PR corrects it.
