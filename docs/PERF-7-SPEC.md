# PERF-7 — the tessellator's face loop is an indexed parallel map

**Status: ratified at dispatch (PERF orchestrator, 2026-09-12).** Binds
the implementer of unit `PERF-7`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is `work/perf/tessellation-is-serial-per-face.md`.

## 0. The finding this executes

`mesh::tessellate` visits faces one at a time
(`crates/mesh/src/tessellate.rs`, `tessellate_impl`'s `for (fk, face)
in body.faces()`): each face's lane (planar, curved, trimmed) reads
the body, the chord pass and the shared prefix of `positions`, and
since PERF-3 returns a `Patch` of LOCAL ids that the sequential
`place` fold rebases in arena order. The lanes are the cost (PERF-5's
tables: `tube_ring` 1.7 s of 2.4 s; `gallery_ring` 0.3 of 0.44 s; a
first open of any ring document), and nothing they read is written by
another face — except one thing, named in §2. D9's addendum ratifies
exactly this shape ("Per-face tessellation — the cheapest, and the
blocker is small", `work/perf/plan.md` §2.2): idiom 1 over faces, then
the arena-order fold that already exists.

## 1. What this unit delivers

`tessellate_impl`'s face loop becomes **D9 idiom 1**: the per-face
lane runs as an indexed parallel map into a pre-sized buffer (one slot
per face in arena order), and `place` stays the sequential arena-order
fold (idiom 2, unchanged). Every mesh is bit-identical to today's at
every thread count — the goldens (`crates/mesh/tests/d9_mesh_goldens.rs`),
the tess-budget baseline, the render lanes and every viewer test are
the pin, and the unit adds a row that runs the goldens under an
explicit 1-thread and a 4-thread `rayon::ThreadPoolBuilder` pool and
asserts the digests equal.

`mesh` gains `rayon` (workspace dependency; today `editor-core` alone
names it — say so in `mesh`'s manifest comment and in `plan.md` §2.2's
state line, which the orchestrator will update from your report). One
spelling of the loop: the map with a pool of one thread IS the serial
path; do not keep a serial twin (`reviewer-style-lane.md` Q1).

## 2. The one shared mutable, and the memo

- `FaceBounds` (`let mut bounds = FaceBounds::new()`, written by
  `compute_chords` and handed `&mut` to the trimmed lane per face).
  State at the type what it holds per edge and what per face; the
  chord pass's writes complete before the map, and each face's lane
  must own its own face's entry (a per-face value returned in the
  slot, or a read-only view of the edge part plus a per-face part) —
  never a lock around the lane. If the split shows a lane reading
  another face's entry, that is a finding: report it and stop.
- `PatchMemo` (PERF-5): lookups by key are pure and may run inside the
  map; INSERTS are a mutation — collect each face's `(inputs, patch,
  key)` in its slot and do the memo's `face`/insert step in the
  arena-order fold. The memo's counters (`hits`, `misses`, the
  picture's `keys`) must be the same numbers as today, in the same
  order (they are order-dependent, disclosed in PERF-5).
- Errors: today the loop stops at the first failing face in arena
  order. The map computes every face and the fold reports the FIRST
  `Err` in arena order — the same error, more work on the failure
  path; say so at the site.
- The chord pass (`compute_chords`, per edge) stays serial in this
  unit unless your measurement shows it is the residual after the
  map; if you take it, it is idiom 1 per edge with the same pins.

## 3. The pin and the measurement

- **Bit identity at any thread count**: the goldens row at 1 and 4
  threads (above); `tess_budget_sweep.sh`'s baseline unchanged;
  `crates/viewer`'s index_memo differential (PERF-5) unchanged.
- **Measurement** (release, 4 vCPU, under the build slot, medians of
  3 with spread): `benches/` `tessellate/*` rows at
  `RAYON_NUM_THREADS=1` and `=4`; `tube_ring`, `gallery_ring`,
  `hollowring` and the tour die through `mesh::tessellate` at both;
  the `perf/explore-gui` harness's first-open index build on the ring
  documents (fetch that branch, do not merge). Report the speedup and
  the 1-thread cost of the map itself (it must be within noise of
  today's serial loop — a regression at one thread is a finding).
- `benches/` runs with the bench profile; add the thread count to the
  criterion row's id or note so the two are not compared as one.

## 4. Out of fence

The lanes' algorithms; `place`/`rebase`; the memo's key; the chord
pass unless §2's clause; `bvh` and the pick index (PERF-9); the viewer.
MESH territory (`crates/mesh/*`), announced in `work/perf/log.md`.

## 5. Report

≤120 lines: the map's shape and the `FaceBounds` split, the memo
insert's move to the fold, the thread-count pin, the measurements of
§3 at 1 and 4 threads, deviations, findings outside the fence.
