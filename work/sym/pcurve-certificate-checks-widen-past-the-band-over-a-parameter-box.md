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
  (a plate with a parametric hole, plain drive) no longer certifies its
  `1024·ε` box whole and refines to its 1024-leaf budget: 3 s → over
  25 min.
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
