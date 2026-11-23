---
id: a-hostless-rims-one-call-and-neck-first-sequence-differ-by-an-ulp
kind: issue
title: On the one-band lantern the hostless neck and the shoulder carved in one call and carved neck-first sequentially differ by one ulp of volume
status: closed
opened: 2026-11-23
priority: P2
cost: M
closed: 2026-11-23
pr: 7414
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

## Disposition

Not a defect. The pins guarded bit-identity of the one-call and
sequential volumes, but nothing promises it: `docs/KERNEL-VERBS.md`
states the one-call result is pinned against the sequential composition
"bit-equal where measured so and one integrator summation ulp off where
not", and the sibling row
(`review_fillet_h5_r1_probes`, the boss's hostless rim beside the ladder)
already pins cells equal and volume to the summation order. The two
compositions are one construction; the order the faces' flux terms are
summed in is incidental (`memories/output-stability-as-justification.md`).
On PR 7414 the three pins (`fillet_h5_hostless_rim::a_hostless_rim_composes_with_a_shared_wall_neighbour`,
`blend_tworims::a_lantern_rim_pair_on_a_shared_wall_composes_in_one_call`,
`blend_tworims::three_chained_shared_wall_rims_carve_in_one_call`) are
re-pinned to cells equal (where the row read cells) and volume within
`4·ε` relative, with the reason in-file.

- 2026-11-23 — closed by the LATHE one-band lane on PR 7414: re-pinned to what is promised.
