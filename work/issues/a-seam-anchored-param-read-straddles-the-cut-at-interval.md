---
id: a-seam-anchored-param-read-straddles-the-cut-at-interval
kind: issue
title: topo: seam-anchored param_near reads at Interval straddle atan2's cut at a point half a turn from u_ref
status: open
opened: 2026-10-19
---


Found by the class sweep of
`work/band/an-interval-fillet-cut-off-at-an-oblique-end-reads-its-arc-backward.md`
(fixed in `sweep::blend::surgery::arc_window`). Suspected, not
measured at these sites.

`Curve3::param_near(p, T::zero())` on a circle, ellipse or spiric at
`Interval` reads the hull `[−π, π]` for a point half a turn from the
carrier's `u_ref`: the `atan2` box straddles its branch cut
(`geom_core::Interval::atan2`'s docs). `f64` reads `±π` there. Each
site below anchors at the seam and then uses the value as a parameter:

- `crates/topo/src/offset_derive.rs`, the section-sense read
  (`candidate.param_near(mid, T::zero())`, then `deriv(at)` into
  `decide("offset_section_sense", …)`) and its `reach` closure — a hull
  parameter makes the cosine wide, so the decide escalates.
- `crates/topo/src/chord_join.rs` `SectionCurve::param`, the oval arm.
- `crates/topo/src/replace_face.rs`, the analytic `seed` read of
  `old_point` that `ends_at((seed, seed))` roots from.
- `crates/topo/src/shell.rs`, the seam-stretch's reversed arm
  (`reversed.param_near(forward.eval(near), T::zero())`, then
  `stretch(&reversed, mid)`). The point is the edge's midpoint; on a
  full-turn window that is the forward carrier's `π`, which is the
  reversed carrier's cut. The arm runs only after the forward stretch
  fails, so reachability is unknown.

A hand copy of the read, which a `param_near(` sweep cannot match:

- `crates/editor-core/src/names/emit_topo.rs` `param_along`, the
  ellipse arm: `Curve3::param_near`'s ellipse arm written out, anchored
  at the edge's mid-parameter, so its cut is half a turn from it — a
  full-turn edge's own end. Reachability is unknown.

`editor-core`'s `emit_topo` read at the same anchor tests only
`is_none()` and is not affected. The `geom-brep` hits are `f64` test
oracles. A fix in the shape of `arc_window` — re-read at `T::pi()`
when the seam's read spans half a turn or more — keeps each read a
function of the point and the carrier; a measured witness per site
comes first.

## Partly moot (WIRE orchestrator, 2026-10-26)

Closed by PR 5758 (2026-10-26, "nothing is ranked", Ev's ruling on PR 5705). The `param_along` bullet is moot: the rank readers are deleted. The rest stands.
