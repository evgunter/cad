---
id: carrier-net-encloses-helper-has-absolute-slack
kind: issue
title: carrier_net's test helper encloses allows 1e-12 absolute slack, so it would miss a remainder deficit below that
status: open
opened: 2026-10-14
priority: P3
cost: E
---


Found by the review of PR 5097 (`pcert/spline-pair-any-carrier`).

`crates/geom-brep/src/carrier_net.rs`'s test helper `encloses` accepts a
sample whose `f64` value lies within `1e-12` of the Bernstein sum's
interval evaluation. A net whose Taylor remainder fell short of the
truth by less than that (a remainder deficit at a fine ε) would pass.
The slack should scale with the sample's own rounding (a few ulps of
the value), or the comparison should be made in interval arithmetic.
