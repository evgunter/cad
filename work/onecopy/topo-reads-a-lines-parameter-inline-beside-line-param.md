---
id: topo-reads-a-lines-parameter-inline-beside-line-param
kind: issue
title: topo reads a line's parameter inline, (p − origin)·dir, beside geom::Line::param
status: closed
opened: 2026-11-03
priority: P1
cost: E
branch: onecopy/small-reads
closed: 2026-11-03
pr: 6475
---


Found by the `onecopy/foot-and-angle` lane's sweep (shape `(p − origin).dot(dir)` with no division, against main at that lane's merge base), which repointed `Curve3::param_near`'s line arm, import's `endpoint_params` line arm and sweep's open-trimline window onto `geom::Line::param`.

The same reading, a point's parameter along a line, is still spelled inline in topo, each taking `dir` as unit or dividing by `|dir|²` by hand:

- `crates/topo/src/boolean/mod.rs` — two `along` closures (the coplanar strip's span reading, and `boundary_cuts`), `(p − origin)·dir`.
- `crates/topo/src/boolean/rim_wedge.rs` `locus_param`, its `Locus::Line` arm, `(p − origin)·dir`.
- `crates/topo/src/chord_join.rs` (the tangent classification's line arm), `(p − origin)·dir / |dir|²`: `Line::param` written out by hand.
- `crates/topo/src/lib.rs` `along_line` (a test-facing helper), `(p − origin)·dir`, with the off-line distance beside it re-spelling `Line::foot`.

Each `dir` is unit at its source by the carriers' convention, so these agree with `Line::param` up to rounding; they are a second spelling of one reading, not a wrong answer. Fix: build a `geom::Line` where the pair is held (or hold one) and read `param`/`foot` on it.

Not in this row: surface-axis heights (`(p − origin)·axis` on a cylinder's or cone's unit axis, e.g. `topo/src/census.rs`, `boolean/boxes.rs`, `geom-brep/src/props/curved.rs`), which are a height along a stored unit axis rather than a line carrier's parameter, and `geom-brep`'s `curve_pair/meet_oracle.rs`, a `#[cfg(test)]` oracle whose independence from the code under test is its point.
