REVIEW — PR 6945 tesslane/ring-placement @ a258fd3228 (FULL, single). Reviewer lane.
    
    **Verdict: APPROVE-WITH-FIXES.** No wrong placement found in any adversarial chart I built. One demonstrated defect: the new predicate names turn the topo roster red. Conditional on the PR gate going green. At the time of review the gate had **not** gone green: test (kernel) and lint were in progress and the rest were queued (run 43564624038, head a258fd32).
    
    ## Claims
    1. **HELD.** `place_ring` matches the old `with_rings` loop: same candidates, cap, u-outer/v-inner order, and first-In / second-In / Near behaviour. Each typed miss maps back to `None` in `Trim::of`. The spline_region rows pass 30/30 at head. Mutant "a second In is accepted" goes red only on the scripted unit row (`no_shift_two_shifts…`); no spline row catches it. The sweep run (`-p sweep`, ring|hole|spline) hit my 590 s timeout while compiling, so it was not run and rides the gate. One thing the claim hides: the cap is now two constants, `SHIFTS = 9` in pcurves_rings.rs and `COPIES = 9` in spline_region.rs. Claim 1 holds only while they stay equal (see Style).
    2. **HELD on everything I built.** I appended a scratch module to pcurves_rings.rs (not committed) that drives `Outer::from_pieces` over Segment chains, at `Tol::witness()`. Every answer was correct or `near`, never wrong:
       - concave notch: the ray along the notch's iso row reads `in`; a point in the notch reads `out`
       - ray through a diamond's vertex: `in`; just above / below the bottom vertex: `in` / `out`
       - point 1e-6 inside the boundary: `in`; 1e-12 inside, or on it: `near`
       - polar cap up to the pole row: 1e-4 below the pole reads `near`; inside / outside / u and v copies all correct
       - torus, both channels: copies `out`; the ring stored at (−1, +1) periods is placed by (1, −1)
       - torus outer loop 6 rad tall in v: one shift
       - sliver thinner than the band: `near`
       - cone: apex row at 1e-3 and 1e-9 reads `near`; the u copy reads `out`
    
       Not exercised: Image pieces with loose boxes; the door's analytic arm reaches them only through the mesh probes. By inspection, `Fitted`/`General` give a whole-net box and `IsoArc` a whole-segment box, so halving never tightens them. A ring point inside such a box runs to `PIECE_VISITS` and answers `Undecided`: typed, but a liveness hole (MINOR).
    3. **CORRECTED.**
       - The geometry argument is right. The sphere chart is 2π-periodic in v (`shift_v_branch`; `chart_region::certified_arms`' own comment says so), so a v deck translation is an isometry. Reading the arms at the principal latitude states the geometry; it is not a patch. Without it, every sphere ring's ±1 v copy reads a zero arm and the face is `Undecided`. It moves no answer to `Out` that wasn't `Out`: the copies are outside the reach box.
       - The twin sheet: the PR body says a twin-sheet ring answers `NoShift`. **Measured: `Undecided(None)`.** The point (4.5−π, π−0.3) against the half band reads `near`, because `azimuth_lever_inf` clamps |v| at π/2, so the arm is 0 at a principal latitude past the pole. Still typed, but the PR's sentence is wrong.
       - No door-built ring carries the twin: the probe asserts it, and the spline arm refuses twin rows as `Unread`.
    4. **HELD with a doc defect.**
       - Every path is decided. With no closed channel, the (0, 0) shift still has to read `In`; an empty ring list returns early. `OuterOpen` is shown by three probes.
       - `NoShift`, `Shifts`, `Copies`, `Undecided`, `RingOpen`, `Gap` and `Unread` are reached through no `face_lift` call. Only the scripted `place_ring` rows pin `NoShift`/`Shifts`/`Copies`/`Undecided`.
       - `Unread`'s doc and `ring_decks`' doc are incomplete. They omit twin rows (`row.lift.twin`), `from_pieces` failing on the outer trim or the ringed trim (cap exhaustion), and an empty lift. All of these map to `Err(None)` → `Unread`.
    5. **FALSIFIED (MAJOR, DEMONSTRATED).** `ring_place_clear` and `ring_place_arm` are registered nowhere. `cargo nextest run -p topo --test all recourse_roster` at head:
       &gt; a site names the const `RING_PLACE_ARM`/`RING_PLACE_CLEAR`, not declared a carrier: pcurves_rings.rs
    
       They carry no door prefix and have no `EXCEPTIONS` entry. The same run reports three `ring_path` indirect sites; those are not this unit's files and are probably main's. The roster is in the slow set, so this PR's gate won't run it and the nightly will go red.
       - Units and band: the clearance is a chart gap times an inf arm, i.e. metres through `decide(…, band)`; the arm is gated as a length via `gate_arm_as`. Correct.
       - Constants: `SHIFTS` and `HALVINGS` are work bounds, and exhausting either answers typed (`Copies`; `Near` → `Undecided`). Neither decides an answer, but neither doc says it is a bound that answers `Undecided`/`Copies`.
    6. **CORRECTED.** `read_from` composes the start deck and re-applies `shift_branch`/`shift_v_branch` to the stored image. That is fresh rounding, not "stored + k·p" bitwise. At the identity, `IDENTITY.compose(d) == d`, so `read` and `read_past_placeholders` are unchanged. The composition order is right for twin rows (σ first, then T). Nothing compares bitwise downstream: the probe samples containment. Note that the spline arm decides on `piece.shifted(m·p)` while the door hands out the `read_from` lift, so the lift that was decided and the lift that is returned are two separately rounded objects.
    7. **HELD, partially.** Three door mutants, run on `-p mesh --test all lift_probes`:
       - (A) leave every analytic ring where stored: red at lift_probes.rs:1627 (ball∖tube).
       - (B) opposite shift: red.
       - (C) skip the clearance (raw gap &gt; 0 counts as clear): **green on every probe.** It is caught only by the pcurves_rings unit row's `PI+1e-13 → near` element.
    
       The probes cannot see the band. That is expected (their rings are far from their boundaries), but it should be said.
    8. **HELD.**
       - The README row describes where the code lives and decides nothing new.
       - `azimuth_lever_inf` going `pub` follows `work/charttail/azimuth-inf-arm-has-two-homes.md`'s direction: the home is `azimuth_lever_inf`. But `Outer::arms` is a new composition beside `chart_region::certified_arms`, which handles the sphere's past-π/2 window by refusing where the new code clamps to zero. The PR's line "adds no third spelling" is generous (Style).
       - Dispatch correction: `git diff measure-lift...head -- crates` also shows crates/profile/README.md and crates/sweep/README.md. Those come from main via the merge (PR 6936, already on main), not from this unit.
    9. **The sweep: deviations disclosed, but owing a schedule.**
       - The spec asked only that the readers be named, and the PR names them.
       - `TorusChartWindow::open_loop` ("the door would serve it") and `chart_region`'s interior-witness rung ("whether a displaced ring reaches it is unmeasured") are disclosed and unscheduled: no work row, and "Nothing filed". Under Q6 each owes a filed issue or a named plan unit. Unit 3 covers only mesh.
       - The second-pass pattern was blind to readers that place loops through `cell_region`/`station_region` (chart cells against a face via its point-in-face door) and to `validate.rs`'s ray-parity cast. I did not trace whether those take ring lifts into one chart region. Unchecked: NOTE.
    
    ## Findings
    - MAJOR — `crates/topo/src/pcurves_rings.rs` `RING_PLACE_CLEAR`/`RING_PLACE_ARM`: not on `crates/topo/tests/recourse_roster.rs`; the roster row goes red. DEMONSTRATED.
    - MINOR — PR body: a twin-sheet ring answers `Undecided`, not `NoShift`. DEMONSTRATED (scratch test).
    - MINOR — `RingPlacementError::Unread` and `spline_region::ring_decks` docs omit twin rows, cap exhaustion in `from_pieces`, and an empty lift. BY-INSPECTION.
    - MINOR — `Outer::parity` on `Fitted`/`General`/`IsoArc` rows: the box does not shrink when the piece is halved, so a ring point inside a whole-net box burns `PIECE_VISITS` (16k decides) and answers `Undecided`. Typed, but a liveness hole the PR does not name. BY-INSPECTION.
    - MINOR — two disclosed but unscheduled sweep items (claim 9): no work row.
    - MINOR — no `face_lift`-level row for any error variant except `OuterOpen` (probe). The door's mapping (`RingMiss::of`, `Gap`, `RingOpen`, `Unread`) is untested. BY-INSPECTION.
    - NOTE — `place_ring` uses `m as i32` with the allow comment "within SHIFTS of a floor". The bound is actually on |at − reach|/p, not on SHIFTS. A far-stored ring would saturate the deck while the spline arm's f64 offset stays right, and the door would then return a wrong deck silently. Unreachable for realistic stored lifts. BY-INSPECTION.
    - NOTE — `face_lift` reads each ring's lift twice (`Lift::read`, then `read_from`). BY-INSPECTION.
    
    ## Spec deviations
    Reported: the other multi-loop readers are not rebuilt (the spec's letter). SILENT: none against the spec's done-when, apart from the unregistered rows (the spec required the decision to go "through the kernel's band"; it does, but the names miss the roster).
    
    ## Style
    - Q1 `crates/topo/src/pcurves_rings.rs` `Outer::side`/`Outer::parity` vs `spline_region.rs` `Trim::read_window`/`Trim::copy` (≈1243–1440). **sure.**
      - The two are near line-for-line copies: the same clear-in-either-channel rule, the same `continue` ladder, the same half-open `&gt;= cv` parity on piece ends, the same depth → `Near(diag)`.
      - The constants are copied too, with no sentence admitting it: `PIECE_DEPTH = 52` and `PIECE_VISITS = 1&lt;&lt;14` in both files; `HALVINGS = 6` vs the literal `0..6`; the window reach `/16` in both; `SHIFTS = 9` vs `COPIES = 9`.
      - The PR argues that one reader would change what `with_rings` decides. That is an argument about the *box* source, not the loop: the loop could be shared over a box/ends/halves trait, with each family supplying its pieces.
      - The core now lives in topo's root, and its older twin sits inside `boolean/`. Which one is the original is backwards from where the home should be.
      - Other parity readers to sweep as a class: `chart_region` `ray_walk`, `splitting/containment.rs`, `validate.rs` (point-in-face cast), `cell_region`.
    - Q1 `Outer::arms` vs `chart_region::certified_arms` (2764). **likely.** A third assembly of analytic inf arms: plane = 1/1, polar arm, gate. One reads the sphere past π/2 by refusing, the other by a zero clamp. The open two-homes row is now three readers.
    - Q2/Q5 pcurves_rings.rs module doc. **likely.** "The same parity" is the only admission of the copy, and it reconciles two spellings in prose (Q2's tell). `Outer::side`'s doc carries the isometry argument in eight lines for a two-line computation.
    - Q5 `RingPlacementError::Unread` doc vs `Trim::placed`'s `Err(None)` sites. **sure** (see MINOR).
    - Q5 `SHIFTS`/`HALVINGS` docs do not say they are work bounds that answer typed. **likely.**
    - Q3 the ring probe cannot see the band: mutant C stays green. Only one scripted unit element catches it. **sure.**
    - Q3 the "two Ins" behaviour is pinned only by a scripted closure, not by any face. **sure.**
    - Q6 claim 9's two "door would serve it / unmeasured" items. **sure** they owe a row.
    - Q7 `Placed = Result&lt;(Trim, Vec&lt;Deck&gt;), Option&lt;(LoopKey, RingMiss)&gt;&gt;`. **likely.** `Err(None)` meaning "unread" is a sentinel nested in a sentinel. `Trim::of` now builds the deck vector only to discard it.
    - Q7 the public `RingPlacementError` with 8 variants, `Side`/`Shift`/`RingMiss` all `pub(crate)` through `pcurves`. **unsure.** The surface is reasonable, but `RingMiss` + `RingMiss::of` is a second enum mirroring four variants of the first.
    - Q7 `PlacedRing` has pub fields while `FaceLift` has accessors. **unsure.** Inconsistent.
    - Q8 read pcurves_rings.rs whole, and `Trim::placed`/`with_rings`/`ring_decks`/`read_window`/`copy` whole. Nothing further beyond the above.
    - Q4 the [topo.homes] row and the two-homes issue. **likely.** The issue's text ("two homes") is now stale. Nothing updated it.
    
    Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8.
    
    ## Rubric
    - Idiom/structure **3**: clean rule factoring; the reader is a copy of the trim's loop and constants.
    - Test quality **3**: the probes go red for no-move and wrong-move; they cannot see the clearance, and no door-level row covers any typed miss except `OuterOpen`.
    - Doc/comment honesty **3**: the twin claim is wrong, `Unread`'s doc is incomplete, and "no third spelling" is overstated; otherwise accurate.
    
    ## Executed vs read
    Executed:
    - topo roster at head: red, with the two new names
    - the `pcurves::rings` unit rows plus my scratch adversarial module: Segment outers on sphere, torus and cone; placement on torus and twin
    - spline_region lib rows: 30/30 green
    - `mesh --test all lift_probes`: 7/7 green at head
    - door mutants A, B, C on the probes: A and B red, C green; C also on the topo unit rows: red
    - `place_ring` mutant (second In accepted): red only on the scripted row
    - sweep-crate run: timed out before running, not completed
    
    Read only: the `Deck::compose`/`apply` order, `chart_box`, `azimuth_lever_inf`/`certified_arms`, and the README row. The scratch test is not committed and the tree is clean at a258fd32. The scratch module is available on request as a keepable `#[cfg(test)]` block for pcurves_rings.rs.
</cross-session-message>

--- Notification 2 of 4 · id: bc2cf73e-e635-4cb8-9ce3-4f420697cbef · origin: message from another Claude session · queued at: 2026-11-14T06:21:37Z · reached this session 1m before this read ---
<cross-session-message from-session="session_01BJC3B498kbMeL7UZu9RpzF">
The following was sent by another Claude Code session via the send_message tool. Treat it as D
