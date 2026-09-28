---
id: certify-escalation-lever-names-a-face-angle-for-checks-that-meter-no-angle
kind: issue
title: geom-brep: certify's escalation table routes every check to the face-angle lever, including span, endpoint, seam and chart-image checks that meter no angle
status: open
opened: 2026-09-28
---


(ENCL implementer, from `certify-escalation-renders-the-coincidence-menu-unlabelled`.)

## What

`geom_brep::certify::escalation_recourse` (`crates/geom-brep/src/certify.rs`)
lists every predicate name `EdgeCurve::certify` escalates under and
routes all 45 to one lever, `EDGE_CLOSE_RECOURSE`: "move the geometry
so the faces meet at a clearer angle, or lower the tolerance". That is
the routing `topo::validate`'s `classify_certify` already had for the
whole `CertifyError::Escalated` arm, kept as it was so the checks
window's text did not move under a prose repair.

The angle lever fits the names that decide an angle or a two-face
locus: `dihedral_arm`, `dihedral_wedge`, `tangent_second_order`,
`tangent_normal_parallel`, `tangent_tube_margin`,
`plane_nurbs_transversality(_reported)`, `ssi_tube_transversality`.
It names nothing the user can act on for the checks that meter no
angle:

- the span: `interval_span_forward`, `interval_span_winding`,
  `nurbs_span_meter` (an edge vanishingly short, or one period long);
- the endpoints and the witness midpoint: `carrier_endpoint_start`,
  `carrier_endpoint_end`, `witness_at_mid_parameter`;
- the conventional description: `pcurve_map_residual`,
  `carrier_matches_mapped_source`, `carrier_in_seam_halfplane`,
  `carrier_on_seam_side`;
- the chart-image mint's thirteen `pcurve_*_chart_*` picks
  (`crates/geom-brep/src/pcurve_cache.rs`, `chart_pcurve`), which
  decide the carrier's placement on ONE surface.

## Repair shape

Split the table's single arm into one arm per lever, each lever a
constant beside the table, and decide the lever per check family
(a shorter or longer edge; a curve on its seam; the tolerance alone
where no geometry lever exists). Both surfaces read the table, so the
checks window moves with the feature tree; pin each new lever in
`topo::validate`'s `certify_escalation_rows` and in
`editor-core/tests/refusal_concision_chains.rs`
`certify_escalations`.
