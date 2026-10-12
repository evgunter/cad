---
id: of-join-sites-are-held-by-grep-alone
kind: issue
title: The Boolean's one route for a chord-join lineage refusal (BooleanError::of_join) is held at fifteen call sites by grep alone
status: open
opened: 2026-10-12
priority: P3
cost: E
refs: [face-lineage-rows-are-checked-by-their-readers-not-where-they-are-added, 4868]
---


Found by the orchestrator's delta check of PR 4868
(`fuse/face-lineage-at-append`).

PR 4868 gave a lineage row the chord join's face division list refuses
one route into the Boolean: `BooleanError::of_join`
(`crates/topo/src/boolean/mod.rs`) lifts
`SplitJoinError::LineageRow` into the Boolean's own
`BooleanError::LineageRow` and nests every other join refusal in
`BooleanError::Join`. The fifteen sites in
`crates/topo/src/boolean/join.rs` that turn a join refusal into the
Boolean's call it (`.map_err(BooleanError::of_join)`), but nothing
holds them to it: the delta check's mutant E, one site reverted to
`BooleanError::Join(…)`, survives every suite, because no model reaches
a join `LineageRow` (it is a kernel defect, and the refusal row reads
`of_join` directly).

What is owed, one of:
- make `BooleanError::Join` unconstructible from a
  `SplitJoinError::LineageRow`: the variant holds a join refusal type
  that has no lineage arm (the lift then being the only conversion), or
  `Join`'s payload goes private behind `of_join`;
- or a source-scan row over `topo/src/boolean`, in the shape of the
  existing door guards (`crate::source_walk`), that refuses a
  `BooleanError::Join(` built anywhere but `of_join`.
