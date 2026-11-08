---
id: a-mirror-of-an-unplaced-body-labels-its-union-mirror
kind: issue
title: The mirror tool's label lands on the Union it inserts, proposed as 'Mirror N'
status: closed
opened: 2026-11-02
closed: 2026-11-03
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

## Closed

2026-11-03, by `the-seated-values-that-already-have-a-home`. The
gesture's noun is the mirror for every body: `add_mirror` answers the
`Mirror` node as what it made, so `Mirror N` lands on it and it is
selected, and the union of the symmetric part takes no label. The
mirror form holds no label box, and the stale comment went with
`tool_noun`'s placed-body branch.
