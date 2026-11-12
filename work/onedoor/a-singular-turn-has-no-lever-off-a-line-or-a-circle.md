---
id: a-singular-turn-has-no-lever-off-a-line-or-a-circle
kind: issue
title: The singular-set turn has no lever for an edge that is neither a line nor a circle
status: open
opened: 2026-11-12
---


Found by `onedoor/three-walks` (2026-11-12).

The turn at a joint on a chart's singular set is decided against its
whole-period marks at a lever, the shorter of the two edges' reach from
the chart's axis (`topo::pcurves`' `singular_turn`, `singular_lever`).
The lever is stated for a line (a cone's ruling), a circle on a cone or
a spindle sheet (a meridian), and a circle on a sphere (`pole_lever`,
capped at its geodesic curvature radius). An edge on any other carrier
refuses `PcurveMintError::SingularTurn` with `SingularMiss::NoLever`.

No such edge reaches the rule today: an edge through a singular point
that is not a meridian has no certifiable chart image, and the mint
refuses it earlier (`NoGeneralLane`; PR 6851's second probe). The
lever is owed with the image lane for those edges. It needs each
edge's geodesic curvature against the surface at the point, which
`pole_lever` reads in closed form for a circle on a sphere only.

The rule also reads each image's first coordinate at the singular
point as the azimuth of its edge's tangent there. That holds for every
image the mint derives through such a point today (a meridian's
azimuth is constant). A lane that images a non-meridian edge through a
pole has to state the limit azimuth at its end for the rule to stand.
