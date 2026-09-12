---
id: mass-properties-are-serial-per-face
kind: issue
title: mass properties and the sign walk visit faces one at a time, and the K-funnel records per thread
status: open
opened: 2026-09-12
---


## The finding

`topo::props` (`crates/topo/src/props.rs`): `mass_properties_impl`
calls `face_flux` per face in arena order and `fold_runs` sums in
arena order; `sign_certified` (PERF-6) does the same per round over
the open faces. The per-face quadrature is the whole cost on curved
bodies (`loft_prism` 157 ms; the round spout 11–18 s per door). D9's
addendum calls this the canonical idiom-2 example. The blocker is not
the loop: the lanes decide through `geom_core::k_stats`, whose
recording is thread-local (`FRAMES`, and `SINK` under `probe`), so a
face decided on a worker thread records into the wrong frame and
k-lint's population shrinks with the thread count.

## What a fix is

A composing door on the funnel (run under a detached frame, hand back
the recording; append a recording to the current frame in the order
given), then idiom 1 per face with the recordings appended in the
arena-order fold; verdict logs and sample counts byte-identical at any
thread count. `docs/PERF-8-SPEC.md`.
