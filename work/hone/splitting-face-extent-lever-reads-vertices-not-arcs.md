---
id: splitting-face-extent-lever-reads-vertices-not-arcs
kind: issue
title: splitting's face_extent lever must over-estimate a face's reach but folds only its boundary VERTICES, so an arc bowing past them under-claims the arm
status: open
opened: 2026-10-06
priority: P2
cost: E
refs: [an-edges-extent-has-four-dispatchers]
---


Found by reading, during the `shell/planar-gate-misses` lane's sweep for
extents folded from boundary vertices (the shape of
`shell-clearance-footprint-reads-vertices-not-arcs`). Not witnessed by
execution.

`face_extent` (`crates/topo/src/splitting/rules.rs`) is the lever arm
for the split's coplanarity and sense predicates, and its docs state the
obligation: the arm *must be an OVER-estimate* of the displacement a
normal-angle error induces across the face, because an under-claimed arm
shrinks a levered margin toward `Zero` — "a wrong answer, not a loud
one". It folds `|p − p_base|` over each half-edge's START vertex and
nothing on an edge's interior. A face bounded by circle or ellipse arcs
reaches past its vertices: a cap whose rim is one closed circle on a
single vertex reads an arm of zero from that vertex while the cap
reaches `2r`, and an arc bowing outward between two vertices carries
region past both. Callers:
`splitting/rules.rs` (two sites) and `chord_join.rs` (two sites).

A fix folds each curved edge's extent in through the one door
`an-edges-extent-has-four-dispatchers` names. Until that lands it is
`splitting::containment::carrier_ball`, which the shell clearance gate
reads (`shell::carrier_box`).
