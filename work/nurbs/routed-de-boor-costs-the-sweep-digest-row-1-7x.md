---
id: routed-de-boor-costs-the-sweep-digest-row-1-7x
kind: issue
title: The routed quadrature de Boor costs the sweep digest row 1.7x, all of it in between's arithmetic
status: open
opened: 2026-10-12
priority: P3
cost: M
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
