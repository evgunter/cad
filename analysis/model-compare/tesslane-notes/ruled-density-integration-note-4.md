(4c implementer) **826552a0e9 is pushed: RouterDefect is gone.**
- No diff from origin/main remains in: `types.rs`, `walk.rs` (side_of's signature restored), `tests/errors.rs`, all of `crates/pncad-py` (tag, tests list, binding census, projection), and `work/tesslie/` (the dated line is out).
- curved.rs's and chords.rs's router-defect sites are main's text again. curved.rs's two unit rows pin `MissingEntity` again. The only remaining diffs in those two files are this unit's: the wedge's 28 and chords' `Lane::Walked` arm.

The walk's two sites are now `unreachable!`, each with its row-0 answer written at the site:
- **Spline chart:** row 0 does not apply locally. Whether a face's row is analytic is `ChartRow::of`'s run-time match on the surface kind, the constructor every arm shares. A walked row by type would change it for all of them. So it is row 4. The message names the chain: the route walks only where `splits_a_straight_side` held, which needs `chart_table::ruled` (cylinder or cone), and their rows are analytic.
- **Polygon not reading as the two rows:** row 0 is no. It is a re-derivation off the polygon the chord pass built after the route ran, and the route cannot hand over a polygon that does not exist yet. So it is row 4. The message says why a correct kernel passes: the same ringless loop in order, chord points only added along edges, the apex decided at the same band. It carries the loop and point counts.
- `walk_face`'s `# Errors` no longer lists a router refusal.

The bore item's neighbours are fixed: sign +1 is the torus (δ 3 to 0.3) and sign −1 the sphere (δ 3 to 0.1), in both places.

Checks on 826552a0e9:
- mesh+pncad-py clippy (all-targets, all-features) is clean; gates rc=0; work lint is ok.
- errors, curved, route_rows, walk, chords, tags, split_rulings and ruled_walk: 134/134.
- `mesh` whole on 469c4b6c54 (the head before this commit), slow set included: 546/546 at the default ε, 1e-6 and 1e-12.

Also on 469c4b6c54 (pushed earlier):
- a pinned D-prism row;
- the undecided-reach row (36 cuts at each ε inside the band window; the "undecided read as greater" mutant gives 0 at all three);
- the K-stream row and its doc at `FaceRoute`;
- the review items.

Next: the digests, with the may_make exclusion count from main's CDT rule; the mutants; the binding census against a wheel; the viewer and sweep rows; then the report.
