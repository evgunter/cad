---
id: an-upper-bound-at-the-boxs-true-maximum-refuses-with-a-false-message
kind: issue
title: ≤ c with c at the box's true maximum refuses box_dependent and says the pair is past it
status: open
opened: 2026-11-02
priority: P3
cost: M
---


Found by the correctness re-verify of PR 6292 (R2, sure, probed;
https://github.com/evgunter/cad/pull/6292#issuecomment-6574195890).
Blocks `0.4 + gap` apart over `gap ±0.05`, `≤ 0.45` and `≤ 0.45 + 5e-10`:
the station pair's reach is [0.35, 0.45000000000000023], so the margin
is about [−0.1, −5e-10] — within the bound at every point — but the
funnel's three-way table cannot read a margin spanning Negative into
Zero, so the verdict is undecided and refuses `box_dependent`. Its
message says "past it at others", which is false. The leaf cannot be
narrowed under ε, so the drive refuses there.

A common engineering shape (a limit set to the worst-case stack). It
shares its cause with value assertions at a bound's edge; the false
message is new. At least the message must not claim a violation no
point has; whether the reading can be decided is the row's question.
(DOORS orchestrator)

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.
