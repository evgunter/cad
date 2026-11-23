---
id: a-vf-record-defers-a-whole-curved-face-pair-at-the-backstop
kind: issue
title: The backstop's v-on-f deferral (planar_face_bridged) defers a whole curved x planar face pair on one recorded vertex, so a non-convex curved face meeting the plane elsewhere is examined by no arm (unmeasured, argued)
status: open
opened: 2026-10-10
priority: P3
cost: M
---


Unmeasured, argued. Raised as NOTE-1 by the review of PR 4559
(`contacthold/census-edge-pass`). The argument below is that NOTE-1
written down at the code, not a measurement.

`census::sweep_cross_solid_backstop`, the curved × planar arm: a pair
`(a, b)` is skipped when `planar_face_bridged` holds, i.e. some
`(v, f_planar)` v-on-f record names the planar face and `v` is a
boundary vertex of the other face (`vf_deferred`, "the declared
interface — confirm pass's pair"). The deferral is per FACE PAIR, but
its evidence is one VERTEX:

- the confirm pass (`confirm_vertex_on_face`) checks only that the
  recorded vertex lies on `f_planar`;
- the exact sweeps examine planar entities and line edges, so they never
  read the curved face's interior or its curved boundary edges.

A curved face that rests on the plane at a recorded vertex and also
reaches the plane somewhere else (a wall that bows back down, or a curved face whose boundary
leaves and re-enters the plane between vertices, or a convex one: a
full cylinder lying on its side, sunk partway into a plate with one rim
vertex exactly on the plate top, whose genuine v-on-f record defers both
(wall, plate) and (cap, plate)) has that second
meeting examined by no arm: the backstop defers it, the confirm pass
reads one vertex, and the sweeps cannot see a curved face. On that
reading the census clears a meeting it never examined, which is the
unsound direction.

This deferral is also the one route by which the line × conic touches
of the closed `census-edge-pass-reads-no-line-conic-crossing` could go
unread. For a convex wall, the non-tangent face at a touching line edge
cannot hold a wall vertex without penetrating the post; for a non-convex
one that is unverified. `tier3_tests::line_conic_review` goes red if a
deferral reaches those seats.

Wanted: a measurement first, convex and non-convex. The sunk lying
cylinder above, and a curved post touching the plate at a vertex and
again at an interior tangency, each with the vertex's v-on-f record; run
`validate_pseudomanifold`. If it clears, narrow the deferral to what the
record evidences (the vertex's neighbourhood), or make the deferred pair
take the curved arm's verdict.
