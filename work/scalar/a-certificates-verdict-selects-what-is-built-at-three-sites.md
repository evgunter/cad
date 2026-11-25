---
id: a-certificates-verdict-selects-what-is-built-at-three-sites
kind: issue
title: A certificate's pass or fail selects what is built at three sites, so a scalar that does not certify builds a different body
status: open
opened: 2026-11-25
priority: P1
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


From the design fork ruled on `[ev]` PR 7719 (2026-11-25). DL5.3 now says: "A certificate's verdict never selects a construction: its refusal is a finding, or the gap of its lane." Two designers found three sites where it does today, by reading (`origin/main` of 2026-11-24; nothing run):

- `crates/topo/src/pcurves/mint.rs` `nurbs_iso_derive`, the `arc_rim` closure: `PcurveCache::certify(rim)` refusing with one of five kinds (`Err(PcurveCertifyError::Incidence(_) | …) => general()`) sends the circle to the general lane.
- `crates/geom-brep/src/pcurve_cache/certify.rs` `row_certifies`: `PcurveCache::certify(..).is_ok()` used as a test. Through `side_row_certifies` / `pair_row_certifies` it decides whether a boolean treats a rim as a row two spline faces share (`crates/topo/src/boolean/spline_known.rs`) and whether `side_image_of` keeps a picked image. That changes topology, not one row. It is `T: Decide`, so a dual certifies through it today.
- The iso schedule's measured step (`mint.rs` `iso_schedule`, `derive_chart_foot` → `computed_foot`): rows chosen from chart feet only the fitted lane solves; skipped where the lane is absent, so the body then holds a different row or none.

Asks: at each, the choice becomes the pick's own decision in construction code, and the certificate then checks it; a picked construction whose certificate refuses is a finding. Before changing the first: measure how many `f64` arc rims pass the pick and fail the certificate today (they fall back to the general lane now and would refuse). Sweep for others: certify `Err` arms that fall through to another lane, and `certify(..).is_ok()` used as a condition, across `topo`, `geom-brep` and `sweep` (`sweep/src/loft.rs` calls `pick_arc_rim`). The third site's feet are also a stored solved value entered outside the admitting door (`stored-solved-values-whose-check-differs-in-kind-are-not-behind-the-door`).
