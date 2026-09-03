---
id: no-public-rim-arc-selector
kind: issue
title: selection: no public 'give me this rim's arcs' selector — every caller hand-rolls a radius/station scan
status: open
opened: 2026-08-29
github: 1246
refs: [1222]
---

## From GitHub issue 1246

Opened 2026-08-29; 0 comments.

Surfaced independently by BOTH end-to-end reports in the BLEND-1 review (PR #1222) — two reviewers hit the same friction from the consumer's seat, which is the consumer evidence this had been missing.

**The shape.** The fillet verbs take `&[EdgeKey]`. A rim a chart seam has split is SEVERAL edges, and the caller has to produce exactly that set — no more (adding a co-surface seam meridian refuses `TangentialEdge` at margin zero) and no less (a strict subset stops at a seam vertex and refuses `SeamVertex`). There is no public door that hands a caller "the arcs of the rim at this radius and station", so every caller writes the same scan: walk `body.edges()`, keep circular carriers matching a radius and centre to some hand-chosen epsilon, then filter out the ones whose two supports are the SAME surface, because a sphere's seam meridian is a great circle that can share a rim's radius and centre exactly.

That last filter is the part that is easy to get wrong and impossible to discover from the API. It is currently hand-rolled in four-plus test files (`verbs_arms3.rs`, `blend_seam_split_rim.rs`, `blend1_r1_probes.rs`, `review_blend1_r2_probes.rs`, plus older radius scans in the ARMS suites), and BLEND-1 has now homed the TEST-side copy in `test-utils` so the tree carries one implementation. The public gap is untouched by that.

**What would close it**: a selection-side door — the natural home is beside the existing selection vocabulary rather than in `sweep` — that names a rim by a stable predicate and returns its arcs in one call, with the co-surface exclusion built in and the "this is not one rim" case refused typed rather than returning a partial set. Whether the key is (carrier circle) or a named-entity query is the design question; the corpus evidence above is what the choice should be made against.

**Consumers**: every fillet caller on a solid of revolution, the tour, and the recipe layer — a recipe cannot ask for "the mouth rim" today either.

## Home

`work/issues/` — the asked-for door is a new selection-vocabulary door whose home is itself the open question, so no program's territory glob or charter claims it yet.
