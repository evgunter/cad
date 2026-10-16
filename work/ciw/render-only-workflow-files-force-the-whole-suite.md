---
id: render-only-workflow-files-force-the-whole-suite
kind: issue
title: A render-only workflow change runs the whole suite
status: closed
opened: 2026-10-16
closed: 2026-10-16
priority: P3
cost: E
branch: ciw/render-only-tier
refs: [an-all-tier-diff-with-no-crate-seed-runs-no-slow-row, 5221]
---

## Found

`scripts/ci-filter.py` `classify` sent every `.github/**` path to TIER=all
(its ALLOWLIST block: only a file inside a member's directory is scopable).
TIER=all is a cold workspace build, the whole suite at three ε rows and,
with no crate seeded, the whole slow set: about 45 min on the `test` job.

That is right for `ci.yml` and for anything the gate runs. It is not right
for `.github/workflows/render.yml` and `.github/actions/rebaseline-lane/**`:
nothing the PR gate runs reads them, so they cannot move a Rust test
result. PR 5221 paid the full run for an edit to rebaseline-lane's error
message. Ev asked for the fix on 2026-10-16.

## Closed

`classify` treats a render-only unit as an aux path, like `demos/`: no
crate tier, lint only. `_render_only_units` derives the set from the tree:
render.yml counts unless ci.yml reaches it, and any other workflow or
action counts when it is referenced and every referrer is render-only.
ci.yml's lint job gains `workflow lint (actionlint, shellcheck)`, which
runs on every tier. The `[render]` dispatch reads the head commit's
subject in the filter job and is independent of the tier.
