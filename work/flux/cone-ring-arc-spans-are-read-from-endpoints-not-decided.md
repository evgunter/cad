---
id: cone-ring-arc-spans-are-read-from-endpoints-not-decided
kind: issue
title: props: a cone's ellipse-arc spans are never decided; the ring-winding test reads each arc's turn from its endpoints while the vector area reads the stored span
status: closed
pr: 4744
opened: 2026-10-11
priority: P2
cost: E
closed: 2026-10-12
---

Found by the sweep of `stored-spans-read-raw-past-winding-bound` (PR
4680). That PR decides every CIRCLE span the cone reads
(`require_cone_circle_spans`, under `props_rim_span_*`, before
`loop_vector_area` and `cone_edge_length` on every loop of the flux lane
and the material sign — so before the ring hole test too). What stays
open is the **ellipse** arc: a tilted section on the cone, read by the
same two lanes.

In `require_cone_ring_contractible`
(`crates/geom-brep/src/props/curved.rs`) each circle or ellipse arc's
turn about the axis is read from its two ENDPOINTS (`turn`, the
principal angle, plus a whole turn when it runs against the arc's
sense), while `loop_vector_area` reads the STORED span
(`a·b·(t1 − t0)`). A hand-built ellipse arc whose stored span is `τ + θ`
is read as turning `θ` by the winding test while its vector area counts
the extra period, so the hole test can pass a ring the flux measures as
wrapping; on the outer loop the ellipse-cut branch integrates the
overrun directly.

Certified bodies are unaffected: every certified door bounds the span at
`τ + zero/r`. Reach is `LoopEdge::hand_built` only.

Fix shape: decide `require_span_within_period` on every ellipse edge the
cone reads, beside `require_cone_circle_spans`, under its own key (an
ellipse arc is not a rim) at certification's ellipse lever — read
`certify.rs` for which semi-axis it uses. One red-first row on a
hand-built loop. Coordinate with
`cone-face-closed-form-runs-an-iso-premise-its-formula-does-not-need`
(PR 4696), which rewrites `cone_face_closed_form`.
