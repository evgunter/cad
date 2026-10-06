---
id: a-conic-touching-a-torus-off-its-face-passes-the-pierce-and-refuses-at-the-section-pass
kind: issue
title: A circle or ellipse edge touching a torus off its face passes the pierce, then refuses FallbackExtentUnsupported at the section pass
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [edge-tangent-to-a-curved-carrier-off-the-face-refuses-at-the-pierce, torus-touch-off-the-faces-refuses-at-the-section-pass]
---

Found by the review of PR 4128 (NOTE-1). The off-face reading
(`crates/topo/src/boolean/carrier_touch.rs:84`, `off_face`, called at
`crates/topo/src/boolean/reduce.rs:3175`) clears a circle or an
ellipse edge that touches a torus's carrier where the face is not, so
the pair passes the pierce. The no-crossings section pass then has no
arm for the pair and refuses `FallbackExtentUnsupported` with
`Refusal::Reach` (`crates/topo/src/boolean/section_cert.rs:327`). No
wrong body ships, but the pierce fix is not observable for these
pairs: they refused at the pierce before PR 4128, and they refuse one
pass later now.

## Measured (head of PR 4128, after its last fix pass)

The reviewer's poses, on `analysis/reach-review/4128`:

- `probes/e2e_pierce_tangent.rs:299` (`circle_torus_rows`): a coin of
  radius 0.2 whose rim circle touches the 270° donut (`R = 2`,
  `r = 1/2`) on its outer equator in the mouth.
- `probes/e2e_pierce_tangent.rs:314` (`ellipse_torus_rows`): an
  obliquely cut rod placed so that its cut ellipse's support point
  touches the same donut in the mouth.

Every op, both orders, at ε 1e-9:

```
FallbackExtentUnsupported { operand: A, face: FaceKey(3v1), what: "a curved face's box overlaps a face of the other solid, no crossing layer saw an event, and the pair has no section classification (an oblique or non-coaxial pose, a cone or a spline face) ..." }
```

The reviewer saw the same at all three ε. With the kernel files
reverted to PR 4128's merge base, both poses refuse
`CurvedPierceUnsupported`, so the new path is reached. The on-face
controls (the touch at azimuth 135°) refuse at the pierce.

## The shape

The section pass needs a torus × plane or torus × cylinder arm at a
non-coaxial pose to prove that the coin's or rod's faces do not meet
the torus face in a closed loop interior to both. Whether the
off-face touch can stand in for that proof (the touch's ball is
already certified clear of the face) is the design question.
