---
id: a-torus-touch-at-a-definite-non-elliptic-point-refuses-r-tan
kind: issue
title: A torus touch that is isolated but not at an elliptic point refuses R-tan
status: open
opened: 2026-10-06
priority: P1
cost: M
design: true
refs: [torus-touch-off-the-faces-refuses-at-the-section-pass, a-torus-touch-along-a-whole-parallel-refuses-r-tan]
---


Found by `torus-touch-off-the-faces-refuses-at-the-section-pass`
(branch `reach/torus-touch-off-faces`). Its brief ruled that a tangency
on the tube's inner (hyperbolic) half keeps R-tan, and the branch
reads a torus touch only at an elliptic point. That is conservative,
so sound, but it refuses touches that are isolated.

## Measured (that branch)

- `crates/sweep/tests/torus_touch_off_faces.rs`
  `a_touch_on_the_inner_equator_refuses_off_the_faces`: a ball of
  radius 1 about `(0, 0, 0.5)`, in the donut's hole, touches the inner
  equator at `(0, 0, 1.5)` from outside the tube. Its cap about the
  touch is taken off, so the bodies are apart, and every op refuses
  `FallbackExtentUnsupported` (tangent).
- `section_cert_rows.rs` `torus_and_ball_every_class`: a ball of radius
  0.5 about `(1, 0, 0)` is `Tangent("section_torus_sphere_near_tube")`.
- `torus_tangencies_off_an_elliptic_extreme_are_not_touches`: a wall
  in the hole on the inner equator is
  `Tangent("section_torus_offset_wall_far_inner")`.

## The shape

The distance from the ball's centre has its strict minimum over the
torus at that inner-equator point (critical values `d − r`, `d + r`,
`d' ∓ r`; gap `min(2r, 2s)`). The relative second fundamental form,
`II_torus + I/ρ`, is definite there because `ρ < R − r`. So the contact
is one point, and nearby poses cut one small disc about it. The plane
on the inner equator is different: it is a genuine pinch (a figure
eight), and a level of the height there is a saddle, not an extreme.

The same holds for a wall in the hole touching the inner equator. It
holds for a ball outside the tube touching it near the top parallel,
where the elliptic margin is undecided but the relative form stays
definite. And it holds for a ball inside the tube touching its inner
half.

## The question

Should the torus arms read a touch at any strict extreme of the
partner's level function (the rule `torus_sphere_touch` already
decides, less its elliptic margin) rather than only at an elliptic
point? Or, more widely, wherever the connected-region reading of
`a-torus-touch-along-a-whole-parallel-refuses-r-tan` holds?
