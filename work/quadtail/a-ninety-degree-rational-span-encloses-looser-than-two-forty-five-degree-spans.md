---
id: a-ninety-degree-rational-span-encloses-looser-than-two-forty-five-degree-spans
kind: issue
title: The mass-props quadrature encloses a 90° rational span looser than the two 45° spans it was: arc lofts' pads and refusal widths grew ~1.5× when C6's η made a quarter arc one span
status: open
opened: 2026-10-18
priority: P2
cost: M
refs: [an-enclosure-carrier-holds-point-weights-and-knots]
---


C6's sub-arc rule (S3 of `work/inlane/an-enclosure-carrier-holds-point-weights-and-knots.md`, η = 1/16) gives a quarter arc one rational-quadratic span: weights `1, cos 45°, 1`.

**What moved.** The shared `arc_section` fixture is a quarter-circle bulge whose sweep `4·atan(tan(π/8))` rounds just past π/2. It used to take two 45° spans (middle weights `cos 22.5°`). The mass-props quadrature encloses the one wide span more loosely. S3 re-cut `crates/sweep/tests/thread-count-digest/eps-{1e-6,1e-9,1e-12}.txt` on that cause, at the orchestrator's ruling (a). At ε 1e-9:
- **`arc_loft_1e9eps`:** vpad 9.8e-7 → 1.5e-6 (`3eb0783411700000` → `3eb9582dca3aaaab`), apad 0.085 → 0.115. Volume and area move in their last bits.
- **`arc_loft_1e11eps`:** still refuses, its face's certified contribution 1.10e-5 → 1.70e-5 m wide.
- **`thin_strip`:** still refuses, 9.5e-5 → 1.6e-4 m wide.
- **The same factors at ε 1e-6 and 1e-12.** The sym-session block does not move.

**The lever** is subdivision inside the integrator, not the carrier's control count. Split a wide rational span for integration, so the enclosure over one 90° span is as tight as over two 45° spans. The count rule gives a quarter arc one span, and that is the representation the kernel holds (the orchestrator's ruling, against doubling every arc's control net).

**Done when:** `arc_loft_1e9eps`'s pads are back at or under their two-span values, and the two refusals' widths are back at or under theirs. Re-cut the digest on that cause.
