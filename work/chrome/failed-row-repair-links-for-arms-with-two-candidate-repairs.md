---
id: failed-row-repair-links-for-arms-with-two-candidate-repairs
kind: issue
title: A Failed row links to no node for AxisInDifferentPlane, EmptyOperand and EmptyHalf, whose repair could be either of two nodes
status: open
opened: 2026-09-29
needs_ev: true
priority: P4
cost: E
design: true
---

## Question

What should a failed tree row link to when the kernel's failure names a second node that could be part of the fix? `EmptyOperand`, `EmptyHalf`, `InstanceOutOfRange` and `AxisInDifferentPlane` link nowhere today. `TreeRow::repair_at` holds one node, and the code comment says one link would pick for the reader. The choice is what a link under a failed row means, and so which nodes these four arms link to.

## Finding

`tree::repair_named` (PR 3450) answers every `NodeErrorKind`, but a
few arms have two candidate repairs and cannot be answered off the
kernel's doc alone, so they link nowhere today (the code site cites
this row):

- `AxisInDifferentPlane { axis, axis_plane, profile_plane }`. The
  kernel's doc says *"the fix depends on which one is wrong, and a
  reader looking at two node numbers can tell"*. Either frame may be
  the repair, and `TreeRow::repair_at` holds ONE node. The question: a
  link per candidate (a `Vec`), or none?
- `EmptyOperand { input }`, `EmptyHalf { input }`. The operand came out
  empty (a boolean that consumed everything, a split whose tool missed
  one side), so the author may repair the input, OR the failing node's
  choice of it. The kernel's doc names neither.
- `InstanceOutOfRange { input }`. A `Part` indexed past its pattern's
  count: the repair is the `Part`'s index or the pattern's count, the
  same two-candidate shape.

`crates/viewer/tests/tree_badges.rs` has a fixture shape
(`a_profile_refused_for_its_frames_direction_links_to_the_frame`) that
a row for either would copy.
