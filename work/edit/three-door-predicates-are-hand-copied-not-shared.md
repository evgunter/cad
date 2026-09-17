---
id: three-door-predicates-are-hand-copied-not-shared
kind: issue
title: Three more node predicates are hand-copied at both doors instead of shared: the assertion bound, a mate's alignment, a placement's frame
status: closed
opened: 2026-09-16
branch: edit/one-predicate-round-two
pr: 2772
refs: [blend-selection-canonical-check-load-only, check-rs-hand-copied-predicate-sweep-undercounts]
closed: 2026-09-16
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

## Built, fix pass (2026-09-16) — three more predicates, and two rules made structural

The style review's verdict was APPROVE-WITH-FIXES and every finding is
taken. What the fix pass added to the unit:

**Three more predicates, one home each, both doors** — the review's
MAJOR-1, each the shape the unit already had in hand:

- **A witness row's key** is `doc::witness_site_fault(doc, node)` with a
  two-arm answer (`NoSuchNode`, `NotSketchBearing`).
  `edit.rs`'s `check_witness_site` and `validate_snapshot`'s walk over
  the store both call it. The load door's fold is gone with it:
  `SnapshotError::WitnessSite` now means "bears no sketch" and
  `WitnessOnMissingNode` is its own arm, as the edit door has always
  named the two apart.
- **The recorded ε** is `doc::epsilon_admissible(eps)`, asked by
  `SetTolerance` and by the snapshot walk. A free function beside the
  field, because the edit door decides the value before it is a
  document's ε.
- **A continuous parameter declared `Count`** is
  `DocParam::is_continuous_count`, asked by `SetDocParam` and by the
  snapshot walk.

**The two weaker pairs, measured rather than assumed.** Both are
irreducible and now say so at both sites. `MetadataUnversioned` already
shares its RULE (`MetaValue::require_versioned`); what differs is the
walk, and a walk over the one value a door is about to write is not a
walk. `DanglingInput` against `UnresolvedInput` is `contains_key` on
the node map at both sites — there is no predicate between them to give
a home to, only a different subject.

**The frame rule, once** (S1). `Frame::admission_fault` on
`placement.rs` is A11/A6's frame half — finite, orientation-preserving
— and `doc::PlacementFault` and `node::PlacementRuleFault` keep their
arms and delegate to it. Its `Display` is one predicate CLAUSE, so
every door that refuses a frame forwards the same sentence into its own
subject ("placement 2 …", "the placement frame on node 7 …").

**The gauge's second leg, structural** (MINOR-1).
`DocEdit::moves_the_mate_graph` is an exhaustive match with no wildcard
arm, and `apply` reads it instead of four hand-set `reconcile = true`
assignments. A new edit arm now fails to compile until it answers
whether it moves the mate graph — which is the invariant that keeps
`PlacementNotGauge` unreachable from the edit doors. Pinned by
`edit::tests::exactly_the_graph_moving_edits_ask_for_reconciliation`.

**The evaluation backstop reaches the same rule** (S8).
`eval::wire`'s `wire_assertion` spelled E10's agreement a third time;
it now decides through `AssertionBoundFault::against`, the
measured-dimension entry point `Node::assertion_bound_fault` reaches
once it has resolved the reference.

**`SnapshotError` gets the exhaustive F6 census** (S2) —
`snapshot_error_display_names_its_content_not_its_struct`, over all
twenty-one arms, so a new arm cannot land unrendered. The two placement
frame arms left the dimension-word row, which is about dimension words.

**Rows added**, in `crates/editor-core/tests/edit_one_predicate.rs`:
a witness on a non-sketch node, a witness on a missing node, a
non-positive ε, and a continuous parameter declared `Count` — each
naming both doors' refusals for one fact. The suite's module doc no
longer claims that a file which loads is a file the edit door could
have produced: it states what the shared predicates actually buy and
cites the slot-dimension gap as the known exception.

**Mutants**, each applied and reverted, at `f305afee6`:

