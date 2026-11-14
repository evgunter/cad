---
id: a-general-pcurve-evaluated-over-a-wide-parameter-widens-thousands-fold
kind: issue
title: a General pcurve evaluated at an interval parameter widens about 3400-fold: the joint gap reads a fitted pcurve's naive enclosure
status: open
opened: 2026-11-14
priority: P2
---



Filed by the sweep-walls-at-t lane (ONEHOME), measured on the closed-spine path sweep (`crates/sweep/tests/self_intersection_proves_injectivity.rs` `a_closed_spine_reads_violated_top_against_bottom`) at `CAD_TOLERANCE_EPS=1e-12`, at `Interval`.

**What widens.** `topo`'s loop joint on a spline chart (`crates/topo/src/pcurves.rs` `spline_gap_closes`) reads `lifted.eval(entry_t)`, the half-edge's stored pcurve at its start parameter. On half-edge `18v1` that pcurve is `Pcurve::General`, a fitted degree-5 `NurbsCurve2`, and `entry_t` is an enclosure 5.4e-14 wide (`[0.49999999999997313, 0.5000000000000269]`). The result is `(1 ± 9.2e-11, 1 ± 9.2e-11)`, 1.83e-10 wide: about 3400× the parameter's width. The fitted pcurve's own speed is of order one, so a centred (mean-value) form would give around 1e-13. Metered by the chart's sup stretch (u ×0.26, v ×11), the joint escalates at ±4.7e-11.

**Why the pcurve is General there.** It shouldn't be: that edge is a cap rim on a wall boundary, whose exact image is the boundary row. The lane fell through because of the edge's own width. That cause is the station-frames row, `work/onehome/three-path-sweeps-refuse-at-1e-12-over-station-frames-minted-at-interval.md`. This row is only the evaluation: wherever a General pcurve is read at an interval parameter, it reads the naive enclosure.

**Owed.** Decide whether `Pcurve::General`'s evaluation at an enclosure parameter (geom-brep `pcurve_cache.rs`, over `geom`'s `NurbsCurve2` evaluation) should use a centred form, or whether its readers (the joint's chart gap, and any other) should read a point parameter's image plus a Lipschitz term. Measure the readers first.
