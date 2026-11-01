---
id: a-plane-touching-a-counterbores-floor-rim-ships-sides-tier-three-prime-refuses
kind: issue
title: a plane touching a counterbore's floor rim ships sides tier 3′ refuses with unrecorded vertex-on-face contacts
status: open
opened: 2026-10-29
priority: P1
cost: M
---


Found by the seam-pose sibling sweep of
`a-plane-touching-a-bore-rim-at-its-seam-refuses-a-stated-interval-zero-in-the-join`
(branch `splitdoor/bore-seam-join`), at ε 1e-9 and 1e-12.

## What

The counterbored tube of `split_across_a_revolve_seam.rs` (`COUNTERBORE`, revolved a full turn
about `y`: a bore of r 0.3 under a counterbore of r 0.6 from y 0.6) split by the plane through the
bore's floor rim point `P = (0.3 cos a, 0.6, 0.3 sin a)` with normal
`s·(ŷ cos t − (cos a, 0, sin a) sin t)`: the floor's trace is tangent to the bore at `P`, the pose
the tube's top cap builds clean (`a_plane_touching_a_bore_rim_splits_alike_under_either_normal`).

At t 0.2, a ∈ {0, 0.3}, both normals, both halves build and pass tiers 1 and 3, and both fail
tier 3′ (`validate_pseudomanifold` with no contact records):

- a = 0: `UnrecordedContact { VertexOnFace { vertex 2v1, face 11v1 } }`, witness
  `(0.3, 0.6, 0.0)` (`P`, margin 0.0), and the two vertices where the floor's trace meets the
  floor's outer rim, witnesses `(0.3, 0.6, ±0.5196)` (margins ~1e-17).
- a = 0.3: `VertexOnFace` on face 12v1 at `(0.1330, 0.6, 0.5851)`, `(0.4402, 0.6, −0.4078)` (the
  outer-rim crossings) and `(0.2866, 0.6, 0.0887)` (`P`), margins ~1e-17.

PR 6020's review (`review_6020_seam_probes.rs`, `review_6020_seam_poses`) reproduced it at
every azimuth and found the `out` lean (the plane leaning toward the axis, so the bore wall is
touched at `P` from outside its face) failing the same way; both halves' volumes are right.

The tube's top cap, which differs only in having no counterbore wall above the rim's annulus,
passes tier 3′ at every azimuth. Which face 11/12 is, and why the contact is undeclared, is
unmeasured. The pose is not seam-specific: off the seam it fails alike. At the seam under
`s = −1` it refused `Join(Euler(Certification { StatedInterval Zero }))` on main before that
branch, the same cause as the tube's seam pose.

## Reproduce

`touching_the_bore_rim` in `split_across_a_revolve_seam.rs` with `COUNTERBORE`, radius 0.3 and
rim height 0.6 in place of the tube's 0.5 and 1.0; read `validate_pseudomanifold` on each half.

## Moved to HELDRECORDS (2026-10-30, SPLITDOOR orchestrator)

Every finding here is `UnrecordedContact { VertexOnFace }` at tier 3′. Both halves' volumes are right, and tiers 1 and 3 pass. These are the split's own touches, which no record covers: the floor's trace touches the bore rim at P and meets the floor's outer rim. That is HELDRECORDS' charge (`the-splits-own-pinch-and-cut-copies-take-two-point-keys-and-no-record`, and the split's tier-3′ gate row), so the row moves there by `git mv`, id and body unchanged. Whether face 11/12 is the counterbore floor annulus is still unmeasured.
