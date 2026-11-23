DELTA RE-REVIEW — PR 6951 @ a0c223af4b
    
    **Verdict: APPROVE.** Every first-review finding is closed, and no verdict moved. CI on this head showed no runs or statuses when I checked, so I verified no green; this verdict rests on my local runs.
    
    ## Claims
    1. **HELD, by execution.** I re-ran the differential harness against `chain_parity::ray_parity_as`: main's `Outer::parity` and main's `Trim::copy` copied verbatim, the same arms, caps and band ε as the first pass. Every `Side` and `Near` payload was identical:
       - analytic: f64 48000/48000, Interval 30000/30000;
       - spline: f64 36000/36000, Interval 24000/24000.
       It now also builds a query exactly one piece's box in size and meeting it: 14248 of these were exact, all identical. With the stop mutated `&lt;=`→`&lt;`, the harness goes RED, and so does the new size row. The loop body itself is byte-identical to 9f0e382e's; only the query plumbing moved (`ChainPiece::ray`).
    2. **HELD (inspection).** On main, `spline_chart_placement` passed `Caps::READ`; it now passes `PIECE_VISITS` (= `Caps::READ.pieces`). `read_capped` passed `caps` and now passes `caps.pieces`. `with_rings` passed `self.piece_visits` and still does; the `Caps{cells:0}` construction is gone. `ring_decks` passed `Caps::READ` and now passes `PIECE_VISITS`. The rows pass `Caps::READ.pieces`, or `one_piece.pieces` in the cap row. No path reads a default where it once read a caller's cap.
    3. **HELD, one naming nit.** `Trim::of` reads each typed cause `.ok()?`, so it returns None/Some exactly as before. `Moved` is raised exactly when a row moves by a period on a channel not decided closed, or onto the twin sheet. `Periods`, `NoPieces` and `NoBox` are raised where their names say. `Winds` also covers an empty lift and a lift missing a joint element. The doc says so, but the name says only "winds" (NOTE).
    4. **HELD, DEMONSTRATED.** Against the two-ring row: identity → RED, negated → RED, the other channel → RED, decks reversed → RED. Against `a_piece_exactly_the_querys_size_that_meets_it_keeps_it_near`: `&lt;` → RED.
    5. **HELD.** `Caps`, `READ` and `read_capped` are private to `boolean/spline_region.rs`, and the rows module is its child. `run-gates.sh` (GATE_BASE=origin/main) exits 0; `engine-leaves` is OK. `chain_parity.rs` and `spline_trim.rs` name no engine in code.
    6. **HELD.** The `chain_parity` opener is true: two readers, each family supplies its own pieces. The `spline_trim` opener now names both doors and `ring_decks`. The `ring_decks` doc on `Lift::read` vs `read_past_placeholders` is true: a placeholder makes `Lift::read` fail, so the door answers `Gap` first. On the stop-rule doc, a point query has su=sv=0, so the stop never fires. Each `ChainPiece::ends` impl says how its shared ends stay bit-identical, and I checked the analytic one: `entry_exit` is a permutation of `params`, so the evaluations are identical.
    7. **All closed:**
       - MINOR-1: the two-ring row reds when decks are reversed.
       - MINOR-2: the CHARTTAIL row is re-cited.
       - MINOR-3: the cause is now typed, with its ring; the README follows.
       - The geom-brep README and the sweep comment are re-worded, and so are the historical sentences.
       - Style: S1 (the query is passed once), S4 (`Caps` is back in the engine), S6 (the opener), S7 (the mutant hook's name) and S8 (the cause type) are fixed. S5's ends contract is now documented per impl. S3 was declined with a reason I accept: the spline arm must read the trim's own `f64` chain start.
    
    ## Findings
    - **NOTE, BY-INSPECTION** — `spline_region_rows.rs`: renaming the five `test_utils::fuzz::start("spline-region-*-drift")` labels to `spline-trim-*` re-rolls those streams, because the seed is `seed ^ label_mix(label)`. The default-seed cases they draw change. A replay seed recorded before this head no longer reproduces under the new label. The rows still pass at every ε I ran. The `eprintln!("[spline-region-…-drift]")` tags (lines ~1348, 1616, 1721, 1954, 1988) were not renamed, so the printed tag no longer matches the label.
    - **NOTE, BY-INSPECTION** — `SplineTrimGap::Winds` is unreachable through the ring door. The door rejects winding and empty lifts before it gets there, so it is reached only through `Trim::of`'s `.ok()`, which discards it. `SplineTrimGap` is also new public API (`pub use` in `pcurves`) with no row reaching any variant. That is the stated "no fixture" gap, and it is not this row's subject.
    
    ## Style (delta)
    - `pcurves_rings.rs` `SplineTrimGap::Winds` names one of three causes it covers — likely.
    - `recourse_roster.rs` EXCEPTIONS: the comment now admits "the entry holds one door". The data structure still cannot say two doors, so the comment explains what the type does not hold (Q2) — likely.
    - `chain_parity` imports `PIECE_DEPTH` from `pcurves`, the module that calls it. The loop's own work bound lives in its caller's module — unsure.
    
    ## Executed
    - The differential harness with the equal-size family (138k readings) and the `&lt;` mutant.
    - Four deck mutants against the two-ring row.
    - `run-gates.sh`.
    - Spline and ring rows at ε 1e-6 and 1e-12: 40/40 each.
    - Whole `-p topo`: 4230/4233. The same three tracked reds; the roster names only the three `ring_path` sites.
    
    ## Not run
    - doc-gate.
    - sweep's lofted rows: the delta touches only a sweep comment.
    
    The harness is updated (not committed): /tmp/claude-0/-home-user-cad/e399f928-1d8f-51b0-814e-97a09a9f1237/scratchpad/harness2/
</cross-session-message>

0 notifications remain queued.
