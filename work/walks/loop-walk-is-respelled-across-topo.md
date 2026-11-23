---
id: loop-walk-is-respelled-across-topo
kind: issue
title: the face loop walk (once(outer).chain(rings)) is respelled across topo/src outside census.rs
status: closed
opened: 2026-09-13
priority: P1
cost: M
closed: 2026-11-22
---


## The finding

`census.rs` now walks a face's loops through one iterator pair
(`face_loops`, `face_cycles`; PERF-12). The same walk —
`core::iter::once(&face.outer).chain(&face.rings)`, then the loop's
`LoopBoundary::Cycle` and `loop_cycle` — is respelled across the rest of
`topo/src`, each site with its own discard disposition:

- `crates/topo/src/boolean/boxes.rs` — `axial_window` (inside `face_box`)
  and `face_window_steps`, the box constructors' own walks;
- `crates/topo/src/boolean/ops.rs` (`ops.rs:918`);
- `crates/topo/src/validate.rs` (`:3750`, `:5128`, `:5858`, `:7276`);
- `crates/topo/src/props.rs` (`:1043`);
- `crates/topo/src/chart_region.rs` (`:2737`);
- `crates/topo/src/coherence.rs` (`:676`), `movefac.rs` (`:99`),
  `pcurves.rs` (`:2076`), `seqgen.rs` (`:605`, `:778`, `:1413`),
  `review_m1_pr4.rs` (`:467`).

Line numbers ride along; the pattern is
`grep -rn "once(&\?face.outer).chain" crates/topo/src`. Out of PERF-12's
fence (census.rs only); the PR-10 fix pass did the same consolidation
for `pick.rs`.

## What a fix is

One walk on `Body` (a `face_loops`/`face_cycles` pair, or an iterator
on `Face`) that every site reads, each site keeping its own answer to an
unwalkable loop — the `loop-boundary-discards` register moves with it.

- 2026-11-21 (TESSLIE) — the chart-coherence examination is retired: `work/tesslie/chart-coherence-ships-off-and-nothing-schedules-turning-it-on.md`. The `coherence.rs` spelling is gone.

## Closed

2026-11-22 (WALKS staleness audit, origin/main 7709b0ea): fixed and a duplicate. `grep -rn "once(&\?face.outer).chain" crates/topo/src` returns nothing; PR 6460 (ONECOPY, `a face's loops walk through one home`) routed every listed site through `FaceView::loops`/`boundary`/`cycles` and `Body::unchecked_face_boundary` (live.rs), and the source pin `live::tests::a_faces_loops_are_walked_in_one_place` holds it. The row was the same finding as `a-faces-loops-are-walked-by-hand-in-thirty-four-places`, filed a day earlier with fewer sites; that row carries the surviving half (what a non-Cycle loop means to each reader).
