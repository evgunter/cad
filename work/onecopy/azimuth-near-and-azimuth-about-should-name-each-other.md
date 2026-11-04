---
id: azimuth-near-and-azimuth-about-should-name-each-other
kind: issue
title: "Azimuth::near and Azimuth::about should name each other: a decided copy near a reference, and an exact evaluation on a known quarter branch"
status: open
opened: 2026-11-04
priority: P3
cost: E
---


Filed by `pcert/projected-one-inverse` (PR 6500), under the pcert orchestrator's ruling after PR 6500's FULL review (fix list Z1).

## What

Two angle doors in `crates/geom/src/azimuth.rs` sit close together, but they answer two questions:

- `geom::Azimuth::about(Quarter)` (PR 6500) evaluates the angle on a known branch, with no decision. It is an exact quarter-turn rotation (a swap and sign flips), then `atan2`. `geom_brep`'s projected image ψ (`ProjectedImage::raw`, through `geom::ChartChannels::read`) names all three of its angles with it, on the branch its derivation decided: the azimuth about `θ_k`, the torus tube angle about `φ_k`, and the sphere latitude about `Quarter(0)`.
- `Azimuth::near(reference: T)` (PR 6482, `onecopy/angle-reads`) decides. It takes the angle window's read and renames it by whole turns through `Period::near`, at `Band::least`.

A quarter arm cannot be selected from a `T` reference. Folding `about` into `near` would therefore either make a second door under one name, or make ψ decide at evaluation. So the two stay.

## Owed (PR 6482's lane)

- One sentence in `near`'s doc that points to `Azimuth::about` for an exact evaluation on a known quarter branch.
- Resolve the mechanical conflicts with PR 6500:
  - `scripts/gates/atan2-sites.tsv`: the rows `projected.rs raw 1` and `projected.rs branch_azimuth 1` go STALE. PR 6500 takes both to 0 (`branch_azimuth` is gone, and `raw` reads through the table). The rows for `window` and `atan2_range` are unchanged.
  - 6482's `Azimuth::tube(h, rho, major)` in `ChartInverse::point`'s torus and spindle arms moves into `ChartChannels::read`, which now holds that arithmetic.
  - `scripts/gates/from-f64-sites.tsv`: both PRs edit it, at different rows. PR 6500 moved the quarter-turn constant's row from `projected.rs` to `azimuth.rs`.

The routed ψ is the old spelling bit for bit in sector. `crates/geom-brep/src/pcurve_cache/projected/old_formula_rows.rs` pins that at `f64` and `Interval`.
