---
id: blend-selection-canonical-check-load-only
kind: unit
title: A blend's non-canonical selection is refused at load only: the insert door accepts a hand-built Fillet/Chamfer variant the load door then refuses
status: closed
opened: 2026-09-08
branch: edit/blend-canonical
pr: 2724
closed: 2026-09-16
---

`Node::Fillet` and `Node::Chamfer` are public variants; `Node::fillet`
/ `Node::chamfer` are the construction doors that canonicalize (sort,
dedup) the selection. A hand-built variant with a non-canonical
selection inserted through `DocEdit::InsertNode` is ACCEPTED by the
insert door (`crates/editor-core/src/edit.rs:1518-1540` checks the
names' nodes are live and asks `Node::input_fault`, which says nothing
about a selection's order) and evaluates; the document it produces is
then refused by `save`/`load` (`persist/check.rs`,
`SnapshotError::BlendSelectionNotCanonical`). One refusal, two doors,
only one of them asking — the asymmetry the shell's ordered
designation does NOT have since LIB-G17's fix pass moved its repeat
check onto `Node::input_fault` (`InputFault::RepeatedDesignation`),
which both doors and the evaluation backstop ask.

The symmetric fix is the same move: a canonical-form fault on
`input_fault` (or a sibling `payload_fault`) asked by both doors, and
`BlendSelectionNotCanonical` retired for `SnapshotError::InputList`.
Not this unit's (the blends are G16's vocabulary and the load-only
check predates it); recorded from R2's review of PR 2150.

## Re-homed (2026-09-08, LIB orchestrator)

