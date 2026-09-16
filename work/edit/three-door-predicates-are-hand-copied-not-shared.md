---
id: three-door-predicates-are-hand-copied-not-shared
kind: issue
title: Three more node predicates are hand-copied at both doors instead of shared: the assertion bound, a mate's alignment, a placement's frame
status: open
opened: 2026-09-16
refs: [blend-selection-canonical-check-load-only]
---

Found by the sweep behind `blend-selection-canonical-check-load-only`,
which moved the blend selection's canonical form off the load door and
onto `Node::input_fault` so the two doors ask one predicate. The sweep
read every `return Err(SnapshotError::…)` in
`crates/editor-core/src/persist/check.rs`'s `validate_document` and
asked, per site, whether the predicate is DELEGATED (a shared
`Node::*_fault` or module function both doors call) or hand-written at
this door. Four were delegated —
`placement_rule_fault`, `input_fault`, `measure_fault`,
`bad_declare_input`, plus `roots::check` — and three were not. All
three have an edit-door twin that spells the same rule a second time:

- **The assertion's bound dimension.** `check.rs`'s `AssertionBound`
  arm reads `doc.nodes.get(measure)`, takes `expr.dim()` and compares
  it to `bound.dim()`. `edit.rs`'s `check_node_slots` ends with an
  inline `if let Node::Assertion { measure, bound, .. }` block — no
  function of its own — that does the same walk and splits the answer
  into `EditError::AssertionTarget` (the reference is not a `Measure`)
  and `EditError::AssertionDimension` (it is, and the dimensions
  differ), where the load door folds both into one arm with
  `measured: Option<Dimension>`. Two spellings, and the load door's is
  the one that cannot say which of the two happened without the reader
  decoding a `None`. Its comment says the check "needs the DOCUMENT,
  which is why it lands here and not on the node" — that is the reason
  it is not already a `Node::*_fault`, and the shared home this row
  wants is therefore a function over `(&Doc, &Node)`, not a method.
- **A mate's alignment is finite.** `edit.rs`'s `InsertNode` arm tests
  `!alignment.is_finite()` inline for `EditError::NonFiniteAlignment`;
  `check.rs` runs a second node walk of its own, after the main loop,
  testing the same thing for `SnapshotError::MateAlignment`.
- **The placement registry's three.** `edit.rs`'s `SetPlacement` arm
  hand-tests the instantiate site (`PlacementOnNonInstance`), the
  frame's finiteness (`NonFinitePlacement`) and its determinant sign
  (`ImproperPlacement`) as three refusals; `check.rs`'s walk over
  `doc.placements` tests the site again (`PlacementSite`) and folds
  finiteness and determinant into one (`PlacementFrame`), then adds a
  gauge rule the edit door does not ask at all
  (`PlacementNotGauge`) — so here the two doors do not even refuse the
  same set, which is the asymmetry the blend row named.

The fix is the same move each time: one predicate with one home, asked
by both doors, each door naming the answer in its own vocabulary. The
gauge rule is the one that needs a decision rather than a move —
whether `SetPlacement` should refuse a non-gauge placement is a
question about the edit, not about where a check lives.

*Filed by the `edit/blend-canonical` lane; the sweep's blind spot is
that it matched only sites spelling `SnapshotError` inline in
`validate_document`, so a duplicated predicate in another load-shaped
door (`persist/wire.rs`, `program.rs`'s replay) is not covered by it.*