| Mutant | Rows it reds |
| --- | --- |
| `assertion_bound_fault` → `None` | both assertion rows |
| `assertion_bound_fault`'s non-measure arm → `DimensionMismatch` | the non-measure row |
| `has_non_finite_alignment` → `false` | the alignment row AND the in-crate save row |
| `placement_fault` → `None` | both placement rows AND the in-crate save row |
| `Frame::admission_fault` → `None` | the improper-placement row, the in-crate save row AND `lib_placedunion::placement_frames_are_held_to_the_cluster_frame_bar` |
| `Frame::admission_fault`'s two arms swapped | the same three |
| `witness_site_fault` → `None` | both witness rows |
| `witness_site_fault`'s two arms swapped | both witness rows |
| `epsilon_admissible` → `true` | the ε row AND the in-crate save row |
| `is_continuous_count` → `false` | the count row |
| `moves_the_mate_graph` with `UpdateReference` removed | **does not compile** (`E0004`, the arm named) |

**CI**: run 35119197792 on head `ecf9fa0de` — success, 39 jobs, twelve
`test (…)` and five `k-lint (gate, …)`, the python suite green, zero
non-success step conclusions.

**Filed by the fix pass**, on this program's slate:
`mate-head-entity-kind-is-decided-only-at-assembly` (the review's
NOTE-4 — a V1 class-2 gap, not a duplication),
`count-continuous-arm-is-shadowed-by-the-display-unit-walk` and
`doc-param-float-walk-is-hand-written-at-both-doors` (both disclosed by
the fresh census below).

**Outside EDIT's fence, mechanical:** `crates/pncad-py/src/tags.rs` and
`crates/pncad-py/src/tests.rs` gain one tag word
(`witness_on_missing_node`) for the new arm. LIB's files.

## Census (2026-09-16, re-read at `edit/one-predicate-round-three`'s fix pass)

**The census is the code's now.** Twice in three rounds a hand-written
table of this door's refusals undercounted — round two by three
predicates, round three by an entire walk
(`first_slot_param_ref_fault`) and its two refusals — so the half a
table can get wrong no longer lives in a table:

- **The walks** are `persist::check`'s `Walk`, one variant per walk,
  in call order. `Walk::ORDER` is what `validate_document` iterates,
  so the roster IS the call order rather than a description of it, and
  `Walk::run` is the exhaustive map from a walk to the function behind
  it and the refusal it raises. A walk added to `validate_document`
  does not compile until both are answered.
- **The refusals** are placed in their walk by
  `check::tests::every_snapshot_error_arm_names_the_walk_that_produces_it`,
  an exhaustive match over `SnapshotError` with no wildcard arm, welded
  to the enum by two `f6_variants!` rosters and compared in both
  directions: a new arm does not compile until it is placed, and a walk
  that refuses with a `SnapshotError` and no arm reds.

What the code cannot know is the column below: the EDIT-door twin of
each refusal, and whether the predicate behind it is DELEGATED (one
function both doors call) or HAND-WRITTEN at this door. **Read at this
branch's head**, one row per refusal `validate_document` can produce,
in `Walk::ORDER`.

