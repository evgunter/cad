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

## Since `reach/split-gate-reads-a-world-axis-box` (2026-10-03)

The gate now reads every unarmed face's reach in the plane's own frame
(`boxes::BoxFrame`, `census::face_reach_in`), and its special cases are
two exact closed forms rather than one slab: `classify::zone_extent` (a
sphere zone's support along a unit direction, the concave maximum of
`h·a + √(1 − a²)·√(r² − h²)` clamped into the window) and
`classify::torus_rect_extent` (a ring torus's chart rectangle, whose
best azimuth does not depend on the latitude). Both are per-coordinate
like every extent in `boxes.rs`, so they can be the sphere arm and the
torus window arm of `FaceBoxRule` as they stand, in either frame. The
fold should take both: the sampled torus window
(`boxes::torus_window_extent`) pays a subdivision charge that refused
cuts up to `3·10⁻³` of the size clear of a torus rounding at the gate,
and the closed form pays none. The torus form needs `R > r` decided
(`split_gate_torus_ring`); the bracket lane would read its `Span`
radii's ends.
