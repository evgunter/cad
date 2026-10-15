---
id: a-pose-is-moved-by-lengths-and-angles
kind: ruling
title: FORK-MOVE: a pose is moved by lengths and angles along and about what it reads; no slot holds a direction component; a default is the constant 0
status: open
opened: 2026-10-14
needs_ev: true
---


Raised in a GUI session (Ev in chat, 2026-10-14): the datum-frame form takes `u`, `v` as six `Scalar` components, squares `v` against `u` at evaluation, and D10's equal-value offer then offers every `1.0`/`0.0` as a variable to share. Ev: "it'd certainly be nice to not need an exception to the variable-offering rule". The same shape survives stage 3 on main: `InFrame`'s scalar coordinates (`PoseCoords`, landed by A), `Revolve`'s `axis_direction: [S; 2]`, the slant's normalised slots, and placement values' façade-written free zeros.

A designer pair weighed it (fork log row 149) and converged after six rounds; both reports are in the `[ev]` PR titled "a pose is moved by lengths and angles". The PR edits D10, VR4 and `docs/INTENT-STAGE3-SPEC.md` (§1, the A′ unit, Q1, Q7, the restate ladder, G's explicit frames, and the bit-equal rows Ev disowned in the same chat). Open to Ev: the source of "united zeroes are still variables" (fork log row 97), which decides whether the façade's default is the constant `0`.

On approval: file `poses-move-by-lengths-and-angles` (A′) and the `Sym` π-fold unit, fill row 149, clear `needs_ev`, close this.
