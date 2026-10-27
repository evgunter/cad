---
id: quad-a-faces-round-zero-is-outside-the-wall
kind: issue
title: a face's round 0 is outside the work wall, and resuming past the wall is only debug-asserted
status: open
priority: P3
cost: E
parent: the-measurement-door-refines-until-a-predicate-resolves
refs: [5589]
opened: 2026-10-23
---

Raised by both DUAL reviewers on PR 5589. The fault is old in kind.

- **Round 0 is outside the wall.** `Run::stop`
  (`crates/geom-brep/src/props/quad/`) spends a round's cost
  after the lane has run it. The wall only decides whether the NEXT
  round runs, so a face whose round 0 alone costs more than
  `FACE_WORK_WALL` still pays for it in full. The round caps had the
  same gap.
- **Resuming past the wall is only debug-asserted.** `Run::enter`
  (`props/quad/`–`748`) replays the earlier rounds' cost and only
  `debug_assert!`s that the entering window's first round fits. A
  release caller that resumes a window past the wall runs that round.

Fix shape: check whether `cost(first)` fits before running round 0 or
a resumed round, and refuse typed (or answer `Wall` with no
enclosure) instead of spending.
