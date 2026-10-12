---
id: a-minted-pair-is-decided-per-op-with-no-cutter-field
kind: issue
title: The minted pair is decided per op with no cutter field: revisit the design when the lint's false alarms on one-face boolean cuts get noisy or a second multi-surface op appears
status: open
opened: 2026-10-12
priority: P4
cost: M
design: true
---


## What

Ev ruled on `[ev]` PR 4749 (2026-10-12): a pair of cells an op mints on a
recorded touch, meeting at a point, cites the record alone in the split
(one plane), takes the boolean's own decision at its door, and refuses in
any other op. The designers' converged cutter field — naming on each
lineage row the surface that minted the cell, so a pair one tool face cut
cites the record alone even in the boolean — is deferred: "yes, it sounds
good to defer this until needed (and revisit this design conversation
then)".

Trigger to revisit (as a design conversation, not a build): the
`unproven-coincidence` lint's false alarms on one-face boolean cuts prove
noisy, or a second op that cuts with more than one surface needs to mint
such a pair. The PR body and fork log row 117 hold both designers'
reports.
