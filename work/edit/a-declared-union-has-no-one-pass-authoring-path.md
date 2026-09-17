---
id: a-declared-union-has-no-one-pass-authoring-path
kind: issue
title: "A union with a declaration cannot be authored in one pass: the working path inserts a duplicate union and rebinds"
status: spec
opened: 2026-09-06
refs: [2028, 2028]
---


## What

DOCM-7 gives `Node::Union` a `declare` edge whose pairs name entities in
the UNION's own name space. A member-space name therefore carries the
union's own node id — and there is no order of edits that authors the
two nodes directly, for two doors that are each correct on their own:

- `crates/editor-core/src/edit.rs:1482` — `InsertNode` admits a payload
  name only when `name.node` is already live
  (`EditError::DeclareNamesMissingNode`; the ruled D3 carve-out). So the
  `Declare` cannot be inserted before the union it names.
- `crates/editor-core/src/node.rs:2429` and DM6 — no edit rewires a live
  node's inputs. So the union cannot be inserted first and given its
  `declare` edge afterwards.

The path that works, and the only one, is the fixture `declared_union`
in `crates/editor-core/tests/docm7_union_declare.rs`: insert a FIRST
union with no declaration, write the `Declare` in that union's space,
insert a SECOND union carrying the edge, `Rebind` every name from the
first union's space onto the second, then delete the first. The
intermediate document holds a duplicate union, and the rebind loop has
its own limit — a name that appears in more than one declared pair (a
chain of contacts declares the middle member's faces twice) must be
rebound ONCE, or the second `Rebind` refuses `RebindNoReferences`.

For an accumulation-entity name (a `Seam`/`Merged`/`Fragment` row of the
union itself) there is no other path even in principle: such a name has
no spelling that predates the union, since it is minted by the union's
own evaluation.

## Why it is chrome's

Nothing in `editor-core` is wrong. What is missing is a SEAT: the
operation "union these bodies and declare these contacts" is one
authoring act and reaches the document as five edits, three of which
exist only to work around the ordering. A seat that composes them (or an
edit that attaches a declaration to a live node, which is a DOCM
question about DM6) is the fix. `work/chrome/placed-union-has-no-session-op.md`
is the neighbouring gap for the same node.

## Where it stands

Open, unscheduled. `work/chrome/plan.md` has no union-seat unit; this
file is the placeholder for one. DOCM-7 ships with the two-pass shape
pinned as a measured fact rather than left implicit —
`a_declare_cannot_name_a_union_that_does_not_exist_yet` and
`a_declared_unions_document_loads_but_does_not_replay_in_order`
(`crates/editor-core/tests/docm7_union_declare.rs`) — the second of which
also records that such a document LOADS (the load door checks the mint
counter, not `order()`) while re-inserting its nodes in document order
refuses. This node is the first to rely on that asymmetry.

(At DOCM's exit sweep, `refs` names the PRs `DOCM-7` stood for: `DOCM-7` = #2028 — the unit rows left the tracker with `work/docm/`; `docs/DOC-LEDGER.md` sweep 14.)

## Re-homed to EDIT, 2026-09-15

Moved out of `work/chrome/` by the CHROME orchestrator. The row offers
two fixes — a composing seat in the viewer, or a new DM6-shaped edit —
and only the second can actually exist: **no `DocEdit` writes
`declare`.** Enumerating the vocabulary (`InsertNode`, `DeleteNode`,
`SetMembers`, `SetParam`, `SetStructuralParam`, `SetExpression`,
`SetDocParam`, `SetDocParamValue`, `Rebind`, `ReWitness`,
`ReWitnessBulk`, `SetAppearance`, `ClearAppearance`, `SetTolerance`,
`SetAppearanceMeta`) turns up nothing that attaches a declaration to a
live node, and `Node::declare_of` reads it on `Boolean` and `Union`
only. A viewer seat can only re-order edits that already exist, so the
door this needs is `crates/editor-core/src/edit.rs` — EDIT's ground
since DOCM's exit (`docs/DOC-LEDGER.md`, sweep 14).

**Citations repointed by subject**, both having rotted:
`EditError::DeclareNamesMissingNode` is still raised at the
`InsertNode` payload-name check in `crates/editor-core/src/edit.rs`
(the `:1482` band is gone); and the *"no edit rewires a live node's
inputs (DM6)"* sentence the row leans on is no longer at `node.rs:2429`
— that band is now `Node::Pattern | Node::PlacedUnion` slot lists — but
in the `declared_union` fixture's own doc comment in
`crates/editor-core/tests/docm7_union_declare.rs`. Both named pins
survive: `a_declare_cannot_name_a_union_that_does_not_exist_yet` and
`a_declared_unions_document_loads_but_does_not_replay_in_order`.

The viewer-side consequence of this gap stays on CHROME's slate as
`work/chrome/addboolean-doc-names-a-vocabulary-that-does-not-exist`.

Signed: (CHROME orchestrator)

## Question for Ev (2026-09-17, EDIT orchestrator) — on the fourth `[ev]` PR

DM6 says no edit rewires a live node's inputs. A union's `declare`
edge names entities in the union's OWN name space, so it cannot be
authored before the union exists and cannot be attached after — the
only path is the five-edit workaround this row records. The question
is whether DM6 admits the one edge that by construction cannot precede
the node it names (a narrow `SetDeclare` on a live `Union`/`Boolean`),
whether the declaration should instead become the union's own payload,
or whether the workaround stays and CHROME seats it. The
recommendation and the alternatives are on the PR; this row is parked
on Ev's answer.

## Revised on the `[ev]` PR (2026-09-17)

Ev asked whether "declare names pre-boolean entities" is possible. It
is how the pair `Boolean` already works (a bare name resolved through
the operands' tables, one pass — `kiss_carry`); it fails for the n-ary
`Union` only because DM4's members can carry identical tables (21
transforms of one ball), so a bare pre-union name cannot say which
member — which is why DOCM-7 made the declaration name the union's own
`FromMember` rows. A SITED declaration (`SitedRef { at: member, name }`
per side — the shape a mate head already has) says which member
without naming the union: one pass, no DM6 exception, and
`DeclareBothOperands` unnecessary by construction. Revised
recommendation: (B) sited declarations, a DOCM-7 shape change (the
persisted `Declare` form, two suites, five corpus documents, the
Python constructor); the narrow `SetDeclare` stays the cheap
alternative. Waiting on Ev.

## RULED (2026-09-17, Ev on `[ev]` PR #2795): sited declarations

A `Declare`'s pairs name SITED entities — `SitedRef { at, name }`,
`at` the member (a pair boolean's operand), `name` the entity in that
member's table — so a declaration names what exists before the union
and is authored in one pass; the union derives each pair's fold step
from its two sites; the site is the side, so `DeclareBothOperands`
retires; the union's published `FromMember` names are unchanged. DM6
is untouched. The DM4 clause is amended on this PR; the DM7 sentence
that cited the old cycle is re-worded with it. Kernel unit (the
resolver in `eval/wire.rs`, the persisted `Declare` form, the two
DOCM-7/8 suites and the five declaring corpus documents, the Python
declare constructors — LIB's, mechanical): v6 dual, block EDIT-B2
slot 0, spec `docs/EDIT-DECL-SPEC.md` at the next claim.


## Spec'd (2026-09-17, EDIT orchestrator) — kernel unit, v6 dual, block EDIT-B2 slot 0

`docs/EDIT-DECL-SPEC.md`; branch `edit/sited-declarations`. Eight
premises: the sited payload (`SitedRef` reused), the site is the side
(`DeclareBothOperands` retires, one new site arm), the union routes by
site and rewrites into member space before the shared resolver
(look-through unchanged), fold-minted rows stop being declaration
subjects by type (the DOCM-7 fold-row rows retire with the class,
each replaced by a cannot-be-written row), the doors follow the
payload (`payload_names` the names, `payload_read_sites` the sites),
the persisted form moves and every declaring document in the tree is
re-authored (no migration — the format is unversioned and nothing
outside the tree declares), the façades follow mechanically (LIB's,
announced), DM6 untouched. Pre-draw fields at the block record
(`edit/b2-block`): difficulty **M**, task-class **STRUCTURAL**.
