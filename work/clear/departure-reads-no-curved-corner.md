---
id: departure-reads-no-curved-corner
kind: issue
title: The injectivity check reads a curved shared edge only where it is a cell's one boundary edge, so a corner where it meets another edge refuses, and a cylinder-cylinder tee cannot be certified embedded
status: closed
opened: 2026-10-29
priority: P1
cost: H
branch: clear/corner
pr: 6045
closed: 2026-10-30
---

## Finding

From CURVEDEP (`departure-presents-no-curved-shared-edge`).

`topo::injective`'s curved presentation (`present_curved`) reads a curved edge only where it is the one boundary edge in reach of a cell.
- The column argument behind it needs the curve to be the only boundary in the rectangle it presents.
- A cell holding a vertex where a curved shared edge meets another edge is therefore read by no presentation, at any depth.
- `beside_curve` refuses it `Structural` (both faces then refuse `Uncertified { Departure }` at once, never at the floor).

The boolean tee `crossing_equal_rods` builds at spins `(0, 0)` (rod `A` mitred by `z = x`, united with rod `B` from `x = 0.2`) refuses so in 36 splits. Its junction is arcs of the bisector ellipse meeting the rods' lines and rims at vertices. The pin is `a_cylinder_cylinder_tee_refuses_typed_at_its_curved_corners` in `crates/sweep/tests/self_intersection_proves_injectivity.rs`.

The ordinary oblique cut refuses the same way. R1 of the CURVEDEP review executed two:
- the `prism_at` rod minus a slab tilted π/5: `Uncertified { Departure }` after 6 splits;
- `extrude(profile::circle)` cut at π/7: the same, after 42 splits.

In each, the cut leaves the wall split in halves, so the ellipse is two open arcs that meet the halves' seam lines at corners. Every curved edge CURVEDEP certifies is closed (the pipe's ellipse, the fillet's rim circle, the mitre's ellipse). The kernel-built cut is pinned as refusing typed by `a_kernel_built_oblique_cut_refuses_typed_at_its_curved_corners`, same file.

**SHELL-4 and this row (corrected at close-out).** The vessel sentence
first written here was wrong. The vessel already held at base, with
`cornered = 0`: `departure`'s carrier arm uses the disc's plane implicit
as ψ against the wall's iso rectangle, so the disc's half-rims are never
presented as curves. SHELL-4's bodies did not need this row: the tube's
rims are closed and the vessel's are read by the carrier arm. Only the
OFF-D dumbbell's curved twin, not yet built, would meet a curved corner.

The iso corners (`convex`, `zero_of`'s `Corner`/`Point`) are the existing reading for two coordinate lines; nothing reads a corner with a curve in it.

## What a fix needs

A presentation of a convex corner whose sides include a curve. One candidate: the region on the face side of the curved edge's closed form continued past the vertex. It holds the face's part of the cell where every other boundary curve in reach lies on its face side, and that must be decided at the vertex itself, where the other edge meets the continuation.

Equal-radius tees also pass through the walls' tangent points when the junction reaches the node plane; there no first-order separator is strict, which is `the-second-order-arm-certifies-a-declared-wedge-0-edge`'s ground or a node arm of its own.

## Closed

CORNER (#6045), with this commit last. The departure arm certifies across
a convex corner whose sides include a curved shared edge. The face is
presented by the shared edge alone, read through the vertex V: a curve
continued by its closed form, or a coordinate line. Each step of the
soundness argument is decided at a named site:
- `self_intersection_corner_departs`: the other edge leaves V into the
  face side and stays there inside R;
- `self_intersection_corner_vertex`: O starts on V, within the band (the
  6th ε concession).

Results:
- The kernel-built oblique cut and the π/5 rod hold with 14 corner
  certificates.
- The tee refuses typed at 9 root pairs in three kinds of corner
  (`departure-reads-no-curved-corner-off-its-shared-edge`, CLEARTAIL).

Review: a DUAL pair at `45cf1e4507` (byte 137, tally 0; the bilateral
MAJOR: "stays" unpinned), fix pass 1, a focused review, and fix pass 2
(`c140ecc3`), which the orchestrator read and accepted.
