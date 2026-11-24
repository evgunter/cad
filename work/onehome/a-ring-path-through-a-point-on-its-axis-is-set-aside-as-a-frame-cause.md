---
id: a-ring-path-through-a-point-on-its-axis-is-set-aside-as-a-frame-cause
kind: unit
title: Carrier frames (ii), ring_path rest: parallel and the crest meet mint their frames decided, a degenerate path set aside as a Frame cause
status: parked
opened: 2026-11-23
priority: P1
cost: M
parent: carrier-frames-the-production-mints
blocked_on: [7631]
---


The two production `Frame::new` sites in topo that the operands lane's PR
for the rest of the topo mints left (`onehome/mints-topo-rest`), held
until PR 7631 merges because their consumers live in `chord_join.rs`, which
7631 rewrites.

- `crates/topo/src/ring_path.rs:552`, `parallel`: `u = (from − centre)
  .normalize()` names no direction where `from` is the circle's centre
  (a parallel of no radius, a path through a point on the axis), and one
  caller's axis is itself arithmetic (`ef × axis`).
- `crates/topo/src/ring_path.rs:1501`, a meridian against a crest: the
  plane's `w` is the meridian centre's offset from the core line,
  normalized, which names no direction on the core axis.

**The ruling (orchestrator, 2026-11-23): (ii), typed.** A degenerate path
is a path that cannot be used, not a fault of the whole ring: the walk
sets it aside and asks the next path, as it does an in-band one. The cause
is carried as its own kind, a `Frame` cause beside `Indeterminate` in what
`chord_join`'s `Rung` keeps (`paths_rung`, `path_parity`,
`Carrier::paths`/`detours`), so that a final `RingSide::Undecided` or a
refusal that rests only on degenerate paths says so and never reads as
in-band. Never fold a decided degenerate, `NonFinite` or `Underflow` into
`Indeterminate`.

Each site's frame is decided where it has no decision today (the
decided mint, then the frame stated, in one local helper per file, as the
rest of the topo mints are), with a row feeding each its degenerate input
and a mutant.
