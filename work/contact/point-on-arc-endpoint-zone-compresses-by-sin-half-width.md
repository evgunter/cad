---
id: point-on-arc-endpoint-zone-compresses-by-sin-half-width
kind: issue
title: point_on_arc's endpoint neighbourhood (None) spans eps/sin(w/2) of arc length, so contain's boundary pre-pass can read a point on a short or near-full arc as off the boundary, or escalate far from any vertex
status: open
opened: 2026-09-25
priority: P1
cost: D
refs: [ATREST-11]
---


Found by the ATREST-11 review (PR #3217), as a class note on the defect
that review found in check 9's arm 5. Unreproduced here.

**The compression.** `boolean::contain::point_on_arc` gives its angular
verdict from `Margin::levered(r̂·m̂ − cos(w/2), radius)`. For a point an
arc length `s` past an end, that margin is about `−sin(w/2)·s`. So:
- **`Zero`, which becomes `Ok(None)`** ("an endpoint neighbourhood: the
  vertex pass owns it"), reaches `s ≤ ε / sin(w/2)`.
- **Escalation** reaches `10ε / sin(w/2)`.

For `w = 0.02` rad those are 100ε and 1000ε along the carrier. The same
holds, by `sin` of the complement, for a near-full arc. The vertex pass
the comment defers to is a DIRECT distance at ε (`bool_contact_vertex`).

**What that may cost `contain`.** `boundary_pre_pass` reads
`Some(true)` as `OnEdge` and anything else as "not this edge".
1. A point ON a short arc, more than ε from its vertex but inside the
   compressed zone, is neither `OnVertex` nor `OnEdge`. It goes on to
   the interior/exterior walk as if it were off the boundary.
2. A point in the escalation zone raises `ContainError::Escalated`
   many ε from any vertex.

Check 9's arm 5 had exactly this shape and was fixed in ATREST-11 without
touching `point_on_arc`. `validate::window` decides the distance to
each arc end first, then a two-apex chordal-defect sum that is never
compressed below the distance it measures. The same repair, or a
conditioned rewrite of `point_on_arc`'s window, would apply here.

**To reproduce:** `contfp` on a planar face with a 0.02-rad boundary arc
of radius 10, with `q` placed `50ε` along the carrier past the arc's end
(outside it), and again `50ε` inside it.
