---
id: lever-arms-read-as-f64-bounds-still-enter-lever-t
kind: issue
title: Lever arms read as f64 bounds still enter Lever<T>, where a door's other callers lever at a T arm
status: open
opened: 2026-11-20
priority: P2
cost: M
---

Found building `a-bound-to-decide-against-never-becomes-a-t` (DL5.2, `crates/geom-core/README.md`: "a lever arm ... never becomes a `T`").

## What

Each of these is an arm read as an `f64` bound and lifted into a `Lever<T>`, because the door it is handed to levers at `Lever<T>` and has other callers whose arm is a genuine `T`:

- `editor-core/src/mate/value.rs` `Chart::read`: the bundle's `Arm` lifted for `RigidMotion::least_turn`. `least_turn` is also called at a sphere's `Lever::curvature_radius(radius)` (`topo::boolean::ops::recut_turn`), so its arm cannot become `Lever<f64>`.
- `geom_core::RigidMotion::least_turn_minted`: the conditioning `T::from_computed(reach_f) * chord - two * arm.length()`. The run's rounding reach is an `f64` (`Tol::rounding_reach`), and the margin sums two `T` terms, one scaled by it, which one allowance does not express.
- `topo::boolean::vtxfac`: `PierceLever::Within(T::from_computed(arm))`, a spline chart's certified arm, charged in the sector sagittas at `T`.
- `geom_brep::dihedral`: `Lever::curvature_radius(chart_arm(..).map_or_else(poison, T::from_computed))`, the lower arm of a two-sided `LeverBounds`.
- `geom_brep::extent`: `ExtentBall::from` an `f64` box (`T::from_computed(mid[k])`, `T::from_computed(half)`), the ball whose radius is the consumed extent's lever.

## What would move them

A lever door generic over where its arm comes from (an arm in `T`, or one read as an `f64` bound that the margin carries as `Margin::levered_at`'s allowance), so `least_turn` and the sector charges take either. `least_turn`'s conditioning then needs a margin of two terms, which is a second shape of allowance (`reach · chord − 2·arm` with the arm in `T`).
