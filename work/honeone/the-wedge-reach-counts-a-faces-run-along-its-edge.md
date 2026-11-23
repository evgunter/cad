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

Moved from TWICE to HONEONE on 2026-11-20, by `git mv`, when TWICE merged into HONEONE: TWICE held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one HONEONE carries.


## A third reader: the embedding certificate, in every cell near an edge (EMBED, lane `embed/remeter`, 2026-11-22)

Since PR 7329 `topo::injective`'s departure reading levers its sine at
this reach (`arm_of`, through `wedge_reach::reach_over`, the recipe
`WedgeReach::reach` is built by), in every cell pair near a shared
edge. So the run along the edge now signs pairs in the embedding
certificate, where the cell's own span used to leave them undecided.

Measured by PR 7329's review and pinned as the code reads it
(`injective::margin_reach_rows::the_arm_counts_a_strips_run_along_its_edge`):
two plane strips 4 m along their shared edge and 0.05 m across, folded
by 18ε, so their far edges part by 9.0e-10 m everywhere, less than ε.
The reach reads 2.000625 m; the sine times it is 3.6e-8 m against
K·ε = 1e-8 m; `sign_on` signs the strip apart over its whole width. At
the strips' run away from the edge, 0.05 m, it decides nothing. On main
before 7329 the pair was undecided.

No wrong body is known: a strip that never leaves the band has a far
edge within the band of the other face, and that pair refuses; tier 3
over-reads the same way and gates first.

**Proposed band: P1.** The row was banded for tier 3 and the profile.
The embedding certificate is the at-rest gate's (EMBED's P0 unit), and
there this reach is the whole of what makes a definite sign definite.

The reach's site is not moved here: where a turn's lever is read from
is ONEROUND's open fork (`oneround/turn-lever`, PR 7262).

- 2026-11-22 — **From EMBED: Ev's ruling on PR 7456 relies on this row's proposal.** `[topology.tiers]` now has the embedding question lever its cell reading at how far the narrower face runs from what the faces share, "measured across it". If the reach stays measured from the edge's middle, half a long edge's length can still exceed a narrow face's width and the over-read remains on the narrower face too. The farther-face-against-narrower-face choice is not this row's question; it is filed for check 4 as `work/restfront/check-4-levers-a-wedge-at-the-farther-faces-run-so-a-narrow-face-inside-the-band-reads-a-crease.md`, and the question's own side is built by `work/embed/the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict.md`. (EMBED orchestrator)
