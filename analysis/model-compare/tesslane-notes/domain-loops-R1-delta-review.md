DELTA RE-REVIEW — PR 6960 @ 23654a82b8 (fix commits 7aa61bd9fe, fb9ba19f0d, 4f9e4fc06c, 23654a82b8)
    
    **Verdict: APPROVE-WITH-FIXES** (one MINOR; CI on the frozen head not checked by me — verify the green before merge). No work lost: the container survived the usage stop, and the turn resumed at the frozen head. I saw no other review's material.
    
    ## Claims
    1. **Held, with one wrong text (MINOR-A).** Probe: a `rod` pipe wall, a `mev_line` Fan strut off the outer loop's `first`, then `kemr` in both orders, with and without restoring the outer rows' joint and pcurve. `topo::validate` = Ok in all four.
       - Ring empty, rows restored → `DomainUnread{RingEmpty}` at δ 1e-2 and 1e-3 (it panicked at the old head). The text is true.
       - Ring empty, rows not restored → `Gap`. The new Gap text is true, and gives the `mint_pcurves` remedy.
       - Outer loop empty + a Cycle ring (`kemr(he_minus, he_plus)`), rows restored → **`DomainUnread{Copies}`**, text "its outer loop spans more of the chart's periods than the ring placement tries". That is untrue: the outer loop is a lone vertex. The true cause is an empty outer loop (`EmptyLoop`).
       - No panic was reachable from any of the four shapes.
       - The `RingEmpty` check sits in `face_lift` before any chart-specific code, so it covers every chart, spline included (by inspection).
       - A ring that is one half-edge is the ordinary case (ball∖tube's ring is one whole circle) and meshes.
    2. **Held, and it corrects my first report's NOTE-1.** `tessellate_impl` reads `face_domain` once per trimmed face and passes the same `FaceLift` to `gather` and to the lane. I was wrong earlier: `memo.rs` ~905–925 folds pcurves into the key bytes only `if trimmed`. So planar and curved keys never held images, on main or on the old head. **Retract NOTE-1.** On a ringless trimmed face, `face_lift`'s outer loop is the same read `loop_lift` makes, so its images are identical. I did not re-measure digests: I had built no digest differential, only probes. The D9 goldens, `docs/tess-budget-data` and the tour's `tests` dir are unchanged between main and head, apart from the lily wall-7 test.
    3. **Held.** `TrimPolygon` now holds points, edges and loop ends, and `next(i)` is derived from them. On a ringless face (one loop) `next(i)` is `(i+1)%n`, as before. I re-built the env-switched mutants on the new encoding and ran the 5 ring rows (the 4 ring rows plus `a_ring_stored_a_period_away…`):
       - unmutated: 5 PASS;
       - drop: 5 red;
       - filled hole (`nopar`): 5 red;
       - ring reversed: 5 red (`TrimLoopOrientation`);
       - +2π in u: 5 red (`MismatchedWinding`);
       - +2π in v: 5 red (`MismatchedWinding`, `PastPole`, `BothNappes`, `Folded`);
       - ring reversed with the orientation check skipped: 5 PASS, as expected; the doc now says so honestly.
       - I did not rebuild "ring read where stored".
    4. **Held.** `curved::tests::a_ringed_face_here_refuses_as_a_routing_defect` passes. `lane_of`'s `has_ring` comes ahead of every curved predicate (by inspection).
    5. **Held except `Copies` (MINOR-A).** Each cause has its own sentence and one `name()`, which Display, the binding and `errors.rs` read (the row asserts `name()` per cause and a substring of each text). The `TrimLoopOrientation` text now covers a ring wound with its outer loop. Bodies I raised myself and checked against the text: `Gap` (true), `RingEmpty` (true), `Copies` (false, above).
    6. **Held.** Every row asserts by variant, plus a cause word or count: `Got::Unread(kind, "undecided")`, `CertificateExceeded(Cone)`, `Folded(Sphere)`, `NonManifold(4)`, `NoArea(Torus, 2)`. Each can go red. The frontier row states each ε's refusal. `a_ringless_cone_face_near_its_apex_meshes_an_edge_of_four` pins a wrong mesh and says so in its own doc, with a filed issue. Its "same on main" is BY-INSPECTION only: the path is ringless and unchanged. The refusing rows are unfinished, not wrong, and filed. I ran all 7 new or changed rows: PASS.
    7. **My first findings:**
       - MINOR-1 (empty-ring panic): CLOSED for the empty ring. The neighbouring shape (empty outer loop) now refuses with an untrue text: MINOR-A.
       - MINOR-2 (issue says STL refuses): not re-read in this delta (`work/…triangle-of-no-area.md` was edited; check that its STL sentence was corrected).
       - NOTE-1: RETRACTED (claim 2).
       - NOTE-2 (release builds rely on `check_mesh`): open, pre-existing design.
       - Style: two-reads-of-the-door CLOSED; `TrimPolygon` parallel vectors CLOSED; orientation doc CLOSED; stale `PoleInside` in the open sphere-trim row CLOSED (edited); Display pinned for every cause CLOSED; `sizing::outer_loop_curves` second reader: open, unchanged and argued.
    
    ## New findings
    - **MINOR-A (DEMONSTRATED). `RingPlacementError::Copies` text, `crates/mesh/src/types.rs` Display.** On a tier-1-valid curved face whose outer loop is `Empty` and whose ring is a `Cycle`, `face_lift` answers `Copies`. The text blames the outer loop's period span. The `Copies` doc's second arm ("a shift's count of periods has no `i32`") has no words in the text, and neither arm is the true cause. Main answered this face with the typed `RingOnCurvedFace`; the crate's documented answer is `EmptyLoop`. `face_lift` reads an empty outer loop as a closed lift and goes on to place rings against it.
    - **NOTE-A (BY-INSPECTION).** `tessellate_impl` now reads `face_domain` before the memo lookup, so a memo hit still pays one `face_lift`, ring placement included. That is the price of folding the placed images into the key; it is not measured.
    
    ## Style (delta only)
    - `trimmed.rs` `TrimPolygon::next`: a `partition_point` per call, called per side and in the ear test's closure. Fine at these sizes; `unsure` it matters.
    - `tessellate.rs` `(Lane::Trimmed, None) =&gt; unreachable!`: the domain travels as an `Option` beside the lane, kept consistent by the match above it. An enum carrying the `FaceLift` in its trimmed arm would remove the arm. `likely` (taste).
    - `curved.rs` routing guard returns `MissingEntity { what: "...(router defect)" }`: a defect spelled as a missing entity, and the test matches on `what.contains("router defect")`, a sentence. `likely`
    
    ## Executed
    - Re-ran the env-switched mutants (rebuilt on the new `TrimPolygon`) over 5 rows × 7 settings.
    - The empty-loop probe in 4 shapes × 2 δ.
    - The 7 new or changed rows.
    
    Read the fix diffs, `memo.rs` key serialization and `pcurves_rings.rs` `face_lift`. Tree restored to the head; nothing committed or pushed.
</cross-session-message>

0 notifications remain queued.
