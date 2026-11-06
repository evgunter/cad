---
id: a-points-distance-from-a-line-is-spelled-as-a-cross-product
kind: issue
title: A point's distance from a line is spelled |(p − o) × a| (/|a|) at about seven sites beside Line::offset, a tighter-or-looser trade at Interval nobody has measured
status: dispatched
opened: 2026-11-06
branch: onecopy/feet-and-params
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

## Closed (`onecopy/feet-and-params`, third unit)

**Measured.** A scratch `Interval` probe, not committed, took a point at `(0.7, 0.2, −0.4)` off the line's origin with half-width `r`, on an exact `ẑ` axis and on two tilted axes normalized at `Interval`, with the origin at the world origin and at `(1234.5, −987.25, 456)`:
- For `r` = 1e-9 or 1e-6 on a tilted axis, `|Line::offset(p)|` is 1.55–1.6× wider than `|w × a|`. That is 4.55e-9 against 2.85e-9, alike at both origins.
- For an exact point, the two are within rounding: the offset is 0.8–1.0× of the cross product.
- On the exact axis they agree.

So the cross product is the tight distance for any point that carries width, which every computed point at `Interval` does.

**Decided.** `Line::distance(p) = |(p − origin) × dir| / |dir|` is the one scalar reading of a point's distance from a line, and its doc carries the width table. `Line::offset` is kept for readers that need the vector. The disposition:
- Every scalar reader is on `distance`: 5 of the cross-product sites above (`ring_path.rs` ×2, `boolean/join.rs`, `section_cert/cone_pair_search.rs`, `sweep/src/blend/battery.rs`), and 30 `offset(p).norm()` sites, most of them made by the second unit.
- `scripts/gates/line-distance-reads-distance.sh` fails on a new `offset(…).norm()`.
- The review pass added `cover.rs`'s `point_range` and `intersect.rs`'s `reject_from(b).norm()`. The gate now also catches `reject_from(…).norm()` and a hand cross norm, over multi-line statements in production sources.
- Not ported:
  - `emit_topo.rs`'s `Segment` row, whose area `|w × d|` goes to `Margin::over_lever` over the segment's length. That door is the home of a measure over its lever.
  - `pcurve_cache.rs`'s and `carrier_resolution.rs`'s sites and `section_cert_cone_rows.rs`, which are `#[cfg(test)]` oracles.
