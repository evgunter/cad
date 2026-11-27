---
id: a-ringed-walls-pocketed-pair-against-the-plate-holds-its-line-contact-unrecorded
kind: issue
title: The rod and square joined along a ruling, pocketed, then joined to the plate come out one solid whose rod wall and square face touch along the ruling with no contact record, which the at-rest gate reads as a crossing
status: open
opened: 2026-11-22
priority: P1
cost: M
refs: [a-carried-line-contact-an-op-cuts-is-dropped, a-finished-body-holds-its-contact-records-at-every-door]
blocked_on: [a-finished-body-holds-its-contact-records-at-every-door]
---


## Finding

Found by EMBED's lane `embed/iso-end` (2026-11-22), measuring the
at-rest gate on `embed/iso-end-measure2` (`origin/embed/gate-measure`
at `ebb2e6f8e` plus that lane's diff). It was masked until then: the
test's eighth ask refused first (`Uncertified { a: 3v1, b: 14v1,
shared: Edge(12v7), arm: Departure }`, a cylinder carrier read that
lane fixes).

`sweep::all a_pierce_held_in_a_curved_face::a_ringed_wall_holding_the_pierce_builds_either_operand_first_and_pocketed_last`.
The rod and the square prism touch along a ruling of the rod (the
square's edge lies on it, at bearing 0.5 rad), which the joined pair
records (the asks of the join hold `records=2`). The tenth gate ask, at
the Boolean door, is a body of 19 faces and **one solid** whose door
holds **`records=0`**. It reads

`Crosses(Crossing { faces: (3v1, 7v1), stations: [(0.5, 1.28), (−0.2121, 0.655)], distance: 1.29e-15 })`,

where 3v1 is the rod's cylinder wall, 7v1 a plane side of the square,
and the station on the wall is the ruling at chart height 1.28 above the
rod's base, world `z = 1.78`, inside the contact's span (the rod over
`z ∈ [0.5, 2]`, the square over `[0.44, 1.81]`). The pocket, at bearing 2 rad, does not reach the
ruling.

So the witness is the pair's line contact, a true touch, not a false
witness. The body that reaches the gate has lost the contact's record
somewhere between the join and the plate's union (the pocketing
subtract's asks also hold `records=0`). That is either a record an op
dropped (the family of `a-carried-line-contact-an-op-cuts-is-dropped`,
here with an op that does not cut the contact) or one the test's own
fold does not carry into the next op. Not traced to the op that drops it.

Under Ev's PR 7068 ruling, a solid with an in-solid contact refuses
typed for now. Held, this ask would refuse typed. Unheld, it reads a
crossing.

Moved from `work/contacthold/` to `work/heldrecords/` on 2026-11-25, when CONTACTHOLD closed (`docs/doc-ledger/contacthold-leaves-the-tracker.md`); id unchanged. The cleanup re-read the row against main at `6003877ad` and found it live. Its test's `joined` helper drops the union's records, and `AtRestBody` holds none yet: re-measure it on `a-finished-body-holds-its-contact-records-at-every-door` (PR 7124) and make it that unit's pin or close it as covered.
