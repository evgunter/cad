---
id: in-band-corner-verdicts-route-to-the-corner-configuration-recourse
kind: issue
title: blend: an in-band corner-independence or cap-transverse verdict is routed to FILLET3_CORNER_RECOURSE, which the refused corner already satisfies
status: open
opened: 2026-09-23
priority: P1
cost: E
---


## Finding

`BlendError::Escalated`'s routed arm sends both of predicate 6's
in-band verdicts to `FILLET3_CORNER_RECOURSE`
(`crates/sweep/src/blend/mod.rs:1379`):

- `fillet3_corner_independence` (`crates/sweep/src/blend/battery.rs:814`)
  is decided only AFTER the corner has passed the valence and
  one-convexity gates just above it, so the corner is trivalent and of
  one convexity. Its
  margin is `|det(n₁, n₂, n₃)|·r`: in band means the three support
  normals are too nearly dependent to call.
- `fillet3_cap_transverse` (`battery.rs:1768`) is decided on a ruled
  link's end cap: in band means the cap is too nearly transverse to
  the ruling to call either way.

`FILLET3_CORNER_RECOURSE` (`mod.rs:678`) tells the reader to "blend a
chain that terminates only in FULLY REQUESTED trivalent vertices of one
convexity between planes, or in TRANSVERSE CAPS on a straight cylinder
edge". The corner that escalated is already trivalent and of one
convexity, and the cap is already transverse up to the margin, so the
recourse names nothing the escalation turned on: a reader who follows
it (requesting the corner's other edges, say) meets the same in-band
margin again.

The levers that do move these margins are the geometry (tilt the
support planes apart, or make the cap clearly transverse or clearly
oblique), the radius (both margins are levered by it) and the
tolerance. A routed sentence of its own, naming those, is the shape
`FILLET3_CONTACT_RECOURSE` already has for `tangent_second_order`.

Found by the CHROME concision fix pass (PR #3108 review, NOTE-2); not
repaired there, since the right sentence is this program's call.
