---
id: the-wedge-reach-counts-a-faces-run-along-its-edge
kind: issue
title: The wedge reach is measured from the edge's middle, so a face's run along the edge counts as lever
status: open
opened: 2026-10-16
priority: P2
cost: M
---

`topo::wedge_reach` (`crates/topo/src/wedge_reach.rs`, `WedgeReach::reach`) measures each face's boundary vertices from the edge carrier's middle. Two planes meeting along a line separate only with distance FROM the line, so a face that is long along the edge's direction and narrow across it reads its length as the lever: a strip `L` long and `w` wide at dihedral `θ` to its neighbour opens `w·sin θ` but reads `(L/2)·sin θ`. That over-states, the unsafe direction for a lever.

It was as true of the edge's own extent under the old lever (the edge's chord is its run along itself). It needs a long thin face at an in-band angle to its neighbour along its long side, and every extrude and revolve of a profile with a near-tangent joint makes one: the section below reaches it through both doors.

The fix measures each boundary point's distance from the edge's carrier (for a line, the perpendicular distance) rather than from its middle, or bounds it below by that. Found by the U-lever review (PR 5204, finding m5).


## Reachable through a door since the profile's joint pass reads its turn (2026-10-19)

`paths/joint-reads-its-turn` (Ev's ruling on #5354, and its fix pass)
made the profile decide a joint's tangency by its turn levered by the
shorter of the two legs' arms (`seg::joint_tangency`,
`path_junction_turn`): the 2-D twin of this reach, without the run
along the edge. The two disagree through `extrude` and `revolve`,
measured at 1e-9, 1e-6 and 1e-12 alike (#5377's fix pass): a line 0.05
long into an arc of radius 1 sweeping 0.05 rad whose start heading is
turned `φ` off the line's, closed by two lines. Both legs' arms are
0.05, so the shorter-arm reading changed nothing here.

| φ | profile | extrude, depth 1 | extrude, depth 4 | revolve 20 m off the axis, full or 1 rad |
|---|---|---|---|---|
| 5ε, 10ε | a tangent joint (`φ·0.05` ≤ ε) | `SliverJoin`, `dihedral_wedge` `φ·0.50` | `SliverJoin`, `φ·1.0` | `SliverJoin`, `φ·1.0` |
| 15ε, 19ε | a tangent joint | `SliverJoin`, `φ·0.50` | builds, tier 3 passes: a sharp edge | builds, tier 3 passes: a sharp edge |
| 21ε and up | the pair pass escalates the line's clearance from the arc's carrier | | | |

The strut runs along the depth, and the revolved rim runs about the
axis, so the reach reads the face's run along the edge where the walls
separate only across their 0.05 m width. The profile records a
tangency the sweep refuses as a sliver or, once the reach clears the
band, builds as a corner: at depth 4 and in the revolve, `φ = 15ε` and
`19ε` build a sharp edge on a joint the profile declared tangent, with
nothing refused. Before the turn reading the profile read this joint
on its carriers' clearance (second order), so it was tangent there too.
What is new is that both readers now read the same first-order turn,
so the only thing between them is the lever, and this over-statement
is the whole of the difference. `a_revolved_wall_arc_join_reads_its_reach`
(`crates/sweep/tests/`) pins the refusing rows of the revolve.
