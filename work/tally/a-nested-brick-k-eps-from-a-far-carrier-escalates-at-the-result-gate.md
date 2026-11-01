---
id: a-nested-brick-k-eps-from-a-far-carrier-escalates-at-the-result-gate
kind: issue
title: A brick through an L-prism's top k·ε from its far carrier unions and subtracts to the right volume, and the result gate refuses it RingContactEscalated
status: closed
opened: 2026-10-03
closed: 2026-10-20
priority: P2
cost: M
refs: [3987]
---


Found by `boolean-door-adopts-the-finished-body-type`'s after-measurement
(`python3 scripts/door-tier3-meter.py`, ε 1e-9, 1e-6 and 1e-12).

`crates/topo/tests/review_cleave_farplane.rs`'s
`booleans_beside_a_far_carrier_never_answer_wrong` pokes a brick
through the L-prism's top with one face `d = k·ε` (k = 2, 5, 9) from
the L's far carrier (the row's "through the top" case; the id's
"nested" is a misreading of the meter's row, kept for the id). On
`origin/main` 82b9ceb2 the door shipped the union and the subtract,
and their volumes matched the closed form (`12 + 0.25·(1 + d)` and
`12 − 0.25·(1 + d)`, the row's own oracle, to 1e-9). With tier 3 at the
door all six refuse `ResultInvalid` carrying
`RingContactEscalated { face: 1v1, ring: 12v1 }` at margins 2ε, 5ε and
9ε, inside the escalate band (10ε), at every ε row. The row pins
exactly those six refusals and that payload, so it goes red the day
they build.

So these are valid bodies the door no longer returns: the ring the
union or subtract leaves on face 1 lies within the band of the face's
boundary, and tier 3 cannot certify its containment. D4 makes that an
honest escalation; whether a door-built ring that close to its face's
boundary should have been decided at the join (where the pipeline knew
the contact) rather than re-asked at rest is the open question.

## Resolution (2026-10-20)

Answered at rest, not at the join. The ring's edge lies `kε` across
from the L's far carrier but far along it, and check 9's `lines_meet`
read the four ends' sides first, so the in-band side escalated. It now
reads the two edges' gap on one line first (`geom_core::one_line`, PR 2
of `work/cpair/two-spans-projections-on-one-line-are-read-four-ways`):
projections apart past the band hold every pair of points apart, the
pair is `Apart`, and tier 3 certifies the ring. All six build to the
closed form at every k; the row's refusal pin is re-baselined to none.
