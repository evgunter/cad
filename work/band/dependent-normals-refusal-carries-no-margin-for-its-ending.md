---
id: dependent-normals-refusal-carries-no-margin-for-its-ending
kind: issue
title: blend: UnsupportedCorner(DependentNormals) carries no margin, so its band-decided Zero arm cannot offer the valued tighten its in-band sibling does
status: open
opened: 2026-10-01
---


(BAND implementer, from `band/recourse-tables-decide-per-tag`.)

## What

`battery::corner_config` refuses a decided-Zero
`fillet3_corner_independence` as `BlendError::UnsupportedCorner {
corner: CornerConfig::DependentNormals, .. }`, which carries the tag and
the policy and no margin. Its recourse is now the decision's own lever
(`FILLET3_CORNER_INDEPENDENCE_RECOURSE`, via `CornerConfig::recourse`),
shared with the in-band arm as D4 ¶1 (iv) asks — but the in-band arm
ends through `BlendDecision::recourse`, which on a margin inside the
zero band but nonzero adds the valued "if this spread of the face
normals is intended, tighten the tolerance below m/K", and the definite
Zero arm cannot: the margin it was decided on is not in the payload.

D4 ¶1 (i) names the decided-Zero arm of a decision that passes on a
positive sign as band-decided, so it owes the same valued conditional.

## Repair shape

Carry the `ClassifiedMargin` on the `DependentNormals` refusal (a field
on the tag, or on `UnsupportedCorner`), and end it through
`BlendDecision::CornerIndependence.recourse(margin.arm())`, as the
other definite arms now do.
