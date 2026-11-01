---
id: periodic-branch-floor-remains-at-the-raw-picks
kind: issue
title: a whole-period branch is still picked by floor at fourteen sites the structural sweep did not name, and reduce_periodic carries the same atom
status: closed
branch: pcert/periodic-branch-raw-picks
opened: 2026-10-12
closed: 2026-10-13
priority: P2
cost: M
refs: [periodic-branch-floor-remains-at-three-sites, loop-walk-branch-is-an-opaque-floor-atom, the-period-jump-has-no-ruling, 5375]
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
MINOR-4). At `Sym` each `floor` is an atom wherever its argument's form
is not zero (see Closed for what that does and does not cost).

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

## Closed

Closed by `pcert/periodic-branch-raw-picks`.

**The count is 14 production picks:**
- 13 from this list. `topo::splitting::classify`'s second
  `reduce_periodic_centred` is inside a test (the anchor-widening row
  that re-derives both anchors inline), not production code.
- Plus one the census missed (PR 4941's review):
  `sweep::blend::open::end_face`'s `first = −⌊−ta/π⌋`. Unexamined but it
  looks sound: it is read through `.lo()` and errs to `far = major`.
  BAND's code; noted on BAND's slate as
  `work/band/end-face-vertex-window-picks-its-first-half-turn-by-floor.md`.

**What a `floor` costs at `Sym`:** it folds only where its argument's
form is the zero polynomial (`geom_core::sym`'s `unary_at_zero`). That is
the exact tie of a `⌊x/p + ½⌋`, where the fold is the true `⌊0⌋` and
agrees with `f64`. Elsewhere it is an atom keyed by its argument's form,
so floors of form-equal arguments cancel. A reading of the pick is
numeric unless such a fold or cancellation makes it an identity.

**Measured first**, with a scratch probe over the topo, sweep,
step-import, mesh, editor-core, geom-brep and profile suites (11257
tests):
- no pick is reached at `Sym<Interval>`;
- most are hit at exact f64 half-period ties in ordinary suites
  (`circle_roots`' `near_mid` 67k times, the pole's strict branch 80k,
  `ring_path::branch` 28k).

So a branch is decided as structure only where an identity reads it.

**Decided as structure:** `topo::offset_axial::keep_turn`, by
`geom_brep::whole_period_count`, levered at the carrier's speed floor.
`offset_axial_edge_turn` reads `kept − read` as zero or whole turns. A
mark decided Zero refuses `TogetherAxialEdge`, and one in band escalates.
Rows: `offset_axial::keep_turn_rows`.

**Sound as-is, each on its own premise:**
- **The window re-centring argument** (written at `ring_path::branch`).
  A tie lands on an end of the span, and on a whole-turn span `lo` and
  `hi` are one point read alike. Cited by `ring_path::branch` itself,
  `circle_torus`, `circle_roots` ×2 and `classify`'s midpoint anchor,
  each naming its readers.
- **chord_join's ring re-homing:** its own seam paragraph. A seam vertex
  lands on `hi`, which reads as `lo` would only because such a run
  carries seam rows along both edges.
- **shell's run shift:** concentric pairs by construction. On pairs not
  known to nest, the tie's placement is filed:
  `work/shelf/shell-encloses-reads-a-tie-placed-run-on-pairs-not-known-to-nest.md`.
- **`profile::sugar::signed_swept`:** the sign is the branch; ±π reads
  behind, sound by the configuration argument at its doc. Added to the
  period-jump question as a "which branch" case.
- **`projected::off_period`:** a lattice distance, continuous at the tie;
  its inputs are form-definite deck offsets.

**Correction of PR 4941's first head:** it said `circle_roots`'
`near_mid` `Interval` straddle was load-bearing for the slit dome's
containment. The review's refuse-mutant passes all of `pole_slit_window`'s rows
(straddled rays end in band and a later ray decides). The floor stays
there because no identity reads the root.

**Moved:**
- FLUX's two `props::curved` sites:
  `props-torus-lift-and-gauss-bonnet-fan-pick-a-branch-by-floor`, closed by PR 5375.
- The pole's strict branch, `reduce_periodic` and sugar's ±π:
  `the-period-jump-has-no-ruling`.
- Found reading the readers: the section ruling's seam-blind root order,
  `work/honeroot/ruling-root-order-compares-raw-parameters-across-the-seam.md`.
