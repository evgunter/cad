---
id: a-sweep-states-no-relation-between-its-profiles-planes
kind: issue
title: Node::Sweep reads two 2-D profiles with nothing stating the path's plane against the profile's, or where the profile sits on the path
status: open
opened: 2026-11-02
priority: P3
cost: M
design: true
refs: [the-klein-loop-sweep-has-no-document-spelling]
---


Found by PLACING's second INTENT sweep (2026-11-02; Ev on PR 6281), read at `72c637cf98`. Filed for a design read before the sweep's geometry is built. (PLACING orchestrator)

- `Node::Sweep { profile, path, stations, v_degree }` (`crates/editor-core/src/node.rs`) reads two `Profile`s; the path is the first loop of a 2-D profile.
- `[variables.kinds]` says a Profile's numbers "are read only against each other", and that a sweep reads a profile plus at most a heading and tilt, or a point and a heading. The path is none of those.
- Nothing states the path's plane against the profile's, or where and how the profile sits on the path. Loft settled the same question by giving each later section station slides and spins in the first section's axes; Sweep has no such field.
- The node's space follows `profile` alone (`doc.rs`). The path's space is never compared, so a path in another space is not refused at the door.
- `wire_sweep` (`eval/wire.rs`) refuses unconditionally (`SWEEP_FRONTIER`), so this is latent until the sweep is built.
