---
id: band
kind: program
title: BAND — the blend and rim bands: what a roll builds over a rim, and what it may call the result
status: ready
opened: 2026-09-20
area: kernel
prefix: band/
tag: (BAND orchestrator)
ab_band: 7800-7899
paths: [crates/sweep/src/blend/*, crates/sweep/src/fillet.rs, crates/sweep/src/chamfer.rs]
keep_out: [opened by CARVE's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - CARVE measured 73 budget points and was cut into three tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are CARVE BAND STRUT and they share crates/sweep by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, TANG owns the declared-tangency lane and ISO the rational carriers, topo/src is TOPO's and its cut siblings']
priority: P2
---
The band a roll builds over a rim, and the record it leaves. Its
opening P0 and P1 rows are built (the D-hole crease, the slit name, the
subdivided side, the cusp record, the recourse tables), and so is most
of Ev's band-feature rule (PR 4963: a blend's result is `S − R` convex,
`S ∪ F` concave): a bore or a bar between a band's caps, a blind bore or
pin on one cap, a dimple or boss on a support's strip.

What is left, by band:
- **P2.** Step 4 of the band-feature rule, whole shells inside the region
  and the seal of a hole whose mouth the fill covers, sharing
  `topo::insert_void`'s bookkeeping
  (`a-concave-fill-and-a-whole-shell-inside-a-bands-region`, cost H);
  the Interval refusal at a curved rim's anti-seam
  (`a-rim-split-at-its-carriers-anti-seam-refuses-at-interval`); and
  three cheap error-propagation rows in `blend/arms.rs` and
  `battery.rs` (`blend-arms-perp-unit-…`, `fillet3-chain-arm-…`,
  `plane-sphere-blend-spine-radius-…`).
- **P3.** Conservative refusals the rule would build (cups the readings
  cannot certify: leaning, ruled, overrun, not disk or annulus) and
  meters that read wider than they need, each with its own item.
- **P4.** Prose and style residue.

Charter and order: `work/band/plan.md`; narrative in `work/band/log.md`.
