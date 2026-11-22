FINAL DELTA REVIEW: PR 7096 at 7a6f5bfe6e (supersedes my interim). The branch tip was still 7a6f5bfe6e at 03:06Z.

**Verdict: APPROVE-WITH-FIXES.** Both MAJORs are cured and no new MAJOR. Before merge, the PR description has to be rewritten (claim 8): at 03:06Z it is still the frozen head's text.

## Claims
1. **MAJOR-1 cured: HELD.**
   - Probe: my own, release, the merge base 54938d6a against head. It records each cell's verdict and triangle count, with an edge census by vertex id.
   - Bodies: cones through the apex at 0.1, 1, 5, 30 and 45°, each at 0.05, 0.3 and π rad; a 5° cone ×1000 at 0.05 rad; the bicone [(0,0),(1,1),(0,3)], full and 0.05 rad.
   - Splits: every line at once and each line alone, at 1, 2 and 5 points; placed and unplaced. δ from 10 (×1000 for the scaled cone) to 0.01.
   - Result: about 1,700 comparable cells. All mesh manifold on head, and head is never denser than base.
   - Head alone at ε 1e-6 and 1e-12: the same.
   - Not compared: the 0.001° cone on base, which hung past 150 s (base pathology).
   - The fans take their triangles by construction. The first triangle holds b0–a0 and the last holds aₙ–b_m. Two triangles hold the same side only when n+m = 1, and that is m = 0, n = 1, the zip's case. So I find no other face where the claim fails.
   - The zip: both chains run from the rim in. Triangles are wound as (a0, a1, apex), and every one has a corner on each side.
   - Mutants: Z1 (zip off) is red on 4 rows. Z2 (zip order reversed) is red on `ruled_walk::a_one_chord_face_to_the_apex_zips_its_sides` and `a_one_chord_face_through_the_apex_with_both_sides_split_meshes`.
2. **MAJOR-2 cured, but over-cut: CORRECTED (cured; costs density it need not).**
   - Soundness: holds. `cert::verdict` certifies only when the bound ≤ δ + allowance, with allowance ≤ ALLOWANCE_SHARE·δ. Every lane but the planar one goes through it, including curved.rs:449. A planar neighbour's midpoint is at distance 0, so the rule always cuts there.
   - A foot outside the trimmed domain errs toward cutting, the safe side.
   - Gap: the neighbour's computed bound can sit under its true deviation by its own rounding. This is covered only while the band dominates that rounding. **unsure**, NOTE.
   - Completeness: `faces_across` reads edge neighbours only. A face touching this one at two vertices and no edge is not read. I found no body that builds one (**unsure**). The cut point is a Local vertex on an interior side, so it makes no new shared edge.
   - Regression, DEMONSTRATED. I added an env-hooked mutant, cut never.
     - With the cut off, the walk is non-manifold only on the bored ball (sign −1 at δ 0.1 to 3; sign +1 at δ 0.3 to 3).
     - Head still cuts where the walk needs no cut:
       - a bored torus (bore x=1.6 through a torus of R 2, r 0.5), both signs, δ 0.1 to 3;
       - a cone-bored ball, both signs, δ 0.1 to 3;
       - a bead (sphere r 1.2 with bore r 1), δ 0.3 to 3;
       - the bored ball itself at δ 0.04 to 0.05, both signs, and at δ 0.1 for sign +1.
     - So 208 of about 1,100 such cells mesh denser than base, all manifold on both trees. Example: bored torus, sign +1, line 0 split at its middle, δ3: walked face 48 triangles, against 26 on base and 16 for the unsplit strip.
     - The implementer's δ 0.04 ↔ 0.035 threshold on the bored ball is the rule's threshold. The cut is needed only from δ 0.1 or 0.3 up.
     - Not a correctness issue, but it misses the unit's density goal on bodies with a curved neighbour round a bore, at coarse δ.
   - The one-chord band's +4: not needed. The neighbour on the same cylinder cannot make the edge. It is a miss of the rule, documented at the site.
   - The decided comparison: `k_stats::decide("walked side beyond its neighbour's reach", …)` and `walkable`'s `is_singular` reads in `route_of` add K-stream entries for split-side bodies. Nothing names or pins them. `route_rows` is unchanged, and it passed under every mutant (filter 'route').
