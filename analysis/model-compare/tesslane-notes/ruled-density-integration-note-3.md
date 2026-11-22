(4c implementer) The integrated head 8ad2d3fd0f is pushed. gates rc=0, fmt, workspace clippy (all-features) and pncad-py clippy are clean. split_rulings, errors and route_rows pass at the default ε, 1e-6 and 1e-12.

**RouterDefect** (634dc16100), per your ruling: it is the one router-defect refusal. All 9 sites carry their face:
- curved.rs: ringed face, pinched face, side count;
- chords.rs: no rectangle, uncertified half-edge;
- walk.rs: uncertified half-edge, rim with no row. `side_of` now takes the face from `traversals`, its one caller, which holds it.
- trimmed.rs: the walk's two.
The suffix is dropped. The tag, errors census and display row, binding census and pncad-py tag list follow. curved.rs's two unit rows re-pin by variant. The Python projection rides `what` as the note and drops the face, as every other variant's face does ("the face is a key and does not cross"). A dated line is on the TESSLIE row. It stays open for the cache misses, the absent chart and `LoopEdgesError::Corrupt`.
- `work.py territory --base origin/main`: 16 paths in other programs' territory.
  - Double claims (tesslane claims too): chords, lib, sizing, tessellate, trimmed, types, walk; tests all, errors, reach_probes, split_rulings.
  - Owned elsewhere: .config/nextest.toml (green), pncad-py src/py/mesh.rs (bind, pydoor), tags.rs, tests.rs and tests/test_binding_census.py (pydoor; census also ciw, green).

**Per-cell cut count** (8ad2d3fd0f): `holds_the_relation(…, cuts)`. The split ruled faces' interior points (patch vertices on no edge polyline) must sum to `cuts`. Each such face owes 2 triangles per interior point beyond the relation, and every other face owes exactly the relation. Where main refuses the unsplit body `OverusedSide`, only a cut cell may pass, and the relation is read without the unsplit count. Measured and pinned:
- Bored ball, the same for both splits:
  - sign +1: δ 3: 15, δ 1: 15, δ 0.3: 25, δ 0.1: 43, δ 0.04: 69; 0 at 0.035 and 0.03.
  - sign −1: δ 3, 1, 0.3: 15; δ 0.1: 21; δ 0.04: 16; 0 at 0.035 and 0.03.
- Band (in the 218-cell row): line edge 1 or 2 takes 2 cuts (one split) or 4 (three splits); every line edge takes 3 or 7; at δ 1 and 0.1. Every other cell: 0.
- Correction to what I said earlier: the band's surplus is 4 to 14 triangles a cell, not a flat +4. And the bored ball's cuts are 15 to 69 a cell.
- The pins are the same at all three ε.
- The over-cut is now bounded per cell. Bored ball: sign +1 δ 0.1 (43) and 0.04 (69); sign −1 δ 0.04 (16). Band: 24 cells, 2 to 7. "Always cut" must now red on the exact cells. The mutant run comes in the report.

Next: the review items, then the runs.
