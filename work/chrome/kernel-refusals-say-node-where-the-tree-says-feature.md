---
id: kernel-refusals-say-node-where-the-tree-says-feature
kind: issue
title: chrome: kernel refusals the tree draws verbatim say 'node N' where the tree's own words say 'feature N'
status: parked
blocked_on: [refusal-values-speak-the-node-with-its-label]
opened: 2026-09-29
priority: P3
cost: E
---

## Question (answered by Ev, 2026-10-01)

What word names a node in text a viewer user reads? The feature tree says "feature N" (`tree::node_number`), while the kernel's refusals, which the tree draws verbatim, say "node N". So one row can read "node 7 failed: … repair node 4" above "see feature 4". The choice is the word, and which layer owns it.


## What

The feature tree names a node `feature 3` (`crates/viewer/src/tree.rs`,
`node_number`, whose doc rules out `node 3` in a surface sentence), and
draws every failed row's refusal verbatim from `NodeError`'s `Display`
(`crates/editor-core/src/eval/mod.rs`, `failed_line`), which opens
`node 3 failed:` and, in the carrying sentences, points at `node N`
("Recourse: open the part and repair node 7",
`crates/editor-core/src/eval/parts.rs`, `PartFault`'s `Display`; "repair
node 4", `crates/editor-core/src/mate.rs`, `MateFault::PlacerRefused`'s).

So one row can read "node 7 failed: … repair node 4" above the tree's
own link "see feature 4": two words for one thing, one of them the word
the chrome's own rule forbids. The kernel cannot say "feature" (it is
the viewer's word), and the viewer draws the kernel's text unaltered by
rule (`tree.rs`'s module header, "Failures are values").

Found while building `edit/part-root-carried-refusal`, whose carried
lines add more such sentences to the tree.

## What would close it

One word for a node across the kernel's refusals and the chrome, or a
rendering door the viewer can hand its own noun to. It is a vocabulary
decision, so it is likely a question for Ev before it is a unit.

## Ev's answer (2026-10-01, on PR 3605): superseded by PR 3565

Ev asked whether PR 3565 bears on this, and it does. PR 3565 (`work/emit/how-a-person-sees-a-node`, ratified in DESIGN.md Band 1 "Node labels") rules that every surface speaks a node as kind + label + tag, e.g. `Extrude \"base plate\" (3fa9c1d2a0b1)`, or `Extrude 3fa9c1d2a0b1` when it has no label. On the tree:
- a labelled row's headline is its label;
- an unlabelled row shows its kind and tag.

So the "node N" against "feature N" choice no longer exists, and the identifier a sentence names is visible on its row. Ev agreed: *"sounds good!"*

**What is left for CHROME:**
- Retire `tree::node_number` ("feature N") and its ~20 callers.
- Route them through the kernel's spoken-node form.

That form arrives with `work/emit/node-labels-are-document-data`, so this row is parked on it.
