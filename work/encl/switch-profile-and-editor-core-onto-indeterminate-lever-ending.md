---
id: switch-profile-and-editor-core-onto-indeterminate-lever-ending
kind: issue
title: profile's validate::lever_ending and editor-core's composed LeverOnly re-spell geom-core Indeterminate::lever_ending
status: closed
pr: 5003
opened: 2026-10-13
branch: encl/lever-ending-switch
priority: P3
cost: E
closed: 2026-10-13
---


## What

PR 4766 landed geom-core `Indeterminate::lever_ending`, the one lever-only-at-build ending. Two copies remain:
- `crates/profile/src/validate.rs` `fn lever_ending`;
- editor-core `crates/editor-core/src/decision.rs`, which composes `geom_brep::recourse::LeverOnly`.

Both are named in `too-close-to-call-remainder`'s profile and editor-core slices as moving to `Indeterminate::lever_ending` once PR 4766 lands.

## Repair shape

Both call `Indeterminate::lever_ending`, and the local copies go. Pins move only if the renderings differ; say what moved.
