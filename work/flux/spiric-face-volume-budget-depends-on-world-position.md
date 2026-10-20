---
id: spiric-face-volume-budget-depends-on-world-position
kind: issue
title: A spiric-bounded face's certified volume spends its quadrature budget in a turned frame 990 m out, inside the box
status: review
branch: flux/spiric-budget-world-position
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

## Cause, measured

The lead held. At s = 100 turned, the plane face's lever from the world
origin is ~990 m, its vector area's enclosure keeps ~7e-11 of width
per component at round 12, and the flux's width is ~8.6e-8, so
`width_len` reads 3.6e-8 against 1.024e-9. Translated only, the
off-normal components enclose zero exactly and the lever multiplies
nothing.

## Fix

Every face is read about its shell's reference (`topo::props`'
`shell_reference`): the stored point of the vertex the outer loop of
the shell's first face starts at, in the shell's own order, so the lever
is bounded by the shell's diameter wherever the body sits. It is not
bounded by the face's own distance from the origin, and it depends on
the shell's stored face order:
`work/flux/a-quadrature-faces-pad-scales-with-its-shells-size-and-stored-face-order.md`. Every walk
(the whole body, a face list in any order, a solid, a shell, every
resumption) reads a face about the same point, and a body sums its
closed shells. `corner_of`, the re-derivation's centre, is the same
point lifted. Rows: `sweep` tests
`a_spiric_cut_far_off_and_turned_reads_as_at_home` (all six ops build at
s = 100 turned and measure the s = 0 volume; every quadrature face meets
its target at the same round; every pad is its pad at home) and
`topo::props::shell_reference_rows` (a closed-form body's volume is
unchanged in the reals).
