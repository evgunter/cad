---
id: parallel-map-costs-a-fixed-price-on-a-cheap-body
kind: issue
title: the per-face parallel map costs a fixed price a sub-millisecond body cannot repay at one thread
status: open
opened: 2026-09-12
---



## The finding

PERF-7 made `mesh::tessellate`'s face loop an indexed parallel map
(`crates/mesh/src/tessellate.rs`, `tessellate_impl`). Measured on the
lane box (4 vCPU, release, under the build slot), the map at **one**
rayon thread is within the criterion lane's ~5-10 % noise floor of the
serial loop it replaced on every body whose tessellation costs more
than about a millisecond — and is measurably SLOWER on bodies below
that:

| body | faces | triangles | serial | map, 1 thread | map, 4 threads |
|---|---:|---:|---:|---:|---:|
| corpus `die` (δ=1e-4, median of 15) | 111 | 348 | 0.47 ms | **0.76 ms** | 0.28 ms |
| `startup_plate` (δ=1e-4, median of 21) | 8 | 160 | 0.31 ms | **0.37 ms** | 0.21 ms |
| `gallery_ring` (δ=1e-4, median of 3) | 6 | 160 260 | 177.6 ms | 173.1 ms | 66.4 ms |
| `tube_ring` (δ=1e-4, median of 3) | 2 | 683 672 | 967.6 ms | 976.1 ms | 511.9 ms |

The cost is the pool round trip plus the map's own per-face slot: a
`par_iter().collect()` issued from outside the pool injects the job and
blocks on a worker, and with a ONE-thread pool there is no worker to
overlap it with. It is already repaid at two threads (`startup_plate`
at 2 threads: 0.30 ms, i.e. the serial figure) and beaten at four, so
nothing a shipped build does on a 4-core box pays it. What pays it is a
caller that has pinned `RAYON_NUM_THREADS=1` — a test runner, a CI row,
or a consumer that wants the kernel off the machine's other cores.

## What a fix is, and why PERF-7 did not take one

The obvious remedy — a serial arm chosen under a face-count or
triangle-count threshold — is **forbidden by the unit's spec**
(`docs/PERF-7-SPEC.md` §1: "One spelling of the loop: the map with a
pool of one thread IS the serial path; do not keep a serial twin") and
by `docs/prompts/reviewer-style-lane.md` Q1. That refusal is right as a
default and the threshold would be another unguarded constant on a
developer box.

What is worth measuring before anything is written:

* whether the cost is the INJECTION (fixed per `tessellate` call) or
  the per-face slot (`FaceWork` carries a `Result<Patch, _>`, and
  `TessellateError` is a wide enum). The two rows above do not separate
  them — 0.29 ms over 111 faces and 0.06 ms over 8 faces fit either
  shape within their spread. A body sweep at fixed triangle count and
  rising face count would;
* whether a caller that already owns a pool (`rayon::ThreadPool::install`
  around the call) pays it at all, which would make this a documentation
  item rather than a code one;
* `scene::fit_delta` is the consumer that feels it most — it is a probe
  that tessellates repeatedly, and its `die` row went 0.50 ms → 0.96 ms
  at one thread while its `tube_ring` row was unchanged.

Raised by PERF-7 (`docs/PERF-7-SPEC.md` §3 calls a one-thread
regression a finding); the numbers above are that unit's own
measurement run.
