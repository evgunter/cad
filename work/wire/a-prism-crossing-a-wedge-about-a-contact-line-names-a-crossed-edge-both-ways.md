---
id: a-prism-crossing-a-wedge-about-a-contact-line-names-a-crossed-edge-both-ways
kind: issue
title: A prism crossing one wedge about a contact line refuses Naming(Emission): a crossed edge's pieces read both sides of the other operand
status: closed
opened: 2026-10-08
priority: P2
cost: M
closed: 2026-10-18
---

## What

Found by the sweep of `three-solids-touching-along-one-line-refuse-their-union`
(program tang), measured on 8fd03fce and again on its branch: the same
orders refuse on both, so the topo change does not reach them.

The plate `[0,3] × [0,2] × [0,1]` and three upright prisms over
triangles from (1.5, 1) to the points 0.4 from it at the bearings
0°–50° (z 0.5–2.0), 120°–170° (z 0.47–1.7) and 30°–60° (z 0.44–1.81).
The first two touch along the vertical line through (1.5, 1); the
third crosses the first's wedge about that line and touches the
second's edge along it.

Through editor-core's union (`crates/editor-core/tests/union_pinch_member_order.rs`'s
`sector_prism`, `crate::fixture::union_over`), the member orders
[2, 1, 3, 0] and [1, 2, 3, 0] (0 = the plate), and [1, 0, 2] and
[0, 1, 2] for the prisms alone, refuse `Naming(Emission { "a crossing's
edge has pieces on both sides of the other operand on one side of it"
})`, raised by `names/emit_topo.rs` `side_in_body`. The other 22 (4)
orders build; topo's union builds every order sound
(`crates/topo/tests/three_solids_on_one_line.rs`,
`a_third_prism_crossing_the_wedges_about_the_line_builds_or_refuses_typed`,
"one wedge crossed at 30°": 3/3′, closed-form volume, the analytic
oracle). The refusing orders fold the crossing prism onto the two
touching ones, so its edge along the line meets the operand's two
coincident edges there. Unmeasured reading: one side of a crossing
vertex has pieces of the two edges classed against two wedges, In
for the one it crosses and Out for the other, where `side_in_body`
assumes one side reads one class.

## Measured (wire-prism, on 92719ced)

Every order builds and names on main: the 24 with the plate and the 6
without publish one table, pinned by
`union_pinch_member_order::a_prism_crossing_one_wedge_about_the_line_publishes_one_table_in_every_member_order`.
The four Emission refusals reproduce on 8fd03fce exactly as filed; a
first-parent bisect puts their end at PR 4816 (`emit/touch-crossing`,
23de7f9acf).

The unmeasured reading is refuted. Instrumenting `senses_at` on
4816's parent (c25ec8890a) shows the refusing vertex is no crossing:
it is operand A's own corner, the first wedge's bottom apex at
(1.5, 1, 0.5), which lies on the crossing prism's edge along the line.
The mixed side is of ONE edge, that prism's lateral edge (operand B):
its pieces ending at the vertex were classed `On` and `Out` (not `In`
and `Out` against two wedges), each row repeated at two B keys at the
point. The identity pass then tried only the kept key and its fusion
partners, so A's corner key, reached only through
`operand_vertex_keys`, went unread and the vertex fell through to
`senses_at`. PR 4816 reads the identity at every operand key the
boolean leaves at the point, so the vertex is now that wedge's
start-cap vertex in every order; and on main no vertex of this fixture
carries a mixed side at all (1806 vertex reads instrumented, none with
two classes on one side), so `side_in_body`'s rule (one side reads one
class, else refuse loud) is the boolean's contract, not the defect.

## Resolution (2026-10-18)

Overtaken: fixed on main by PR 4816 (`emit/touch-crossing`, merge
23de7f9acf), which reads a vertex's identity at every operand key at its
point (bisected by the `wire-prism` lane, PR 5302). Pinned by PR 5302's
`union_pinch_member_order::a_prism_crossing_one_wedge_about_the_line_publishes_one_table_in_every_member_order`.
