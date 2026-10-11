---
id: cone-ring-arc-spans-are-read-from-endpoints-not-decided
kind: issue
title: props: a cone ring's arc spans are never decided; the ring-winding test reads each arc's turn from its endpoints while the vector area reads the stored span
status: open
opened: 2026-10-11
priority: P2
cost: E
---

Found by the sweep of `stored-spans-read-raw-past-winding-bound`
(branch `flux/rim-span-decide`). That unit decides every rim's span where
the `Rim` is minted (`Rim::decided`, `props_rim_span_*`), but the ring
edges of a cone face never reach a mint: in `cone_face_closed_form` a
ring goes through `endpoint_read` and then the hole test
(`require_cone_ring_contractible`, `crates/geom-brep/src/props/curved.rs`),
not through `cone_boundary`.

There, each circle or ellipse arc's turn about the axis is read from its
two ENDPOINTS (`turn`, the principal angle, plus a whole turn when it
runs against the arc's sense). The flux, meanwhile, reads the STORED span
through `loop_vector_area`'s `R²·(t1 − t0)` term. A hand-built ring arc
whose stored span is `τ + θ` is therefore read as turning `θ` by the
winding test, while its vector area counts the extra period. The hole
test can then pass a ring the flux measures as wrapping.

Certified bodies are unaffected: every certified door bounds the span at
`τ + zero/r`. The reach is `LoopEdge::hand_built` only, as in the parent
row.

Fix shape: decide `require_span_within_period` on every ring arc, at its
own radius, before the turn is read — either in
`require_cone_ring_contractible` or wherever the cone's ring pass first
touches the edge. Use the rim pair's names if the arc is a rim. An
ellipse arc wants its own key, since its lever is the major semi-axis.
The row is due one red-first row on a hand-built ring. Coordinate with
`cone-face-closed-form-runs-an-iso-premise-its-formula-does-not-need`,
which is reshaping the same function.
