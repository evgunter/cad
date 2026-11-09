---
id: the-ring-torus-question-has-two-homes
kind: issue
title: Whether a torus is a ring is decided by geom::ring_torus and again by Variety::of, and census drops Variety::of's Indeterminate
status: open
opened: 2026-11-09
---


Found by the FULL review of PR 6616 (`onecopy/variety-gaps`, head 94036920), notes F7 and F8.

**Two homes for one question.** Whether a torus's tube clears its axis, R − r against the band, is decided in two places:
- `geom::ring_torus` (geom surfaces.rs ~496). Its doc says it "lives here … so that every door that leans on the ring reads it whatever its layer". It is read by offset, circle_torus, torus_convention, validate and face_normal. `geom::is_ring_torus` also decides the tube half `r > 0` first.
- `Variety::of` (geom-brep variety.rs ~91, from 22e00023). It spells `decide(ring, Margin::of(R − r))` again under a site name the caller supplies, and never decides the tube.

**Its callers each map the refusal by hand.** `Variety::of` returns `Result<Option<_>>`, and its four non-test callers apply three policies:
- slit.rs ~1475 and ~1615 escalate with `map_err(refused)?`.
- reduce.rs ~4130 reads the refusal as a documented `false`. No test pins that in-band → false reading.
- census.rs ~4212, `Variety::of(surface, name, band).ok()??`, drops the Indeterminate.
  - It is unreachable today: the function returns None unless the surface is a Cylinder or a Cone, and `Variety::of` errs only for a Torus.
  - A comment holds that safety ("only a torus asks it"), not a type.
  - It passes the line-reading site's `name` as the ring site.

**The fix.**
1. `Variety::of` reads the ring through geom's one door.
2. The refusal policy lives in one place: either a typed reading that callers cannot drop, or one method per policy.
3. census.rs's site gets a type that cannot drop the refusal.
