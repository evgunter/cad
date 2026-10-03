---
id: sphere-straddling-a-cylinder-carrier-refuses-at-the-extent-scan
kind: issue
title: A ball straddling a cylinder wall's carrier, clear of the wall face, refuses FallbackExtentUnsupported at the extent scan
status: closed
pr: 3805
opened: 2026-10-02
priority: P1
cost: M
refs: [ball-inside-a-two-sphere-body-refuses-at-the-extent-scan, non-circle-conic-edge-refuses-against-every-curved-face]
closed: 2026-10-02
---

Found by the REACH lane for the ellipse rim
(`work/reach/non-circle-conic-edge-refuses-against-every-curved-face.md`),
which gave the extent scan's cylinder arm its carrier certificate.

## Measured

`boolean::ops::sphere_extent_scan`'s cylinder arm now clears a sphere
that is definitely clear of the wall's whole cylinder, or definitely
inside it (`bool_sphere_cylinder_gap`, `bool_sphere_cylinder_nested`):
a ball held inside a drum or a cylinder, or parked in its box's corner,
builds. What still refuses is a sphere that STRADDLES the carrier while
missing the trimmed face: a slab whose wall turns three quarters of the
way round, and a ball of radius 0.05 straddling the wall's cylinder in
the missing quarter, refuses `FallbackExtentUnsupported` ("… the sphere
straddles the wall's carrier …"). Pinned by
`crates/sweep/tests/m5_s13_pips.rs`,
`a_ball_straddling_a_notched_walls_carrier_refuses_typed_at_the_scan`,
which flips when this row lands.

## What a fix has to supply

A face-scoped verdict, the sibling of the sphere pair's
(`ball-inside-a-two-sphere-body-refuses-at-the-extent-scan`): whether
the sphere ∩ carrier curve (a quartic space curve in general; two
circles when the sphere's centre is on the axis) meets the wall's
trimmed region. The wall's chart outline (`solid_contain::wall_outline`)
places points; a certified azimuth window for the sphere's footprint on
the carrier — `|azimuth − azimuth(centre)| ≤ asin(r/d)` — wholly outside
the face's window would certify the common case.

## Outcome (PR 3805, after #3801 merged)

Fixed by #3801's route, measured on the merge of `origin/main` at
`b4dbcd826` into `reach/conic-edge-curved-face`: sphere × cylinder pairs
are the section pass's (`boolean::ops::section_pass_takes`), whose
certificate decides per FACE. The straddling pose builds under every op
in both orders, tier 3, against `¾·π·0.35²·1.3` and `4π·0.05³/3`:
`crates/sweep/tests/m5_s13_pips.rs`,
`a_ball_straddling_a_notched_walls_carrier_builds` (which replaces the
refusal row this item named). The carrier-only certificate this item
was filed against is gone.

## Closed (2026-10-02, PR 3805)

As in the outcome above: #3801's section-pass route, merged in, carries
these pairs, and the straddling pose is a row that builds.
