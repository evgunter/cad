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

## Measured

On `fb8c7cbb` (PR 4207's head), release, branch `join/near-tangent-two-cones`.

**The link holds two cones, not one.** At `w345 nt e0 a0 d1e-7 face`,
the cube's face plane crosses the horizon of `v`'s link 5.8e-6° (1.0e-7
rad) before the wedge's edge at 345°, on the in side. So the ∩ holds a
sliver between the wedge's top face, its 345° face and the cube's
face. The sliver is about 2.2e-7 wide at the edge's far end and 1.06e-6
deep. It touches the main lump only at `v`, because the 15° gap between
345° and 360° separates the two everywhere else. Its cell in the link is
a triangle whose three arrangement vertices lie within about 1e-7 of
each other.

Both counters sample each arrangement vertex's cells at a fixed 1e-5
step: review r1's probe, and `sweep/tests/common/pinch_cones.rs`
`cones_at`. A 1e-5 step lands past the sliver's cell, so they read one
cone. With the step at 1e-9, review r1's counter reads `c2` for ∩ in
both orders, which matches the kernel's two vertices.

The kernel's second solid is that sliver. Its four vertices are:
- `v` itself;
- `(2, −0.535898384862, 1)`, the wedge's 345° top corner;
- `(2, −0.535898603421, 1)`;
- `(2, −0.535898384862, 0.999998938670)`.

**The decision that "drops the sliver" is the counter's, not the
kernel's.** None of the three candidates is at fault:
- no band read (the sliver is 200 bands wide);
- the operands' fan;
- `σ_B∘σ_A`.

`split_cones` reads two cones because there are two.

**Review r1's `nt` set, step 1e-5 against 1e-9, one kernel.** 7 200
lines each. 151 lines change their cone reading, all at tilt 1e-7, and
every one of the 115 `CONEMISMATCH` lines among them agrees with the
kernel at 1e-9:
- 45 `c1 v2` → `c2 v2`: this row's class, 32 face ∩, 7 face ∖, 6 edge ∩;
- 48 `c2 v1` → `c1 v1`;
- 22 `c0 v1` → `c1 v1`;
- 9 `nm` → counted (8 agree with the kernel, 1 refused);
- 21 change only the free count, and 6 refused lines their cones.

No line gains a mismatch. The residue that does not move is the probe's
own counter, which has no lunes and matches points within 1e-9. It is
the same at every tilt.

**Ported (`join_pierce_runs_sweep.rs` `near_tangent_battery`, on the
repo counter).** The fixed `cones_at` (`round_vertex`: the step shrinks
until every sample is on `v`'s side of each circle missing `v`) is
measured against the old one on one kernel:
- 71 face lines move from a finding to "one vertex per cone", all at
  1e-7: 32 `2 vertices for 1 cone` (this row), 7 the same with a lune
  freed, 14 `1 for 0`, 10 `1 for 2`, and 8 "not counted";
- no other line moves;
- head: 0 findings over the face placement's 3 299 built lines.

Outcomes by tilt, unchanged by the counter:

| tilt | SOUND | BAD | refused |
|---|---|---|---|
| 1e-3 | 1 415 | 0 | 25 |
| −1e-3 | 1 394 | 0 | 46 |
| 1e-5 | 1 271 | 0 | 169 |
| −1e-5 | 1 202 | 0 | 238 |
| 1e-7 | 1 315 | 70 | 55 |

The 70 BAD lines fail tier 3′ alone (`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`).

**Not read: the edge placement's points.** In the edge placement, the
cube-first ops hold `v` as the cube edge's split, 5.6e-17 to 2.5e-15 off
`v`. `vertices_at`'s exact match reads none there (324 lines), and a
1e-9 match reads every line one vertex per cone. So the battery reads
cones in the face placement only, as `the_sweep_subset_ships_no_bad_body`
does.

**Pinned:** `a_near_tangent_sliver_is_a_cone_of_its_own` (the witness,
both orders). It is red on the old counter: `cones_at` reads `(1, 0)`.
