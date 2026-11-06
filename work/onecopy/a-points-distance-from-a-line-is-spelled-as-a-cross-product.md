---
id: a-points-distance-from-a-line-is-spelled-as-a-cross-product
kind: issue
title: A point's distance from a line is spelled |(p − o) × a| (/|a|) at about seven sites beside Line::offset, a tighter-or-looser trade at Interval nobody has measured
status: open
opened: 2026-11-06
---


Found by `onecopy/feet-and-params` (second unit). That unit ported the subtractive spellings `(p − o) − a·((p − o)·a)` onto `geom::Line::offset`. Its sweep for the cross-product form, `(p − o).cross(a).norm()` (sometimes `/ a.norm()`), and its review found these sites beside it:

- `crates/topo/src/ring_path.rs` (~`:893`, a cylinder residual; ~`:1466`, `/ dir.norm()`)
- `crates/topo/src/boolean/join.rs` (~`:3551`, `/ d.norm()`)
- `crates/topo/src/boolean/section_cert/cone_pair_search.rs` (~`:579`)
- `crates/sweep/src/blend/battery.rs` (~`:3333`, `/ d.norm()`)
- `crates/geom-brep/src/carrier_resolution.rs` (~`:112`, `:131`, `/ axis.norm()`)
- `crates/topo/src/boolean/section_cert_cone_rows.rs` (~`:689`, a rows module, so possibly an oracle)
- `crates/geom-brep/src/pcurve_cache.rs` (~`:5970`, a ruling's generator `g` through `apex`, read along the edge's line)
- `crates/editor-core/src/names/emit_topo.rs` (~`:3996`, `(p − q0) × d` over the segment's length)

Each is the same fact as `Line::offset(p).norm()`. Some may sit in `#[cfg(test)]` items, which can stay as oracles. Check each before porting.

`splitting/join.rs` `ruling_pairs` was the one cross-form site ported, because its own row (`three-line-offsets-wait-on-line-offset`) asked for it and its ruling directions are unit. The rest were not ported, because the two spellings differ at Interval:
- `Line::offset` is `w − w.project_onto(a)`. It names `w = p − o` twice, so a wide `w` charges about `2·width(w)` along `a`, where the true offset has no width.
- The cross product names `w` once per component, so it is tighter for a wide `w`.
- Both name `a` more than once. The cross form names it in the norm as well, so neither form is tighter in every case. The `reject_from` trial in PR 6540 showed that this trade decides real escalations at ε 1e-12.

Fix: measure the two forms on these sites' actual widths. Then either port the sites onto `Line::offset`, or give `geom::Line` a distance reading in whichever form is tighter, with the sites read through it. Do not keep two spellings.
