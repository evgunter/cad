---
id: a-ringed-spline-face-has-no-volume-lane
kind: issue
title: A spline face holding a ring has no volume lane, so a boolean that walls an island off a lofted wall refuses VolumeUncomputable
status: open
opened: 2026-11-25
priority: P2
cost: M
---


Filed by INSIDE's `inside/ray-spline-face-arm`, whose ray test lets the
booleans below reach the result's measurement for the first time.

`topo/src/props.rs` (the per-face arm that reads a face's loops, at
`reads_rings`) refuses `MassPropsError::RingOnCurvedFace` for any face
with more than one loop whose surface is not a cone, cylinder, plane,
sphere or torus: a NURBS or `Approx` face holding a ring has no volume
lane. Measured at ε 1e-9: `sweep`
`a_partner_across_a_lofted_wall_is_joined_on_its_section_value`'s brick
wholly inside the loft prism's flat wall (`prism ∪ brick`, `prism ∖ brick`)
and its two-pronged fork (`prism ∪ fork`) pass the join and the
inside/outside test and refuse `ResultInvalid { findings: NotFinished {
VolumeUncomputable { source: RingOnCurvedFace { face } } } }`, the face the
wall the island was walled off.

The row is whether the spline face's volume lane can read a second
loop (the analytic lanes read every loop's Green form), or whether a
walled-off island on a spline wall needs a lane of its own. Not
measured past the refusal.
