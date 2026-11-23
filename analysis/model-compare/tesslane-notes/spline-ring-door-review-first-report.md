REVIEW — PR 6951 (tesslane/spline-ring-door) @ 9f0e382e4a, FULL, single reviewer
    
    **Verdict: APPROVE-WITH-FIXES.** No MAJOR. The move is verbatim and the one loop gives the same verdicts as the old ones over 138k differential readings. The fixes: one row the re-citation missed, the deck row can't see a per-ring mix-up, and two doc lists are incomplete.
    
    ## Claims
    1. **HELD.** Script, per top-level item: every item of main's `boolean/spline_region.rs` (at the merge-base) appears in `spline_trim.rs`@c9c9fdf6b6 or the engine file, verbatim after removing `pub(crate)`. Two items (`image_pieces`, `bezier_pieces`) differ only by rustfmt line-wrapping. Nothing in spline_trim@c1 is new. `spline_region_rows.rs` is not touched in c1. Gate tables: counts 11→5; the 8 `re-pose` sites moved row for row.
    2. **HELD.** The root module names nothing under boolean or splitting in code; it does in doc prose (Style S6). The trim is rule 2. Before the move the cells already broke rule 1, sitting in an engine. At the root they are no further from it, and `engine-leaves` crate-visible fell. The open SPLICE row (P3) names the geom-brep home. A step, not a new violation.
    3. **HELD, one caveat.** No old string remains anywhere in the repo outside the unit's spec and one closed SPLICE issue. `decision_words` follows. No pinned text changes. Caveat: `EXCEPTIONS`' door field is informational only (`names_have_doors` checks only that a name is listed), and the ring door raises these names typed `RingPlacementError::Undecided`, not `BooleanError`. So "BOOLEAN, first door" silences the row more than it describes the code (S9).
    4. **HELD.** `TrimChart::of` + `chain` are main's `Trim::of` text re-cut: same order of checks, same refusals. `ring_decks` and `with_rings` both call `ring_shift` on an outer-only trim, so the door cannot hand out a deck the trim did not decide. One difference: the door's outer comes from `Lift::read`, the trim's from `read_past_placeholders`; at rest these are the same, and on a placeholder the door answers `Gap` first. Unread paths: none outside `SplineTrimUnread`/the rule's miss/`Gap`. But the variant's doc misses one case, a row moved by a period on a channel the chart states but does not decide closed (`TrimChart::chain`). MINOR-3.
    5. **HELD, one gap.** The fixture validates: probe ran `validate::validate` → Ok, and printed the lifts. The outer is u∈[−0.1,0.1]; hole 1 deck 0 stays at [0.02,0.06]; hole 2 deck u=−1 moves to [−0.08,−0.04]. Mutants on `ring_decks`: identity → RED, negated → RED, channels swapped → RED, u+1 → RED. **Rings' decks reversed → GREEN** (each fixture holds one ring). MINOR-1.
    6. **HELD by execution.** Harness: main's `Outer::parity` and main's `Trim::copy` copied verbatim (cap as a parameter) against `ray_parity_as`. Inputs: random and adversarial chains (grid vertices, rays through vertices, horizontal sides, on-side queries, in-band offsets, rect queries for spline, zero-weight piece → no box). Arms {1,1e3,1e-3,0,1e-300}, band eps {run ε,1e-3,1e-12,0.05}, caps {PIECE_VISITS,1,2,3,7,40,200}. **Every `Side` including the `Near` payload was identical:**
       - analytic f64 48000, analytic `Interval` 30000;
       - spline f64 36000, spline `Interval` 24000.
    
       The harness can go red: mutants in `ray_parity_as` (inconsistent half-open, `u1&lt;=cu`, `visited&gt;=visits`) each RED. It cannot see the size-stop `&lt;=` vs `&lt;` mutant, since it never generates an equal-size boundary.
    
       Rounding: the spline gaps are `T::from_computed(gap(..))`, as before; the analytic gaps are computed in T, as before. The only reorder: the spline cap is now checked before `bbox()`=None; both return `Near(None)`, so nothing observable changes.
    7. **HELD.** Same variants, same `Near(Option&lt;Indeterminate&gt;)` payload; `Side` adds `Copy`. The engine and rows edits are mechanical.
    8. **HELD.**
       - Whole `-p topo` here: 4229/4232. The three reds are the PR's: the roster names only `ring_path.rs: spline_crossings`, `spline_wall.rs: paths/reach`; the needle row gives exactly −1.1025e-16 vs −1.0142e-16. All three are tracked: `work/green/main-red-two-offered-tolerances…`, `nightly-reds-default-eps-behaviour-rows`, `nightly-reds-the-source-censuses`. I did NOT rebuild main to see them red there.
       - Spline rows + ring rows at eps 1e-6 and 1e-12: 39/39 each.
       - Sweep lofted rows: 17/17.
       - CI on the head: lint, kernel, app, corrupt-input, mesh and gate ok all success.
    9. **CORRECTED (scope).** `crates/mesh` planar.rs/trimmed.rs even-odd is a CDT face flood-fill by constraint multiplicity: combinatorial, not a point-vs-chain ray reader, so it is not this class. Nor is editor-core `coincide/stated.rs` `parity ^=` (union-find sense). The PR states what its pattern can't match but not that it swept only topo/geom-brep. NOTE.
    10. **Mostly held.** All 13 re-citations are true where they cite moved code. **Missed:** the open `work/charttail/sphere-region-refuses-null-scaffolding-the-torus-and-ruled-doors-step-over.md:63` still cites `boolean::spline_region::Trim::of` (MINOR-2, a SILENT spec deviation). The README rows decide nothing, but the ring row's list of typed answers is incomplete (MINOR-3). `SplineTrimUnread` having no row is honest: no fixture reaches it, a stated gap.
    11. **HELD.** `run-gates.sh` (GATE_BASE=origin/main) exit 0. `doc-gate.sh --workspace-pass` OK. The trim's *The trim, in the chart* and *Rounding* sections still describe what is beside them.
    
    ## Findings
    - **MINOR-1 DEMONSTRATED** — `boolean/spline_region_rows.rs` `the_ring_door_moves_a_spline_faces_ring_into_its_outer_loops_region`: single-ring fixtures, so deck→ring assignment is unpinned (rings-reversed mutant in `spline_trim::ring_decks` passes).
    - **MINOR-2 BY-INSPECTION** — the missed CHARTTAIL row above.
    - **MINOR-3 BY-INSPECTION** — `pcurves_rings.rs` `RingPlacementError::SplineTrimUnread` doc and `crates/topo/README.md`'s ring row (it says only "a spline trim with no piece reading") omit causes: the stated-but-undecided period case, placeholder chart, no cells, period mismatch.
    - **NOTE** — `crates/geom-brep/README.md:1241-1248` and `crates/sweep/tests/reach_wall_chord_rows.rs:371` still describe the trim reading as `topo::boolean::spline_region`. Half true now: the placement stays there, the trim moved.
    - **NOTE** — four rows' historical "That PR charges … (`topo/src/spline_trim.rs` …)" now cite a file the past PR never touched.
    - **NOTE** — the size-stop boundary (`u1-u0 &lt;= su`) is unpinned by my harness; I did not check repo rows against it.
    
    Spec deviations: Caps/CELL_VISITS/extra items moved, the rows edited after c1, the rename in its own commit — all REPORTED. The CHARTTAIL re-citation and the mesh scope limit are SILENT.
    
    ## Style
    Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (pcurves_rings.rs read whole; spline_trim.rs non-verbatim parts + module docs + mutant hook).
    - S1 likely — `pcurves_rings.rs:365-373` `ray_parity_as` takes the query twice (`q: P::Query` and `at: RayQuery`). Each caller derives one from the other (`Outer::side`, `Trim::copy`), and nothing checks they agree.
    - S2 likely — `pcurves_rings.rs:406-408`: the "meets within the query's size" stop is spline-only in effect (analytic queries are points, su=sv=0). One loop with a family's special case written as a general stop. The `visits` parameter exists for the trim's per-instance cap.
    - S3 likely — Q1: two per-ring deck loops (the inline analytic arm in `face_lift` `pcurves_rings.rs:460-486`, and `spline_trim::ring_decks`) with two notions of "the ring's first point" (`first_point` = lift entry in T; trim chain's first piece start in f64). The analytic adapter lives with the rule; the spline adapter lives in the trim. `Trim::ring_shift` and the analytic closure are two spellings of "read a candidate".
    - S4 likely — `spline_trim.rs` `Caps` bundles the carrier walk's cell cap with the trim's piece cap. The trim stores only `piece_visits`, `with_rings` fabricates `Caps{cells:0}`, and `CELL_VISITS` (engine-only) now sits at the root.
    - S5 likely — `pcurves_rings.rs:335-337` `ChainPiece::ends` "bit-identical wherever two pieces share one" is a prose invariant the parity's correctness rests on; no type or check enforces it.
    - S6 sure — Q5: `spline_trim.rs:1-4` defines the root module by its engine consumer (`boolean::spline_region`) and never mentions the ring door / `ring_decks`, its second reader.
    - S7 likely — `spline_trim.rs:581-611` keeps the mutant env `SPLICE_SPLINE_REGION_MUTANT` and the `spline-region-*` site names. The old engine name lives on in the root home; the class is any other `spline_region` spelling in moved text.
    - S8 likely — `SplineTrimUnread` is a unit variant collapsing ≥5 causes with no ring key, while each sibling carries `ring`.
    - S9 unsure — `tests/recourse_roster.rs:186-190`: "the boolean's is the first door to raise them" cannot be checked, and the door field does no checking.
    - S10 unsure — `pcurves.rs` re-export list grew by 7 (`gap`, `PieceBox`, …): CHARTTAIL's module becomes a hub for ring-module internals.
    - S11 unsure — `SHIFTS` doubling as `COPIES` is re-affirmed in new doc text (pre-existing).
    
    ## Rubric
    - idiom/structure **4** — a real single loop and a verbatim move. The query is passed twice and Caps is mixed (S1, S4).
    - test quality **3** — the deck row reds on 4/5 deck mutants, but its single-ring fixtures leave assignment unpinned. No committed differential pins loop equivalence.
    - doc/comment honesty **4** — accurate except the incomplete Unread lists and the module opener.
    
    ## Executed vs read
    - **Executed:** item-level verbatim diff script; differential harness + 4 loop mutants; 5 deck mutants; fixture validate probe; whole `-p topo`; eps 1e-6/1e-12 sets; sweep lofted rows; run-gates; doc-gate workspace pass.
    - **Read only:** tracker rows, README, roster semantics, CI status.
    - **Not run:** the three reds on main itself.
    
    Harness (not committed): /tmp/claude-0/-home-user-cad/e399f928-1d8f-51b0-814e-97a09a9f1237/scratchpad/harness/ — `review_diff_analytic.rs`, `review_diff_spline.rs`, and `wiring-and-probe.patch`. The patch holds the `#[path]` mod lines, the `PIECE_DEPTH` re-export and the validate probe row. Note: the harness calls `decide`, so it perturbs the source-census rows if left in the tree.
</cross-session-message>

0 notifications remain queued.
