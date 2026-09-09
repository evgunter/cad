---
id: blend-selection-canonical-check-load-only
kind: issue
title: A blend's non-canonical selection is refused at load only: the insert door accepts a hand-built Fillet/Chamfer variant the load door then refuses
status: open
opened: 2026-09-08
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
