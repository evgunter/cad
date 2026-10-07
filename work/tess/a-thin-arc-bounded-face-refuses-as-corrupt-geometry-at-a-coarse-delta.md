---
id: a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta
kind: issue
title: a valid thin arc-bounded planar face refuses tessellation as corrupt geometry at a coarse chordal delta
status: open
opened: 2026-10-06
priority: P3
cost: M
---



Filed by CARVE (PR 4187's sweep), outside its fence, and re-measured
at that PR's review.

## What

A C-shape (a counterclockwise region bounded by one convex 350° arc of
radius 1 and a clockwise polyline at radius `1 − w`, the `c_shape`
family of `crates/sweep/tests/m5_s10_face_sense.rs`), extruded by 1,
validates and certifies at tier 3. `mesh::tessellate(body, δ, tol)`
then refuses some `(w, δ)` with `TessellateError::Triangulation`. Its
sentence reads "the CDT rejected a point insertion on a face — a
non-finite or out-of-range chart coordinate, i.e. corrupt geometry",
and the body is not corrupt. Measured 2026-10-06 (`Tol::witness()`,
`check_mesh` on every mesh that built):

| w \ δ | 0.5 | 0.1 | 1e-2 | 1e-3 |
|---|---|---|---|---|
| 0.1 | Ok | Ok | Ok | Ok |
| 0.06 | Ok | Ok | Ok | Ok |
| 0.04 | Ok | **refused** | Ok | Ok |
| 0.02 | Ok | **refused** | Ok | Ok |
| 0.01 | **refused** | **refused** | Ok | Ok |

The PR 4187 reviewer measured the same refusals on base, so the cap
orientation change does not cause them. The result is not monotone in δ.
No mesh that built failed `check_mesh`, and in particular none failed
`MismatchedWinding`. The chord polygon's winding, which this row first
suspected (`chart_frame`'s Newell over the chord walk), did not fail
at any width tried: the π/4 cap on a chord's sweep (`chords.rs`) bounds
the sagitta, which keeps the chord polygon's area sign through w = 0.01.

## Suspected cause (not confirmed)

The outer arc's chords cut across the inner boundary wherever their
sagitta exceeds the region's width, so the CDT receives crossing
constraint edges. If so, the refusal is an honest "this δ cannot mesh
this face", worded as corrupt geometry, and the fix is either a δ
the face's own width bounds or a refusal that names the width and
offers the δ that meshes it.
