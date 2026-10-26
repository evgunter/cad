---
id: a-two-surface-conventional-seam-never-rejoins
kind: issue
title: edge_join re-joins an OnPair seam only along an iso family of one of its charts; off-iso pairs and pair-beside-tangency pieces stay two edges at rest
status: open
opened: 2026-10-12
priority: P1
cost: E
---


## Finding

Filed by the TANG orchestrator from the five-conventions designer pair.

`topo::boolean::edge_join`'s `locus_of` (in `crates/topo/src/boolean/edge_join.rs`) admits a `Chart` edge only when
`sf == sg && chart.surface == sf`. A conventional edge between two
distinct surfaces is therefore never joinable. That is a smooth seam the
must-carry rule read as under-determined: a G2 profile join, or a
tangent piece too short for its second order. So the two halves of such
a seam cut at a vertex stay two edges at rest, against D1's maximal
edges.

`curved-joinable-vertices-are-left-unjoined` (closed, PR
4233) covered intersections, tangencies and one-surface `Chart` seams,
not this case.

If Ev takes decision 1 of PR 4907 (such a seam stores
`OnPair { pair }`), the fix is a same-pair `Locus` arm beside
`Intersection`'s and `TangentIntersection`'s. If not, the arm reads both
pieces' `Chart` as one locus on the pair.


## What landed, and what is left

Decision 1 was taken, and TANG's `OnPair` PR (`tang/onpair-description`) added the
same-pair arm: two `OnPair` edges on the faces' pair join where both
carriers lie on one iso family of the first of the pair's charts that
has one (`Locus::PairIso`, `edge_join::locus_of`), and the joined edge
is asked the must-carry rule again over its own span
(`geom_brep::reask_pair` in `joined_spec`), so a join that lengthens
the seam past the rule's band refuses rather than store a pair it no
longer is (`sweep` `wedge_end_doors::a_join_of_two_pair_pieces_re_asks_the_joined_seam`).

Still unjoined, so still against D1's maximal edges:

- a pair whose carriers lie on no iso family of either chart;
- an `OnPair` piece beside a `TangentIntersection` piece of one locus
  (the short end `split_edge` leaves of a tangent seam): two `Locus`
  values, never one;
- plane × plane `OnPair` (two coincident plane records): the planar fast
  path returns before `locus_of` and admits only `Intersection` — its
  own row, `a-plane-plane-pair-never-rejoins`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: `edge_join` joins some seam loci and leaves the rest two edges against D1's maximal edges: a special case that should be handled uniformly, P1 by `work/README.md` Priority.
- Moved to TANG (`work/tang/`) by `git mv`, id and body unchanged: which filed it: a seam's rejoin is TANG's ground (`edge_join`'s loci and the must-carry rule). CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
