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
`ClassificationInvariant`. It measures the operands and the result with
the certified quadrature, through the sign-level walk run to its
reporting target. That walk's face dispatch sends every spline face
(`Surface::spline_chart`) to the NURBS-patch quadrature lane, so a NURBS
operand whose patch lane encloses it is measured, not refused.

The scalar's policy decides whether the backstop runs at all
(`AtRestPolicy::gate_volume_backstop`). A dual runs nothing (DL3), so it
has no closed-form refusal to give here.

A props refusal at a certifying scalar surfaces in tier 3's reading
(`validate::classify_mass_props`), as one of two errors:
- `BooleanError::VolumeCorrupt { operand, source }` when the body's
  structure does not resolve;
- `BooleanError::VolumeUnmeasured { operand, source }` otherwise. For a
  NURBS face that is the patch lane's typed refusal, e.g. a trim it
  cannot enclose.

A sign the enclosures still leave open beyond the band at the last
round refuses `VolumeUndecided`.

Still unmeasured end to end, for the reason above (the containment
probe refuses first).
