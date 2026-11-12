---
id: the-rotational-lane-omits-the-image-chains-junction-gaps-from-its-region-term
kind: issue
title: The rotational lane's region term integrates each pcurve image and omits the connectors its junction gaps leave, which no pad covers
status: closed
opened: 2026-11-04
priority: P2
cost: M
closed: 2026-11-04
pr: 6509
---


Filed by the SPIRICARM stage 1 fix pass (PR 6478, 2026-11-04), while
padding the lane's chart term.

## What

`geom_brep::props::quad::rotational`'s region term is the sum over a
face's edges of each pcurve image's own `−∫F(v) du`
(`crates/geom-brep/src/props/quad/rotational.rs:696`, `edge_terms`). The
loops' closure (`loop_windings`, `rotational.rs:933`) accepts a junction
whose two images' ends differ by up to the slack `end_slack` reads at each
end (`rotational.rs:906`: `1.5·env`, plus a harmonic end's bracket times
its speed). The image chain is then open at that junction by up to the
slack, and the region it bounds is closed only by a connector the sum
does not integrate. That connector's term is `−F(v_j)·Δu`, at most
`r·|v_j − v_a|·g` for a gap `g` at level `v_j`, `v_a` the lane's anchor
level.

The map-residual pad (`r·Σ L·env`) pays for the area between the images
and the true boundary along the edges, not for the omitted connector,
whose term grows with the junction's height above the anchor rather than
with the edges' length. The harmonic lane has had this since it existed.
The chart term's pad (`chart_term_pad`, same file) does pay for the
connectors' share of `A⃗`.

## Why it matters

The term is of order `r·H·env` per junction, `H` the face's height: on a
tall narrow face it can pass the residual pad, so the enclosure could
miss the face by more than its width. That is a soundness gap, though
small, at ε.

## Done when

Each junction's connector term is added as a pad, `r·|v_j − v_a|·g` at
the slack the closure accepted, or the closure reads the images' ends as
one point. A row with a tall face whose images' ends sit apart inside
the slack holds its closed form.

## Closed (SPIRICARM stage 2, PR 6509, 2026-11-04)

The lane reads each junction's connector, `F(v)·(Δu − m·τ)` over the two ends' levels, and pads the region between it and the true vertex (`connector_terms`), on every chart it holds. The rows `a_tall_face_whose_images_stop_short_holds_its_closed_form` and `images_short_of_their_pieces_and_moved_out_hold_the_square` go red without the term and the pad respectively. The spiric wall lane's twin is `the-spiric-wall-lane-omits-its-junctions-connectors-from-its-region-term`.
