---
id: a-slow-row-added-to-a-demo-suite-runs-nowhere-on-the-pr-that-adds-it
kind: issue
title: A slow row a PR adds to a demo suite runs nowhere on that PR: the demos job has no step for the slow tests a diff adds
status: open
opened: 2026-11-23
priority: P4
cost: E
---


Found by BOXFINISH PR 7384's second review (2026-11-23), by execution.

The kernel and app test jobs have a step that runs the slow tests a
diff adds or edits. The `demos (tour + wild)` job has none: its
`demos tour suite` step runs `--profile ci`, and a test in
`demos/tour/.config/nextest.toml`'s slow set is selected by nothing on
the PR.

The instance: PR 7384 added
`demos/tour/tests/tolerance.rs`
`the_cut_plates_first_subtract_refuses_a_bores_u_ref_over_the_study`
to the tour's slow set (0.44 s on CI's profile). On the hosted run
(45078968298, head `54418d90`) the demos job was green and the row did
not run; locally `--profile ci` selects 0 tests for it. Its only
evidence on the PR is the author's and the reviewer's local runs. The
nightly runs it, at `--release`.

Same family as
`a-slow-rows-eps-pins-run-nowhere-on-the-pr-that-adds-them` and
`demos-row-skips-kernel-changes-to-doors-the-tour-calls`.

**Closing test:** a PR that adds a slow-set row under `demos/tour` or
`demos/wild` shows that row executed in its hosted run's steps.

(BOXFINISH orchestrator)
