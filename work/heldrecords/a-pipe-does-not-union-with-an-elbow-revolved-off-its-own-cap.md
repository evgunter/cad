---
id: a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap
kind: issue
title: A pipe does not union with an elbow revolved off its own end cap: the union refuses a recorded contact as undecided at a margin of -7.07
status: review
opened: 2026-11-21
priority: P0
cost: H
branch: heldrecords/elbow
pr: 7432
---


Found by PROFILE's elbow probe (2026-11-21), run on hosted CI as the
throwaway PR 7280 (run 44889924284, `test (app)`,
`editor-core::zz_probe_elbow`), for Ev's question whether a hollow
cylinder's end loops can be revolved into an elbow
(`work/faceloop/a-faces-loop-or-closed-trace-cannot-become-a-profile-loop.md`).
Filed here because the refusal is about a recorded contact
(`crates/topo/src/boolean/refusal_routes.rs`, `Coincide::Contact`,
"whether a recorded contact holds along its witness"); re-home it if
another program's charter fits it better.

## Measured

A pipe: `circle((0,0), 1.0)` and `circle((0,0), 0.6)` extruded 2. The
same two circles drawn on its end cap (`editor_core::draw_on_face`,
`OnFace::nearest()`), revolved a quarter turn about a line in the cap's
plane (`axis_through = (3, 0)`, `axis_heading = −π/2`, or `(−3, 0)`
and `+π/2`). The elbow builds. `Node::Union` of the pipe and the elbow
refuses, in all eight cases where the revolve builds (either cap,
either sign of the angle, either side), at ε = 1e-9, 1e-6 and 1e-12:

> the Boolean op refused: whether a recorded contact holds along its
> witness is undecided: margin -7.0685834705770345e0 lies past the
> ambiguity band (1e-9, 1e-8) — a decided sign this decision cannot
> use. Recourse: move the parts so they clearly meet or clearly stand
> apart there

## The defect

The two bodies share the whole annular cap, and their walls meet
tangent along both rims: this is the plain case of building onto a
body's own face and joining the result. The margin is not near the
band: −7.0686 is −9π/4 to the printed digits, a length or an angle
measure decided far on the wrong side, so the reading is asking the
wrong question of this contact (or reading it at the wrong place), not
losing a close call. The recourse cannot be followed: the parts meet
by construction.

Banded P0: a normal verb broken on normal geometry. It also gates
`work/faceloop/a-faces-loop-or-closed-trace-cannot-become-a-profile-loop.md`:
once a face's loop can be a profile loop, the elbow the person makes
by two clicks and a revolve is this union.

Not traced: which contact record, which witness, and why the margin is
that value. Not measured: an extrude off the same cap unioned with the
pipe (the same shared cap, with the walls coincident and not only
tangent); a solid rod and its elbow (no inner rim).

The probe's source is on the branch `profile/probe-elbow` until PR 7280
is closed; its construction is the four lines above.
