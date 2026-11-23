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

## Not what the cell produces on main (GREEN, 2026-11-14)

From PR 6466 (`sectarms/cylinder-node-listing`, merge `c053873aa`) on,
the 3° pose's B ∖ A at ε 1e-12 returns `ResultInvalid { NotFinished {
VolumeUncomputable } }` on `props_chart_junction_closed`, with
`SolidKey(3v1)` present. So the pieces sort ran, and `Pieces(Probe …
RayExhausted)` is not what the cell produces; every other 3° cell refuses
the same way. The row's pin now names that refusal and cites NEARPOINT's
`work/nearpoint/a-cylinder-circle-and-a-chart-junction-escalate-in-band-at-1e-12.md`,
not this row. `bisector_nodes::at_1e_12_the_slanted_bores_refuse_every_cell_and_the_glued_rod_escalates_in_both_orders`
asserts the six cells. Whether the probe ray still exhausts behind the
volume refusal cannot be read until that refusal goes. Whether this row
closes is TANG's call.
