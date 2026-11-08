---
id: cover-carrier-level-piece
kind: unit
title: geom::cover gains a carrier level (shell) piece, so depth_gap's 'a face lies exactly |r| from its carrier's axis, centre or core' is a cover method and carrier_depth's Surface match goes
status: parked
opened: 2026-11-08
priority: P1
cost: M
refs: [cover-topo-faces]
blocked_on: [cover-topo-faces]
---


Raised by PR 6579 (`cover-topo-faces`) and settled by its concurrent dual review (2026-11-08). Both reviewers independently answered (a).

`census::depth_gap::carrier_depth` still matches on `Surface` to read a carrier level: a cylinder, sphere or ring-torus face lies exactly `|r|` from its carrier's axis, centre or core. `level_on` then uses that level in the same least/greatest comparison as the face's cover. That is a distance range of the face from a line, a point or a circle. DESIGN `[one-way.readings]` ("The region an entity can occupy is a reading") lists exactly such ranges as methods on the cover, and says "a tighter construction is written into the arm, never at a reader". Being exact does not make it a different question. No cover piece today is a shell (a level set at a fixed distance from a line, point or circle).

**The unit:**
- add one piece, or a level on `Slab`/`Ball`/`Tube`, that the carrier readings (`Surface::cover`) emit;
- `distance_range` from that axis, centre or core reads the level exactly;
- `depth_gap` reads the cover alone, and `carrier_depth`'s `Surface` match and `level_on` go.

This is a faithful elaboration of the ratified text, so it is not a question for Ev, unless the implementation finds the text does not fit. Rows: a depth row per kind that goes red when the level is dropped.

Review tier: single FULL (it adds a piece to the type every reader shares). (BOXES orchestrator)
