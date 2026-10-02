---
id: loop-walk-branch-is-an-opaque-floor-atom
kind: issue
title: the loop walk stores each row's whole-period branch as an opaque floor atom, so check 4's azimuth fidelity is numeric on every walked row over a parameter box
status: open
opened: 2026-10-02
priority: P0
cost: M
design: true
refs: [3781]
---

Found by `pcert/certificate-incidence-fidelity`, by measurement.

`topo::pcurves::pin_branch` lands each walked half-edge's image on its
predecessor's exit with
`cand.shift_branch(ku, τ)`, `ku = (prev.x − raw.x).periodic_branch(τ)`.
At `Sym<Interval>` that `ku` is a `floor` atom, opaque to the tier, even
where its enclosure is the single integer it denotes over the whole box.
So the stored azimuth is `α + floor(…)·τ`, never `α + k·τ` with `k` a
literal, and every row but the walk's first carries the atom. That
includes rows whose `ku` is 0.

Check 4's fidelity term compares the stored azimuth with the one it
re-derives from the carrier (`fidelity` in
`crates/geom-brep/src/pcurve_cache.rs`). On a walked row that difference
has the form `floor(…)·τ`, so it is not the zero polynomial. The decision
falls to the value channel, where `α_stored − α_derived` carries the
enclosure width of `α` twice. Every incidence term is a theorem on all
16 wall rows of M10-7's plate. `cif_term_fidelity_u` is theorem on 4
rows (each loop's first) and numeric on 12. It is the plate's bound:

- as built: the plate certifies whole to `4.8077e2·ε`, and the only
  over-band check is `pcurve_envelope`;
- with check 4's azimuth fidelity dropped (scratch, unsound,
  measurement only): `6.9445e2·ε`, now bounded by `pcurve_map_residual`;
- with it dropped and check 3 off the box certificate as well:
  0.2631 of the real study, bounded by `assert_bound`. That is the
  plate's ceiling before the closing mint.

Two homes, and it is a design question which:

- the walk pins a literal branch. `topo` would read the integer off the
  value, which needs a bracket door on the walk's scalar; or
- the tier folds `floor(X)` to the integer `k` when `X`'s certified
  enclosure lies inside `[k, k + 1)`. That is a value-reading fold of
  rule C's class, SYM's ground (`crates/geom-core/src/sym.rs`). It
  would also reach the `pcurve_loop_continuity` rows.
