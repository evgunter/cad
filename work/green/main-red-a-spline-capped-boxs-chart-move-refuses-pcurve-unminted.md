---
id: main-red-a-spline-capped-boxs-chart-move-refuses-pcurve-unminted
kind: issue
title: main red: sweep::all shell_general_door::a_spline_capped_box_moves_every_chart_onto_its_closed_form refuses Pcurve Unminted on face 1v1
status: open
opened: 2026-11-25
priority: P1
cost: E
---


## Finding

Reported by EMBEDWALK's lane `embedwalk-trimmed` (2026-11-25), while
running sweep's slow set for PR 7787: the slow-set test
`sweep::all shell_general_door::a_spline_capped_box_moves_every_chart_onto_its_closed_form`
(`crates/sweep/tests/shell_general_door.rs`) fails with
`Pcurve Unminted { face 1v1 }`, identically with that PR's two source
files reverted to main's. Not reproduced by the orchestrator; no row
named the test. Not bisected: candidates are the recent pcurve-minting
changes (the finish completing a body's pcurve rows, and the curve
placeholder's retirement merged into this file 2 days ago).

P1 as a red main slow test (a slow-set red is a red nightly). Filed by
the EMBEDWALK orchestrator for the program that owns the file.
