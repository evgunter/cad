---
id: ring-sites-names-are-not-unique-across-readers
kind: issue
title: RingSites::new refuses a shared name within one reader, but nothing stops two readers from sharing row names
status: open
opened: 2026-11-09
---


Found by the FULL review of PR 6641 (NOTE 7). It was recorded on the closed row `the-ring-torus-question-has-two-homes`, and the delta review asked for its own row.

`geom::RingSites::new` (`geom/src/surfaces.rs:625`) is a `const fn` that refuses `tube == ring`, so one reader cannot name both halves alike. Nothing checks names across readers. The readers today:
- `topo/src/boolean/reduce.rs:4106` (`SHEET_RING_TORUS`);
- `topo/src/boolean/slit.rs:1216` (`RIDER_RING_TORUS`);
- `topo/src/injective.rs:136` (`RING_TORUS`);
- `RingSites::CONVENTION`;
- the test row `geom-brep/src/variety.rs:169`.

Any of these could pass another's names and compile. The K stream and `docs/predicate-dimension-audit.md` would then merge two readers' samples under one row, and only a reading of the ledger would show it.

**Options.** Ordered roughly from cheapest to most thorough:
1. A gate that collects every `RingSites::new(..)` literal and refuses a name used twice.
2. A roster of reader site names checked at test time.
3. The same check for the funnel's site names at large, if this is the general case. `SpiricRegimeSites` (PR 6621) has the same shape.
