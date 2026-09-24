---
id: load-door-checks-slot-dimensions-for-profile-nodes-only
kind: issue
title: The load door re-spells the slot-dimension predicate and asks it of profile nodes only
status: closed
pr: 2780
branch: edit/one-predicate-round-three
opened: 2026-09-16
closed: 2026-09-16
---


Found by the sweep behind
`load-shaped-doors-outside-check-rs-may-duplicate-edit-predicates`,
which the `edit/one-predicate-round-two` unit ran. The original sweep
matched `return Err(SnapshotError::…)` sites, so a duplicated predicate
that refuses in a DIFFERENT typed vocabulary from the same file was
invisible to it. There is one.

**The predicate.** "A slot's expression carries the dimension the slot
address fixes" (`SlotId::dimension`, spec D6) is spelled twice:

- `crates/editor-core/src/edit.rs`, `check_node_slots` — walks
  `Node::slots()` for EVERY node kind, refuses
  `EditError::SlotDimensionMismatch`, and where `Node::expr(slot)`
  answers `None` refuses `EditError::UnknownSlot` (a vocabulary bug
  surfaced, not hidden). It then runs `check_param_refs` on the same
  expression.
- `crates/editor-core/src/persist/check.rs`, `first_program_fault` —
  walks `program.slots()` for `Node::Profile` nodes ONLY, refuses
  `ProgramFault::SlotDimension`, and `continue`s where the expression
  is absent.

`Node::slots()` answers `p.slots()` for `Node::Profile(p)`, so the load
door's walk is a RESTRICTION of the edit door's, re-written: the same
comparison, a second vocabulary, a narrower domain and a different
answer for the missing-expression case.

**What the narrowing costs.** A file carrying an `Extrude` whose
`distance` expression is an angle, a `Fillet` whose `radius` is a
count, a `Datum::Plane` whose origin is dimensionless — every
non-profile slot in the vocabulary — passes the load door, because
nothing outside `first_program_fault` asks the question and that walk
skips every node that is not a profile. MEASURED, not inferred: a
saved one-extrude document whose distance literal is retyped on the
wire from `Length`/`m` to `Angle`/`rad` (both halves moved, so the
literal stays well-formed through `Expr::literal_with_unit`) LOADS
clean, and the edit door refuses the same node. That is the asymmetry
`three-door-predicates-are-hand-copied-not-shared` named one class up:
a document the load door admits that the edit doors could not have
produced.

**Why the unit that found it did not fix it.** The move is the same
one — one predicate with one home, each door naming the answer — but it
is not mechanical, because the two doors do not currently ask the same
QUESTION and unifying them is a decision:

- the missing-expression case (`UnknownSlot` versus `continue`);
- whether the load door also asks `check_param_refs` (a slot
  expression referencing an undeclared parameter, or one declared with
  another dimension), which today it never does;
- which vocabulary survives — `ProgramFault::SlotDimension` is
  persisted-refusal vocabulary with a stable Python tag, and widening
  it past profile programs makes its name wrong.

Each of those changes what a file is allowed to carry, so the row is a
unit of its own rather than a follow-through.

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/one-predicate-round-three`. Round three of the move the
blend unit made and round two made six times: one predicate, one home,
both doors, each naming the answer in its own vocabulary. Three rows in
one unit, with the decisions each row said it needed RULED here — by
the precedent that a document the load door admits and the edit doors
could not have produced is the defect, and a guard nothing can reach
is documentation whose repair is deletion
(`docs/prompts/implementer-discipline.md`).

1. **Slot dimensions** (`load-door-checks-slot-dimensions-for-profile-nodes-only`).
   One predicate over a node's slots — `Node::slot_dimension_fault(&self)`
   beside the other `*_fault` methods — walking `Node::slots()` for
   EVERY node kind, asked by `check_node_slots` and by the load door.
   The missing-expression case is a fault at BOTH doors (the edit door's
   `UnknownSlot` reading: a vocabulary bug surfaced, never skipped).
   The load door also asks `check_param_refs` (a slot expression naming
   an undeclared parameter, or one of another dimension, is a document
   the edit door refuses — same class). Vocabulary: measure whether
   `ProgramFault::SlotDimension` stays reachable once the node-level
   walk runs first; if it is shadowed, delete it and retire its tag
   (LIB's, mechanical); the load door's refusal for a non-profile slot
   is a `SnapshotError` arm naming node and slot, with its tag and F6
   row. The measured file (the retyped extrude distance) is the red
   row, and the green documentation row the round-two fix pass left
   (`load_door_slot_dimension.rs`) flips to red-then-green here as its
   header promises.
2. **The count/continuous shadow** (`count-continuous-arm-is-shadowed-by-the-display-unit-walk`).
   Delete `SnapshotError::CountContinuous` and its tag: unreachable
   through either persistence door because the display-unit walk
   refuses first, measured by the row that exists. The row that pins
   the divide keeps asserting what the load door actually answers. LIB's
   tag inventory follows mechanically; say so in the PR body.
3. **The doc-param float walk** (`doc-param-float-walk-is-hand-written-at-both-doors`).
   One predicate beside `DocParam` — `DocParam::first_non_finite() ->
   Option<DocParamField>` (the nominal, or which distribution offset) —
   asked by `SetDocParam`'s arm and by the load door's `param_site`.
   The edit door's refusal carries the field too
   (`EditError::NonFiniteDocParam { name, field }`): the move is toward
   the richer answer, as the row says; the F6 row and the tag follow.
4. **Rows**: per predicate, one fixture the edit door refuses and the
   load door refuses for the same fact, both refusals asserted by name;
   the mutant that breaks the shared predicate reds both; the arm-swap
   mutant (a door mis-mapping the shared answer) reds the row. Re-run
   the `validate_document` census on `three-door-predicates-are-hand-copied-not-shared`
   at your head and update it in place (the census is that row's
   `## Census`); the three rows close on it.

