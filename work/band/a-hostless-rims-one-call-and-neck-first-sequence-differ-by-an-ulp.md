---
id: a-hostless-rims-one-call-and-neck-first-sequence-differ-by-an-ulp
kind: issue
title: On the one-band lantern the hostless neck and the shoulder carved in one call and carved neck-first sequentially differ by one ulp of volume
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
---


Found by the LATHE one-band lane (2026-11-23, PR 7414), measured on its
branch `lathe/full-revolve-one-band`.

## What

`sweep` `fillet_h5_hostless_rim::a_hostless_rim_composes_with_a_shared_wall_neighbour`
pins that the repaired lantern's neck `(1, 0)` and shoulder `(0.8, 0.6)`
carved in ONE call is the sequential composition bit for bit, in both
orders. With a full revolve's curved walls one band each, the census,
face order and surface kinds of the one-call body and the neck-first
sequential body agree (6 faces, 9 edges, 5 vertices), but their volumes
differ by one ulp: `4612097951860993249` against `4612097951860993250`
(`f64::to_bits`). Before, the shoulder's plan re-read seam meridians the
neck's carve had split (`refresh_annulus_seams`); now the rims cross the
sphere wall's one seam, and the second carve reads a wall the first
already cut at that crossing, so some stored datum differs at rounding
between the two orders.

The row stays red on PR 7414 rather than being loosened: either the
one-call plan and the sequence are made one computation again, or the
claim is re-ruled.

## Ground

`crates/sweep/src/blend/` (`surgery.rs`, the annulus rim plan): band.
