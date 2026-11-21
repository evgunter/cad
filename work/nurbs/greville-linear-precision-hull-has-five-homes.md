---
id: greville-linear-precision-hull-has-five-homes
kind: issue
title: the Greville linear-precision hull is hand-copied in three production sites and two test re-derivations
status: open
opened: 2026-09-04
priority: P1
cost: D
---


The bound `sup |B(v) − M(v)| ≤ maxᵢ |bᵢ − M(ξᵢ)|` — Greville
abscissae `ξᵢ = mean(knots[i+1..=i+p])`, valid for unit weights by
linear precision plus the convex hull — is now hand-copied at:

* `crates/geom-brep/src/pcurve_cache.rs` — the CAP-class line-rim arm
  (the original), and
* `crates/geom-brep/src/pcurve_cache.rs` — the SEAM-class Line limb
  (EXCH-H1, transposed onto the seam channels), and
* `crates/step-import/src/recognize_curve.rs::straight_map` — INV-C5's
  straight MAP reading (EXCH-H1's fix pass; moved off promotion onto
  the column readers by EXCH unit B, PR 7170),

plus two test-side re-derivations of the same ξᵢ arithmetic
(`pcurve_cache.rs`'s seam-limb rows; `recognize_curve.rs`'s L2/L3
pinned values). Five copies of one three-line derivation, each with
its own weight gate, and the copies have drifted: geom-brep's asks
unit weights (`*w != 1.0`), step-import's asks one CONSTANT weight
(`NurbsCurve3::constant_weight`, which cancels), and step-import's
distances are formed at `Interval` and read outward while geom-brep's
are `f64` (EXCH unit B, 2026-11-20). The natural shared home is
`geom-core/src/spline/` — S-CERT's fence (Track N), which no other
program edits — so this item RECORDS the sites and flags S-CERT
coordination rather than proposing a lane: consolidation is a
`spline::` API addition (a `greville_abscissae` iterator, or the hull
itself), dispatched by or with S-CERT.

Found by the EXCH-H1 fix pass (PR #1798), from the adjudicated review
union.

Moved from `work/issues/` to `work/nurbs/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.
