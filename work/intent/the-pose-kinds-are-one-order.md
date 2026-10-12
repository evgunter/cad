---
id: the-pose-kinds-are-one-order
kind: issue
title: "Stage 2 FORK-1b: the pose kinds are one order (Ev's lattice); the revolve holds its axis line"
status: closed
opened: 2026-10-08
priority: P0
cost: E
closed: 2026-10-08
refs: [d10-one-way-to-say-intent-is-unbuilt, operations-state-their-outputs, tube-spine-reads-an-axis-origin]
---

Raised by Ev on PR 4222, on FORK-1's split over the revolve's axis: "we have kind of a profusion of names for specific subsets of pose information. i wonder if it'd be possible to have something more like a lattice". It blocks unit A (`operations-define-output-variables`) only in what A carries for the revolve and the axis datums.

A designer pair converged over three rounds (`docs/DESIGN-FORK-LOG.md` row 86; reports on `design/intent-s2-fork1b-A` and `-B`):

- a pose is a frame known up to its kind's symmetry; the subgroups form the lattice, and the kinds are ordered by which determines which (the join of two kinds depends on the values, so it is a named construction, not a kind);
- D10's five kinds stay; an element with no reader (a point on an axis, a line in a plane) becomes a kind when a slot needs one;
- an incidence is constructed, never checked; a slot holds one kind, and a finer value is read through its projection;
- a 2-D value lives in the node that holds its frame; the revolve holds its axis line and defines `body` and `axis`;
- unit A: no `AxisInPlane` kind, both axis datums define `Axis`, `Revolve` has two ports; unit B: the tube reads a `Frame` (`tube-spine-reads-an-axis-origin`).

The text is in PR 4222's diff (D10 Variables, Operations and Coincidence; ASSEMBLY A11 (1)). Ev's answer closes this row.

**Ruled (Ev, 2026-10-08, PR 4222):** approved, adding "ideally the mates' `Subgroup` stuff can literally be shared, at least in part": one `Subgroup` type is a pose's symmetry and what a mate folds, growing a `Point`'s and a `Direction`'s when a reader needs them. Unit A carries no `AxisInPlane` kind, both axis datums define `Axis`, and `Revolve` defines `body` and `axis`; unit B gives the tube a `Frame`.
