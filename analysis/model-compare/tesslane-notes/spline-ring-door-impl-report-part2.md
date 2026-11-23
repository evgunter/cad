TESSLANE spline-ring-door report, part 2 of 2. Part 1 had a placeholder where the PR body belongs; sections 3–6 are here in full. Head: 9f0e382e4a5175109e8ac61d181f9e61b6250b3b.
    
    3. PR BODY
    =====
    ## Measurement
    
    The spline trim (`Trim` and its reading of a query) and the chart's Bézier cells reach nothing in an engine. Compiled at the crate root, they read only root items: `crate::pcurves` (`Lift`, `ChartPeriods`, `Side`, `place_ring` and the work bounds), `crate::chart_region::spline_chart_arms` and its arm gate, `crate::validate::decide`, `crate::live::proven`, `crate::joint::Winding`, and the arena keys. The one engine item the module used, `splitting::Escalation`, is read only by the two placements that stay.
    
    What in the engine reaches them:
    - `boolean/spline_region.rs`: `spline_face_placement` (through `read` → `read_capped`) and `spline_chart_placement` build `bezier_cells` and `Trim::of`, then read `Trim::place`. `carrier_walk` reads `Patch` (`floor`, `extent`, `quartered`, `rect`), `Rect` and `Caps`.
    - `boolean/mod.rs` `decision_words` names the trim's two predicates.
    - `boolean/spline_region_rows.rs` reaches the trim's insides: `Trim::{of, from_pieces, with_rings, place}`, `Patch`'s fields and `split`/`restricted`/`floor`, `Piece` and its methods, `point_rounding`, `bezier_pieces`, `image_pieces`.
    - No other engine file names a trim item. `reduce.rs` reads only the rows' fixtures.
    
    Where the spec's list was wrong:
    - `Caps` moves, and so does `CELL_VISITS` behind `Caps::READ`. It was not a root work bound before.
    - `CELL_DEPTH` and `keep` stay, since only the carrier walk reads them.
    - The list left out `Control`, `tracks`, `bezier_insertions`, `close_gap`, `walks_forward`, `half_width`, and the test hook `mutant`/`MUTANTS`. All of these move, because every mutant site is in moved code.
    - The ONEDOOR row's citation of `spline_region` is still right after the move. It names the error, `Escalation` and `Uncrossable`, which stay in the engine.
    - More open rows cite moved items than the spec listed (see *Seams*).
    
    No at-rest body from a door holds a ring on a spline face, as the spec measured. An Euler-built body does: `spline_region_rows.rs`'s `holed_sheet` on `square_tube`, a sheet across the tube's seam with a hole. The door measures the outer loop's lift over `u ∈ [−0.1, 0.1]`. The hole at `u ∈ [1.02, 1.06]` is stored at `[0.02, 0.06]` with deck 0. The hole at `[0.92, 0.96]` is stored a period away, with deck `u = −1`.
    
    ## The change
    
    **`crates/topo/src/spline_trim.rs`, a crate-root module.** It holds the trim, the query's placement, the ring rule over the trim (`Trim::ring_shift`), the chart's Bézier cells and their rounding charges, and the module docs' *The trim, in the chart* and *Rounding*.
    - `[topo.homes]` rule 2 places it: `Trim::of` reads over a `Body` and names no engine type, so the module is a crate-root reading named by what it reads.
    - The cells take no `Body`. Rule 1 alone would send them to `geom-brep`. They move with the trim because the trim meters by them. `work/splice/the-spline-chart-cell-walk-is-written-twice` already owns the move of the cells, the cut and the floor to one home in `geom-brep`, where `spline_incidence` cuts the same way. Splitting them off here would leave a third copy.
    - The boolean's spline arm keeps the carrier walk and the two placements (`spline_face_placement`, `spline_chart_placement`, `read`, `read_capped`, `carrier_walk`, `Reading`, `CELL_DEPTH`, `SPLINE_REGION_CARRIER`), and calls the root.
    - The trim's predicates take the module's prefix: `spline_trim_clear` (was `bool_spline_region_trim`) and `spline_trim_arm` (was `bool_spline_region_arm`). The roster lists them as the boolean's, the first door to raise them, as it lists the other names two doors read.
    
    **The ring door reads a spline face.** `Body::face_lift` on a spline chart calls `spline_trim::ring_decks`.
    - `ring_decks` builds the outer loop's trim from the lift the door already read. It uses the trim's own chart reading and chain (`TrimChart::of`, `TrimChart::chain`, both factored out of `Trim::of`).
    - It places each ring's first chain point by `Trim::ring_shift`, the rule `Trim::with_rings` now places the trim's own rings by. The door's deck and the trim's are therefore one decision.
    - `RingPlacementError::SplineChart` retires. It said only that the door did not read the chart. `SplineTrimUnread` takes its place for the cases where the trim really is not read: a placeholder chart, no Bézier cells, a period stated other than decided, or a chain with no piece reading.
    
    **One parity loop.** `pcurves_rings.rs` `ray_parity_as` is the ray parity of a query over a closed chain of pieces. Each chart family supplies its pieces through `ChainPiece`:
    - the analytic `Piece`, boxed by `Pcurve::chart_box`, gaps in `T`;
    - the spline `Piece`, boxed by its control points widened by their rounding, gaps in `f64`.
    
    Both readers call the loop: `Outer::side` under `RING_PLACE_CLEAR`, `Trim::copy` under `SPLINE_TRIM_CLEAR`, and the funnel hands the name on. `Place` retires for `Side`, its variant-for-variant twin. `Trim::ring_shift` reads `Side` directly, and the engine and the rows spell `Side::`.
    
    Why the new answers are right:
    - **Every verdict rides on the same reading.** The loop's arithmetic is each old loop's own, term for term: the same box per family, the same gap per channel and the same `decide` under the same name and arm. It has the same `continue` ladder, the same half-open `v ≥ v_c`, and the same stops (`PIECE_DEPTH`; for a query of some size, a box inside it that meets it, read off the same `f64` gap). It reads the same visit caps (`PIECE_VISITS`; the trim's `Caps`).
    - **The deck the door hands out is the trim's own ring decision.** One `ring_shift` serves `with_rings` and the door. The new row `the_ring_door_moves_a_spline_faces_ring_into_its_outer_loops_region` pins it: deck 0 for the hole stored in the outer loop's region, deck `u = −1` for the one stored a period away, and each moved ring's lift inside the outer loop's hull in `u`. Forcing the door's deck to the identity, or negating it, reds the row.
    - **Both families run through the one loop.** Making the parity's half-open rule inconsistent (`s ≥ v_c` against `e &gt; v_c`) reds the analytic `the_analytic_reader_reads_notches_vertices_slivers_poles_and_apexes` and the spline `r1_a_ray_through_a_trim_vertex_counts_once` and `review_r2_parity_through_a_vertex_and_along_a_side`.
    - `spline_region_rows.rs` is unedited through the move commit. Later commits change only names: the two predicate consts, `Place` → `Side`, and the new deck row. Every row passes at ε 1e-9, 1e-6 and 1e-12, and sweep's lofted-wall rows pass.
    
    `crates/topo/README.md`'s *Where in the code*: the ring row now says a spline chart reads the point through its trim, and a row for `src/spline_trim.rs` is added.
    
    **Not pinned:** `SplineTrimUnread` has no row. No fixture reaches a spline face with rings whose trim the trim refuses to read.
    
    ## Seams
    
    `python3 scripts/work.py territory --base origin/main` names:
    - **SPLICE's and the boolean programs' ground** (`boolean/spline_region.rs`, `spline_region_rows.rs`, `boolean/mod.rs`). The trim and cells leave `spline_region.rs`. `decision_words` names the two renamed predicates. The rows file's edits are the renamed const, `Place` → `Side`, and one new row.
    - **CHARTTAIL's `pcurves.rs`.** It now re-exports `ChainPiece`, `PieceBox`, `RayQuery`, `gap`, `ray_parity_as`, `RingMiss` and `Shift` from the ring module. `pcurves_rings.rs` holds the loop.
    - **TCOST/TINT's `tests/recourse_roster.rs`.** It takes the renamed consts, the two names as the boolean's exceptions, and the funnel `ray_parity_as` (a carrier, and its forwarding site).
    - **GUARD's gate tables.** In `engine-leaves.counts`, `crate-visible boolean/spline_region.rs` falls 11 → 5. In `bare-rounding-sites.tsv`, the 8 `re-pose` sites move from `boolean/spline_region.rs` to `spline_trim.rs`.
    
    Open rows that cite moved code now cite its home:
    - SPLICE: `a-spline-faces-trim-reads-nothing-at-a-closed-seam-or-a-pole` (`Trim::read_window`, `Trim::exits_clear`), `the-spline-chart-cell-walk-is-written-twice`, `the-spline-face-floors-read-charge-is-wider-than-its-read` and `a-meeting-on-a-closed-charts-seam-is-not-isolated`.
    - PCERT: `the-projected-frame-box-reads-an-uncharged-f64-restriction`.
    - CHARTTAIL: `the-spline-chart-arms-read-an-uncharged-f64-cut` (`window_arms`) and `the-interior-witness-rung-reads-a-ring-where-it-is-stored-not-in-its-outer-loops-sheet` (`face_lift` now reads a spline chart).
    - HONE: `the-spline-arc-pieces-read-an-uncharged-f64-restriction`.
    - ISOTAIL: `the-edge-piece-tube-certifies-an-uncharged-f64-restriction` (`point_rounding`).
    - ONEROUND: `site-rounding-charges-owed-a-re-posing`.
    - RESTFRONT: `the-region-arm-is-silent-on-spline-faces-and-off-admits-edges` (`Trim::of`, `Trim::with_rings`).
    - TESSLANE: `a-curved-faces-domain-is-read-from-every-loop` and `an-analytic-outer-loops-whole-span-boxes-never-tighten-under-the-ring-reader`.
    
    Rows that cite what stays in the engine keep their citations: SPLICE `the-spline-placement-read-certifies-no-second-in-trim-foot` (`read`), `an-end-on-a-spline-face-is-placed-in-space-not-on-its-sheet` (the two placements), `the-spline-region-carrier-walk-records-in-band-margins-by-construction-under-k-lint`; ONEDOOR `chord-joins-ring-side-reads-the-splitting-engines-loop-containment`; SEAL `three-escalation-endings-route-by-the-predicate-name`; ONECOPY `the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes`.
    
    ## Sweep (§5): readers of a point against a closed boundary by ray parity
    
    First pass: `parity ^=` and `% 2 == 1` across `crates/topo/src` and `crates/geom-brep/src`. Second pass, at the toggles and spellings that pattern misses: `= !inside`, `= !parity`, `is_multiple_of(2)`, `&amp; 1 ==`, and `parity` as a word. Hits and dispositions:
    
    - `pcurves_rings.rs` `Outer::parity` and `spline_trim.rs` `Trim::copy`: **served**, now the one loop `ray_parity_as`.
    - `chart_region.rs` `point_in_polygon` (through `ray_walk::walk`, `!crossings.is_multiple_of(2)` in `ray_walk.rs`): a chart polygon read by a ray schedule over exact segments, with grazes set aside. **Not this unit, filed:** `work/charttail/the-chart-polygons-ray-parity-and-the-chain-piece-parity-are-two-readers` (CHARTTAIL owns `chart_region`).
    - `splitting/containment.rs` `point_in_vertex_polygon` and `point_in_loop` (`!crossings.is_multiple_of(2)`, arc members through `PiecedArc::crossings`): a planar loop in space, read by `ray_walk`. **Not the class:** it is not a chart, and its reader is one home already. Named in the filed row.
    - `validate.rs`'s `GRAZED` ending: the point-in-face door's model-space ray cast, through `ray_walk`. **Not the class**, as above.
    - `cell_region`: reads in model space and draws no polygon. **Not the class.**
    - `ring_path.rs` `path_parity` (`n % 2 == 1`): a path's meetings with a loop's arcs on a surface, not a point against a boundary. **Not the class.**
    - `link.rs` (`count % 2 == 1`, a wedge count in a vertex's link), `boolean/reduce.rs` (a certified crossing count against its ends' sides), `star.rs` (`is_multiple_of(2)`, a fan's face order), `boolean/insert.rs` (record pairing and bit masks): **not the class.**
    - Test oracles (`chart_region.rs` `strictly_in`, `census/pierce_tilted.rs` `signed`, `splitting/spline_arc.rs` and `spiric_arc.rs` truth, `test_support_meeting.rs`, the fuzzers): **not readers.**
    
    What the pattern cannot match is a parity kept as a count compared elsewhere, or a reader that reads a side some other way (a winding number, an angle sum). The second pass covers the toggles. A winding number is not a ray parity, so it is out of the class by definition, and `Winding`/`loop_winding` read a loop's turn, not a point's side. A parity spelled as a `bool` passed through a fold (`fold(false, |p, c| p ^ c)`) would escape both passes. `grep -rnE "\^ [a-z_]*\)|fold\(false" crates/topo/src` found one hit, `boolean/fate.rs`'s `try_fold(false, …)`, which folds a touch and not a parity.
    
    ## Filed
    
    - `work/charttail/the-chart-polygons-ray-parity-and-the-chain-piece-parity-are-two-readers` (P3).
    =====
    
    4. WHAT I RAN LOCALLY (on the merged head 9f0e382e4a, and before each earlier push)
    - bash scripts/run-gates.sh: exit 0.
    - cargo fmt --all --check: clean.
    - cargo clippy -p topo -p geom-brep -p mesh --all-targets --all-features -D warnings: clean.
    - cargo nextest run -p topo, whole, default eps: 4232 run, 4229 passed, 3 failed. All three are main's tracked reds:
      - offer_rows::every_offered_tolerance_passes_just_below_it (work/green rows);
      - shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void (work/green nightly row);
      - recourse_roster, naming only main's three ring_path sites and nothing of mine.
      At c9c9fdf6b6, link::cone_fuzz also failed once on a random seed (work/splitdoor/the-cone-fuzz-draws-a-cone-bounding-no-one-material-at-a-seed). It passed on the later runs.
    - CAD_TOLERANCE_EPS=1e-6 and 1e-12 over spline_region::rows and pcurves::rings: 39/39 at each.
    - cargo nextest run -p sweep -E 'test(lofted_wall) | test(lofted_walls)': 7/7.
    - work.py lint: ok.
    - demos/tour and demos/wild: not run. Neither names RingPlacementError or face_lift (grep).
    - Left to CI: hosted gate, other crates' suites, nightly eps rows.
    
    5. PUSHED HEAD: 9f0e382e4a5175109e8ac61d181f9e61b6250b3b. None of my commits carries a trailer. The Co-Authored-By/session lines in the branch history belong to main's commits, which came in through the merge.
    
    6. WHAT SLOWED ME DOWN
    - The whole topo suite takes about 25 min a run, and I ran it four times: after the move, after the door, after the loop, and after merging main, which brought geom-brep changes.
    - I wrote the loop commit in a side worktree so the topo source censuses would not read half-edited files mid-run.
    - My first clippy call hit the 120 s default timeout and went to the background. I polled it to completion in the foreground.
</cross-session-message>

0 notifications remain queued.
