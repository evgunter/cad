---
id: a-cone-against-a-ball-or-a-cone-refuses-at-the-cone-face
kind: issue
title: A cone against a ball or another cone in general pose refuses CurvedBooleanUnsupported at the cone face before its marched section is read
status: open
opened: 2026-11-24
priority: P0
cost: M
---


Filed by the cone × sphere / cone × cone door lane (PR 7457).

The section value now classifies and carries a cone against a sphere off
its axis and against another cone in general pose (`ssi::march_cone_ruled`,
`pair_section/cones.rs`), `serves` takes the pairs and `join::rides_carried`
carries them. The boolean still refuses both pairs, early (about 1.6 s
into each run on CI), with `BooleanError::CurvedBooleanUnsupported
{ operand: A, face: FaceKey(1v1), kind: Cone }`, the cone's face, on
`crates/sweep/tests/a_cone_with_a_ball_or_a_cone_in_every_order.rs`'s two
fixtures. The site was not located: that refusal is raised at
`boolean/recl.rs` (a carrier-identity check that admits planes only),
`boolean/sectors.rs` (`SectorError::Unsupported` / `Unplaced`, the
tangent-locus lane) and `boolean/reduce.rs` (`spline_face`,
`spline_interior`). The witness runs every op in both orders and holds
the volumes closed where an order builds, and refuses a desync or a broken
invariant; it flips to building when this row is fixed.

Likely shared with U3 (`the-join-reads-the-cone-pairs-on-the-value.md`):
the join's reading of the cone pairs past their germs.
