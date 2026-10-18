---
id: the-wedge-reach-counts-a-faces-run-along-its-edge
kind: issue
title: The wedge reach is measured from the edge's middle, so a face's run along the edge counts as lever
status: open
opened: 2026-10-16
priority: P3
cost: M
---

`topo::wedge_reach` (`crates/topo/src/wedge_reach.rs`, `WedgeReach::reach`) measures each face's boundary vertices from the edge carrier's middle. Two planes meeting along a line separate only with distance FROM the line, so a face that is long along the edge's direction and narrow across it reads its length as the lever: a strip `L` long and `w` wide at dihedral `θ` to its neighbour opens `w·sin θ` but reads `(L/2)·sin θ`. That over-states, the unsafe direction for a lever.

It was as true of the edge's own extent under the old lever (the edge's chord is its run along itself), and needs contrived geometry, a long thin face at an in-band angle to its neighbour along its long side. It is not known to be reachable through a door: no row reaches it.

The fix measures each boundary point's distance from the edge's carrier (for a line, the perpendicular distance) rather than from its middle, or bounds it below by that. Found by the U-lever review (PR 5204, finding m5).

