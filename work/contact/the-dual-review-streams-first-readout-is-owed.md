---
id: the-dual-review-streams-first-readout-is-owed
kind: ruling
title: The dual review stream reached twelve fair pairs at DR-13, so its first pre-registered readout is owed to Ev (DUAL-REVIEW-PROTOCOL rule 9)
status: closed
opened: 2026-09-28
priority: P2
closed: 2026-09-28
---


`docs/DUAL-REVIEW-PROTOCOL.md` rule 9 owes a readout when the tally reaches eight or
the fair-pair count reaches twelve, whichever comes first. DR-13
(CONTACT-5) is the twelfth fair pair. The readout is off-file per rule
10. It lives on branch `analysis/dual-review/readout-1`, where a
separate agent wrote it so that the orchestrator stays blind to it. The
ruling asked for: what the dual stream does next (continue as is,
narrow the dual tier, change triage, or change the instrument). Duals
continue until Ev rules.

## Ruled (Ev, PR 3342, 2026-09-28)

"cool. let's continue duals until there are 12 pairs which found any
MAJOR". Rule 9 of `docs/DUAL-REVIEW-PROTOCOL.md` now states the next
readout point: twelve fair pairs that found any MAJOR, or a tally of
eight, whichever comes first. At the ruling, four fair pairs had
found a MAJOR (DR-5, DR-9, DR-11, DR-13). The readout branch
`analysis/dual-review/readout-1` stays off main, and PR 3342 is closed
unmerged.
