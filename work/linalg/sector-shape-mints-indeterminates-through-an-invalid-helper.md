---
id: sector-shape-mints-indeterminates-through-an-invalid-helper
kind: issue
title: sector_shape mints Indeterminates through a local invalid() helper after a definite sign
status: closed
opened: 2026-09-20
priority: P0
cost: M
closed: 2026-10-01
---


## What

`crates/topo/src/sector_shape.rs` builds `Indeterminate`s of its own
after a DEFINITE sign, through a file-local
`fn invalid(band, predicate) -> SectorFault` helper rather than a struct
literal — so the escalation its caller receives is on no frame's
escalation log, and the spelling census PR 2928 added over `geom-brep`
would not see it even if it were pointed at this crate.

Two shipped sites, both in the sector ladder: the collapsed-arm gate
(`Ok(_) => return Err(invalid(band, SECTOR_ARM))`) and the straightness
gate (`Ok(Sign::Positive | Sign::Zero) => return Err(invalid(band,
SECTOR_STRAIGHT))`). Both are exactly the shape PR 2928 retired at its
own eight sites: ask the funnel, receive a definite answer the predicate
cannot use, mint the refusal by hand.

## Shape

`geom_core::k_stats::decide_positive` is the door for the first.
`SECTOR_STRAIGHT`'s admissible set is "definitely negative", which is
`decide_positive` with the margin negated or a third gate; decide which
at the site rather than by adding a door speculatively. The helper can
stay — it is the `SectorFault` wrapper, and wrapping is fine; what has
to move is where the `Indeterminate` inside it comes from.

## Why it is filed rather than fixed

PR 2928 owned the eight `geom-brep` sites its item named. This is the
same class one crate over, found by sweeping for the SHAPE rather than
the spelling; `work/curved/` carries the rest of `topo`'s instances,
and this file carries the two on ground PROPS owns.

## Closed (2026-10-01)

Closed by #3686. Both gates now go through topo's funnel wrappers. A
new door, `geom_core::k_stats::decide_negative`, mirrors
`decide_positive` and keeps the straightness margin's sign,
`cos θ × arm`. That sign matters: `refusal_routes`' recourse table
reads it, and so do the committed K data.

The review caught an earlier spelling that negated the margin and
inverted the Boolean's tighten advice. A polarity row now pins it.
The escalation now reaches the frame's log. The accept/refuse
partition is unchanged across 8,652 cases.

The same class elsewhere is filed on its owners' slates (see the PR
body's sweep table). The funnel bypass is
`work/restfront/topo-calls-k-stats-past-its-own-classification-funnel.md`.
