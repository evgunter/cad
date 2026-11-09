---
id: the-variety-certificate-leaves-two-gaps-its-review-found
kind: issue
title: PR 6577's delta review: slit's conic site has no pin for an in-band ring escalating, and the breakpoints gate misses two dedup shapes and over-fires under filter-like adapters
<<<<<<< HEAD
status: open
opened: 2026-11-09
priority: P3
cost: E
=======
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
branch: onecopy/variety-gaps
pr: 6616
>>>>>>> origin/main
---


Filed from PR 6577's DELTA review (MERGE).

- **The conic site's escalation is unpinned (MINOR).** Mutating slit's conic site to swallow `Variety::of`'s `Indeterminate` (`.ok().flatten()` at slit.rs ~1475) leaves all of topo and sweep green, 6581/6581. The spiric site has the pin (`a_spiric_against_a_torus_not_a_ring_is_unread_or_escalates`); the conic site needs a conic against an in-band ring that escalates.
- **`reduce::spline_off_carrier` swallows `Indeterminate`.** `let Ok(Some(..)) else false` turns it into "not certified off". That is conservative for a bool, but the doc's list of false cases (on, in band, not a member) doesn't name an in-band ring. Name it.
- **The `breakpoints-by-hand.sh` gate.**
  - It misses `values().to_vec()…dedup_by` on a `Knots` (the `to_vec` rule names `knots()` only), and `knots().iter().copied().collect(); b.dedup()`. Catch them or name them in the header.
  - It over-fires under `filter`, `find`, `any`, `all` and `inspect`: `runs().filter(|(v, _)| …).map(|(_, m)| m)` needs the multiplicity, and the `_` drops nothing from the stream there. The header's claim "drops its multiplicity" is false for those adapters. Nothing in the tree trips it today.
- **Nits.** The module is still named `spiric_quadric_rows`, and CarrierUnread's doc at slit.rs:186 runs past the wrap width.
