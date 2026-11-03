---
id: three-line-offsets-wait-on-line-offset
kind: issue
title: Three off-line distances read p − Line::foot(p) by hand, waiting on Line::offset
status: parked
opened: 2026-11-03
priority: P2
cost: E
blocked_on: [the-frame-carries-every-placement-reading]
---



Found by PR 6475's review (`onecopy/small-reads`). That PR moved the line parameters and feet at these sites onto `geom::Line`. The distance off the line beside each is still spelled by hand, because `geom::Line` on main has no offset reading. PR 6463 (`the-frame-carries-every-placement-reading`) brings `Line::offset`.

- `crates/topo/src/shell.rs` `MovedWall::cut_counting`, the ball's offset off `L`: `(c − line.foot(c)).norm()`.
- `crates/topo/src/lib.rs` `along_line`, a vertex's distance off the line: `(p − line.foot(p)).norm()`.
- `crates/topo/src/splitting/join.rs` `ruling_pairs` (the ruling-side `off` closure): `|(p − line.origin) × line.dir|`.

Fix: read `line.offset(p).norm()` at all three once `Line::offset` lands, and re-baseline whatever bits move.
