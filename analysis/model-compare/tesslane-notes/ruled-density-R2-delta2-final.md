FINAL, second delta of PR 7096, section A. Read at 8ad2d3fd0f. Probes and runs re-taken at 469c4b6c54. 826552a0e9, which removes RouterDefect, I read by diff only. Description read at 21:30Z (updated 21:03Z).

**Verdict: APPROVE-WITH-FIXES.** No MAJOR. The fixes are small: the description's stale lines (A7) and one unstated rounding premise (A4). Hosted CI on 826552a0e9 is still queued ("change filter" queued since 21:03Z), so no CI verdict yet on any head after 7a6f5bfe6e.

## Claims
- **A1. Main's arms unchanged by the lift: HELD.**
  - By inspection, `faces_holding_side` is main's inline test, `fa.iter().any(|f| fb.contains(f))`, as a non-empty filter, with the same loop-neighbour test.
  - Differential, base 54e6a5c5b8 against head 8ad2d3fd0f. Release, CAD_FUZZ_SEED=1, every tessellate call hashing each patch. Suites: mesh whole, step-import whole, and sweep's 63 modules that call mesh::tessellate.
  - 479 common tests; 81,521 patches byte-identical.
  - Moved patches appear only in 6 split-side tests: the curved multiply-carried iso side and the split-placed wedge, sweep a_box_along_an_inflection_ruling, reach_probes' cone on its axis, and split_rulings' draft-cone and frustum-seam rows.
    - Cone 59,557 → 1,585 triangles; Cylinder 5,402 → 940; Plane 4,376 → 3,672.
  - NO refusal added or removed. No mover in any other test.
  - My digest classes by surface kind, not by lane. I can't separate the 24 rectangle-lane rim neighbours, but they sit inside these tests.
- **A2. The walk against main's rule and census: HELD.** My probe (~3,100 cells: cones 0.1–45° at 0.05/0.3/π, the bicone, ×1000, the bored ball ±, a bored torus, a cone-bored ball, a bead, a ring; splits at 1, 2 and 5 points; placed and unplaced; δ 10 to 0.01).
  - No walked cell reaches OverusedSide or UnclearedSharedSide. The only refusals are main's 7 unsplit bored-ball cells.
  - Cut off (env hook, and also the source mutant): the census refuses exactly the bored ball. Sign +1, the TORUS (I printed the surface kinds), at δ 3, 1, 0.3; sign −1, the SPHERE, at δ 3 to 0.1. That is the implementer's fourteen-cell set.
  - On the bored torus, cone-bored ball and bead every cut is an over-cut: the walk alone is manifold at every δ.
  - `holds_the_relation`'s per-cell count pins the number the RULE cuts. It goes red on a change to the rule (mutants: cut never turns 4 rows red; undecided-read-as-greater turns `a_side_whose_reach_the_band_cannot_decide_is_cut` red). It cannot tell a needed cut from an over-cut, and my over-cut bodies aren't among its rows.
  - The tip gives byte-for-byte the same probe results as 8ad2d3fd0f.
- **A3. A gain against main: HELD, with a count to restate.** Against main at the merge (not the old base), head is denser on 256 of ~2,640 comparable cells and never refuses where main meshes. No MAJOR.
  - The 256: the bored ball at δ 3–0.04, both neighbours, the needed-cut cells included at +2 (62→64), which the description's table shows; the bored torus and cone-bored ball at δ 0.1–3 (+8 each); the bead at δ 0.3–3 (+4 to +16).
  - Not compared: the 0.001° cone; main hung past 200 s on it, and head answers it.
  - The description's "208 of about 1,100 cells denser than the base" is my old count against the pre-7198 base. Against merged main it is 256 in my set (and the other review's 440 in its own).
- **A4. may_make sound: HELD, one premise unstated.**
  - `cert::verdict` certifies only bound ≤ δ + allowance, with allowance ≤ ALLOWANCE_SHARE·δ.
  - A planar neighbour cuts, because its midpoint is at distance 0. But the doc's premise, "a triangle the neighbour emits passes its certificate", does not cover the planar lane, which has none.
  - A foot outside the trimmed domain errs toward cutting. Undecided cuts, now pinned by `a_side_whose_reach_the_band_cannot_decide_is_cut`.
  - The neighbour's computed bound can sit under its true deviation by its own rounding (when bound ≤ δ the allowance is not consulted). That is covered only while the band dominates that rounding, and it is NOT stated at `trimmed.rs` `may_make`'s doc (~3556–3570). NOTE, BY-INSPECTION.
