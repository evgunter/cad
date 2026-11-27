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
orders. With a full revolve's curved walls one band each, the census
of the one-call body and the neck-first sequential body agree (6 faces,
9 edges, 5 vertices), but their volumes differ by one ulp:
`4612097951860993249` against `4612097951860993250` (`f64::to_bits`).

Measured (2026-11-25, both bodies dumped with `{:#?}` and compared):
**no stored number differs.** The 346 floats of the two bodies are the
same multiset bit for bit; what differs is which arena slot holds
which record. The two torus bands occupy swapped face slots (face slots
1–4 read the same plane, cone, sphere and plane surfaces in both, and
slots 5 and 6 hold the neck's band and the shoulder's in opposite
orders), so the `TangentIntersection` records of their trimlines and
their surface keys sit at different keys too, which is the difference
the PR 7414 review read. The volume sums the faces' flux terms in face
slot order, so it sums the same terms in a different order. The order
reaches the mesh too: the PR's re-review (R2) measured 175 of the 494
mesh positions of the two bodies differing, by at most `6.8e-16`, so the
two compositions agree in what they store and differ at rounding in
what is computed from them in slot order, the volume and the mesh.

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
summed in is incidental (`memories/output-stability-as-justification.md`),
and the measurement above shows the stored records are the same; what
is computed from them in slot order (the volume, the mesh positions)
differs at rounding.
On PR 7414 the three pins (`fillet_h5_hostless_rim::a_hostless_rim_composes_with_a_shared_wall_neighbour`,
`blend_tworims::a_lantern_rim_pair_on_a_shared_wall_composes_in_one_call`,
`blend_tworims::three_chained_shared_wall_rims_carve_in_one_call`) are
re-pinned to cells equal (where the row read cells) and volume within
`4·ε` relative, with the reason in-file.

- 2026-11-23 — closed by the LATHE one-band lane on PR 7414: re-pinned to what is promised.
- 2026-11-25 — the differing datum measured: none stored; the two bands' face slots are swapped, so what is computed in slot order differs at rounding (the volume by one ulp, 175 of 494 mesh positions by at most 6.8e-16, R2).
