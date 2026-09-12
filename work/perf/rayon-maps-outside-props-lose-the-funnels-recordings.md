---
id: rayon-maps-outside-props-lose-the-funnels-recordings
kind: issue
title: editor-core's other rayon maps record into the worker's frame and sink
status: open
opened: 2026-09-12
---



## The finding

`geom_core::k_stats` records into two THREAD-LOCAL channels: the open
`Bracket`'s frame (`FRAMES`) and, under the `probe` feature, the sample
sink (`SINK`). Work handed to a rayon worker therefore records into
that worker's frame stack and sink, and the caller's open bracket and
installed sink see none of it — the funnel's own
`work_on_another_thread_records_nowhere_while_this_thread_holds_a_bracket`
is that loss, stated as a row.

Four rayon maps in `editor-core` run kernel decisions on workers, and
none of them composes the recordings back (PERF-8 added the door that
would: `k_stats::detached` / `k_stats::splice`):

| site | switch | default |
|---|---|---|
| `mc.rs`'s sample map (`McConfig::parallel`) | runtime | **on** |
| `stackup.rs`'s two maps (per-name, per-leaf) | runtime | per caller |
| `drive.rs`'s leaf map (`DriveConfig::parallel`) | runtime | off |

`eval/mod.rs`'s node map is the fourth and is `wire`'s, filed as
`work/wire/parallel-node-map-loses-the-funnel-and-the-symbolic-session.md`
— it carries a second defect this one does not.

**What it costs today.** A verdict log taken around one of these calls
is short by everything the workers decided, and a `probe` run's sample
population — the numbers `docs/K-REPORT.md` and `tools/k-lint` are
computed from — shrinks with the thread count rather than being a
property of the document. `mc.rs` is the one whose switch is ON by
default, so its sampler is the live instance.

**What it does NOT cost.** No decision changes: these scalars carry no
other thread-local state (`drive.rs` opens its symbolic session INSIDE
the leaf, on the worker that runs it). This is a recording loss, not a
determinism one.

## What a fix is

Each unit runs under `geom_core::k_stats::detached` on its worker and
the map's sequential fold `splice`s the recordings back in the map's own
index order — the shape `topo::props`' face walk uses, one call each
side. The order is the fold's, so the composed log is the serial walk's
at any thread count.
