---
id: a-mate-on-a-placed-copys-cap-over-a-widened-tilt-cannot-mint-its-chart
kind: issue
title: A mate on a copy's cap one rung above a widened tilt cannot mint its chart's carried direction at 5e-2: pose_unit_norm encloses [0, 3.76], on both lanes
status: open
opened: 2026-11-03
priority: P3
cost: M
---

Found by FRAME's re-measure of
`work/frame/a-widened-derived-placement-normalises-a-straddling-newell-sum`
(branch `frame/newell-remeasure`, on `origin/main` at `0f01217666`), by
execution with a scratch probe that is not committed.

## The document

R2's `tiltV stacked-2` restated in today's vocabulary (its builder in
`m10_derived_frame_tilted_interval` went with `Datum::FaceFrame`):

- `t = 0.25 ± half` (a Scalar document parameter);
- an exact unit cube `C0`; the tilted pose is `PoseDef::Spin` of `C0`'s
  end-cap plane about its front rim by `atan2(t, 1)`, as
  `m10_derived_frame_tilted_interval::boss_on_tilted(half, true)` builds it;
- `n` rungs: a unit cube (extrude 1.0) whose copy is seated (start cap
  on the plane, `Orientation(PlusX, MinusZ)`, slides and spin at zero)
  on the previous pose, the next pose being that COPY's end-cap plane;
- a half-size square extruded 0.25, seated the same way on the last pose.

`n = 0` is `boss_on_tilted(half, true)` itself.

## Measured

Every body is extruded once over exact points; only the placements
carry `t`.

| rungs `n` | `half = 1e-3` | `half = 1e-2` | `half = 5e-2` |
| --- | --- | --- | --- |
| 0 | both lanes certify | both certify | both certify |
| 1 | both certify | both certify | **both refuse** at the boss's placement |
| 2 | both certify | both certify | **both refuse** at the second rung's mate (onto the first rung's copy's cap); the boss is poisoned through it |
| 3 | not run | plain `Interval` refuses at the boss's placement; `Sym<Interval>` (shipped) certifies | both refuse at the second rung's mate |

The refusal is the same on every refusing row, and on `Sym<Interval>`
under `SymRules::shipped()` it is the same as on plain `Interval`:

```
a value's chart cannot mint the direction its least turn carries as a unit.
Recourse: read the mate's poses off geometry whose directions are unit
— PoseDegenerate { construction: Chart }
blocked pose_unit_norm Indeterminate enclosure (0.0, 3.759348532782267)
```

The site is `crates/editor-core/src/mate/value.rs`, `Chart::charted`,
the `unit` closure (`UnitVec3::new(v, topo::query::POSE_UNIT_NORM, band)`)
over a direction the least turn carries. Its true norm is 1 at every
`t` in the box. The enclosure reaches zero only through dependency
widening: a rotation of ±0.047 rad (`atan2` over `t ∈ [0.20, 0.30]`)
becomes a carried axis whose components span about ±2, because the
mate reads its target off a copy's cap that is already a widened
map's image and composes another turn over it. The refusal is
typed, which is the right failure, but every mate onto a copy's cap
over the tilt refuses at `5e-2`, and with three rungs the plain lane
refuses at `1e-2` too. The symbolic tier does not prove it either.

## Related, not the same

- `a-pin-read-off-a-copys-geometry-carries-its-width-into-the-copy`
  (this slate) is the same shape, with ε-wide geometry read by
  `pin_at`/`place_at`. This row is about a document parameter's width
  read by a mate on a placed copy's cap.
- `work/topo/a-cap-normal-reads-the-height-s-width` widens a cap's
  stored normal through the height. Here no body reads `t`.

## What would close it

The chart carries the target pose's frame, which the previous placement
already certified as a unit frame, without re-deriving it from the
copy's widened geometry. Or the interval solve shares the placement's
enclosure through the composition. The table above is the acceptance
ladder: `n = 1, 2` certify at `5e-2` on both lanes.
