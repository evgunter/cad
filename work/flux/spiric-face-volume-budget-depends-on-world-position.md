---
id: spiric-face-volume-budget-depends-on-world-position
kind: issue
title: A spiric-bounded face's certified volume spends its quadrature budget in a turned frame 990 m out, inside the box
status: open
opened: 2026-10-14
priority: P2
cost: M
---

(Found by a designer weighing
`work/germ/plane-torus-section-gaps-decide-far-coordinates-uncharged.md`,
reproduced on GERM's `germ/pt-section-gaps`.)

## What

The certified volume lane's budget depends on where a body sits in
the world, inside D4 ¶4's 1 km session box.

The donut `R = 2`, `r = 0.5` about `y` (`sweep` tests
`germ_plane_torus_frame::torus(2.0, 0.5)`) against the half-space
`x ≥ 0.3` (`germ_plane_torus_chord::half`), whose plane cuts two
spiric ovals. Both are moved by `topo::transform_rigid` under
`translation((8, −3, 5)·s) * rotation_about_axis(origin,
(0.3, −0.5, 0.8)/‖·‖, 0.7)`. At ε `1e-12`, through
`germ_plane_torus_frame::six`:

| pose | T ∪ P | P ∪ T | T ∩ P | P ∩ T | T ∖ P | P ∖ T |
|---|---|---|---|---|---|---|
| s = 0, 1, 10, 50, turned | built | built | built | built | built | built |
| s = 100 (\|o\| ≈ 990 m), turned | built | built | refused | refused | refused | built |
| s = 0 … 100, translated only | built | built | built | built | built | built |

Each refusal is `VolumeUnmeasured { source: Face { source:
QuadratureBudget { width_len ≈ 3.59e-8, target_len 1.024e-9,
rounds 13 } } }`.

## Where

- Raised by `geom_brep::props::spiric::spiric_face_rounds`
  (`crates/geom-brep/src/props/spiric.rs`, its `round_exit` call),
  with the target `QUAD_TARGET_LEN_FACTOR·ε`
  (`crates/geom-brep/src/props/quad.rs`, `QUAD_TARGET_LEN_FACTOR`).
- A lead, not yet a measured cause: `face_bounds` in the same file
  takes a plane face's flux as `dot(face.lever, va)`, with `lever` a
  point of the face less the world origin. The vector area's
  enclosure width is then multiplied by about 990 m. On a translated
  pose with an axis-aligned normal, the vector area's off-normal
  components may enclose zero exactly, which would explain why only
  the turned pose refuses. Measure that before you fix it
  (`memories/refusal-text-is-not-cause.md`).

## Done when

The certified volume of a body in the box converges, or refuses,
the same way at any rigid placement in the box. A row pins the pose
above at s = 100 and asserts that all six ops build.
