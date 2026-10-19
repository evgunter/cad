---
id: the-probe-gated-assembly-battery-gathers-an-unplaced-fixture
kind: issue
title: assemble_one_local_battery gathers an unplaced fixture and reds under --features probe, which CI never runs
status: closed
opened: 2026-10-10
priority: P2
cost: E
closed: 2026-10-11
branch: intent/probe-battery-places-its-fixture
---


`crates/editor-core/tests/assemble_one_local_battery.rs:55`
(`the_assembly_gate_runs_the_census_over_the_gathers_verdict`, gated on
`feature = "probe"`) gathers `fixture::two_blocks_and_their_union`
(`crates/editor-core/tests/fixture/mod.rs:2179`), which places nothing.
Since stage 2 C an unplaced body is not in the product, so the gather
refuses `EmptyProduct { unplaced }` and the row panics before the
battery comparison it exists for. It is red on main.

Found by stage 5 B while adding the `Probe` impl of
`checks::at_rest::LocalizeLane` (the `--all-features` clippy row
compiles the suite; no CI job runs editor-core's tests with `probe`).

**Fix.** Place the union in the fixture's caller (a `Place` of the
union node, as stage 2 C's re-placed fixtures do), and give the probe
suites a CI row, or roster this one as not executed
(`scripts/gates/probe-suite-census.sh`).

## Closed

The row's caller places the union (`fixture::place` on the node
`two_blocks_and_their_union` returns), so the gather has a product and
the battery comparison runs; the fixture itself is unchanged, and its
other callers (`parallel_node_map_probe`, `parallel_node_map_interval`)
read node maps, not the product. No other probe-gated suite calls
`product_recorded`, `assemble_gathered` or names `EmptyProduct`.

No new CI row and no unrun declaration: the suite is already in the
census's executed set (`RUN_FLOOR`'s `plain:editor-core:assemble_one_local_battery`),
which nightly `k-lint (dev-probe)` runs through `k_probe_sweep.sh`'s
plain pre-pass, and `probe-suite-census.sh` refuses a suite that is
both rostered and declared unrun. The row was red there, not unrun.
