---
id: no-ci-row-runs-topo-under-the-per-op-postcondition-scalpel
kind: issue
title: no CI row builds topo with per-op-postcondition or --all-features, so a row the scalpel flips reads green
status: open
opened: 2026-10-01
priority: P3
cost: E
---


Filed by the TOPO lane that closed
`kev-describing-row-fails-under-all-features` (branch
`topo/kev-all-features`).

## The hole

`topo`'s `per-op-postcondition` feature (the scalpel,
`crates/topo/Cargo.toml`) changes what an ungated row observes: it
sweeps after every operator inside a surgery scope too
(`Body::tier1_sweep_is_mine` in `crates/topo/src/surgery.rs`), so a
row that runs an operator on a body it tore on purpose, inside a scope
it drops unswept, sees a panic where the default build sees `Ok`. Two
such rows were red under `cargo nextest run -p topo --all-features
--lib` on `origin/main` d25fcce4a2 and green in every hosted job:
`euler_kill::tests::kev_describing_asks_the_survivors_point_only_where_a_question_needs_it`
and `review_d18::kill_anchors_on_a_few_torn_bodies` (the second on
every seed: one effort-1 run caught 77,803 operator sweeps). Bisected:
`--features per-op-postcondition` alone reds both; `--features probe`
alone reds neither.

The only hosted row that builds the scalpel is `nightly.yml`'s `the
per-op postcondition scalpel (topo)` step, and it runs
`cargo test -p topo --lib --features topo/per-op-postcondition --
surgery::tests::` — one module. Nothing runs the rest of `topo`'s lib
or its `all` suite under it, nor the `sweep` suite the feature's own
doc names (`cargo test -p sweep --features topo/per-op-postcondition`).
`clippy (--all-features)` compiles the arm and runs nothing; no
`nextest` row passes `--all-features` to `topo`.

## Measured (on the closing lane's branch)

| run | lib | `all` |
| --- | --- | --- |
| default | 1096 run, 1096 pass | 764 run, 764 pass |
| `--all-features`, `origin/main` | 1103 run, 2 fail | 772 run, 772 pass |
| `--all-features`, with the fix | 1103 run, 1103 pass | (unchanged) |

The 15 extra rows at `--all-features` are `probe`-gated; the lib's
seven are the shape `work/guard/feature-gated-lib-unit-tests-are-compiled-and-never-run.md`
already holds. Cost of the scalpel build: one more `topo` lib-test
compile; the slowest row under it, `kill_anchors_on_a_few_torn_bodies`,
ran 33–41 s against 16–23 s at default features.

## The shape to give

Widen the nightly scalpel step from `surgery::tests::` to the whole
`topo` lib (and decide whether `--test all` and `-p sweep` join it),
or run `topo` at `--all-features` in one nightly row, which takes the
probe-gated lib rows above with it. A nightly row, not per-PR: the
scalpel is a localisation tool, and the per-PR gate is sized for
latency (`work/ciw/latency-cut.md`).
