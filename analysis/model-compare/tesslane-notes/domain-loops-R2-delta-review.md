# Delta re-review — PR 6960 @ `23654a82b8`
    
    ## Verdict: APPROVE-WITH-FIXES
    The fix pass does what it says; nothing new in the lane. Two small doc points (D1, D2), and CI is not green on the PR's current head, which is not the frozen one (D3). My container survived the outage; no work was lost.
    
    ## Claims
    1. **Held.** Empty ring (my `mev_line` + `kemr` + `mint_pcurves` recipe) on cylinder, sphere, cone and torus: `topo::validate` passes each, `face_lift` answers `RingEmpty`, `tessellate` answers `DomainUnread{RingEmpty}`; no panic. A ring that is one strut edge (two half-edges; second `mev_line` before the `kemr`), four charts: `mint_pcurves` refuses `Certify{Incidence}`, the door answers `Gap`, the mesh refuses typed (`DomainUnread{Gap}` on the cylinder; `UnsupportedCurve` from the chord pass on the other three). An empty outer loop with a ring: not built; by reading it ends in `Copies`/`NoShift`, not a panic. `a_ring_that_is_one_vertex_answers_typed` passes at the default ε, 1e-6 and 1e-12.
    2. **Held.** `face_domain` is the one call (`tessellate.rs`), handed to `gather` and the lane; `memo` no longer mints a band or calls the door. Differential re-run, merge base `77cf43f7b2` vs this head, same 1602-line probe: 0 of 1098 ringless/no-body lines differ (mesh digest, `check_mesh` answer, refusal). All 504 ringed lines are `RingOnCurvedFace` on base. Head: 441 mesh + check ok, 42 `CertificateExceeded`, 18 `Undecided`, 3 `FoldedTriangle`, and every ringed digest equals the one I took at `4bd6709df9`, so the fix pass moved no ringed byte either. Key digests themselves I did not measure: by reading, the planar and curved keys never folded an image, and a ringless trimmed face folds the same `loop_lift` rows.
    3. **Held.** `TrimPolygon` is points, edges, `ends`; `next(i)` derived; no parallel slices left. Mutants re-authored on this head and run over the four ring rows plus the moved-ring row: dropped, parity, `u` shift, `v` shift, reversed → all five rows red; read-where-stored → ball ∖ tube and the moved-ring row red, cone, cylinder and torus rows green (their rings are stored in place).
    4. **Held.** `curved::tests::a_ringed_face_here_refuses_as_a_routing_defect` passes; `lane_of` is unchanged and still the only caller's router.
    5. **Held.** Raised and read against a body: `RingEmpty` (four charts), `Gap` (strut ring; stripped caches), `OuterOpen` (seamless bands), `Undecided` (cone s = 1e-3; spiric outer loop) — each sentence is now true of its body; the `Undecided` text names all three door causes. `RingOpen`, `NoShift`, `Shifts`, `Copies`, `SplineTrimUnread` I could not raise; read only. `TrimLoopOrientation` covers the ring (reversed-ring mutant raises it). One `name()`, read by Display's test, the binding and both probes.
    6. **Held, one correction (D1).** The frontier row, the non-manifold row and the moved-ring row pass at the default ε, 1e-6 and 1e-12, each pinned by `Got` variant or by the boolean's `Debug` variant prefix. The non-manifold row says in its own doc that it pins a wrong mesh. `Got::NoArea(Torus, 2)` matches my count.
    7. **My first findings.**
       - m1 viewer clippy: closed (`Box`, 80 bytes; that job is green on run 43632075942).
       - m2 cone ring near the apex: open by design, pinned and on its row.
       - m3 spiric outer loop: open by design, pinned and on its row.
       - m4 `TrimLoopOrientation` text: closed.
       - m5 `Gap`/`Undecided` text: closed.
       - m6 empty-ring panic: closed.
       - m7 STL sentence in the filed row: closed (now says STL writes the facet).
       - m8 moved-ring rows: closed.
       - N1 non-manifold cone: pinned and filed. N2 near-pole fold: pinned.
       - S1 (two door reads), S2 (curved guard), S3 (`TrimPolygon`), S4 (dead arm), S5 (three spellings), S6, S7, S8, S10: closed. S9 (retired lily wall kept as code): unchanged.
    
    ## New findings
    - **D1 MINOR, DEMONSTRATED.** `reach_probes.rs` `ringed_curved_faces_that_still_refuse_answer_by_cause`, doc: "at `s = 10⁻³` the ring is within the band of the outer loop". The door's payload there is `Undecided { cause: None }`, and `cause` is the in-band decision when one stopped the reading, so the band is not what answered. The row pins only `undecided`, so the sentence is an unchecked cause beside a true pin.
    - **D2 NOTE, BY-INSPECTION.** The PR body still describes the first head: `tessellate_trimmed` reading the door, `PoleInside`'s account, a successor stored per point, the stored-ring mutant red on one body, clippy without viewer. It is the change's logical record and is stale after the fix pass.
    - **D3 NOTE, DEMONSTRATED as a fact, cause unchecked.** The PR's head is now `74ad36775e`, past the frozen head; I did not read it. On run 43632075942 `test (kernel)` and `gate ok` are failures. The three failing rows are in `sweep` at non-default ε: `band_strip_mouth_cups::a_cups_mouth_a_hair_across_the_trimline…` and `tilted_sphere_pair::a_section_passing_near_a_pole…` at 1e-6, `full_turn_wall::both_doors_answer_the_bead_by_its_height_window` at 1e-12. Each fails inside a boolean, pcurve or props door, before any mesh call. I did not check whether main fails them; I did not confirm which SHA that run is on.
    
    ## Style (delta only)
    Exercised: Q1, Q2, Q3, Q5, Q7 on the fix commits.
    - `curved.rs` ring guard — the routing defect is `MissingEntity` with "(router defect)" in its `what`, and the row matches on `what.contains`. A refusal identified by a substring of its sentence. **likely**
    - `errors.rs` `each_domain_unread_cause_has_its_own_sentence_and_word` — nine causes written out by hand; a tenth cause would compile and not be read by the row. **likely**
    - `tessellate.rs` `(Lane::Trimmed, None) =&gt; unreachable!` — the lane and its domain are two values the match has to reconcile. **unsure**
    - `reach_probes.rs` `refused_at` — float equality on ε picks the expected refusal; an ε the table does not list silently means "builds". **unsure**
    - `trimmed.rs` `domain_loops` — still a `Vec` of loop refs beside `FaceLift`, now used by `memo` and `singular_vertices` only, while `trim_polygon` walks `outer()` and `rings()` itself: two ways to iterate one domain. **likely**
    
    ## Executed
    Head and merge-base test builds; the 1602-line differential on both; six mutants over five rows; empty-ring and strut-ring probes on four charts; the four new rows at three ε; the curved lane's unit row. Read: the four fix commits under `crates`, the changed `work/` rows, the PR description, check runs and one failed job's log. Not run: sweep, tour, pncad-py suites. I saw no other review's material; the PR now shows one comment, which I did not read.
</cross-session-message>

0 notifications remain queued.
