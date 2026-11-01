---
id: the-census-plane-residual-decisions-read-off-a-stored-origin-are-uncharged
kind: issue
title: The census decides plane residuals read off the face's stored origin uncharged: their rounding, about 4u·|p − o|, passes ε = 1e-12 a few hundred metres from it
status: open
opened: 2026-10-19
priority: P3
cost: M
---



Found by the sweep of `restread/ef-pierce-charge` (census closed forms
read from a stored origin).

## What

Each of these decides one plane residual `(p − o)·n`, `o` the plane's
stored origin, through the funnel as it stands, with no charge for its
rounding. `p − o` and the dot round to about `4u·|p − o|` (`u` the unit
roundoff), so the residual's error passes ε where `|p − o|` passes about
`ε/(4u)`: 2.3 km at ε = 1e-12 under the first-order bound, 560 m
under `geom_core::rounding_charge`'s `16u`. D4's session box is 1 km,
so at ε = 1e-12 a residual read across it is off by up to about half
of ε, uncharged, and a margin read within that of the band's edge
reads its side off the rounding. Suspected, not measured.

- `crates/topo/src/census.rs` `pair_edge_face`, `pm_census_ef_residual`
  (each end's sign; the pierce's placement off these residuals is
  charged since `restread/ef-pierce-charge`, the signs are not).
- `pair_vertex_face`, `pm_census_vf_residual`.
- `pair_holds_point` and its edge-pair sibling,
  `pm_census_confined_carrier`.
- `confirm_edge_face`, `pm_census_confirm_ef`.
- `crates/topo/src/census/point.rs` `circle_plane`, the circle centre's
  offset `s = n·(c − o)` read by `pm_census_point_plane` and, less
  `r·sin`, by `pm_census_point_gap`.

No division amplifies these, which is why they are a separate row from
the pierce placement: each is off by the rounding alone.

## Owed

Measure first: a vertex `Kε` off a face whose stored origin is 1 km
away, at ε = 1e-12, read at `f64` against `Interval`. If it reproduces,
read each residual's enclosure `r ± rounding_charge(|p − o|)` through
`enclosure_reading`, as `pierce_span` reads the pierce's lever.
