---
id: TCOST-B2
kind: unit
title: helper dedup for the five crates B1 skipped, with the one-declaration guard
status: dispatched
opened: 2026-09-03
blocked_on: [TCOST-B1]
branch: tcost/b2-dedup-remaining
---

Cut at batch style review 2 (`log.md`). TCOST-B1's one-declaration
pass over the five crates it skipped — step-export, step-import, stl,
geom, geom-core (11 933 redundant compiled lines, ~3 % of the class)
— with the same script, and the guard assertion B1 added (a suite
file declares no modules) carried into each converted crate's
`every_suite_file_is_aggregated`.
