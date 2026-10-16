---
id: an-imported-off-iso-line-on-a-bspline-face-takes-no-general-row
kind: issue
title: an imported off-iso line on a B-spline face has no rung that reaches its General image
status: open
opened: 2026-10-12
priority: P2
cost: M
---


Asked of `pcert/fitted-row-reads-carrier-and-chart` by its
orchestrator, and measured there by reading the ladder plus a
topo-level probe. Not reproduced through a STEP file.

## What happens

An imported edge whose carrier is a `Line` (promoted by D7, or written
as one) and that runs off-iso across a B-spline face meets the
adoption ladder (`crates/step-import/src/adopt.rs`, `adopt_edges`):

- **IsoCurve rung**: `iso_curve_candidates` offers a promoted line only
  the wall's boundary COLUMNS (`line_column_match`), so an off-iso line
  gets no candidate;
- **Intersection rung**: refused at the edge certificate, which demands
  a spline carrier for a plane × NURBS or NURBS × NURBS pair
  (`work/exch/d7-promotes-an-off-iso-straight-spline-the-intersection-rung-then-refuses.md`);
- **conventional rung**: for a plane × wall pair (`nurbs_rim`) the line
  adopts as an image in the plane's chart. On the wall, the pcurve mint
  reaches its `Chart`/`Scaffold` line-rim arm, which picks a row through
  the start foot and mints a wrong `IsoLine`. The import then refuses
  the face (`LoopDiscontinuity` on the probe,
  `work/pcert/an-off-iso-straight-edge-on-a-spline-face-refuses-its-rows.md`).
  For a wall × wall pair no rung is left.

So no rung reaches the `General` image the fitted-grade row can now
certify for a line.

## What remains

Once the pcert row is fixed, the plane × wall case adopts through the
conventional rung and mints `General`. The wall × wall case also needs
the edge certificate to take a line carrier, or recognition to leave
the spline alone (the D7 row).

## The plane × wall case is closed (2026-10-13)

`pcert/off-iso-line-takes-its-general-row` closed the plane × wall case,
reproduced through a STEP file: `step-import`'s
`an_off_iso_line_on_a_spline_wall_imports_its_general_row`. The file is
a hand-emitted tetrahedron whose base is a degree 2 × 2 B-spline
parabolic cylinder, and whose diagonal `LINE` is shared with a slanted
plane. Main refused the import (`Pcurves { LoopDiscontinuity }`). On that
branch the diagonal adopts through the conventional rung, mints its
`General` row on the wall, and the import ships tier-3 valid.

**The wall × wall case stays open**, as above: it needs the edge
certificate to take a line carrier, or recognition to leave the spline
alone (`work/exch/d7-promotes-an-off-iso-straight-spline-the-intersection-rung-then-refuses.md`).

## The certificate half of the wall × wall case is closed (2026-10-14)

`pcert/spline-pair-any-carrier` let the NURBS × NURBS edge certificate
take a line carrier: `geom-brep`'s
`spline_pair_any_carrier::the_door_certifies_a_closed_form_carrier_on_a_spline_pair`
certifies an off-iso `Line` between two bilinear spline charts through
`EdgeCurve::certify_via`.

**Still missing**: a STEP file that reaches it. That branch tried the
tetrahedron above with its slanted face a non-planar degree 3 × 2 patch
through the triangle's three lines (`h = c·L_oq·L_oa·L_qa` off the
plane). The import refused, not at the diagonal but at a plane × patch
`LINE` (`ao` or `qa`): the uniqueness tube's boundary walk resolved no
count. The likely cause, not measured: at the triangle's corners two
of `h`'s factors vanish, so the patch meets the triangle's plane to
second order there. A fixture whose second wall meets
its other faces cleanly (its other edges isos of it, say) is what
remains, and with it the row that the diagonal adopts at the
`Intersection` rung and mints its `General` row on both walls.
