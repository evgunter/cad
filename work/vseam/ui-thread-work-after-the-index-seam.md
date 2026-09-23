---
id: ui-thread-work-after-the-index-seam
kind: issue
title: Three unbounded steps still run inside the frame after the pick index moved off it
status: open
opened: 2026-09-05
priority: P1
cost: D
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
   (`crates/viewer/src/scene.rs:444`), from `app.rs:~672`. It walks
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
`scene.rs:444` (`:439` as recorded, shifted by #2661).

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

## (2) is DIAGNOSED (2026-09-15) — and the number above is an instrument artifact — still OPEN

Method: release, `viewer`'s own corpus through `DocSession::inline`
then `PickIndex::build`, scratch harness not in the diff, one machine
(4 vCPU, 15 GB, no swap). Every figure below is δ=1e-5 unless it says
otherwise.

### Neither candidate. The walk is linear and it runs at copy-loop speed

| document | triangles | tessellate alone | `PickIndex::build` | `scene_focused` FIRST at that size | steady | ns/triangle steady |
| --- | --- | --- | --- | --- | --- | --- |
| `hollow_tube_ring` | 11 605 976 | 7 349 ms | 21 694 ms | 4 449 ms | 664–971 ms | 62 |
| `tube_ring` | 6 393 816 | 7 112 ms | 15 206 ms | 2 445 ms | 385–426 ms | 64 |
| `loft_prism` | 5 043 838 | 5 286 ms | 9 290 ms | 2 177 ms | 325–346 ms | 67 |
| `hollow_tube_elbow` | 2 776 556 | 1 063 ms | 2 877 ms | 159 ms | 159–176 ms | 61 |
| `die_composed_tour` | 1 801 262 | 491 ms | 1 360 ms | 114 ms | 98–110 ms | 57 |

`hollow_tube_ring` at 1e-4: 1 161 216 triangles, tessellate 518 ms,
`scene_focused` first 78 ms, steady 59–63 ms — **53 ns/triangle**.

**Candidate one is false.** `PickIndex::parts()` holds ONE part on all
five documents, and `SceneMesh::stats().triangles` equals the
tessellation's own triangle count exactly (11 605 976 on both sides for
`hollow_tube_ring`). The walk is over the product's triangles once.

**Candidate two is false.** 53–67 ns per triangle, across five
documents spanning 6.4× in triangle count and across both δ. The loop
writes 3 corners × (12 B position + 12 B normal + 4 B id + 4 B flags)
plus 12 B of index per triangle = 108 B, so 62 ns/triangle is about
1.7 GB/s of stores. There is nothing hiding in the per-triangle work.

**The "~5 µs per triangle" above is arithmetic against the wrong
triangle count**, and so is the ratio: the record says `scene_focused`
is *"ten times a full tessellation of the same body
(`hollow_tube_ring` tessellates in 511 ms)"*. **The 511 ms is a δ=1e-4
tessellation** — I measure 518 ms there — while the 5 123 ms is δ=1e-5.
At EQUAL δ the ratio is **0.61× cold and 0.09× steady**, and at 1e-4 it
is 0.12× steady. `plan.md`'s *"a measurement can be an artifact of the
instrument, and the first number out is the one to distrust"* held
twice over on this row.

### Where the time actually goes: first-touch on ~1.25 GB of buffers

Phase split inside `SceneMesh::build_parts_focused`, `hollow_tube_ring`
at 1e-5, first call at that size vs. a later one:

| phase | first | later |
| --- | --- | --- |
| triangle count | < 1 µs | < 1 µs |
| the four `Vec::with_capacity` | 25–156 µs | 25–41 µs |
| the corner/normal loop | 3 591–4 263 ms | 586–645 ms |
| `indices` = `(0..n).collect()` | 462–474 ms | 52–61 ms |
| `Aabb::from_points` | 67–72 ms | 67–72 ms |

`Aabb::from_points` is the control: it walks `mesh.positions`, which
already exists, and allocates nothing — and it does not move. **Every
phase that allocates is 6–9× slower on its first run at that size and
flat thereafter.** 34 817 928 corners is 418 MB of positions, 418 MB of
normals, 139 MB of ids, 139 MB of flags and 139 MB of indices: ~1.25 GB
of freshly mapped pages, paid at fault rate once and at store rate
after.

**So `scene_focused` costs ~0.7 s per hide or focus change on the worst
corpus document at 1e-5, not 5 s** — but the FIRST build at a given
size costs the 4–5 s, and the viewer pays that by construction rather
than by accident: `ViewerApp::sync_scene` holds the previous
`Arc<SceneMesh>` alive for the whole of the new build, because
`self.scene = Arc::new(mesh)` assigns in the `Ok` arm and a refused
build must leave the stale picture on screen. Freeing first is not
available; the peak is two pictures' buffers, by contract.

### What follows — and why this lane changed nothing

The seam question is now a 0.7 s question, not a 5 s one, and the
answer is no longer obviously "a worker". The lever that would actually
move it is **rebuild SCOPE and buffer SIZE, not where the walk runs**:
a focus change alters only `flags` (4 B of the 108 B a triangle emits)
and a hide alters only which parts are emitted, yet either rebuilds
positions, normals and ids for the whole picture. That is a change to
what `SceneMesh` is — an incremental update rather than a rebuild — and
it wants a decision before it wants a diff, which is why the item is
left open with this recorded rather than closed behind a seam.

One contained redundancy found on the way is filed on its own:
`work/view/scene-mesh-carries-an-identity-index-buffer.md`.
