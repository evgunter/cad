---
id: an-open-sign-row-reds-main-at-1e-6-with-section-loop-mixed
kind: issue
title: reach_volume_backstop's an_open_sign_beyond_the_band row reds main at CAD_TOLERANCE_EPS 1e-6 again: the scaled oblique rod's boolean refuses SectionLoopMixed
status: open
opened: 2026-10-01
priority: P0
cost: E
---

## What

`crates/sweep/tests/reach_volume_backstop.rs`,
`an_open_sign_beyond_the_band_at_the_last_round_refuses`, fails on
`origin/main` at `CAD_TOLERANCE_EPS=1e-6`. It passes at the default ε
and at 1e-12. The failure is at :41 (`build`):

    the scaled oblique rod: the boolean refuses: Join(SectionLoopMixed { face: FaceKey(9v1) })

## Who measured it

- Hosted CI on EDIT's PR 3676, run 36920139603 (the PR merged into
  main at `4c95f504c`) and run 36927574529 (merged into `97d9e3380`),
  eps step at 1e-6. That PR does not touch `sweep`, `topo` or
  `geom-*`.
- EDIT's fix lane reproduced it on a clean `origin/main` (`97d9e3380`),
  in its own worktree and target dir:
  `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep -E 'test(/an_open_sign_beyond_the_band/)'`.

- REACH's #3737 fix pass reproduced it identically on
  `reach/pxn-envelope-red` (which carries main to `978feeb296`), at
  1e-6 only, and the #3737 reviewer on `9fb3de684`. #3737's own change
  (the Boehm step's hull meet) does not move it.
- The refusal is `SectionLoopMixed`, the shape of ZIP's P0
  `role-resolution-interior-tiers-certify-only-planar-region-faces` (a
  chord-midpoint probe on a curved edge reads both loops alike). That
  it is the same defect is inferred, not shown.

## History

This is the row's third shape at 1e-6:
- `reach-volume-backstop-fails-off-the-default-eps` (closed by
  PR 3636) restated it at s = 1e12·ε and 1e13·ε;
- `work/contact/point-in-solid-ray-denominators-are-not-lengths.md`
  records that it then built by the last bits of its scale.

It now refuses earlier, in the boolean's join, not the point-in-solid
denominator, so something on main since PR 3636 moved the rod's
section. That is not bisected here. Candidates by date are the
convex-step change (#3524) and the LINALG door adoptions (#3725,
#3727).

## Final state

The row holds at every ε the CI runs, or names and pins where its
verdict changes with ε. Main's eps step is green for a diff whose
eps filter is `all()`.

Filed by the EDIT orchestrator. Every PR whose eps filter reaches
`sweep` is blocked by it.
