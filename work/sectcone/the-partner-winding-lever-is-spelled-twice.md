---
id: the-partner-winding-lever-is-spelled-twice
kind: issue
title: The cone pair's partner-winding read and winds.rs are two spellings of one lever, with drifts between them
status: open
opened: 2026-11-23
priority: P3
cost: M
---

From PR 7455's dual review (two Opus reviewers on `7447c5a47f`). These were adjudicated as filed, not fixed there.

- **Two implementations of the winding lever.** `pair_section/ruling.rs` `Sweeper::winds_about` and `pair_section/winds.rs` `winding` read the same argument: a box clear of the centre, a principal difference trusted within `TRUSTED_TURN`, and whole turns. They differ in two ways:
  - *Frame discipline.* `winds_about` reads the stored `u_ref`/`v_ref` raw (`Sweeper::clearance`, `Frame::azimuth`). `winds.rs` clears boxes on `ssi::residual::certified_frame`'s polar factor.
  - *Meet semantics.* `winds_about`'s joint is `V3::meet`, the hull where rounding parts the boxes. `winds.rs`'s `meet` answers `Apart`.
- **The silent hull fallback.** `Iv::meet` returns the two enclosures' hull where rounding parts them, and nothing reports that it happened. Every reader of `meet` in `ruling.rs` inherits this.
- **O(n²) re-decide.** `winds_about` recomputes every box and joint after each single split.
- **Angle margins levered by the ruled cone's lever.** The chord-turn and half-turn margins in `winds_about` and `whole_turns` are levered by `Ruled::lever` (the region's reach from the cone's apex). That is not the loop's distance from the partner's axis, which is what an angle there is read against.
- **A third copy of the branch cut.** `split_piece`'s branch arm repeats `branch_side`'s cut-where-`Δ`-decides ladder. It re-cuts the arc's unrefined `rise`/`fall` cells rather than the cells `branch_side` already narrowed.
- **Two sources for the partner's axis.** `cones::cone_ruled` hands the sweep the partner surface's frame (`about`), while the sweep's form reads `Form::at`/`Form::d` (the apex's foot on a cylinder's axis, carrying `Form::slack`). These are the same line by construction, but not one value.
