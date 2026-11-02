---
id: a-mirror-of-an-unplaced-body-labels-its-union-mirror
kind: issue
title: The mirror tool's label lands on the Union it inserts, proposed as 'Mirror N'
status: open
opened: 2026-11-02
priority: P3
cost: E
---


## Finding

`pane/create.rs` `tool_noun` proposes "Union N" for a drawn body and
"Mirror N" otherwise. But `session.rs` `add_mirror` always inserts
the Mirror and then `Union[body, image]`, and returns
`Wrote::one(part)`. So the creation label lands on the Union, and an
unplaced body's "Mirror N" label names a Union node. The comment
"an unplaced body's image stands alone" is stale against the code.
Under the create rule the label box goes, and the insert stores
"Kind N" on the node the gesture's noun names. Decide which node that
is for a mirror, and label it.
(FORMS orchestrator, from a read-only scout of origin/main.)
