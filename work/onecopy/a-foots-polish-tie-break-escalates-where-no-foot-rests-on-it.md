---
id: a-foots-polish-tie-break-escalates-where-no-foot-rests-on-it
kind: issue
title: on_carrier's smaller (curve_foot_polish) decides a polish tie-break at the least band and still records an escalation when an Interval enclosure straddles zero, though no foot rests on it
status: open
opened: 2026-11-03
priority: P1
cost: E
refs: [6449]
---


Found by the PR 6449 fix pass (`onecopy/foot-and-angle`), which unlogged `angle_near`'s half-turn flip (`curve_foot_branch`, audit ledger row F31) for this reason.

`geom::curves::on_carrier`'s `smaller` (`curve_foot_polish`), which serves the ellipse, spiric and spline polish, is the same kind of choice: a heuristic tie-break between two candidates. At `Interval`, an enclosure that straddles zero records a live escalation. `editor-core`'s `drive::log_read` reads a node's first escalation as the leaf's verdict, so a leaf can refuse or bisect on a choice that no answer depends on.

Fix: establish whether any foot depends on the pick. If none does, route it through `check_unlogged` with a ledger row, as F31 does. If one does, the escalation is real, and the row should say which foot depends on it.
