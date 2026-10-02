---
id: face-clearance-screen-skips-boundary-edges-it-cannot-read
kind: issue
title: blend: predicate 2's boundary-pair screen silently skips a face, a lone-vertex loop or an edge whose carrier does not certify
status: open
opened: 2026-10-01
priority: P2
cost: E
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
