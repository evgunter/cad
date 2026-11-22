(4c implementer) Note (2): the merge compiles. Note (b) follows. The merge is committed locally (ade737b13a) and not pushed, because one row needs restating first (below). I'll push once it is green.

**The merge:**
- RouterDefect is gone (variant, Display, tag, census entry, projection). The walk's two sites are main's `MissingEntity { what: "… (router defect)" }`.
- One test of "a side one face across holds both ends": I lifted main's inline check in `TrimmedFace::emit` into `TrimPolygon::faces_holding_side(across, (places_a, a), (places_b, b)) -> Vec<FaceKey>`. That is main's predicate verbatim: not loop neighbours by polygon places, and one face in both of main's `faces_across` lists. emit reads `!…is_empty()`, so the CDT arms are unchanged. The walk reads the same function and calls `may_make` on each face it returns. My mesh-id `beside` set is gone, replaced by main's places.
- The tessellate table gets `Entry::Walked`, and the wedge row reads 28 (main 38).
- split_rulings, route_rows, errors, reach_probes, side_census, ruled_walk, sizing: 94/95 pass. The one red is the bored-ball row: its baseline is the unsplit ball, which main now refuses with `OverusedSide` at the coarse cells.

**(b) Walk merged, its cut OFF** (`&& false` on the cut), default ε:
- **Bored ball** (both splits): `OverusedSide { uses: 4 }` on exactly sign +1 at δ 3, 1, 0.3 and sign −1 at δ 3, 1, 0.3, 0.1. That is your reviewers' list, and the same cells where the unsplit ball refuses on main. With the cut off, δ 0.1 to 0.03 for sign +1 and δ 0.05 to 0.03 for sign −1 mesh at the count relation (sign +1 split@0.5: 134, 188, 212, 298, 322).
- **Every other row I hold passes with the cut off:** the 218-cell `no_walked_triangle_has_an_edge_a_neighbour_makes`, every split_rulings row, reach_probes, route_rows. So no cell outside the bored ball needs a cut, and the 24 band cells' +4 are over-cuts.
- I don't hold the reviewers' bored torus, cone-bored ball or bead. On my bodies:
  - needed: 14 cells (bored ball, 7 per split variant);
  - over-cut: 10 bored-ball cells (sign +1 at δ 0.1, 0.05, 0.04; sign −1 at δ 0.05, 0.04; ×2 splits; e.g. sign +1 split@0.5 δ 0.1: 220 with the cut against 134 without) plus 24 band cells (+4 each).
- "Walk's cut off" mutant: red by `OverusedSide` as required.

**Next, the restated rows:**
- `holds_the_relation` takes the number of sides cut per cell (`Walked::Cut(k)`: surplus exactly 2k on the walked face; `Exact` = 0). The bored-ball row and the band cells pin k per cell, so a cut the rule does not ask for is red. "Always cut" will red the 218-cell row and the bored ball's exact cells.
- Where main refuses the unsplit body (the bored ball's 7 coarse cells), the walked face's baseline comes from Euler on its own patch: T = B + 2I − 2, with I = k cut points. The neighbour is checked by check_mesh plus the geometry read.
Then the review items, then the runs.
