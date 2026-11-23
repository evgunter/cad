---
id: a-first-harmonics-roots-and-stretch-are-solved-three-ways-in-the-pose
kind: issue
title: A first harmonic's roots and stretch are solved three ways in the pose, and Slant copies Range's state
status: open
opened: 2026-11-21
priority: P3
cost: M
---


Filed by ONEHOME from PR 7208's delta review. No answer is wrong; one
home is owed.

**Three solvers of one quantity**, all in
`crates/geom-brep/src/curve_pair/pose.rs`. A first harmonic's roots and
its in-band arc are each solved separately:
- `Slant::roots` (`pose.rs:1588`): `θ_hi ± acos(−c₀/A)`, a touch's two
  roots;
- `harmonic_roots` (`pose.rs:2371`): the half-angle solve for two
  definite crossings;
- `Slant::read`'s `φ₁`, `φ₂` (`pose.rs:1614`) and `Range::band_arc`
  (`pose.rs:2250`): the arc where the residual lies within a level.

One home for "the angles where `c₀ + A cos(θ − θ_hi)` takes the value
`y`", with its conditioning stated once, would serve all three.

**Slant copies Range's state.** `Slant` holds `at` (`θ_hi`), `swing`
(`A`) and `c₀` in `r`. `Range::first_harmonic` already holds these. The
reading should borrow the range, or the range should carry what the
slant reads.
