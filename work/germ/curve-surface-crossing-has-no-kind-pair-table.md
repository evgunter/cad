---
id: curve-surface-crossing-has-no-kind-pair-table
kind: issue
title: Curve-surface crossing has no kind-pair table: wildcard arms, fallback ladders, and circle-plane solved in six places
status: open
opened: 2026-10-14
priority: P1
cost: X
points: 16
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

## What

"Where does this curve cross this surface" has no `(CurveKind, SurfaceKind)` table. Today:

- **Planar faces:** `reduce::sweep_direction` (`crates/topo/src/boolean/reduce.rs:1046`).
  - Line uses interpolation and conic uses `splitting::conic_plane_candidates` (`crates/topo/src/splitting/classify.rs:563`).
  - Spiric and spline are refused `CrossingCarrierUnsupported`.
  - Dispatch is an Option gate, not a match.
- **Walls:** `reduce::curved_face_arm` (`:1918`) → `conic_clearance` (`:2737`) → `wall_crossing` (`:3320`) → per-pair root finders (`line_quadric_roots`, `line_cone_roots`, `conic_quadric_roots` at `crates/geom-brep/src/curve_face/conic_quadric.rs`, `circle_torus_roots`, `ellipse_torus_roots`).
  - Wildcards: `wall_crossing` `_ => Unsettled` (twice), `line_quadric_roots` `_ => Uncertain`, `conic_quadric_roots` `_ => {}`.
  - Runtime fallbacks: enclosures first, then roots, then `carrier_touch::off_face`; the dip-bound clearance, then roots; `implicit::conic_residual_extremes` harmonic, else sampled.
- **Circle×plane is solved in about six places:**
  - `carrier_cross::circle_meets_plane`: retired (stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227));
  - `conic_plane_candidates`;
  - `ops::apply_cut_ins` (`crates/topo/src/boolean/ops.rs:6053`);
  - `sphere_region::ray_roots` (`crates/topo/src/sphere_region.rs:414`);
  - `ring_path::plane_roots` (`crates/topo/src/ring_path.rs:1520`);
  - `pcurve_cache` incidence.
- **Line×cone** is solved twice, with different rows (`line_cone_roots`, and `solid_contain::cast_ray`'s own quadratic at `crates/topo/src/solid_contain.rs:5239`).
- **Census confirm:** `census::point::edge_face` (`crates/topo/src/census/point.rs:253`) serves only (circle, plane) and (line, sphere), and refuses the rest typed.

## Owed

Re-aimed to D3 "One reading per question". A curve's crossings of a surface are a pair *outcome*. One crossings reading, one exhaustive match over `(CurveKind, SurfaceKind)`, which each consumer reads. Where the crossing depends on the pair's pose (a curve lying in the surface, a tangent touch), it takes the pose reading's answer and decides no pose of its own.

- Closed forms where they exist. The certified walk for spiric and spline is those arms' one method, fixed at write time, not a method reached because a closed form declined.
- The wildcards (`wall_crossing` `_ => Unsettled`, `line_quadric_roots` `_ => Uncertain`) become listed arms with typed refusals naming what they lack. `conic_quadric_roots`' `_ => {}` answers a default value (no roots), which D3 forbids.
- The ladders: a stage before the method is allowed only if it is total in kind, decides the same question soundly at the same band, and cannot change the verdict by which stage decided (a box cull before exact roots). Enclosures may meet that bar. `carrier_touch::off_face` after roots, the dip-bound clearance then roots, and harmonic-else-sampled are fallbacks.
- A consumer that serves fewer pairs (`census::point::edge_face`) matches on the reading's answer and refuses the rest typed.

`carrier_cross`'s curve×plane parts and the census edge-face cut (`Uncrossable`) are in `fuse/the-curve-pair-reader`'s callers and are left to it. This row is the rest.

Moved from ROOTS to GERM on 2026-11-02, by `git mv`, when ROOTS merged into GERM: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so GERM, GERMCERT, ROOTS and APEX came in under one budget.

## Priced (GERM orchestrator, 2026-11-20)

P1: two or more implementations of one question (circle × plane in
about six places, line × cone in two) and wildcard arms that entrench
as each new carrier pair is built on them. `X`, 16 points: one reading
with an exhaustive `(CurveKind, SurfaceKind)` match, the wildcards
turned into typed arms, the fallback ladders retired or shown total,
and every consumer moved onto the reading — about two hard rows of
work, staged by a spec before any lane builds.

## Ratified (Ev, PR 7167, 2026-11-20)

C5's curve × face paragraph (`crates/geom-brep/README.md`) states the
design:
- a curve × surface pose: `Lies | Apart | Tangent | Through | Isolated`.
  `Through` is any point where the surface has no tangent plane.
  `Tangent` is `Isolated`'s pose-decided special case;
- one `curve_face::cross` through `route`;
- refusals that carry each arm's own undecided windows, read against
  the trim every time;
- every door in geom-brep.

Ev also ruled that a spline span lying within the band over part of
its length refuses as an undecided window for now; a run meeting is
not read. Staging: U1 moves the line roots door with no behaviour
change; U1b gives line × cone one rule set; U2 moves the ConicQuadric
door before the first-harmonic arm's Part 2; U3 builds `pose`, `cross`
and the windows and moves the small consumers onto them; U4 merges the
sweep's two lanes; U5 moves the remaining readers. The `design` flag
was never set on this row; the U0 PR settled the shape.

## Progress (2026-11-21)

- **U0** (PR 7167): C5 ratified.
- **U1** (PR 7179): the line roots door is in geom-brep.
- **U1b** (PR 7184): one line × cone rule set.
- **U2** (PR 7209): the conic × quadric door is in geom-brep.
- **U3a** (PR 7226): `curve_face::pose` and `curve_face::cross` are built, and the trace and the census touch confirm read them. Its dual review found one class, cone readings that skipped the floor on narrow cones, now fixed across every cone rung.
- **Remaining:** U3b (the ring path, classify, `on_surface`, region; in flight); U4 (the sweep merge, which needs local windows plumbed first); U5 (the remaining readers).
- **U4 spec**: `docs/GERM-CROSSING-U4-SPEC.md` stages U4 as U4a (local windows), U4b (the face reading and the curved lane), U4c (the planar lane, `slit`, `factored_crossing`) and U4d (the spline arms, after a fork put to Ev on its PR).
- **U4a** (`germ/u4a-local-windows`): `pose` refuses `Undecided<T, PoseRefusal>`, its graze and line apex rungs local; the doors' methods answer `Walked`; the walk reads past refused pieces; `cross` drops windows off the span and merges the rest; the wrappers read the first window that meets the span. `pose::arc` and `plane_conic`'s roots re-posed, and a line's arc no longer held to half a turn. Closes `root-slack-meters-roots-outside-the-arc` and `a-circle-touching-a-torus-off-the-arc-escalates-the-arcs-side-decision`.
- **U4a review fix pass**: every window meets the span through one turn-aware reading (`curve_face::on_span`), so a window across a conic's seam holds its crossings at both ends. A line's apex window is local only once `Lies` is excluded. A taken meeting widens its window by `Kε`, and the placement's end windows are held to the span. Readers of one window read a fault first (`CrossRefusal::leading`). The safety fuzzes are back on drawn arcs, judged on the arc alone (`conic_oracle::arc_truth`).
