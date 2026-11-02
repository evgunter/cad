---
id: hand-written-ulp-pads-stand-in-for-interval-arithmetic
kind: issue
title: about 60 sites pad an f64 result by a hand-written 'k·f64::EPSILON·|x|' instead of computing it in interval arithmetic
status: open
opened: 2026-10-30
priority: P3
cost: M
---


(QUAD 6013 fix lane; the delta review's style finding on PR 6013. Filed, not swept.)

A pad written by hand as `k * f64::EPSILON * |x|` (or `f64::EPSILON * k * ...`) claims an `f64` result lies within `k` ulps of `|x|` of its exact value. That claim holds only where `|x|` bounds the operands the result was computed from, not just the result itself. PR 6013's own two instances show it fails otherwise: `knot_split::piece_box` padded a line's `origin + dir·t` by ulps of the result, and `rectangle_winding` padded iso images by ulps of the rectangle's coordinates. The first now runs in `Interval` arithmetic. The second takes a per-kind slack from the image's own terms (`quad_lane::end_slack`, still this shape, scaled from what each endpoint was computed from).

Owed, per site: compute the quantity in `Interval` (or the scalar's own `Bounds`) where it is a certificate, or state at the site which operands the pad's `|x|` bounds and why.

**The hit list** at this filing, from `grep -rnE "[0-9.]+ \* f64::EPSILON \* |f64::EPSILON \* [0-9.]+ \* " crates --include=*.rs`, with `tests/` directories and comment lines left out. Many hits sit inside `#[cfg(test)]` or fuzz modules in `src/`, which this pattern does not tell apart. Triage those first; a test's own pad is a lesser defect than a certificate's.

- `crates/topo/src/offset_axial.rs:3620`
- `crates/topo/src/separation.rs:269`
- `crates/topo/src/census/segment_fuzz.rs:357`
- `crates/topo/src/census/segment_fuzz.rs:425`
- `crates/topo/src/boolean/reduce.rs:7622`
- `crates/topo/src/boolean/boxes.rs:5281`
- `crates/topo/src/boolean/boxes.rs:6267`
- `crates/topo/src/boolean/section_cert_rows.rs:3084`
- `crates/topo/src/boolean/section_cert_rows.rs:3225`
- `crates/topo/src/boolean/conic_quadric/cone_rows.rs:171`
- `crates/topo/src/boolean/section_cert/square_search.rs:361`
- `crates/topo/src/boolean/section_cert/cone_search.rs:712`
- `crates/topo/src/boolean/section_cert/cone_pair_search.rs:206`
- `crates/topo/src/boolean/section_cert/cone_pair_search.rs:207`
- `crates/topo/src/boolean/section_cert/cone_pair_search.rs:268`
- `crates/topo/src/boolean/section_cert/cone_pair_search.rs:363`
- `crates/topo/src/boolean/vtxfac.rs:3674`
- `crates/topo/src/injective.rs:740`
- `crates/topo/src/props.rs:8956`
- `crates/topo/src/ring_path/graze_rows.rs:152`
- `crates/topo/src/ring_path/graze_rows.rs:247`
- `crates/topo/src/ring_path/graze_rows.rs:266`
- `crates/topo/src/props/quad_lane.rs:1291`
- `crates/profile/src/seg_reach_fuzz.rs:193`
- `crates/profile/src/seg_reach_fuzz.rs:519`
- `crates/sweep/src/swept.rs:1597`
- `crates/sweep/src/blend/reach.rs:3757`
- `crates/geom-core/src/linalg/vec.rs:1134`
- `crates/geom-core/src/linalg/vec.rs:1177`
- `crates/geom-core/src/linalg/vec.rs:1178`
- `crates/geom-core/src/linalg/vec.rs:1179`
- `crates/geom-core/src/linalg/vec.rs:1198`
- `crates/geom-core/src/linalg/vec.rs:1206`
- `crates/geom-core/src/linalg/vec.rs:1207`
- `crates/geom-core/src/linalg/vec.rs:1208`
- `crates/geom-core/src/real.rs:2795`
- `crates/geom-brep/src/carrier_resolution.rs:172`
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz/projected.rs:631`
- `crates/geom-brep/src/pcurve_cache/harmonic_sup_rows.rs:328`
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz.rs:878`
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz.rs:1060`
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz.rs:1114`
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz.rs:1189`
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz.rs:1210`
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz.rs:1720`
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz.rs:1836`
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz.rs:1921`
- `crates/geom-brep/src/extent.rs:1735`
- `crates/geom-brep/src/curve_pair/one_curve_rows.rs:292`
- `crates/geom-brep/src/curve_pair/one_curve_rows.rs:293`
- `crates/geom-brep/src/carrier_net.rs:1331`
- `crates/geom-brep/src/ssi.rs:3860`
- `crates/geom-brep/src/pcurve_cache.rs:13173`
- `crates/geom-brep/src/props/loop_area.rs:321`
- `crates/geom-brep/src/ssi/enclose.rs:1952`
- `crates/geom-brep/src/ssi/enclose.rs:2483`
- `crates/geom-brep/src/ssi/enclose.rs:2546`
- `crates/geom-brep/src/ssi/compensated.rs:270`
- `crates/geom-brep/src/ssi/compensated.rs:306`
- `crates/geom-brep/src/ssi/compensated.rs:472`
- `crates/geom/src/surfaces/nurbs.rs:2013`

**What the pattern cannot match:** pads spelled `f64::EPSILON * x * k`; a named constant (`ULP`, `EPS`) times a magnitude; `x.abs() * 1e-15`-style literals; `next_up`/`next_down` chains; and pads inside macros. A second pass shaped at those is owed with the triage.