| Walk | Refusal | Predicate | Edit-door twin |
| --- | --- | --- | --- |
| `NonFinite` | `NonFinite { site: Epsilon }` | hand-written (`!epsilon.is_finite()`) | `SetTolerance`'s `InvalidTolerance`, whose rule is `doc::epsilon_admissible` — **this walk asks the finiteness half a second time**, and shadows the `EpsilonInvalid` row below for a non-finite ε. Kept: the two report different facts (WHICH float the format cannot write, against WHICH rule the ε breaks), and the float walk is the D2 round-trip class. |
| `NonFinite` | `NonFinite { site: DocParam }` | **delegated** — `DocParam::first_non_finite` | `SetDocParam`/`SetDocParamValue`/`SetDocParamDistribution`'s `NonFiniteDocParam`, through `write_doc_param`'s shared tail, carrying the same `DocParamField`. |
| `NonFinite` | `NonFinite { site: Metadata }` | **delegated** — `MetaValue::first_non_finite` | `SetAppearanceMeta`'s `MetaNonFinite`, same function. |
| `NonFinite` | `NonFinite { site: Edit }` | **delegated** — the same `edit_non_finite` over the log | None, and there cannot be: the edit door sees one edit, not a log. Its `SetDocParamValue` arm is the one float site here that CANNOT delegate to `DocParam::first_non_finite` — the payload is a bare `f64`, not a parameter — and says so at the site; what it shares is the site vocabulary. |
| `Distribution` | `Distribution` | **delegated** — `Distribution::check` | `SetDocParam`'s `InvalidDistribution`, same function. |
| `DisplayUnit` | `DisplayUnit` | **delegated** — `UnitSym::measures` | `SetDocParamUnit`'s `DocParamUnitMismatch`, same function. **And the load door's answer for a continuous parameter declared `Count`**: this walk refuses it whatever notation it carries, which is why the structural/continuous divide has no arm of its own here. |
| `SlotDimension` | `SnapshotError::SlotDimension` | **delegated** — `Node::slot_dimension_fault` over `Node::slots()` for EVERY node kind, per slot through `SlotId::dimension_fault` | `check_node_slots`'s `SlotDimensionMismatch`, and `set_slot`'s — the same per-slot comparison, asked of an expression the node does not hold yet. One `Display` clause (`SlotDimensionFault`) forwarded into each door's subject. |
| `SlotParamRef` | `SlotUnknownDocParam` / `SlotDocParamDimension` | **delegated** — `Doc::param_ref_fault` | `check_param_refs`'s `UnknownDocParam` / `DocParamDimensionMismatch`, and the payload-expression walk beside it (`UnknownPayloadParam` / `PayloadParamDimensionMismatch`) — three callers, one rule. The PAYLOAD half is not asked at this door: `load-door-does-not-check-payload-expression-param-refs`, measured by `rv_onepred3_probes::rv_a_measure_expression_reading_an_undeclared_parameter_still_loads`. |
| `Program` | `ProgramRefusal::Transition` | **delegated** — `ProfileProgram::check`, which `InsertNode`'s VQ9 door also calls | `EditError::ProfileProgramRefused`. A step argument's DIMENSION left this walk for the slot walk above, and `ProgramFault::SlotDimension` is gone with it. |
| `Snapshot` | `EpsilonInvalid` | **delegated** — `doc::epsilon_admissible` | `SetTolerance`'s `InvalidTolerance`. |
| `Snapshot` | `OrderMismatch` | hand-written | **No twin.** `order` against the node map is a file's fact; `apply` maintains the two together by construction. |
| `Snapshot` | `IdBeyondCounter` | hand-written (`check_id`, one closure, six call sites) | **No twin.** The mint counter is `apply`'s own monotone state. |
| `Snapshot` | `DanglingInput` | hand-written | `InsertNode`/`SetMembers`'s `UnresolvedInput`. **Irreducible**: the rule IS `doc.nodes.contains_key`, which both sites already call — there is no predicate between them to extract, only a different subject. Said at both sites. |
| `Snapshot` | `ForwardInput` | hand-written | **No twin.** Insertion order is topological by construction at the edit door. |
| `Snapshot` | `PlacementRule` | **delegated** — `Node::placement_rule_fault` | `InsertNode`'s rule arms. Its frame half is `Frame::admission_fault`, shared with the placement rows below. **It is also what refuses a `PlacedUnion` whose count spelling and rule disagree** (`CountSpelling`): such a node carries no count SLOT, so the slot walk passes over it rather than answering for an address `Node::expr` cannot read. |
| `Snapshot` | `InputList` | **delegated** — `Node::input_fault` | `InsertNode`/`SetMembers`. |
| `Snapshot` | `MeasureRefs` | **delegated** — `Node::measure_fault` | `InsertNode`. |
| `Snapshot` | `DeclareInput` | **delegated** — `Node::bad_declare_input` | `InsertNode`. |
| `Snapshot` | `AssertionTarget` / `AssertionBound` | **delegated** — `Node::assertion_bound_fault` | `check_node_slots`'s `AssertionTarget` / `AssertionDimension`; the evaluation backstop reaches the same rule through `AssertionBoundFault::against`. |
| `Snapshot` | `MateAlignment` | **delegated** — `Node::has_non_finite_alignment` | `InsertNode`'s `NonFiniteAlignment`. |
| `Snapshot` | `WitnessSite` / `WitnessOnMissingNode` | **delegated** — `doc::witness_site_fault` | `ReWitness`/`ReWitnessBulk`'s `WitnessOnNonSketch` / `UnknownNode`. |
| `Snapshot` | `PlacementSite` / `PlacementNonFinite` / `PlacementImproper` | **delegated** — `doc::placement_fault`, over `Frame::admission_fault` | `SetPlacement`'s three arms. |
| `Snapshot` | `PlacementNotGauge` | hand-written | **No twin, and that is the invariant.** `SetPlacement` keys the row on the gauge instead of refusing a non-gauge key, and `mate::solve::reconcile` re-keys the registry after every edit that moves the mate graph — which `DocEdit::moves_the_mate_graph` decides exhaustively. Said at both sites. |
| `Snapshot` | `Roots` | **delegated** — `roots::check` | `SetRoots`, `on_insert`, `on_delete`, `on_set_members`. |
| `Snapshot` | `MetadataUnversioned` | **delegated** — `MetaValue::require_versioned` | `SetAppearanceMeta`'s `MetaUnversioned`. **The rule is shared; only the WALK differs, irreducibly**: the edit door holds the one value it is about to write, and this door holds a map that arrived whole. Said at both sites. |

