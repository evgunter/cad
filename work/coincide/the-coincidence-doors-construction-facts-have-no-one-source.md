---
id: the-coincidence-doors-construction-facts-have-no-one-source
kind: issue
title: The coincidence door learns what a construction guarantees through a hand-written side channel (stated.rs: axis_of, lies_on, slides_along_its_generators) beside rung 3; it needs one consistent source
status: open
opened: 2026-11-18
priority: P2
cost: H
design: true
refs: [a-construction-s-identities-are-a-generated-cache]
---


Filed by the DOCTAIL orchestrator at Ev's request on `[ev]` PR 7073 (2026-11-18): "if there isn't a row already for 'figure out a consistent strategy to make this info available instead of having this one weird side channel' then could you add one?"

## The finding

The coincidence door (`crates/editor-core/src/coincide.rs`, `coincide/stated.rs`) proves that two cells coincide structurally in two ways.
- **Rung 3, the ratified way.** The coincidence is zero as a polynomial in the document's variables, computed from the operations' own output definitions. Ev's 2026-10-06 rule: coincidences are proved by "output definitions plus rung 3, not node theorems".
- **Hand-written minter statements.** These are facts the door reads off the operation that made a cell:
  - `axis_of`: a revolve's band is about its own axis;
  - `lies_on`: a rim lies on its cap;
  - `slides_along_its_generators`: an extrude's wall slides along its heading;
  - `stated::frame_z`: reads a traced frame's heading only through an `Axis` read.

  Each is a second description of what the geometry code computes. Ev allowed such a description only as a cache "generated from the symbolic evaluation and checked against it", added where the algebra is measured too slow or undecidable (`a-construction-s-identities-are-a-generated-cache`; D10 `[coincidence.door]`). Today they are hand-written and are not generated.

7073 adds a candidate fourth: an extrude's caps are normal to its heading. It is needed so that a traced profile drawn on a face's placement chart stays tangent by construction. The build first tries rung 3, on the box mitre's precedent (`dot(d, cross(d, t))` is identically zero). Ev allowed it to join the side channel if rung 3 fails, "since we'll be revisiting it later". This row is that revisit.

## What deciding it needs (`design`)

One consistent source for what the door may assume about a construction:
- **Measure rung 3 on each existing hand statement.** For example, coaxiality under FORK-S6-1's option (b), whose polynomial fold was unmeasured. Retire each statement rung 3 proves within budget.
- **For the rest,** decide the mechanism: a cache generated from the symbolic evaluation and checked against it (the parked row above), or something else.
- **One constraint:** the door must never certify a coincidence the geometry code does not hold. That is the information-flow risk of a second description.

Weigh with designers first.
