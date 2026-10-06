---
id: the-tree-calls-a-boolean-boolean-where-a-join-says-subtract
kind: issue
title: The tree calls a Boolean node "Boolean <tag>" where a name's join on the same screen says "Subtract <tag>"
status: open
opened: 2026-10-06
---


Found by the fourth review of PR 3886 (RECIPE, `names-render-a-faces-leaf-role-in-words`), Q7.

A name carried through a Boolean's B says its join by the operation: `names::words::expand` spells the node with `Speaker::node_as_kind(at, VerbKind::Boolean(op).noun())`, so a face reads "…, cut in at Subtract 1669…". Every other sentence, the tree's rows among them, spells the same node through `Doc::spoken`, whose kind noun is `spoken::node_kind_noun` (`crates/editor-core/src/spoken.rs`, `Node::Boolean { .. } => "Boolean"`). One node then has two names on one screen: "Boolean 1669" in the tree and in a strand row, "Subtract 1669" in a face name beside it.

The fix is one spelling. The likeliest is for `node_kind_noun` to say a Boolean by its operation (`VerbKind::Boolean(op).noun()`), which moves every sentence and tree label that says "Boolean <tag>" (goldens across `editor-core`, `viewer` and `pncad-py` tests); the other is for the join to say "Boolean". Which noun the tree shows is this program's call, so it is filed here rather than changed by RECIPE.
