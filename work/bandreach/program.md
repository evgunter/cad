---
id: bandreach
kind: program
title: BANDREACH — predicate 2 reading wider than it needs: the blend's reach meter and its support screen
status: ready
opened: 2026-10-26
group: kernel/sweep/blend
prefix: bandreach/
tag: (BANDREACH orchestrator)
ab_band: 17900-17999
paths: [crates/sweep/src/blend/reach.rs, crates/sweep/src/blend/battery.rs]
keep_out: [opened 2026-10-26 by BAND's priority-seam split (work/README.md Track size) when BAND measured about 100 points against 30 - BAND keeps the P2 slate and the P4 residue, BANDRULE the shapes the band-feature rule would build, BANDMETER the surgery's ring cap and strip meters, all four share crates/sweep/src/blend by design and CARVEREST holds battery.rs's chain walk and corner readers, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

**Predicate 2's over-refusals, P3.** Predicate 2 asks whether a
band's material meets any face other than its supports. It is read
twice: by the support screen in `battery.rs` (sampled gaps along each
requested edge) and by the reach meter in `reach.rs` (each other
face's cells against the band's reach). Every row here is a place
where one of them reads wider than the geometry: it refuses what
clears, or excuses a pair it never meters:

- **The reach.** A NURBS face metered on its box alone, an open arc
  link's reach taken over the whole turn, a bore clear of a ruled
  cut-off's sliver, an oblique cell past a window's end that the
  cross-chain excuse misses, and two bands of one chain never metered
  against each other.
- **The screen.** It reads a support's vertices and not its edges. It
  refuses a feature across the requested edge's line, where the band
  never reaches. It skips adjacent boundary features wherever they
  approach.

Charter and order: `work/bandreach/plan.md`; narrative in
`work/bandreach/log.md`.
