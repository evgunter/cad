---
id: parallel-map-costs-a-fixed-price-on-a-cheap-body
kind: issue
title: the per-face parallel map costs a fixed price a cheap body cannot repay, and one more per face on the memo path
status: open
opened: 2026-09-12
priority: P2
cost: D
---


## The finding

PERF-7 made `mesh::tessellate`'s face loop an indexed parallel map
(`crates/mesh/src/tessellate.rs`, `tessellate_impl`). It pays for
itself on every body that costs more than about a millisecond to
tessellate and does not on the ones below that. Measured on the lane
box (4 vCPU, release, under the build slot; medians of three
interleaved rounds of 41 and 201 reps, round-to-round spread under
±10 % on every row), serial = `origin/main` built in its own worktree:

| body | faces | triangles | serial | map @1 | map @4 |
|---|---:|---:|---:|---:|---:|
| n-gon prism, δ=0.5 | 6 | 12 | 0.017 ms | 0.062 ms | 0.153 ms |
| n-gon prism, δ=0.5 | 18 | 60 | 0.105 ms | 0.168 ms | 0.129 ms |
| n-gon prism, δ=0.5 | 66 | 252 | 0.557 ms | 0.701 ms | 0.556 ms |
| n-gon prism, δ=0.5 | 258 | 1020 | 2.461 ms | 2.540 ms | 2.084 ms |
| corpus `die`, `tessellate_with`, memo primed (111 hits, 0 misses) | 111 | 348 | 0.421 ms | 0.751 ms | 0.370 ms |

Two costs, and they are not the same cost:

- **A fixed price per call**, on the memo-free path: about **0.05 ms**,
  and no per-face term resolvable above the rows' own spread — the
  6-face prism pays +0.044 ms and the 258-face prism +0.079 ms, which
  is the same number twice. It is the pool round trip: an indexed
  `par_iter` collected by a caller that is not a pool worker injects
  the job and parks on a latch. At four threads a body this cheap pays
  it twice over (the 6-face prism is **9× slower** than serial), and
  break-even against serial is around 60 faces / 0.5 ms.
- **About 3 µs per face on the MEMO-HIT path**: the die's 111 hits cost
  +0.330 ms at one thread, which nearly doubles that pass. This is the
  one a preview-lane caller feels, because a memo-primed re-index is
  exactly what an edit produces.

Both are repaid at four threads on everything above the break-even, and
the `die`'s memo-primed row is 0.370 ms against 0.421 ms serial — so
the shipped width is not where the problem is. What pays these prices
is a caller pinned to one thread, and any caller at all on a body below
break-even.

**The same price, one door over — measured, and it settled a grain**
(PERF-11, same box, release, under the slot). `topo::classify_shells`'
per-shell face walk was made the same indexed map and the map was then
REVERTED, because the census meets no body that repays it:

| document | solids / shells | faces | serial | map @1t | map @4t |
|---|---:|---:|---:|---:|---:|
| heat_sink, 40 fins | 41 / 41 | 271 | 0.10 ms | 0.1 ms | 0.3 ms |
| heat_sink, 160 fins | 161 / 161 | 991 | 0.37 ms | 0.3 ms | 1.0 ms |

Three times slower at four threads on the heat sink, about 4.6× on a
two-shell voided brick and 2× on a hollow box (the two review arms
re-measured both), and a gain on **no** body in the corpus — every
shell the census meets has a handful of faces, and no corpus shell is
many-faced on the quadrature lane, which is the only thing that would
carry the price. The cost is the finding above with a second subject,
paid **once per shell** because the map is over a shell's faces while
the loop over shells is sequential.

**What that adds to this item**: on a document whose shells are cheap
and NUMEROUS, the grain that could repay the price is the shell, not
the face — and on the heat sink the shell grain measures at about
break-even, so it is a question and not an answer. Changing it is not
this item and not PERF-11's: `classify_shells_of`'s outer loop is the
census's, whose pair sweeps are PERF-12's ground. The face grain is
settled (serial), with the reason stated at
`topo::props`' `decide_faces_serially`, so it does not want re-trying.

## What was tried, and did not work

The review's diagnosis was that the per-face term is the memo hit's own
allocations: `FaceLookup` building a `Patch` out of the stored entry on
a worker for the fold to consume and free on the caller. **That was
fixed and it did not move the number.** `PatchMemo::lookup` now hands
the fold the `&StoredPatch` itself and `StoredPatch::place` renames it
straight into the mesh arena, so a hit allocates no patch at all and
the triangles are walked once instead of twice. Before and after, three
interleaved rounds, the die's all-hits `tessellate_with`:

| | @1 | @4 |
|---|---:|---:|
| before (`c26d3220f`) | 0.671 ms | 0.352 ms |
| after | 0.701 ms | 0.356 ms |

Within the row's spread, which is ±0.05 ms. The change stays because it
is strictly less work and one fewer intermediate representation, **not
because it bought anything measurable** — and the per-face cost it was
supposed to explain is still there.

## What is left to try, in the order the measurements point

1. **`par_iter().with_min_len(k)`.** The fixed price and the per-face
   price are both consistent with rayon's split-and-join bookkeeping
   rather than with any allocation: the map splits to single faces, so
   a 111-face body pays ~222 joins. `with_min_len` bounds that without
   a second spelling of the loop — it is still one indexed map,
   combined positionally, bit-identical — which is what makes it
   different in kind from the arm ruled out below. It needs a measured
   `k` and an honest note that `k` is a reading on one box.
2. **The per-face work the memo path does before the lookup.**
   `FaceInputs::gather` walks every loop and every edge and clones
   carriers, positions and chord parameters into a fresh `FaceInputs`,
   and `key()` serialises all of it; that is a dozen allocations per
   face on a worker, against the two a restored patch cost. If (1) does
   not explain the 3 µs, this is where to look next.
3. **Nothing on the placement side.** It has been measured and it is
   not the cost.

**A serial arm under a face-count or triangle-count threshold is
ruled out** (`docs/PERF-7-SPEC.md` §1, `docs/prompts/reviewer-style-lane.md`
Q1): two spellings of one loop, and the threshold would be an unguarded
constant on a developer box. `crates/mesh/tests/budget_meter.rs`'s
`no_face_lane_runs_on_the_thread_that_called_tessellate` is the standing
row that turns red if one appears.

Raised by PERF-7 (`docs/PERF-7-SPEC.md` §3 calls a one-thread
regression a finding) and re-measured against both reviews of PR 2448.
