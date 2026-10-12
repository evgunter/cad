---
id: periodic-branch-floor-remains-at-three-sites
kind: issue
title: Real::periodic_branch, an opaque floor atom over a box, still picks branches at chord_join, chart_region and solid_contain
status: closed
opened: 2026-10-02
closed: 2026-10-12
priority: P2
cost: M
refs: [loop-walk-branch-is-an-opaque-floor-atom, periodic-branch-floor-remains-at-the-raw-picks]
---


Found by PR 3812's dual review (R1 S3, R2 Q4), as a class.

`loop-walk-branch-is-an-opaque-floor-atom` was closed by giving
`topo::pcurves::pin_branch` a literal branch (`geom_brep::whole_periods`,
sign decisions at the half-period marks). The same fold, a `floor` atom
that is opaque to the symbolic tier over a parameter box, still picks a
whole-period branch at three other sites:

- `crates/topo/src/chord_join.rs:2270`:
  `let mut k = (prev - raw).periodic_branch(tau);`
- `crates/topo/src/chart_region.rs:1556`:
  `let k = (window_mid(&uv_a) - window_mid(&uv_b)).periodic_branch(T::tau());`
- `crates/topo/src/boolean/solid_contain.rs:2217`–`2218`: `ku`/`kv`
  from `(pu − entry.0).periodic_branch(tau)` and its `v` twin.

Whether any of these runs over a parameter box and then meets the atom
in a decision (as the walk's did in check 4's fidelity) has not been
measured. A shift taken from any of them that lands a row more than
`MAX_BRANCH_PERIODS` periods out would now refuse at check 4's fidelity
(`BranchOutOfReach`), where it used to certify.

The fix, where a site turns out to run over a box, is the walk's:
decide the branch as structure with `whole_periods`.

## Closed

Closed by `pcert/periodic-branch-structural`. All three sites now decide
their branch as structure with `geom_brep::whole_periods`:

- `solid_contain::torus_chart_joint` meters the marks through the
  channel's rate, as its closure is metered.
- `chart_region::period_fold` levers them at the radius.
- `chord_join::nearest_branch` uses the vertex's lever
  (`pcurves::joint_arm`), off the pole and apex.

Measured first. None of the three is reached over a parameter box in
the topo, sweep, step-import, mesh or editor-core suites. All three are
generic (`T: Decide`), though, so a box reaches them by type, and at
every `Sym` call the floor carried an opaque atom. A row per site at
`Sym<Interval>` over a box shows the decision that reads the branch
deciding as a theorem. A floor-restoring mutant is red on each.

One behaviour moved: a sphere face slit to its pole. At `Interval` its
walk now escalates at the joint after the slit, where the floor carried
the straddle on as a window a period wide
(`crates/sweep/tests/pole_slit_window.rs`). No row reaches
`MAX_BRANCH_PERIODS`. The six other floor branch picks the sweep found
are `periodic-branch-floor-remains-at-the-raw-picks`.

