---
id: circle-recognition-normalizes-its-plane-normal-with-no-mint
kind: issue
title: recognize_curve's circle arm normalizes its three-sample normal with format gates only and mints no witness
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Found by the sweep of
`recognize-normalizes-without-a-length-decision-and-cannot-mint-the-witness`.

`crates/step-import/src/recognize_curve.rs`, the circle arm
(`try_circle`): the plane through the three samples is
`(s1 − s0).cross(s2 − s0)`, normalized by `checked_direction` — the
format gates only, no decision — and its `u_ref` likewise. The sagitta
read before it is a decision on a different quantity. The surface
recognizer's plane arm now mints its normal and roll as one
`RigidMotion::from_axis_and_reference` at `Band::least()` (an
estimator's choice, which its certificate decides); the circle arm is
the same shape and could take the same mint, so the circle's
`Frame` comes out of a witness rather than two bare normalizes.

The cylinder arm of `recognize.rs` has the same shape (`axis_sum`,
`chord`, `radial` through `checked_direction`, each after or before a
file-band read); it was left out of the plane's unit because the
torus/sphere/cone unit was editing beside it.
