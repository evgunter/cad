---
id: engine-leaves-by-visibility
kind: unit
title: "[topo.homes] held by the compiler once the moves land: every engine item narrower than pub(crate) unless it is a lib.rs door, so root code cannot name it; the gate shrinks to the visibility check, the door check and a door-name check, and the token scanner retires"
status: parked
opened: 2026-11-03
priority: P2
cost: M
blocked_on: [crossing-doors-leave-topo-for-geom-brep]
---


Filed by the ONEDOOR orchestrator on 2026-11-03, from PR 6458's three reviews. Each review found that `engine-leaves`' token-level counter, an approximation of Rust's name resolution, could be satisfied without a real move. Each fix added a branch, and the next spelling slipped past.

**The final state that ends this.** Rust's visibility does the work:
- An item inside an engine directory is either `pub` and re-exported by `lib.rs` (a door), or narrower than crate-visible. The narrower form is `pub(in crate::boolean)`, or `pub(super)` that stays inside.
- Then the compiler refuses any root reference to a non-door item.
- The gate shrinks to three checks:
  - no crate-visible item in an engine, which it already does;
  - every `pub` engine item is re-exported by `lib.rs`;
  - root code names no door, by name over the small, distinctive door set.
- Engine ↔ engine references follow the same pattern: an engine reaches the other only through its doors, and a door-name check covers that.

It waits until the moves have emptied most of the allowlist. Until then the scanner, with its stated blind spots, is the migration device. Banded P2: it hardens a guard that already stops the plausible spellings.

The blocker listed is the first move unit; the real trigger is "the allowlist is near empty", re-read when the successor program's library moves land.
