---
id: offset-doors-small-doc-and-message-drift
kind: issue
title: Offset doors: a door name printed on another door's errors, stale shell.rs module docs, two conventions for a move's distance, and stale test prose
status: open
opened: 2026-09-28
priority: P4
cost: E
---


## Finding

The two designers who weighed `dup/the-chart-partition-has-a-topo-src-home-and-a-test-home` (2026-09-28) found four small drifts while reading the offset doors. Neither designer could edit the repo, so they were not fixed there. They are filed here because a redesign of the doors, if Ev takes it, rewrites most of these lines anyway.

- **The wrong door's name on its errors.** `ReplaceFaceError::Op`'s `Display` prefixes "replace_face_offset:" even when the error comes from a simultaneous door (`crates/topo/src/replace_face.rs` ~640–643).
- **Stale module docs.** `crates/topo/src/shell.rs`'s module docs, item 1 (~30–45), say "two doors, anything curved goes chart by chart". The code picks among three doors, including the axial one (`offset_door`).
- **Two conventions for which way a distance points.**
  - Along the chart's stored normal: `ChartMove`, `replace_face_offset` and `shell::inward`. This is what the code does.
  - Along the face's outward direction: the axial door's nappe comment and `KERNEL-VERBS`.

  The docs should say one thing.
- **Stale test prose.** `sweep/tests/sf2a_r2_probes.rs`'s `r2a_one_move_spanning_two_planes` says "nothing checks it". The doors refuse that case now (`TogetherChartMixed`), and `ChartMove`'s docs say so.

A fifth finding is not repeated here: one chart split across two moves gets past the doors' checks. It is part of the redesign question on the dup row. If the redesign is not taken, it becomes its own row here: a structural refusal when two moves name one surface key.
