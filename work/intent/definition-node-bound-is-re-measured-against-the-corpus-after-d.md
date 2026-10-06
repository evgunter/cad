---
id: definition-node-bound-is-re-measured-against-the-corpus-after-d
kind: issue
title: DEFINITION_NODE_BOUND is re-measured against the corpus once every slot formula is a definition
status: open
opened: 2026-10-05
---


`DEFINITION_NODE_BOUND` (`crates/editor-core/src/edit.rs`) is 4096,
spec §1's number. `docs/INTENT-LITERALS-SPEC.md` §9 asks for it to be
picked as "the corpus maximum ×16". PR A (#4065) could not take that
measurement: no committed document holds a definition, and the largest
slot expression the committed corpus holds is one node.

After INTENT-LITERALS C and D every written slot formula is a variable
with a definition, so the corpus will hold definitions and the
measurement means something. Once D lands:

- measure the largest expansion (`Doc::expansion_fault`'s sizes) over
  every committed document — the corpus files, the tour, the goldens;
- set the bound to that maximum ×16, or say in the PR why 4096 stands;
- a document near the bound refuses `DefinitionTooLarge` at the edit
  and load doors, so a lower bound is a re-baseline of any file it
  would refuse.
