---
id: viewer-cannot-author-a-duplicate-node
kind: issue
title: viewer: a 'duplicate' node reachable from the UI (Ev's request)
status: review
opened: 2026-09-17
priority: P0
cost: D
branch: author/part-and-duplicate
pr: 3052
---

**Ev requested this specifically** (in chat, 2026-09-17): a
**duplicate** node, authorable from the viewer's UI.

Ev's expectation, recorded as said rather than investigated: it should
be very close to **transform**, and could even be a **subtype of
transform**. The next step after duplicating is generally to move the
copy away anyway, so duplicate is "transform, with the original kept"
where transform is "transform, with the original consumed".

What a first look found (2026-09-17, not a design pass). The document
is a recipe DAG, and `Node::Transform { input, … }`
(`crates/editor-core/src/node.rs`) is a new node that places its
`input` without consuming it: the input stays in the DAG. So "keep the
original" is mostly a question of whether the input remains a visible,
independent body in the viewer, not a missing kernel capability.
`Node::Pattern` with a linear kind and count 2 already gives "the
original plus a moved copy" as a multi-body value, and `Node::Part`
selects one instance of it. It is worth comparing against before
building anything new.

Whether duplicate is a flag on transform, its own node, or a viewer
gesture over `Pattern`/`Transform` is a design choice to bring to Ev,
not to settle in the lane. If it does need a new document node, that
half is EDIT's (the document vocabulary), not CHROME's.
