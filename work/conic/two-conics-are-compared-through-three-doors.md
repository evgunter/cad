---
id: two-conics-are-compared-through-three-doors
kind: issue
title: Two conics are compared through three doors of the conic: the same content, the same locus in either sense, and one's level along the other's position
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [7523, 7510]
---

Owed by Ev's answer on PR 7523 (2026-11-23) and filed when CONIC opened. The three designers agree (their second round, and each final text in PR 7523's body) that code comparing two conics asks one of three questions, each a door of the conic and none a read of two centres:

- **the same content**: the two store the same curve (today `Curve3::same_content_by` walks the stored data, under PR 7510's gate);
- **the same locus, in either sense**: exactly, as an answer about the two curves and not about their frames;
- **one's level along the other's position**, read as a sign pattern: where the second runs inside, on and outside the first.

The readers that compare two conics today each form the comparison from both centres: in `curve_pair/pose.rs` `conic_conic` (whose parallel constant certified `Apart` for a circle crossing an arc-held ellipse), `coplanar_circles`, `coplanar_ellipses` and `one_curve_gap`; in topo `injective.rs` `one_conic` and `one_stated_conic`, and `ring_path.rs` `coplanar_apart`. `oneround/conic-pair-readers` re-poses the curve-pair ones as plain readings from the stored point; this row is the doors themselves and the callers that unit does not reach. Build on the polynomial along a conic (`the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre`) where the level needs it.

## 2026-11-23: `one_stated_conic` reads no centre for two ellipses (`embed/conic-symmetry`)

Evidence for this row, from the EMBED lane. Since PR 7491
`one_stated_conic` compares two stored ellipses by `same_content` and
the stored `v`, never by a derived centre; `embed/conic-symmetry` widens
that to the other's exact restatements (`Ellipse::restatements`: the
seam a half turn on, run back, both), which is "the same locus, in
either sense: exactly" for the pairs an exact negation of stored numbers
reaches and for no other. Two circles are still compared by their whole
stored frames (`one_stated_plane`), and `one_conic`'s lifted arm still
reads the lifted centres as one value. What that leaves this row on the
EMBED side: a quarter turn
(`a-sections-two-arcs-seamed-a-quarter-turn-apart-are-two-statements-a-rounding-apart`),
read as two by both of the certificate's readings, and an ellipse held
at its centre against the same one held on its arc, which the stated
reading says are two and the lifts say are one exactly where every
lifted component is a point, the derived centre among them (corrected
on 24 Nov from "both read as two today", after the review of PR 7640
showed the second through `one_conic`).

