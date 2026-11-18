---
id: line-frame-is-a-third-frame-completion-seed-rule
kind: issue
title: step-import: adopt.rs's line_frame is a third frame-completion seed rule beside first_proj_axis and build_axes
status: open
opened: 2026-11-18
parent: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
priority: P4
cost: E
---


Found by the confirmation re-review of `importgate/interp-layer` (PR 7091).

## What

Three rules complete a frame from one direction in step-import:

- `crates/step-import/src/adopt.rs:1254` `line_frame` seeds with the
  coordinate axis of the direction's smallest component, projected off
  the direction, for a `PlacedSegment` placement.
- `crates/step-import/src/entities.rs:2625` `first_proj_axis` seeds the
  line reader's `build_axes` call (`unit_line_direction`).
- `geom_core::RigidMotion::from_axis_and_reference` (`build_axes`)
  completes every `AXIS2_PLACEMENT_3D`.

A line's direction and its placed segment's frame can therefore be
completed by two different seeds.

## Fix

One seed rule: `line_frame` builds through
`RigidMotion::from_axis_and_reference` with `first_proj_axis`'s seed (or
both use `line_frame`'s), so a line's frame is the one its direction's
read built.
