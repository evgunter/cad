---
id: a-face-bounded-by-one-closed-spline-edge-has-no-spanned-run
kind: issue
title: A face bounded by one closed spline edge reads no run from it
status: open
opened: 2026-11-23
priority: P2
cost: M
---


The embedding question's extent is the narrower face's run from the
shared edge (`geom_brep::BoundaryStations::run_from_edge`), read at the
face's boundary stations. A face whose whole boundary is the shared
edge has every station on it; for a circle, an ellipse and a spiric
oval the spanned bound stands in (`geom_brep::centre_line`,
`injective::crosses_oddly`: the face meets the line through the curve's
centre). A closed spline rim
(`Curve3::Periodic`, or a clamped spline closed on one vertex) has no
centre the bound can name, so such a face reads a run of about zero
from its rim and the pair refuses.

No body of `crates/sweep/tests/self_intersection_proves_injectivity.rs`
or of the tour's 85 stops builds one (2026-11-23, `embed/verdict-read`:
every loft and duct cap there is bounded by several edges, and reads
its run at the others' middles). A capped tube swept from one periodic
profile would.

The crossing count reads any boundary; what is missing for a spline rim
is the line: one the rim stands off, chosen without a centre.
