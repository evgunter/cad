MERGED-RUN lane — FINAL REPORT, part 1 of 2 (the PR body is part 2).

5. PUSHED HEAD: 64e55e46990c5d10adc5e457df8fa5bee073879a on tesslane/merged-run (confirmed with `git ls-remote`). It is a merge of origin/main (PR 7049 and 43 later commits) over the unit's last commit 9bb51c669e. No PR opened, no GitHub comment, hosted CI not waited for.

1. THE MEASUREMENT, AND WHAT THE SPEC HAD WRONG
 - Base (0700268f2a), spec's bodies, 7480 cells per ε (5 half-angles × wall/solid × full/half × each line edge × 4 split shapes × 8 gaps × 3 δ): default ε 1296 DevelopedMerge / 5337 Meshed / 27 UnresolvedWinding on a PLANE face / 820 no-split; ε 1e-12 1479 / 4269 (+1248 that tessellate with the volume unreadable) / 300 Plane / 4 no-body. Thresholds match the spec's. Every merged group, both ε, is a run on one ruling (2 or 3 points); zero two-sides, lattice-node, two-loops or apex merges.
 - Census elsewhere: `mesh` suite whole (default profile, slow set), `sweep`'s 64 meshing modules, `step-import` whole, `demos/tour`'s 106 tests: 0 faces refuse DevelopedMerge on the base and 0 take the new path on the head, outside the rows this unit adds. (Tour: counted on the head only, with a counter of faces taking the path AND of refusals; a base pass got 22/106 tests and 2461 patches with 0 refusals in 70 min in a debug container before I stopped it.)
 - Not in the spec: at ε 1e-12 the 0.01° bodies do not revolve (ChartResidual in the band) and the 0.1° bodies are not finished bodies (VolumeUncomputable), so the volume oracle cannot read them.
 - Head: default 1296/1296 → Meshed through the whole oracle; 1e-12 1479/1479 (963 whole oracle, 516 on the 0.1° bodies); no cell goes from meshing to anything else.
 - The quoted analysis, corrected (each measured): (a) "a, P, b collinear" is false for the commonest cell (an end split merges with the polygon's corner), so the anchor is the neighbour ALONG the ruling, not always `a`; (b) "one column" fails: a ruling's far end is read through its rim's image (u = 0.0 against −1.2246e-16), 153 of the 1296 cells kept refusing until the run and anchor were read from the sides' carriers; (c) the run's vertex is a pinch to `planar::Pinches` and would refuse PinchWedge; (d) a thin triangle has no inside for the shared-side cure; (e) two runs anchored by the apex in one triangle needed a rule (they are cut along both). Honest counter-evidence on (a): with the spec's literal "always a" rule as a mutant, NO default-ε integration cell reds; only the unit rows and the ε 1e-12 apex wedge do.
 - One decision I did not take: a run whose ruling-side triangle is a needle (apex cone, ε 1e-12, gap 1e-10, δ 0.001) makes added triangles under the winding allowance → typed UnresolvedWinding (11 of 12 probed cells; were DevelopedMerge on the base). The better rule (cut the fan along its squarest edge) reads developed coordinates rather than polygon order, which is a design choice; pinned and filed instead.

2. WHAT CHANGED
 - crates/mesh/src/trimmed.rs: `Run`, `Anchor`, `developed_runs` (replaces `developed_merge`), `run_of`, `carried_by_a_line`, `CdtPlane::of`, `TrimPolygon::prev`, `Corner`; `insert_points` (one walk entry per run); `emit` (per-CDT-triangle list of output triangles, the re-split); module docs; unit rows `a_run_is_read_from_the_polygons_order`, `a_split_seams_close_points_are_a_run_on_each_ray`.
 - crates/mesh/src/planar.rs: `Walk::skip`. crates/mesh/src/types.rs: DevelopedMerge's doc and message.
 - crates/mesh/tests/split_rulings.rs: 7049's two pins → meshing rows; five new rows. .config/nextest.toml: the grid row in the slow set.
 - work/tesslane: the merge row closed; three rows filed.
 - Commits: 5826273327 (the cure), 74f6252a0c (carriers + unit rows), f2eefef8fd (two runs at the apex), 170f2b4b27 (rows, slow set), 1f452d92c8 (docs, tracker), 901787a06c (clippy), 9bb51c669e (the in-crate row by ε); merges of cone-axis-winding and main between.

4. RUN LOCALLY (all green unless said)
 - fmt --all --check; scripts/run-gates.sh; doc-gate --workspace-pass; clippy -D warnings: mesh + geom-brep --all-features --all-targets, viewer --features app --all-targets, demos/tour --all-targets; work.py lint; slow-set names for mesh.
 - mesh whole, default profile incl. slow set: 516/516 on the final merged tree; split_rulings + development_tests at ε 1e-6 and 1e-12: 16/16 each.
 - On 9bb51c669e, before the last main merge (which touched chords.rs/sizing.rs/memo.rs but none of these crates' sources otherwise): mesh --features budget 517/517; stl 18/18; tools k-lint 49, tess-lint 93, tess-meter 36; viewer --ignore-default-filter display_budget + review_pick_r2 13/13; demos/tour 106/106 (run with `--release` plus debug assertions on, and with my inert census counter compiled in — not the bare head); sweep's meshing modules 584/584 and step-import 329/329 (at 170f2b4b27 plus the digest hook; later commits are docs, text, one clippy rewrite and a test).
 - LEFT TO CI: everything on the final head after the last main merge except mesh; the ε rows of anything but the changed rows; sweep whole; the nightly.
 - Reds: none seen, including none of the listed possibly-main's.

6. WHAT SLOWED ME
 - demos/tour's suite is hours in a debug container (22/106 in 70 min); I ran it once, in release codegen.
 - `scripts/check-slow-set-names.py` with no stdin blocks silently (it waits for a listing); it sat until the harness backgrounded it.
 - `sweep`'s meshing modules are not byte-stable run to run (296 Plane/Cylinder patches differ between two runs of one tree), which costs an extra 10-minute run to tell from a regression.
 - cargo's incremental cache reached 11 G (4.5 G free) mid pre-push; cleared, CARGO_INCREMENTAL=0 after.
 - Blinding note: no trailer or session line is in any commit body, but the container's git identity authors every commit as "Claude <noreply@anthropic.com>" (author and committer fields). That is in the pushed history; I have not rewritten it.
