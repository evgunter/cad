# PERF-9 — the pick index's BVH reuses what did not change: per-face trees under a top-level tree

**Status: ratified at dispatch (PERF orchestrator, 2026-09-12).** Binds
the implementer of unit `PERF-9`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is `work/perf/bvh-is-the-index-after-the-memo.md`.

## 0. The finding this executes

After PERF-5 a recomputed root's tessellation is served per face from
the patch memo, but `MeshPick::build` (`crates/editor-core/src/resolve/
pick.rs`) still flattens every patch's triangles into one box list and
builds one `Bvh` over all of them (`crates/bvh`), so on the tour die's
one-pip edit the BVH is ~85 % of the memo'd index build (148 of 174 ms;
PERF-5's corrected table) and on the ring documents ~28 %. The item
names the shape: a per-face BVH with a top-level tree over the faces'
boxes, the unchanged faces' subtrees reused whole, keyed like the
patches.

## 1. What this unit delivers

- **Two-level index.** `MeshPick` holds one `Bvh` per patch (over that
  patch's triangles, in the patch's emitted order) and one top-level
  `Bvh` over the patches' boxes (in face-arena order). A ray query
  walks the top level to candidate patches in ascending patch order,
  then each patch's tree, and the hit selection — nearest exact hit,
  ties to the lower (patch, triangle) index — is the same function of
  the same exact tests as today, so every pick answer is identical.
  `crates/bvh`'s contract (candidates in ascending input order, a
  subsequence of the arena order, independent of tree shape) is what
  makes the two-level candidate set the same SET; say so at the type.
- **Per-face trees are memoized beside the patches.** A patch's tree
  is a function of the patch's local geometry alone (the `Patch`'s
  vertices and triangles before `place` rebases them — PERF-3's
  `PatchVertex::{Shared, Local}` and the shared prefix's points), so
  it carries the same content key as the patch (PERF-5's
  `FaceInputs` key) and lives in the same memo (`PatchMemo` or a
  sibling in the index seam with the same generational eviction —
  choose the one that keeps `mesh` free of `bvh`: the tree over local
  ids is built where the patch is placed, in `editor-core`'s index
  seam, keyed by the patch's key). A memo hit serves the tree; a miss
  builds it. The top-level tree is rebuilt every time (it is
  `#faces` boxes).
- **Level 1 unchanged**: `PickMemo`'s node-level reuse (PERF-5) still
  serves whole `NodePick`s.
- If the per-patch build profiles as the residual on a first open,
  the per-patch trees may be built as an indexed parallel map (D9
  idiom 1; `editor-core` already has `rayon`) — state it as such and
  pin bit identity of the trees at 1 and 4 threads; the top-level
  build stays serial.

## 2. The pin

- **Pick identity.** PERF-5's `crates/viewer/tests/index_memo.rs`
  differential (fixed rays per document, every landing) is the pin:
  every pick answer identical to a fresh single-level `MeshPick` on
  the same mesh — keep a single-level reference build in the test
  (test-only; not a production twin) and compare hit-for-hit on the
  corpus and gallery documents, plus a row of rays chosen to cross
  patch boundaries and to hit coincident triangles from two patches
  (the tie-break row). Write the reference comparison first against
  main, commit, then change the index.
- **Memo behaviour.** Per-patch trees hit exactly where patches hit
  (PERF-5's hit floors, extended to trees); eviction keeps one
  picture's worth; a `(δ, tol)` change misses everything.
- **`crates/bvh` unchanged** unless a two-level door is cleaner there
  than in `pick.rs` — if you add one, its differential suite
  (`review_*`, the conservative-superset contract in `plan.md` §2.1)
  gains the two-level shape.

## 3. Measurement to report

The `perf/explore-gui` harness (fetch, do not merge) on main then on
this branch, release, medians of 3 with spread: `die_composed_tour`
one-pip edit and `die` (index build, tessellate / BVH / rest), the
ring documents' one-parameter edits and first opens, `kitchen_sink`;
memory of the tree memo beside PERF-5's numbers. Expect the tour
die's edit to drop from ~174 ms toward the tessellate-plus-rest
figure (~26 ms) with the top-level tree as the residual; the rings'
first open should not regress (a two-level query costs one extra
descent — measure `Bvh::ray` on both shapes).

## 4. Out of fence

The patch memo's key; the tessellator (PERF-7 runs beside you — merge
`origin/main` before opening the PR); the BVH's split rule; the
boolean and separation consumers of `bvh`; the viewer's scene. GUI/
VIEW ground in `editor-core`'s index seam and `crates/bvh`, announced
in `work/perf/log.md`.

## 5. Report

≤120 lines: the two-level shape and where the per-patch tree memo
lives, the pick-identity argument and the tie-break row, the memo
pins, the measurements of §3, deviations, findings outside the fence.
