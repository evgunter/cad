---
id: chart-window-walk-written-twice
kind: issue
title: One home for the chart-window walk: chord_join::run_azimuth_window and solid_contain::torus_chart_windows are the same construction written twice
status: open
opened: 2026-09-01
github: 1483
refs: [1464]
---

## From GitHub issue 1483

Opened 2026-09-01; 0 comments.

Recorded at BOOL-3's adjudication (PR [#1464](https://github.com/evgunter/cad/pull/1464)); both blinded reviews flagged the unscheduled deviation independently.

`topo::chord_join::run_azimuth_window` and `topo::boolean::solid_contain::torus_chart_windows` are the same construction — walk a face's outer cycle, take each edge's closed-form chart image from `geom_brep::chart_pcurve`, pin each edge's branch by nearest-branch continuity against the previous exit, and hull the result — written twice. They differ in two ways that kept BOOL-3 from sharing them:

- the split/join copy answers ONE channel (the azimuth) and carries the sphere chart's pole-junction rule (`split_sphere_window_pole`), which a ring torus neither needs nor can use;
- the containment copy answers BOTH channels and no junction rule, and additionally carries the closure and bounding-box checks BOOL-3 added (`bool_torus_chart_closure`, `bool_torus_chart_box`).

The unification is a channel parameter with the pole arm gated on the azimuth channel, plus a decision about whether the box checks belong to both callers. `chord_join.rs` was outside BOOL-3's scope fence, and it is a hot file, so this was left unscheduled there. The hazard is drift: the branch-pin argument is stated at both sites and a change to one is a change to both, with nothing enforcing it. The relationship is stated at both sites as of PR #1464.

## Home

`work/bool/` — BOOL-3's own disclosed residue, and one of the two sites (`topo::boolean::solid_contain`) is inside S-BOOL's territory glob `crates/topo/src/boolean/*`.
