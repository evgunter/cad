---
id: an-unrecorded-curved-edge-meeting-is-unseen-at-tier-three-prime
kind: issue
title: Tier 3′'s edge-edge pass reads lines only: an unrecorded meeting of a curved edge with another edge reads clean
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [5027]
---

Found by FUSE's sweep for `a-corner-on-a-circles-interior-is-unseen-at-tier-three-prime`
(branch `fuse/census-tier3-corners`).

That unit gave tier 3′ a pass 2′, `census::sweep_vertex_curved_edge`,
which finds an unrecorded vertex on a curved edge's interior through
`census::curved::on_curved_interior`. Its sibling stands: pass 5
(`sweep_edge_edge`) pairs only the `Line` edges the snapshot keeps
(`census::snapshot`, `edge_is_line`). So an unrecorded crossing,
touch or overlap of a curved edge with another edge is no finding. An
op that lost such a record ships a body whose meeting no tier sees.

The confirm lane can read such a pair
(`census::curved::curved_interiors_meet`), so a pass 5′ over it is
the shape. It is held on `[ev]` PR 5027: the curve-pair reading it
would sweep with is what that PR decides. Related, and narrower:
`work/contacthold/two-coincident-curved-edges-touching-record-no-edge-edge-row.md`
(the writer side, for two coincident curved edges).