3. **The route carries the walk: HELD.**
   - Fact 4 is answered once, by `walkable` (apex via `AnalyticRow::is_singular`), as `Trim::Walked` → `Lane::Walked`.
   - The readers are `face_target` and the chord pass (`Lane::Trimmed | Lane::Walked`), the memo key (`Lane::Walked => 3`, so walked and structural keys differ) and the dispatch (`walked` bool).
   - `walked()` is gone, and no second predicate remains.
   - `RouterDefect` is reachable only if `Rows::of` disagrees with `walkable`. Both read the apex by `is_singular`, so no valid body reaches it on my sweep: no cell refused at all.
   - The undecided-apex and both-nappe faces stay `Structural`, as before.
   - Mutant R1 (route's walked fact off): red on 9 rows.
4. **Nothing outside the class moved: HELD.** My own per-patch differential: base 54938d6a against head; release, CAD_FUZZ_SEED=1.
   - Suites: mesh whole (head 539/539), step-import 351/351, and sweep's 62 modules that call `mesh::tessellate` (578/578 on both trees).
   - 461 common tests. 80,639 patches byte-identical.
   - Moved patches appear only in 6 tests, all split-side bodies: the curved multiply-carried iso side and the split-placed wedge, sweep's `a_box_along_an_inflection_ruling`, reach_probes' cone on its axis, and split_rulings' draft-cone and frustum-seam rows.
     - Cone 59,557 → 1,585 triangles; Cylinder 5,544 → 940; Plane 4,376 → 3,672.
   - NO refusal added or removed.
   - Renamed rows are not in the common set.
5. **7054's re-split is pinned again: HELD.** With `developed_runs` returning no runs, 5 rows go red, against 2 at the frozen head:
   - `a_run_on_a_cone_face_cut_by_an_oblique_plane_meshes`
   - `a_run_on_a_cut_cone_faces_seam_meshes`
   - `a_run_of_three_on_a_cut_cone_face_meshes`
   - `two_runs_anchored_by_the_apex_on_a_cut_cone_face_mesh`
   - `a_run_anchored_by_the_apex_on_a_needle_refuses_unresolved`
   
   The undecided apex is pinned (`a_split_beside_the_apex_of_a_needle_cone_refuses_undecided`); it matches my frozen-head cell. I did not run the per-behaviour mutants (corner anchor, run of three).
6. **Smaller findings.**
   - FIXED:
     - rim_runs (`rim_run_starts`);
     - the apex read (one reading, `is_singular`);
     - `has_trim_carrier` asked twice;
     - `MissingEntity` → `RouterDefect`;
     - the dup-id `continue`;
     - the trimmed module header and sizing.rs's opening;
     - stale citations (grep clean);
     - the 7054 rows renamed;
     - the volume-unread arm;
     - the CI slow-set entry;
     - the deleted row (subsumption accepted).
   - FIXED DIFFERENTLY / PARTIAL:
     - The carrier is still spelled two ways: `edge_carrier` in `walkable`, `carried_by_a_line` in `walk_face`.
     - "No edge of four" is now enforced by `may_make` (good). But `walk_face`'s doc still says the triangle is "replaced by the three joining its sides to the surface point at its chart centroid… each adds two". The code cuts the SIDE at its chart midpoint into two triangles. Its `# Errors` still says `MissingEntity` where the code returns `RouterDefect`. (MINOR, by inspection)
   - NOT AS CLAIMED: the singular flags passed to `row.cert` (trimmed.rs ~3617/3691) carry no reason at the site.
7. **Theorem beside the step functions: HELD.**
   - `sizing::tests::a_walked_triangle_lands_at_delta` is the shape of `torus_grid_steps_meet_the_bound_with_equality`: within δ, and landing at δ to 1%, on a cylinder, a frustum and a face to the apex.
   - ruled_walk.rs:27-41 now states the apex row and the zip.
   - The test does not exercise the zip or a cut side. NOTE.
8. **PR description: FALSIFIED** (read 01:55Z and 03:06Z; unchanged since 01:51Z). It still says:
   - `trimmed::walked`, `tessellate::lane_of`;
   - "The walk needs no shared-side cure";
   - "Nothing new filed";
   - "154 cells", and the digests "before the 7054 merge".
   
   It has no `[mesh.sizing]` quotes, no stage-1 scope caveat, no `RouterDefect`, no route integration, and no account of the cut or its density cost. It needs the whole fix pass before merge.
9. **What is filed: HELD, with one stale line.** The unsplit bored ball is non-manifold on head: the sphere sign at δ 3, 1 and 0.3; the torus sign at δ 3, 1, 0.3 and 0.1. That is exactly the file's cells. The bored torus, bead and cone-bored ball unsplit are manifold.
   - The file's "What cures" paragraph first says the walk "cuts every walked side whose two ends one neighbouring face holds". The narrowed rule cuts less.
   - Its "candidate cure" for the strip would inherit the over-cut I measured. NOTE.
10. **Mutants: HELD.** Each mutant and the rows it turns red:
    - Z1, the zip off: 4 rows.
    - Z2, the zip order reversed: 2.
    - R1, the route's walked fact off: 9.
    - cut never: 1 (`a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`).
    - cut always: the same 1.
    - T1, the threshold at δ alone: the same 1.
    - H1, the headroom kept on a walked face: 7.
    - D1, the re-split off: 5.
    
    Weakness: `Walked::SharedSidesCut` accepts any even surplus, 0 included. So no row bounds the over-cut, and cut-always passes the 218-cell row. (MINOR, test quality)
11. **CI at 7a6f5bfe6e: green but for `test (kernel)`.** Its one red is the profile fuzz row `seg::pair_contact_tests::every_pair_the_shared_end_touch_rescues_escalates_at_its_joint` (ε 1e-6, random seed 0x68011bdfc75b3870). The crate is untouched: not this PR's. `gate ok` is therefore red.
    - Green: test (app), python suite, lint, tools, demos, the budget meter, viewer wasm, step import, corrupt input.
    - Locally I ran mesh whole release (539/539, default filter), step-import, sweep's meshing modules, and my probes at the three ε.
    - NOT run: geom-brep whole, mesh `--ignore-default-filter`, the viewer slow-set rows, the Python suite locally.
12. **Other breakage:** none found beyond the above.
    - `RouterDefect`'s Python projection shares `MissingEntity`'s arm, so the face key is dropped from `note` (**unsure** whether other variants project the face).
    - `walkable` builds `AnalyticRow::new(surface, f64::NAN)` with a dummy target. Style.

## Findings
- **MINOR (DEMONSTRATED):** `trimmed::may_make` over-cuts. In 208 cells the head is denser than base where the walk alone is manifold (claim 2). No row bounds the surplus (claim 10).
- **MINOR (BY-INSPECTION):** the PR description is stale throughout (claim 8).
- **MINOR (BY-INSPECTION):** `trimmed::walk_face`'s docs describe a centroid split and `MissingEntity`; the code does neither.
- **NOTE:** the K-stream entries from `decide` and `walkable` are unnamed and unpinned. The singular-flag reason claimed by the fix pass is absent. The carrier is still spelled two ways. The filed issue's first cure sentence is stale.

## My first review's findings
- CI slow-set: fixed.
- [mesh.sizing] quote: not in the description yet.
- 7054 rows' coverage and docs: fixed.
- sizing.rs:20: fixed.
- Seam skip: removed.
- Apex readings, rim_runs, has_trim_carrier, MissingEntity: fixed.
- Q7 routing: fixed by the integration.
- The walk's text on the apex row: fixed.
- The needle pin: fixed.

## Executed and read
- Executed: my probe (kept at /tmp/claude-0/-home-user-cad/d484124a-0ba4-5a04-b0e2-fb843aed7ea5/scratchpad/review_7096_delta.rs.keep), about 3,100 cells on head at three ε plus the base comparison; 8 mutants against 79 rows; the 3-crate digest differential; the hosted CI logs.
- Read only: ruled_walk.rs whole; the trimmed / tessellate / memo / chords / types diffs; the sizing test; the filed issue; the pncad-py diff.
- The tree is restored to 7a6f5bfe6e. Nothing committed, pushed or posted.
