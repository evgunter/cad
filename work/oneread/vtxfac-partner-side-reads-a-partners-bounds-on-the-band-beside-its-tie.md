---
id: vtxfac-partner-side-reads-a-partners-bounds-on-the-band-beside-its-tie
kind: issue
title: vtxfac::partner_side reads a partner's bound sides of the touched face on the band alone, about a vertex its tie off the point
status: open
opened: 2026-11-02
priority: P2
cost: E
refs: [the-cone-takes-a-bounds-side-from-the-one-bound-side-reading, the-cone-sites-read-every-bounds-side-through-bound-read]
---


Found by `the-cone-takes-a-bounds-side-from-the-one-bound-side-reading`'s sweep (§5: a side read on the band alone where the reading it feeds is about a point a tie off).

`vtxfac::partner_side` reads each bound of a paired vertex's sectors against the touched face (`PierceDatum::side`, `sectors::pierce_side_reported` on `Reading::plain`), and `boolean::mod`'s touch pass (the `vtxfac::partner_side(other, touch.datum, touch.flush, band)` call beside `vtxfac::Partner { vertex: partner, at, .. }`) takes its `PartnerRead::Side` verdict. Each bound's departure is read from its own base, the partner vertex `at`, which stands off the touch point by up to the pair's tie: a bound whose side the band decides at a margin inside twice that tie reads that side, while the true far point's side of the face is not decided. It is the shape lane 1 closed at recl's held face (`recl::tests::a_held_faces_side_decided_within_the_cones_tie_is_no_side`, reachable at `K ≤ 2` with a tie inside the zero band).

Owed: read the partner's bounds on a reading over the partner's offset from the touch point (`link::metric::Reading::tied`, the slack twice `|at − p|`), or say why the offset cannot reach a decided side; a row at `K = 1.4` with a partner `0.9·zero` off the point and a bound `1.5·zero` off the face.
