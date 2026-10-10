---
id: the-world-still-speaks-of-roots
kind: issue
title: Under the world the kernel still names placements roots: CheckFinding.root, ChecksError::Root, PartRoot*, ProductError::Root*, "part root" wording
status: open
opened: 2026-10-08
---


Found by INTENT stage 2 unit C (branch `intent/s2-c-world`), which
retired A10's root list: the product is the copies its placements make.
C reworded the sentences it touched ("placement N output 0" where a
check said "root N output 0") but left the kernel's own names, which
other lanes' bindings and tags read:

- `CheckFinding::root` and `CheckEvidence::NotSeparated { other_root, .. }`
  (`crates/editor-core/src/checks.rs`) hold placement ids;
  `ChecksError::Root` (its text says "checks: placement …").
- `ProductError::RootInvalid`, `RootFailed`, `RootPoisoned`
  (`crates/editor-core/src/product.rs`) are refusals about a placement.
- `PartFault::PartRootFailed` / `PartRootPoisoned` and the "part root"
  wording (`crates/editor-core/src/eval/parts.rs`); the viewer reports
  the part refusal saying "poisoned its root, PlaceInWorld …".
- The Python tags that mirror them (`root_failed`, `root_poisoned`,
  `root_invalid`, `root_without_value`, `part_root_poisoned`).

Also seen by the viewer lane: `resolve` of a vanished name under a
`Placed` wrapper names no node ("a face it derives from vanished
upstream first", `crates/editor-core/src/resolve/mod.rs`), and a check
finding is spoken by the placement's kind noun ("PlaceInWorld <tag>
output 0") rather than by the body it places.
