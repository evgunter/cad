---
id: the-torus-floor-row-and-across-have-no-reader
kind: issue
title: geom-brep's TT_COINCIDENT_FLOOR and TorusDisplacement::across have no reader, and the dimension audit cites the retired declared_reading
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by `a-reach-straddle-goes-through-the-straddle-door`'s sweep (SEAL).

- **Two dead public items.** `geom_brep::TT_COINCIDENT_FLOOR` (`pair_section/torus_coincidence.rs`, re-exported in `pair_section.rs` and `lib.rs`) and the `TorusDisplacement::across` field are read nowhere in the workspace, demos and tools included (`grep -rn "TT_COINCIDENT_FLOOR\|tt_coincident_floor"`; `across` is only written, in `TorusDisplacement::of`). Their one reader was the declared pair's floor reading, which has gone. Delete both, or name the reader they wait on.
- **The dimension audit cites a retired function.** `docs/predicate-dimension-audit.md`'s two `boolean/carrier_eq.rs (\`declared_reading\`)` rows name a function that no longer exists. The reach row's margins are now read by `coincident_over_extent` (with the straddle rows `bool_plane_reach_straddle`, `carrier_*_reach_straddle`), and the floor row (`*_reach_floor`, `tt_coincident_floor`) has no site left.
- **The torus witness has no audit row.** `boolean/carrier_pair.rs` `torus_pair_witnessed` decides `carrier_torus_witness` (a point's distance to a torus, in metres, `Margin::of`) and mints `carrier_torus_witness_straddle`. Neither name appears in the audit.

