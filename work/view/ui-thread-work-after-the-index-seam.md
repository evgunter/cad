---
id: ui-thread-work-after-the-index-seam
kind: issue
title: Three unbounded steps still run inside the frame after the pick index moved off it
status: open
opened: 2026-09-05
---


## Where this came from

The sweep VIEW-6b owed. 6b moved the expensive step — `mesh::tessellate`
plus the triangle BVH, inside `PickIndex::build` — onto its own worker,
and the sweep for the SHAPE (unbounded per-document work run inside
`eframe::App::ui`, not behind a seam) found three more instances. None
is 6b's to fix: each is its own decision about a different seam, and
6b's fence was the index.

## The hits

1. **The display budget's probe tessellation**, `crates/viewer/src/
   scene.rs:917-919` (`fit_delta`), called from `crates/viewer/src/
   app.rs:~634` inside `sync_scene`. It gathers the product and
   tessellates it at `PROBE_FACTOR` (8×) the requested δ, so it costs
   about **an eighth of a full tessellation** — on the `hollowring`
   fine-δ row 6b measured (6.5 s of tessellation) that is roughly
   0.8 s of frozen window, once per document that ARRIVES. It runs
   BEFORE the index is submitted, so an `Open` still stops repainting
   for it.
2. **The drawable scene's vertex assembly**, `PickIndex::scene_focused`
   → `SceneMesh::build_parts_focused` (`crates/viewer/src/pick.rs:932`,
   from `app.rs:~672`). It walks every drawn triangle to build the GPU
   buffers, and it runs not only when an index lands but on every HIDE
   and every FOCUS change over an index that is already current — the
   two paths that reach it with no new tessellation behind them.
3. **The landing's gather, check registry and A5 certification**,
   `DocSession::land` (`crates/viewer/src/session.rs:613-655`), run
   from `pump` at the top of `sync_scene`. The advisory registry and
   `assemble_gathered` are kernel computations over the whole product;
   they are outside the evaluation seam by construction, because they
   run on the RESULT after it lands.

## Why it is not one fix

The index had somewhere to go: the seam vocabulary existed and the
payload was already `Send`. These three do not share that. (1) belongs
to the display budget and would want its probe on the index worker or
a cheaper estimator; (2) is per-frame-ish work whose input is the index
the worker already holds, so it is a question about what the worker
should RETURN rather than about a new seam; (3) is a landing-time
computation the evaluation seam deliberately does not carry.

`work/view/scene-gathers-the-landed-product-twice-more.md` overlaps hit
(1) and is a DIFFERENT question about it: that item is about gathering
the same product twice, this one is about where the remaining cost
RUNS. Neither subsumes the other.

## Cost, honestly

Unmeasured. 6b measured the step it moved; these three are named from
their call sites and their shapes, not from a stopwatch, and (2) and (3)
in particular could be milliseconds. A taker should measure before
choosing, the way #1259's own table did.
