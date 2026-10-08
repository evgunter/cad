---
id: placed-carriers-compare-through-their-frames
kind: issue
title: D10 stage 4 PR H: a placed carrier's canonical form reads its placement's frame, so a mate-placed face is structural
status: parked
opened: 2026-10-08
priority: P0
cost: H
design: true
blocked_on: [carriers-compare-in-canonical-form, a-placement-is-the-bundle-of-mates, a-mate-reads-face-variables]
needs_ev: true
---

INTENT stage 4, PR H. Spec: `docs/INTENT-STAGE4-SPEC.md` §9. Design open: FORK-S4-5 (a mate-placed face proven through its frame vs a mate rung), reconciled with the stage 3 spec.

`PoseForm` reads a placement's `Frame` variable (stage 3) instead of C's opaque chain atom, so a mate-placed face reduces equal to its partner. `compose_placed` and `GeomSource`'s `Placed` arm go.

FORK-S4-5 was weighed with FORK-S3-4 as FORK-S3O (fork log row 96) and
went to Ev in an `[ev]` PR. After Ev's comments on #4325 there is no
rewrite system. A placed copy's frame is the construction its ordered
bundle states, and the door proves a mate-placed face (and a fillet
flank on it) by replaying that construction at `Sym` with the rest of
the document, as #4322's door does for any coincidence. So H is "the
mate fold runs at `Sym`" (`SolveScalar` for `Sym`, the fold's `atan2`
an opaque symbol per #4322). Measure first whether rung 3 proves
`turn/4` identities through the fold.
