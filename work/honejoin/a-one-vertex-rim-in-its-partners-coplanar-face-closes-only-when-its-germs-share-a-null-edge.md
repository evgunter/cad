---
id: a-one-vertex-rim-in-its-partners-coplanar-face-closes-only-when-its-germs-share-a-null-edge
kind: issue
title: A closed one-vertex rim lying in its partner's coplanar face joins at one site only when its two germs share a null edge, so a peg in a pocket tangent to its rim refuses UnpairedLooseEnds in both differences
status: open
opened: 2026-11-11
priority: P2
cost: M
refs: [shared-vertex-crossings-that-tie-or-interleave-are-unprobed]
---


Found by GAUGE's shared-vertex pairing fork (`shared-vertex-pairing`,
designers B and C, round 3) and pinned by its build (PR 6832).

## What

`join::one_site_segments` (`crates/topo/src/boolean/join.rs`, near
line 963) takes a one-site record (`join::one_site`, near line 895):
a record both of whose germs name one curved locus pair, so they can
only match each other. A closed rim with one vertex, lying in its
partner's coplanar face, joins there only when the insertion put its
two germs on one null edge. Otherwise the germs stay open and the join
refuses `SplitJoinError::UnpairedLooseEnds` (`join.rs`, the leftover
count near line 1850).

Witness, on main and on PR 6832 alike:
`a_closed_conic_in_the_partners_face::a_peg_in_a_pocket_tangent_to_its_rim_builds_its_unions_and_intersections`
(`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`). It is a
block with a blind pocket of radius 0.3, against a slab with a peg of
radius 0.1 tangent inside the pocket's rim.
- With one-segment rims, `A ∖ B` and `B ∖ A` refuse
  `UnpairedLooseEnds { count: 4 }` under every pairing start. The
  designers measured main's start, the Out start and the In start
  (which refuses `SingleSiteSectionLoop { count: 2 }`).
- The two-arc rims build both differences `SOUND`.
- The difference has one cone at the point (designer B), so no
  pairing puts the two germs of each rim on one null edge and one cone
  apart from the other: the pairing is not at fault.

A typed refusal on valid input: a row-2 limit. P2: a normal verb on a
tangent-rim pose the other forms of the same pose build.

## Owed

Close a one-vertex rim at its site from germs on two null edges, or
state why the cross pairing cannot close there, then flip the pin's
one-segment differences to `SOUND` at `va − vab` and `vb − vab`.
