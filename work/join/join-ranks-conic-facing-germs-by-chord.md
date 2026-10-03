---
id: join-ranks-conic-facing-germs-by-chord
kind: issue
title: The join's find_match and loose_partners rank conic-facing germs by chord, and the rotational facing test accepts germs back to back across a gap
status: review
opened: 2026-10-02
priority: P2
cost: M
refs: [JOIN-1, rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity]
pr: 3985
branch: reach/arc-from-pairing
---


Found by REACH's sweep on `reach/aligned-half-rods`. Unmeasured on a
whole body: no fixture reaches it yet.

## What

`crates/topo/src/boolean/join.rs` `find_match` and `loose_partners`
face a conic germ by rotational sense (`germs_face_each_other`: the two
senses `axis·((p−c)×dir)` oppose) and then pick the NEAREST candidate
by chord length (`bool_join_nearest` on `chord.norm()`).

The rotational test cannot tell "facing along the arc between them"
from "back to back across a gap": a CCW germ at angle 0.2 and a CW germ
at angle 0.1 have opposed senses too, and their chord (≈ 0.1 r) beats
the true partner's at, say, angle 3.0 (≈ 2 r). The straight-chord test
the conic arm replaced never accepted that pair (both facings are
positive iff the arc turns less than a half turn). The face-pair filter
confines the candidates to one wall face's arc, which hides most of
this. It does not hide two polygon edges on one face pair with a narrow
gap between them, such as a two-half rod crossing a U plate whose slot
is narrower than its prongs.

## A measured hazard for the fix

Ranking by the swept arc `r·φ` (φ in the germ's own sense) fixes the
ranking, and was tried in `find_match` and `loose_partners` on
`reach/aligned-half-rods` (PR 3845, dropped once PR 3823 superseded
that branch's REST-zip patch). It reorders the join's surgery:
`axis_lap::a_blind_d_pocket_builds_from_below_and_refuses_from_above`
went to `JoinDesync { "ring-run winding is degenerate" }`. JOIN-1's
locus identity replaces the face-pair filter this rests on, so the
ranking belongs with JOIN-1/JOIN-3, measured on the U-plate fixture
above. That fixture currently stops earlier, at the wall's pierce ring
(`SectionArcWindow`, `work/tang/pierce-ring-has-no-join-arm.md`).

## The `is_up` filter (PR 3845's dual review)

Both reviewers of PR 3845 noted that this item reads `find_match`'s
candidate set without its `is_up` filter (`join.rs:700`), which may
already exclude some back-to-back pairs; `loose_partners` has no such
filter. A fix measures which of the two the hazard reaches first.

## Measured and fixed (2026-10-03, PR 3985, `reach/arc-from-pairing`)

Reached on a whole body, in the suite:
`crates/editor-core/tests/reach_slab_cut_sector_side.rs`,
`a_slab_across_a_round_boss_builds_in_four_orders_and_stops_typed_in_two`,
order `[1, 2, 0]`. The boss wall third carries a ring hole where the
slab passes through it; the plate's top cuts the circle `z = 1` across
it at `x = 1.4` and `x = 1.6` (`y ≈ 1.59`), whose germs point AWAY from
each other into the face (the long way round), and `find_match` paired
them by chord (0.2) with the true partners further along the walk. On
`main` the window selector then took the short arc, across the hole,
and the op stopped later at the ringed-wall door; reading the germs'
arc instead, it refused `RingHomingAmbiguous`.

The fix is not a re-ranking: `partners` rejects a conic pair when
another site of the same locus lies strictly between them along the
near germ's walk (`walk_passes`, `bool_join_walk_site` /
`bool_join_walk_order`), the order `conic_pairs` pairs by. The chord
ranking among valid pairs is untouched, so a match where chord-nearest
already was walk-nearest keeps its surgery order — the hazard this item
recorded (`axis_lap::a_blind_d_pocket_builds_from_below_and_refuses_from_above`)
stays green. `loose_partners` shares `partners`, so it sees the same set.
