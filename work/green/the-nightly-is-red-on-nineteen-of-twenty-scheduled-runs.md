---
id: the-nightly-is-red-on-nineteen-of-twenty-scheduled-runs
kind: issue
title: The scheduled nightly concluded failure on 19 of its last 20 runs (green only 11-08), so its colour carries no signal and a new red lands unseen
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Found by a designer weighing the fast-set bar (2026-11-22). Every scheduled nightly since 11-08 concludes `failure`. Two things keep it red: main's held reds, and fuzz rows left red by Ev's 11-17 ruling. So the run's colour cannot tell a new red from the standing ones. Anything proposing that "the nightly owns X" has to name a separate check, since a red that lands there is buried.

Owed: make a new red distinguishable from the held ones. One way is for a held red to be pinned or listed so the leg goes green; another is a named check that fails only on reds no live row holds. That is GREEN's charter (a green main). Weigh it against Ev's ruling that fuzz rows may stay red.
