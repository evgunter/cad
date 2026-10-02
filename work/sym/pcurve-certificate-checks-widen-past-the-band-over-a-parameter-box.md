---
id: pcurve-certificate-checks-widen-past-the-band-over-a-parameter-box
kind: issue
title: the pcurve certificate's envelope and map-residual checks widen past the band over a parameter box, so a closing mint refuses where the body otherwise builds at the certified lanes
status: open
opened: 2026-10-01
---

Found by PCERT's `pcert/at-rest-rows-mandatory` (pcurve rows mandatory
at rest, C4), by execution. `sweep::extrude` now closes with
`topo::mint_pcurves`, so the pcurve certificate
(`crates/geom-brep/src/pcurve_cache.rs`, `PcurveCache::certify`) runs on
every extruded wall at every scalar, where before it ran on none. Over
a parameter box, two of its checks are residuals whose true value is
zero and whose enclosure neither `Interval` nor the tier cancels:

- `pcurve_envelope` — `‖image_form − carrier_form‖` summed over the
  `Harmonic` image's four coefficient vectors (`c`, `a`, `b`, `l`), the
  chart image pushed through the surface against the carrier's own
  closed form;
- `pcurve_map_residual` — `|S(P(t)) − C(t)|` at the schedule's samples;
  the door registers the rim identity for the rims (56 of the plate's
  180 go `registered` under the shipped set), and the rest stay numeric.

**What it costs, measured** (identical at ε = 1e-6, 1e-9 and 1e-12, in
units of ε):

- **M10-7's plate** (`editor-core`'s `m10_7_plate::plate`, the
  document `m10_10_pins_interval` pins) certifies whole up to
  **3.9044e2·ε** of its box under every rule set alike — `shipped`,
  `shipped_without_the_door` and `without_the_algebra` — bounded by
  `pcurve_envelope`. Its ceiling was 0.2631 of its REAL study under
  `shipped` (bounded by `assert_bound`'s dependency widening) and
  7.81e2·ε under `without_the_algebra`. At 0.2632 of the real study the
  replay now stops at the extrude (two `pcurve_map_residual` samples,
  enclosure `[0, 2.2e-4]`), so `assert_bound` is never reached. The
  M10-10 mechanism row (the door and the algebra move the plate
  together) is masked: no rule set lifts the plate past the
  certificate's ceiling.
- **`m10_3_r2_probes_interval::a_consumer_drives_a_two_parameter_document_at_four_widths`**
  (a plate with a parametric hole radius and depth, the default drive
  at 1024 leaves) passes its invariants in 2833 s (it took 3 s) and now
  reads:

      half-width | cert | refused | splits | certified mass
        0.125eps |   32 |       0 |     31 |         1.0000
            1eps |    0 |    1024 |   1023 |         0.0000  (budget)
            8eps |    0 |    1024 |   1023 |         0.0000  (budget)
         1024eps |    0 |    1024 |   1023 |         0.0000  (budget)

  so above an eighth of ε no leaf of that study certifies. It moved to
  the slow set (`.config/nextest.toml`).
- **`sweep`'s `sym11_far_placement_rows`**: the certified lane
  (`Sym<Interval>`, an `r` box of ±ε/64) refuses the stadium at the
  origin at every ε row (`Envelope`, enclosure `[0, 1.32e-9]` at
  1e-9), and at `1e6` (`Envelope`) and `3.7e7` (`MapResidual`) at
  1e-6. The point lanes and the revolved washer do not move.

**Where the fix could live.** Both quantities are identities of the
construction (the image is the carrier's exact closed form), so either
the tier discharges them — the door registering what the extrude's
wall carriers and chart share, as it does the rim — or the certificate
states them in a form whose enclosure does not widen with the box. The
rows above are re-baselined on that branch to the measured state, each
naming this file, so a fix reds them loudly.

**After check 4 was restated** (`pcert/certificate-incidence-fidelity`,
on a periodic chart: the carrier's incidence plus the stored image's
fidelity to the re-derived one), measured at `Sym<Interval>` under the
shipped set at 1e-9 unless named:

- **The plate**: whole to `4.8077e2·ε` (the same multiple at 1e-6 and
  1e-12), still bounded by `pcurve_envelope`. On its 16 wall rows every
  incidence term is a theorem. The azimuth fidelity is a theorem on the
  4 rows each loop starts from and numeric on the 12 the loop walk
  shifted, whose branch is an opaque `floor`
  (`work/pcert/loop-walk-branch-is-an-opaque-floor-atom.md`).
- **The `m10_3` four-widths drive**: `1ε` certifies 680/680 leaves,
  mass 1.0 (it was 0/1024). `8ε` and `1024ε` stay 0/1024 on the budget.
  565 s, down from 2833 s.
- **The `m10_3` two-parameter plate**: whole only to `3.85e-2·ε`, bounded
  by `pcurve_envelope` (the same fidelity row). Both `m10_3_driver` rows
  still certify 0/256.
- **The M10-4 stack-up**: the `ε/8` study certifies in 16 leaves, where
  main certified it in one and this branch's head in none.
- **The shallow-arc grid at bare `Interval`**: main's census again at
  1e-9. At 1e-12, three cells at `off = 1000` refuse at check 3's
  `MapResidual`; none refuses at the envelope.
- **Unchanged**: `sym11` (the stadium refuses at `Envelope` at every ε
  row, the same fidelity term), the tour's tolerance study (0% certified)
  and `chaintol`. `chaintol`'s fractions stay `[6.5e-7, 2.2e-7, 1.1e-7,
  6.8e-8]`, bounded by `pcurve_loop_continuity` when `transform_rigid`
  re-certifies the rows.

With check 3 skipped over the box (scratch, never committed) nothing
moves, because the fidelity row refuses first. With check 3 skipped and
that row's term also dropped (scratch, unsound, measurement only), the
plate reaches 0.2631 of its real study, bounded by `assert_bound`. That
is its ceiling before the closing mint. Under the same pair, the
two-parameter plate reaches `3.57e-2` (bounded by `dihedral_wedge`), the
four-widths drive and both `m10_3_driver` and M10-4 rows certify, and
the tour's tolerance study passes. `sym11`'s stadium builds at every ε
row. `chaintol` does not move.
