---
id: viewer-cannot-author-a-duplicate-node
kind: issue
title: viewer: a 'duplicate' node reachable from the UI (Ev's request)
status: open
opened: 2026-09-17
---


**Ev requested this specifically** (in chat, 2026-09-17): a
**duplicate** node, authorable from the viewer's UI.

Ev's expectation, recorded as said rather than investigated: it should
be very close to **transform**, and could even be a **subtype of
transform**. The next step after duplicating is generally to move the
copy away anyway, so duplicate is "transform, with the original kept"
where transform is "transform, with the original consumed".

Not investigated when filed. Whoever takes this row should check first
whether the document vocabulary already has a node that keeps its input
(EDIT's ground if it doesn't), and decide whether to build duplicate as
a flag on transform or as its own node. That choice is a design
decision to bring to Ev, not to settle in the lane.
