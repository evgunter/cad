---
id: ssi-march-short-of-fit-counts-in-band-gaps-as-within-the-tolerance
kind: issue
title: ssi: MarchShortOfFit counts gaps that fall in the band as within the tolerance
status: closed
closed: 2026-10-26
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

## Closed (2026-10-26, tracker sweep)

**Fixed by the sweep (text only).** The count is what the field docs
say (`SsiError::MarchShortOfFit::in_band` and
`RefineStop::NothingToHalve::in_band`, "Gaps half of which falls in the
band"; `crates/geom-brep/src/ssi.rs` ~:664, `ssi/refine.rs` ~:70), so the
three renders that called it "within the tolerance" now say "in the
band": `crates/geom-brep/src/ssi.rs` :1120 (`MarchShortOfFit`), :1171
and :1185 (`RefinementExhausted`'s two `NothingToHalve` arms). No test
pins those strings (`git grep 'not settling where'` finds only the
three renders); the orchestrator's build verifies it.
