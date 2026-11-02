---
id: departure-presents-no-curved-shared-edge
kind: issue
title: The injectivity check's departure arm refuses at depth 0 where a shared edge is neither a chart iso nor a straight line, so a cylinder cut obliquely or a cylinder-cylinder T cannot be certified embedded
status: closed
opened: 2026-10-28
priority: P1
cost: H
refs: [self-intersection-drops-every-vertex-sharing-face-pair-globally, SHELL-4]
branch: clear/curvedep
pr: 5991
closed: 2026-10-29
---


## Finding

From the dual review of PR #5955 (SELFX), R1's m4.

`topo::injective`'s departure arm needs one of the two faces to present the shared edge:
- an iso on the chart side (`present_chart`), or
- a line on the plane side (`present_plane`'s `segment`).

When neither can, it refuses `Uncertified{Departure}` immediately, without splitting. That covers every body whose faces meet along a curve that is not a chart iso of either face: a cylinder with an oblique planar cut, a cylinder–cylinder T, a fillet's non-iso rim. All of them now refuse the injectivity question. Before #5955 the global vertex drop passed them vacuously.

The refusal fails closed, so it is correct. But `SHELL-4`'s cavity clones are exactly such bodies, so this row stands between the engine and that consumer.

## What a fix needs

A departure separator for a curved shared edge. Two candidates:
- the graph of the edge over a projection, as the Graph separator already does for a near edge;
- presenting the face by its pcurve-bounded region, not an iso clip.

Weigh both before building.

## Closed

CURVEDEP (#5991), with this commit last. The departure arm certifies a
touching pair across a CLOSED curved shared edge: the oblique-cut pipe,
a cylinder–cylinder mitre and a filleted drum all hold.
`present_curved` presents a face's cell as the region on one side of
the edge's chart curve, read as a graph over one chart axis. It covers
a plane's conic and a cylinder's harmonic row.

Open arcs that end at corners, which is the ordinary kernel-built
oblique cut and the tee, still refuse `Departure` typed. They wait on
`departure-reads-no-curved-corner` (P1/H), and so do SHELL-4's bodies.

Review:
- A DUAL concurrent pair at `b7394bd758` (byte 162, tally 0) found one
  bilateral blocking gap: lax guards went red on no row.
- Fix pass 1 added a red row per mutant. A focused review confirmed
  them and found six more surviving mutants.
- Fix pass 2 (`3470ea3b`) gave three of them red rows and argued the
  other three at their guards. It was read and accepted by the
  orchestrator.
