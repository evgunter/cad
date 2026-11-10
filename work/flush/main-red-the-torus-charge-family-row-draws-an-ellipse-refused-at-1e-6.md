---
id: main-red-the-torus-charge-family-row-draws-an-ellipse-refused-at-1e-6
kind: issue
title: main red at CAD_TOLERANCE_EPS=1e-6: pcurve_cache's torus-charge family row draws an ellipse Curve3::ellipse refuses (draw 12)
status: open
opened: 2026-11-10
---

Found by the ladder lane's 1e-6 run before pushing PR 6758 (2026-11-10). It is red identically on `origin/main` at `fab26ee06ff`. It passes at the default ε and at `1e-12`.

- **The test:** `geom-brep` `pcurve_cache::circle_lane_rows::the_torus_charge_holds_on_every_family`, added by PR 6761 (`95a97aa509c`, `6315949594c`).
- **The panic:** `crates/geom-brep/src/pcurve_cache/circle_lane_rows.rs:1788`, `unreachable!("draw 12: an ellipse of finite semi-axes")`.
- **The draw:** `Curve3::ellipse(centre, n, radius, radius * s.log_uniform(0.05, 1.0), u, band())` with `radius = scale * s.log_uniform(1e-4, 3.0)`. At `1e-6` the band refuses draw 12's ellipse, presumably a semi-axis or their ratio within the band. So the `unreachable!`'s premise, that a finite pair of semi-axes always constructs, holds only at a fine band.
- **Fix direction:** draw the semi-axes clear of the band (scaled by `band().zero()`), or skip a draw the constructor refuses, with the refusal counted. Not an `unreachable!`.

Reproduce: `CAD_TOLERANCE_EPS=1e-6 cargo nextest run --profile ci -p geom-brep the_torus_charge_holds_on_every_family`.
