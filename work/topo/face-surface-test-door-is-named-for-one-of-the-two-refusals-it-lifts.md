---
id: face-surface-test-door-is-named-for-one-of-the-two-refusals-it-lifts
kind: issue
title: set_face_surface_stranding_for_tests is named for one of the two refusals it lifts
status: open
opened: 2026-10-01
priority: P4
cost: E
refs: [set-face-surface-passes-a-swap-off-the-faces-own-boundary, 3598]
---

## What

Found by the review of PR 3598 (style Q4/Q6).
`Body::set_face_surface_stranding_for_tests` (`crates/topo/src/attach.rs`)
takes out both of `Body::set_face_surface`'s refusals of a swap it
cannot vouch for: `RechartStrandsDescriptions` and, since PR 3598,
`RechartUnvouched`. Its name says only the first. Several of its call
sites lift only the second (each says which in its `// Lifts` comment).

## Fix

Rename it for what it does, e.g. `set_face_surface_unvouched_for_tests`,
and re-spell every caller. It is a mechanical rename across about 45
files in topo, sweep, mesh, step-import and editor-core. It was deferred
from PR 3598 because many of those files are live lanes' ground. Take it
when they are quiet, or in a lane that already owns most of them.
