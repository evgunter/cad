---
id: node-error-kind-transform-and-mirror-have-no-producer
kind: issue
title: NodeErrorKind::Transform and NodeErrorKind::Mirror have no producer once a placement, the twist and the mirror move what a value holds
status: open
opened: 2026-11-10
priority: P3
cost: M
---

Found by the ONEHOME rigid-motion lane (branch `onehome/rigid-motion`).

## What

A placement's read into a computing frame (`eval::Framed::read_in`),
the computing frame's twist (`eval/wire.rs`, `twisted`) and the mirror
(`wire_mirror`) now move what a value holds (`eval::Held::moved`,
`topo::FinishedBody::moved`). A move's one refusal is the session box,
which reaches the tree as `NodeErrorKind::NotFinished`. So nothing
constructs `NodeErrorKind::Transform` or `NodeErrorKind::Mirror`
(`crates/editor-core/src/eval/mod.rs`) any more, although both arms
are still matched:
- by `eval/class.rs`;
- by `viewer/src/tree.rs`;
- by `pncad-py/src/tags.rs` (`transform_error_tag`), with its tests;
- by `editor-core/tests/refusal_concision_chains.rs` (`transform()`).

## Repair shape

Delete both arms, the `NodeErrorClass` arms they map to and every
match on them. Re-pin the tag inventory. `topo::TransformError` stays:
it is `transform_rigid`'s and `mirror_body`'s, and the STEP reader's
`Placement`.
