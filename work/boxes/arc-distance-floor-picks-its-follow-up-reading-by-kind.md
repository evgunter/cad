---
id: arc-distance-floor-picks-its-follow-up-reading-by-kind
kind: issue
title: arc_distance_floor picks its row name and its piece walk by the carrier's kind, where the cover could say whether its nearest distance is exact
status: open
priority: P3
cost: M
opened: 2026-11-14
---


`topo` `splitting::containment::arc_distance_floor` reads every
carrier's floor from its cover's nearest distance (since
`cover-kind-match-gate`), but still matches on `Curve3` to choose two
things: the `ArcFloor` variant, which names the decision row the cell
door reads it on (`cell_region`'s `SEGMENT`/`CONIC`/`BALL`), and whether
a piece walk (`ArcPieces`) follows a floor that is not clear (a spiric
and a spline only). Both follow from one fact about the cover, whether
its nearest distance is the arc's own (a segment's, a circle arc's) or a
bound on it, which the cover has no reading of. `geom_brep::extent`'s
`cover_is_exact` answers the same question by its own kind match. A
`Cover` reading of it would retire both matches; the row names are
recorded verdict names, so merging or renaming them is its own move.
`scripts/gates/cover-kind-match.sh` pins the match (`arm`,
`splitting/containment.rs`). (boxes-cover7 lane)
