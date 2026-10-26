---
id: profile-arms-the-weld-renders-fall-short-of-the-shape-guard
kind: issue
title: ProfileError's ArcBelowSceneResolution renders 117 words, and InconsistentArc states its repair unlabelled
status: parked
opened: 2026-10-20
priority: P4
cost: E
blocked_on: [profile-refusals-say-what-they-mean]
rides_with: profile-refusals-say-what-they-mean
---


Found by `wire-census2`, whose PR welds the feature-tree census's
`Profile/` rows to `profile::ProfileError`'s arms
(`every_forwarded_arm_has_a_row`, `crates/editor-core/tests/refusal_concision_chains.rs:1366`). The arms the
roster turned up with no row render short of the standard
(`crates/profile/src/validate.rs`, `ProfileError`'s `Display`):

- **`ArcBelowSceneResolution`** (`validate.rs:1338`, `Display` at
  `:1469`) renders 117 words through the feature
  tree, over the 75-word budget, on `at` loop 0 segment 1,
  `ArcCheck::Landing`, margin 3e-10 and headroom 1e-9. Most of it is
  the recourse: "Recourse: move the profile nearer the origin, or
  author the arc with a smaller radius (a flatter arc stores its centre
  farther out); a coarser tolerance also reads it, and an arc the path
  lattice constructs is verified at its construction, so validating its
  loop with that provenance (a `ConstructedProfile`,
  `pncad::validated`) does not re-read it". The opening also reads
  "an arc whose its sweep does not carry …", a doubled determiner. The
  row stands on `OVER_BUDGET` (`crates/editor-core/tests/refusal_concision_chains.rs:106`) at a ceiling of 117 words,
  citing this file.
- **`InconsistentArc`** (`validate.rs:1327`, `Display` at `:1481`) states its repair without the label: "… its
  start vertex is off its carrier circle — author the arc through the
  path lattice, which stores a carrier consistent with its ends". On
  `FILED_NO_RECOURSE` under a comment citing this file.

## Repair shape

Cut `ArcBelowSceneResolution` to the budget (the provenance clause is
the candidate) and fix the determiner, then take it off
`OVER_BUDGET`. End `InconsistentArc` in a
labelled `Recourse:` and take it off `FILED_NO_RECOURSE`.

**2026-10-21.** The recourse's "a coarser tolerance also reads it" is
gone (`nurbs/scene-resolution-recourse`, D4 ¶1 (i)). The row now
renders 85 words and stands on `OVER_BUDGET` at 85, still over the
75-word budget, so the cut above is still owed.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEPROSE (`work/profileprose/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

## Rides with `profile-refusals-say-what-they-mean` (2026-10-26)

PROFILEPROSE folded into PROFILETAIL; its refusal-text rows travel as one unit. This row is that unit's spec for its part and closes with its own evidence when the unit lands.
