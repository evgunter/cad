---
id: editor-core-cites-a-spec-d3-no-document-numbers
kind: issue
title: editor-core's eval cites "spec D3" and bare "(D3)" for node semantics, a numbering no document in the tree still uses
status: open
opened: 2026-11-06
priority: P4
cost: C
---



Found by the review of PR 6557 (2026-11-06). `rg 'spec D3|\(D3\)|; D3\)' crates/editor-core/src` finds about 40 sites, most of them under `eval/`, that cite "spec D3" or a bare "(D3)" for a node's semantics, for example "spec D3 fixes each node's semantics" and "wire, don't invent". No document in the tree numbers a D3 that way any more. DESIGN.md's old-numbering table maps its own D3 to `[surfaces]`, which is a different subject.

PR 6557 re-cited the two sites it touched, `NodeErrorKind::EmptyOperand` and `wire_combine`'s "nothing ∅-absorbing", to `[topology.conventions]` (∅ is a typed result) and `[loud-failure]` (nothing absorbs a failure).

The rest is a citation chore. For each site, find the current clause (`[operations]`, `[recipe]`, `[topology.conventions]` and so on), or drop the citation where the sentence stands on its own.
