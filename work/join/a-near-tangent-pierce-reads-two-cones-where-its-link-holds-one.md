---
id: a-near-tangent-pierce-reads-two-cones-where-its-link-holds-one
kind: issue
title: At a 1e-7 tilt a near-tangent pierce's ∩ reads two cones at the pierce point where its link holds one: the split leaves two vertices and two solids touching there
status: open
opened: 2026-10-06
priority: P2
cost: M
refs: [boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm, two-copies-of-a-pierce-carry-edges-that-run-within-the-band]
branch: join/near-tangent-two-cones
---

## What

Found by PR 4139's review r1 (NOTE-2, its `nt` set: the four reflex
corners against a cube whose near face is tilted 1e-3 to 1e-7 off one
of the corner's edges). Measured on that PR's head, main `897a2c24` vs
head, release, the vertices at the pierce point against the exact
link's cones read from the operands' convex pieces
(`crates/sweep/tests/common/pinch_cones.rs`'s counter).

At tilt 1e-7, on 37 lines (21 poses, face placement; 32 ∩ and 5 ∖),
the link holds one cone, main holds one vertex and head two:
- the two come from `zip::split_cones`, whose `σ_B ∘ σ_A` reading of
  the operands' runs gives two cones: with the split disabled the
  witness refuses `ZipCorrespondence` ("a cone the seams meet twice
  fuses a vertex to itself");
- after the split the shells part, and the witness ∩ is two solids
  touching at the point where main's is one;
- the edges leaving the two vertices part by 0.26 to 2.08 rad: no
  sub-band neck runs between them;
- the volume is the oracle's to 1e-9 and the body meshes, on both;
  28 lines are `SOUND` on both, and 9 fail tier 3′ on both, census
  escalated within the band (`pm_census_ee_span`, margin 2.2e-9 on
  the witness).

In the exact geometry a sliver about 1e-7 rad wide along the
near-tangent edge joins the two halves into one cone. The kernel's
operand topology at the point has no such join. Unknown: whether a
decision read the near-tangent edge within the band at a short lever
(`boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm`),
or the operands' fan at the point is read wrong.

**Witness.** `w345 nt e0 a0 d1e-7 face`, `xy I` and `yx I`: review
r1's probe, `R1_PICK="w345 nt e0 a0 d1e-7 face" cargo run -p sweep
--release --example r1_4139_probes nt` (branch
`join/pinch-one-vertex-per-cone-review-r1`).

## Owed

Read the operands' runs at the witness's pierce point against the
link, and settle which decision drops the sliver: if a band read,
whether the band's reading should hold the two halves as one cone;
if not, fix the reading.
