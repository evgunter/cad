---
id: mate-head-entity-kind-is-decided-only-at-assembly
kind: issue
title: A mate head's EntityKind is decided at assembly and never at the edit door
status: open
opened: 2026-09-16
refs: [three-door-predicates-are-hand-copied-not-shared]
---


Raised by the style review of `edit/one-predicate-round-two` (PR 2772)
while reading that unit's shared-predicate moves. **It is not a
duplication**, which is why it is its own row rather than a hit on the
sweep: nothing spells this rule twice, because only one door spells it
at all.

**The finding.** A `Node::Mate`'s two heads are `SitedRef`s whose
`StableName` carries an `EntityKind`. A mate is a FACE-to-FACE contact:
`crates/editor-core/src/assembly.rs`'s `resolve_face` refuses
`RefusedRef::NotAFace { found }` for a head whose name is not a face,
and says at the site that kind precedes multiplicity.

That is the only place the kind is decided. The edit door that ADMITS
the mate — `InsertNode`, through `check_node_inputs` — checks the
heads' referenced nodes and the alignment's finiteness
(`Node::has_non_finite_alignment`, now shared), and says nothing about
what the heads denote. `EntityKind::Face` appears in `edit.rs` only for
the appearance and metadata doors. So a mate whose `b` head names an
EDGE inserts cleanly, saves cleanly, loads cleanly, and refuses at
evaluation.

**Which class this is.** It is the V1 class-2 shape — a document may
hold a node that refuses to evaluate — and that reading is defensible:
a head's name-level resolution needs a product, which the edit door
does not have (the ruled `Declare` carve-out applies the same way to
the second name-referencing edit). But the KIND is not name-level
resolution: `name.kind` is data on the `StableName`, readable with no
product at all, and `assembly.rs` reads it before it consults any
table.

**The question the row asks**: should the edit door refuse a mate head
whose `EntityKind` is not `Face`, in the vocabulary
`AppearanceWrongKind` already uses for the same shape of mistake? If
yes, the predicate has one home (beside `Node::Mate`, or on the
`SitedRef`) and `assembly.rs` names its answer — the round-two unit's
move, applied to a rule that currently has one door instead of two. If
no, both sites say why the kind waits for evaluation although it needs
no product.

Not built by the round-two unit: it is an addition to what the edit
door refuses, not a relocation of a predicate, so it changes which
documents exist.
