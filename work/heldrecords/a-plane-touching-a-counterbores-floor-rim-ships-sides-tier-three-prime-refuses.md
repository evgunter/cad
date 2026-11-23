---
id: a-plane-touching-a-counterbores-floor-rim-ships-sides-tier-three-prime-refuses
kind: issue
title: a plane touching a counterbore's floor rim ships sides tier 3′ refuses with unrecorded vertex-on-face contacts
status: closed
opened: 2026-10-29
priority: P1
cost: M
branch: heldrecords/nesting
pr: 7436
closed: 2026-11-23
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

## Measured: face 11 is on the cut plane (restread, branch `restread/self-touch-coincident-sheets`)

At `a = 0.3`, `t = 0.05`, lean in, the self-touch sweep's link at the
rim crossing `(0.1330, 0.6, 0.5851)` holds four faces: the counterbore's
step wall (a cylinder corner), the floor (`y = 0.6`, a corner), a face
on the cut plane cornered there (13v1, `sense` false, outward `+n`), and
face 11v1, on the same plane with the other outward normal, holding the
point inside it (the `VertexOnFace` above). Face 11 is a cut-plane face,
not the floor annulus, and it and face 13 hold a common area of the cut
plane at that point: material either side of one sheet, a slit. The
same shape holds at `P` and at the other rim crossing, at both azimuths,
under both normals and in both halves (`census::sweep_self_touch_points`'s
link, dumped). Tier 3′ now refuses those points
`BoundaryCrossesItselfUndecided { why: Degenerate }`, as for any two
planar faces on each other
(`review_6020_seam_probes::a_counterbore_half_with_two_faces_on_each_other_refuses_at_the_point`).
So the defect reads as the split's: a half would carry one cut face notched
at the crossing, not a face across it and a face of the other sense
cancelling it. A record for the `VertexOnFace` contacts would not clear
these halves.

## 2026-11-22 — measured: the same cause; fixed on `heldrecords/nesting`

Dumped at t 0.2, a ∈ {0, 0.3}, both leans, both normals: every half
held one outline (the cylinder's whole ellipse, one edge) and one hole
face (`sense` false, no rings). The hole is the counterbore's ellipse
arc closed by the floor's chord (two line edges where the chord is cut
at `P`), and leaning in, the bore's ellipse through `P` as well, one
loop through `P` twice. `nest` left it unplaced because the chord's
line crosses the outer ellipse's carrier (`line_clears_conic`), which
is the elliptic row's cause: face 11 in the earlier measurement is the
outline's face, and face 13 the hole's. The contacts tier 3′ reported
are the hole's vertices and edges lying in the outline's face.

With the edge reading of `a-hole-in-an-elliptic-section-keeps-a-face-of-its-own-that-tier-3-prime-refuses`
the hole nests as a ring, and each half carries one cut face.
`review_6020_seam_probes::a_plane_touching_a_counterbores_floor_rim_splits_at_tier_three_prime`
pins tiers 1, 3 and 3′ (no records) and the quadrature's volume on each
half at t 0.05 and 0.2, a 0 and 0.3, both leans and both normals. Every
pose builds at ε 1e-9 and 1e-12. At ε 1e-6 the pose leaning 0.05 in at
a 0.3 refuses under both normals, upstream of the nesting, with
`Reduce(SliverSector { .. "enters_material" })`. Its margin is −9.9e-6,
inside that ε's escalation band (1e-5): a sector about 1e-5 wide at the
bore rim, which that band does not resolve, so the refusal is right
there. The row pins it as that typed refusal at that pose at ε 1e-6
alone, and requires every pose to build at every other ε; it refuses
any other. It replaces
`a_counterbore_half_with_two_faces_on_each_other_refuses_at_the_point`,
whose two faces no longer exist; the census's `Degenerate` reading of
two planar faces on each other keeps its own rows in `link` and
`two_wedge_ends_on_one_line`.

## Closed (2026-11-23, HELDRECORDS orchestrator)

Closed by PR 7436 (`c51d65be18`). The split nests a hole as a ring of its outline: `section_loops::edges_clear` reads `geom_brep::curve_pair::meet`, and an unplaced hole refuses typed `UnplacedHole`. The "sound by cancellation" allowance is retired. Pinned at tier 3′ across three ε. Residue filed: `nest-reads-an-undecided-pair-of-enclosing-outlines-as-a-contradiction` (P2).
