TESSLANE spline-ring-door FIX PASS report. Head a0c223af4bb96237a87c290ac6faec3c6bad040f, pushed. Main was merged first, at fb811d1562; main has not moved since. No verdict moved and nothing was re-pinned.
    
    ITEMS
    1 TAKEN. holed_sheet now takes a slice of rings. The deck row is renamed `the_ring_door_moves_a_spline_faces_rings_into_its_outer_loops_region`: one face holds both holes, with decks 0 and u=−1. Each ring's expected deck is keyed by where that ring is stored, not by loop order. The doc names the reversed mutant. Mutant: ring_decks' decks reversed → red ("past the seam: the deck").
    2 TAKEN. Re-worded charttail sphere-region row l.63 → `spline_trim::Trim::of`. geom-brep README: the placement stays `topo::boolean::spline_region`, "over the trim and the chart's Bézier cells of `topo::spline_trim`". sweep reach_wall_chord_rows doc → `topo::spline_trim`.
    3 TAKEN. The four "That PR charges…" sentences (charttail, hone, isotail, pcert) now give the address the PR had, then "now `topo/src/spline_trim.rs`". Same for the SPLICE cell-walk row's "copied … into" sentence.
    4 TAKEN.
    - `SplineTrimUnread { ring: Option&lt;LoopKey&gt;, cause: SplineTrimGap }`. `ring` is None for the chart or the outer loop's trim.
    - New pub `SplineTrimGap`, re-exported from `pcurves`: Placeholder, NoCells, Periods, Winds, Moved, NoPieces, NoBox. Moved is your case: a period on a channel the chart states but does not decide closed, or onto the twin sheet.
    - `TrimChart::of` and `chain` now return those causes. `Trim::of` reads them `.ok()?`, so its decisions are unchanged.
    - The README ring row lists every cause.
    - No row reaches `SplineTrimUnread`, as before.
    5 TAKEN. `ChainPiece::ray(q) -&gt; RayQuery` is an associated fn. `ray_parity_as` takes `q` alone and derives the query inside.
    6 TAKEN.
    - New row `a_piece_exactly_the_querys_size_that_meets_it_keeps_it_near`. The query is cut to a quadratic arc's widened box, and the row asserts the two sizes agree bit for bit. The answer is Near(None); halving would clear both halves and read Out.
    - Mutant `&lt;` for `&lt;=` → red, on this row only.
    - The loop doc now says a box exactly the query's size is included, and a point query is size zero, which the stop never ends.
    7 TAKEN, not a pure move.
    - `Caps` and `CELL_VISITS` are back in `boolean/spline_region.rs`. `Trim::of` and `from_pieces` take `piece_visits: usize`, and the `Caps { cells: 0 }` fabrication is gone.
    - The rows pass `Caps::READ.pieces`.
    - To keep the engine leaf, `Caps`, its fields, `READ` and `read_capped` are now private to the engine. The rows reach them as the module's child.
    - engine-leaves: crate-visible spline_region 5→4.
    8 TAKEN.
    - The spline_trim opener says what the module is and names both readers: the boolean's spline arm via `Trim::place`, and the ring door via `ring_decks`.
    - The mutant hook is renamed `SPLINE_TRIM_MUTANT` with `spline-trim-*` sites. I grepped scripts, CI and docs: nothing outside the crate read either name. The rows' eprintln tags `[spline-region-*-drift]` are unchanged.
    - The `ring_decks` doc says the door's lifts are `Lift::read` and the trim's are `read_past_placeholders`. These are one lift at rest, so the deck equals `with_rings`'. On a placeholder the door answers `Gap` first.
    - Each `ChainPiece` impl documents how its shared ends stay bit-identical:
      - analytic: one midpoint parameter on one image, and joint segments copy `Outer::of`'s evaluated row ends;
      - spline: the refined row's shared control point, one de Casteljau midpoint, `close_gap` copies, and one shift arithmetic.
    9 TAKEN. The roster comment now says both doors raise the names: the boolean (`BooleanError` via `ContainError::Escalated`) and the ring door (`RingPlacementError::Undecided`). The entry holds one door, so it names the boolean's.
    
    JUDGEMENT
    - One per-ring loop: DECLINED. `place_ring` is already the one per-ring rule, and each arm hands it a candidate reader.
      - The arms differ in what "the ring's first point" is. The analytic arm moves the T point by exact `Period::times(k)`. The spline arm must read the trim's own f64 chain start, so its deck is the decision `with_rings` makes.
      - Folding them would cost one of those two properties, for about 10 lines saved.
    - The re-exports: TAKEN, as a departure. The loop now lives in its own root module, `crates/topo/src/chain_parity.rs` (`ray_parity_as`, `ChainPiece`, `PieceBox`, `RayQuery`, `gap`). `pcurves` re-exports only what it did before, plus `RingMiss` and `Shift` for the trim.
      - This also takes the loop off CHARTTAIL's `pcurves_rings.rs`.
      - The roster forwarder key is now "chain_parity.rs: ray_parity_as: name".
      - Its `Side` stays in the ring module.
    
    LINES since fb811d1562: code +336/−303; tests and rows +117/−63; work/ 6 files, +6/−6; gate table 1 line.
    - spline_trim.rs +108/−98
    - pcurves_rings.rs +51/−136
    - chain_parity.rs +135 (new)
    - spline_region.rs +26/−4
    - spline_region_rows.rs +110/−58
    - roster +6/−4
    - READMEs and sweep doc: 5 lines
    - engine-leaves.counts: 1 line
    
    RAN on the head
    - run-gates: exit 0.
    - doc gate (`--workspace-pass --skip-viewer-toolkit`): exit 0.
    - fmt --check: clean.
    - clippy -p topo geom-brep mesh sweep, all-targets, all-features, -D warnings: clean.
    - `-p topo` whole: 4233 run, 4230 passed. The 3 fails are main's tracked ones: offer_rows, shell_role_reader needle cavity, and the roster naming only main's three ring_path sites.
    - eps 1e-6 and 1e-12 over spline_region::rows and pcurves::rings: 40/40 each.
    - sweep lofted_wall(s) plus `a_vertex_on_a_curved_face_is_confirmed_by_its_trim`: 8/8.
    - work.py lint: ok.
    - Mutants:
      - decks reversed → deck row red;
      - size stop `&lt;` → size row red;
      - IDENTITY and negated deck: run last pass against the one-ring row and not re-run against the two-ring row. Its per-ring deck asserts subsume them.
    - Left to CI: the hosted gate and other crates.
    
    PR BODY CHANGES
    - Replace "`RingPlacementError::SplineChart` retires. … `SplineTrimUnread` takes its place for the cases where the trim really is not read: …" with: "`RingPlacementError::SplineChart` retires. `SplineTrimUnread { ring, cause }` takes its place where the trim really is not read, naming the ring whose chain it is (`None` for the chart or the outer loop's trim) and the cause (`SplineTrimGap`: a placeholder chart, no Bézier cells, a period stated but not decided, a lift that winds, a row moved by a period on a channel the chart states but does not decide closed or onto the twin sheet, an image with no piece reading, a box with no value)."
    - Replace the "**One parity loop.** `pcurves_rings.rs` `ray_parity_as` …" opening with: "**One parity loop.** `crates/topo/src/chain_parity.rs`, a crate-root module, holds `ray_parity_as`, the ray parity of a query over a closed chain of pieces; each chart family supplies its pieces and turns its query into the ray's centre and rectangle through `ChainPiece`:"
    - In the change section, add: "`Caps` (the carrier walk's cell cap beside the trim's piece cap) stays in the engine with the walk; the trim takes its piece cap alone. `Caps`, `read_capped` are private to the engine."
    - Replace the "The new row … pins it" sentence with: "The row `the_ring_door_moves_a_spline_faces_rings_into_its_outer_loops_region` pins it on one face holding two holes: deck 0 for the hole stored in the outer loop's region and `u = −1` for the one stored a period away, each checked against where that ring is stored, and each moved ring's lift inside the outer loop's hull in `u`. Forcing the deck to the identity, negating it, or handing the two decks out reversed reds the row."
    - Add to "Why the new answers are right": "**The size stop holds at its boundary.** `a_piece_exactly_the_querys_size_that_meets_it_keeps_it_near` reads a query cut to a piece's box, bit for bit, as `Near`; a stop that took only a strictly narrower box (`&lt;`) reads it `Out`."
    - Seams:
      - CHARTTAIL `pcurves.rs`: "re-exports `RingMiss` and `Shift` from the ring module for the trim"; drop the five loop names.
      - GUARD: "falls 11 → 4".
      - Re-cites: add CHARTTAIL `sphere-region-refuses-null-scaffolding-the-torus-and-ruled-doors-step-over`, `crates/geom-brep/README.md` and `crates/sweep/tests/reach_wall_chord_rows.rs`.
      - Add: "Sentences about what a past change did keep the address it had and add the present one."
    - Sweep: hit 1 becomes "`pcurves_rings.rs` `Outer::parity` and `spline_trim.rs` `Trim::copy`: **served**, now the one loop `chain_parity::ray_parity_as`."
    
    FILED: nothing new.
</cross-session-message>

0 notifications remain queued.
