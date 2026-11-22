---
id: the-meet-window-refuses-decidable-meetings-it-could-read
kind: issue
title: The meet window refuses decidable meetings it could read
status: open
opened: 2026-11-21
priority: P2
cost: M
---


Filed by ONEHOME from PR 7277's full review (step 3d). No answer is
wrong; this is a refusal of an answer that can be decided.

**The finding.** 3d lets the pose's basin pass on pairs that main
refused on `curve_pair_pose_basin`. Some of those now refuse later:
- on the window read (`curve_pair_meet_window`, the straddle of the
  window's floor against the pair's gap,
  `crates/geom-brep/src/curve_pair.rs`, `window_off`);
- on the walk's pair gap (`curve_pair_walk_pair_gap`).

The review's dense oracle reads the least distance of each.
- In its shape A, all 48 new `curve_pair_meet_window` and all 14 new
  `curve_pair_walk_pair_gap` refusals have the oracle's least distance
  below `0.9·zero`. They are meetings the band decides.
- Across its shapes, 658 of the 1,355 `curve_pair_meet_window` refusals
  look the same.

None is a regression: main refused each of them on the basin.

**What is owed.** Where the polished pair's gap reads off but a pair of
the window lies within the zero, the window read should find that pair,
by polishing further or reading the window's least distance, rather than
straddling the floor against the polished gap.
