---
id: an-euler-attachment-refusal-opens-with-a-stage-prefix
kind: issue
title: an Euler op's certification refusal renders with the stage prefix 'geometry attachment gate:' in the feature tree
status: open
opened: 2026-11-10
priority: P3
cost: E
---

Found by the ONEHOME rigid-motion lane (branch `onehome/rigid-motion`).

## What

`topo::EulerOpError::Certification`'s text (`crates/topo/src/euler.rs`,
the `format!("geometry attachment gate: {}", …)` arm, ~:1396) opens the
certification refusal with a stage label. Through the feature tree it
reads "the extrude op refused: an Euler operation refused: geometry
attachment gate: the crossing angle at sample 4 escalated: …".
`editor-core/tests/refusal_concision_chains.rs`'s
`every_node_refusal_renders_within_the_budget` refuses a row that
carries that stage prefix.

No row reached this route before. A transform's re-certification
(`TransformError::Certify`) carried the routed certify refusals until
the rigid-motion unit removed that arm, since a move now carries its
certificates. The rows moved to
`Extrude/Op/Certification/Routed/*` (`certify_refusal_routes`). They
are held out of the budget roster until this prefix goes, and
`every_certify_refusal_ends_in_its_routed_sentence` still reads each
one's ending.

## Repair shape

Drop the stage label, as
`replace-face-refusals-open-with-a-stage-prefix-and-name-keys` (SHELL)
and `merge-coplanar-refusals-open-with-a-stage-label` did for theirs,
then chain `certify_refusal_routes()` back into `node_refusals`.

## Evidence: the label is gone, the budget still bites (walks/refusal-split)

The arm is now `EulerOpError::DescriptionUncertified { edge, error }`,
whose text states the fact rather than the stage ("the minted edge's
description does not certify: …", or "edge {key}'s …"). Chaining
`certify_refusal_routes()` back into `node_refusals` then fails
`every_node_refusal_renders_within_the_budget` on two rows only:
`…/Routed/lever arm, enclosure` (76 words) and
`…/Routed/spline meter turns back` (77 words), over 75 by one and two.
What is left is a shorter opener or a shorter ending for those two;
the roster stays unchained until then.
