---
id: geom-brep-tol-census-omits-six-band-minting-suites
kind: issue
title: geom-brep's tol.rs census omits six suites that mint their own band
status: open
opened: 2026-10-08
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
