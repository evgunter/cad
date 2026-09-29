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
