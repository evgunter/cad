---
id: clearance-window-tightening-needs-chart-boundary
kind: issue
title: Clearance windows are bounding rectangles: tightening needs the face boundary in chart coordinates
status: open
opened: 2026-09-03
---

## What

M10-5's clearance engine subdivides each face's **carrier window** — a
conservative superset of the face's trimmed region
(`crates/editor-core/src/clearance.rs:41`, `window_of` at
`crates/editor-core/src/clearance.rs:1314`). A planar face's window is
its bounding rectangle; a cylindrical face's window is the whole turn at
the face's axial span.

The looseness runs the safe way for a defect gate — `Holds` covers
strictly more than the faces and is therefore sound about them — but
`Violated` can report a sub-`c` approach at a place neither face
occupies, and the unit ships that as deviation D3.

## How big it actually is, measured

Two shapes, both pinned in
`crates/editor-core/tests/m10_5_r2_probes_interval.rs`:

- **A non-convex planar face.** An L-shaped bottom cap's window is the
  full bounding square, which covers the notch. A block parked in the
  notch stands 0.45 m from the FACE and 0 m from the WINDOW, so a bound
  of 0.3 — which the two faces satisfy with 50 % to spare — is reported
  `Violated`, with a witness `(u, v)` landing where the body has no
  material. This is not a rounding-scale near-miss: the error is 150 %
  of the bound.
- **A coplanar pair.** Two coplanar faces of one body have windows that
  overlap in the carrier's own parameters even when the faces are metres
  apart, so the reported separation can be 0 where the real one is not.

Both are reported, never missed, which is why the unit ships with them.

## What a fix needs

The face's boundary in CHART coordinates — the pcurve layer's
description work. With it:

- a planar window can be intersected with the loop's own 2-D extent, and
  a cell that falls entirely outside the trimmed region can be dropped
  rather than classified;
- a cylindrical window can be cut to the face's real angular span
  instead of the whole turn.

Neither is a change to the funnel or to the receipt identity: the
subdivision would simply start from a smaller set and drop cells the
boundary excludes. What it is NOT is a tolerance — a cell is dropped
only when the boundary description certifies it outside, so the
direction of the looseness is preserved.

## Home

`work/m10/` — the code is `crates/editor-core/src/clearance.rs`, an M10
deliverable; the dependency (`pcurve`-layer chart boundaries) is not
scheduled in M10, so this is a consumer waiting on it.
