---
id: nightly-76-reds-greened
kind: unit
title: nightly 76's reds (rustdoc, the full suite's hung sweep row, the viewer app and gui-montage jobs, k-lint dev-probe) are greened, each with its fix row on its owner
status: open
opened: 2026-10-29
priority: P1
cost: M
---


## What

Nightly 76 (run 40823227622, main `5648e577eb`) ran with #5952, #5958
and #5966 in, and was red on five jobs:

- `rustdoc (gate, every root)`: two `topo` links,
  `splitting/neighborhood.rs:122` (`OutwardNormal` unresolved) and
  `sector_shape.rs:80` (a redundant explicit target).
- `full suite (every eps row, slow set included)`: cancelled at its time
  limit while `sweep::all
  the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names`
  ran past 960 s.
- `viewer app feature (eframe + wgpu)` and `render lanes / viewer gui
  montage`: new reds.
- `k-lint (dev-probe)`: still red after #5966's ray-wall pin.

## Owed

GREEN's rule (`program.md`): fix a doc link at its site, re-bless an
explained move, otherwise pin or ignore with a fix row on the owner.
A hang is pinned as an `#[ignore = "disabled red: …"]` with its row,
since it cannot be pinned at an output. Done when nightly 77 is green
on these jobs, or each remaining red is named with the open PR or row
that greens it.
