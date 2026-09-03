---
id: TCOST-C3
kind: unit
title: python suite seed-keyed like the viewer toolkit; ungated nightly
status: dispatched
opened: 2026-09-03
branch: tcost/c3-python-suite-seeded
---

CI-posture unit (Ev's ask). `pncad-py` sits in nearly every kernel
change's closure, so the wheel — another kernel compile with the
`python` feature — is built on almost every code-tier run. Seed-key it
exactly like `RUN_VIEWER_TOOLKIT`: PRs run it only when the seeds
intersect {pncad, pncad-py, editor-core} or the tier is `all`, an
always-run step prints the verdict, and the nightly runs it ungated.
