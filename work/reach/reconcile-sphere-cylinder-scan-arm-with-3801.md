---
id: reconcile-sphere-cylinder-scan-arm-with-3801
kind: issue
title: Reconcile #3805's sphere × cylinder extent-scan arm with #3801, which hands those pairs to the section pass
status: closed
pr: 3805
opened: 2026-10-02
priority: P1
cost: E
refs: [non-circle-conic-edge-refuses-against-every-curved-face, sphere-straddling-a-cylinder-carrier-refuses-at-the-extent-scan, ball-inside-a-two-sphere-body-refuses-at-the-extent-scan]
closed: 2026-10-02
---


Two open REACH PRs change the same site and the same two sweep rows.
Whichever lands second reconciles; this item records what that means.

## The overlap

- **#3805** (`reach/conic-edge-curved-face`) keeps the sphere × cylinder
  arm of `boolean::ops::sphere_extent_scan` and gives it a carrier
  certificate: `bool_sphere_cylinder_gap` (sphere definitely clear of the
  wall's whole cylinder) and `bool_sphere_cylinder_nested` (definitely
  inside it), each charged `rounding_charge(|w| + r + r_w)`. A sphere
  straddling the carrier still refuses `FallbackExtentUnsupported`,
  filed as `sphere-straddling-a-cylinder-carrier-refuses-at-the-extent-scan`.
- **#3801** (`reach/extent-scan-faces`) removes that arm: sphere ×
  cylinder pairs go to the section pass (`section_pass_takes`), whose
  certificate has a `sphere_cylinder` arm and decides per face.
- Both rename the pinned rows in `crates/sweep/tests/m5_s13_pips.rs`
  (`cylinder_near_sphere_refuses_typed_at_the_scan`) and
  `crates/sweep/tests/verbs_cylsph_opening.rs`.

## What reconciling means

If #3801 lands first, #3805 merges `origin/main` and:

1. drops its cylinder arm and the two rows `bool_sphere_cylinder_gap`,
   `bool_sphere_cylinder_nested` (and their `offer_rows` / census
   entries), taking #3801's `section_pass_takes` route;
2. takes #3801's versions of the two sweep files, then re-adds only the
   rows #3805 needs that #3801 does not already cover (the ball held in
   a drum, the ball in the box corner — #3801's
   `a_ball_in_the_wall_boxs_corner_is_certified_separated` covers it);
3. re-measures the straddling pose against the section pass; if it
   builds, closes `sphere-straddling-a-cylinder-carrier-refuses-at-the-extent-scan`
   with that row, else re-files it at the section pass's refusal;
4. re-runs the three ε and the class sweep for the conic rows.

If #3805 lands first, #3801 does the mirror: its section-pass route
supersedes the carrier arm, and the same two rows go.

## Outcome (PR 3805)

#3801 landed first (`b4dbcd826`). The merge into
`reach/conic-edge-curved-face` took its route in full: the scan's
sphere × cylinder carrier arm and its two rows
(`bool_sphere_cylinder_gap`, `bool_sphere_cylinder_nested`) are gone,
and those pairs go to the section pass per `section_pass_takes`.
`m5_s13_pips.rs` and `verbs_cylsph_opening.rs` take #3801's rows; the
one pose #3801 did not cover, the ball straddling a notched wall's
carrier, now builds there and is kept as a row (closing
`sphere-straddling-a-cylinder-carrier-refuses-at-the-extent-scan`). The
conic rows (`conic_edge_curved_face`) pass on the merged route at the
three ε.

## Closed (2026-10-02, PR 3805)

As in the outcome above: #3801's section-pass route, merged in, carries
these pairs, and the straddling pose is a row that builds.
