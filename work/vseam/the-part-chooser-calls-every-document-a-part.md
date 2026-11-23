---
id: the-part-chooser-calls-every-document-a-part
kind: issue
title: The Add part chooser calls every document a part, and shows neither which entries are assemblies nor how deep a pick nests
status: open
opened: 2026-11-22
priority: P3
cost: E
---

Split out of `document-seam-no-in-session-change-detection` §3 (2026-11-22, VSEAM orchestrator). The designers weighing that row agreed that this part is a labelling question and has nothing to do with store semantics.

`Add part…` (`crates/viewer/src/parts.rs`, the chooser in `pane/create.rs`) lists every document in the directory. Picking an assembly nests it, which works as intended (A1's scope ladder). But the chooser calls every entry a "part" and does not show:
- which entries are assemblies;
- how deep a pick would nest;
- that a pick pulls in a whole tree.

Both facts can be derived from the documents themselves: a document is an assembly if it holds an `InstantiatePart` node, and the depth is the longest chain of such references. So the chooser needs a derived label, not anything stored.
