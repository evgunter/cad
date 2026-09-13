# PERF-10 — the pick index serves a patch whole: table, boxes and tree under one key

**Status: ratified at dispatch (PERF orchestrator, 2026-09-13).** Binds
the implementer of unit `PERF-10`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is `work/perf/pick-index-triangle-table-rebuilt-every-build.md`.

## 0. The finding this executes

After PERF-9, `MeshPick::assemble` (`crates/editor-core/src/resolve/pick.rs`)
still walks every triangle of every patch on every build: it copies
the three corners into `PickTri` (72 bytes a triangle) and recomputes
each triangle's `Aabb`, whether the patch's tree is then served from
`PickMemo`'s tree level or built; the memo saves `Bvh::build` only.
On the tour die's one-pip edit (85 of 89 patches hit) `MeshPick::
build_with` is 36 ms against `build`'s 62 — the table copy, the box
recompute and four missed trees — and that is now the largest term of
the memo-primed index (34 ms total; tessellation 15). Every other
term of the edit path has been made proportional to what changed;
this one is still proportional to the picture.

## 1. What this unit delivers

**A `PickPatch` served whole.** The memo's tree level becomes a patch
level: under the patch's content key it holds the tree, the pick
table and the boxes together, so a hit is O(1) per patch and the only
per-build work is the top-level tree over `#patches` hull boxes plus
the missed patches. Two shapes are admissible; choose by clarity and
say why:

- the entry keeps the boxes and the hit still proves itself by
  `Bvh::is_over` against boxes recomputed from the placed mesh — but
  then the recompute is the cost you are removing, so this shape only
  works if the boxes can be proven equal without recomputing them
  (they cannot, unless the key already proves the positions — see
  the next shape);
- the hit is trusted under the patch memo's own byte-compared key
  (PERF-5's `FaceInputs`, which covers the shared-prefix chord
  positions the boxes depend on), threaded through to the index seam
  as the fact "this patch's placed geometry is bit-identical to the
  stored one". State at the type exactly which inputs the key covers
  and why they determine every corner the table holds; PERF-9's
  reviewer established that the key covers the chord positions —
  cite the site, do not re-derive.

`mesh` stays free of `bvh`; `PickTri` stays where it is. Level 1
(`NodePick` reuse) and the top-level tree are unchanged. If the
stored corners are the mesh's positions again (three points a
triangle), measure the memo's bytes against PERF-5's and PERF-9's and
state the marginal cost honestly (PERF-9's reviewers caught a figure
that counted shared `Arc`s as new bytes — do not repeat it).

## 2. The pin

- **Pick identity**: PERF-9's `crates/viewer/tests/index_memo.rs`
  differential (the single-level reference, hit-for-hit after every
  landing, the tie-break row) unchanged and green; the tree-for-tree
  row extended to table-for-table (corners and boxes) — a served
  `PickPatch` equal to a fresh build's by bits after every landing of
  the corpus and gallery documents and the worker row.
- **The key proves the corners**: a row that moves a shared vertex
  whose patch key does NOT change must not exist — if you find one,
  it is a PERF-5 key defect: report it and stop. And a row that moves
  one under a changed key asserts a miss and a fresh table.
- **Memo behaviour**: hits equal the patch memo's hits (PERF-5's
  floors extended); generational eviction; a `(δ, tol)` change misses
  everything (the `tol` half is CI's per-ε rows, as PERF-9 recorded).
- `crates/bvh` unchanged unless a door there is cleaner than in
  `pick.rs`.

## 3. Measurement to report

The `perf/explore-gui` harness (fetch, do not merge) on main and on
this branch, release, medians of 3 with spread: `die_composed_tour`
one-pip edit and `die` (index build: tessellate / BVH / rest), the
rings' one-parameter edits and first opens, `kitchen_sink`; the memo's
bytes. Expect the tour die's memo-primed index to drop from ~34 ms to
the tessellation-plus-top-level figure (~16 ms) with the missed
patches as the residual; first opens must not regress.

## 4. Out of fence

The patch memo's key; the tessellator; the BVH's split rule; the
viewer's scene; `crates/bvh`'s query. GUI/VIEW ground in the index
seam, announced in `work/perf/log.md`.

## 5. Report

≤120 lines: the shape chosen and why the other was not, what the key
proves and where that is stated, the pins, the measurements of §3,
the memo's marginal bytes, deviations, findings outside the fence.
