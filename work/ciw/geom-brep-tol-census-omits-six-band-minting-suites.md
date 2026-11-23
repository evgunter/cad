---
id: geom-brep-tol-census-omits-six-band-minting-suites
kind: issue
title: geom-brep's tol.rs census omits six suites that mint their own band
status: closed
opened: 2026-10-08
priority: P4
cost: C
closed: 2026-11-22
---


`crates/geom-brep/tests/shared/tol.rs`'s module docs keep a census of
the suites that build a band deliberately not the run's ("That list is
a census ... a new suite minting its own band belongs in it"). A grep
for `Band::new`, `Band::linear_at` and `Band::from_thresholds` under
`crates/geom-brep/tests/` finds six suites the census does not name:

- `axis_rows_read_as_one_sum.rs` (`narrow`, `Band::new(z, 1.2 * z)`)
- `curved_torus_arc_residual.rs` (`Band::linear_at(Tol::witness(), eps)`, twice)
- `m5_pr7_ssi.rs` (`Band::new(eps, eps * K)` at a named ε)
- `pcurve_conic.rs` (a fixed 1e-9 .. 1e-8, twice)
- `pcurve_spiric.rs` (`wide_band`, `Band::linear_at(tol(), 100.0 * eps)`)
- `sphere_circle_certificate.rs` (a fixed 1e-9 .. 1e-8)

The census also counted "six" suites while naming five; the count was
dropped when `span_reach_differential.rs` was added to it. Each
unlisted suite wants its line (and its site's reason for pinning, if
it lacks one), or, if it follows ε after all, a move to `band()`.

Moved from TINT to CIW on 2026-11-20, by `git mv`, when TINT merged into CIW: TINT held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CIW carries.

## Closed

`shared/tol.rs`'s census names the five suites that still mint (`axis_rows_read_as_one_sum`, `curved_torus_arc_residual`, `m5_pr7_ssi`, `pcurve_conic`, `pcurve_spiric`); `sphere_circle_certificate.rs` no longer exists.
