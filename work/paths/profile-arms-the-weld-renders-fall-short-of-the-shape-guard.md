---
id: profile-arms-the-weld-renders-fall-short-of-the-shape-guard
kind: issue
title: ProfileError's ArcBelowSceneResolution renders 117 words, and InconsistentArc states its repair unlabelled
status: open
opened: 2026-10-20
priority: P4
cost: E
---


Found by `wire-census2`, whose PR welds the feature-tree census's
`Profile/` rows to `profile::ProfileError`'s arms
(`every_forwarded_arm_has_a_row` in
`crates/editor-core/tests/refusal_concision_chains.rs`). The arms the
roster turned up with no row render short of the standard
(`crates/profile/src/validate.rs`, `ProfileError`'s `Display`):

- **`ArcBelowSceneResolution`** renders 117 words through the feature
  tree, over the 75-word budget, on `at` loop 0 segment 1,
  `ArcCheck::Landing`, margin 3e-10 and headroom 1e-9. Most of it is
  the recourse: "Recourse: move the profile nearer the origin, or
  author the arc with a smaller radius (a flatter arc stores its centre
  farther out); a coarser tolerance also reads it, and an arc the path
  lattice constructs is verified at its construction, so validating its
  loop with that provenance (a `ConstructedProfile`,
  `pncad::validated`) does not re-read it". The opening also reads
  "an arc whose its sweep does not carry …", a doubled determiner. No
  admission lets a budget breach stand, so the census holds no row for
  it: `ROWLESS` cites this file.
- **`InconsistentArc`** states its repair without the label: "… its
  start vertex is off its carrier circle — author the arc through the
  path lattice, which stores a carrier consistent with its ends". On
  `FILED_NO_RECOURSE` under a comment citing this file.

## Repair shape

Cut `ArcBelowSceneResolution` to the budget (the provenance clause is
the candidate) and fix the determiner, then give it a row in
`profile()` and take it off `ROWLESS`. End `InconsistentArc` in a
labelled `Recourse:` and take it off `FILED_NO_RECOURSE`.
