---
id: kernel-refusals-say-node-where-the-tree-says-feature
kind: issue
title: chrome: kernel refusals the tree draws verbatim say 'node N' where the tree's own words say 'feature N'
status: open
opened: 2026-09-29
priority: P3
cost: D
---


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
