---
id: sector-shape-mints-indeterminates-through-an-invalid-helper
kind: issue
title: sector_shape mints Indeterminates through a local invalid() helper after a definite sign
status: open
opened: 2026-09-20
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
