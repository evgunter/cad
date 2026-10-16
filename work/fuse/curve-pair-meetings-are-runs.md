---
id: curve-pair-meetings-are-runs
kind: unit
title: A curve pair meets along stretches, one meeting each: ends read first, a found meeting takes its stretch, a found root divided out; the census sweeps joined pairs unchanged
status: open
opened: 2026-10-16
priority: P1
cost: H
design: true
refs: [the-curve-pair-reader, an-unrecorded-curved-edge-meeting-is-unseen-at-tier-three-prime, two-coplanar-ellipses-have-no-meeting-method]
---


## Why

Stage (c3) of `the-curve-pair-reader` adds pass 5′: the census sweeps every curved edge pair for an unrecorded meeting through `curve_pair::meet`. Run over the workspace, it turns 31 rows red (lofted STEP imports, swept and filleted bodies) through 361 refusals. 355 of them are at a vertex the two edges share.

| Refusals | Pair | Fault |
|---|---|---|
| 336 | NURBS × line | `curve_pair_span_end_chord` escalates on walk points 1–7 nm off the joint (dyadic piece middles: the polish step was dropped) |
| 12 | circle × line, Interval | `curve_pair_pose_root_gap`: a tangent joint's root gap from √(R²−h²), with noise ≈ √ulp·R |
| 7 | ellipse × ellipse/circle | `Unsettled(Meter)` at fillet joints |
| 6 | coplanar ellipse × ellipse | the known frontier |

## The design (a designer pair, converged after two rounds; fork-log row 151)

- **The reader takes no topology.** A joint is read as the meeting of the two spans' ends. Tier 3 pins every carrier end within the zero band of its vertex, so a joint reads End/End on an exactly zero chord.
- **Ends first.** Each span's ends are read against the other span before any arm candidate, and a found end-meeting claims its run.
- **The ring rule.** A candidate within the band's escalation `e` of a found meeting can only be that meeting again, off, or an escalation. It is never a second meeting, so it is dropped. This applies to every found meeting: shared ends, unshared ends on an interior, and interior crossings.
- **A run holds exactly one decided meeting.** A run with none or several escalates, so no second crossing can hide in an undecided stretch.
- **Deflation.** A closed form divides a found meeting's root out and solves for the rest:
  - line×circle and line×ellipse: linear, by Vieta;
  - coplanar circles: the reflection across the line of centres;
  - a conic across a plane: linear;
  - coplanar ellipses: a cubic.
- **Fix the walk's dropped polish step** (`step_taken`/slack, or a NURBS foot clamping at the domain end).
- **`Meeting::run` and `run_candidates` go.** Every consumer filters them out.
- **The census is unchanged in shape.** Pass 5′ reads joined pairs like any pair:
  - End/End at a shared key is structural;
  - End/On is the vertex event;
  - On/On owes the edge-edge record.

## Left open

- The 6 coplanar-ellipse refusals: `two-coplanar-ellipses-have-no-meeting-method`.
- Check sweep's edge-edge record writers at tangent joints: confirm may start reporting stale records on pairs whose interiors meet only in a joint's run.
- File in geom-brep: the walk reads no span end as a candidate of its own.

## Ruling

Ev approved the C5 sentence on PR 5214 (2026-10-16) in Ev's own terms: a *stretch* is a longest interval of one span nowhere definitely off the other, a *meeting* a longest interval everywhere definitely on it; a stretch holding exactly one meeting reports it, one holding none or several escalates. Ev's condition: "presuming that this terminology doesn't contradict any code".

## Bringing the code to the terms (part of this unit)

The code does not yet use the ruled terms. Each of these is the code this unit changes anyway:
- `curve_pair::Meeting` is documented as "A point on both spans" (`crates/geom-brep/src/curve_pair.rs:94`), while a meeting can be a shared run, which the code carries separately as `Meetings::shared`. Document `Meeting`/`Meetings` in the ruled terms (a meeting is a point or a run; `shared` is a run meeting).
- `Meeting::run` (`curve_pair.rs:103`) and `pose::run_candidates` call a meeting's fringe its "run". Both go (above).
- `Meetings::points` states the old ring rule ("a point the zero band puts at a meeting already read is not one again", `curve_pair.rs:116`). Restate it as the stretch rule.
- `curve_pair/span.rs:5` calls a span "the stretch of the carrier an edge covers". Reword it ("the part of the carrier") so *stretch* names only the ruled term in the curve-pair reader.

## Status

The implementation lands with c3.
