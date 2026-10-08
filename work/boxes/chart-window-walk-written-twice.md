---
id: chart-window-walk-written-twice
kind: issue
title: One home for the chart-window walk: chord_join::run_azimuth_window and solid_contain::torus_chart_windows are the same construction written twice
status: open
opened: 2026-09-01
github: 1483
refs: [1464]
priority: P1
cost: E
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

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## The per-edge extent, too (pcert/chart-box-harmonic-extent, 2026-10-01)

The two walks also spelled each edge's chart EXTENT differently. That
branch moved `chord_join::chart_azimuth_range` onto
`geom_brep::Pcurve::harmonic_span_box`, the one home `chart_box` and
the torus window walk (`boolean::boxes::TorusChartWindow::step`) read.
`solid_contain::torus_chart_windows` still takes the endpoint hull of
the linear part alone (`let at = |t| (p0.x + pl.x * t, …)`), dropping
whatever trigonometric part `bool_torus_chart_affine` classified Zero
in band. The span box encloses that residue; the unification above
should read it rather than carry a third spelling.

## Evidence (PR 3985, 2026-10-03)

`chord_join::run_azimuth_window` is gone: PR 3985 (`reach/arc-from-pairing`)
retired the chord's window selection, so no chord reads a window. The
split/join copy of the walk survives in `crates/topo/src/chord_join.rs`
as `face_azimuth_window` / `face_azimuth_images` with its pole rule
(`split_sphere_window_pole`) and the apex case (`ApexUnlifted`), and its
readers are now `boolean/solid_contain.rs` (containment), the pcurve
mint and the ring lane — none of them a chord. Both reviewers of that
PR flagged the hosting (about 500–770 lines of window walk in a module
whose chords no longer read it). The unification this item asks for is
now also a move: one home beside `torus_chart_windows`, out of
`chord_join.rs`.

## Where the torus walk stands (CONTACT-11, 2026-10-08)

`torus_chart_windows` now also checks that each entry equals the
previous exit (`bool_torus_chart_closure`). It decides
`bool_torus_chart_box` through `solid_contain::chart_polygon_box`,
which it shares with the cone trim. The walk itself is still a second
copy of `chord_join::face_azimuth_window`'s.
