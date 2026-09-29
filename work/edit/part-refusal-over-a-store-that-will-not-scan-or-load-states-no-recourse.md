---
id: part-refusal-over-a-store-that-will-not-scan-or-load-states-no-recourse
kind: issue
title: editor-core/pncad: a part whose store will not scan, or whose file will not load, draws the store's sentence with no recourse and no roster row
status: open
opened: 2026-09-29
priority: P3
cost: M
---


(EDIT, found by `edit/part-refusal-recourse` sweeping every store
refusal `PartFault::Unresolved` can carry.)

## What

`PartFault::Unresolved` (`crates/editor-core/src/eval/parts.rs`,
`impl Display for PartFault`) renders its resolver's message. The
shipped resolvers now carry the store's sentence without its stage word
(`WorkspaceError::sentence`, `PersistError::sentence`) and state a
recourse for the arms a resolution meets most: an unknown id, an
unreadable file, a pin that would not compute, a moved pin, and the ε
seam (`crates/pncad/src/workspace.rs`, `resolve_recourse`). The
real-text rows are `crates/viewer/tests/instance_authoring.rs`,
`every_unresolved_part_badge_meets_the_refusal_standard`.

Two families reach the same badge with no recourse and no row:

- **The viewer's scan** (`crates/viewer/src/docio.rs`,
  `impl PartResolver for DirResolver`): `WorkspaceError::Io` on the
  directory, `Header` and `DuplicateId`. `DuplicateId`'s sentence also
  opens on a label-shaped clause ("duplicate document id …:"), which
  `test_utils::refusal::stage_prefixes` flags, and so does `Io`'s
  ("io error at `…`:").
- **A part file that will not load** (`WorkspaceError::Load` over every
  `PersistError` but `ToleranceConflict`): the load door's sentence,
  whose recourse is unlabelled where it exists (`REGENERATE_RECOURSE`
  on `Unreadable` and `HeaderId`) and absent elsewhere. `EditReplay`
  forwards the edit door's recourse, which
  `persist-edit-replay-forwards-the-edit-doors-recourse` already holds.

## What would close it

Real-text rows for each family through the viewer's resolver (the scan
arms by a junk `.pncad` file and a copied part; the load arms by a part
file damaged in each way the load door names), then the recourse each
supports, stated where `resolve_recourse` states the others, or in the
load door's sentence where the load door's own refusal lacks one.
`WorkspaceError` is LIB's text; the seam is announced on its PR.
