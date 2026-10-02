---
id: a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added
kind: issue
title: A boolean that becomes flush after it is committed has no recourse but cascade delete and re-add
status: parked
opened: 2026-09-30
priority: P2
cost: M
design: true
refs: [no-docedit-splices-a-deleted-node]
blocked_on: [declared-pairs-are-a-booleans-own-payload]
---


Raised by AUTH-9's correctness review (PR #3543).

## What

AUTH-9 gives a flush union a recourse at the one moment the boolean
door sees it: before the union is committed. The door refuses, offers
the declaration, and commits the `Declare` and the union together. A
union that becomes flush *after* it is committed gets none of that:

- **an edit makes it flush**, e.g. a `SetSlot` or parameter edit that
  moves the boss down onto the block, or changes a height so two caps
  meet; or
- **a poisoning ancestor is repaired**. A boolean poisoned by an
  upstream failure commits (`RefusedBoolean::read` answers `None` for
  it), and when the upstream node is fixed, the boolean's own contact
  refusal appears on its row.

Either way the committed union's row shows `UndeclaredContact`, whose
recourse is to declare the pair on its `declare` input. No edit
attaches a declaration to a live node, so the author's only path is
to cascade-delete the union and everything downstream of it and author
it again through the tool.

## Why it is a design question

This is DM6's own reopening trigger. `crates/editor-core/REFERENCES.md`,
"DM6 — Splice is not added", keeps
`work/recipe/no-docedit-splices-a-deleted-node` open as the record of
"the one trigger that would reopen the question: a chain that a flat
operator cannot flatten and that a user needs to edit from the middle".
A union in the middle of a chain that an upstream edit makes flush is
that shape. The candidate answers are the ones EDIT weighed on the
`[ev]` PR behind `a-declared-union-has-no-one-pass-authoring-path`:
a narrow edit that attaches a declaration to a live `Boolean`/`Union`,
or a viewer seat that replaces the node through a splice. Both reopen
DM6, so this goes to EDIT and Ev with a designer weighing before
anything is built. `needs_ev` is not set yet.

## Declaring a contact on a live boolean (2026-09-30)

Two designers weighed this together with `the-boolean-door-evaluates-its-boolean-twice`. Their evidence is in AUTHOR's log (`git show 29b8874a1:work/author/log.md`) (2026-09-30, "boolean-judge fork").

**Most booleans are mid-chain, not leaves.** A boolean stops being a leaf as soon as anything takes it as an input, and a sketch frame on one of its faces is such an input (`Datum::FaceFrame { at }`). So after the boss-on-face gesture the union is already mid-chain. An upstream edit to any finished model is this row's case.

**The kernel's own recourse prescribes an edit that does not exist.** `UndeclaredContactFinding::recourse` says "wire it into the Boolean's declare input". No edit can do that on a committed boolean.

**The answer both designers reach:**
- Commit every boolean. The seam evaluates it once.
- A contact refusal shows on the boolean's own row with a **Declare** control.
- That control **sets the declaration on the live boolean**.

**Why this is not what DM6 rules out.** A declaration is a parameter, not an operand:
- It carries no material and mints no names. `Node::Declare` evaluates with an empty name table, and a boolean names only its operands.
- Its sites must be the consumer's operands (`DeclareSiteNotAnOperand`).
- So changing it changes no name anywhere. DM6's reasons (which input survives; names composed over a removed node) do not reach it.

**Two shapes of the edit:**
- **A1: keep the `Declare` node** and add one edit that points a live boolean's or union's `declare` edge at a `Declare`, or clears it. DM6 then reads "no edit rewires a live node's *operands*", with this edit beside `SetMembers` as its second named exception.
- **A2: the declared pairs become the boolean's or union's own payload,** set by a whole-list edit shaped like `SetMembers`. The separate node, its orphan report, its re-rooting and its kind check become unrepresentable. DM6 needs no exception, because there is no edge. This changes DM4's node shape and the persisted form. What it gives up is one `Declare` shared by two consumers: legal today, but used nowhere in the tree.

Both designers lean A2 and are unsure. A1 is the reversible minimum.

**Undo.** The boolean is one step and the declaration another. Undo walks the declaration off first, then the boolean.

**If neither shape is taken,** the fallback is to *replace* a boolean nothing refers to: delete it and its `Declare`, then insert `Declare` + boolean as one action. It needs no ruling, but it dead-ends every mid-chain boolean, and the new id strands appearance keys and display state on the old one.

## Ruled 2026-10-01 (Ev, #3587): attach, shape A2

**A declaration can be set on a live boolean or union. The declared pairs become the node's own payload (A2).** Ev conditioned this on the `Declare` node having no other use. It has none: its value is read only through the `Boolean`/`Union` declare edge (`eval/wire.rs` `declared_pairs`). Everything else is bookkeeping, machinery that exists because it is a separate node, or constructors. The one scope cost Ev was told of: Python's declare API (`Node.declare`, `Doc.declare`/`declare_all`, `Node.boolean(..., declare=)`) changes shape, as do the persisted form and the schema.

**Where the work is:**
- **EDIT `declared-pairs-are-a-booleans-own-payload` (P1).** The node shape (DM4's `declare` edge becomes a payload) and a whole-list edit shaped like `SetMembers`, with nothing inferred. `Declare`, its orphan report, its kind check and its re-rooting go. Also persistence, the schema bump, `pncad-py`, and the kernel recourse sentence ("wire it into the Boolean's declare input") rewritten to name the new edit. With no edge, DM6 needs no exception.
- **AUTHOR, both rows: the viewer half, blocked on it.**
  - `add_boolean` becomes a plain commit (no judge, no `evaluate_beside`, no `RefusedBoolean` generation rule).
  - A contact refusal shows on the boolean's own row with a Declare control that records the edit.
  - This row and `the-boolean-door-evaluates-its-boolean-twice` close together.

## Evidence: a count edit makes the union flush (SHOW, 2026-10-02)

`demos/tour/src/heatsink.rs` (`flush_fins`, narrated live on every
tour run and in the tour's test suite). Five fins flush on a rounded
plate, a `PlacedUnion` group, the five feet declared through
`find_flush_candidates` + `declare_node`, one `Boolean(Union)`: builds
at the closed-form volume. `SetStructuralParam` 5 → 7 makes the union
refuse `UndeclaredContact` on `Instance(5)`. This row's fallback then
works as written: `DeleteNode` the union, `DeleteNode` its `Declare`,
re-detect (seven pairs), insert a new `Declare` and a new union. The
re-added union builds at the closed-form volume of 7 fins, recomputing
2 nodes and reusing 8. Cost: four edits per count step, and a union
under a new id. That scene's subject is one edit recomputing only what
is downstream of it, so it keeps its fins sunk 1/16 into the plate
instead and waits on `declared-pairs-are-a-booleans-own-payload`.

PR 3902 deleted `Node::Declare` and renamed `declare_node` to `declared_pairs`: the pairs are a boolean's or union's `declare` payload, set on a live node by `DocEdit::SetDeclare`.
