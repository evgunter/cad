---
id: a-wedge-clipped-line-touching-the-window-reads-an-invalid-margin
kind: issue
title: A line clipped to an arc's wedge at a touch point reads its level through a zero-length chord, and the ring meter escalates on an Invalid margin
status: open
opened: 2026-11-11
priority: P3
cost: E
refs: [an-open-tangent-chain-of-mixed-links-refuses-at-the-open-carve, 6810]
---


Filed by the band-openmix lane (2026-11-11), from the delta verification of PR 6810's fix pass.

**Witness.** R2's lean-back L-plate with `L = 0.15`. It is the L-plate's run (radius `R = 0.3`, `r = 0.1`), whose arm along `x = 2` is 0.15 long and ends at a wall leaning back at 45° to `(1.85, 0.30)`. It refuses `Escalated { RingClearance, margin: Invalid }`; before the fix pass it refused `RingClearance` at margin Zero. The leaning edge's endpoint `(1.85, 0.30)` lies on the arc's window plane `y = R`, at radius 0.15 from the arc's centre. That is outside the band's radius range `[0.2, 0.3]`, so the body fits and the refusal is false.

**Likely cause.**
- `StripLevel::wedge_margin` (`crates/sweep/src/blend/surgery.rs`) clips a line to the wedge. Where the line only touches the wedge's boundary, the clip collapses to one parameter (`let (lo, hi) = (lo.min(hi), lo.max(hi));`, surgery.rs:3635).
- It then reads that zero-length window's level through `piece_distance` (surgery.rs:2554).
- `piece_distance` builds the chord `pb − pa = 0` and asks `geom::Line::distance_within` (`crates/geom/src/curves.rs:788`) for a foot along a zero direction. The projection divides by `|dir|²`, which yields the poison `Invalid` that the ring meter escalates on.

**Fix.** Read a degenerate window as its one point, `|pa − c|` for both extremes. This should be decided without a rounding slack, since the bare-rounding gate refuses one. Pin the witness building at Pappus, `ΔV = −A[(2−R) + (π/2)(R−d) + (L−d)]`.
