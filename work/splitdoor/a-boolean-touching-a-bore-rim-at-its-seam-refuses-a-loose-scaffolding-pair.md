---
id: a-boolean-touching-a-bore-rim-at-its-seam-refuses-a-loose-scaffolding-pair
kind: issue
title: a Boolean whose face touches a tube's bore rim at the seam vertex refuses JoinDesync on a loose scaffolding pair
status: open
opened: 2026-10-29
priority: P2
cost: M
---


Found by PR 6020's review (branch `splitdoor/bore-seam-join-review`,
`crates/sweep/tests/review_6020_seam_probes.rs` `review_6020_boolean_seam_poses`), at ε 1e-9,
1e-6 and 1e-12, on PR 6020's head and on its merge base alike.

## What

The Boolean twin of the split pose `split_across_a_revolve_seam.rs`
`a_plane_touching_a_bore_rim_at_its_seam_splits_alike_under_either_normal`: the tube
(`TUBE` revolved a full turn about `y`, bore r 0.5) against a cube of side 10 whose face lies in
the plane through the bore's top rim point `P = (0.5, 1, 0)` (azimuth 0, the bore rim's seam
vertex), normal `ŷ cos t − x̂ sin t` at t 0.2 (the `in` lean: the face's trace tangent to the bore
at `P`), the cube on either side (both `s`). It refuses

`JoinDesync { what: "derived ring role order separates a loose scaffolding pair" }`

from `boolean/join.rs` `ring_order`. The `out` and `across` seam poses and the poses at azimuth 0.3
build with the quadrature's volumes and pass tier 3′.

Distinct from `work/secttail/a-plane-tangent-to-a-rim-at-its-wrap-edges-vertex-has-no-join-arm`,
which refuses otherwise. The split's chord at the same pose (two copies of `P` joined by a whole
turn, PR 6020) is unreached here: the refusal comes first. Why the role order separates the pair
is unmeasured.

## Reproduce

`review_6020_boolean_seam_poses` logs the pose's refusal (`in`, azimuth 0, both `s`).
