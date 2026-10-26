---
id: a-difference-leaving-a-cusp-along-a-wall-ruling-refuses-sector-straight
kind: issue
title: A difference whose result has a zero-angle cusp along a cylinder wall's ruling refuses Escalated at sector_straight; does a cusp-edged result build or refuse typed?
status: closed
opened: 2026-10-15
priority: P3
cost: M
refs: [subtracting-a-slit-tool-resting-its-floor-on-a-face-escalates-sector-straight]
closed: 2026-10-26
---

## What

Found while probing PR 5218 (the curved half of
`a-vertex-minted-on-an-edge-lying-on-a-curved-face-is-unread`), and
measured more widely by that PR's review.

**Witness.** The line body is the cylinder r = 1, z ∈ [0, 2], unioned
(cylinder first) with the box x ∈ [1, 1.5], y ∈ [0, 0.5],
z ∈ [0.5, 1.5]. The box's face x = 1 is the wall's tangent plane along
the ruling at (1, 0), and its edge x = 1, y = 0 lies on that ruling. The
tool is the wedge prism with its apex edge on the ruling, sides 0.5 long
at 60° and 120° from +x, z ∈ [0.2, 1.8] (the `wedge` fixture in
`crates/sweep/tests/a_minted_vertex_on_a_curved_face.rs`). The mirror
pose answers the same: box y ∈ [−0.5, 0], tool at −120°..−60°.

- tool ∖ line refuses
  `Escalated { decision: LineWedges, predicate: "sector_straight" }`:
  margin 1.0, rejected `{ sign: Positive, passes: Negative }`;
- line ∖ tool builds at its closed form (6.414761), tier 3 clean;
- ∩ and ∪ build in both orders.

∖ is not order-symmetric, so the two orders building differently is no
defect in itself. What matters is where the refusal sits. tool ∖ line
leaves the wedge's part outside the cylinder and the box, and that part
meets the cylinder's wall along the ruling at a zero angle: the result
has a cusp edge there. The rejected margin, 1.0, is the z-overlap of the
box and the tool along the ruling (z 0.5..1.5). So sector_straight reads
a real fold, not a reading in the band.

## Extent

PR 5218's review measured 6 such cells over its probe poses, 2 of them
refusing the same way on main, so the shape predates PR 5218. With `rod`,
`wedge` and `line_body` of
`crates/sweep/tests/a_minted_vertex_on_a_curved_face.rs`, those two are:

- **A**: line body `rod ∪ wedge(50, 90, 0.5, 1.0)`, tool
  `wedge(60, 120, 0.7, 0.5)`; tool ∖ line refuses at sector_straight
  with margin 0.5;
- **B**: line body `rod ∪ wedge(−90, −50, 0.2, 1.6)`, tool
  `wedge(−120, −60, 0.2, 1.6)`; tool ∖ line refuses at sector_straight
  with margin 1.6.

Each margin is the z-overlap along the ruling. Both are reproduced on
main (`3bdf48c60`) at ε 1e-9, 1e-6 and 1e-12, and pinned by
`a_difference_leaving_a_cusp_along_the_ruling_refuses_sector_straight`
(the decision, the predicate and the rejected margin), so they flip when
fixed. The two
witness cells above refused earlier on main (`JoinDesync`); PR 5218's
reads clear them past the join to here. It is wider than boxes: any tool whose difference
leaves a zero-angle cusp along a ruling of a wall. It is the same shape
as `work/cleave/subtracting-a-slit-tool-resting-its-floor-on-a-face-escalates-sector-straight.md`
(a slit-bearing tool resting its floor on a face, margin 2.5).

## Open question

Does a result with a cusp edge build (a wedge-0 edge, D1's wedge-end
rules) or refuse typed, with a recourse? Today it refuses `Escalated`,
which reads as a band question, and no tolerance answers it.

## 2026-10-19: cells A and B build (CLEAVE, `cleave/gate-refuses-overlapping-wedges`)

The refusal came from ∖'s reverted-line check (PR 5106): it read the
line body's coincident edges along the ruling and then the result's
wedge there, where `sector_straight` rejects the cusp edge's zero angle
(a definite reading, reported as an escalation). That check is folded
into the Boolean's result gate, which reads only lines where two or
more of the result's edges lie, and the result here keeps one cusp edge
on the ruling. So `tool ∖ line` builds in both cells at the closed form
(the cusp sliver under the tool's far side plus the triangle beyond the
piece's far side, times the height), tier 3 clean, its cusp read by
tier 3's wedge-end arm: the pin is now
`a_difference_leaving_a_cusp_along_the_ruling_builds`. Where a cusp or
slit edge does share a line with another edge, the gate refuses typed
(`LineWedgeFault::Unread { why: Degenerate }`, D1: a consumer with no wedge-end answer
refuses at the consumer), not as an in-band escalation. The row's
question, build or refuse, is answered for this shape: it builds.

## Closed (2026-10-26, easy-fixes lane)

The row's own witness builds too. Cells A and B were already pinned. The line body `rod ∪` box `x ∈ [1, 1.5]`, `y ∈ [0, 0.5]`, `z ∈ [0.5, 1.5]` against the tool `wedge(60, 120, 0.2, 1.6)`, and its mirror in `y = 0`, now give `tool ∖ line` = `(triangle − area_in_disc(60, 120))·1.6 − (LEN·sin 60°)²/(2 tan 60°)·1.0`, tier 3 clean. Pinned as `crates/sweep/tests/a_minted_vertex_on_a_curved_face.rs` `a_difference_leaving_a_cusp_beside_a_tangent_box_builds`. Verified: `cargo nextest run -p sweep --test all -E 'test(/a_minted_vertex_on_a_curved_face/)'`, 4 passed at the default ε, at `CAD_TOLERANCE_EPS=1e-6` and at `1e-12`.
