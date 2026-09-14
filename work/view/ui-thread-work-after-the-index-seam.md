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
   scene.rs:994-1000` (`fit_delta`), called from `crates/viewer/src/
   app.rs:~634` inside `sync_scene`. It gathers the product and
   tessellates it at `PROBE_FACTOR` (8×) the requested δ, so it costs
   about **an eighth of a full tessellation** — on the `hollowring`
   fine-δ row 6b measured (6.5 s of tessellation) that is roughly
   0.8 s of frozen window, once per document that ARRIVES. It runs
   BEFORE the index is submitted, so an `Open` still stops repainting
   for it.
2. **The drawable scene's vertex assembly**, `PickIndex::scene_focused`
   (`crates/viewer/src/pickindex.rs:894`) → `SceneMesh::build_parts_focused`
   (`crates/viewer/src/scene.rs:439`), from `app.rs:~672`. It walks
   every drawn triangle to build the GPU
   buffers, and it runs not only when an index lands but on every HIDE
   and every FOCUS change over an index that is already current — the
   two paths that reach it with no new tessellation behind them.
3. **The landing's gather, check registry and A5 certification**,
   `DocSession::land` (`crates/viewer/src/session.rs:966-1060`), run
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


## Hit (2) is half-false as of #1908 (VIEW orchestrator, 2026-09-05)

Routed here rather than edited by the lane that invalidated it —
`work/README.md`'s one-file-one-item rule makes a second program's
edit of this file a merge conflict by design, and the lane reported it
instead, which is the contract working.

**`scene::fit_delta` no longer gathers.** This file's hit (2) reads
*"`fit_delta`'s probe tessellation and gather"* and cites
`scene.rs:994-1000`. #1908 (merged `b20e13da`) made `fit_delta` take
the landing's body, so:

- the **gather** half of that hit is gone — the landing pays it once
  and hands it on;
- what remains is the **probe tessellation**, which is still on the UI
  thread and still ~1/8 of a full one, once per document that arrives;
- the line numbers moved.

So the hit is smaller than recorded and still real. It stays on this
list; nothing about the *class* — unbounded per-document work run
inside `eframe::App::ui` — changed.

One residue of the residue, recorded because it is the kind of thing
that dies otherwise: #1908 left a **gather** on the refused-A5-gate
path, at the fit's own call site (`scene::product_of_evaluation`, in
`app.rs`'s fit block). It is deliberate, argued and named there, and it
runs once per opened document rather than per landing — but it is a
gather on the UI thread, so it belongs on this list rather than only in
`refused-a5-gate-eats-the-body-the-fit-then-regathers`.

## Hit (1) is DONE (2026-09-14) — and the other two are MEASURED

The lane took hit (1) only, and measured all three first. Method:
release build, `viewer`'s own corpus (`tests/corpus`), each document
landed through `DocSession::inline` and then timed at the two δ the
budget rows use (1e-4 and 1e-5). The numbers below are one machine's;
what they are for is RANKING the three, which is what the item asked a
taker to do.

### (1) — moved, and the ordering was load-bearing

`scene::fit_delta` (now `crates/viewer/src/scene.rs:1094`, not the
`:994-1000` recorded above) runs on a **third worker**: `evalseam`'s
`FitService` / `FitRequest` / `FitDone`, with `InlineFitter` and
`ThreadFitter` beside the two seams that were there. `app.rs`'s fit
block submits and `ViewerApp::take_fit` takes.

The ordering WAS load-bearing, and that is what made the fix bigger
than a `spawn`: the fit's answer is the δ the index is built at, so the
index cannot be submitted while a fit is outstanding without paying the
un-budgeted build the budget exists to avoid. `PickCache::sync` now
takes an `Option<DisplayTolerance>` and an unsettled δ takes the
nothing-to-index way out, which drops the previous document's index in
that window exactly as a submit would.

Measured cost of the ladder, which is what stopped being on the frame:
**118 ms** (`loft_prism`, 1e-5), **116 ms** (`tube_ring`), **64 ms**
(`hollow_tube_ring`). The ratio to a full tessellation of the same body
is **0.10–0.13** across every document dense enough to matter, which
reproduces this file's "about an eighth" as a measurement rather than
an estimate — and is what makes 6b's 6.5 s `hollowring` row the ~0.8 s
of frozen window recorded above.

The gather on the refused-A5-gate path went with it: the fit request's
`FitSubject::Ungathered` arm names the pair and the WORKER gathers.
`refused-a5-gate-eats-the-body-the-fit-then-regathers` still owns the
double gather itself; what is settled here is only where it runs.

### (2) is the BIGGEST of the three, by an order of magnitude — still open

`PickIndex::scene_focused` is at `crates/viewer/src/pickindex.rs:941`,
not the `:894` recorded above; `SceneMesh::build_parts_focused` is at
`scene.rs:439` as recorded.

The guess above — "(2) and (3) in particular could be milliseconds" —
is **false for (2)**. Timed over an index already built, at 1e-5:

| document | `scene_focused` |
| --- | --- |
| `hollow_tube_ring` | **5 123 ms** |
| `tube_ring` | 2 322 ms |
| `loft_prism` | 1 682 ms |
| `hollow_tube_elbow` | 205 ms |
| `die_composed_tour` | 132 ms |

At 1e-4 the same documents read 82 / 38 / 39 / 16 / 8 ms. This runs on
every HIDE and every FOCUS change over an index that is already
current, so unlike (1) it is not once per document — it is once per
interaction, and it is **ten times a full tessellation of the same
body** (`hollow_tube_ring` tessellates in 511 ms).

**That ratio is the thing a taker should chase first.** The step is a
linear walk that copies corners and computes one normal per triangle;
at ~5 µs per triangle it is far more expensive than a copy loop can
account for, so *where the time goes* is an open question and not
obviously answered by moving the walk to a worker. Two candidates
worth separating before choosing a seam: the index holds a mesh per
(node, body) rather than one for the product, so the walk may be over
several times the product's triangles; and the per-triangle work
itself may be doing more than it looks.

### (3) is the SMALLEST of the three — still open, and smaller than recorded

`DocSession::land` is at `crates/viewer/src/session.rs:966` as
recorded. Timed as one call on the landing frame: **under 6 ms on 27 of the 28
corpus documents that gather**, with one outlier at **197 ms**
(`loft_prism`). It does not depend on δ. So the class is real and the
instance is small: a row worth keeping for the outlier, not a frozen
window.
