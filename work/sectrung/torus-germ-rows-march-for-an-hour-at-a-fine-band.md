---
id: torus-germ-rows-march-for-an-hour-at-a-fine-band
kind: issue
title: The torus germ rows' marches run past an hour in debug at ε 1e-12
status: open
opened: 2026-10-29
priority: P2
cost: M
---


Filed by SECT's `sect-components` lane (branch `sect/components-by-foot`,
2026-10-29).

## Measured

Since the germ mint carries a torus pair's section (a site's component
is its foot), every union whose germs ride a cylinder × torus or torus ×
torus pair marches the pair at the mint. In debug at ε 1e-12, run
locally on the branch:

- `crates/sweep/tests/germ_torus_components.rs`
  `a_drum_against_a_moved_donut_passes_the_germ_mint` (12 unions) did
  not finish in 3 260 s; `two_donuts_in_general_pose_pass_the_germ_mint`
  (6) took 815 s and `a_ball_round_the_donuts_axis_passes_the_germ_mint`
  (2) 329 s. At the compiled ε the drum row takes 467 s.
- `crates/sweep/tests/germ_tilted_rod.rs`
  `other_spins_and_steeper_tilts_refuse_at_their_germs` (32 runs) did not
  finish in 3 420 s; on main it stopped at the mint without a march. At
  the compiled ε, after the torus ring path landed on main, it takes
  1 560 s, every run now reaching the result gate's volume.

At the nightly's settings (opt-level 2), the dual review of PR 6011
measured at 1e-12: the tilted-rod spins row 1 015–1 770 s, the drum row
400–700 s, the two donuts 520–900 s, against the `full-suite` job's 180
minutes.

The rows are in the slow set, so this is the nightly's cost. The PR 5972
log measured the door rows at 230–865 s each at 1e-12, so most of the
cost is likely the march itself (`ssi::torus_torus_ssi`,
`ssi::cylinder_torus_ssi`) at a fine band. A pair the mint marches is
marched once per operation (`SectionTable::carried`): the germ mint now
marches each held pair it mints a germ on, and the table is built per
operation at the glue door, so no march is shared across operations.
Four ops in two member orders march one pair eight times.

## What a fix has to supply

A measurement of where the march spends its time at 1e-12, and either a
cheaper march or rows sized for the nightly's budget.