**2026-09-17** — the `SlotParamRef` row's last sentence is out of date: the PAYLOAD half IS asked at the load door now (`edit/load-door-payload-refs`, PR #2793, a new `PayloadParamRef` walk after this one), so `Doc::param_ref_fault` has four callers rather than three, and the probe that row cites has moved — the fact is `load_door_payload_param_ref::a_measure_expression_reading_an_undeclared_parameter_refuses_to_load`, not `rv_onepred3_probes::rv_a_measure_expression_reading_an_undeclared_parameter_still_loads`, which is gone.

**The walk ORDER is a contract**, not an implementation detail: a
document broken in two ways at once is refused by the EARLIER walk, so
that is the answer every caller comparing two doors reads, and moving
a walk changes the diagnosis of every file broken both ways.
`validate_document`'s rustdoc says so and
`rv_onepred3_probes::rv_the_slot_walk_shadows_a_structural_refusal_it_did_not_shadow_before`
is the row that pins the class.

**What this census cannot see.** It reads `validate_document` and the
functions it calls. A predicate duplicated between an edit door and a
door that admits a document by ANOTHER route is invisible to it — the
blind spot the companion row
(`load-shaped-doors-outside-check-rs-may-duplicate-edit-predicates`)
names and partly runs, whose own residue is `assembly.rs`'s mint and
instantiate seats, the workspace store's scan, and
`ProfileProgram::from_recorded`. The twins column is also prose, which
nothing compiles: the code roster cannot be wrong about the walks or
the arms, and only a reader can be right about a twin.

**Residue disclosed by this census, filed rather than left here**:
`doc-param-float-walk-is-hand-written-at-both-doors`,
`count-continuous-arm-is-shadowed-by-the-display-unit-walk` and
`load-door-checks-slot-dimensions-for-profile-nodes-only`, all three
built and closed on it by `edit/one-predicate-round-three`; and the
two rows that unit's own review opened,
`load-door-does-not-check-payload-expression-param-refs` (open,
measured) and `set-slot-spells-the-slot-rule-a-third-time` (closed by
the fix pass). No row above is an open hit.

## Closed (2026-09-16, EDIT orchestrator)

Built and merged as PR #2772 after one opus style review
(APPROVE-WITH-FIXES: one MAJOR — the sweep's closing table undercounted
the load door's hand-copied predicates by three — three MINOR, four
NOTE, eight style findings; every one taken in the fix pass, two argued
with a measurement). Six predicates now have one home each, asked by
both doors — the assertion bound, the mate alignment, the placement
registry, the witness site, the recorded ε, the structural/continuous
divide — with the frame rule stated once beside `Frame`; the gauge's
second leg is an exhaustive match over `DocEdit` rather than a hand-set
flag; `SnapshotError` has an exhaustive F6 census; the evaluation
backstop reaches the shared assertion rule; and `validate_document`'s
refusals are a fresh census on this row (`## Census`), which is what
let the sweep row and the review's undercount row close. Two pairs
measured irreducible and said so at both sites. Residue in its own
files: `load-door-checks-slot-dimensions-for-profile-nodes-only`,
`count-continuous-arm-is-shadowed-by-the-display-unit-walk`,
`doc-param-float-walk-is-hand-written-at-both-doors`,
`mate-head-entity-kind-is-decided-only-at-assembly`.
