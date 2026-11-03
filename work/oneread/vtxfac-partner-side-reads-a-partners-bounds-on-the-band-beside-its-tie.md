---
id: vtxfac-partner-side-reads-a-partners-bounds-on-the-band-beside-its-tie
kind: issue
title: vtxfac::partner_side reads a partner's bound sides of the touched face on the band alone, about a vertex its tie off the point
status: closed
closed: 2026-11-02
pr: 6407
opened: 2026-11-02
priority: P2
cost: E
refs: [the-cone-takes-a-bounds-side-from-the-one-bound-side-reading, the-cone-sites-read-every-bounds-side-through-bound-read]
branch: oneread/cone-sites-bound-read
---


Found by `the-cone-takes-a-bounds-side-from-the-one-bound-side-reading`'s sweep (§5: a side read on the band alone where the reading it feeds is about a point a tie off).

`vtxfac::partner_side` reads each bound of a paired vertex's sectors against the touched face (`PierceDatum::side`, `sectors::pierce_side_reported` on `Reading::plain`), and `boolean::mod`'s touch pass (the `vtxfac::partner_side(other, touch.datum, touch.flush, band)` call beside `vtxfac::Partner { vertex: partner, at, .. }`) takes its `PartnerRead::Side` verdict. Each bound's departure is read from its own base, the partner vertex `at`, which stands off the touch point by up to the pair's tie (within the zero band). A bound decided off the face at first order is not exposed: decided past `K·zero`, its side does not flip under an offset within the zero band (`K > 1`). The exposure is a **descended** bound, tied at first order: its side is its parting's, decided past `K·zero` on its own, while the first-order residue the tie leaves (up to the zero band) and the base's offset off the face's plane stand beside the parting and are not counted, so a parting decided by a margin inside their sum can read the wrong side. This is the class `the-cone-takes-a-bounds-side-from-the-one-bound-side-reading` closed inside a cone (a slack tie's descent counts its rise, `bound_side::tests::a_slack_ties_descent_counts_its_rise`).

Owed: read the partner's bounds on a reading over the partner's offset from the touch point (`link::metric::Reading::tied`, the slack twice `|at − p|`), or say why the offset cannot reach a decided side; a row at `K = 1.4` with a partner `0.9·zero` off the point and a bound tied with a curved face at first order whose parting clears `K·zero` by less than the residue and the offset.
