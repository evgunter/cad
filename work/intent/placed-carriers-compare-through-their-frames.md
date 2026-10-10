---
id: placed-carriers-compare-through-their-frames
kind: issue
title: D10 stage 4 PR H: a placed carrier's canonical form reads its placement's frame, so a mate-placed face is structural
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [carriers-compare-in-canonical-form, a-mate-relates-two-poses, a-mate-reads-face-variables]
---

INTENT stage 4, PR H. Spec: `docs/INTENT-STAGE4-SPEC.md` §9. FORK-S4-5 is settled by FORK-S3O (fork log row 96, PR 4325), below.

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

After the stage 3 re-spec (2026-10-09), H waits on stage 3 C (`a-mate-relates-two-poses`), not on the placement unit B. The bundle exists after B, but its mates read poses off geometry only after C. Replaying B's offset frames at `Sym` would be written twice.
