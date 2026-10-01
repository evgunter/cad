---
id: line-edge-crossing-a-sphere-face-has-no-root-lane
kind: issue
title: A line edge straddling a sphere face has no root lane in the reduction (lily wall probe 12 stops here)
status: open
opened: 2026-10-01
refs: [full-period-wall-has-no-containment-verdict, sphere-union-sphere-refuses-though-the-section-is-closed-form]
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
  ruling at azimuth 0, `(0.06, 0, -0.92) → (0.06, 0, -0.72)`;
- face `5v1` of operand A (the corm): its SPHERE zone, centre
  `(0, 0, -0.5)`, radius `0.3`;
- the endpoints are on opposite sides of the sphere, so the pair takes
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
