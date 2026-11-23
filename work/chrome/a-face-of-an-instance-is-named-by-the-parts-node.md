---
id: a-face-of-an-instance-is-named-by-the-parts-node
kind: issue
title: a face picked on an instance is named by the part file's node, which the assembly does not hold
status: open
opened: 2026-10-31
priority: P2
cost: M
---

## Finding

Found driving the viewer (2026-10-31; Ev: P2). After a mate between two instances of `plate.pncad`, the selection's Properties header reads `face of node a59299af4a92` beside a disabled `Delete node a59299af4a92`. `a59299af4a92` is the Extrude inside `plate.pncad`: the face was made by the part's feature, the assembly does not hold that node, so the header can only call it "node" and Delete cannot act. A face of an instance should be named by the instance (`face of InstantiatePart "InstantiatePart 2" — plate.pncad`), and Properties should offer what the instance offers. Related: `a-picked-face-is-named-by-its-placement`.

## After `doors/face-words` (2026-11-01)

The header now says the face through `tree::PickedName`: the leaf
role's words, whose feature is still the part's node the assembly does
not hold (`the end cap of node a59299af4a92`), then `, on
InstantiatePart "InstantiatePart 2" (…)` — so the instance is named, and
which face it is. What stays is this row's other half: the header's
Delete still targets the part's node (`PropertiesPane::entity_header_ui`
deletes `FaceSelection::feature`), which the assembly cannot act on.
