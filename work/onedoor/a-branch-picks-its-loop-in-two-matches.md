---
id: a-branch-picks-its-loop-in-two-matches
kind: issue
title: The split lane and the boolean each turn a section branch into a loop with their own match, and each turns LoopSplit's margin into Positive/Negative under its own K name
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Filed by the ONEDOOR orchestrator on 2026-11-14, from R1's review of PR 6929 (the section branch moved to geom-brep).

Two places choose a loop from a branch, each with its own match:
- `crates/topo/src/chord_join.rs:1607-1618`;
- `crates/topo/src/boolean/join.rs:3310-3325`.

Both send Positive to the first loop and Negative to the second, and refuse Component/Line with nearly the same desync text. The margin read that produces the branch is also written twice: the split lane's (`chord_join.rs:1619-1635`, K name `chord_section_loop`) and `SectionLoops::branch_at` (`join.rs:3286-3304`, K name `bool_join_section_loop`). Each turns `LoopSplit`'s margin into Positive/Negative. Line numbers are at PR 6929's head `a26425ee`.

Now that the branch selector is one shared type, the read-and-pick has one natural home beside `LoopSplit`. P2, by the `[topo.homes]` siblings: the two copies are the same logic in both engines, and a third caller would copy one of them.
