---
id: the-cone-stores-its-slope
kind: unit
title: Surface::Cone stores its opening as its slope k = tan α, not its half-angle; every reader, STEP import/export and the representability margins move
status: open
opened: 2026-11-21
priority: P1
cost: H
parent: the-cone-has-no-implicit-enclosure-or-metres-conversion
refs: [7326]
---


Decided by Ev on `[ev]` PR 7326 (2026-11-22: "sure sounds good"). A cone stores its opening as its slope `k = tan α`, the generator's radial run per unit of axial rise. C9 states it.

**What moves:**
- **Producers:** revolve stores `|dr|/|dz|` directly instead of `atan` of it, and the blend arm and the section certificate's cone search store their own ratios.
- **STEP:** STEP import converts once (`tan`), and export converts once (`atan2`).
- **Readers:** read `cos α = 1/√(1+k²)` and `sin α = k·cos α`, with no transcendental at any scalar.
- **Representability:** becomes `k > 0` and `1/k > 0`.
- **Also moving:** `same_content_by`, the offsets (`v` shifts by `d/k`), and every `half_angle` reader (about 266 files).

**Sequencing:** it starts after the cone units in flight land, so it rebases nobody's work mid-flight: `the-search-certificate-reads-limb-two-through-the-edge-reader`, `the-tube-enclosure-has-a-cone-arm` and `the-cone-ruled-side-on-the-section-value`. Those cross `sin α`/`cos α` at `Interval`; this unit replaces those crossings with the slope's algebra.

Priority: P1, an architecture that entrenches as more cone readers are written.
