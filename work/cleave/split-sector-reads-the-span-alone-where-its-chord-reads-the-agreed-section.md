---
id: split-sector-reads-the-span-alone-where-its-chord-reads-the-agreed-section
kind: issue
title: the split's rule (a) reads a cylinder wall's reach over the span alone while its chords read the agreed section
status: open
opened: 2026-10-08
priority: P4
cost: E
---

Found by TANG PR 4292's second review (S5).

## What

The split reads a cylinder wall two ways:
- `splitting::rules::apply_rule_a` (and `wall_graze`) decide the sector rows (`split_sector_coplanar`, `tangent_sector_osculation`, `bends_into_material`, `enters_material`) at `face_extent`, which reads the wall's edges over their spans alone (`Reach::span_reach_from`).
- The chord the split then mints through that wall reads the plane×cylinder section through `chord_join::wall_section`, which serves only where the span-bounded and whole-turn reaches agree (`chord_join::agreed_section`). `wall_section` is lane-neutral, and the Boolean's germ join reads it on the declared-tangency path.

So a sector rule (a) reads as grazed, or as definitely not coplanar, can still escalate at its chord with `pc_axis_plane_parallel_disagreement`. Nothing is served wrong, but the split refuses later, at a different row than the one that decided.

## The shape of a fix

This goes away when the agreed reading does: `germ-takes-the-span-bounded-face-reach-alone` (TANG, parked on D10's declared path) lets a section read where no declaration is carried take the span alone. Until then, either:
- rule (a) reads the agreed reading too, giving up the split rows' served-where-main-escalated (1,582 plane and 1,011 cylinder samples in PR 4292's differential); or
- the split lane passes `wall_section` a lane flag. The review ruled the second out ("wall_section stays lane-neutral").
