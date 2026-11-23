DELTA RE-REVIEW — PR 6945 @ a9618dd72cd934a39347186ef8ccb4691c35a7db (delta from a258fd3228, crates/topo + crates/mesh + work/; viewer/placing churn is main's merge, not read).
    
    **Verdict: APPROVE.** The MAJOR is closed and every claim held. What is left is NOTE/style, plus the usual gate condition: I did not check CI on a9618dd; the PR gate must be green on this SHA.
    
    ## Claims
    1. **HELD.** `cargo nextest run -p topo --test all recourse_roster` at a9618dd names only the three `ring_path` indirect sites, which is the same set as a258fd's run minus the two RING_PLACE consts. The tracking row `work/sectarms/main-red-recourse-roster-misses-two-ring-path-indirect-sites.md` exists on main. The registration matches what the roster means by a door:
       - door `RING_PLACE`, prefix `ring_place_`, error `topo::pcurves::RingPlacementError`, which carries the escalation (`Undecided { cause }`);
       - the two consts declared as carriers;
       - `RING_PLACE_ARM` still reaches the existing `gate_arm_as` forwarder.
    
       It is not a silencing entry.
    2. **HELD.**
       - Against origin/main, spline_region.rs differs only in three places: the work-bound import, the `place_ring` call inside `with_rings` (same reading, `.ok()?` restores `None` for every miss), and `WINDOW_SHARE`/`HALVINGS` replacing the literals 16.0 and 6. `Trim::of`/`with_rings` signatures are main's, and spline_region_rows.rs is byte-identical to main.
       - `pcurves_rings.rs` names nothing under `boolean` (by grep).
       - A spline face with rings answers `SplineChart` before any placement, so no ring is handed out undecided; a spline face with no rings answers `Ok` with an empty ring list.
       - spline_region + pcurves::rings rows: 37/37 green.
    3. **HELD.** The values did not change: SHIFTS = old COPIES = 9; PIECE_DEPTH 52, PIECE_VISITS 1&lt;&lt;14, HALVINGS 6 (was the literal `0..6`), WINDOW_SHARE 16 (was `/16.0` in both readers; Outer's ladder `16·2^h` is unchanged). Style note below on `SHIFTS as COPIES`.
    4. **HELD, with one gap.** The only f64→i32 sites are `place_ring`'s candidate count (`whole(m)?` → `Copies`) and `Outer::side`'s principal-latitude k (`whole` → `Near(None)`). Both are the right answers there; NaN and ±∞ fall to `None`. The other cast, `n as usize`, is bounded by `n ≤ SHIFTS`. The one side effect: on the spline trim, a count of periods past i32 used to place by its f64 offset and now answers `None`. That is unreachable and in the safe direction. The gap: **mutant M4** (make `whole` saturate: `Some(m as i32)`) passes all 37 topo rows green, so no row pins `whole`'s refusal.
    5. **HELD.** `by == Deck::IDENTITY` only when `place_ring` returned the (0,0) shift, which requires that candidate to read `In`, and the second-`In` / `Near` exits run before it can return. With no closed channel the only candidate is (0,0), which must also read `In`. The first read is `Lift::read` at the identity, the same thing `read_from(IDENTITY)` produced, so nothing undecided can come out.
    6. **HELD: each mutant goes red.**
       - M1, skip the clearance (raw gap &gt; 0 counts as clear): red on `the_analytic_reader_reads_notches_vertices_slivers_poles_and_apexes` and `an_analytic_outer_loop_reads_…_past_the_pole`. The mesh probes stay green, as before.
       - M2, `place_ring` always answers `NoShift`: red on spline_region's `a_face_across_a_closed_charts_seam_reads_its_region_and_its_hole` and on lift_probes' `a_ring_sits_in_its_outer_loops_polygon_but_on_the_ball_less_the_tube`.
       - M3, exhausted halvings read `Out` instead of `Near` (drops the twin's `Undecided`): red on `a_sphere_ring_on_the_twin_sheet_answers_undecided` and `the_analytic_reader_…`.
    7. **HELD as far as I checked.**
       - **`RingOpen`**, by argument: a zero-winding outer loop bounds a disc-with-holes inside the lifted sheet, so every ring is null-homotopic there and winds zero. `RingOpen` therefore needs `OuterOpen` first, and is a typed guard that cannot be reached.
       - **No at-rest ring on a spline face:** not verified; I did not survey the fixtures.
       - **`NoShift`/`Shifts`/`Copies`/`Undecided`:** pinned only at the rule and the reader, as claimed.
    8. **HELD.**
       - Module doc: "two today", near line for line, one loop over two box sources, with the unit that makes them one.
       - `Undecided` doc: names the twin sheet and the non-tightening boxes.
       - `PlacedRing::lift` and `Lift::read_from` docs: say fresh rounding, not "bit for bit".
       - README row: present tense, no work id; it says what `SplineChart` means and that the work bounds live beside the rule.
       - Filed rows: the four the brief lists exist (spline-arm unit, P1, with the parity-reader class sweep listed; non-tightening boxes, P2; torus chart window; interior-witness rung), and azimuth-two-homes gains "a third assembly".
    
    ## Findings
    - NOTE — `pcurves_rings.rs` `whole`: no row pins its range check; mutant M4 survives. DEMONSTRATED.
    - NOTE — `RingPlacementError::RingOpen` cannot be reached from a body by the topology argument above. It is a fine guard, but its doc does not say so. BY-INSPECTION.
    - NOTE — `spline_region_rows.rs` r1 lost the deck pin this PR had added. That is correct now that `with_rings` returns `Option` again, but the trim's deck is now pinned nowhere; only its region is. BY-INSPECTION.
    
    ## Style (delta)
    - Q1 `spline_region.rs` `use crate::pcurves::{…, SHIFTS as COPIES, …}`. **likely.** One constant now bounds two different counts: shifts per channel in `place_ring`, and the copy *product* `us.len()*vs.len()` in `Trim::copies`. The doc names both, but raising one moves the other, which is the coupling a rename hides.
    - Q1 the duplicated parity loop is still there, now disclosed and scheduled (P1 unit with a class sweep), so it is owed and booked, not silent. **sure.**
    - Q2 the module doc names a work file twice, and `Undecided`/`SplineChart` docs name work ids in a public enum's rustdoc. **unsure.** That prose rots when the unit lands; the README row correctly does not do this.
    - Q7 `RingMiss` + `RingMiss::of` still mirror four `RingPlacementError` variants. The doc now justifies it (the trim reads the rule with no ring key). **unsure.**
    - Q5 `SHIFTS`' doc says a channel past it answers `RingMiss::Copies`. On the trim's copy product it answers `Place::Near(None)`, not `Copies`. **likely.**
    
    ## Executed
    At a9618dd:
    - the roster row
    - `topo --lib pcurves::rings spline_region`: 37/37 green
    - `mesh --test all lift_probes`: 7/7 green
    - mutants M1–M4 as above; every file was restored and `git status` is clean
    
    Read: the delta's topo/mesh/work diffs, spline_region against origin/main, pcurves_rings.rs whole. Not run: `-p sweep`, CI status.
</cross-session-message>

0 notifications remain queued.
