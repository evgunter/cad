---
id: the-clearance-payload-spells-a-node-standing-in-its-own-words
kind: issue
title: The clearance goldening form spells a node standing in words no other channel uses
status: open
opened: 2026-10-02
priority: P4
cost: E
---

Found by PR 3782's review (`memoized-refusals-speak-inner-nodes-through-the-frame`).

`SelectionRefusal::payload` (`crates/editor-core/src/clearance.rs`) is the clearance goldening form's machine spelling of a selection refusal. It writes a `NodeStanding` as `not_evaluated`, `not_in_document`, `failed` or `poisoned`. `crates/pncad-py/src/tags.rs` already spells the same standing four other ways, door by door: `node_standing_tag` uses `node_not_evaluated`/`node_failed`/`node_poisoned`, `eval_reason_tag` uses `unknown_node`/`poisoned`, and the export, product and checks doors keep their own. None of them is a kernel word that both crates could read. The unplaced cause got a kernel word in PR 3782 (`Unplaced::word`). `unplaced_tag` could not read it, because the tag inventory (`the_whole_tag_table_matches_its_committed_inventory`) reads `tags.rs` and cannot follow a call into editor-core, so a test holds the two equal instead. The standing words could not, because the bindings' vocabularies merge arms the goldening form keeps apart.

The question is whether `NodeStanding` gets one kernel word per arm (`NodeStanding::word`), which the goldening form prints and each binding vocabulary maps from, or whether the goldening form adopts one of the binding spellings.
