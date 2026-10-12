---
id: periodic-branch-floor-remains-at-the-raw-picks
kind: issue
title: a whole-period branch is still picked by floor at six sites the structural sweep did not name
status: open
opened: 2026-10-12
priority: P2
cost: M
refs: [periodic-branch-floor-remains-at-three-sites, loop-walk-branch-is-an-opaque-floor-atom]
---


Found by `pcert/periodic-branch-structural`'s sweep for the shape (a
`floor` of a periodic gap, through `Real::periodic_branch` or open-coded).
That PR moves the three sites
`periodic-branch-floor-remains-at-three-sites` named onto
`geom_brep::whole_periods`. Six more still pick a branch by `floor`,
four through `periodic_branch` and two open-coded as `⌊x/p + ½⌋`. At
`Sym` each one carries an opaque `floor` atom into whatever decision
reads the result:

- `topo::ring_path::branch` (`t + (mid − t).periodic_branch(τ)·τ`). Its
  doc argues the pick is sound whichever way it falls, because the
  in-span reading decides afterwards. At `Sym` that reading is numeric,
  never a theorem.
- `topo::chord_join`'s ring re-homing (`u_p = raw + (mid −
  raw).periodic_branch(τ)·τ`, in the ring-vertex `read` closure). It
  feeds the crossing decisions. Measured at f64 only in the topo, sweep
  and editor-core suites.
- `topo::chord_join::run_azimuth_images`' sphere-pole strict branch
  (`q.floor() + 1` / `−(−q).floor() − 1`). This one is not a nearest
  branch: it is the unique branch in an open period, chosen by
  orientation, and at the pole the vertex lever is zero, so a
  lever-metered mark cannot decide it. At `Interval` it straddles the
  slit's jump on purpose (`crates/sweep/tests/pole_slit_window.rs`).
  Its structural form needs a meter that is not the joint lever, which
  is a design question.
- `geom_brep::props::curved`'s torus lift (`turns = (at −
  angle).periodic_branch(τ)`), which accumulates rim levels for the
  props integrals. FLUX's ground.
- `topo::shell`'s chart run shift (`τ·⌊(at − mean)/τ + ½⌋`, in the run
  lift that writes `(u + shift)·lever`), open-coded `periodic_branch`.
- `geom_brep::props::curved`'s Gauss–Bonnet fan closure (`turns =
  ⌊(gauss − fan)/4π + ½⌋`), a nearest branch at period 4π, open-coded.
  FLUX's ground.

Not in this class: `topo::boolean::rim_wedge`'s `x − τ·⌊x/τ⌋` is a
reduction into `[0, τ)`, not a branch pick.

None of them has been measured over a parameter box. The fix, where one is
owed, is the one the three named sites took: decide the branch as
structure with `whole_periods`.