## Built (2026-09-16, `edit/one-predicate-round-three`)

One predicate, one home: `Node::slot_dimension_fault` walks
`Node::slots()` for EVERY node kind and answers `SlotDimensionFault`,
one comparison per slot through `SlotId::dimension_fault`. Both doors
name that answer — `check_node_slots` and `set_slot` as
`EditError::SlotDimensionMismatch`, and `validate_document`'s
`first_slot_fault` walk as
`SnapshotError::SlotDimension { node, slot, expected, found }`, with
its tag and its F6 row — and both RENDER it the same way: the fault's
own `Display` is one clause, forwarded into each door's subject, so a
profile step's address reads `loop 0 step 2 · centre x` at both
(`SlotId::label`, its one home).

**The missing-expression case is an INVARIANT, not a refusal** (the
fix pass, measuring what the spec assumed): `Node::slots()` is
`Node::expr()`'s domain, so a slot with no expression is a
disagreement between two matches in `node.rs` rather than a document a
door may refuse. `slot_dimension_fault` asserts against it at the
site; `SnapshotError::SlotExpressionMissing` and its tag are deleted,
and `check_node_slots` no longer maps one (`EditError::UnknownSlot`
stays for `set_slot`, where a caller CAN name a slot the node does not
carry). `switch_slots::every_node_kinds_slots_are_all_readable` pins
the invariant over every node kind and every payload shape that
answers a different slot list, welded by two `f6_variants!` rosters.

It found one violation, fixed here: a `Node::PlacedUnion` with no
count listed `SlotId::Count` among its slots while `Node::expr`
answered nothing for it. The count spelling and the rule disagreeing
is `PlacementRuleFault::CountSpelling`, refused at both doors through
`Node::placement_rule_fault` — so `Node::slots` now reports the slots
the node actually carries (`rule_slots`, the domain of the `rule_expr`
mapping it already shared with `Node::Pattern`), and the slot walks
pass over a node whose rule is broken rather than answering for an
address nothing can read.

The walk runs BEFORE the program walk, because the program walk probes
the replay and a step whose argument is the wrong quantity is not a
walk worth probing — and `validate_document` now runs its walks BY the
roster that names them (`Walk::ORDER`), so the order is data rather
than a sequence of `if let`s with a comment over them. Measured there:
`ProgramFault::SlotDimension` is
then unreachable, so it is deleted and its tag retired — a program
slot is a slot like any other, and the profile-only spelling was the
narrowing this row named.

The load door also asks the PARAM TABLE's half, as this row's spec
rules: `Doc::param_ref_fault` is a second predicate given one home — a
slot expression names a declared parameter and reads it at the
declared dimension — asked by `check_param_refs`, by the payload-expr
walk beside it (a third spelling, folded in) and by the load door's
`first_slot_param_ref_fault`, which answers
`SnapshotError::SlotUnknownDocParam` / `SlotDocParamDimension`. What
does NOT follow is a reference that fails to EVALUATE: that is V1
class 2 and still passes every door here.

The PAYLOAD expressions' half of the param-table rule (a measure's
expression, an assertion's bound) stays edit-door-only and is filed as
`load-door-does-not-check-payload-expression-param-refs`, MEASURED by
`rv_onepred3_probes::rv_a_measure_expression_reading_an_undeclared_parameter_still_loads`
— green because the load door admits what the edit door refuses, and
red on the day that row is built.

The measured file — a saved one-extrude document whose distance
literal is retyped from `Length`/`m` to `Angle`/`rad` on the wire — is
now refused at both doors, and `load_door_slot_dimension` is the
red-then-green row for it, with a frame datum's origin component
beside it as a second node kind and a pattern's Count slot as a third
(`rv_onepred3_probes::rv_a_retyped_pattern_count_is_refused_at_both_doors`,
the review lane's). `m4_pr6_refusal`'s program row reads the new
refusal.
## Closed (2026-09-16, EDIT orchestrator)

Built and merged as PR #2780 with its two sibling rows, after one opus
style review (APPROVE-WITH-FIXES: the MAJOR was the hand-written census
undercounting again — the walk this PR added — and the fix pass made
the census the code's: `persist::check::Walk` is iterated by
`validate_document` and an exhaustive match places every
`SnapshotError` arm on its walk, so a new arm or walk fails to compile
until placed). The load door walks every node kind's slots and asks the
parameter-table rule of slot expressions; the missing-expression case
is an invariant asserted at the site, with `slots() ⊆ expr()` pinned
over every node kind (which caught a placed union claiming a count slot
it did not carry — fixed in-fence); the unreachable `CountContinuous`
arm is gone; a document parameter's finiteness is one predicate naming
the field at both doors; `set_slot` asks the same one comparison. Two
disagreements argued with a measurement (the "does not ban" paragraph
had moved, not vanished; the roster is load-bearing rather than beside
the function). Residue in its own file:
`load-door-does-not-check-payload-expression-param-refs` (measured).
