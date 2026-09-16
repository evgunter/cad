---
id: three-door-predicates-are-hand-copied-not-shared
kind: issue
title: Three more node predicates are hand-copied at both doors instead of shared: the assertion bound, a mate's alignment, a placement's frame
status: review
opened: 2026-09-16
branch: edit/one-predicate-round-two
pr: 2772
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

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/one-predicate-round-two`. The move is the one
`blend-selection-canonical-check-load-only` made: one predicate with
one home, asked by both doors, each door naming the answer in its own
vocabulary. This unit also RUNS the sweep row
`load-shaped-doors-outside-check-rs-may-duplicate-edit-predicates`
and closes it with its table.

1. **The assertion's bound.** One function over `(&Doc, &Node)` — a
   method cannot, the check needs the document, which is the row's own
   reason — with a two-arm answer (target is not a `Measure`; target
   is, and the dimensions differ). `check_node_slots`'s inline block
   and `check.rs`'s `AssertionBound` arm both call it; the load door's
   `measured: Option<Dimension>` fold, which cannot say which of the
   two happened, becomes the two-arm answer in `SnapshotError`'s own
   vocabulary. `SnapshotError` is persisted-refusal vocabulary, so the
   F6 census and the `pncad-py` prose/tag censuses follow (LIB's files,
   mechanical; say so).
2. **A mate's alignment is finite.** One predicate beside the mate's
   own type; `edit.rs`'s `InsertNode` arm and `check.rs`'s second node
   walk both ask it; the second walk goes if the main loop can ask it.
3. **The placement registry.** One predicate over a placement (site,
   finiteness, determinant sign), both doors, each naming its arms.
   The gauge rule is the decision, and it is decided by precedent, not
   here: measure whether `SetPlacement` can ACCEPT a placement
   `validate_document` then refuses as `PlacementNotGauge`. If it can,
   that is the class the blend row fixed — a document the edit door
   builds and the load door refuses — and the edit door refuses through
   the same predicate. If the gauge is established by A11
   reconciliation AFTER the edit, so an edit-time check would refuse a
   document the reconciliation was about to make right, say so at both
   sites (present tense, the invariant, not the history) and the load
   door's rule stays alone. Report which, with the fixture that
   decided it.
4. **The sweep the other row asks for.** For each door that admits a
   document or a node from outside the edit log — `persist/wire.rs`,
   `persist/mod.rs`'s `load`/`save` seats, `program.rs`'s replay — list
   every refusal it can produce and whether an edit door decides the
   same predicate. Fix an in-fence hit the same way. `persist/wire.rs`
   is PORT-announced (`load-path-stringifies-structured-refusals`
   lands on its `Error::custom`): a hit there is FILED with that row
   cited, not fixed, so two programs do not restructure one error type
   at once. Close the sweep row with the table in a `## Closed`
   section; the blind spot the table cannot see is stated.
5. **Rows.** Per predicate: one fixture the edit door refuses and the
   load door refuses for the same fact, asserting both refusals by
   name; and a mutant that breaks the shared predicate reds BOTH doors'
   rows — that is the property the move buys, and the row that shows
   it is the acceptance. Corpus goldens unmoved.

## Built (2026-09-16)

Three predicates, one home each, asked by both doors; each door names
the answer in its own vocabulary.

- **The assertion's bound** is `Node::assertion_bound_fault(&self, doc)`
  with a two-arm answer (`TargetNotMeasure`, `DimensionMismatch`).
  `check_node_slots`'s inline block and `validate_snapshot`'s
  `AssertionBound` arm both call it. The load door's
  `measured: Option<Dimension>` fold is gone: `SnapshotError` now has
  `AssertionTarget` beside `AssertionBound`, carrying the dimensions it
  actually knows, so nothing has to decode an absent one.
- **A mate's alignment** is `Node::has_non_finite_alignment`, over
  `Alignment::is_finite` (which was already the one rule — what was
  duplicated was the node-level destructuring and the walk). `check.rs`'s
  second pass over `doc.nodes` is gone; the question is asked in the
  main loop.
- **The placement registry** is `doc::placement_fault(doc, node, frame)`
  — site, finiteness, determinant sign — asked by `SetPlacement` and by
  the load door's walk. The load door's `PlacementFrame` fold is gone
  too: `PlacementNonFinite` and `PlacementImproper` name the two facts
  the edit door has always named apart.

**The gauge, decided by measurement, not by this unit.** `SetPlacement`
cannot accept a placement `validate_document` then refuses as
`PlacementNotGauge`, and not because reconciliation repairs it after
the fact: the edit door does not REFUSE a non-gauge key at all, it KEYS
THE ROW ON THE GAUGE (`crate::mate::gauge_of`), and
`mate::solve::reconcile` re-keys the whole registry on every edit that
moves the mate graph. A non-gauge row is therefore unrepresentable
through the edit doors, and an edit-time refusal would refuse a
placement the edit door instead accepts and normalises. Said at both
sites in the present tense; the load door's rule stays alone. The
fixture that decided it is
`a_placement_off_the_gauge_is_keyed_on_it_rather_than_refused`: two
mated instances, `SetPlacement` authored against the LATER one, the row
landing on the gauge, and the file that carries the key the edit door
rewrote refusing `PlacementNotGauge` at load.

**Rows**, in `crates/editor-core/tests/edit_one_predicate.rs` — one per
FACT, naming both doors' refusals for it, since a row that asked one
door could not see the two drift. Five facts: a non-measure target, a
mismatched bound dimension, a non-finite alignment, a placement on a
non-instance, an improper frame; plus the gauge measurement above.
Two arms are unreachable from a FILE — JSON carries no non-finite token
— so their save-door halves are pinned in-crate beside the validator
(`persist::check`'s `non_finite_alignment_and_placement_refuse_at_save`,
which builds through `apply` and then breaks one coordinate).

Mutants run, each reverted: `assertion_bound_fault` answering `None`
reds both assertion rows; `has_non_finite_alignment` answering `false`
reds the alignment row AND the in-crate save row; `placement_fault`
answering `None` reds both placement rows AND the in-crate save row.

**Deviations from the spec**, both argued rather than scheduled:

1. The spec asks for the assertion predicate as "one function over
   `(&Doc, &Node)` — a method cannot, the check needs the document".
   The premise is false, and the row's own evidence says so:
   `Node::bad_declare_input(&self, doc)` is one of the four DELEGATED
   predicates the row lists, and it is a method that takes the
   document. Built as a method for that reason — the four shared
   node predicates now have one shape between them.
2. The spec's point 3 says "each naming its arms" for the placement but
   does not say the load door's `PlacementFrame` fold goes. It does:
   folding "a coordinate no predicate can read" into "a mirror this
   build declines to admit" is the same defect as the assertion's
   `Option`, and it renders `determinant NaN` for the first. Split into
   `PlacementNonFinite` and `PlacementImproper`.

**Outside EDIT's fence, mechanical:** `crates/pncad-py/src/tags.rs`
(three tag words for the three new arms, one retired),
`crates/pncad-py/src/tests.rs` (the committed tag inventory) and
`crates/pncad-py/tests/test_binding_census.py` (a hand-carried arm
count in prose, which was already stale by one before this change —
deleted rather than re-counted, the exhaustive match being what
enforces it). Those are LIB's files.

**Filed:** `load-door-checks-slot-dimensions-for-profile-nodes-only`
(this program's slate) — the sweep's one hit.
