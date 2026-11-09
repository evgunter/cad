---
id: a-points-height-along-a-unit-line-is-spelled-inline
kind: issue
title: A point's height along a unit 3D line, (p − o)·u, is spelled inline beside geom::Line::height
status: open
opened: 2026-11-06
---


Found by the sweep of `onecopy/planar-foot`. The shape was `(a − b).dot(u|dir|tau|d|chord)` over `crates/*/src`, with tests, rows and fuzzers excluded. After PR 6568, a point's height along a line in metres has one home, `geom::Line::height`: `(p − origin)·dir / |dir|`, correct for any nonzero `dir`.

The sweep found these 3D sites that read the unit-direction height inline, `(p − o)·u` with no division:
- `crates/topo/src/ring_path.rs` `LoopSplit::margin`, `Side` arm (~`:890`)
- `crates/topo/src/boolean/region.rs` (~`:1133`)
- `crates/topo/src/clearance.rs` (~`:3402`)
- `crates/sweep/src/blend/surgery.rs` (~`:2484`, `:2613`, `:3167`, `:5972`)
- `crates/sweep/src/blend/reach.rs` (~`:1096`, `:1454`)
- `crates/geom-brep/src/intersect/conic_arc.rs` (~`:355`)

Each reads its direction as unit, which PR 6568 measured holds only to the convention's `ε/arm` for a frame axis, and not at all for a `Curve3::Line` dir. Porting moves values by about `δ·|w|` where the direction is off unit by `δ`, and widens them by about 1.5× at `Interval` where only the direction carries width (`Line::height`'s doc table).

Fix: port each onto `Line::height` where the direction is a line's, and measure any margin that lands near the band. A site whose direction is a decided `UnitVec3` may wait for the typed axis (`an-axis-reads-its-direction-as-unit-by-convention-not-type`). Say which you chose and why.
