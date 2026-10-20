---
id: a-narrow-cones-ruling-direction-datum-widens-its-interval-near-the-apex
kind: issue
title: A narrow cone's ruling direction datum p·cos α − h·sin α widens its Interval enclosure near the apex; 48 rulings at α 0.1, scale 1e3, ε 1e-12 escalate that certify on main
status: open
opened: 2026-10-19
priority: P4
cost: E
---


Found by delta review d5382 of PR 5382's fix pass (`58767c1c08`).

## What

PR 5382 reads a ruling's distance off its generator in the frame
(ĝ, N̂, e × q̂). The direction datum's Line term is
`√((p·cos α − h·sin α)² + m²)` (`crates/geom-brep/src/pcurve_cache.rs`
about :4968). On a narrow cone, `p·cos α − h·sin α` subtracts two nearly
equal terms, so its Interval enclosure pays the sin and cos enclosure
widths, multiplied by the reach.

Measured on one of the rows (α 0.1, scale 1e3, ε 1e-12, a ruling ON
the cone, anchor at v = 1):

| term | main | PR 5382 |
|---|---|---|
| Line | 4.18e-14 | 1.94e-13 (4.6× wider) |
| Radius | 1.5e-13 | 1.6e-16 |
| incidence total | 0.97e-12 | 1.08e-12 |

The extra in the total is mostly FidelityV (8.88e-13 against 7.77e-13).

Over d5382's 11,592-row grid, 48 Interval rulings certify on main and
escalate on the PR, all in the α 0.1, scale 1e3, ε 1e-12 cell.
Against that, the PR certifies 6,647 more Interval rulings than main
(18,892 against 12,245).

These are availability losses only: no envelope reads under the truth.

## Fix shape

Spell the direction datum without the cancelling difference, for
example from `sin(β − α)` with β the datum's own angle, or as a cross
product against ĝ. Add a row at α 0.1, scale 1e3, ε 1e-12 that is red
on the current spelling.
