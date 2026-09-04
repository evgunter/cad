---
id: no-public-rim-arc-selector
kind: unit
title: selection: no public 'give me this rim's arcs' selector — every caller hand-rolls a radius/station scan
status: open
opened: 2026-08-29
github: 1246
refs: [1222]
needs_ev: true
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

## For Ev — the key shape of the rim selector, and its home

Every consumer hand-rolls the same scan: circular carriers at a radius
and centre, then drop the co-surface seam meridians (a sphere's seam is
a great circle that can share a rim's radius and centre exactly). The
test-side copy is `test-utils`' `rim_arcs_at`.

1. **Keyed by one edge — `rim_of(body, edge) -> Result<Vec<EdgeKey>,
   RimError>` (recommended).** The caller names any one arc, by pick,
   by stable name or by any existing query, and gets the rim whole:
   every edge on the arc's carrier circle whose two supports lie on the
   same two SURFACES as the given arc (several faces of one surface
   across chart seams), co-surface seam meridians excluded by
   construction. Refuses typed when the edge is not a circular arc, and
   when the matched set does not close into one chain (naming the gap)
   rather than returning a partial set. This is exactly the request
   `FILLET3_SEAM_VERTEX_RECOURSE` tells the caller to make, and it is
   name-addressable today: a recipe can say "the rim of edge ⟨name⟩"
   now, and "the mouth rim" whenever rims get names of their own (the
   names vocabulary, SEAT's / Track V's — not this door's question).
2. **Keyed by the carrier circle** — `rim_at(body, centre, axis,
   radius, band)`, the `test-utils` shape made public. Needs a match
   tolerance the caller has to think about, and a recipe cannot spell a
   circle without knowing the geometry first.
3. **Keyed by the support pair** — `rim_between(body, surface_a,
   surface_b)`. Ambiguous whenever two surfaces meet in more than one
   rim (a torus and a plane).

Home: `crates/topo/src/query.rs`, the kernel query seat (the family
`edge_adjacent_matches` lives in) — SEAT's territory, entered by
announced seam in SEAT's log; FILLET writes the spec, and the
`test-utils` copy becomes a call to the door. Not `sweep`: the
selector is a topology question with no blend vocabulary in it.

A 👍 on 1 ratifies the key shape and the unit is cut.

**Ruled (Ev, approved PR 1735, 2026-09-04): option 1.** The door is
`rim_of(body, edge)` in `topo::query`: the whole rim the given arc
belongs to, co-surface seam meridians excluded by construction, "not
one rim" refused typed. This item is now the unit that builds it; the
spec follows, and the `topo/query.rs` seam is announced in SEAT's log
at dispatch.
