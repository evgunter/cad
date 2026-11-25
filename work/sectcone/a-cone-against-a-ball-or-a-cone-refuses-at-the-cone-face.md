---
id: a-cone-against-a-ball-or-a-cone-refuses-at-the-cone-face
kind: issue
title: A cone against a ball or another cone in general pose refuses CurvedBooleanUnsupported at the cone face before its marched section is read
status: open
opened: 2026-11-24
priority: P0
cost: M
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

**Where it is.** The join, not the sweep: `join.rs` `choose_roles`, the
ring lane's `RingClosure::Chord` arm. In both fixtures the section loop
lies in `K`'s face clear of `K`'s seams, so on `K` it is a ring a carried
chord closes, and that arm named the carried ring's faces by kind
(cylinder, sphere, torus, spline wall) and refused any other quadric
`CurvedBooleanUnsupported` naming the face: the cone, the one quadric
missing. It fires in both orders because the ring is on `K` whichever
operand it is. Behind it `chord_join.rs` `run_loop_arcs` read a carried
spline run edge on a cone as unread (`RingIslandUnread`), on the stated
ground that the section table carries no cone pair, which PR 7457 made
false.

**The fix.** A carried ring on a cone is read as on a cylinder: the ring
path's pieces on a cone (`ring_path` `Quadric::ruled_paths`) are its
parallels and rulings, off its apex, which lie on it, so a crossing of a
certified spline arc on the surface is a meeting in space (`SplineArc`'s
argument). `choose_roles` takes every carrier the paths read, and
`run_loop_arcs` reads a spline run edge by `SplineArc::of_edge` on every
carrier. A face no path reads (a spline whose iso curves are not read)
still refuses `CurvedBooleanUnsupported` there.

**Seam with U3** (`the-join-reads-the-cone-pairs-on-the-value.md`): the
cause is the join's ring lane, not `rides_carried`, which already carries
`ConeSphere` and `ConeCone`. The change is the two arms above; U3's
`rides_carried`, node and `coverage` work is untouched.
