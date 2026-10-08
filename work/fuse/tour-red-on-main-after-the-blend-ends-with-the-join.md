---
id: tour-red-on-main-after-the-blend-ends-with-the-join
kind: issue
title: demos/tour is red on main after PR 4353: the die and the teapot document's face, edge and vertex pins still read the blend's pre-join counts
status: open
priority: P1
cost: E
opened: 2026-10-08
---


Found by INTENT stage 4 A's PR (#4364), whose `demos (tour + wild)` job went red after merging main; reproduced on bare `origin/main` `83c1b0f6` (`cd demos/tour && cargo nextest run --release`). Main's push CI does not run the demos, so main shows green.

Four rows fail, from two pins:

- `demos/tour/src/diefillet.rs` (the die stop), `assert_eq!((cf, ce, cv), (26 + 21 * 3, 48 + 21 * 7, 24 + 21 * 5))`: the composed die reads `(89, 174, 108)` against the pinned `(89, 195, 129)`. That fails the tour binary itself, so `eps_regression::tour_runs_green_at_default_eps`, `…_at_eps_1e_6` and `…_at_eps_1e_12` all go red.
- `demos/tour/tests/teapot_document.rs`, `one_request_builds_the_kernels_body`: `census(&kernel)` is `(12, 21, 11)` against `(14, 23, 11)`.

PR 4353 (FUSE step 3 C, "blend surgery ends with the join") is the change that moves them: both are blend results, and the join now runs at the end of the blend. Whether the new counts are right (the join removing valence-2 vertices and their edges, and merging faces at the teapot) is FUSE's call; if they are, re-baseline the pins and say what moved.
