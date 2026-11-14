---
id: the-rotational-lanes-map-residual-pad-reads-only-the-images-length
kind: issue
title: The rotational lane's map-residual pad pays e times the image's length, while the strip to the true boundary is bounded by e times the mean of both lengths, short by second order where the image is the shorter curve
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Filed by SPIRICARM's stage 2 fix pass (PR 6509), while building the
row that witnesses the sphere's projected speed bound.

## What

`geom_brep::props::quad::rotational`'s map-residual pad is
`(metric length of the image)·e` per edge (`RotationalChartQ::residual`,
read by `cylinder_edge_terms` and `sphere_edge_area`). The certificate
bounds the true boundary to within `e` of the image at each parameter,
and the area the homotopy between the two sweeps is at most
`½·e·(L_image + L_true)` (the chart-term pad's own argument,
`chart_term_pad`). Where the true boundary is longer than its image, the
pad is short by `½·e·(L_true − L_image)`, of order `e²·κ·L`. A cap read
through a rim a little below its true one: the strip is
`2πr²(sin(V+δ) − sin V)` and the pad `2πr·cos(V+δ)·rδ`, short by about
`πr²·sin V·δ²`. Found by derivation, unverified by a run: on every body
the envelope is near `1e-12`, so the shortfall is near `1e-24`, and the
speed bound's slack (an `L1` cross product, a sup over pieces) has
covered it on every row built.

## Done when

The pad reads both lengths (the image's, and an upper bound on the true
boundary's, such as the carrier's plus the envelope's turning), or the
argument for the image's length alone is stated with the condition it
needs; with a row whose image is the shorter curve.
