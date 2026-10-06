---
id: inner-loop-circles-bound-the-plate-study-at-arc-span
kind: issue
title: The two-hole plate as one extrude with two inner-loop circles certifies whole boxes only to ~1e-2 of its study, bounded by arc_span, and certifies 0 of 512 leaves over the real study
status: open
opened: 2026-10-03
priority: P3
cost: M
---



Found by SHOW's `the-plate-document-never-cuts-its-holes` (PR 3922);
measured with a scratch probe, not pinned in the tree.

The tour's two-hole plate written as ONE extrude of a profile whose
loops are the 8 × 4 mm rectangle and two `LoopProgram::Circle` inner
loops (centres `∓half_spacing`, radii `hole_a_r`/`hole_b_r` — the
study's three parameters), the web measure reading the two bore walls
of that extrude. This spelling needs no boolean, so it never meets the
subtract's volume tie
(`work/reach/a-hole-wholly-inside-its-target-ties-the-subtract-volume-bound.md`).

Whole-box drive (one leaf), default ε, fraction of the real study
(±0.05 mm spacing, σ = 0.01 mm radii):

| box | certifies whole | over the band (shape-report replay) |
|---|---|---|
| 1e-9, 1e-6, 1e-3, 1e-2 | yes | — |
| 1e-1 | no | `arc_span` `[-7.32e-5, 1.03e-4]` 1/14 |
| 1 | no | `arc_span` `[-1.45e-3, 1.27e-3]` 2/10; `assert_bound` (the study's real flip); `ray_side` `[-5.7e-5, 2.0e-5]` 1/20 |

At the real study with the tour's 512-leaf budget the drive certifies
0 of 512 (511 splits; review measurement), where the two-extrude
spelling the tour ships certifies 193. The holes stand 0.75 mm off the
outer loop at every point of the box, so `arc_span` straddling zero
at a 5 µm box reads as dependency widening of the span margin
(`crates/profile/src/seg.rs`, `arc_span`), not a real flip.
Undiagnosed beyond that.
