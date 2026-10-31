---
id: band
kind: program
title: BAND — the blend and rim bands: what a roll builds over a rim, and what it may call the result
status: ready
opened: 2026-09-20
group: kernel/sweep/blend
prefix: band/
tag: (BAND orchestrator)
ab_band: 7800-7899
paths: [crates/sweep/src/blend/*, crates/sweep/src/fillet.rs, crates/sweep/src/chamfer.rs]
keep_out: [opened by CARVE's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - CARVE measured 73 budget points and was cut into three tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are CARVE BAND STRUT and they share crates/sweep by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled rather than drawing a fence, TANG owns the declared-tangency lane and ISO the rational carriers, topo/src is TOPO's and its cut siblings', split 2026-10-26 along its priority seam (work/README.md Track size) when it measured about 100 points - BAND keeps the P2 slate and the P4 residue, its P3 tail went to BANDRULE (shapes the band-feature rule would build) BANDREACH (predicate 2's reach and screen) and BANDMETER (the surgery's ring cap and strip meters), with the chain-walk and corner readers to CARVEREST, and the four share crates/sweep/src/blend by design]
---
The band a roll builds over a rim, and the record it leaves. Its
opening P0 and P1 rows are built (the D-hole crease, the slit name, the
subdivided side, the cusp record, the recourse tables), and so is most
of Ev's band-feature rule (PR 4963: a blend's result is `S − R` convex,
`S ∪ F` concave): a bore or a bar between a band's caps, a blind bore or
pin on one cap, a dimple or boss on a support's strip.

What is left:
- **P2.** Step 4 of the band-feature rule, whole shells inside the region
  and the seal of a hole whose mouth the fill covers, sharing
  `topo::insert_void`'s bookkeeping
  (`a-concave-fill-and-a-whole-shell-inside-a-bands-region`, cost H);
  the Interval refusal at a curved rim's anti-seam
  (`a-rim-split-at-its-carriers-anti-seam-refuses-at-interval`); two
  cheap error-propagation rows in `blend/arms.rs`
  (`blend-arms-perp-unit-…`, `plane-sphere-blend-spine-radius-…`); and
  the knife-edge wedge's `1 + n_a·n_b` in `plane_plane_blend`
  (`a-knife-edge-plane-blend-divides-by-one-plus-the-normals-dot`,
  reachability unprobed).
- **P3, four cheap rows beside step 4.** The surgery's ring walks and
  half-edge reads take a ring's members and an edge's half by a walk
  nothing proves (`blend-rim-and-seam-walks-…`,
  `unclaimed-half-edge-…`), the empty request returns its input
  (`a-blend-of-no-edges-returns-its-input`, `design`), and test repairs
  a run-wall sweep made no-ops (`swept-operand-merge-repairs-left-in-tests`).
- **P4.** Prose and style residue.

The P3 tail of conservative refusals and wide meters left on
2026-10-26 for BANDRULE, BANDREACH and BANDMETER (`work/band/log.md`).

Charter and order: `work/band/plan.md`; narrative in `work/band/log.md`.
