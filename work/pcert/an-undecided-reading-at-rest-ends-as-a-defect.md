---
id: an-undecided-reading-at-rest-ends-as-a-defect
kind: issue
title: An undecided chart reading at rest ends in the defect ending (ChartWinding, ImageIdentity, Sector), against DESIGN's at-rest sentence
status: open
opened: 2026-10-27
priority: P2
cost: M
design: true
refs: [who-placed-it-is-read-off-the-description-not-the-door]
---


## Finding

Raised by the designer pair on `who-placed-it-is-read-off-the-description-not-the-door`. Both designers verified it against main `86589d04`. It is off that fork's question for every check except incidence.

`PcurveCheck::routing` (`crates/geom-brep/src/pcurve_cache.rs`, the arm naming the placement integer and the stored copies) routes `ChartWinding`, `ImageIdentity`, `ChartIncidence` and `Sector` to `Routing::Unsized(Unsized::Defect)`.

`Unsized::recourse` (`crates/geom-brep/src/recourse.rs`) ends every arm of `Unsized::Defect` in the defect ending, the undecided arm included, at every door. So an undecided reading of any of these four at rest says "a kernel defect or a damaged file".

DESIGN `[verdicts.consumers]` says: "At rest, only a definite contradiction of what is stored is a defect." It also says that in band refuses with the decision's own lever.

Incidence is settled by the `[ev]` fork on who placed a carrier: an in-band read ends in the lever at every door. `ChartWinding`, `ImageIdentity` and `Sector` keep the conflict.

## The question

For an exact check of a stored copy, what does an undecided reading at rest end in? There are three readings:
- the decision's lever, as DESIGN's sentence reads;
- the last resort;
- or the defect ending stands, because the kernel wrote the copy exactly and an undecided re-read of its own exact write is the kernel's. In that case DESIGN's sentence needs its exception stated.
