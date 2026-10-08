---
id: each-space-computes-in-its-earliest-members-frame
kind: issue
title: D10 stage 3 PR E: an operation computes in a frame chosen from its own reads for conditioning, never the world's (D9); export alone composes the world's map
status: parked
opened: 2026-10-08
priority: P0
cost: M
blocked_on: [a-placement-is-the-bundle-of-mates]
refs: [intent-stage3-is-built]
needs_ev: true
---

INTENT stage 3, PR E. Spec: `docs/INTENT-STAGE3-SPEC.md` §6.

An operation computes in a frame chosen from its own reads, never the world's (FORK-S3-6, D9). Export is the one reader that composes the world's map, and a grep gate holds that nothing else does.

Product digests move into computing-frame coordinates, and STEP bytes move by rounding. Test 15 checks that each moved digest moved by its world map alone. An edit to the world mate moves no body bit.

FORK-S3-6 was weighed with FORK-S3-1 as FORK-S3P (fork log row 95,
PR 4324), and that answer changes this unit. Ev's principle: "use
whatever frame makes things behave well numerically, and hopefully it
doesn't break caching". An operation computes in a frame that is a
function of its reads alone (so it is keyed with them), chosen for
conditioning, and never the world's. The frame is not part of the
meaning: the body up to the rigid map, names, and verdicts outside the
sliver band agree in any two frames, which is the invariant this unit's
test pins (compute in two frames and compare). Where conditioning does
not decide, a construction computes in the base's own coordinates and
an operation over copies in its first listed operand's. A
value-informed refinement is allowed: re-centre on the operands' bounds,
snapped to a power-of-two grid at their scale, with the author's order
as tie-break, measured on the far-from-origin rows such as
`a-far-meeting-point-fails-membership-by-its-own-rounding`. There is no
`world_of(space)` for the viewer: a space not related to the world is
drawn from display state no logic reads. This unit needs
`a-minted-reference-direction-follows-the-computing-axes` first (or
with it), since that is what makes the frame free of meaning. The
at-rest census is defined order-free.
