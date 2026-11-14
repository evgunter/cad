---
id: a-slow-rows-eps-pins-run-nowhere-on-the-pr-that-adds-them
kind: issue
title: A slow row's ε = 1e-6 and 1e-12 assertions run nowhere on the PR that adds it: the slow-set step runs at the default ε only, and the ε rows skip the slow set
status: open
opened: 2026-11-14
priority: P3
cost: E
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
