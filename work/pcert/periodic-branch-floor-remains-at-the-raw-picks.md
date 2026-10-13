---
id: periodic-branch-floor-remains-at-the-raw-picks
kind: issue
title: a whole-period branch is still picked by floor at fourteen sites the structural sweep did not name, and reduce_periodic carries the same atom
status: dispatched
branch: pcert/periodic-branch-raw-picks
opened: 2026-10-12
priority: P2
cost: M
refs: [periodic-branch-floor-remains-at-three-sites, loop-walk-branch-is-an-opaque-floor-atom]
---


Found by `pcert/periodic-branch-structural`'s sweep for the shape (a
`floor` of a periodic gap, through `Real::periodic_branch` or open-coded).
That PR moves the three sites
`periodic-branch-floor-remains-at-three-sites` named onto
`geom_brep::whole_periods`. Fourteen more still pick a branch by
`floor`: four through `periodic_branch`, two open-coded as `⌊x/p + ½⌋`,
and eight through `Real::reduce_periodic_centred`
(`self − p·self.periodic_branch(p)`, the same floor behind a wrapper,
which the first sweep's pattern could not match; PR 4875's review,
MINOR-4). At `Sym` each one carries an opaque `floor` atom into whatever
decision reads the result.

Through `periodic_branch` or open-coded:


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

Through `reduce_periodic_centred` (each a nearest branch about a
reference, `ref + (x − ref).reduce_periodic_centred(τ)`, or the centred
residue itself):

- `topo::splitting::classify`, two candidate re-centrings (the
  `centred` reads beside each `anchored` one).
- `topo::offset_axial`'s re-read (`old + (read − old)…`).
- `topo::boolean::circle_torus`'s root re-centring (`thetas[count]`).
- `topo::boolean::circle_roots`, the bracket root (`root = mid + …`) and
  the `near_mid` closure.
- `geom_brep::pcurve_cache::projected`'s centred angle residue
  (`(angle − shift).reduce_periodic_centred(τ).abs()`).
- `profile::sugar`'s signed turn (`((to − from)·turn)…`).

**`reduce_periodic`, the `[0, τ)` reduction, carries the same atom.** It
is `x − p·⌊x/p⌋` (`geom_core::Real::reduce_periodic`), and at `Sym` a
`floor` is opaque unless its argument's form is the zero polynomial
(`geom_core::sym`, `unary_at_zero` / `SymOp::Floor => atom1`). So every
decision reading a `reduce_periodic` result is numeric at `Sym`, never a
theorem. Stated from the tier's construction, not measured. It is a
reduction, not a branch pick, so whether a structural form is owed is
the open question here. Its sites:
- `chord_join`'s arc extent (`(th2 − th1).reduce_periodic(τ)`);
- `splitting::classify`'s two `anchored` reads;
- `shell`'s split and half reads;
- `boolean::boxes`' crest read;
- `profile::sugar`'s `ccw` and extent;
- `topo::boolean::rim_wedge`'s open-coded `x − τ·⌊x/τ⌋` (×4).

None of them has been measured over a parameter box. The fix, where one is
owed, is the one the three named sites took: decide the branch as
structure with `whole_periods`.
