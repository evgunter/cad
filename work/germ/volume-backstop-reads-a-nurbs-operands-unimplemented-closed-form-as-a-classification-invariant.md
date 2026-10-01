---
id: volume-backstop-reads-a-nurbs-operands-unimplemented-closed-form-as-a-classification-invariant
kind: issue
title: The volume backstop reports a NURBS operand's unimplemented closed form as a classification invariant on a planar body
status: open
priority: P4
cost: E
opened: 2026-09-29
refs: [nurbs-plane-section-has-no-component-arm]
---

## What

`ops.rs` `volume_backstop` computes the closed-form mass properties of
both operands and the result, and maps any refusal to
`ClassificationInvariant { what: "volume backstop: mass properties
refused on a tier-valid planar body" }`. The closed form has no NURBS
arm (`geom-brep/src/props/curved.rs`: `Surface::Nurbs(_) |
Surface::Approx(_) => Err(PropsError::Unimplemented)`, measured on a
bicubic bump block through `topo::mass_properties_structural`:
`Face { source: Unimplemented }`). So a union with a NURBS operand that
got past the join would refuse here as corruption on a planar body,
which it is neither.

Unmeasured end to end: today the join's containment probe refuses such
a union first (`nurbs-plane-section-has-no-component-arm`). The repair
is a typed capability refusal naming the kind, at the backstop.

## The backstop changed under this row (REACH, branch `reach/volume-backstop`, 2026-10-01)

`volume_backstop` no longer reads the closed form or raises
`ClassificationInvariant`. It measures the operands and the result
through the scalar's own lane (`AtRestPolicy::quad_lane`): at a
certifying scalar that is `topo::mass_properties`' walk, whose face
dispatch sends every spline face (`Surface::spline_chart`) to the
NURBS-patch quadrature lane; at a dual it is the closed form. Any props
refusal now surfaces as `BooleanError::VolumeUnmeasured { operand,
source }`, carrying the `MassPropsError` whole — at a dual a NURBS
face's `Face { source: Unimplemented }`. That is the typed capability
refusal this row asks for; still unmeasured end to end, for the reason
above (the containment probe refuses first).
