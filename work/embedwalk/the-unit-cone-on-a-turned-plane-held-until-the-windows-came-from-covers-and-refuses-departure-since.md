---
id: the-unit-cone-on-a-turned-plane-held-until-the-windows-came-from-covers-and-refuses-departure-since
kind: issue
title: The unit cone revolved on one turned sketch plane held until a face's window was read from its edges' covers (PR 7120) and refuses Departure at its apex since
status: open
opened: 2026-11-23
priority: P1
cost: M
---

## Finding

Filed by lane `embed/pole` (PR 7115, 2026-11-23).

The triangle `(0, 0) (1, 0) (0, 1)` revolved a full turn about `y` on
a sketch plane turned 1.1 rad about `(0, 1, 1)`, not moved
(`turned_plane(3)` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`), asked
`injective` at f64 with the default budget. It is a valid cone. With PR
7115's reading of the apex (the apex placed, its meridians run on to
the apex's side), discharged / refused / splits:

| case | mains `dc7059c445`, `94b6b284e`, `c3ffedeb2` | mains `ae076e050`, `a3d5ce89c` (PR 7120 in) |
|---|---|---|
| `y0 = 0`, apex up, body | holds | 32,408 / 18,250 / 50,652 |
| the same, its door's cavity | holds | 32,417 / 18,256 / 50,667 |
| `y0 = 1.7`, apex down, body | holds | 32,186 / 18,248 / 50,428 |
| the same, cavity | holds | 32,045 / 18,244 / 50,283 |

The refusal on the first is
`Uncertified { a: FaceKey(1v1), b: FaceKey(2v1), shared: Edge(EdgeKey(4v1)), arm: Departure }`:
the two half-cones across a shared ruling. On the apex-down body it is
`3v1`, `4v1`, `Edge(2v1)`. The same at ε 1e-9, 1e-6 and 1e-12. On the
other two turned planes of that file the unit cone holds, in about
5,200 splits; in the plain frame it holds in 173.

PR 7120 (`embed/rational-window`) reads a face's boundary box from each
edge's cover over its span (`clearance.rs`, `cover`, `boxes`). It moved
cones both ways: three cases that refused before it hold since (the
table on
`work/embed/two-half-cones-sharing-both-rulings-refuse-departure-at-their-apex-in-a-turned-frame.md`),
and these four, which held, refuse. With main's own `injective.rs` the
cone runs out of pairs on every main, its apex unplaced, so the change
of verdict is visible only with PR 7115.

**The exit** (traced on the body, on `ae076e050`): `zero_of` reads
`Local` in the cells at the apex, 90,849 times in the run: 58,517 at
its last arm, with both shared rulings zero sides of one cell, and
32,332 at the arm where a shared vertex is no one corner of two clips.
That is exit A of the two-half-cones row; this cone is one of that
row's cases, filed here for what moved it.

**Not traced:** which cover box changed, and why the walk then visits
cells that read `Local` where it did not before. Whether the cone held
before PR 7120 by a reading or by the cells the walk happened to visit
is not known either.

Pinned as a refusal, with the pair and the shared edge, by
`self_intersection_proves_injectivity::the_unit_cone_on_the_third_turned_plane_refuses_departure_at_its_apex`.
Red when it holds.

**On the landing train** (`origin/embed/wave3-land` at `f06547ccf`,
merged into PR 7115, 2026-11-23): the four refuse as before, the same
pairs and edges, at the same splits (50,652, 50,667, 50,428, 50,283).
The receipts read 25,153 / 1 / 50,652 and alike, since the ask now ends
at its first finding.
