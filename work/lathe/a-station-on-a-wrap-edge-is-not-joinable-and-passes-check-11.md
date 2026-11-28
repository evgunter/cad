---
id: a-station-on-a-wrap-edge-is-not-joinable-and-passes-check-11
kind: issue
title: A station on a wrap edge is not joinable, so check 11 passes a hand-split one-band seam at rest
status: closed
opened: 2026-11-23
priority: P1
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
cost: M
closed: 2026-11-26
pr: 7532
---

Found by the LATHE one-band lane (2026-11-23, PR 7414), measured on its
branch `lathe/full-revolve-one-band`.

## What

A full revolve's curved wall is one face, its one meridian a wrap edge
whose two half-edges both lie in that face's loop. `split_edge` at the
meridian's mid-height leaves a vertex on one carrier, between two pieces
of one edge's chart image: a station (`docs/DESIGN.md`, maximal edges).
The join's predicate (`crates/topo/src/boolean/edge_join.rs`, `joinable`)
reads the two pieces' sides and returns `None` where they are one face
(`if f == g || pair(s2) != (f, g)`), so it never takes the vertex; tier 3's
check 11 reads by that predicate (`crates/topo/src/validate.rs`,
`joinable_at_rest_errors`), so `topo::finish` passes the split body at
rest. With two π-bands each meridian had two faces, and the same split
was construction state, refused `JoinableVertexAtRest`.

Failing on the branch: `sweep`
`shell7_seam_corner::the_line_arm_carries_a_hand_split_drum_seam_to_its_foot_at_the_direct_door`
("a body holding stations is not at rest", `common/stations.rs`) and
`shell7_r2_probes::p4_split_edge_carries_its_childrens_pcurves` (expects
`JoinableVertexAtRest`, gets `Ok`). Likely the same cause, not yet
traced: `two_tubes_in_the_zero_band::a_rod_on_a_puck_with_its_seam_on_the_ruling`,
where the union's record of the rod's seam lying in the puck's top now
names the seam's piece between the rim's two touches (`x ∈ [−1, 1]`)
rather than the rod's whole length, as an edge left split at stations
on a wrap edge would, and tier 3′ adds an `UnrecordedContact`
edge–face overlap beside fewer undecided pairs.

## Exposure

A body holding a station on a wrap edge reads at rest, though the join
that finishes every door would leave the edge whole: two bodies of one
shape compare unequal, and the station reaches every consumer that
reads a wrap edge as one edge (the wrap tip reads, the seam-twice
meshing lane). A public door mints one: `topo::union` of a rod lying
on a puck with its seam on the contact ruling
(`two_tubes_in_the_zero_band::a_rod_on_a_puck_with_its_seam_on_the_ruling`)
returns the rod's one wall with its seam in three pieces, split at the
puck rim's two touches, in both operand orders, and the same rod
anywhere else holds one seam edge. A one-band regression from a public
door, so it gates the one band's landing.

## Ground

`crates/topo/src/boolean/edge_join.rs` (the predicate and the join's
Euler step over a loop that passes the station twice) and
`crates/topo/src/validate.rs` check 11, which follows it. The join is
BOOL's; claimed by LATHE as the one-band consumer gap.

Cost M: the join's kill over a vertex the one loop visits twice (both
half-edge pairs at once), its descriptions (the joined edge stays the
face's wrap edge), and the two rows back to construction state.
