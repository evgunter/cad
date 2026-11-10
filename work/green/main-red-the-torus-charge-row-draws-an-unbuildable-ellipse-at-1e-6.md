---
id: main-red-the-torus-charge-row-draws-an-unbuildable-ellipse-at-1e-6
kind: issue
title: main red at CAD_TOLERANCE_EPS=1e-6: circle_lane_rows::the_torus_charge_holds_on_every_family's draw 12 builds no ellipse
status: open
opened: 2026-11-10
priority: P2
cost: C
---


Found by ONECOPY's replay-axis-lever lane on its `CAD_TOLERANCE_EPS=1e-6` run, which CI's eps lane runs on the crates a diff touches (`.github/workflows/ci.yml`'s eps matrix). It fails identically on main at dd2dd683, so it is not that lane's.

`crates/geom-brep/src/pcurve_cache/circle_lane_rows.rs:1788` (`the_torus_charge_holds_on_every_family`, `ci` profile) panics:

    internal error: entered unreachable code: draw 12: an ellipse of finite semi-axes

The draw's radius is `scale * log_uniform(1e-4, 3.0)` and its minor semi-axis `radius * log_uniform(0.05, 1.0)` (`circle_lane_rows.rs:1776-1784`). The likely cause, not yet confirmed: at a small scale the minor semi-axis can be well under a micrometre, and at ε = 1e-6 `Curve3::ellipse(…, band())` refuses an ellipse that small, which the row reads as unreachable. It passes at the default ε and at 1e-12.

**Owed.** Either draw the semi-axes clear of the band the row builds at, which the band's ε gives, or read the constructor's refusal as a draw to skip, counted so that a sweep that skips everything reds. Then check whether 95a97aa5 (6761's review, which last touched the row's draws) moved this.
