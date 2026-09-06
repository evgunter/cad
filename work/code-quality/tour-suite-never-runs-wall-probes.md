---
id: tour-suite-never-runs-wall-probes
kind: issue
title: demos/tour's test suite never executes the wall-probes loop — the walls gate only through the render walk
status: closed
opened: 2026-08-31
closed: 2026-09-03
github: 1434
track: X
branch: smell/x-tour-probes
refs: [1421]
---

## From GitHub issue 1434

Opened 2026-08-31; 0 comments.

(S-MESH orchestrator) Filed from MESH-2's dual review ([#1421](https://github.com/evgunter/cad/pull/1421)). Proven by execution in two independent lanes: `cd demos/tour && cargo test --release` — the spec-level local acceptance command, and the "demos tour suite" CI row — is fully green at a base where all four Klein wall-7 lottery cells refuse `Triangulation`. The wall-probes loop (`klein::wall_probes` and its siblings) runs only in the binary's render walk (`main.rs::walk_tour`), i.e. the k-lint gate's tour step and the render lanes — a green "demos tour suite" job name sits over unexecuted wall probes, the silent-coverage class's fourth face (after CONFLICTING-no-run, queued-with-zero-jobs, and the skipped-step k-lint rows).

The ask: either the tour test suite gains a row that drives the wall-probe loops (cheap: a `#[test]` that calls the walk's probe pass over the scenes that declare walls), or the coverage split is stated where a reader of the CI row will see it (the row's name currently claims "the tour's own probes"). Not urgent — the k-lint gate does execute the walls — but the local acceptance command being blind to wall regressions cost nothing only because CI's sampled k-lint row happened to draw.

Ground: `demos/` is SMELL Track X's fence (its Python J's); filed for whoever holds X. Line numbers as of #1421's head.

## Home

`work/code-quality/` — the issue names `demos/` as SMELL Track X's fence and files it for whoever holds X; the code-quality register owns Tracks K–X.

## Landed

**Both `wall_probes` loops now execute in `cargo test --release`**, the first of the two options the ask offered, in one PR and two halves:

- `klein.rs` — `#[cfg(test)] mod wall_probes_run_here` calls `wall_probes::<f64>(Tol::witness())`, driving all seven of the bottle's walls (the sharp-band fillet pair, the two boolean walls, the one-body sweep, the hollow ring's STEP export, and wall 7's four retired lottery cells);
- `lily.rs` — the matching test for `lily::wall_probes`.

**In-bin, and that is forced, not preferred:** `demo-tour` is bin-only (no `[lib]`; its modules hang off `main.rs`), so nothing under `demos/tour/tests/` can name `klein::wall_probes` or `lily::wall_probes` at all. A `tests/` file was never an option for either half.

**Nothing new is asserted.** `walls::wall` already panics on both off-nominal outcomes — a DIFFERENT refusal (the frontier moved under the probe) and no refusal at all (the wall is gone and its findings entry must be retired) — and klein's walls 6 and 7 assert their retirements inline. Running the loop IS the check, which is why a missing caller was the whole defect.

**Cost, measured.** The klein half is **0.90 s** in isolation (`cargo test --release --bin demo-tour wall_probes_run_here`, warm build): it rebuilds `bottle::<f64>`, the sharp-band pair, the hollow ring at two scalars, the loop's sweep, a STEP export and four wall-7 tessellations. The in-bin unit suite's own wall clock is unmoved — 34.35 s over 24 tests before, 34.94 s over 25 after — because the tests run in parallel and this one is nowhere near the critical path. Whole-suite `cargo test --release` on a warm build measured 48.9 s before and 43.2 s after, i.e. run-to-run noise swamps the change.

**What this did NOT reach, said so the CI row's name is not read as fully earned.** `teapot.rs:1193` and `:1227` run two more `walls::wall` probes, and they sit INSIDE `teapot::stops()` rather than in a `wall_probes` function — so they still execute only when something builds the stop list, which in the suite nothing does. They are not the loop this issue named and were not its ask, but a reader of "the tour's own probes" would expect them covered. `.github/workflows/` is unowned ground per `plan.md` §"What this partition leaves out", so neither the row's name nor the teapot gap was touched here; both want a row of their own, minted with the fence.
