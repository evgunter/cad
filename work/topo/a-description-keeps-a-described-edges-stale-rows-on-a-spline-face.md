---
id: a-description-keeps-a-described-edges-stale-rows-on-a-spline-face
kind: issue
title: set_edge_curve and kev_describing keep a certified edge's rows on a spline-chart face across a description that moves its carrier or interval
status: open
opened: 2026-11-23
priority: P3
cost: E
---

Found by the WALKS lane that made the finish complete a body's pcurve
rows (`walks/finish-completes`), sweeping the construction-state doors
for one that leaves a stale row now that no producer re-mints to close.
Found by reading; not reproduced, and nothing in the suite went red on
it.

`Body::description_rows` (`crates/topo/src/attach.rs`, `fn
description_rows`) plans a re-mint only on the faces
`Body::description_remints` selects, and that predicate excludes a
certified edge's face on a spline chart (`face_on_spline_chart`): "a
certified edge's face is left where the closed-form lane does not
reach". Both callers, `Body::set_edge_curve` (`attach.rs`, the
`set_edge_curve_certifying` body) and `Body::kev_describing`
(`crates/topo/src/euler_kill.rs`), then swap the carrier and leave that
face's rows as found. Where the description moves the edge's carrier or
interval, those rows are stated over the old ones, and the posture
table says so (`pcurves::staleness_posture`'s `Completes` variant: "A
certified edge's face on a spline chart is left as found, for the
tier-3 pass").

Under completion (`docs/DESIGN.md`, `[one-way.places]`'s at-rest
validity row: "in construction state a door keeps a stored row current
or drops it") that is a door that does neither. The finish keeps the row
and tier 3 refuses it (`RowInterval` or `Certify`) rather than the
clearing mint replacing it, so a producer that re-describes an edge of a
spline face now refuses where it used to build. The same face left
half-minted by the site mint is the other arm of the predicate
(`StoredRows::remints` false): its described halves keep their rows too.

The fix is the drop: the halves of a described certified edge whose face
the plan does not re-mint lose their rows in the door's mutation phase,
unless the description restates the stored carrier and interval
(`Body::restatement`), whose rows stay current. The finish completes
them.


## Note (2026-11-25, PR 7497)

Reached in the suite once main's single cut landed:
`two_rims_review_probes::a_ball_segment_inside_its_ball_builds_every_op`'s
ball ∪ segment goes through the Boolean's fallback, whose
`finish_output` join stage grows a kept edge through `kev_describing`
(`crates/topo/src/boolean/edge_join.rs`, `join_one`). The kept edge's
rows stood over its old interval and tier 3 refused them
(`LoopDiscontinuity`), where the old closing mint had replaced them.
The join doors now drop the rows of every face the kept edge bounds
(`drop_kept_faces_rows`, in `join_all`), so completion derives those
faces whole. The general door, `kev_describing` / `set_edge_curve`
keeping a described edge's spline-face rows, stays this row's.
