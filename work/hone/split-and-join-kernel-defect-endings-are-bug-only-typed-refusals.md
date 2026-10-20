---
id: split-and-join-kernel-defect-endings-are-bug-only-typed-refusals
kind: issue
title: topo: nine split, join and piece-sort refusals end in KERNEL_DEFECT_ENDING for states only a kernel bug reaches
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

Doors: the split doors, and the Boolean doors through the join, the
containment reads and the piece sort.

- `crates/topo/src/chord_join.rs` `SplitJoinError` `Display`,
  `LineageRow` (~668). **Class 2, likely**: the join appends these rows
  itself, and `lineage::RowRefusal` is refused at append.
- `crates/topo/src/splitting/mod.rs` `SplitError` `Display`,
  `LineageRow` (~792). **Class 2, likely**, for the same reason.
- `crates/topo/src/splitting/containment.rs`
  `PointInLoopError::render_as`, `CorruptLoop` (~492). **Class 2,
  likely**: a torn body, and tier 1 holds at every public door
  (`[failure]`'s closure property).
- `crates/topo/src/splitting/finish.rs` `FinishError` `Display`,
  `NestingContradiction` (~839). **Class 2, likely**: the variant's own
  doc says outlines decided disjoint cannot each enclose the other.
- `finish.rs` `FinishError` `Display`, `SlitUnplaced { in_hole: false }`
  (~848), and `SlitUnplaced { in_hole: true }` (~858). **Class 2,
  likely**: the variant's doc says a section region of the side covers
  the slit's edges.
- `crates/topo/src/splitting/section.rs` `SectionError` `Display`,
  `NestingContradiction` (~250). **Class 2, likely**, as `finish.rs`'s.
- `section.rs` `SectionError` `Display`, `Corrupt` (~255). **Class 2,
  likely**: a traversal over a tier-1 body does not fail.
- `crates/topo/src/pieces.rs` `PieceSortError` `Display`, `Move`
  (~169). **Class 2, likely**: the sort is moving a piece it has just
  found into its own solid. `pieces.rs` is in no program's territory,
  and HONE's charter is the nearest.

`work/hone/kernel-bug-refusals-end-without-the-shared-ending.md` covers
the same files' "(kernel bug)" tags. Its repair, which appends this
constant, is the shape this row retires; see the note there.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.
