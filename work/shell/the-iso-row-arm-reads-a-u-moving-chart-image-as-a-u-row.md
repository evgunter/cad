---
id: the-iso-row-arm-reads-a-u-moving-chart-image-as-a-u-row
kind: issue
title: replace_face's iso-row arm reads any IsoLine chart image as the u-row at p0.x, but chart_image mints images that move u, so a v-row is planned as the wrong row
status: open
opened: 2026-10-09
priority: P2
refs: [a-moved-fitted-faces-corners-have-no-root-on-a-derived-spline-section]
---

Found by SHELL's fitted-corners unit, measured on a scratch fixture not
kept in the tree: the unit box with its cap swapped for a bilinear NURBS
patch over `[0, 2]²` at `z = 1`, each cap edge re-described as
`EdgeDescriptionSpec::chart_image(cap, Pcurve::IsoLine { p0, pl })` on the
patch's own key (the shape `crates/sweep/tests/common/approx.rs:234`'s
`box_with_approx_cap` writes), `mint_pcurves`, valid at rest. Moving the
cap by ±0.05 through `topo::replace_face_offset`:

- `offset_edge_plans_for_tests` plans all four edges `Ok(None)` through
  the iso-row arm in `plan_edge` (`crates/topo/src/replace_face.rs:2095-2171`,
  "The one description that gets an EXACT carrier"; the premise at
  :2104-2107).
- The door refuses `ReanchorPastCarrierEnd { gap: 2.0 }` on a vertical
  side edge: a corner landed two metres off.

The arm's premise reads "An iso image on a DESCRIPTION is u-const by
construction — `EdgeDescriptionSpec::iso` is the only door that mints
one". `EdgeDescriptionSpec::chart_image`
(`crates/geom-brep/src/description.rs:283`) is public and mints any
`IsoLine`, so the two cap edges along `x` carry `pl = (0.5, 0)`. The arm
reads them as the `u`-row at `u = p0.x` with `v0 = v1`, extracts that
row from the new fit, and plans its ends as the corners. A later gate
caught it here, with a refusal that names the re-anchor, not the misread.

The same fixture over `[-1, 3]²` (rows interior to the fit) refuses
`IsoRow { error: Interior { u: 0.25 } }` at every edge, where the same
edges with stale descriptions derive as sections with the held planes
and the move builds (`crates/sweep/tests/encl_curved_loft_shell.rs:719`,
`a_moved_fitted_cap_stands_its_corners_on_the_held_sides`).

Owed: the arm takes a `u = const` image only, and an image that moves
`u` — or an interior row beside a plane — goes to the section route or
refuses by its own name.

Why it was not fixed where it was found: deciding `pl.x` zero at a
generic scalar is a decision on a chart slope with no stated margin. The
unit's review answered that this reason is thin: an exact guard, `pl.x
!= 0` refusing by name, can only refuse more than the arm does today —
it never takes a row the arm would not — so it needs no margin to be
sound. The guard is owed here either way; the margin question is only
whether a slope in band should take the section route rather than
refuse.

## Decided

The SHELL orchestrator's spec for unit 17, carrying the iso-row half of
`a-wall-seam-between-two-fits-has-no-section`'s `## Designed` (ruled by
Ev in PR 4515).

**The gap.** `plan_edge`'s iso-row arm extracts a row of the moved fit
for ANY `IsoLine` chart image on the old key. Its premise is that "two
sides of one chart move with the chart". That is true only when both
sides are the same face (a wrap, or a seam the face shares with
itself), or when the distinct neighbour holds the move.

- **Spline or fitted neighbour.** The arm refuses
  `FittedBoundaryUnsupported` with "a row of this fit shared with a
  spline face" or "…with another fitted face". The same seam's twin,
  on the wall's other side, routes as `Approx × Nurbs` and refuses
  `NeighborPairUnroutable`. So the description's asymmetry, not the
  geometry, picks the refusal.
- **Analytic neighbour that does not hold the move** (a tilted side
  plane). The arm extracts a row that does not lie on the neighbour,
  and only tier 2 catches it, unnamed.
- **The u-moving `IsoLine`.** This item's bug: `chart_image` is public
  and mints any line, and the arm reads every one as the u-row at
  `p0.x`.

**Decided.**

1. The iso-row arm fires only for a self-shared image (both sides the
   same face), or a distinct neighbour that holds the move.
   `offset_derive::holds_the_move` gains its NURBS/Approx-mover arms: a
   neighbour containing the moved chart's normal along the row, decided
   levered by |d| as every arm is. A plane is the case to build. For a
   spline neighbour that is the C7 normal-alignment margin along the
   seam, which is `two-fits-sharing-a-smooth-seam-disagree-by-their-certificates`'s:
   leave it refusing under its own name there, or answering false, and
   say which.
2. Every other iso image on a moving fit takes the section route
   (`derive_edge` through C5). The two seams of one wall then answer
   alike: `NeighborPairUnroutable(Nurbs, Nurbs)`, or `(Approx, Nurbs)`,
   until the NURBS × NURBS arm lands. The guard's two strings retire.
   If `FittedBoundaryUnsupported` is then unused, retire the variant,
   or say what still mints it.
3. The row the arm reads is the image's own line. An axis-aligned
   u-moving line is the v-row at its fixed v. A line that is not
   axis-aligned is not a row, and refuses typed, or takes the section
   route. Decide that `pl.x` or `pl.y` is zero EXACTLY: an exact zero
   test can only refuse more, so it needs no margin.
4. Re-pin the rows that move (`shelling_the_curved_loft_refuses_at_a_walls_fit`;
   `the_fitted_obstruction_holds_on_a_curved_fit` and
   `a_fitted_walls_rims_answer_for_themselves_behind_its_seams`; the
   moved-fit seat rows `box_with_approx_cap` uses), and add rows for a
   u-moving `chart_image(IsoLine)` cap edge, which now plans its own
   row; a tilted analytic neighbour, which now takes the section route;
   and a self-shared seam, which still extracts.
5. Docs: the arm's comment; the `replace_face.rs` module doc's "two
   sides of one chart move with the chart", narrowed to a self-shared
   image; O4's refusal list in `crates/geom-brep/README.md`.

Out of scope: the general simultaneous door
(`shell-moves-every-chart-of-a-solid-through-one-simultaneous-door`)
and the NURBS × NURBS arm.
