---
id: main-red-slanted-bores-b-minus-a-exhausts-its-probe-ray-at-1e-12
kind: issue
title: Main red: bisector_nodes' 3° slanted bores (B ∖ A) exhausts its probe ray at 1e-12
status: open
opened: 2026-10-26
---


**Main red.** Filed by PCERT's E1 lane (`pcert/lane-choices`) while verifying PR #5500's suites. It is red on main `663dd78fef`, re-run at `CAD_TOLERANCE_EPS=1e-12`, and red the same way on E1's head.

`crates/sweep/tests/bisector_nodes.rs` `slanted_bores_build_every_operation_one_difference_pinched_to_itself` fails at 1e-12 only (it is green at the default ε and at 1e-6, on main and on E1):

```
3° (B, A), ∖: the node builds, got Err(Pieces(Probe { shell: ShellKey(2v5), source: Loop(RayExhausted { loop: LoopKey(16v5) }) }))
```

The 3° pose's B ∖ A operation refuses in the shell-pieces probe: every ray cast from the loop is exhausted without a decided crossing. No bisect was run. The row was green at 1e-12 in E1's pre-merge round, on a tree based on main `9039ea9c45` (2026-10-25), so a main change between `9039ea9c45` and `663dd78fef` is the likely cause.

**Done when** the 3° pose builds all its operations at 1e-12, with the probe ray decided rather than the row's pins loosened, and the row is green on main at every ε.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Still red at 1e-12 with the same message. The merge is PR 5664
(`work/charttail/main-red-ringed-curved-doors-reds-four-sweep-rows`). The
row is pinned at 1e-12 on `3° (B, A), ∖: the node builds, got
Err(Pieces(Probe`; it goes red the day the row passes.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured.
