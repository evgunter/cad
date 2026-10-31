---
id: blend-battery-levers-a-chains-g1-from-below
kind: issue
title: The blend battery levers a chain's G1 and co-ruling at a sampled extent from below, and a Zero there is acted on
status: open
opened: 2026-10-27
priority: P2
---


`crates/sweep/src/blend/battery.rs` `extent_of` levers what the battery
reads along an edge (`Link::arm_len`: `chain_g1`'s junction tangency,
and a ruled pair's co-ruling) at the farthest pair of
`CHAIN_SAMPLES` points on the edge's carrier. That is a bound from
below on the edge's point-set diameter, and its doc argues the safe
side from it: "every chord lower-bounds arc length, so the lever never
over-reports the edge's extent — a margin in meters folded against it
stays conservative".

`chain_g1` acts on the Zero: `Margin::levered(sin θ, arm)` decided Zero
is the junction accepted as G1 (`Ok(decided.margin)`), and only a
definite sign refuses (`BlendError::ChainNotG1`). At an arm short of the
edges' extent a turn that is Positive at the extent reads a definite
Zero, so the short side is the unsafe one for this reading. The same
sign error was in `geom_brep::edge_extent`'s doc ("a smaller arm only
escalates more") and was corrected by OFFSET's
`edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short`:
that unit split the edge extent into `geom_brep::edge_extent_inf` and
`geom_brep::edge_extent_sup`, and moved every Zero-acting consumer it
found in `topo` to the bound from above.

The sampled pairs are a fair bound from below on a circle or an
ellipse arc; the gap is widest on a spline or a steep conic arc whose
farthest pair falls between samples. The fix is to lever `chain_g1`
(and any other Zero-acting read of `arm_len`) at
`geom_brep::edge_extent_sup` over the link's carrier and interval, keep
the sampled pairs where a reading acts on a definite nonzero sign, and
correct the doc. Found by OFFSET's sweep for the class (the "shorter is
safe" claim on a lever whose Zero is acted on).
