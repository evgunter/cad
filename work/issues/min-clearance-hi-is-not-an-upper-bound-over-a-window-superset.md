---
id: min-clearance-hi-is-not-an-upper-bound-over-a-window-superset
kind: issue
title: min_clearance's hi is an upper bound over the carrier windows, not over the faces, so the enclosure can exclude the truth
status: open
opened: 2026-09-08
---


## What was measured

`crates/editor-core/tests/r2_m10_6_probes_interval.rs`'s notched pair —
a C-shaped solid and a block 0.1 m apart — evaluates
`MeasurePrimitive::MinClearance` at `Interval` to

```text
notched pair: min_clearance = [0, 0.02576941016012847], true solid separation 0.1
```

The enclosure does not contain the true value.

## Why

`clearance::window_of` gives a planar face the rectangle its boundary's
AABB projects to on the carrier's frame axes
(`crates/editor-core/src/clearance.rs`, the `Surface::Plane` arm of the
`(u, v)` match). For a NON-CONVEX boundary — the C's cap — that
rectangle covers points that are not on the face, here the notch the
block sits in. A minimum taken over a superset of the two faces is at
most the minimum over the faces, so BOTH ends of the enclosure move
down: `lo` collapsing to zero is the direction the row already
documents and is sound, and `hi` landing below the truth is the same
mechanism in the direction nothing guards.

**This is not new, and it is not the frame's.** Until PROPS's sign-hull
unit the same row read `hi ≥ 0.1` and the assertion `truth <= hi`
passed. The window is the boundary AABB projected on the frame's axes,
so changing the carrier's stored `u_ref` changes WHICH superset of the
face the window is — and which of two supersets happens to carry the
closer spurious pair is a coincidence, not a property of the engine.
The unit's re-derivation of that assertion into a printed measurement
is what surfaced it; the defect predates the unit.

## What a fix would look like

Either trim the window to the face (a parameter-space boundary rather
than a rectangle hull, which is the same problem `mesh`'s planar
triangulation solves for a different consumer), or state on the
measure's own door that a `min_clearance` enclosure's `hi` is an upper
bound over the carrier WINDOWS and is not claimed to enclose the true
minimum over the faces — in which case every consumer reading `hi` owes
a look. The first is M10's; the second is a doc change with a
correctness question inside it.

## Home

`crates/editor-core/src/clearance.rs` (`window_of`'s planar arm) and
the `MinClearance` measure that reads it. Found by PROPS's sign-hull
unit; reported rather than fixed, because the window model is M10's.
