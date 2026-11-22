---
id: check-twos-turn-reads-first-speed-meters-at-a-tenth-of-a-ball-cylinder-boolean
kind: issue
title: The edge door's check 2 (decide_turn) spends about a tenth of a ball × cylinder boolean, mostly first speed-meter computes, beyond the clone recomputes NURBS owns
status: open
opened: 2026-11-21
priority: P1
cost: M
---


Filed by SECT's orchestrator, 2026-11-21, from the re-profile on
`a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build`
(its 2026-11-21 section, PR 7285): in ball ∪ rod the edge door's check 2
(`decide_turn`) is 13 % of the op, 142 calls, 0.25 s, with `speed_pieces`
11 % of it; across the op `speed_pieces` is 15 % (28 computes, 0.46 s).
`work/nurbs/a-curve-cloned-before-its-speed-meters-first-read-computes-them-again.md`
(P3) owns only the clone recomputes (about −0.13 s); the first computes
and the turn's own cost are unowned. Banded P1 as a blocker of that P1
row. Measure first: what the turn reads per call, and whether the first
speed meters can be cheaper or shared with the certificate that already
reads the same carrier.
