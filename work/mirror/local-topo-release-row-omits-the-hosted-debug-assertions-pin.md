---
id: local-topo-release-row-omits-the-hosted-debug-assertions-pin
kind: issue
title: ci-local.sh's topo_release omits CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false and review_d18, so 'corrupt input (release profile)' can never pass locally
status: open
opened: 2026-09-26
priority: P2
cost: E
---


(GATHER orchestrator, 2026-09-26.) Found running the local battery that
Ev authorized as the merge gate while the hosted queue is deep.

**The drift.** Hosted's `corrupt input (release profile)` job
(`.github/workflows/ci.yml`, about lines 3395-3415) pins
`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS: "false"` and filters
`review_m1_pr2::release_corruption`,
`review_m1_pr4::kill_ops_survive_torn_bodies_without_panicking` and
`review_d18`. Its comment explains why: `[profile.release]` in
`Cargo.toml` has `debug-assertions = true`, and the rows this job exists
for are `#[cfg(not(debug_assertions))]`. The local half's `topo_release`
(`local-scripts/ci-local.sh`, about lines 621-645) sets neither the pin
nor `review_d18`. So `garbage_in_garbage_out_release` is compiled out,
and the row's own grep reds on every tree:
`ERROR: garbage_in_garbage_out_release did not run` (seen on PR 3264's
battery at `b9cf830e0`, 6 passed). Run as hosted runs it, the row gives
20 passed, all three named rows ok.

**Why the parity gate missed it.** `check-ci-mirror-parity` says it
refuses a semantics-bearing environment variable set on one half of a
mirrored pair and not the other. Either
`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS` is not in its semantic set, or
this pair is not recognized as mirrored. Say which, and close that hole
too. The missing `review_d18` filter is a second one-sided difference
the gate did not name.

**Fix.** Give `topo_release` the same env pin and filter list as the
hosted step, and add its `review_d18` assertion if hosted's has one.
Then make the parity gate fail on this shape.
