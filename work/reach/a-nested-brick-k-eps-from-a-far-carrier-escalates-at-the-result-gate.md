---
id: a-nested-brick-k-eps-from-a-far-carrier-escalates-at-the-result-gate
kind: issue
title: A brick nested k·ε from an L-prism's far carrier unions and subtracts to the right volume, and the result gate refuses it RingContactEscalated
status: open
opened: 2026-10-03
priority: P2
cost: M
refs: [boolean-door-adopts-the-finished-body-type]
---


Found by `boolean-door-adopts-the-finished-body-type`'s after-measurement
(`python3 scripts/door-tier3-meter.py`, ε 1e-9, 1e-6 and 1e-12).

`crates/topo/tests/review_cleave_farplane.rs`'s
`booleans_beside_a_far_carrier_never_answer_wrong` nests a brick in the
L-prism with one face `d = k·ε` (k = 2, 5, 9) from the L's far carrier.
On `origin/main` 82b9ceb2 the door shipped the union and the subtract,
and their volumes matched the closed form (`12` and
`12 − 0.25·(1 + d)`, the row's own oracle, to 1e-9). With tier 3 at the
door all six refuse `ResultInvalid` carrying
`RingContactEscalated { face: 1v1, ring: 12v1 }` at margins 2ε, 5ε and
9ε, inside the escalate band (10ε), at every ε row. The row accepts a
typed refusal, so it stays green.

So these are valid bodies the door no longer returns: the ring the
union or subtract leaves on face 1 lies within the band of the face's
boundary, and tier 3 cannot certify its containment. D4 makes that an
honest escalation; whether a door-built ring that close to its face's
boundary should have been decided at the join (where the pipeline knew
the contact) rather than re-asked at rest is the open question.
