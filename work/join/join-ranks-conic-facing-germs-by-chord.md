---
id: join-ranks-conic-facing-germs-by-chord
kind: issue
title: The join's find_match and loose_partners rank conic-facing germs by chord, and the rotational facing test accepts germs back to back across a gap
status: closed
opened: 2026-10-02
priority: P2
cost: M
refs: [JOIN-1, rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity]
closed: 2026-10-04
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

## JOIN-3 (re-measure owed)

The hazard above went red on the blind D pocket's ring-run winding
(`Zero`). Since JOIN-3 the ring lane closes its run with the chord the
join mints rather than the straight one, and the blind D builds from
both faces, so the arc ranking's cost should be re-measured on that
head. A second shape of the same ranking, unmeasured: sites on one
conic where the true partner lies more than a half turn away along the
arc, and another opposed-sense site past it, nearer by chord
(`2r·sin(Δ/2)` falls past `Δ = π`).

## Built

Both shapes are ranked by the turn along the conic now. `find_match`
and `loose_partners` read each germ's candidates in the germ's own
rotational sense: its half-turn first (`germ_arm`), then, within one
half-turn, the side of the incumbent's axis plane the candidate lies on
(`nearer_along`, `bool_join_arc_travel`). That angle is monotone along
any centred conic, in either half-turn, where the chord is not.

- **Back to back across a gap** (the first shape). A site behind the
  germ is `Behind` and loses to every `Ahead` site. The U-plate fixture
  (`crates/sweep/tests/pocket_ring_steep_ellipse.rs`
  `u_plate_battery`, 192 runs) goes from 66 sound, 30 bodies failing
  tier 3′ and the certificate, and 96 refusals on main to 192 sound.
- **The true partner more than a half-turn away** (the second shape).
  Within `Behind` the turn orders the sites, so the nearer one along the
  conic wins where the chord preferred the farther. No fixture reaches
  it: review r2 of PR 4008 looked for a pose with only `Behind`
  candidates left on a locus and found none. The steep-ellipse
  batteries in that file check every germ's pick against its least turn
  over both arms: none is off it across their 5 736 runs, where the
  chord order makes 8 208 such picks on the plate battery alone. None of
  those picks is known to be a choice between `Behind` sites only.

The rotational facing test (`germs_face_each_other`) still ACCEPTS a
back-to-back pair: the two senses oppose. The ranking is what puts such
a pair after the true partner. It never filters it out.


## Also (PR 3985, `reach/arc-from-pairing`)

Reached on a whole body, in the suite:
`crates/editor-core/tests/reach_slab_cut_sector_side.rs`,
`a_slab_across_a_round_boss_builds_in_four_orders_and_stops_typed_in_two`,
order `[1, 2, 0]`: the plate's top cuts the boss wall's circle `z = 1`
at `x = 1.4` and `x = 1.6`, whose germs point away from each other into
the face, and the chord paired them with the true partners further
along the walk. Once a chord takes the arc its pairing names, that pair
refused `RingHomingAmbiguous`.

Under the ranking above that pair is not taken, and the chord takes
the arc the right pair's germs leave along. `crates/sweep/tests/four_crossings_on_one_section_circle.rs`
(a slab crossing one section circle four times, the germs' neighbour
along the walk farther by chord) holds the ranking and the arc
together; it is red under a chord ranking.
