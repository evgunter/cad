---
id: plane-nurbs-limbs-reads-a-non-finite-plane-into-its-margins
kind: issue
title: geom-brep: plane_nurbs_limbs and certify_rung3 read a non-finite plane into their margins, so the refusal blames a margin, not the plane
status: open
opened: 2026-10-01
---


(SSI implementer `ssi-diag`, from the §5 sweep of PR "SSI: refusals
name their operand and limb, and end by the recourse table", which
refuses a non-finite operand at the SSI's `f64` doors.)

## What

`geom_brep::ssi::plane_nurbs_ssi`, `trace_plane_nurbs_uncertified`,
`cylinder_sphere_ssi` and `idealized_trace_r3` now refuse an analytic
operand whose stored datum is not finite as that operand's own fault
(`SsiError::OperandNotFinite`, `ssi.rs` `finite_operand`). Two doors
over the same operands do not:

- `geom_brep::plane_nurbs_limbs` (`crates/geom-brep/src/edge_nurbs.rs`)
  destructures `Surface::Plane { normal, .. }` and feeds the normal
  into the per-sample transversality (`normal_angle_sine`,
  `plane_nurbs_transversality`). A non-finite normal escalates there
  with an unreadable margin and ends as the transversality decision's
  (`PlaneNurbsRefusal::TransversalityEscalated`: "move the geometry so
  the faces cross at a clearer angle; an unreadable or collapsed margin
  may indicate a kernel bug"). A non-finite ORIGIN is not read there;
  it reaches `certify_rung3`'s limb 1 (`analytic_limbs`,
  `implicit_residual`), escalates, and ends as the on-locus limb's last
  resort, "loosen the tolerance". Neither names the plane.
- `geom_brep::ssi::certify_rung3` (generic over the certifying scalar)
  has the same exposure for every analytic operand, through
  `analytic_limbs` and `probe_tube_analytic` / `probe_tube_chart`. Its
  consumers forward its refusal through `edge_nurbs::refusal` and
  `pcurve_cache::ssi_refusal`, neither of which has a vocabulary for an
  operand fault, so a guard there needs an arm in both.

Not measured: whether any edge-certification caller can hand these a
non-finite plane today. At rest, `topo::validate`'s check 1
(`poisoned_datums`) reads a stored surface's poison, but whether it
runs before the edge certification on every path was not traced.

## Repair shape

Refuse the operand at `plane_nurbs_limbs`' door (and at
`certify_rung3`'s, with the forwarding arms), naming the plane and the
datum, as the SSI doors do.