- **A5. The router defect: CORRECTED, then fixed by 826552a0e9.** As I reported in the interim, `[failure.taxonomy]` row 4 rules out a typed variant. At 826552a0e9:
  - `RouterDefect` is gone, and pncad-py and errors.rs are identical to main.
  - Main's sites are as main has them. The only diff against main in chords.rs is the `Lane::Walked` arm.
  - The walk's two sites are `unreachable!`, each with its row-0 reasoning and a row-4 message giving the face and the counts.
  - I accept both row-0 answers.
  - work/comb/S43.md:27 still cites main's `MissingEntity` form, correctly, as idiom 2.
- **A6. The merge: HELD** (remerge-diff read on all nine files).
  - Main's side is intact: PR 7198's rule via `faces_holding_side` in `emit`; `joined`/`overused_side` at the join; `UnclearedSharedSide`; `walk::sides`.
  - The unit's side is intact: `Entry::Walked` and `Trim::Walked` in `Routes`; the memo arm (`Lane::Walked => 3`); the dispatch; the zip; the 7054 re-pins (the rows pass at three ε).
  - `route_rows` now pins the walk's K-stream entries (`a_split_side_body_records_the_walks_decisions`, 4e82b645ea). The cut-never mutant turns it red.
- **A7. The description: CORRECTED.** True on what I measured: the bored-ball table's needed cells and labels, the exception, the cut-never cells, the D-prism statement, the taxonomy paragraph. Stale:
  - "What moved": "Not yet re-taken on the integrated head". My differential above is a take on the integrated head.
  - The 208-cell line (see A3).
  - `walkable`'s apex is now read by `chart_table::at_a_pole`, not `AnalyticRow::is_singular` (the same decision).
  - The mutants table: "undecided read as greater: being built". It exists and is red.
  - "(The commit that does this is landing)": it has landed (826552a0e9).
  - The bore item's labels are fixed at 826552a0e9 (469c4b6c54 had them reversed).
- **A8. CI and local runs.**
  - Hosted: only queued runs on 469c and 826552.
  - Local, release, at 469c4b6c54: mesh whole with `--ignore-default-filter`, 546/546 at the default ε, 1e-6 and 1e-12. split_rulings, route_rows, errors, ruled_walk and the sizing row: 52/52 at 1e-6 and 1e-12.
  - At 8ad: step-import has 3 reds (nist_ftc_09 `step_snap_relation` in band: `snap::tests::nist09s_five_cylinders…`, `wild::wild_refusals_are_typed…`, `tier_gate::every_corpus_import…`), and sweep has 1 (`bisector_nodes::the_near_node_poses_at_the_witness_band`). All 4 are identical on base 54e6a5c5b8: main's, in release.
  - NOT run: `test_binding_census` against a wheel, the viewer slow-set rows, geom-brep.
- **A9. My earlier findings.**
  - First review: all fixed, except the [mesh.sizing] text, which is quoted in the description and goes to Ev.
  - Delta:
    - over-cut: kept, stated and bounded per cell (accepted as a stated cost);
    - description: rewritten, with the stale lines above;
    - walk_face doc, singular-flag reason, one carrier spelling, K-stream pin: fixed in 4e82b645ea;
    - the `Walked::SharedSidesCut` weakness: fixed (exact per-cell counts);
    - the filed issue's labels: fixed at 826552a0e9.
  - Interim: RouterDefect: fixed (826552a0e9).

## Findings
- **NOTE (BY-INSPECTION)**, trimmed.rs `may_make` doc: the rounding premise is unstated, and the planar lane is outside its certificate premise (the result is still right).
- **NOTE (BY-INSPECTION)**, the PR description: the stale lines in A7.
- **NOTE (DEMONSTRATED)**: over-cuts on curved neighbours round a bore that no row holds: the bored torus, cone-bored ball and bead, +4 to +16 triangles at coarse δ against main. They are the stated class; the rows hold only the 1.05 ball and the band.

## Executed
- My probe at 8ad and at 469c, plus the cut-never variant.
- The base and head per-patch digests over three crates.
- The mesh whole suite at three ε.
- Two source mutants: cut never, and undecided read as greater.
- A probe of the bored ball's surface kinds.

## Read only
- The remerge-diff of the merge.
- 634dc16100, 4e82b645ea, 469c4b6c54 and 826552a0e9.
- DESIGN `[failure.taxonomy]`, S43, the PR description.

The tree is clean; nothing committed, pushed or posted.
