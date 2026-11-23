REVIEW — PR 6960 `tesslane/domain-loops` @ 4bd6709df9 (full review + style lane)
    
    **Verdict: APPROVE-WITH-FIXES**, conditional on a green that does not exist yet: at review time the head's check runs were "change filter" queued and "prime the build cache" skipped. I re-ran no CI-covered suite except `-p mesh` whole once (with a panic armed, below).
    
    ## Claims
    1. **Held.** I armed a panic in `tessellate_trimmed` on any ringless face that misses `face_lift` and ran `cargo nextest run -p mesh` whole: 461/462 pass. The one that fired is `m5_pr11::missing_pcurve_caches_refuse_typed`, a deliberate Gap where `loop_lift` also fails, so main refused it too. By inspection, `face_lift`'s outer loop is `Lift::read`, the same call `loop_lift` makes, so a ringless face gets an identical polygon. The only new door miss on a ringless face is `OuterOpen` (nonzero winding). On sphere and cone charts that state was `PoleInside` on main. On cylinder and torus charts, a face with one loop is a disc, so the loop winds zero. I did not run sweep or the tour with the panic armed.
    2. **Held, with one exception (MINOR-1 below).** The lane places nothing; `domain_loops` reads only the door. Refusal texts: `Gap` says "a finished body stores every one, so this is a kernel defect". A tier-1 scaffolding body (pipe wall plus a strut, then `kemr`) reaches `Gap` and is not finished, so the text is untrue for it (`unsure` on whether a finished body ever reaches it).
    3. **Held for what I could build.** `classify_faces` is reused, not copied. `trim_orientation` checks every ring (unit row). Pinch and ring-touching cases were not built: a ring sharing a vertex with another loop is one loop in a valid B-rep. Two rings: the pipe rows. Seam-straddling rings and rings a period away on each torus channel were exercised only as mutants (claim 4).
    4. **Held. I built every mutant through an env switch on `TrimPolygon` and ran the 4 ring rows:**
       - ring dropped → red on all 4 (debug chord-segment census at `tessellate.rs:613`);
       - filled hole (ring sides add 2 to the crossing count, so parity never toggles) → red on all 4 (same census);
       - ring shifted +2π in u → `check_mesh` `MismatchedWinding` on ball, cone, torus and pipe;
       - ring shifted +2π in v → torus `MismatchedWinding`; on the non-periodic v channels, sphere `PastPole`, cone `BothNappes`, pipe `FoldedTriangle`;
       - ring points reversed → `TrimLoopOrientation` on all 4;
       - ring reversed with the orientation check skipped → **all 4 rows PASS, same mesh.** Even-odd is blind to direction and emission follows the outer loop, so the ring orientation check only checks validity; no mesh depends on it (STYLE-5).
       - I found no body where a filled hole passes `check_mesh`: the ring edges stay constraints, so a filled hole gives non-manifold edges and a dropped ring gives boundary edges. In release builds the census is off, so only `check_mesh` would catch them.
    5. **Held by inspection.** `chords::tighten` runs per edge over both sides, and `lane_of` sends ring faces to Trimmed, so non-circle ring edges are now sized against the trimmed lattice. `singular_vertices` reads every loop. Shared edges are watertight on every census mesh (`check_mesh` passes). I did not tabulate chord counts from each side.
    6. **Held.** Chords are capped at h_u ≤ π/4, so a closed lift has no side of 1.5π, and any loop round a pole or apex winds. `PoleInside` is unreachable, not merely unbuilt. A sphere with a pole inside its outer loop and a ring elsewhere answers `OuterOpen`. A face holding both poles with an un-wound outer loop answers `TrimLoopOrientation` if ringless and `NoShift` if ringed; both texts are true-ish. Spindle not built.
    7. **Held by argument.** v is linear in the chart. A disc region bounded by the outer lift has the same v range as its boundary, and every placed ring lies inside it, so the outer loop alone gives the right `ρ_max` and widest-cos window.
    8. **Partly exercised.** Only the T1 ring was built (it meshes at 12 δ). A ring holding the apex in its hole can only be the seamless frustum, which answers `OuterOpen`. A ring near the apex was not built.
    9. **Corrected (NOTE-1).** Ringless trimmed faces fold the same images as before. **But on the Planar and Curved lanes `gather` now folds `pcurve: None` where main folded `loop_lift`'s images**, so every planar and curved face's key changes. That is harmless (neither lane reads an image; the memo is in-process and generational), but the PR's "Unmoved faces fold `loop_lift`'s images as before" is false for those lanes, and the change is undisclosed.
    10. **Held, with MINOR-1.** The `pncad-py` rename is complete (tag, inventory, shared words, pyi, census). A grep over every root finds `RingOnCurvedFace` left only in `MassPropsError`, which is correct. Ringed spline face: not built.
    11. **Held, with a corrected claim in the filed issue (MINOR-2).** Census over 5 ring bodies × 3 ring-keeping ops × 12 δ (4e-2 … 5e-4), 180 meshes: all `Meshed` except the torus at δ=1e-3 in ∪, ∪ and A∖B, each with 2 triangles of shape 1.67e-15. The next-worst shape anywhere is 4.3e-4. On this head, sweep's tilted pair (ringless, on a path unchanged from main) gives 2 no-area triangles per op in A∪B, B∪A and B∖A at δ=0.04, shapes 1.1e-15 to 5.9e-15. It is one class: a lattice point within rounding of a polygon point, and a ring only adds polygon points. Downstream, `check_mesh` and the volume bracket accept the mesh. `stl` does **not** refuse: the cross length is 4.17e-18, not 0, so it writes the facet with a noise normal.
    12. **Held.** The sweep re-pins assert mesh + `check_mesh` (+ volume on the D-section); lily wall 7 asserts `check_mesh` + volume. The gate line removal is required: the gate reds on a count mismatch in either direction, and it passes.
    13. **See Spec deviations below.**
    
    ## Findings
    - **MINOR-1 (DEMONSTRATED). An empty ring on a curved face panics.** Pipe wall + `mev_line` strut + `kemr` gives `LoopBoundary::Empty` as a ring, and `topo::validate` is OK. With the outer rows' joint and pcurve restored, `mesh::tessellate` panics at `topo/src/pcurves_rings.rs:427` (`unreachable!: a ring's loop holds a half-edge`), reached through `face_lift`. Main refused the same face with the typed `RingOnCurvedFace`, and the documented answer is `EmptyLoop`. `has_ring` routes the face to the door before any `EmptyLoop` check, in `memo::FaceInputs::gather` and in `tessellate_trimmed`. Without the restore, it refuses `Gap`, whose text is untrue for that body (claim 2).
    - **MINOR-2 (DEMONSTRATED).** `work/tesslane/a-lattice-point-within-rounding-…` says "an STL export of it would refuse `DegenerateTriangle`". False: `stl` refuses only a cross length of exactly 0, and these are about 4e-18, so the export ships an arbitrary-normal facet. The P1 row understates the risk.
    - **NOTE-1 (BY-INSPECTION).** The memo key change on Planar/Curved faces, and the PR text about it (claim 9).
    - **NOTE-2 (BY-INSPECTION).** Watertightness against a dropped ring or filled hole is enforced by a debug-only census; release relies on a caller running `check_mesh`. This is pre-existing design.
    
    ## Spec deviations
    - Torus ring "done" at 2 of 3 δ (`NoArea` at 1e-3): REPORTED, and filed P1 `design: true`. A fix that would move main bytes was rightly not taken.
    - The notch is left to unit 4: REPORTED, and noted on the work row.
    - The measurement is in the PR body only (the spec asks for that): fine.
    - The memo key change on non-trimmed lanes: SILENT.
    - The empty-ring panic: SILENT.
    
    ## Style
    - Q1 `crates/mesh/src/memo.rs` `FaceInputs::gather` and `trimmed.rs` `tessellate_trimmed`: the door is read twice per trimmed face per picture. One read swallows the error (`.ok()`); the other maps it. The PR's "read once" holds only per reader. `likely`
    - Q1 `sizing.rs:671` `outer_loop_curves` (`face.outer`) is a second reader of the outer loop beside `face_lift` for the windows, defended in prose; `trimmed.rs:354/388` (`has_trim_carrier`, `has_tilted_circle`) also read the outer loop only. `likely` (the windows are sound; this is about structure).
    - Q7 `trimmed.rs` `TrimPolygon`: four parallel vectors (points/edges/next/starts) kept consistent by convention, and `next` is derivable from `starts`. `TrimmedFace` takes `polygon` and `next` as two loose slices instead of the struct, and unit rows pass `next: &amp;[]` beside non-empty polygons (`trimmed.rs` ~4096), which would panic the day a path indexes it. I would have passed loops (`&amp;[&amp;[_]]`) or the struct. `likely`
    - Q2/Q5 `trimmed.rs` `trim_orientation` doc: "refuses rather than mesh by one of them" implies a meshing hazard. The reversed-ring mutant with the check skipped meshes identically (claim 4); the check is a validity check only. `sure`
    - Q2 `trimmed.rs` `singular_row_shape` doc: "So no singular point lies strictly inside a polygon read here" is now prose that replaces a check, with no assertion behind it. `unsure`
    - Q4 `work/tesslane/the-sphere-trim-shape-refusal-names-the-sphere-on-every-singular-row-chart.md` (open) still lists `PoleInside` among the causes. `sure`
    - Q3 `crates/mesh/tests/errors.rs`: the Display row pins only `OuterOpen`'s text; the other six cause texts are unpinned. `likely`
    - Q3 `errors.rs` `two_faces_refusing_differently_report_the_first_in_arena_order`: narrowed to ringless faces, so a ringed poisoned face is no longer exercised. `unsure`
    - Q3 `reach_probes.rs` `geometry`/`Miss::NoArea`: the no-area check went from a failure to a recorded variant for every row, and `NoArea` keeps only the first kind. It still reds where a row expects `Meshed`. `unsure`
    - Q5 `lift_probes.rs` doc: "a face that holds a singular point inside it is this one refusal" is said of the frustum, whose apex is in its ring's hole, not in the face. `unsure`
    - Q2 `TessellateError::DomainUnread` Display for `Gap`: "a finished body stores every one" (claim 2). `unsure`
    - Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 partial: I read `trimmed.rs`'s domain, polygon, orientation and singular passes and the constraint and emit passes, plus `memo.rs` gather whole, but not the 4.4k-line file end to end.
    
    ## Rubric
    - Idiom/structure 3: the door is genuinely the one source, but `TrimPolygon`'s parallel vectors and loose `next` slice, and the door read twice.
    - Test quality 4: the ring rows go red on every mutant I built. The ring orientation refusal pins validity, not geometry; the Display row covers one cause.
    - Doc honesty 3: the memo claim is wrong, the orientation rationale is overstated, the issue's STL claim is false, and a stale `PoleInside` reference remains in an open row.
    
    ## Executed / read
    Executed:
    - `-p mesh` debug build; the full mesh suite with the panic armed;
    - a 180-mesh ring census, extra shape and STL-normal probes, and the tilted-pair class probe;
    - 6 env-gated mutants over the 4 ring rows;
    - the empty-ring Euler probe;
    - `scripts/gates/loop-boundary-discards.sh`.
    
    Read only: sweep and tour re-pins, the `pncad-py` diff, the work rows.
    
    I committed and pushed nothing, and restored the tree to the head. I saw no other review's material; of the PR I read the description and check runs only.
    
    Tests worth keeping (my local probes, not committed):
    - The empty-ring row: `rod` pipe → `mev_line` Fan strut off the cylinder face's outer `first`, then `kemr(strut.he_plus, strut.he_minus)`, restoring the saved joint and pcurve rows; `tessellate` must return a typed refusal rather than panic.
    - The ring census: the shape histogram over the reach ring bodies at 12 δ.
</cross-session-message>

0 notifications remain queued.
