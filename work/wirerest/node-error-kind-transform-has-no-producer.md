---
id: node-error-kind-transform-has-no-producer
kind: issue
title: NodeErrorKind::Transform has no producer once a placement and the twist move what a value holds
status: open
opened: 2026-11-10
priority: P3
cost: M
---

Found by the ONEHOME rigid-motion lane (branch `onehome/rigid-motion`).

## What

A placement's read into a computing frame (`eval::Framed::read_in`)
and the computing frame's twist (`eval/wire.rs`, `twisted`) now move
what a value holds (`eval::Held::moved`, `topo::FinishedBody::moved`).
A motion's one refusal is the session box, which reaches the tree as
`NodeErrorKind::NotFinished`; its map refuses only a state a finished
body never holds (`topo::MoveError::under_a_motion`). So nothing
constructs `NodeErrorKind::Transform`
(`crates/editor-core/src/eval/mod.rs`) any more, although the arm is
still matched:
- by `eval/class.rs`;
- by `viewer/src/tree.rs`;
- by `pncad-py/src/tags.rs` (`transform_error_tag`), with its tests;
- by `editor-core/tests/refusal_concision_chains.rs` (`transform()`).

`NodeErrorKind::Mirror` keeps a producer: the mirror
(`wire_mirror`) moves through `Held::moved_by`, and a re-minted row's
refusal is the map's own (`topo::MoveError::Map`), raised as `Mirror`.

## Repair shape

Delete the `Transform` arm, the `NodeErrorClass` arm it maps to and
every match on it. Re-pin the tag inventory. `topo::TransformError`
stays: it is `transform_rigid`'s and `mirror_body`'s, the mirror's
`Mirror` arm, and the STEP reader's `Placement`.
