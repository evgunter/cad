---
id: main-red-the-stated-wrong-general-rows-refuse-chart-image-unavailable-in-the-slow-set
kind: issue
title: Main red: the two stated-wrong-general rows refuse ChartImageUnavailable, and only the slow set runs them
status: open
opened: 2026-11-11
priority: P1
cost: E
---


Found by the RESTREAD lanes on PR 6816 (`restread/self-touch-lunes`);
filed by the RESTREAD orchestrator. Neither PR 6816 nor RESTREAD
caused it.

## What

Both rows
`a_bending_carrier_on_a_spline_wall_takes_the_general_lane::a_stated_wrong_general_row_keeps_its_term_{f64,interval}`
(`crates/topo/tests/a_bending_carrier_on_a_spline_wall_takes_the_general_lane.rs`)
fail with `ChartImageUnavailable { chart: Nurbs, carrier: Circle }`:

    cargo nextest run -p topo --test all -E 'test(/a_stated_wrong_general_row_keeps_its_term/)'

They fail run alone, in their own process. They fail identically at
`b125b84b30` and at `4d193f273d` (main), so they are red on main.

## Why the PR gate is green

`df2a4a7693` ("the crossing and stated-row rows join the slow set")
put them in the slow set. CI's gate runs `--profile ci`, whose
default-filter in `.config/nextest.toml` excludes the slow set, so
`cargo nextest list --profile ci -p topo --test all` lists neither row.
Only the default profile, and so the nightly, runs them. The rows were
written as `a-minted-refusals-provenance-is-guarded-only-by-its-doc`'s
witnesses (that row's "Rows:" line), so that guard is currently red,
not held.

## The fix

Decide whether the rows or the code is wrong: the row says a `Stated`
image keeps its term, and the certification ends instead in
`ChartImageUnavailable`. Then make both green in the default profile.
