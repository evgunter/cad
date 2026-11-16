---
id: the-census-reads-no-loop-over-a-literal-array-of-names
kind: issue
title: The predicate census reads no for-loop over a literal array of (name, value) rows: a decide site whose name is the loop's binding is listed, and the array's names are read by no roster
status: open
opened: 2026-11-16
priority: P3
cost: M
---


Found by ONECOPY census-tables (#6793) when it merged main.

`test_utils::source::predicate_census` opens a tuple's first element only where a declared `NameCarrier::Call` is handed the tuple (`source::tuple_heads`). It does not read a `for (name, value) in [("lit", …), …]` loop whose body hands `name` to a `decide*` door. The door site is keyed by its binding (`<file>: <fn>: name`) and listed in a roster, and the array's literals are read by no census. So a reference to one of those names resolves only if something else decides it, and a rename of one is not seen.

Production sites listed this way today (geom-brep's roster, `LOOP_ROWS`):
- `crates/geom-brep/src/pcurve_cache.rs:7754` (`spiric_copies`): `pcurve_spiric_major`, `pcurve_spiric_minor`, `pcurve_spiric_offset`.
- `crates/geom-brep/src/pcurve_cache/closed_bound.rs:241` (`two_ovals`): `pcurve_closed_spiric_tube`, `pcurve_closed_spiric_ring`.

geom-core's `LOOP_ROWS` entries are test-only loops of the same shape.

**Done** when the census reads the heads of a literal array a `for` pattern destructures, wherever the pattern's first binding is a `decide*` site's first argument in the loop body. The two entries above, and any other `LOOP_ROWS` entry it then reads, go from the rosters.
