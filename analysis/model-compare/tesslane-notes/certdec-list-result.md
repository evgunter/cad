certificate-decides: the pre-push list's result on the frozen head 1cfa09180c (branch unchanged since; nothing pushed; `git status` clean). Dev profile, one chain, complete. It reads exactly as the first pass on 73a4fdd625: no new red.

GREEN: `cargo fmt --all --check` and the benches'; `work.py lint`; `scripts/run-gates.sh` (every gate, `bracket-mid-by-hand` among them); `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`; `cargo clippy --workspace --exclude viewer --all-targets --all-features -- -D warnings`; the same on `viewer`; `cargo clippy -p mesh --all-targets --features budget -- -D warnings`; the viewer's `display_budget::the_budget_commits_the_delta_it_always_has` and `review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus` (both at my re-pinned values).

WITH THE KNOWN REDS ONLY:
- `mesh` whole suite at ε 1e-9, 1e-6, 1e-12: 562 of 563 each; the red each time is `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause` (the near-pole fold).
- `cargo test -p mesh --features budget --no-fail-fast`: lib 253/253, `tests/all.rs` 316/317 (the same one red), doc tests none.
- `sweep`'s 131 meshing modules, 934 tests, ε 1e-9: 932 pass; red `tilted_sphere_pair::a_circle_near_the_pole_meshes_at_a_coarse_delta` (the fold) and `bisector_nodes::the_near_node_poses_at_the_witness_band` (main's listed red).

NOT RUN on 1cfa09180c: `scripts/payload-rung-sweep.py --check` (you said it is red on main), `step-import`, `editor-core`, the tour, `sweep` at the other two ε, and the four mutants (all last run before the final three merges, as the body says). The body's sentence "the whole list above is running again and is NOT yet in" can now read: the list on 1cfa09180c is complete, with the same result as on 73a4fdd625.

Holding: no push until you say. Ready for the fix pass when the reviews come back, and for the integration merge after PR 7573.
