---
id: a-face-of-an-instance-is-named-by-the-parts-node
kind: issue
title: a face picked on an instance is named by the part file's node, which the assembly does not hold
status: open
opened: 2026-10-31
priority: P2
---

## Finding

Found driving the viewer (2026-10-31; Ev: P2). After a mate between two instances of `plate.pncad`, the selection's Properties header reads `face of node a59299af4a92` beside a disabled `Delete node a59299af4a92`. `a59299af4a92` is the Extrude inside `plate.pncad`: the face was made by the part's feature, the assembly does not hold that node, so the header can only call it "node" and Delete cannot act. A face of an instance should be named by the instance (`face of InstantiatePart "InstantiatePart 2" — plate.pncad`), and Properties should offer what the instance offers. Related: `a-picked-face-is-named-by-its-placement`.
