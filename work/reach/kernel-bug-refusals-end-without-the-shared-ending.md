---
id: kernel-bug-refusals-end-without-the-shared-ending
kind: issue
title: topo: the split's kernel-bug refusals end in a '(kernel bug)' tag, not the shared kernel-defect ending
status: open
opened: 2026-09-28
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`.)

## What

The kernel-defect ending has one home now:
`geom_core::KERNEL_DEFECT_RECOURSE` ("There is no way through: this is
a kernel defect; report it"), `KERNEL_OR_FILE_DEFECT_RECOURSE` for a
body that may have been read from a file, and the
`geom_core::kernel_defect_recourse!` macros for a `&'static str` site.
The split's kernel-bug refusals end instead in a "(kernel bug)" tag,
which carries neither the marker `test_utils::refusal::recourse_markers`
counts nor the report:

- `topo::splitting::SplitReduceError::ConsecutiveOnSectors`
  (`crates/topo/src/splitting/mod.rs`).
- `topo::splitting::SplitFinishError::TornComponent`
  (`crates/topo/src/splitting/finish.rs`).
- `topo::chord_join::SplitJoinError::UnpairedLooseEnds`,
  `SectionLoopMixed`, `CutInvariant` (`crates/topo/src/chord_join.rs`;
  shared with TANG by territory).

All are rows in `editor-core/tests/refusal_concision_chains.rs`
(`KERNEL_KEYED` admits their keys), and all render zero markers.

## Repair shape

Replace each tag with ". {KERNEL_DEFECT_RECOURSE}" (or the file
variant where the body may have been read).
