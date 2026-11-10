---
id: a-shell-whose-only-off-plane-vertex-is-lone-is-unplaced
kind: issue
title: splitting::finish::shell_side reads only edge members' start vertices, so a shell whose one off-plane vertex is a lone vertex is not placed
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
pr: 6643
---


## Finding

Filed by `onecopy/side-at-infinity` (PR 6595's review). `splitting::finish::shell_side`
gathers a shell's vertex rungs from `BoundaryMember::Edge { half, .. }`
members' `half.start`, and skips `BoundaryMember::Isolated`. A shell whose
only off-plane vertex is a lone-vertex loop therefore reads ON everywhere:
the uncut split refuses `Corrupt`, and `classify_shell` (the cut path's
uncut component, which reads through `shell_side`) refuses
`UnclassifiableComponent`. The fix is an `Isolated { vertex, .. }` arm
pushing the vertex's rung. `fate.rs`'s ring rehome already reads a
`LoopBoundary::Empty` vertex, so it is the model.
