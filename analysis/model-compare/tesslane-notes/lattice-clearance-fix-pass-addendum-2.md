IMPLEMENTER tesslane/lattice-clearance — answer to your notes of 05:18 and 05:39 (both reached me after my addendum of ~05:35, which already carries the re-baseline). HEAD is `05a7f8dbfd9b6fe026b2b4f7005f1a0229998e1c`: ONE commit past the head the reviewers read (6f41f1b0ef), and it touches exactly two files, `crates/viewer/tests/display_budget.rs` and `crates/viewer/tests/review_pick_r2.rs` (the two pins). I pushed it before your "do not push without telling me" arrived; nothing else has been pushed and nothing is uncommitted.

1. THE TWO PINS (re-baselined at 05a7f8dbfd; both green there with `--ignore-default-filter`).
- Cause: the clearance, first pass. Shown: with `crates/mesh` from the first-pass head `ba68f8ed0a` the pick tally is the identical (451512, 147856, 16596, 8142). Not the both-bands rule; no fix-pass item moves a mesh.
- Why I counted 1,551 and hosted CI is green: I ran `--profile ci`, whose default-filter skips the slow set; a plain `cargo nextest run -p viewer --features app` (default profile) does include them. Run whole now: 1,589 of 1,592, the three reds the adapter-less GPU rows.
- `display_budget::the_budget_commits_the_delta_it_always_has`: of 68 answers two differ, both `cut_cylinder` (a cylinder cut by an oblique plane; its wall is a trimmed cylinder face): δ 1e-4 largest_probe 4544 → 4520, probe_triangles 7848 → 7816; δ 5e-6 81336 → 81208, 84640 → 84504. Every `delta`, `requested_cost` and `stop` of all 68 is unchanged, so the budget commits the same δ for every document: the row's assertion (the table equals what the fit answers) holds on the new meshes. The table is the generator's: the row prints "what it answers now, as this table's own source", and the two entries are those printed lines; the row passing is the equality of the committed table with that output. No other corpus document's mesh moved, so none other holds a trimmed face that loses a node.
- `review_pick_r2::…`: the four numbers are (rays cast, one per mesh vertex aimed at per landing; rays answered at the aimed vertex; candidate triangles refused at the certified determinant; rays with at least one refusal): (451800, 147952, 16824, 8244) → (451512, 147856, 16596, 8142). Fewer vertices, so fewer rays (−288) and fewer near-degenerate candidates beside them. None of them moving is a change of the picker: the row's ASSERTION is the empty `genuine_refused` list (no well-conditioned crossing refused) and the empty wide-winner list, and both are still empty; the tally is a pin on the corpus's meshes. Per-document attribution of the tally was not taken; the budget corpus says `cut_cylinder` is the document whose mesh moved.

2. THE CLASS SWEEP: slow-set rows that pin a mesh-derived number. The slow set is skipped only under `--profile ci`; every run below is the DEFAULT profile (or `--ignore-default-filter`), so it includes the slow set.
- `-p mesh` whole (497): green. D9 digests re-baselined in the first pass (4 of 40, tabled). Budget feature job: green; `docs/tess-budget-data` unmoved (spline faces only).
- `-p sweep` whole (3,775, 51 slow): green but the two `mass_props_are_thread_count_invariant` rows, NOW CONFIRMED RED ON `origin/main` ITSELF (worktree at f94c7b0663, that module alone: 3 pass, the same 2 fail). Not mesh-derived, not mine.
- `-p viewer --features app` whole (1,592): the two pins above re-baselined; nothing else moved.
- `-p stl` (18), `-p step-import` (329), `-p pncad` (85), `-p pncad-py` Rust rows (132), Python wheel suite (959): green, nothing mesh-derived moved.
- `-p editor-core`: the rows of every file that tessellates (154, 4 slow): green. `-p topo`: the same selection (46): 45 green; `pcurves::lift_agreement_rows::no_file_outside_the_mint_pins_a_chart_branch` red on a count in `mesh/src/walk.rs`, which this branch does not touch (not mesh-derived; main's, NOT confirmed on main by a build).
- `demos/tour`, WHOLE, nightly-only rows included (`cargo nextest run --cargo-profile ci-tour`, default profile): 106 of 106 (9 slow), no pin moved. `demos/wild`: no tests (nextest finds 0).
- tools (k-lint, tess-lint, tess-meter `cargo test`): green.
- NOT run: editor-core whole and topo whole beyond the files that name mesh; benches; `interval-transcendentals`. A consumer of meshes that does not spell `mesh::` or `tessellate(` in its own file would be outside my selection there.
Result: two rows re-baselined (above), one corpus re-baselined in the first pass (D9), everything else green or not mesh-derived.

3. "RUN LOCALLY", lines to add (run with the slow set included):
- `cargo nextest run -p viewer --features app` (default profile, slow set in): 1,589 of 1,592; the three reds are the GPU rows without an adapter. Earlier lines that said 1,551 were `--profile ci`.
- `demos/tour` whole with its nightly-only rows: 106 of 106.
- `-p pncad` whole: 85 of 85; `-p editor-core` and `-p topo`, the rows of every file that tessellates: 154 of 154 and 45 of 46 (the red a source census of `mesh/src/walk.rs`, untouched here).
- The two `sweep` mass-props reds are red on `origin/main` at f94c7b0663.

"WHAT MOVED", line to add: as in my addendum (the `cut_cylinder` rungs and the pick tally, both the clearance's).

I am holding: nothing further will be pushed unless you ask.
