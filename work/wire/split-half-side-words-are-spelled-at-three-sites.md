---
id: split-half-side-words-are-spelled-at-three-sites
kind: issue
title: SplitHalf/PlaneSide side words are spelled at three sites (topo, editor-core eval, viewer tree)
status: open
opened: 2026-10-02
---

Found by the review of PR 3797 (TQUERY). The words for a split's two
sides are written out at three sites:
- `topo::PlaneSide::word` (`crates/topo/src/splitting/mod.rs`, added
  by PR 3797 for split's refusals);
- `editor-core`'s `SplitHalf` match in `eval/mod.rs` (the
  `SplitHalf::Above => "above"` arm in a node error's `Display`);
- `viewer::tree::split_half_label` (`crates/viewer/src/tree.rs`), which
  is CHROME/DOORS ground.

The two `SplitHalf` sites could share one `SplitHalf` word in
editor-core. Whether they also read through `PlaneSide::word` depends
on whether `SplitHalf` maps onto `PlaneSide` (it has no `On`).
