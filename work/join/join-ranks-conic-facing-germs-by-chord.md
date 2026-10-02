---
id: join-ranks-conic-facing-germs-by-chord
kind: issue
title: The join's find_match and loose_partners rank conic-facing germs by chord, and the rotational facing test accepts germs back to back across a gap
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [JOIN-1, rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity]
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

## Why not fixed with the REST lane's patch

Ranking by the swept arc (`join::germ_separation`, added on that
branch for the REST zip) fixes the ranking. But in `find_match` it
reorders the join's surgery: `axis_lap::a_blind_d_pocket_builds_from_below_and_refuses_from_above`
went to `JoinDesync { "ring-run winding is degenerate" }`. JOIN-1's
locus identity replaces the face-pair filter this rests on, so the
ranking belongs with JOIN-1/JOIN-3, measured on the fixture above. That
fixture currently stops earlier, at the wall's pierce ring
(`SectionArcWindow`, `work/tang/pierce-ring-has-no-join-arm.md`).