Moved from `work/lib/` to `work/docm/`: the check moves from the load validator onto `Node::input_fault` (`crates/editor-core/src/{edit,node}.rs`, `persist/check.rs`), DOCM's territory. Id, body and header
are unchanged; the directory is the claim (`work/README.md`). LIB's
half — the Python/façade rows that move when this closes — is named in
the body and stays LIB's to execute once the kernel side lands.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/edit/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): EDIT is DOCM's successor on the document-model ground (persist, the edit vocabulary, the node and resolver doors). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/blend-canonical`. The shape is the row's own, verified
against the tree: `crates/editor-core/src/edit.rs` (`InsertNode`) and
`crates/editor-core/src/persist/check.rs` both already ask
`Node::input_fault` (`edit.rs` at the `match node.input_fault()` site,
`check.rs` at its `InputList` arm), and `check.rs` separately hand-tests
`selection.windows(2).any(|w| w[0] >= w[1])` for `BlendSelectionNotCanonical`.

1. `Node::input_fault` (or a sibling `payload_fault` if you judge the
   selection is a payload and not an input — say why) answers a typed
   fault for a `Fillet`/`Chamfer` selection that is not sorted and
   deduplicated, naming the position at which order breaks (the
   `RepeatedDesignation { first, again }` precedent: positions, not
   the names). The predicate has ONE home; `check.rs`'s `windows(2)`
   line goes.
2. `SnapshotError::BlendSelectionNotCanonical` is retired; the load
   door refuses the same document through `SnapshotError::InputList`.
   Grep-prove the variant absent, including `crates/pncad-py` (the
   tag rows are LIB's — a mechanical follow-through, said in the PR).
3. Find whether the evaluation backstop asks `input_fault` too (the
   row says it does for `RepeatedDesignation`; measure).
4. Rows that go red: a hand-built `Node::Fillet` with an unsorted
   selection is refused by `InsertNode` (RED on main today — write it
   first); the same node through save/load refuses `InputList` with
   the same fault; a duplicate is refused at both; a mutant that drops
   the predicate at either door is caught by that door's row. The
   construction doors `Node::fillet`/`chamfer` still canonicalize and
   a row says so.
5. The `M6-5` history in `check.rs`'s comment does not move with the
   predicate; the new doc states the invariant (one canonical form,
   asked by every door that admits a node).

## Built (2026-09-16)

The predicate has one home. `InputFault::SelectionNotCanonical { at }`
answers for a `Fillet`/`Chamfer` selection whose entries do not
strictly increase — "sorted and deduplicated" is one predicate, so a
swap and a repeat are one fault at one position — and every caller of
`input_fault` asks it. The doors it is REACHABLE at are two, not
three: `SetMembers` is a caller but refuses `SetMembersOnNonList`
first, since no designation-carrying kind has a list input. `check.rs`'s `windows(2)` line and
`SnapshotError::BlendSelectionNotCanonical` are gone; the load door
refuses the same document through `SnapshotError::InputList`, the arm
that already carried every other `input_fault` answer. The edit doors
name it `EditError::SelectionNotCanonical { node, at }` and forward the
fault's own sentence, as `RepeatedDesignation` does.

On the spec, point by point:

1. `input_fault`, not a sibling `payload_fault`. `RepeatedDesignation`
   — a shell's `open` list — is already a name PAYLOAD living there,
   and a blend's selection is the same kind of thing: a structural form
   a construction door establishes, that only a hand-built variant or a
   corrupt file can arrive without. A second function would split the
   two designation rules across two homes and give the load door two
   arms where it has one. The enum's own header said "INPUT LIST",
   which `RepeatedDesignation` had already made stale; it now states
   the actual scope.
2. Done. Grep-proven absent, including `crates/pncad-py`.
3. **The spec's premise is wrong, and the code said so too.** No
   evaluation site asks `input_fault` — its callers are exactly
   `edit.rs`'s `check_node_inputs` (from `InsertNode` and `SetMembers`)
   and `check.rs`'s `validate_document`. What evaluation has is its own
   independent refusal per payload: `topo::ShellError::OpenFaceRepeated`
   for a repeat, and for a blend `resolve_selection` re-SORTS the
   resolved keys and leaves a duplicate to the kernel. So an unsorted
   selection was not merely accepted at the insert door — it evaluated
   silently, and only `save`/`load` ever objected. `input_fault`'s own
   comment claimed "the insert door, the load door and the evaluation
   backstop refuse alike"; it now says the two doors, which is what is
   true.
4. Rows in `crates/editor-core/tests/edit_blend_canonical.rs`, one per
   door so a mutant that drops the predicate at either is caught by
   that door's row rather than by its twin: unsorted at the insert door
   (fillet and chamfer), a repeat at the insert door, unsorted at the
   load door, a repeat at the load door, the construction doors still
   canonicalize, an empty selection is canonical (evaluation's refusal,
   not this door's), and the two doors forward one sentence.
   `m6_5_selection_wire`'s load-door arm moved to the new refusal.
5. The `M6-5` history did not move: the new prose states the invariant
   — one canonical form per payload, asked by every door that admits a
   node, repaired at none.

Filed: `three-door-predicates-are-hand-copied-not-shared` (this
program's slate) — the sweep found the assertion bound, a mate's
alignment and the placement registry spelled twice, once per door, with
the placement pair not even refusing the same set.

## Fix pass (2026-09-16, after the style review)

Verdict was MERGEABLE, no MAJOR; twelve findings, all taken except the
two the orchestrator ruled pre-existing.

- **The establisher now has one home too.** `sort(); dedup();` was
  spelled three times — `Node::fillet`, `Node::chamfer`,
  `rebind_payload_names`. `canonicalize_selection` is the named door,
  the `dedup_keeping_first` precedent for the ordered twin, and
  `Rebind`'s repair therefore re-establishes exactly the form the
  predicate checks. `a_rebind_leaves_a_canonical_selection` pins it.
- **`at` is pinned away from zero.** The reviewer's probe rows
  (adopted through a merge of `review/blend-rv`, folded into
  `edit_blend_canonical.rs`) drive both faults at positions 0 and 1,
  both at once in either order, and both doors reporting `at == 1`.
  The mutant `at: 0` now reds four rows; it survived the whole suite
  before.
- **Both door rows go through a door.** `both_doors_forward_one_sentence`
  built its fault by hand and so could not catch a door forwarding the
  wrong index; it now drives `apply` and `load` over a real refused
  node and reads the rendered sentence.
- **The empty selection evaluates.** The row named
  `BlendSelectionEmpty` without ever asking for it; it now evaluates
  and takes that refusal.
- **Prose corrections.** "every door" → "every door that ADMITS a
  node" (`Rebind` repairs, and that is not an exception); the
  hand-synced arm count in `tags.rs` is gone; the `at + 1` rendering
  and the one-index payload are reconciled in the field's own doc;
  `pncad.pyi` and `edit_payload.rs` now say the VARIANT decides what
  `first` means.

Filed in this pass:
`load-shaped-doors-outside-check-rs-may-duplicate-edit-predicates` —
the sweep's blind spot as a row rather than a sentence in a PR body.
`three-door-predicates-are-hand-copied-not-shared` re-points its
assertion-bound citation at the inline block in `check_node_slots`
(there is no `check_assertion_bound`).

Not taken, ruled pre-existing classes: the `as u32` narrowing in
`input_fault`, and the phantom `RecipeNodeId(next_id)` that reaches
Python as `EditPayload.node` on an `InsertNode` refusal.

## Closed (2026-09-16, EDIT orchestrator)

Merged as PR #2724 after one opus style review with a correctness arm
(MERGEABLE, two MINORs, every finding built at the fix pass) and the
orchestrator's read. Residue in its own files:
`three-door-predicates-are-hand-copied-not-shared`,
`load-shaped-doors-outside-check-rs-may-duplicate-edit-predicates`.
