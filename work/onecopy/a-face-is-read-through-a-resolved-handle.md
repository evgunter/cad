---
id: a-face-is-read-through-a-resolved-handle
kind: unit
title: A face's loops, boundary, cycles and edges are read through one resolved handle (body.proven_face / linked_face → FaceView) in place of the source × shape grid of Body methods
status: dispatched
opened: 2026-11-03
priority: P1
cost: M
refs: [6460]
branch: onecopy/face-view
---


From the FULL review of PR 6460 (`onecopy/face-loops`, Style Q1/Q3 and claim 6). Parked until that PR lands.

## Why

PR 6460 put a face's loop walks in one home (`crates/topo/src/live.rs`). But that home is a grid of free methods on `Body`: a key's source (proven, linked, or unchecked) crossed with the shape read (loops, boundary, cycles, edges). The grid includes `proven_face_boundary` beside `face_boundary_linked`, `proven_face_cycles` beside `face_cycles_by_loop`, and five `face_cycles*` functions across the crate.

Nothing makes the resolving walk the shortest spelling, so a raw key plus a silent `get_loop(lk)?` stays one step away. PR 6460's guards are a source-text pin and a `pub(crate)` `loop_keys`, which are a ban rather than a shape.

## Fix

Resolve a face once, by its source, into a handle that holds `&Body`, and read every shape from the handle:
- `body.proven_face(k)` and `body.linked_face(k, holder)` return a `FaceView`;
- the view offers `.loops()` (with each loop's role), `.boundary()`, `.cycles()` and `.edges()`.

The source decides once how a miss reads. Every shape reads from a resolved face, so walking with a silent resolve is no longer the easy path. Validators keep an explicitly unchecked reader.

Retire the grid and the `face_cycles*` family onto it. Check `crates/topo/README.md` (and DESIGN's D2 table) for ratified text on these names first. If a clause fixes them, this is a question for Ev rather than a unit.
