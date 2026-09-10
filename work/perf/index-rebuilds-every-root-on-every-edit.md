---
id: index-rebuilds-every-root-on-every-edit
kind: issue
title: the pick index re-tessellates every root on every edit, whether one face moved or all of them
status: open
opened: 2026-09-10
---

## The finding

Measured by the PERF GUI lane (`perf/explore-gui`,
`crates/viewer/examples/perf_gui_stages.rs`; release profile, 4-vCPU
box, medians of 3). On every document whose edit lags, evaluation is
noise and the pick index is 83–97 % of the edit→picture wait:

| document | triangles | eval (memo) | index build | edit→picture |
|---|---:|---:|---:|---:|
| `die_composed_tour` | 974 526 | 88 ms | 1231 ms | 1375 ms |
| `gallery_ring` | 995 348 | 0.3 ms | 1517 ms | 1566 ms |
| `tube_ring` | 1 002 528 | 0.1 ms | 2130 ms | 2192 ms |

`PickCache::sync` keys the index on `(generation, δ)`
(`crates/viewer/src/pickcache.rs:266,285`); every committed edit bumps
the generation, so `PickIndex::build` re-walks every root
(`crates/viewer/src/pickindex.rs:742`) and `NodePick::build` calls
`mesh::tessellate` on each body from scratch
(`crates/editor-core/src/resolve/pick.rs:354`). The cost is identical
whether one face moved or all of them (`die_composed_tour`: one pip's
depth re-tessellates 974 526 triangles). The split is ≈55/45 between
the tessellator and the triangle BVH plus id map (the BVH half is a
subtraction from a separately timed whole-product tessellation, not a
direct reading — time `MeshPick::build` and `IdMap::build` directly
before acting on that split).

Interaction consequence: `PickCache::sync` clears the index before
submitting (`pickcache.rs:302`), so for the whole build every pick is
refused typed while the old picture stays on screen.

## What a fix is

"Stop doing this": re-tessellate only the faces whose geometry changed.
The tessellator is already per-face (walk → CDT → certify) and D9 makes
"same bits ⇒ same mesh patch" a theorem, so the memo key is the bit
content of a face's geometry and the memo lives beside the index the
worker already holds. `work/perf/plan.md` §2.1 names this the biggest
preview-lane win. The clarity cost is real — a cache with a lifetime
and an invalidation story where there is a pure function today — and
the spec has to say where the cache lives and what evicts it.

Upstream of it and cheaper: `work/mesh/torus-grid-step-one-step-both-
directions` (≈65× the triangles the chord asks for on torus charts,
which is what makes the ring documents million-triangle documents in
the first place). Fewer triangles before caching any.

Parallelising the per-face loop (`crates/mesh/src/tessellate.rs:106`
threads `&mut positions`; D9 idiom 1 with arena-order id assignment)
is the "do this faster" complement, up to ~4× on this box, and neither
`mesh` nor `bvh` names rayon today.
