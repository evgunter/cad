---
id: reflex-corner-edge-in-face-poses-zip-a-ring-parallel-to-its-section-loop
kind: issue
title: At the 315-degree reflex corner, B's section ring runs parallel to A's section loop (SeamOrientation), and 32 edge-in-face poses reach it once joined
status: open
opened: 2026-10-02
priority: P1
cost: H
refs: [reflex-corner-vertex-vertex-sites-refuse-under-a-tilted-cap, locus-matching-moves-frontier-refusals-to-join-desync]
---


(Filed by the `join/pole-strut-binding` lane, the residue of
`locus-matching-moves-frontier-refusals-to-join-desync`. Evidence for
the reflex-corner P0 row, which owns the site class; absorb it there
if that row's diagnosis covers it.)

## What

`crates/sweep/tests/join1_r1_probes.rs` `join1_r1_reflex_battery`,
release, JOIN-1 head `0b6e39ca` against main `3ee0e4b6`: 32 poses
move to `SeamOrientation`, 28 from `Join(UnpairedLooseEnds)` and 4
from `RestZipUnsupported` (the `U` of `sqQ2` at the four movers'
shears). At each of them an edge of `b`'s tilted bottom cap lies in
`a`'s top face `z = 1` (edge-in-face) and ends at, or runs through,
the reflex corner `(0, 0, 1)`; not every such pose moved (`sqQ1` at
`sy = 0` answers soundly):

| profile | shears `(sx, sy)` | ops |
|---|---|---|
| `sqQ2` | `(0, −0.5)`, `(0, −0.25)`, `(0.25, 0)`, `(0.5, 0)` | `I`, `U`, `S_ab` |
| `dLeft` | `(0.25, ±0.25)`, `(0.5, ±0.5)` | `I`, `U`, `S_ab` |
| `eRight` | `(0.25, 0)`, `(0.5, 0)` | `I`, `U`, `S_ab`, `S_ba` |

Main left those edges loose; locus matching joins them, and the
operation then refuses where main already refuses 144 poses of the
same profiles at the neighbouring, non-edge-in-face shears.

## Measured: the ring is parallel, not antiparallel

Raising site `zip.rs` `zip_seam`, the record-keyed alignment: at
`j = 0` a ring half leaves the correspondent of `ob[0]`'s start but
none ends at the correspondent of `ob[n−1]`'s start. Dumped at that
return, `eRight`, `I`:

- `(sx, sy) = (0.25, 0)` (a mover): the A section loop (in `a`'s top
  face) runs `(−1,−0.5) (0,−0.5) (0,0) (0,0.5) (−1,0.5)`, all at
  `z = 1`; B's ring runs through the correspondents in the SAME order.
- `(0.25, −0.25)` (refuses on main too): the loop runs
  `(0,0.5) (−1,0.5) (−1,−0.5) (−0.5,−0.5) (0,0)`; the ring again in
  the same order.

Both loops hold the reflex corner. In both, the B section face is
wound as A's (normal `+z`) where the zip needs it opposed: the
mover's refusal is the same symptom as main's own, which is why the
lane did not treat it as a join defect of locus matching. The order
of the declarations is not the cause: `b ∖ a` refuses the same with
`flush_declarations(b, a)` as with `(a, b)`.

## Not measured

Which side of the pair chose the wrong B face (the B section face
selection, or a sense bound at the reflex vertex), and whether a
convex corner under the same tilts zips. The bar for closing:
`join1_r1_reflex_battery` reports no `SeamOrientation`.
