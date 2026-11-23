---
id: an-offer-button-names-a-variable-unlike-every-other-viewer-surface
kind: issue
title: An offer button names an anonymous variable by the slot reading it, where every other viewer surface speaks its SpokenVar tag
status: open
opened: 2026-11-02
priority: P4
cost: E
---

Filed by the REFS orchestrator (2026-11-02) from PR 4247's stated
follow-up, which was never given a file.

`props::variable_label` (`crates/viewer/src/props.rs`) names an
unnamed variable by the first slot that reads it ("the distance of
Extrude …", VR2's wording), and the offer's buttons use it. Every other
viewer surface speaks an unnamed variable by its `SpokenVar` tag
(`Refusal::DrivenByExpression`'s `variables`, `SlotDriver::Expression`,
the variable rows' `label`). One variable therefore reads two ways in
one panel. Pick one spelling and route both through it.
