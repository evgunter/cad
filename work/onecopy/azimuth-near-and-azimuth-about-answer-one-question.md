---
id: azimuth-near-and-azimuth-about-answer-one-question
kind: issue
title: Azimuth::near and Azimuth::about answer one question: the angle's copy within half a turn of a reference
status: open
opened: 2026-11-04
priority: P1
cost: E
---


Filed by `pcert/projected-one-inverse` (PR 6500) under the pcert orchestrator's ruling of 2026-11-04 (D3, "One reading per question": one door per question).

## What

Two doors name an angle's copy within half a turn of a reference:

- `geom::Azimuth::about(Quarter)` (`crates/geom/src/azimuth.rs`, PR 6500). It is the projected image's old `branch_azimuth`: an exact quarter-turn rotation (a swap and sign flips), then `atan2`, with no decision. `geom_brep`'s projected image ψ (`ProjectedImage::raw`, through `geom::ChartChannels::read`) names its azimuth and its torus tube angle with it, about each piece's centre.
- `Azimuth::near(reference)` (PR 6482, `onecopy/angle-reads`). It is the angle window's read, renamed by whole turns through `Period::near`, with its decisions taken at `Band::least` outside the log.

## The ruling

There is to be one door, `near`, and its arm for a quarter-turn reference with the direction facing it is `about`'s exact rotation. That rotation keeps ψ bit-identical: PR 6500 compared the routed ψ with the old formula over 23 M `f64` reads and found it equal bit for bit (the only difference is the sphere latitude, which `about` does not name, at `z = −0.0`), and it keeps ψ in step with its box form (`ProjectedImage::window`, `atan2_range` on the same quarter-rotated box).

## Owed (PR 6482's lane, which lands second)

- Fold `about` into `near` as that exact arm. Callers read `Azimuth::near`, and `about` stops being public.
- Add a row showing `near(θ_k)` equals the rotation bit for bit over a sample of those reads: all the charts, the cone's two nappes and centres in −4..8, including signed-zero points. The one-shot test that took the 23 M reads is in PR 6500's description.
- Resolve the mechanical conflicts:
  - `scripts/gates/atan2-sites.tsv`'s rows `projected.rs raw 1` and `projected.rs branch_azimuth 1` go STALE, because PR 6500 takes both to 0. The counts for `window` and `atan2_range` do not change.
  - 6482's `Azimuth::tube(h, rho, major)` in `ChartInverse::point`'s torus and spindle arms moves into `ChartChannels::read`, which now holds that arithmetic.
  - `scripts/gates/from-f64-sites.tsv` is edited by both PRs, at different rows: 6500 moved the quarter-turn constant's row from `projected.rs` to `azimuth.rs`.
