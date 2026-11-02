---
id: the-slow-tests-a-diff-adds-run-at-the-default-eps-only
kind: issue
title: The slow-set tests a diff adds or edits run at the default eps only, so a red at 1e-6 or 1e-12 first shows on the nightly
status: open
opened: 2026-10-28
priority: P2
cost: E
---


Found closing `work/sect/main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6.md`
(SECT, 2026-10-28).

## Measured

PR 5872 added `crates/geom-brep/tests/pair_section.rs` and edited
`crates/sweep/tests/cylinder_sphere_frame.rs`. Four of the rows it added
or edited are in the slow set, and all four were red at
`CAD_TOLERANCE_EPS=1e-6` on its merge; its gate was green, and the
nightly (run 40637814390) was the first to show them.

`.github/workflows/ci.yml` runs the slow-set tests a diff adds or edits
(step "test (the slow-set tests this diff adds or edits)") at the
default ε only. Its 1e-6 and 1e-12 step runs `--profile ci`, which
leaves the slow set out, so a test the diff itself added is never run
at either row before merge.

## What closing it takes

Run the step's seeded slow tests at 1e-6 and 1e-12 as well (or say in
`work/ciw/latency-cut.md` why the latency cut keeps them out). The four
rows cost 9–60 s each at 1e-6 and 4–50 min at 1e-12 in a debug build
on a shared box, so the 1e-12 row is the expensive half.
