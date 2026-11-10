---
id: the-lemniscate-node-row-exceeds-the-accounting-budget-at-1e-12
kind: issue
title: the lemniscate's node row exceeds the chart accounting's cell budget at ε 1e-12
status: open
priority: P2
cost: M
opened: 2026-11-10
---


Found while verifying NURBS × implicit stage 3b at the nightly's three ε.

## What happens

`a_loop_through_a_node_names_both_its_ends`
(`crates/geom-brep/src/pair_section/spline_implicit/node_rows.rs`) reads
the lemniscate of Gerono, `z = x⁴ − x² + y²` against `z = 0` over
`x ∈ [−1.2, 1.2]`, `y ∈ [−1, 1]`: two loops, each with both ends at the
node at the origin. At ε 1e-9 and 1e-6 it lists both loops, and is the
slowest of the node rows by far (about 28 s release at 1e-9, against 2–5 s
for the cross and oblique saddles). At ε 1e-12 the whole reading refuses
`Unaccounted { cause: "ssi: the exhaustiveness subdivision exceeded its
200000-cell budget — refused rather than truncated" }`: the chart
accounting (`exhaust::account_chart_partner`, called from
`ImplicitSection::all` in `crates/geom-brep/src/ssi/implicit_chart.rs`)
runs out of cells. The row is pinned red at 1e-12 on that refusal.

The cross and oblique saddles, whose arcs are short and reach the sides,
account at 1e-12 once the node box grows to its cap at every ε
(`GROW_ROUNDS` in `pair_section/spline_implicit/sweep/critical.rs`).
The lemniscate's long closed loops and its two enclosed minima are
what differ; which cells the accounting spends its budget on has not
been measured.

## What would close it

Measure where the accounting's cells go at 1e-12 (along the loops'
tubes, about the node box, or about the enclosed minima), and either
cover that ground with what the reading already proves or show the
budget is the limit; then lift the pin.
