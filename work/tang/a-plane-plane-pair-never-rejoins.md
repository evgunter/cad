---
id: a-plane-plane-pair-never-rejoins
kind: issue
title: edge_join's planar fast path never joins an OnPair between two coincident plane records
status: open
opened: 2026-10-13
priority: P1
cost: M
---


## Finding

Filed by TANG's `OnPair` lane (`tang/onpair-description`), raised by its review.

`topo::boolean::edge_join::joinable` (`crates/topo/src/boolean/edge_join.rs`)
takes a planar fast path when both faces are planes: it joins only two
lines described as the `Intersection` of the two plane keys, and returns
before `locus_of`, so an `OnPair` between two coincident plane records
(the flush section boundary a split leaves beside a face lying in its
plane, `topo::splitting::finish::describe_section_boundary`; STEP's
coincident-records rung) is never joined. Two pieces of such a chord
cut at a vertex stay two edges at rest, against D1's maximal edges.

Live: `sweep` `split_edge_on_faces_poses::a_smooth_edge_at_the_zero_bands_edge_states_the_pair_and_builds`
builds exactly this neighbourhood (a section face flush with a prism
face, the edge between them `OnPair`).

The structure that decides "one carrier" there is the line: both edges
lines on the pair, the vertex between them, and collinearity decided
(the planar arm's own reasoning, two planes meet in one line, does not
hold for coincident planes, which share every line in them). A join on
that reading must ask the must-carry rule again over the joined span,
as the same-pair arm does (`geom_brep::reask_pair`).

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: the planar fast path joins `Intersection` and never an `OnPair`, against D1's maximal edges: a special case that should be handled uniformly, P1 by `work/README.md` Priority.
- Moved to TANG (`work/tang/`) by `git mv`, id and body unchanged: which filed it: a seam's rejoin is TANG's ground (`edge_join`'s loci and the must-carry rule). CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
