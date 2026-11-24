---
id: a-cone-against-a-ball-or-a-cone-refuses-at-the-cone-face
kind: issue
title: A cone against a ball or another cone in general pose refuses CurvedBooleanUnsupported at the cone face before its marched section is read
status: dispatched
opened: 2026-11-24
priority: P0
cost: M
branch: sectcone/cone-face-refusal
---


Filed by the cone × sphere / cone × cone door lane (PR 7457); updated in
its fix pass.

The section value now classifies and carries a cone against a sphere off
its axis and against another cone in general pose (`ssi::march_cone_ruled`,
`pair_section/cones.rs`); `serves` takes the pairs and `join::rides_carried`
carries them. The boolean still refuses both fixtures of
`crates/sweep/tests/a_cone_with_a_ball_or_a_cone_in_every_order.rs`, every
op in both orders, with `BooleanError::CurvedBooleanUnsupported` naming the
cone's face: `{ operand: A, face: FaceKey(1v1), kind: Cone }` with the cone
first and `{ operand: B, .. }` with it second (the row pins each run).

**What is ruled out.** Review r1 of PR 7457 read the first ball fixture
(`ball_poled_y`) as having its south pole on `K`'s face, a vertex-on-curved-
face contact (`topo/src/boolean/vtxfac.rs`). The fix pass turned the ball
(`ball_poled_z`, poles at `(0.9, 0.6, ±0.4)`, off `K`), and the refusal
did not move. So it is not that vertex.

**Where it likely is (not located).** Each fixture has an edge of the
partner crossing `K`'s face: the ball's seam meridian runs through the
section loop's interior, and the second cone's seam generators run from
its apex, inside `K`, out through `K`'s side. Review r2 named the lanes an
edge piercing a curved face reaches ahead of the section pass:
`topo/src/boolean/reduce.rs` `spline_interior`'s `unbuilt`, and
`vtxfac.rs` `unglued_flat_sector`. The first discriminator is a fixture
with no partner edge crossing the cone: a ball turned so its seam
meridian stays clear of `K`.

Likely shared with U3 (`the-join-reads-the-cone-pairs-on-the-value.md`).
