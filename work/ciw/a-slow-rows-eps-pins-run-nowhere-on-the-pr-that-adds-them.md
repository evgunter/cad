---
id: a-slow-rows-eps-pins-run-nowhere-on-the-pr-that-adds-them
kind: issue
title: A slow row's ε = 1e-6 and 1e-12 assertions run nowhere on the PR that adds it: the slow-set step runs at the default ε only, and the ε rows skip the slow set
status: open
opened: 2026-11-14
priority: P2
cost: E
refs: [6897, 7057]
---


Found by the delta review of BOXFINISH's PR 6946 (2026-11-14, run
43635938074 on head `34c263a6`).

`crates/geom-brep/tests/far_walls_of_many_arcs.rs` is a row whose typed
pins all live at ε = 1e-12. It costs 0.47 s on the runner, so it is in
the slow set. On its own PR the gate ran it once, in the "slow tests
this diff adds or edits" step, at the default ε. The ε = 1e-6 and
1e-12 step runs under the `ci` nextest profile, which skips the slow
set. So the assertions that are the row's subject ran only on the
reviewer's box before the merge, and first run hosted on the nightly.

`memories/review-and-dependency-policy.md`: a stated coverage gap
blocks when the untested axis is the row's own subject. Here the gate
left that axis untested without saying so.

What closes it: the step that runs a diff's added or edited slow tests
runs them at every ε row the diff's crates are run at; or the gate's
log says which added rows it did not run at which ε.

## Second instance: a slow row merged red at 1e-6 (PR 6897), 2026-11-17

PR 7057's review traced how `ssi::certify::node_end_rows::a_chain_ends_at_a_node_only_where_its_end_is_at_the_node` reached main red at ε = 1e-6. The row is in the slow set (`.config/nextest.toml`'s `ci` filter). Its 1e-6 case was red at PR 6897's merge `dfa288a929`, the PR that added it. The extra-ε step (`.github/workflows/ci.yml`, "test (eps 1e-6, 1e-12 — …)") runs `cargo nextest run --profile ci`, which excludes the slow set, so the red case ran nowhere before the merge. The same holds for `pair_section::wiring::close_rings_carry_counted`, whose pin lives at 1e-6 only. So the gap is wider than the PR that adds a row. Every slow row a diff adds or edits runs on that PR at the default ε only, and a slow row the diff moves without editing it runs at no ε at all.

Raised from P3 to P2 on this evidence: the gap has now let a red row merge onto main, where it stayed red until another lane found it.
