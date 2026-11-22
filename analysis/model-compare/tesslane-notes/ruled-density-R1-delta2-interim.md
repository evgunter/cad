INTERIM (second delta, PR 7096 @ 8ad2d3fd0f; base = merged main 54e6a5c5b8) — about 45 min in, continuing.

Verdict so far: APPROVE-WITH-FIXES. No new MAJOR of the walk's own.

- My whole oracle on the integrated head, default ε (census, polylines, analytic winding, sampled deviation, volume, count relation): cones 14,635 meshed, frusta/walls 31,115, bores 1,566, bands/D-prisms 1,386. Zero census, polyline, winding, deviation or volume failures; no walked body reaches `OverusedSide` or `UnclearedSharedSide`; the only count departures are the cut cells (bore 504, band 180).
- Against merged main on 19,426 common cells (four coarsest δ per body; main's lattice is too heavy for the rest): 18,884 manifold on both; 264 cells main refuses and the head meshes (`Triangulation` 252, `UnclearedSharedSide` 12).
- **12 cells mesh on main and refuse on the head** (A3). All are the thin D-prism family the orchestrator ruled "pin and state": D-prism 0.5 rad at δ 0.1 and 0.2 rad at δ 0.03, EVERY line edge split (six split patterns each). The refusing face is a planar cap (`UnresolvedWinding` on the collinear triple chord-end, split point, chord-end; `Triangulation` at five points), not the walked face.
- **Correction to the description's account of them:** "exactly as the unsplit body refuses on both trees" is not so on this head. The unsplit D-prism meshes on both trees at every δ I ran (12 triangles at δ 1 to 0.1). What refuses on both trees is the body with only a cap's straight chord split. Main meshes the all-edges-split body because the headroom chords the rim finer.
- Denser than main in 440 of those cells: the bore cells at δ at or past the shell thickness (+2 triangles each at R 1.05: 62→64, 78→80, 102→104), and 24 D-prism cells (+2, the flat face holding both sides). Totals over the common cells: 9.76 M triangles on main, 0.80 M on the head.
- BY-INSPECTION: `faces_holding_side` is main's inline test verbatim in meaning; nine `RouterDefect` sites, each with its face, none left as `MissingEntity "(router defect)"`. `walk_face`'s doc still describes the centroid form and lists `MissingEntity` (the brief says docs are landing).

Still to run: per-patch digests against merged main, the three ε, mutants (cut off → census; zip; route; re-split), hosted CI.
