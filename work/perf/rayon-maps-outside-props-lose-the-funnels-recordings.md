---
id: rayon-maps-outside-props-lose-the-funnels-recordings
kind: issue
title: editor-core's other rayon maps record their samples into the worker's sink
status: open
opened: 2026-09-12
---


## The finding

`geom_core::k_stats` records into two THREAD-LOCAL channels: the open
`Bracket`'s frame (`FRAMES`) and, under the `probe` feature, the sample
sink (`SINK`). Work handed to a rayon worker records into that worker's
frame stack and sink, and the caller's open bracket and installed sink
see none of it.

**The verdict channel is already safe at these sites, and that is worth
stating precisely because the obvious reading is wrong.** All three maps
below hand whole DOCUMENT EVALUATIONS to their workers, and
`editor_core`'s `eval_node` opens its own `Bracket` INSIDE the mapped
closure and returns the `Recorded` on the node. So the verdicts and
escalations of a node evaluated on a worker land in a frame of that
worker's and come back as a value — the first of the two sound shapes
`k_stats`' module docs name.

**What IS lost is the thread-local state nothing hands back**: the
`probe` sample sink (`start_recording` installs it on the caller's
thread; `take_samples` reads it there), which is the population
`docs/K-REPORT.md` and `tools/k-lint` are computed from, and
`geom_core::sym::report`'s `ACTIVE`/`SHAPES`/`NAMES`.

| site | switch | default |
|---|---|---|
| `mc.rs`'s sample map (`McConfig::parallel`) | runtime | **on** |
| `stackup.rs`'s two maps (per-name, per-leaf) | runtime | per caller |
| `drive.rs`'s leaf map (`DriveConfig::parallel`) | runtime | off |

`mc.rs`'s is the live instance, and its own comment beside the map is
the thing to read against this: *"the two schedules see the same samples
in the same order and the report is the same bits either way"*. True of
the BITS — it is a clean idiom-1 map — and silent about the recording,
which is not the same bits either way in a `probe` build. A comment that
is right about its own subject is how this class stays invisible.

`eval/mod.rs`'s node map is the fourth and is `wire`'s, filed as
`work/wire/parallel-node-map-loses-the-funnel-and-the-symbolic-session.md`
— it carries a second defect this one does not, and that one changes
decisions rather than recordings.

**Scope, so the claim is not read wider than it is.** No decision
changes at any of these three: `drive.rs` opens its symbolic session
INSIDE the leaf, on the worker that runs it, and `mc.rs` and
`stackup.rs` run no symbolic lane. This is a recording loss in a
non-shipped build configuration, not a determinism one.

## What a fix is

Each unit runs under `geom_core::k_stats::detached` on its worker and
the map's sequential fold `splice`s the recording back in the map's own
index order — the shape `topo::props`' face walk uses, one call each
side. The order is the fold's, so the composed population is the serial
walk's at any thread count. A `detached` frame around a closure that
already opens a bracket simply nests, which is defined, so the fix does
not have to tell the two cases apart.
