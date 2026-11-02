---
id: a-points-radial-offset-from-an-axis-is-spelled-inline
kind: issue
title: A point's offset from its foot on an axis, (p − o) − a((p − o)·a), is spelled inline at about twenty sites
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by the review of PR 6328. That PR repointed every spelling of a point's *foot* on a line (`origin + a·((p − origin)·a)`) at `geom::Line::foot`/`param`. It left alone, by one rule, every spelling of the *offset* `p − foot`, written as the rejection `(p − o) − a·((p − o)·a)`, usually with `w = p − o` computed a line earlier. That offset is the same fact (`p − Line::foot(p)`), and its sites far outnumber the foot's. Most feed a radial distance or a radial direction (`.norm()`, `.normalize()`) that a decision reads, and some take `a` as unit while others divide by `a·a` or normalize first. So repointing them moves margin bits and wants its own review.

Sites, on main as of PR 6328 (sweep: an inline `(p − o) − a·((p − o)·a)`, and `let w = p − o;` followed within three lines by `w − a·(w·a)`):

- `crates/sweep/src/blend/reach.rs` (the sector's plane, the `st.p − c` rejection, dividing by `u·u`);
- `crates/step-import/src/recognize.rs` (the envelope's `rho_max`, and the chord offset `off`);
- `crates/step-import/src/recognize_curve.rs` (the chord offset `off`);
- `crates/topo/src/face_normal.rs` ×3;
- `crates/topo/src/ring_path.rs` ×3;
- `crates/topo/src/boolean/edge_join.rs`;
- `crates/topo/src/boolean/join.rs` (radical-plane frames, the axis-offset row);
- `crates/topo/src/boolean/rim_wedge.rs` ×2;
- `crates/topo/src/boolean/solid_contain.rs` ×4;
- `crates/topo/src/boolean/node.rs` (divides by `|axis|²`);
- `crates/topo/src/boolean/region.rs` `polar`;
- `crates/topo/src/offset_axial.rs` (the generator side's `off`).

Also owed with it: `crates/editor-core/src/pose.rs` `AxisLine::through`, whose stored foot is the origin's foot written `point − d·((point − O)·d)`. That PR left it so as not to move a stored axis foot's bits.

Not included: rejections of a free direction (`v − a·(v·a)` where `v` is no point's offset), which are a vector's component square to `a`, not a point's offset from a line.

Fix: a reading on `geom::Line` for the offset (or the radial distance), with these sites repointed in one PR that re-baselines whatever bits move.
