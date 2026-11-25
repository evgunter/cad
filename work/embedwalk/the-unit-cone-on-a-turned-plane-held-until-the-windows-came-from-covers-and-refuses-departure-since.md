---
id: the-unit-cone-on-a-turned-plane-held-until-the-windows-came-from-covers-and-refuses-departure-since
kind: issue
title: The unit cone revolved on one turned sketch plane held until a face's window was read from its edges' covers (PR 7120) and refuses Departure at its apex since
status: dispatched
opened: 2026-11-23
priority: P1
cost: M
branch: embedwalk/unit-cone
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

## Trace

Lane `embedwalk/unit-cone`, on main `e74caf6479` (PR 7679 in), f64,
the default budget. Measured with a local, env-gated instrument in
`window_of` and `boundary_of` that did not land.

**Re-measured first.** On `e74caf6479` the four cases still refuse:
the apex-up body at 25,153 / 1 / 50,652 on `1v1`, `2v1`,
`Edge(4v1)`; the apex-down body (`(0, 2.7) (0, 1.7) (1, 2.7)`) at
25,182 / 1 / 50,735 on `3v1`, `4v1`, `Edge(2v1)`. The review sweep's
cavities refuse at 50,587 and 50,334 splits.

**Which cover box changed.** The two half-circle rims' boxes. The
rulings' boxes were and are the segment's own box, to the bit. A rim's
box read from the interval evaluation of its circle over `[0, π]` (the
old reading) is wider than its cover's (on face `1v1`, the rim's `x`
reaches `1.084` against `0.776`). The cone's window reads `v` as the
slant of the face's boundary box along the cone's axis, and in a
turned frame that axis is off the world's axes, so the box's projection
onto it reaches past the boundary both ways: past the base and, across
the apex, into the other nappe. Half-cone windows on the third plane:

| face | `v` before PR 7120 (old box) | `v` since (cover box) | the boundary's `v` |
|---|---|---|---|
| `1v1` | `(-3.271, 0.600)` | `(-2.997, 0.266)` | `[-√2, 0]` |
| `2v1` | `(-3.271, 0.542)` | `(-2.896, 0.268)` | `[-√2, 0]` |

The first two turned planes overshoot the same way (since PR 7120,
frame 1's `v.hi` is `0.47` and `0.28`, frame 2's `0.11` and `0.28`),
where they hold at 5,204 and 1,036 splits; in the plain frame the
axis is a world axis, the projection is exact and `v.hi` is `0`.

**Whether it held by a reading or by the cells visited: by the cells
visited.** With main's `injective.rs` unchanged:

- main with the old box restored alone holds the third-plane cones
  again, at 2,041 and 1,972 splits;
- main with the apex end of each straddling cone window set to `t`
  (both faces alike) holds at `t = 0` (254 / 224 splits), `1e-12`
  (207 / 199), `0.01` (269 / 260), `0.1` to `0.3` (341 to 511),
  `0.4` (1,513 / 1,579), and refuses `Departure` on the same pairs at
  `t = 0.5` (20,437 splits) and `0.6` (41,000). The old box's ends
  (`0.600`, `0.542`) held; the cover's (`0.266`, `0.268`) refuse.

So the verdict follows how far the windows reach past the apex, and
not that alone: the old box reached farther (`0.600`, `0.542`, with
its own far ends) and held, where a shared end of `0.5` refuses.
What was demonstrated is that the verdict follows the apex end and
that ending the window at the apex holds in every frame in about the
plain frame's splits. Why a cell straddling the apex reads `Local`
where one ending at it does not was not traced cell by cell; that a
straddling cell holds both nappes, so the apex is inside it rather
than its side, is read off the code.

**The cause is the window's, and the fix is there.** `window_of` now
reads each affine chart coordinate (a plane's `u` and `v`, a
cylinder's axial `v`, a cone's slant) as the boundary's range along
the coordinate's direction, each boundary piece's support along it
(`Cover::range_along` over the hull of the edges' covers and the
vertices), not the projection of the boundary's world box. On the
third plane the half-cones' windows end at the apex within rounding
and the four cases hold: 247 / 268 splits (bodies), 273 / 269
(cavities). The pin of the refusal is retired; the third plane is a
row of `singular_points_hold_far_from_the_origin_and_in_turned_frames`
with a ceiling of 400 splits on every unit-cone row.
