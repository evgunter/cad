---
id: quadrature-unsupported-says-its-reason-in-prose-alone
kind: issue
title: PropsError::QuadratureUnsupported carries its reason as a sentence alone, so a reader that pins one raise site matches its text: a regression from one refusal to a sibling (a Projected image to a Fitted one) stays green
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Found by the dual review of SECT step 1b (PR 5894, 2026-10-28) and
filed by that lane.

## What

`geom_brep::PropsError::QuadratureUnsupported { what: &'static str }`
(`crates/geom-brep/src/props/mod.rs`) is raised at some sixty sites
across `geom-brep` and `topo::props`, each naming why with a sentence
and nothing typed. A row that pins which door a body reaches can tell
them apart only by matching the text:

- `crates/sweep/tests/cylinder_sphere_frame.rs`'s `door` tells the
  cylinder arm of `topo::props::quad_lane::cut_face_lane` ("curved-cut
  face half-edge carries a pcurve with no harmonic form …") from its
  sphere arm ("conic trim on a cone/sphere/torus chart …") by prefix.
  The first sentence covers Fitted, Projected and focal-section images
  alike, so a chord whose pcurve regressed from Projected to Fitted
  would stay green;
- the PR's reviewers counted 39 sweep-test sites matching a refusal's
  text this way (R1), among them the test files that name the variant
  (`geom-brep/tests/{review_flux_probes_r1.rs, shared/patch.rs}`,
  `sweep/tests/{r1_area_gauge_probes, split_edge_fitted_rows,
  one_segment_loop, encl_curved_loft_shell, shell_general_door}.rs`,
  `topo/tests/a_whole_spiric_oval_bounds_its_mirror_torus.rs`).

## Done when

The refusal carries what tells its raise sites apart as data: the image
kind it found (`geom_brep::PcurveKind`) where an image is the reason, the
chart kind where a chart is, and the lane otherwise. The rows above read
that field rather than the sentence.
