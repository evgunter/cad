---
id: a-tangent-edge-printed-as-a-crossing-line-refuses-at-the-build
kind: issue
title: step-import: a tangent edge the writer printed as a crossing line √(2·r·gap) from the ruling the snap makes exact refuses at the build
status: open
opened: 2026-11-19
priority: P2
---


Found by `importgate/snap`, which makes `nist_ftc_09`'s tangencies exact
(D7 step 3, `crates/step-import/src/snap.rs`).

## What

A writer that states a plane biting a cylinder by a gap `g` inside its
ε_in (the pair tangent at the file's band) prints the edge between
them as one of the pair's two crossing lines as it wrote them. That line
lies on both surfaces within `g`, but `√(2·r·g)` from the tangent ruling
across the tangency, and the position across a tangency is
square-root-conditioned. Once the snap makes the tangency exact, the
build (`crates/step-import/src/build.rs`, `tangent` then
`on_description`) compares the printed carrier with the built ruling
(`step_edge_on_description`) and reads it off in or past the file's band.

`tests/fixtures/wild/nist/nist_ftc_09_asme1_rd.stp`, ε_in = 33.8 µm, its
band (33.8 µm, 338 µm), the same at ε 1e-9, 1e-6 and 1e-12 (pinned by
`snap.rs`'s `nist09s_five_cylinders_snap_and_its_printed_tangent_edges_are_read_off_them`):

| edge | cylinder | r (mm) | gap g (µm) | printed line off the ruling | reading |
|---|---|---|---|---|---|
| `#4950` | `#4962` | 1.4166 | 11.5 | 180 µm | in band |
| `#4964` | `#4962` | 1.4166 | 5.3 | 123 µm | in band |
| `#4978` | `#4990` | 4.5467 | 13.0 | 344 µm | past the band |
| `#5090` | `#5102` | 1.4104 | 4.7 | 115 µm | in band |
| `#5233` | `#5231` | 3.5171 | 8.3 | 233 µm | in band |
| `#5338` | `#5336` | 0.4146 | 1.45 | ≈ 35 µm | builds |

`tests/tier_gate.rs`'s `NIST09_PRINTED_OFF_ITS_TANGENCY` pins `#4950`.

## The question this leaves (for Ev)

D7 (`docs/DESIGN.md` `[import]`): "Healing may move geometry by up to
O(ε_in) to make the chosen interpretation true". The snap's surface
moves are O(ε_in). Building the tangent edge where the made-exact
description puts it moves the edge by `√(2·r·g)`, which is not O(ε_in).

- **(a) As now.** A printed carrier not within ε_in of the description
  it states is the file's own inconsistency, refused. NIST09 refuses.
- **(b) Read a tangent edge's printed carrier against its two
  surfaces** (its distance from each, at the file's band) rather than
  against the built ruling, and report the edge's move across the
  tangency on the ledger. The surfaces stay within ε_in. The edge moves
  `√(2·r·g)` and the vertices with it, which D7's O(ε_in) bound would
  have to admit for a tangent description.

## What would close it

A ruling between (a) and (b). Under (b), the build's tangent arm reads
the printed carrier on its surfaces and the vertex readings follow.
Under (a), this row closes as the file's, with `nist_ftc_09`'s row.

## 2026-11-23 (GERM): `nist_ftc_09` no longer reaches these edges; a synthetic row witnesses them

Since PR 7171 the snap refuses a pair it reads in the file's band. In this
file, the cylinder `#5130` between parallel planes `#5116` and `#5144`
bites `#5116` by 56.8 µm (edge `#5118`), which is in the file's band;
`#5130` × `#5144` (edge `#5132`) is in band too. The snap refuses at `#5130`
(`step_snap_relation`) before any tangency is made exact, and the build
reads the same gap at `#5118` (`step_edge_on_description`, `Ambiguous`).
So `nist_ftc_09` refuses under (a) and (b) alike. The table above was
measured before that and stays true of the file. Its pins are re-aimed at
`#5130` (`work/germ/main-red-axis-normal-snap-refuses-nist-ftc-09.md`).

The in-tree witness of question (b) is now
`a_tangent_edge_printed_as_a_crossing_line_is_read_off_its_ruling`
(`crates/step-import/tests/a_relation_the_file_band_reads_zero_is_snapped.rs`).
It takes the unit fillet (r = 1), states its radius `g` long and prints the
edge on `x = 2` as a crossing line, `d = √(2rg + g²)` along the plane. A
1 or 2 nm bite refuses `MissInBand { step_edge_on_description }` at that
line edge, with a miss of `d` within `g`. A 2 µm bite refuses
`EvidenceMiss`. This holds at ε unset, 1e-6 and 1e-12. Under (b) that row
would import.
