---
id: TCOST-C1
kind: unit
title: corrupt input (release profile) job to the nightly
status: dispatched
opened: 2026-09-03
branch: tcost/c1-corrupt-input-nightly
---

CI-posture unit (Ev's ask, in-chat 2026-09-03). The job builds topo's
lib tests in release with debug assertions off and runs five rows, two
of them `cfg(not(debug_assertions))` and run nowhere else. A regression
there persists in the tree, so the job may be demoted: it moves to
`nightly.yml` verbatim (count guard and name-grep tripwires included),
leaves `ci.yml`, and the billed-minute delta (−2 per topo-closure run)
goes in `docs/CI-MINUTES-2026-08.md`.
