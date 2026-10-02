---
id: split-gate-sphere-zone-folds-into-face-box-rule
kind: unit
title: Fold the split gate's sphere latitude-zone box into FaceBoxRule, so both box lanes and the gate read one sphere rule
status: open
opened: 2026-10-02
priority: P1
cost: M
refs: [sphere-operand-box-is-the-whole-ball, split-gate-refuses-a-whole-body-for-one-unarmed-face]
---


Filed by the split-gate lane (PR 3843), at its review's request.

`splitting/classify.rs` `gate_face_reach` boxes a sphere face by its
latitude zone: the axial slab over the window
`solid_contain::sphere_chart_trim` pins, met with the ball. It takes the
zone only when `geom_brep::props::boundary_material_sign` (the
iso-rectangle side reading tier 3's curved sense check runs) agrees with
the face's sense bit, since a rectangle's boundary also bounds its
complement. Everything else keeps `census::face_reach`, whose sphere arm
is `FaceBoxRule::WholeBall`.

That is a second sphere box rule, living outside `boxes.rs`, which is the
class `sphere-operand-box-is-the-whole-ball` names. **The unit:** move the
zone, with its side guard, into `FaceBoxRule` as the sphere arm, so that
`boolean::boxes::face_box`, `census::face_reach` and the split gate read
one rule (`the_two_box_lanes_agree_face_for_face` then pins it as it pins
the others). Then delete `gate_face_reach`'s special case. Expect the
census backstop and the boolean's candidate sweep to see tighter sphere
boxes. Re-measure what that unblocks rather than holding the old
refusals.

The same unit should weigh boxing in the carrier's own frame
(`reach/split-gate-reads-a-world-axis-box`), which the slab cannot do as
an axis-aligned box.
