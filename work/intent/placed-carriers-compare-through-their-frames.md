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
went to Ev in an `[ev]` PR. A placed copy's frame is never a variable or
a stored value the door reads, and `Place` defines no `Frame` port. A
copy's poses reach the door as `Carried { copy, pose }`, and each mate of
a pinned bundle is a rewrite rule `Carried { copy, a } ≡ b` (mod the
kind's symmetry). Because every excess equation is structural, the
rewrite system is confluent. So H is "rewrite through bundles into one
space, then decide as a within-part relation", not "compose a frame".
Measure first whether rung 3 proves `turn/4` angle identities at `Sym`
(an extrude's cap against a profile's right-angled wall): nearly every
multi-mate bundle has an angle excess equation. Add that case to stage
4's mitre measurement before stage 3 F is dispatched.
