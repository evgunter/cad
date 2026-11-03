---
id: the-spiric-evaluators-radicand-cancels-near-the-fold
kind: issue
title: The spiric's evaluators root f = √(ρ² − d²) as a difference of squares, which cancels near the fold
status: open
opened: 2026-11-02
---



Found by BOXES `cover-edge-doors` (`[simple.conditioned]` sweep).

`geom::spiric_radial` (`crates/geom/src/curves.rs`, `spiric_radial`) computes `f = √(ρ² − d²)` with `ρ = R + r·cos v`, the radicand as a difference of squares. Near the fold (`ρ_min = R − r` close to `|d|`) the squares cancel: their rounding at the scale `ρ² + d²` reaches `f` amplified by about `ρ²·u / (2·f)`. On the review's fold spiric (`R = 788.17…`, `r = 5.35…`, `d = 782.82…`, margin 1.05·escalate at the default ε) that is far past ε. The deciding door admits it.

All three evaluators (`eval`, `deriv`, `deriv2`) read this one spelling, and so do the chart images (`geom_brep::SpiricImage`).

The cover's period rectangle no longer has this problem. Under the same PR, `spiric_radicands` (`crates/geom/src/curves.rs`) poses each radicand as `(ρ − |d|)·(ρ + |d|)`, with `ρ` carried as a two-sum, so `ρ − |d|` keeps its digits. So near the fold the cover's `f_min` is accurate and the evaluated point is not. A box test that samples `Curve3::eval` there reads the evaluator's error as the box's (`topo`'s spiric rows sample a double-double `f` for this reason, `spiric_f_reference` in `crates/topo/src/boolean/boxes.rs`).

Re-pose `spiric_radial`'s radicand the same way, or say why the evaluator need not be. The evaluators are D9-ordered, so the change moves evaluated bits and every spiric baseline that reads them.
