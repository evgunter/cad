---
id: blend-sheet-clip-picks-its-coaxial-arm-on-a-bracket
kind: issue
title: sweep::blend::reach::sheet_clip picks its sphere and cone arms by reading a bracket's hi() against the band zero, so a bracket decides which construction runs
status: open
opened: 2026-11-02
priority: P3
cost: E
---


Found by a designer lane of BOXES' extent-home fork (2026-11-02),
checked by the BOXES orchestrator against main.

`crates/sweep/src/blend/reach.rs` `sheet_clip` (a `Decide + Bounds`
function) chooses the sphere arm and the cone arm with
`let near = |x: T| x.hi() <= band.zero();` over the centre's or apex's
offset from the band's axis, and the cone's axis tilt. That is a
coaxiality decision read off a bracket's upper end with no margin and
no `decide`, which the `Bounds` scope rule (`geom-core/src/real.rs`,
"brackets never decide") forbids even inside an allowlisted file: the
allowlist admits the compound bound, not a bracket choosing a lane.
Both answers are sound bounds (a missed arm leaves the box unclipped),
so the cost is a looser box or a different box across the band edge,
not a wrong one; hence P3 (a guard-shaped violation), not higher.

The fix is a named `decide` per arm (the sphere's centre on the axis,
the cone's apex on it and its axis along it), or retiring the per-kind
match altogether if BOXES' extent home lands (`work/boxes/log.md`,
2026-11-02): `sheet_clip`'s edge match is one of the per-kind doors that
design folds into one cover per face.
