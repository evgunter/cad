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

