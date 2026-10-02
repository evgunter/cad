---
id: face-clearance-screen-skips-boundary-edges-it-cannot-read
kind: issue
title: blend: predicate 2's boundary-pair screen silently skips a face, a lone-vertex loop or an edge whose carrier does not certify
status: closed
opened: 2026-10-01
priority: P2
cost: E
pr: 3786
closed: 2026-10-02
---


## Finding (by reading; not measured)

`consumption_sweep` (`crates/sweep/src/blend/battery.rs`), predicate
2's boundary-pair screen, drops what it cannot read with a bare
`continue`: a support face that does not resolve, a loop whose
boundary is not a `Cycle` (a lone-vertex loop), a half-edge that does
not resolve, and an edge whose carrier does not certify
(`carrier_of` → `None`). Each skipped feature is left out of every
pair, so the screen reports clear having metered nothing for it — a
silent skip, where the surgery's closed-form meters refuse typed on the
same inputs (`ring_clearance_pass`'s support-boundary walk and cap
meter in `crates/sweep/src/blend/surgery.rs`).

Since `band/annulus-host-outer-metered` every closed rim's support
outer boundary is metered in closed form behind the screen, so for
those faces the skip is no longer the only guard; an open link's
support faces and every support's RINGS other than circles still lean
on it (rings refuse in `ring_circle` when not one circle).

Found by the sweep for edges skipped beside the sampled screen
(`band/annulus-host-outer-metered`).

## What the taker owes

Decide per skip: an unresolved face or half-edge is a broken body
(`BodyNotIntact`), and an uncertified carrier or a lone-vertex loop is
either refused typed or argued harmless at the site.

## Measured

Five bare `continue`s, not four: the unresolved loop and the cycle that
does not walk were two. Reachability through `fillet_edges`, which
runs the battery on any `&Body` without a tier-2 gate:

- unresolved face, unresolved half-edge, unresolved loop, cycle that
  does not walk: no public door builds such a body (the arena mutators
  are crate-private), and on any body the half-edge is live because
  the closed walk that returned it read it, and the face because
  `resolve_link` read it. Refused `BodyNotIntact`.
- lone-vertex ring (`mev` into a support face, then `kemr`) and null
  strut (`mev_null` at a support-face corner): both tier-1 valid, both
  reach the screen. Before the fix the strut fell through to predicate
  6 (`UnsupportedCorner`, valence 4) and the ring to the surgery's
  `ring_circle` (`BodyNotIntact`). Each now refuses at the screen as
  `UnsupportedGeometry`, the tag the surgery gives the same reads (the
  ruled cap meter's lone-vertex cycle, every certified-carrier read).
  Rows: `crates/sweep/tests/band_clearance_screen_reads_every_feature.rs`.
