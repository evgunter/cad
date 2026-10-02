---
id: montage-kernel-reset-discards-freecad-rebaseline
kind: issue
title: render.yml's montage job resets the tree after the kernel lane commits, so the freecad lane's drift in the same job is never seen
status: open
opened: 2026-10-02
priority: P3
cost: E
---

Filed by the SHOW orchestrator from `render-on-a-commit-tag`'s lane (PR 3791). The lane noticed this while reading and did not chase it; it predates that PR.

## Finding

In `.github/workflows/render.yml`'s montage job the kernel and freecad lanes re-baseline in sequence. The kernel lane's `rebaseline-lane` step runs `git reset --hard origin/$PUSH_TO` before it commits.

When the kernel lane commits, that reset puts the freecad lane's rendered cells in the working tree back to the branch tip. So the freecad lane's drift in the same job can go unseen, and is never committed or reported.

Unverified: whether the freecad cells are rendered into the tree before the kernel step runs, which is what decides whether the reset can reach them. Read the step order in the montage job, then plant a two-lane drift on a scratch branch.

## Where

`.github/workflows/render.yml`, the montage job's two re-baseline steps.
`.github/actions/rebaseline-lane/action.yml`, the reset.
