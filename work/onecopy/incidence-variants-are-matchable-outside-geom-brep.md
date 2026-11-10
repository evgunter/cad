---
id: incidence-variants-are-matchable-outside-geom-brep
kind: issue
title: The incidence refusal's variants are public and matchable everywhere; a typed payload behind accessors would make a pin by variant impossible
status: open
opened: 2026-11-09
priority: P3
---


Filed by the ONECOPY `onecopy/sign-homes` lane as option B of the
`incidence-pins-in-three-rows-name-the-variant` fork. The orchestrator
chose option A, the counted gate `scripts/gates/incidence-variant-sites.sh`,
and asked for this row as the stronger fix.

## What

`geom_brep::PcurveCertifyError::CarrierOffChart { chart, carrier, why, margin }`
and `Escalated { check: PcurveCheck::ChartIncidence, .. }`, and their
`CertifyError` twins, are public variants with public fields. Any crate
can tell a miss from an in-band read by matching their shape rather than
by asking `geom_brep::IncidenceRead::of` / `CertifyError::incidence_read`.
The gate counts those namings and lets them only fall. When the sweep
began, about 45 sites outside geom-brep named them:

- production ending maps: `crates/topo/src/validate.rs`,
  `crates/topo/src/boolean/ops.rs`, `crates/topo/src/test_support_samples.rs`;
- fixtures that build a refusal: the unit tests in `validate`, `ops`,
  `edge_join`, `euler`, `pcurves`, `shell` and `chart_window`,
  `crates/step-import/src/error.rs`, and
  `crates/editor-core/tests/refusal_concision_chains.rs`;
- `decision()` door pins: `a_curve_off_its_face_at_a_build_is_the_kernels`
  and `an_arc_off_a_plane_or_cylinder_face_ends_as_its_incidence_read`.

`scripts/gates/incidence-variant-sites.tsv` holds the current list.

## What closing it takes

Make the incidence refusal one variant carrying a typed payload whose
fields are private, read through accessors: the read
(`IncidenceRead`), the kinds, and the why. Give it constructors for
fixtures. A match by shape outside geom-brep then does not compile, the
gate and its table can be deleted, and the ending maps read the payload's
`IncidenceRead`. Touches every site in the table. It is a refactor of a
public error type, so weigh it against the gate, which already holds the
rule.

## Two more findings for this fix

Both were found by the FULL review of PR 6688 (S5, S6).

- **One arm, two answers for a read that never reaches it.** The
  classifier reads an incidence refusal before either map is reached,
  so both maps list `CarrierOffChart` for exhaustiveness only. They
  answer differently:
  - `walk_certify_refusal` (`crates/topo/src/boolean/ops.rs:1701`) puts
    it in the `WalkRefusal::Torn` arm (`ops.rs:1705`);
  - `validate.rs`'s reason map (`crates/topo/src/validate.rs:4095`)
    calls it `unreachable!`.

  A typed payload would take the variant out of both maps, so neither
  answer would need writing.
- **A bare `Escalated { .. }` / `ResidualExceeded { .. }` pin is
  invisible to the gate.** Such a pin names neither `CarrierOffChart`
  nor `ChartIncidence`, so `scripts/gates/incidence-variant-sites.sh`
  never counts it, yet it can stand for an in-band or a residual
  incidence read. Under a typed payload the incidence check would be the
  payload's, not a field of the general `Escalated` arm, so such a pin
  could no longer stand for an incidence read.
- **Other refusals carry chart and carrier kinds with no accessor**
  (the delta review of PR 6688, D4). `PcurveCertifyError::NoGeneralLane
  { chart, carrier, .. }` (`crates/geom-brep/src/pcurve_cache.rs:1945`) and
  `CertifyError::ChartImageUnavailable { chart, carrier }`
  (`crates/geom-brep/src/certify.rs:459`) name the kinds too. A test reads them only by
  matching the variant: the "not yet" pin in
  `a_circle_not_shown_on_a_spline_row_refuses_alike_in_every_form`, and
  `a_wall_row_read_in_band_takes_the_lane_a_miss_takes`'s `not_yet`.
  `IncidenceRead::kinds_of` covers the incidence refusal alone. The
  typed payload would give the kinds one accessor across every refusal
  that names them.
