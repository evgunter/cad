---
id: a-points-height-and-radial-offset-pair-is-spelled-inline
kind: issue
title: A point's (height, radial offset) pair about an axis, h = w·a then w − a·h, is spelled at about thirty sites beside two helpers, with no home on geom::Line
status: dispatched
opened: 2026-11-06
branch: onecopy/feet-and-params
---


Found by `onecopy/feet-and-params` (second unit). That unit ported the radial offsets `(p − o) − a·((p − o)·a)` onto `geom::Line::offset`. A second sweep pass looked for the shape `let h = w.dot(a);` followed within three lines by `w − a·h`, over `crates/*/src` with test modules, rows and fuzzers excluded. It found 30 sites that read the axial height and the radial offset as one pair, sharing `h`:

- `topo`:
  - `ring_path.rs` (~`:163`), `validate.rs` (~`:10260`), `injective.rs` (~`:4226`), `chart_window.rs` (~`:602`)
  - `splitting/containment.rs` (~`:1596`), `census/depth_gap.rs` `split` (~`:382`, which also serves free vectors)
  - `boolean/`: `torus_region.rs` (~`:516`), `rim_wedge.rs` (~`:707`, the rim's), `slit.rs` (~`:1618`), `solid_contain.rs` (~`:1223`, `:1361`, `:3532`), `ruled_region.rs` (~`:479`), `carrier_touch.rs` (~`:160`), `fan.rs` (~`:52`), `section_cert.rs` (~`:1573`), `carrier_pair.rs` (~`:396`), `section_cert/cone_pair_search.rs` (~`:824`)
- `step-import`: `cr_r1_probes.rs` (~`:198`), `adopt.rs` (~`:1141`)
- `sweep`: `blend/reach.rs` (~`:295`), `blend/open/end_face.rs` (~`:542`)
- `geom-brep`: `implicit.rs` (~`:112`), `curve_face/circle_torus.rs` (~`:245`), `curve_pair/polish.rs` (~`:126`), `props/curved.rs` (~`:4904`, `:5042`, `:5482`, `:5936`)
- `geom`: `curves/on_carrier.rs` (~`:582`) and its `height_and_radius` helper

Some of these, for example `ruled_region.rs`'s `mean` and `section_cert.rs`'s `delta`, may reject a free vector rather than a point's offset. Check each before porting.

Why not ported in that unit: the pair is its own reading, and its home is the open question. That unit's ~45 radial ports already decided that `Line::offset`'s `/ |a|²` is acceptable: a last-place f64 move on a unit axis, which moved no test outcome at either ε. So the bits are no reason to keep the pair apart. What remains is that splitting it into `Line::param(p)` and `Line::offset(p)` computes `w` twice, and writing the radial half as `w − a·h` with a shared `h` is a third spelling of the offset.

Before you fix it, decide the home. One option is a pair reading on `geom::Line`, e.g. `Line::split(p) -> (T, Vec3<T>)`, beside `param` and `offset`, which `on_carrier`'s `height_and_radius` and `depth_gap::split` would then read through. Its radial half should read `Line::offset`'s expression, so the two cannot drift.
