---
id: a-wide-loft-section-escalates-chord-side-at-1e-12-on-main
kind: issue
title: A wide loft section escalates chord_side at the 1e-12 row on main
status: open
opened: 2026-10-12
priority: P0
cost: E
---


## What

`sweep::loft_profiles_door::at_interval_a_wide_section_refuses_as_section_not_a_point`
fails at `CAD_TOLERANCE_EPS=1e-12` on main (`d7eb3abe8b`): validating the
section escalates `chord_side` with band zero 1e-12 instead of refusing
the section. The test arrived with PR 4741
(`intent/nurbs-weights-knots-in-t`). Main's per-PR CI does not run
sweep's eps rows unless a diff touches sweep, so main reads green; the
first run to take the row was PR 4740's (CI run 38184535720, eps
1e-6/1e-12 job), and the failure reproduces identically on `origin/main`
in a clean worktree. The nightly will go red on it.

Filed by CONTACTHOLD, which met it on PR 4740 and did not fix it.
