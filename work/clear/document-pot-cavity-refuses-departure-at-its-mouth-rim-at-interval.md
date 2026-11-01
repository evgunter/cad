---
id: document-pot-cavity-refuses-departure-at-its-mouth-rim-at-interval
kind: issue
title: the editor-core corpus pot's cavity refuses Departure at its mouth rim at Interval, though the same pot built in sweep shells Ok, so SHELL-4's gate over-refuses a valid document body
status: closed
opened: 2026-10-31
priority: P1
cost: M
closed: 2026-11-01
---


## Finding

SHELL-4's lane found this on resume, on `clear/shell4` at `3a6c20799f`,
which has main merged in after FRUSTUM, SPIRIC and EPSROWS.

- **The body.** The pot in editor-core's corpus `vessel::document()`,
  evaluated at `Interval`
  (`lib_g17_shell_node::both_documents_evaluate_green_at_the_interval_scalar`).
- **The operand** certifies: Holds, `cornered` 0, refused 0.
- **The shell refuses**, sealed or opened:
  `CavityUncertified { Uncertified { a: 2v1, b: 6v1, shared: Edge(12v1), arm: Departure } }`.
  - `2v1` is the mouth plane, `y = 1/8`.
  - `6v1` is a sphere belly band, centre `y = 1/16`, `r = 5/64`.
  - `Edge(12v1)` is the mouth rim circle, `r = 3/64`.
  - All the geometry is exact and dyadic.
- **The same pot built in sweep shells Ok** at `Interval` and at
  `Sym<Interval>`. That is CAVITYDIAG's `corpus_pot`, which uses
  `bulge_from_center` and a sweep-side revolve.

So the refusal depends on how the document builds the pot, not on its
surfaces. The candidates are the `ProgramArcData::Center` arc and the
document's revolve: where the seam sits, how the band is split, or which
way it is oriented.

Over-refusing a valid shell is ruled out (SHELL-4's ruling), so SHELL-4
is parked on this row.

## Diagnosis

The readings part at **the presentation the departure arm takes**. The
two pots are the same B-rep: the same faces, edges, charts, seams,
senses and pcurve forms, the band split the same way. They differ only
in width. The document's `ProgramArcData::Center` lands the belly
sphere exactly (`c = (0, 1/16)`, `r = 5/64`, point intervals), and
`bulge_from_center` carries it about `1e-16` wide.

The refusal is at the cavity's mouth-rim corner on a band half, where
the seam meets the rim. The cavity is the operand moved a wall of
`1/128` inward: its sphere has radius `9/128` about `(0, 1/16)` and its
mouth plane is `y = 15/128`, so its rim is at `v = asin 7/9`. (The
operand's own rim, at `y = 1/8` on `r = 5/64`, is at `asin 4/5`.) `present_chart` clips the
corner cell to the seam line `u = 2π` and the rim line, and then asks
that the seam's `v` range cover the rectangle's side. That side ends
at the rim line's outer bound, while the seam's range ends at its own
end enclosure: these are two enclosures of the one corner vertex,
rounded by two routes.

| build | seam end `hi` | rim line `hi` | gap | `few_ulps` |
|---|---|---|---|---|
| document | `0.891122507886654` | `0.8911225078866563` | `2.3e-15` | `1.8e-15` |
| sweep | `0.8911225078866712` | `0.8911225078866714` | `2e-16` | `1.8e-15` |

Sweep's wide sphere dominates both roundings, so its gap falls inside
`covered`'s concession. The exact sphere exposes the route difference.
No cell narrows either enclosure, so the corner cell reads `Local` at
every depth and the pair refuses `Uncertified { Departure }`.

The fix is in the reader (`injective.rs`, `run_on`, read only by `sides_are_the_faces`): a clip side's
edge that ends at a corner it shares with a clip on the other axis runs
on from its end's own enclosure to that line's far bound. No producer
changes.

