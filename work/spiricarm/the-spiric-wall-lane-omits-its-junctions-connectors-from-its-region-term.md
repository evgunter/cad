---
id: the-spiric-wall-lane-omits-its-junctions-connectors-from-its-region-term
kind: issue
title: The spiric torus-wall lane accepts junctions whose images' ends sit up to their envelopes apart and integrates each image alone, so the connectors closing its chain are in no term or pad
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Filed by SPIRICARM's rotational-lane stage 2 (`spiricarm/rotational-lane-sphere`,
2026-11-04), the sweep behind
`the-rotational-lane-omits-the-image-chains-junction-gaps-from-its-region-term`,
which that stage fixes for the rotational lane's charts (the cylinder
and the sphere) by reading each junction's connector
(`geom_brep::props::quad::rotational`, `connector_terms`).

## What

`geom_brep::props::spiric` `wall_windings` closes each loop of a
spiric-bounded torus wall through `closure::interval_loop_turns`, with
each junction's slack the two lifted images' envelopes
(`slack: certified_hi(e.env + next.env)`). The wall's Green form then sums
each edge's own integral over its image, so where the closure accepted a
gap the chain is open by up to that slack, and the connector that would
close it, `−F(v_j)·Δu` on the torus's `F`, is in no term. The wall's
map-residual pad (`Σ chart length · envelope` at the flux density) pays
for the strips along the images, not for the connectors, nor for the
region beside each one between it and the true vertex. Unverified by a
run: found by reading, as the same structure the rotational lane had.

## Done when

The wall's region term reads each junction's connector at the levels
between its two ends, and pads the region beside it, or its closure
reads the images' ends as one point; with a row whose images stop short
of their vertices within their envelopes that goes red without it (the
rotational lane's `a_tall_face_whose_images_stop_short_holds_its_closed_form`
and `images_short_of_their_pieces_and_moved_out_hold_the_square` are
the shape). The torus stage of the rotational lane (stage 3) may take
the spiric wall in as one more edge term, which would close this.
