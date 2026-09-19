---
id: the-fold-left-thirteen-one-line-fixture-wrappers-still-copied
kind: issue
title: The box fold turned ten-line copies into one-line copies: six fixtures still spelled once per suite
status: open
opened: 2026-09-19
---

## Finding

- **Where**: `crates/sweep/tests/` — six fixtures, fourteen declaration
  sites, listed below.
- **Importance**: low-medium
- **Confidence**: sure; measured on the folding branch's own diff
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19,
  reading its own diff for X4 before handing back

`private-extruded-box-builders-outside-the-brick-door` folded 33
private box builders onto `sweep::test_support::{brick, block, cube}`.
Each builder's BODY is now one line. What the fold did not remove is
the **naming**: where two suites wanted the same box, they still each
declare a wrapper for it, and the wrappers are now identical one-liners
rather than identical ten-liners.

| body | declared as |
| --- | --- |
| `block(4.0, 4.0, 1.0, …)` | `m5_pr9_boss_union::plate`, `m9_3_wall_door::plate`, `m5_s13_pips::slab`, `m5_s13_pips_interval::slab` |
| `brick((0.0, 6.0), (0.0, 4.0), (z0, z0 + 1.0), …)` | `curved_mergedoor::plate6`, `r1_probes_m9_3::plate6`, `m9_3_zip::plate` |
| `brick((0.9, 1.1), (1.25, 1.35), (0.3, 0.7), …)` | `m5_s10_face_sense::pellet`, `m5_s11_concave_sense_interval::pellet` |
| `brick((cx - h, cx + h), (-h, h), (z0, z0 + 0.4), …)` | `n3r1_prune::small_box`, `s16_box_soundness::small_box` |
| `brick((-0.9, x_max), (-0.15, 0.15), (-0.1, 0.1), …)` | `n3r1_prune::rim_plate`, `s16_box_soundness::rim_plate` |
| `brick((-0.15, 0.15), (y_min, 0.9), (0.9, 1.1), …)` | `n3r1_prune::top_rim_plate`, `s16_box_soundness::top_rim_plate` |

**The wrappers themselves are not the defect.** A name that says what
the box is FOR in a suite is what `sweep::test_support::block` and
`cube` are, with a stated reason; deleting `pellet` for
`brick((0.9, 1.1), …)` at each call site would cost the suite its
vocabulary. The defect is that ONE fixture is named in two or four
places, which is the same finding one layer up.

**Why it was not fixed in the fold.** `crates/sweep/src/test_support.rs`'s
header sets the rule — *"A fixture only earns a place here once a
consumer OUTSIDE this crate needs it or a second suite inside it does;
the narrower homes, and the rule that routes between them, are stated
in `sweep`'s own `tests/common` module"* — so each of these six is
eligible, and each needs the routing question answered (`tests/common`
or `src/test_support`) rather than assumed. That is a vocabulary
decision across four more suites, and it is the unit after this one.

The last three rows are also inside
`n3r1-prune-and-s16-box-soundness-hold-one-corpus-twice`, which is the
larger fact about that pair: seven fixtures, not three.

