---
id: line-edge-crossing-a-sphere-face-has-no-root-lane
kind: issue
title: A line edge straddling a sphere face has no root lane in the reduction (lily wall probe 12 stops here)
status: closed
pr: 3659
branch: reach/sphere-union-sphere
opened: 2026-10-01
refs: [full-period-wall-has-no-containment-verdict, sphere-union-sphere-refuses-though-the-section-is-closed-form]
closed: 2026-10-01
---

Found by the `reach-fullperiod` lane once the full-turn cylinder band
got a face-door verdict: `demos/tour/src/lily.rs` wall probe 12 (corm ∪
stem foot, declared cylindrical `Rest`) no longer stops at the corm's
bore wall, and stops one pair later on a different door.

## Measured

```
CurvedPierceUnsupported { operand: B, face: FaceKey(5v1), edge: EdgeKey(4v1), .. }
```

- edge `4v1` of operand B (the foot): a LINE, the foot wall's seam
  ruling at azimuth 0. As AUTHORED (`lily.rs`, `foot(FOOT_BOTTOM_Z, 0.0,
  STEM_R, tol)`) it runs `z ∈ [-0.92, 0]`, and both of those ends are
  outside the sphere. The pair the refusal names is in the reduction's
  WORKING COPY of B, measured at the arm (`curved_face_arm`'s `pu`,
  `pv`): earlier pairs split the ruling where it meets the corm's bore
  rims, and the fragment that keeps the key runs
  `(0.06, 0, -0.92) → (0.06, 0, -0.72)`. Its lower end is outside the
  sphere (distance `≈ 0.424` from the centre), its upper end inside
  (`≈ 0.228`);
- face `5v1` of operand A (the corm): its SPHERE zone, centre
  `(0, 0, -0.5)`, radius `0.3`;
- the fragment's endpoints are on opposite sides of the sphere, so the
  pair takes
  the straddle arm of `reduce::curved_face_arm`
  (`(Positive, Negative) | (Negative, Positive)`), and `wall_crossing`
  answers `SpanVerdict::Unsettled` because `line_wall_root_count` has
  no sphere arm: `_ => return Ok(Err(SpanVerdict::Unsettled))`, with
  the comment "A sphere face: no root lane here". The crossing is at
  `z = -0.5 - √(0.09 - 0.0036) ≈ -0.794`.

The face door can place the landing point (a full-turn sphere zone is
served by `contain::sphere_face_containment`), so what is missing is the
root lane alone: a line × sphere quadratic, certified the way
`solid_contain::line_wall_roots` certifies a line × cylinder one.

## Note

`sphere-union-sphere-refuses-though-the-section-is-closed-form` is the
CIRCLE × sphere pierce; this is the LINE × sphere one on the same arm.
Whoever builds one should look at the other.

## Built (2026-10-01, `reach-snowman`)

`line_wall_root_count` has a sphere arm: the ray lane's quadratic,
factored out of `solid_contain::cast_ray` as
`solid_contain::line_sphere_roots` so the ray and the edge sweep solve
the same one (`bool_ray_sphere_disc`, metered `disc/2r`). Lily wall
probe 12 re-measured: the fragment's crossing is found at `t = 0.1261`
(`z ≈ -0.794`, the predicted point) and the pair pierces. The wall does
NOT flip: the sweep stops one pair later at
`CurvedPierceUnsupported { operand: B, face: FaceKey(3v1), edge: EdgeKey(5v1) }`
— the foot's seam ruling at azimuth 120° (`z ∈ [-0.92, 0]`) lying ON
the corm's full-turn bore wall, declared-covered, both ends past the
bore's height window — which is
`full-turn-bore-rest-mate-does-not-union`'s door, not this one.

A public-op row reaches the lane too: a square bar poking out of a ball
(`crates/sweep/tests/snowman.rs`,
`a_bar_through_a_ball_crosses_the_sphere`) refused at this door before
and now stops at `CurvedSectorSideUnsupported`.

## Closed (2026-10-01, PR 3659)

The line × sphere root lane exists: `solid_contain::line_sphere_roots`,
shared by the ray lane and the crossing layer, and general in the
line's direction. Lily wall probe 12 now pierces at z ≈ −0.794, then
stops one door later, at `full-turn-bore-rest-mate-does-not-union`.
