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


## Reachable through a door since the profile's joint pass reads its turn (2026-10-19)

`paths/joint-reads-its-turn` (Ev's ruling on #5354) made the profile
decide a joint's tangency by its turn levered by the longer of the two
legs' arms (`seg::joint_tangency`, `path_junction_turn`): the 2-D
twin of this reach, without the run along the edge. The two now
disagree through `extrude`, measured at the default ε: a line 0.05 long
into an arc of radius 1 sweeping 0.05 rad whose start heading is turned
`φ` off the line's, closed by two lines.

| φ | profile | extrude, depth 1 | extrude, depth 4 |
|---|---|---|---|
| 10ε, 15ε, 19ε | a decided tangent joint (`φ·0.05` ≤ ε) | `SliverJoin`, `dihedral_wedge` 5.0ε, 7.5ε, 9.5ε | 19ε: builds, a sharp edge |

The strut runs along the depth, so the reach reads about `depth/2`
where the walls separate only across their 0.05 m width: the profile
records a tangency the extrude refuses as a sliver or, deep enough,
builds as a corner. Before the turn reading the profile read this joint
on its carriers' clearance (second order), so it was tangent there too;
what is new is that both readers now read the same first-order turn,
so the only thing between them is the lever, and this over-statement
is the whole of the difference.
