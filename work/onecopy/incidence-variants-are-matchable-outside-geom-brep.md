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
