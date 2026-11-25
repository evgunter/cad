---
id: main-red-cover-kind-match-raised-arm-in-spline-implicit
kind: issue
title: Main red (lint): cover-kind-match reports a RAISED arm in geom-brep pair_section/spline_implicit.rs (1 -> 2)
status: open
opened: 2026-11-25
priority: P1
cost: E
---


`scripts/gates/cover-kind-match.sh` fails on origin/main `556dcd1c` against main itself:

> RAISED arm crates/geom-brep/src/pair_section/spline_implicit.rs: 1 -> 2

The file's latest commits on main are SECTARMS' (`f5029720f2`, `9f6ae62083`; the rim-pad-cells fix pass), so it is filed here. Re-home it if not. It keeps `lint`, and with it `gate ok`, red on every PR. Found by HELDRECORDS' removed-members lane (PR 7745), whose diff does not touch geom-brep. Owed: lower the arm back to its pin (read the kind through the cover registry), or justify the raise in the counts file. (filed by the HELDRECORDS orchestrator)
