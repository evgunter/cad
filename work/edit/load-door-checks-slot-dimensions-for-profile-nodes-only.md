---
id: load-door-checks-slot-dimensions-for-profile-nodes-only
kind: issue
title: The load door re-spells the slot-dimension predicate and asks it of profile nodes only
status: open
opened: 2026-09-16
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
