---
id: ssi-march-short-of-fit-counts-in-band-gaps-as-within-the-tolerance
kind: issue
title: ssi: MarchShortOfFit counts gaps that fall in the band as within the tolerance
status: open
opened: 2026-10-11
priority: P3
cost: E
---


## What

`SsiError::MarchShortOfFit` (`crates/geom-brep/src/ssi.rs`) counts
`in_band` gaps, documented as "Gaps half of which falls in the band",
and its Display, with the two sibling arms beside it, renders that
count as "{in_band} within the tolerance". A gap in the band lies
beyond ε, so the words claim a nearness the march did not find.

Read from the tree, not run. The fix is a text change ("in the band");
measure a refusal first to confirm the count is what the field doc says.

Found by the CLEAVE lane sweeping "within tolerance" refusal texts for
the split's `SliverVertex` fix (2026-10-11).
