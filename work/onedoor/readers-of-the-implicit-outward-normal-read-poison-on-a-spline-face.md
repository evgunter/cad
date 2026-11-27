---
id: readers-of-the-implicit-outward-normal-read-poison-on-a-spline-face
kind: issue
title: Readers that call implicit_outward_normal on a face's carrier read poison on a spline face, an escalation or a kernel fault where it is a limit
status: open
opened: 2026-11-27
priority: P2
cost: M
refs: [other-readers-map-a-declined-face-normal-to-a-kernel-fault]
---


Filed by the LATHE declined-normal lane (`lathe/declined-normal-readers`,
2026-11-27). It is the blind spot of that lane's sweep: readers that
never ask `face_normal`'s doors, so never see `NoNormalAt`.

## What

`geom_brep::implicit_outward_normal` (and `face_normal::surface_outward_normal`
over it) folds `implicit_gradient`, whose `Nurbs` and `Approx` arm is
`poison_vec()`. The boolean admits `Nurbs` operands
(`boolean::reduce::boolean_arm_exists`). So a reader that calls it on a
face's carrier without asking the kind reads a poison normal on a
spline face:

- **If the poison reaches a decided margin**, the margin is `Indeterminate`
  (`MarginKind::Invalid`). The refusal is typed, but it reads as an
  in-band escalation, where it is the reading's limit.
- **If the reader tests for poison itself**, it decides what the poison
  means. `splitting::join::leave_on` reads it as a cone apex and
  refuses `JoinError::SectionInvariant`, a kernel fault, for any wall
  without a normal.

## Hit list

`grep -rn "implicit_outward_normal(\|surface_outward_normal(\|chart_outward_normal(" crates/topo/src`
outside tests and `face_normal.rs`. None of these was checked for a
kind guard upstream:

- `boolean/reduce.rs` (≈2186): a junction's glued face, `junction_end_which`;
- `boolean/reduce.rs` (≈2406): `face_corner_within_pi`;
- `boolean/trace.rs` (≈527): `crossing`, through `surface_outward_normal`;
- `boolean/contact_verify.rs` (≈1572, ≈1619);
- `injective.rs` (≈1453): `chart_outward_normal`, from a chart normal the
  caller holds, so likely not this shape;
- `ruled_region.rs` (≈701): a ruled region, so a quadric by
  construction, likely not this shape;
- `bound_side.rs` (≈1795, ≈2115, ≈2366);
- `splitting/join.rs` (≈434): `leave_on`, the kernel-fault case above.
  It is reached for a face with more than two crossings of the split
  plane. Whether a spline face gets there is not known.

## Done when

Each hit either asks the kind first (a spline face refusing as the
reading's limit, typed), or states why no spline face reaches it. One
door that reads a face's outward normal at a point on its carrier, and
says why where it reads none, would serve them all
(`face_normal::face_outward_normal_at_foot` is that door for the
analytic kinds).
