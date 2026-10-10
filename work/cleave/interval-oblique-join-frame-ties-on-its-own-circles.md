---
id: interval-oblique-join-frame-ties-on-its-own-circles
kind: issue
title: The join order's oblique frame still ties crossings on one great circle of plane normals per face direction (det(n, m, r) = 0); some poses the x-axis frame answered now refuse at Interval
status: open
opened: 2026-10-10
priority: P3
cost: M
---


Residual of `interval-steep-cut-through-cylinder-caps-refuses-order-escalated`
(PR 4224), measured by that PR's review.

**The condition.** `splitting::order::in_plane_frame` takes, off an
axis plane, `u` from the first oblique `SCHEDULE` member `r` that
projects definitely (for most normals `r = (0.5, 0.25, 1)`), and
`v = n × u`. Two crossings on one planar face of normal `m` lie along
`n × m`; that is parallel to `v` — so they share `u` in truth, and
`split_join_order_u` straddles at `Interval` — exactly where
`det(n, m, r) = 0`. One great circle of plane normals per face
direction:

| faces of normal | tie circle under `r = (0.5, 0.25, 1)` | under the old x-axis frame |
|---|---|---|
| z (cylinder caps, box tops) | `n_x = 2n_y` | `n_y = 0` |
| y | `n_z = 2n_x` | `n_z = 0` |
| x | `n_z = 4n_y` | every normal (`det(n, x, x) = 0`: the frame's `v` is `n × x` itself) |

**Measured enumeration** (the review, at `Interval`, the 62 integer
normals with components in −2..2, through the body's centre):

- the unit cylinder answers 42 on the base (`dfcd7f3504`'s x-axis
  frame) and 53 on PR 4224;
- the box answers 2 on the base and 48 on PR 4224;
- **regressions** — the cylinder at `n = (2, 1, ±1)`, `(2, 1, ±2)` and
  `(2, 1, 1.5)` answered on the base and refuses `split_join_order_u`
  on PR 4224: all on the z-face circle `n_x = 2n_y`.

Witness for this row: the unit cylinder (`z ∈ [0, 1]`) cut through
`(0, 0, 0.5)` with normal `(2, 1, 1)`, at `Interval`. (Not pinned as a
test: a pin would assert the defect.)

**The frame is discontinuous at the axis planes** (the review's
NOTE-1). `n = (1, 1e−17, 0)` is not an axis plane, so it takes the
oblique frame and its cut answers at `Interval`; `(1, 0, 0)` takes the
exact x/y frame and refuses (the axis-plane row,
`interval-axis-plane-cut-along-a-cylinder-refuses-its-rims-tie`). The
`f64` visiting order also jumps under that sub-ulp tilt — and with it
`SectionFace` completion indices (the `emit` row
`section-face-and-hole-rim-are-ordinals-over-their-group` has the
evidence).

**Absolute orientation.** The tie circles are fixed in world axes, not
in the body: a rigid motion of the body moves which of its cuts refuse.

**Candidate shapes, not weighed:**

- *A ladder over `SCHEDULE`* until every comparison of the sort
  decides, as the in-plane ray parity already walks the table. Lane
  agreement: at `f64` the exact band never straddles, so the `f64`
  lane always stops at the first member; an `Interval` lane that steps
  on would sort in a different order from the `f64` run it replays.
  Agreement needs both lanes to step on the same verdicts — e.g. a
  near-tie read at the run band at both, which brings back the
  ambiguity window DR-30 rejected (refusals where `f64` answered)
  unless the ladder steps rather than refuses in it.
- *An intrinsic or topological tie-break*: where `u` ties, order by
  something both lanes hold exactly (the null edge's discovery index,
  the crossed edge's key, `v`). The `f64` lane never sees an exact tie
  of distinct computed values, so it would have to adopt the same
  near-tie reading, with the same window cost. The other extreme is a
  fully topological order: no geometric decision, agreement by
  construction. Cost: what still reads the sweep order — the book's
  rule for faces with at most two crossings and for fixed partners an
  earlier chord parted, and `SectionFace` completion indices — has to
  be shown order-free or re-pointed.
