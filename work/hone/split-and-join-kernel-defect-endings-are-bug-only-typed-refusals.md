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

(ENCL implementer, from the census in
`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`,
on main 5df6e6c854.)

`[failure]` (D2) splits every state in two: one input reaches, which
is a typed refusal whose ending must name the decision's lever, and one
only a kernel bug reaches, which MUST panic ("downgrading one … to a
typed error launders a bug into a supported outcome"). A typed refusal
ending in `geom_core::KERNEL_DEFECT_ENDING` is therefore wrong in either
class. The one exception is a kernel approximation limit, which takes
`[tolerance.recourse]`'s last resort (`KERNEL_LIMIT_RECOURSE` /
`KERNEL_LIMIT_LAST_RESORT`), not this ending.

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

- **Class 2** (only a kernel bug reaches it): `unreachable!` or
  `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0
  first (a type that cannot hold the state is preferred). The tests
  that pin the refusal's text move to `#[should_panic]`, or are deleted
  where they only built the error value by hand.
- **Class 1** (input reaches it): the decision's own lever, written in
  its own words (`[tolerance.recourse]` (i)), and a test that reaches it
  through the door.
- **Class 3** (an approximation limit): the last-resort recourse.

Re-check each `likely` or `unsure` argument before acting on it: one
that fails moves the site to class 1.
