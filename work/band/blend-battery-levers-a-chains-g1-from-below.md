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

## Read again, and the G1 converted, by `oneround/reach-from-the-read-point` (PR 7661)

- **`chain_g1`'s arm is converted.** On `main` at `093205aa3`, `chain_turns` read `a.arm_len.min(b.arm_len)`, the sampled diameter from below, for a datum read at the shared vertex whose Zero is acted on.
  - It now reads `junction_arm`: the nearer of the two links' reaches from the junction's point, each from above (`geom_brep::span_reach_bounds(..).above()`, the cover's far distance). This is sound, and as tight as the cover.
  - The closed single link reads the same from its start vertex.
  - Row: `blend::battery::junction_arm_rows`, a hook-shaped spline link whose farthest point from the junction falls between samples. It is red on the sampled diameter, on the samples' reach from below, and on the farther link.
- **Left: `Link::arm_len` itself.** It still holds `extent_of`'s sampled diameter.
  - `cap_transverse` reads it at a vertex and acts on both signs (Zero a circle, a definite sign an ellipse), so it is owed both bounds of the link's reach from that vertex.
  - The ruled pair's co-ruling reads the same `extent`, and its direction was not read here.
  - The tight answer is for `Link` to hold its region, or a reach from each end, which changes what the record stores.
