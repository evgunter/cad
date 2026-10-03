---
id: an-approx-face-on-line-edges-has-no-finished-fixture
kind: issue
title: The operand gate's Approx × Plane germ-pair refusal is reached by no finished body through the public door
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [boolean-door-adopts-the-finished-body-type]
---


Found by `boolean-door-adopts-the-finished-body-type` (finished operands
at the boolean door).

`sweep/tests/offc_r1_probes.rs`'s
`a_skinned_base_approx_face_earns_the_germ_pair_refusal` reached the
operand gate's pair-scoped `CurvedPairUnsupported { kind: Approx,
other_kind: Plane }` by setting a certified offset surface on a box
cap whose four edges keep their line descriptions
(`set_face_surface_stranding_for_tests`). That box is not a finished
body: `AtRestBody::validate` refuses `DescriptionNotAdjacent` on the
four cap edges, plus `ApproxCertification` and `Pcurve`, which the row
now pins. The finished Approx body the file also builds
(`a_boolean_against_the_twisted_approx_body_refuses_typed`) refuses
earlier, at the edge rule (`CurvedEdgeUnsupported`), so no row reaches
the Approx germ-pair arm end to end.

What closes it: a finished body with an Approx face whose edges carry
honest descriptions on `Line`-free or certified carriers, through the
public door, pinning the germ-pair refusal again; or a statement that
the arm is reachable only through the edge rule's lift.
