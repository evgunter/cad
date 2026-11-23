---
id: spline-lanes-lever-their-per-sample-reads-at-the-spans-diameter
kind: issue
title: the spline lanes lever each sample's transversality at the span's diameter from below, not at its reach from that sample
status: open
opened: 2026-11-23
priority: P1
cost: H
---


Part two of `work/oneround/diameter-levers-of-a-datum-read-at-a-point.md`, cut from it by `oneround/reach-from-the-read-point` (PR 7661), which converted the sites whose read point is in hand. In each site below, one lever is handed into a lane, and the lane makes its own read points (samples, roots) from it. So the honest arm, the span's reach from each read point, can only be read inside the lane. Lines are on `main` at `fce2fbd91`.

**The sites.**

- **`crates/geom-brep/src/certify.rs` (about :5654).** `let extent = crate::edge_extent_inf(&spec.carrier, t0, t1)` is handed to the residual × NURBS lane.
  - **Per-sample transversality.** Each sample's transversality is read at the sample, levered at `extent`, and a definite sign is acted on. It is owed the span's reach from the sample, from below.
  - **The tube.** `extent` also sizes the lane's uniqueness tube, which is a whole-span datum and stays a diameter.
- **`crates/geom-brep/src/edge_nurbs.rs` (about :2152).** The tube's `edge_extent_inf(&carrier, p0, p1)` is handed to `ssi::certify::certify_branch` (`Limbs::Tube`). Read which of that lane's readings are per sample and which are of the whole span.
- **`geom_brep::boundary_section`.** It is reached from `topo::offset_derive::plane_spline_root` (the spline arm of the corner root, through `replace_face::corner_arm` and `root_along`) and from `level_row`.
  - It isolates the plane's roots along a spline over the whole carrier, and levers each root's graze at the one `extent` handed in.
  - The graze is read at the root, and its consumers act on both signs (the analytic arm's reading, PR 7661).
- **`crates/topo/src/boolean/contact_verify.rs` (about :339).** `edge_extent_sup` caps the tangent schedule's angular margins at each sample. It is from above: sound, but loose by up to the ratio of the diameter to the reach.

**What each lane would have to take.** The carrier and its span (or its cover over the span) in place of one lever. Each read point then reads `geom_brep::span_reach_bounds(carrier, span, point)` (or `topo::cover::piece_reach_bounds`) and takes the bound its verdict is owed: from below where a definite transversality is acted on, from above where a Zero is, and both where both are. The tubes keep the whole span's extent.

**Which bodies' admission it decides.**
- **`certify.rs` and `edge_nurbs.rs`:** the spline certificate, so which NURBS intersection edges certify. A shorter arm refuses more, so a body the base admitted can be refused. Each such body is owed a report: the old lever, the new one, and whether the old admission was right.
- **`boundary_section`:** an offset door's corners on spline edges, i.e. whether a face move succeeds.
- **`contact_verify`:** whether a declared tangent contact verifies. From above, it only tightens toward admitting.

**What a row needs.** A sample (or root) at a span's middle whose reading sits between the thresholds the reach and the diameter give, as PR 7661's rows do for the analytic arm.
