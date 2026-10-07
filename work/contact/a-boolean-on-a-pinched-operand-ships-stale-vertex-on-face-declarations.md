---
id: a-boolean-on-a-pinched-operand-ships-stale-vertex-on-face-declarations
kind: issue
title: A boolean on a pinched operand ships VertexOnFace declarations tier 3′ reads as stale
status: parked
priority: P2
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
refs: [the-cone-vertices-at-a-pinch-share-the-pierce-points-key, a-pinch-no-kept-face-can-cross-refuses]
opened: 2026-10-07
---

## What

On r2's pinched-operand battery (PR 4139's review r2), the boolean of a
pinched operand P with a cube C builds bodies whose own contact
declarations include `VertexOnFace` entries that tier 3′ refuses as
`StaleContactDeclaration`. Volume, tier 2, the certificate and the
legal-operand check all pass, and every one of these bodies meshes.
P is a cube minus a smaller box sharing its corner, so P is pinched at
the origin, and C's near face holds that point.

Once PR 4207 put a pinch's cone vertices on one point key, this is the
only tier-3′ finding left on the battery. Before that PR, these lines
also failed on `UndeclaredContact { VertexVertex }` at the pinch. The
stale declarations are the same on both sides, line for line.

## Measured

`join_pinch_cones_r2_probes::r2_pinched_operand_battery` (720 lines),
release, main `3e9d1a96` and PR 4207's head `9d7b9228`:

- 120 lines are `OK BAD` on tier 3′ alone, all with
  `StaleContactDeclaration { VertexOnFace }`: `pc U` 36, `cp U` 37,
  `cp S` 32, `pc S` 6, `pc I` 5, `cp I` 4. They span poses
  `i` = 1, 5, 6, 7, 9, 10, 12, 13, 14, 18, 19, 20, 22, 25, 28, 32,
  35, 36, 37, 43, 44, 47, 48, 50, 51, 52, 53, 55, 56, 57, at both
  ψ = 0 and ψ = 1.1.
- Main holds stale declarations on the same 120 lines, with the same
  count on each.
- Examples:
  - `i=1 ψ=0 cp S`: two declarations on face `2v1`;
  - `i=9 ψ=0 pc U`: one, `VertexOnFace { vertex: 22v1, face: 21v5 }`.

## Repro

The battery file was a review probe, kept outside the tree
(`join_pinch_cones_r2_probes.rs`, PR 4139's review r2): add it under
`crates/sweep/tests/` with a `mod` line in `all.rs`, limit its loop to
pose `i = 1`, and print `validate_pseudomanifold`'s error in
`common/differential.rs`'s `outcome`. Then:

    cargo test --release -p sweep --test all -- --ignored --exact \
      join_pinch_cones_r2_probes::r2_pinched_operand_battery --nocapture

## Not yet known

Where the declarations come from. Either the boolean's declared-contact
pass names a vertex-on-face that the pinch's cone split or the zips
then make into a corner of that face, or the declaration is right and
tier 3′'s staleness test misreads a vertex on the face's own boundary.
This is held on D10, which settles how declared contact is said.

