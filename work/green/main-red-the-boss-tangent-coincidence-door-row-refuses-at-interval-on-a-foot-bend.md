---
id: main-red-the-boss-tangent-coincidence-door-row-refuses-at-interval-on-a-foot-bend
kind: issue
title: main red: editor-core coincidence_door::a_boss_typed_tangent_to_a_cylinder_glues_one_unproven_tangency refuses at Interval on section_site_foot_bend (slow set)
status: open
opened: 2026-11-24
priority: P1
cost: M
---



Found by the BOXFINISH one-plane lane (`boxfinish/one-plane`, 2026-11-24)
while running `coincidence_door`'s slow rows. No open row held it:
`work/charttail/the-ruled-region-span-holds-two-stage6-scenes-off-the-registry.md`
names the scene, from when the row pinned an escalation at `Interval`;
the row asserts today that the union certifies there
(`certifies_at_interval`, `crates/editor-core/tests/coincidence_door.rs`).

**Measured** on main `dc21cb117` (the lane's tree with main's
`geom-brep/src/plane_eq.rs` and `carrier_eq.rs` restored), default ε,
the gate's profile:
`cargo nextest run -p editor-core --test all --ignore-default-filter a_boss_typed_tangent_to_a_cylinder`
fails at `certifies_at_interval`'s
`boss_tangent_by_value's union certifies at Interval`. The payload:

- the union's node fails
  `Boolean(Escalated { decision: Coincidence(Section, Moot), diag: Indeterminate { margin: Invalid, predicate: "section_site_foot_bend" } })`;
- its first escalation is `curve_pair_pose_parallel` at
  `[-1.0000000050247593e-8, -9.99999993922529e-9]` against an escalate
  band of `1e-8` (the enclosure straddles the band's edge), then two
  `curve_carrier_piece_clear_by` terminal slivers near `6.603e-9` and
  `curve_pair_pose_parallel` again with an invalid margin.

The same payload, bit for bit, with the lane's change applied, so the
one-plane reading does not reach it. A designer's probe on main
`36cc57d5ea` (2026-11-24) read the same enclosure. The first bad merge
is not found; the cause is not traced (the parallel pose reading on the
band's edge is where the payload points, and is not shown to be the
cause).

**Done when** the row is green or pinned on its refusal with the row
that holds the cause.

Signed: (BOXFINISH one-plane lane)
