---
id: a-chord-ends-triangle-could-read-a-cycle-outside
kind: issue
title: blend: a chord end's exact triangle could certify a cycle wholly outside its sliver; today such a cycle refuses as a bound
status: open
opened: 2026-10-13
priority: P3
cost: E
---

Found by PR 4957's review (NOTE 7). A chord end's sliver is read as its
exact triangle (`crates/sweep/src/blend/open/end_face.rs`, `Exact::Triangle`),
and `CapSliver::read_cycle` reads a cycle not clear of `Ω` three ways:
inside, crossing, or a bound. It has no "outside" verdict, so a cycle
in `Ω` that the triangle itself could certify wholly outside `S` still
refuses as a bound. On a round end that is forced (`Ω` contains `S` and
answers only "clear"); on a chord end it is a missed loosening: a
cycle lies wholly outside a convex polygon when every piece stays past
one side, `piece_along`'s `high` along that side's inward normal
negative, and the least of those is its clearance.

**Witness.** `crates/sweep/tests/band_planar_consume.rs`
`a_bore_past_a_rim_in_kept_material_is_never_consumed`: a C-section
whose upper jaw overhangs the lower jaw's support across a `0.005` gap,
a bore of radius `0.003` at `(0.97, 1.01)` (and its transpose) in the
upper jaw, inside `Ω` about the chord's midpoint but `≈ 0.0157` outside
the triangle past one rim side. The fillet builds and keeps the bore at
`ΔV = −A`; the chamfer refuses `RingClearance { bounded: true }`. The
row accepts either outcome; once the chord end reads the ring outside,
the chamfer's arm should build at `ΔV = −D²/2` unless another meter
refuses, and the row can pin which.

**Not a defect**: a refusal, and a bound, so nothing wrong is built. The
row pins that the cycle is never consumed and never measured as a
crossing, and keeps doing so after the fix.
