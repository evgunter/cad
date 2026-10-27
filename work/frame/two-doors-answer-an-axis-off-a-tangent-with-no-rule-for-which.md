---
id: two-doors-answer-an-axis-off-a-tangent-with-no-rule-for-which
kind: issue
title: path_start_frame and Vec3::orthonormal_basis both answer 'an axis off a tangent' and nothing says which a caller uses
status: open
opened: 2026-09-15
priority: P1
cost: M
design: true
---

## Finding

**Raised by SCALAR's `S393` fix pass** (2026-09-15), which put the
sweep corpus and the demo tour onto `path_start_frame` and, in the same
sweep, put one test helper onto `Vec3::orthonormal_basis` — and had
nothing written to decide which caller got which.

`geom_core::linalg::frame::path_start_frame(origin, tangent, tol)` and
`Vec3::orthonormal_basis(self)` both answer *"give me axes across this
direction"*. They differ in everything else and both differences are
documented at their own doors, but the CHOICE between them is not
documented anywhere:

- the frame door takes a point and a tolerance, returns an `Affine3`,
  decides the tangent's length and each reference rung under the linear
  band, and REFUSES typed when nothing decides;
- the vector door takes a unit vector by precondition, returns a pair
  of `Vec3`, is total, is branch-free (`copysign`, no threshold), and
  is bitwise-pinned against the Duff spelling.

So a caller with a curve tangent in hand can reach either, and S393's
hit list split its sites by taste: a start placement for a sweep went
to the frame door, an `(axis, u_ref)` pair for a chart went to the
vector door. That reading is defensible and it is nowhere written, so
the next caller re-derives it — or does not, and the tree grows a third
convention.

What would settle it is one paragraph, in `linalg::frame`'s module docs
(the "three constructors" list is where a reader is standing when the
question arises): the frame door is for a PLACEMENT — something with an
origin, a roll policy and a refusal — and the vector door is for chart
DATA stored beside an axis, where a caller has already certified the
axis and wants no policy at all.

**Where**: `crates/geom-core/src/linalg/frame.rs` (module docs,
`path_start_frame`), `crates/geom-core/src/linalg/vec.rs`
(`orthonormal_basis`).

**Confidence**: sure (both doors read as described; the absence is the
finding).

This is the second half of `work/comb/S35.md`'s
`frame::path_start_frame` row, whose first half (*"no kernel caller;
`sweep` still builds its own frames"*) S393 moved; that row carries an
evidence line pointing here.

**Verdict:**

## Parked under the D10 hold (2026-10-21)

FRAME orchestrator. Half answered since filing: D10 `[placement.frame]` (a minted reference is a function of the inputs) and PR #5547's gate confine `orthonormal_basis` to scratch and input-coordinate homes, so the vector door is no longer a choice for a kept frame. The other door's fate is D′'s (see `path-start-frame-is-the-only-frame-from-a-tangent-and-is-named-for-the-start`). Re-priced `M` with `design: true` from legacy `D`.

## Released (INTENT, 2026-10-26)

This row was parked on the stage-3 placement pivot (`a-placement-pins-by-written-values-without-a-mate`). All three of its PRs are merged: D′1 (PR 5592), D′2 (PR 5613) and D′3 (PR 5636, `29110056e8`). Re-check this row against main, then fix, narrow or close it. That is the owning track's call.
