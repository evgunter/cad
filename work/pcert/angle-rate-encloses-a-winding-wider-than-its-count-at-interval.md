---
id: angle-rate-encloses-a-winding-wider-than-its-count-at-interval
kind: issue
title: angle_rate hands every reader at Interval a winding's rate as w·τ/τ, an enclosure wider than the integer it is
status: open
opened: 2026-10-30
priority: P3
cost: M
---


## Finding

Found by CLEAR's RIMSCALAR (PR #6090). `geom_brep::pcurve_cache::angle_rate`
computes a winding's rate as `Period::new(τ).rate(w, Period::new(τ))`,
which is `w·τ/τ`. Both periods are the same `τ` by the function's own
stated assumption, so the rate is the integer `w`. At `f64`,
`(w·τ)/τ == w` for every |w| ≤ 10 (the first miss is |w| = 11). At
`Interval` the quotient of two enclosures of one turn is wider than the
point: `[0.9999999999999998, 1.0000000000000004]` for `w = 1`.

RIMSCALAR met it through `Channel::coefficients` / `harmonic_form`, read by
`topo::injective::Trig::of`'s `exactly(pl, ±1)`: at `Interval` the
corner presentation could not read a cylinder's rim or a cut's ellipse.
It fixed that consumer locally, in `topo::clearance::harmonic_of`, which
reads the rate off the stored winding. These readers still take
`w·τ/τ`:

- `crates/topo/src/chart_region.rs` (`angle_rate::<T>(winding) * t`, near line 2510)
- `crates/topo/src/boolean/boxes.rs` (`harmonic_form`, near line 1148)
- `crates/topo/src/chord_join.rs` (`harmonic_form_of`, near line 3873)
- `crates/topo/src/props/quad_lane.rs` (`harmonic_form` and `harmonic_form_of`, near lines 614 and 844)
- `crates/topo/src/revert.rs` (`harmonic_form`, near line 745)
- the pcurve derivations themselves (`pcurve_cache.rs`, about 15 calls)

## Why it was not fixed at the home

RIMSCALAR tried the fix at the home: `angle_rate` returning
`T::from_f64(f64::from(winding))`, with its local rewrite removed. The
`ci` nextest profile over geom-brep, topo, sweep, mesh and editor-core
passed (11 162 tests). The slow set then moved a golden in another
program's territory:

- `editor-core::all m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row`.
  For the plate at ε `1e-9`, the Plain/Decision line goes from 20618 forms
  (digest `5b68ab11…`) to 20612 (digest `e0e51439…`), and three more
  lines' digests and counts move with it. At the Sym tier `w·τ/τ` is no
  longer built.
- This row is also red on main for a different line (an Early/Decision
  digest; `work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`).
  The move above is in addition to that.
- The slow-set run was stopped at about 4 300 of 12 698 tests, once this
  move had decided the question. Its other reds (`perf12_census_goldens`,
  `revalue_heat_sink_fins` and two `sym_9_retry_interval` rows) also fail
  at RIMSCALAR's head without the home fix. All are known main reds.

So the fix at the home is a Sym-ledger re-bless, which is this program's
call. If it is taken, `Period::rate` has no other caller, and its doc
("A winding becomes a slope only here", period.rs) would need restating.
RIMSCALAR's local `harmonic_of` read would then go; its unit row
`clearance::winding_rate::a_windings_rate_is_its_count` stays as the pin.
