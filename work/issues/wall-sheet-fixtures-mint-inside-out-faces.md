---
id: wall-sheet-fixtures-mint-inside-out-faces
kind: issue
title: Test fixtures mint curved sheet faces whose loop bounds the other side from their sense: cyl_wall_sheet, boxes' revolved_wall, chain_sheet's clockwise chains
status: open
opened: 2026-10-24
priority: P3
cost: M
---


Found by CHART's `curved-face-containment-answers-none-on-a-ringed-face`
(branch `chart/ringed-curved-doors`). The cylinder and cone point doors
now read a face's region from its boundary, with the face to the left of
its loops under its outward normal (S10), as the sphere and torus doors
always have. Tier 3's check 6 holds the same convention
(`ValidationError::CurvedSenseInverted`) wherever its curved arm reads a
side. It exempts an ellipse-trimmed wall, whose boundary parse refuses,
but such a wall's flips are refused by other arms all the same. A lone
flip is a `LaminaWedge` where two walls share the cylinder, and a
`NegativeVolume` or `PinchCornerCrossed` where the wall is one face. A
whole-body inversion is a `LoopRoleInverted` at the planar faces. This
was executed in PR 5664's review
(`step-export` `m6_6_sense_gate`). The chart trims the doors replaced
read no orientation, so these fixtures never showed it.

## What

- `test_support_fixtures::cyl_wall_sheet_keyed` (and `cyl_wall_sheet`,
  `unit_cyl_sheet`) runs the wall's loop clockwise about the chart
  normal and leaves its sense `true`. Tier 3 reports
  `CurvedSenseInverted` on the wall; with sense `false` it does not.
  The region the loop bounds with sense `true` is the strip's
  complement.
- `boolean::boxes::tests::revolved_wall` (`cyl_wall`, `cone_wall`) is
  the same construction, copied.
- `boolean::section_cert_rows::chain_sheet` gives the face the chain's
  own order with sense `true`, so a clockwise chain is inside-out. That
  includes every apex sector the rows walk outward first. It now calls
  `set_sheet_sense`, which exchanges the face's loop with its lamina
  twin's where the flux lane reads the loop's side. That lane reads no
  side off some loops through a cone's apex (`props_cone_apex_loop_sweep`,
  `props_meridian_apex` escalations), and such a chain keeps its order.
  The small-notch row now writes its apex L counter-clockwise for that
  reason.
- `section_cert_cone_rows::seamless_cone_band` ran both rims about `+z`,
  so its band's two loops bounded no annulus. Fixed on this branch: each
  rim turns about its cap's inward normal, and tier 3 reads the frustum
  clean.

## What the branch did, and what is left

The rows that read containment on these sheets now name their side with
`test_support_fixtures::set_sheet_sense`: census, `carrier_touch_rows`,
`reduce`'s two pierce rows, `offer_rows`' sheet corner, `boolean::tests`'
tangent line, `section_cert_rows`' off-carrier witness, and boxes' mirror
nappe row. The fixtures themselves still mint the inverted wall. Making
`cyl_wall_sheet` call `set_sheet_sense` moved structural rows that pin
which loop the wall holds (`null::tests::a_kef_that_releases_the_loop_…`,
`euler_site_pcurve_rows::a_kill_that_re_describes_only_…`), so it was
left out of a containment unit.

Owed: mint the wall the right way round at the construction (the `mef`
side, or a `sense: false` default with the callers that name a side
moved to `set_sheet_sense`), and re-baseline the structural rows that
pin the old loop.
