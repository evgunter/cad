---
id: checked-is-blind-to-a-wrong-arc-witness-by-a-node
kind: issue
title: The cone section oracle's checked skips witnesses near a node, so a wrong-arc witness there passes; the apex margin reads the foot-based axis without its slack
status: open
opened: 2026-11-22
priority: P3
cost: E
---

From PR 7357's re-verification review, on `3335b164d8`.

- **`checked` is blind near a node.** `crates/geom-brep/tests/cone_pair_section.rs` (the per-stretch test in `checked`) skips any witness within `radius + link` (0.1) of a node. The reviewer reverted `Sweeper::walk` to its pre-fix body and found that the misplaced witnesses sit 0.02–0.04 from the node, so `checked` passed at every seam that had been bad. Only the seam row's ray-toward-witness test (`a_crossing_reads_the_same_wherever_the_cones_seam_stands`) goes red. Fix: read a witness near a node against its ray's side, or pick each arc's witness away from its ends.
- **The apex margin omits the foot slack.** `partner_residual(apex)` (`crates/geom-brep/src/pair_section/cones.rs`, about line 350) reads the axis through the foot `o + d·(d·(apex−o))` without `Form::slack`. The band absorbs about 1e-11 today. Charge it as `p₁`/`p₀` do.
- Note: the slack is not carried into `δ′` either, an `O(slack/|v×d|)` term.
