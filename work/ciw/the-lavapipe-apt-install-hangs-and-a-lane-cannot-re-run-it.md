---
id: the-lavapipe-apt-install-hangs-and-a-lane-cannot-re-run-it
kind: issue
title: The viewer-app job's lavapipe apt install hangs past its 8-minute step timeout, and an agent lane cannot re-run the job (403)
status: open
opened: 2026-10-22
priority: P4
cost: M
---


Filed by FIT's orchestrator, 2026-10-22.

## Finding

`ci.yml`'s `viewer app feature (eframe + wgpu, wasm32)` job runs
`scripts/apt-install.sh --no-install-recommends mesa-vulkan-drivers vulkan-tools`
in the step `install a software Vulkan adapter (lavapipe)`. Twice
today, on PR 5596's branch `fit/ctrl-wheel-zoom`, that step hung,
retried once ("attempt 1 failed or hung; retrying") and then died at
the step's 8-minute timeout. Both times the build had finished and no
test body had run. The second was run 39826010533, job 119989799102,
at head `38df5e3880`.

## Why it costs more than a re-run

The agent integration cannot re-run a failed job:
`rerun-failed-jobs` answers `403 Resource not accessible by
integration`, both from an implementer lane and from an orchestrator.
So a hung install leaves a lane two options, and both are wrong. It
can push an empty commit, which the standing rules forbid (one lane
did, earlier the same day, on the same PR). Or it can leave the PR red
until an unrelated push.

## What a fix has to decide

Either the install stops hanging (for example a mirror, a cached
`.deb`, a pre-baked image, or a tighter per-attempt timeout so the
retry has room), or a hung install becomes re-runnable from where a
lane stands (for example a `workflow_dispatch` or comment-triggered
re-run the integration can reach). The second is the general one,
since any job that dies before its tests runs into the same wall.
