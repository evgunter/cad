---
id: transform-certify-refusal-names-the-edge-by-arena-key
kind: issue
title: topo: TransformError::Certify's Display names the mapped edge by arena key in the feature tree
status: open
opened: 2026-09-28
priority: P2
cost: E
---


(ENCL implementer, from `certify-escalation-renders-the-coincidence-menu-unlabelled`.)

## What

`topo::TransformError::Certify`'s `Display` (`crates/topo/src/transform.rs`,
the `Self::Certify { edge, source }` arm) writes "mapped edge
{edge:?} failed re-certification: {source}", so the feature tree's
fault line shows `EdgeKey(3v1)`, a key the person holding the mouse
cannot find. `editor-core/tests/refusal_concision_chains.rs` admits it
by listing every row on that route in `KERNEL_KEYED`: `Transform/Certify`
and the six `Transform/Certify/Routed/*` rows. A certification
refusal at a transform is not a kernel bug the key serves: the
escalated arms end in a user lever.

## Repair shape

Name the edge in words ("an edge the map moved") and keep the key in
`Debug`; then drop the seven rows from `KERNEL_KEYED`, which is the check
that this is done.
