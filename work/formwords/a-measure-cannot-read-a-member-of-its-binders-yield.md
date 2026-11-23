---
id: a-measure-cannot-read-a-member-of-its-binders-yield
kind: issue
title: A measure inside a binder cannot read the member of the binder's yield: a sited reference carries no index
status: closed
closed: 2026-11-23
opened: 2026-11-23
---


D10's per-copy assertion has a gap. The row
`crates/editor-core/tests/g_index_families.rs`
`one_assertion_over_a_family_quiets_each_members_own_finding` writes
`{k in 0..n: fins = place(fin); gap = measure(fins.start, base.end);
assert gap ≤ −0.1}`. Since PR 7506 it cannot be built:

- The fins must be the binder's yield, or the world's copies (the
  product) cannot gather them: containment refuses a product on a
  k-reader no binder yields.
- A yield is a family to every reader, so the measure's sited reference
  to `fins` is refused by kind (`SlotVarKind`, found `Bodies`, at
  `Operand(Measured(Gap, 0))`).
- `SitedRef` (`crates/editor-core/src/node.rs`) carries a node and a
  name but no index, so `fins[k]` cannot be written at a measure seat.

A node's body seat reads the member as `xs[k]` (DM3). A measure's sited
reference needs the same read, or the measure must read the member some
other way. The row is held `#[ignore]` against this item.

**Closed (2026-11-23).** The measure reads the member as a selection of
`fins[k]`: `Operand::select` over the member read at the binder's own key.
The at-rest quieting takes such a selection as the member the assertion
stands at: `checks::at_rest::reference`, via
`Doc::reads_own_member`. The row is un-ignored and green.
