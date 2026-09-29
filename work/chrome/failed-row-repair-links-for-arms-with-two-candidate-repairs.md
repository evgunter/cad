---
id: failed-row-repair-links-for-arms-with-two-candidate-repairs
kind: issue
title: A Failed row links to no node for AxisInDifferentPlane, EmptyOperand and EmptyHalf, whose repair could be either of two nodes
status: open
opened: 2026-09-29
priority: P4
cost: E
design: true
---

## Finding

`tree::repair_named` (PR 3450) answers every `NodeErrorKind`, but a
few arms could not be answered off the kernel's doc alone, so they
link nowhere today:

- `AxisInDifferentPlane { axis, axis_plane, profile_plane }`. The
  kernel's doc says *"the fix depends on which one is wrong, and a
  reader looking at two node numbers can tell"*. Either frame may be
  the repair, and `TreeRow::repair_at` holds ONE node. The question: a
  link per candidate (a `Vec`), or none?
- `EmptyOperand { input }`, `EmptyHalf { input }`. The operand came out
  empty (a boolean that consumed everything, a split whose tool missed
  one side), so the author may repair the input, OR the failing node's
  choice of it. The kernel's doc names neither.

`crates/viewer/tests/tree_badges.rs` has a fixture shape
(`a_profile_refused_for_its_frames_direction_links_to_the_frame`) that
a row for either would copy.
