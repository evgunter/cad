# PERF-5 — the pick index reuses what did not change: node-level and face-level memo

**Status: ratified at dispatch (PERF orchestrator, 2026-09-11).** Binds
the implementer of unit `PERF-5`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is `work/perf/index-rebuilds-every-root-on-every-edit.md`.

## 0. The finding this executes

On every document whose committed edit lags, the pick index is 83–97 %
of the edit→picture wait and is rebuilt from scratch on every edit:
`PickCache::sync` keys the index on `(generation, δ)`
(`crates/viewer/src/pickcache.rs`), every committed edit bumps the
generation, `PickIndex::build` walks every root
(`crates/viewer/src/pickindex.rs:741`) and `NodePick::build`
tessellates each body from scratch and builds its BVH
(`crates/editor-core/src/resolve/pick.rs:354-355`). Measured (release,
4 vCPU, `perf/explore-gui`'s `crates/viewer/examples/perf_gui_stages.rs`):
`die_composed_tour` 1231 ms of index for a one-pip edit that
re-evaluates 5 of 32 nodes; `gallery_ring` 1517 ms; `tube_ring` 2130 ms
— split ≈55/45 between `mesh::tessellate` and the BVH plus id map.
(PERF-1 has since cut the ring documents' triangle counts ~24×; re-take
the baseline on main before you start, with the same harness.)

The evaluation memo (`editor_core::evaluate`'s `prior`) already knows
which nodes were reused (`Evaluation::reused`, the per-node content
keys). The tessellator is per-face and since PERF-3 each face's patch
is a value (`mesh::tessellate::Patch`, `PatchVertex::{Shared, Local}`),
and D9 makes "same bits ⇒ same patch" a theorem. Nothing on the index
path uses either fact.

## 1. What this unit delivers — two levels, both "stop doing this"

**Level 1 — node-level.** A `NodePick` (mesh + BVH + id map) for a
node whose evaluated value the memo REUSED, at the same `(δ, tol)`, is
the same value; the index worker keeps the previous generation's
`NodePick`s and reuses them for reused nodes instead of rebuilding.
This covers every multi-root edit that touches few roots, BVH included,
and costs nothing per face.

**Level 2 — face-level.** Inside a node that was recomputed, most
faces' geometry is bit-identical to the previous body's. `mesh` gains
a `PatchMemo` and a `tessellate_with(body, δ, tol, &mut PatchMemo)`
door: the chord pass runs as today (it is per edge and cheap), then
per face the lane is skipped when the memo holds a patch under the
face's **content key**, and the patch is placed by PERF-3's fold. The
key is the bits the lane reads and nothing else — the face's surface
bytes, for each loop in walk order each edge's carrier bytes, its
orientation, its chord positions AND chord parameters
(`ChordPass::params`; the trimmed lane evaluates pcurves on that
schedule), the pcurve bytes where a lane reads them, δ_s and ε/band.
State the key at the type as the list of inputs each lane reads, and
prove it by the differential in §3 — a key missing an input the lane
reads is the one defect this design can have, and the reviewer will
attack exactly that. `FaceKey` and arena ids are NOT part of the key
(keys are lineage-scoped and a memo across evaluations must not depend
on them); positions and parameters are hashed by their f64 bits.

**Eviction is generational**: at the end of a build, entries not hit
in that build are dropped, so the memo holds exactly one picture's
worth and cannot grow. Both memos live where the previous generation
already lives — the index worker (`ThreadIndexer` / `InlineIndexer`
own it; `PickIndex::build` takes it; `NodePick::build_all` threads it
to `tessellate_with`). The inline indexer the tests drive gets the same
memo, so every viewer test exercises the memo path.

## 2. D9 and the hash

A `HashMap` keyed by a content digest is permitted: a LOOKUP by key
influences nothing; only iteration order may not, and nothing iterates
the memo to produce a value (eviction iterates it to drop entries —
order-independent). Use a fixed digest (the crate already uses FNV-1a
in `crates/mesh/tests/d9_mesh_goldens.rs`; a 128-bit content digest
with the structure folded in, not `std`'s `RandomState`). A digest
collision is a wrong mesh, so the value stored beside the digest
carries the key's full bytes (or enough of them) and a hit compares
them — a false hit is impossible by construction, not improbable.

## 3. The pin

**Bit identity across edits.** A row in `crates/viewer/tests/` (the
inline seam) drives, for each corpus document that has at least one
parameter and for the gallery documents: open → land → index; then a
sequence of edits (change a parameter, change another, revert the
first) → land → index; and after every landing asserts that the memo-
assisted `PickIndex`'s meshes are byte-identical to a fresh
`mesh::tessellate` of each root body (digest as the goldens row does)
and that its `MeshPick` answers the same picks (a fixed set of rays
per document). Write it first against main, commit, then build the
memo under it. Also assert the memo's size after each build equals
the picture's face count (eviction), and that a `(δ, tol)` change
misses everything.

**The key is complete.** For each lane (planar, curved per
`ChartKind`, trimmed on an analytic carrier, trimmed NURBS, `Approx`),
a row that mutates ONE input the lane reads — a surface parameter, a
chord position, a chord parameter, a pcurve, ε — and asserts a miss;
and one that mutates something the lane does NOT read (a face key, an
unrelated face) and asserts a hit.

**Benches and goldens.** `d9_mesh_goldens` unchanged (the plain
`tessellate` door is untouched); `benches/` rows unmoved (no memo on
that path).

## 4. Measurement to report

The GUI harness (`perf/explore-gui`, `perf_gui_stages.rs`; fetch, do
not merge) on main-with-PERF-1 as the baseline, then on this branch:
`die_composed_tour` one-pip edit, `gallery_ring` and `tube_ring` one-
parameter edits, `die`; report index build, its tessellate/BVH split,
and edit→picture, medians of 3 with spread, release. Expect level 1 to
make a multi-root edit's index cost proportional to the roots touched
and level 2 to make a single-body edit's tessellation cost
proportional to the faces touched; the BVH of a recomputed root is
still rebuilt whole — say what fraction remains and file it
(`work/perf/`, one item) if it is now the largest term.

## 5. Out of fence

The BVH build itself (`MeshPick::build`, `crates/bvh`); the evaluation
memo; `scene_focused` and the display budget's probe (a separate unit);
parallelising the per-face loop (PERF-6/7 territory, D9 idiom 1 — this
unit must not make it harder: the memo lookup is per face and pure,
the placement is the fold). The lanes' three argument shapes and the
trimmed lane's `&mut FaceBounds`: if unifying the lane inputs into one
`FaceInputs` value is what makes the key honest, do it here and say
so — it is in scope when it serves the key, out when it is only tidier.

## 6. Report

≤150 lines: where each memo lives and its lifetime, the key's input
list per lane, the differential rows and their coverage, the
measurements of §4 with the BVH residual, deviations, and findings
outside the fence.
