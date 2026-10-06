---
id: a-verdicts-other-nodes-are-said-by-tag-once-deleted
kind: issue
title: A selection verdict's diagnosis names nodes the selection does not keep, and says them by tag once deleted
status: parked
opened: 2026-10-06
priority: P3
parent: node-labels-are-document-data
blocked_on: [no-docedit-splices-a-deleted-node]
---


Found in review of `a-selected-node-deleted-is-said-by-tag-where-the-tools-say-its-label` (PR 4086).

**The case.** The selection keeps only the nodes `Selection::nodes`
names (`crates/viewer/src/session/select.rs:264`): the node itself, or
for a pick its minter, its feature and the hit body. `standing_verdict`
(`crates/viewer/src/pane/properties.rs`) says a `ResolveError` by
`Speaker::of(landed).or_held(said)`, and a `Vanished` verdict's
`Diagnosis` (`crates/editor-core/src/resolve/mod.rs:292`) can name other
nodes:
- `Diagnosis::StructuralParam`'s `node` (`resolve/mod.rs:340`);
- `Diagnosis::Upstream`'s `node` (`resolve/mod.rs:367`);
- `Diagnosis::Cascade`'s `through` (`resolve/mod.rs:353`);
- `Diagnosis::BorderDelta`'s `gone` and `new` walls (`resolve/mod.rs:307`).

If the landed document no longer holds one of these, the selection kept
nothing for it, so it is said `node <tag>`. Its minter in the same
sentence is said by its label, so one sentence mixes the two.

**The fix.** Keep the nodes the verdict names as well as the
selection's. Either respeak the verdict's own nodes with `held_by` when
the standing is computed, or widen what the session keeps to the nodes
the diagnosis names.

**Reachability.** No session operation reaches this case today. A
`Vanished` verdict names a node the landed document lacks only if that
node fed the still-live minter in the last-good document and has since
been removed. Every node a `Diagnosis` names is either in
`derivation_nodes(name)` (`crates/editor-core/src/resolve/mod.rs:2329`)
or a strict ancestor of the minter in either run (the scope rule at
`upstream_nodes`, `resolve/mod.rs:2356`); the border delta's and the
group's `new` names come from the current run, which the landed
document holds. Neither set can lose a node while the minter lives:
- `DocSession::delete_node` (`crates/viewer/src/session.rs:3033`)
  deletes the whole `cascade_delete_order` cone
  (`crates/editor-core/src/edit.rs:4834`), so deleting an ancestor
  deletes the minter, and the verdict is then `NodeGone`, which names
  the minter the selection already keeps.
- No session operation rewires a live node's inputs. `SetMembers`,
  `Rebind`, `Promote` and `Fold` appear in the session only as match
  arms (`session.rs:2942`, `session.rs:2949`, `session.rs:2956`), never
  built; `EditProfile` builds `DocEdit::SetProgram` over loops alone
  (`session.rs:2574`), not the plane; a mate's, an instance's and a
  gauge's references are names, not `Node::inputs` edges
  (`crates/editor-core/src/node.rs`, `Node::inputs`).
- Undo and redo move between versions in which every newer node is
  downstream of the older ones, and `Open` and `NewDocument` drop the
  selection with the landed run.

It becomes reachable when an edit that rewires a live node's inputs
reaches the session: splice (`no-docedit-splices-a-deleted-node`), or a
`SetMembers` door.

**When it becomes reachable.** Respeaking the verdict with `held_by`
(`crates/editor-core/src/spoken.rs:532`) when the standing is computed
cannot fix it: the standing is spoken from the landed run, which
already lacks the node. Keeping only the nodes the current verdict names
cannot either, since the node is deleted before any verdict names it.
What can is keeping the nodes a verdict on the selection *can* name:
- In `Derived::said` (`session.rs:471`), keep `derivation_nodes(name)`
  plus the strict ancestors of the minter (`roots::strict_ancestors`,
  `crates/editor-core/src/roots.rs:150`), each read off the shown and
  the landed document, beside `Selection::nodes`
  (`crates/viewer/src/session/select.rs:264`), when the selection is
  made (`DocSession::spoken_now`, `session.rs:955`).
- After each operation (`DocSession::perform`, `session.rs:1443`),
  merge that set from the new shown document into
  `said.respoken(doc)`, the respoken copy winning a tie, so a deleted
  node keeps the last label it had.
- That needs a public `verdict_nodes(doc, name)` beside the scope rule
  in `resolve/mod.rs` (the ancestor walk is `pub(crate)`), re-exported
  through `pncad::select`, and a join of two `HeldNodes`.

