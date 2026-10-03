---
id: the-rim-routings-sense-guard-has-no-finished-fixture
kind: issue
title: The rim routing's second-sense guard has no finished fixture: its row reversed a torus's walls, which the at-rest gate refuses
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [boolean-door-adopts-the-finished-body-type]
---


Found by REACH's `boolean-door-adopts-the-finished-body-type` (the
boolean door takes finished operands).

`verify_tangent_declaration` (`crates/topo/src/boolean/mod.rs`) hands
`classify_shared_rim` one sense bit per declared face: two adjacent
`bool`s, so a call site passing the first face's bit twice compiles.
`sweep/tests/mate7a_torus_rest.rs`'s
`the_rim_routing_reads_the_second_faces_sense_and_not_the_firsts`
guarded that by reversing the outer torus's walls with
`Body::set_face_sense`. Those walls then face inward, and the body is not
a finished body: `AtRestBody::validate` refuses `CurvedSenseInverted` on
exactly the reversed walls, which the row now pins. So no row guards
the transposition.

What closes it: a finished fixture whose two rim faces carry different
sense bits with correct outward normals (a torus charted the other way,
its sense `false`), through `union_with` with the `Tangent` claim, so a
door reading the first bit twice answers wrong; or a unit row on
`classify_shared_rim`'s caller inside `topo`.
