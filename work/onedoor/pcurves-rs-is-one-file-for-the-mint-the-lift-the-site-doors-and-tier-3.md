---
id: pcurves-rs-is-one-file-for-the-mint-the-lift-the-site-doors-and-tier-3
kind: issue
title: pcurves.rs is one eleven-thousand-line file for the mint, the lift, the site doors and tier 3
status: closed
opened: 2026-11-13
priority: P4
cost: M
closed: 2026-11-14
pr: 6939
branch: onedoor/pcurves-split
---

Found by `onedoor/three-walks` (2026-11-13), in the fix pass for PR 6900.

`crates/topo/src/pcurves.rs` is about 11,400 lines and holds five
things a reader has to tell apart to review any one of them:

- the pass's walk and the one joint decision (`walk_cycle`,
  `decide_joint`, `singular_turn`, `half_turn_off`, `apex_nappe`);
- the lift and its readers (`Lift::read`, `Lift::decided`,
  `lift_decks`, `lifted_images`);
- the doors' limbs: the site mint (`site_rows`, `site_walks`), the
  split's carried rows (`split_cache`) and a kill's turn
  (`turn_element`);
- tier 3 (`pcurve_pass`, `validate_pcurves`, `loop_windings`,
  `read_loop`);
- the posture census (`staleness_posture`) and about 4,800 lines of
  rows.

One test module already lives beside it (`pcurves_lift_rows.rs`). The
decision and the lift are what DESIGN `[one-way.places]` names, and a
file of their own would let the guard
`no_file_outside_the_mint_pins_a_chart_branch` state "the one decision"
as a file rather than as a function found by its header line
(`crates/topo/src/pcurves_lift_rows.rs`, the `MAKERS` check).

A move with no design implication; not done in the unit that found it
because its diff would bury that unit's review.


## Closed (PR 6939, 2026-11-14)

`crates/topo/src/pcurves.rs` is now a module directory with five files:
- `mint.rs`;
- `lift.rs`;
- `site.rs`;
- `tier3.rs`;
- `decision.rs`, the joint decision with the whole walk. The row asked for it to stand on its own.

The module doc's "The files" section maps them.

**Proven a pure move.** The reviewer's item extractor maps 374 items onto 380; the extras are a new `OWN` const and the `mod` declarations. The 59 tests run unchanged in the same modules. `pub(crate)` stays at 134 and no `pub` was added. Every widening is `pub(super)`, and each is needed by code in another file.

**Guards.** The MAKERS guard reads the module set through `test_utils::source` and matches `decide_joint` by name; a mutation test pins it. The path-keyed gates point at the new files. `pcurves_rings.rs` stays where it was, behind `#[path]`.

Filed: `chart-facts-are-a-sixth-concern-inside-pcurves-mint` (P4). Model-compare row: `docs/model-compare-log/rows/PR-6939.md`.
