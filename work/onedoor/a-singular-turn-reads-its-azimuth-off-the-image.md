---
id: a-singular-turn-reads-its-azimuth-off-the-image
kind: issue
title: The singular-set turn reads each edge's azimuth at the point off its image
status: open
opened: 2026-11-12
---


Found by `onedoor/three-walks` (2026-11-12).

The turn at a joint on a chart's singular set is the angle between
the two edges' tangents there, on the face's side (`topo::pcurves`'
`singular_turn`). The rule reads each tangent's azimuth as its image's
first coordinate at the point: the exit of the arriving image and the
entry of the leaving one.

That holds for every image the mint derives through such a point
today, where the edge is a meridian and its azimuth is constant. An
edge through a singular point that is not a meridian has no
certifiable chart image, and the mint refuses it earlier
(`NoGeneralLane`; PR 6851's second probe). A lane that images such an
edge has to state the limit azimuth at its end for the rule to stand,
or the rule has to read the tangent off the carrier.

The lever is not part of this: the turn is levered at the faces'
extent (`JointSides::lever`, the shorter of the two edges'
`topo::wedge_reach`), which any certified edge has.
