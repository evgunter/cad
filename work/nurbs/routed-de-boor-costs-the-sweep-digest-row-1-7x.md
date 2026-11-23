---
id: routed-de-boor-costs-the-sweep-digest-row-1-7x
kind: issue
title: The routed quadrature de Boor costs the sweep digest row 1.7x, all of it in between's arithmetic
status: closed
opened: 2026-10-12
priority: P3
cost: M
closed: 2026-10-12
branch: nurbs/between-cost
pr: 4841
---


PR 4807 routed `props::quad`'s de Boor through the blossom. A node inside its span's closure now goes through `blossom::diagonal_in_span` (`crates/geom-core/src/spline/blossom.rs:296`) from `de_boor_on` (`crates/geom-brep/src/props/quad.rs:1062`). Every level of it is `Certification::between` (`crates/geom-core/src/interval/certification.rs:354`), where it used to be the affine `(1 − α)·x + α·y`. The certified result is tighter: the step-export pads fall 1.7–4x. The cost is wall time.

**Fixture:** the sweep thread-count digest row, `the_walk_matches_the_serial_golden_at_four_threads` (`crates/sweep/tests/mass_props_are_thread_count_invariant.rs:582`), at ε 1e-9. Each tree was timed solo, on its own `CARGO_TARGET_DIR`, two runs each:

| profile | main (360cf55e) | PR 4807 head | head, `between` swapped back to affine in the door |
|---|---|---|---|
| debug | 207 / 200 s | 368 / 365 s | — |
| CI opt-level 1 | 38.5 / 37.9 s | 64.0 / 63.2 s | 34.0 s |

**Attribution:**
- All of the ~1.7x is `between`'s arithmetic. With only the step swapped back, the routed head is as fast as main.
- It is not allocation. The door allocates one window copy per node (`blossom.rs:312`). PR 4807's one-door refactor removed the first cut's per-node argument vector and row re-collect.

**Shape is open** to the lane that takes this. Two candidates:
- a fast path in `between` for a point ratio;
- hoisting the convex form's per-level work.

Soundness must not move: `between`'s enclosure claim and its pinned meets (PR 4551's batteries and mutants) hold as they are. A faster form must be bit-equal, or a subset that stays certified. Re-measure on the same row, solo, on isolated targets. (NURBS orchestrator, via PR 4807)

## Closed (NURBS, branch `nurbs/between-cost`)

**Where the time went.** I instrumented the row at ε 1e-9 and profiled
`between` with callgrind.
- **Inputs:** 115,076,450 calls. 99.96% have a point ratio (the knots'
  ratios divide exactly at these nodes), and 36% have point sources
  as well.
- **Cost:** about half of `between`'s 5286 instructions a call went to
  interval `mul`, at 8 corner products a multiply, each with its
  exactness witness. The affine step it replaced was 1445 a call.

**What changed. No output bit moves.**
- **The backend's `Mul`** (`interval-transcendentals/src/arith.rs`):
  - a point operand's corners repeat, so it folds the 2 distinct
    corners (point × interval) or the 1 (point × point) instead of
    all 4;
  - the fold's extremes over the de-duplicated corners are the same
    bits.
- **`Certification::between`:** a certified ratio already inside
  `(0, 1)` skips `clamped_to(0, 1)`, which would hand it back
  unchanged (same bounds, decoration capped at its own).
- **Result:** `between` runs at 2668 instructions a call. Every level
  is still `between`, and its enclosure claim and pinned meets are
  untouched.

**Rows:**
- `interval-transcendentals` `arith::tests::a_point_product_is_the_four_corner_fold`:
  every product over an edge table (signed zeros, subnormals, the 2Prod
  floor, the infinities, two decorations) equals the four-corner fold
  bit for bit.
- `geom-core` `between_exact_fuzz::a_ratio_is_read_as_its_clamp`:
  `between` with a ratio inside, on or outside `[0, 1]` equals
  `between` with that ratio clamped first.
- Mutants, each red: a dropped corner, a point × point product padded
  the wrong way, and a clamp skip that admits a ratio below 0.

**Timings.** The row was run solo at ε 1e-9, two runs each, on one box.

| tree | opt-level 1 | debug |
|---|---|---|
| pre-4807 main (affine) | 27.9 / 27.7 s | 158.1 / 159.7 s |
| 4807 head | 47.5 / 47.1 s | 275.5 / 274.3 s |
| this branch | 33.4 / 34.8 s | 177.7 / 176.7 s |

That is 1.70x → 1.22x of the affine cost at opt-level 1, and
1.73x → 1.12x in debug. The rest is `between`'s own operation count:
two forms per corner, met.

The `Div` sibling, which folds four corner quotients for a point
operand as well, is filed:
`work/nurbs/interval-division-folds-duplicate-corners-for-a-point-operand.md`.
