---
id: main-red-the-collinear-walls-probe-expects-the-edit-door-to-refuse-a-tangent-junction
kind: issue
title: Main red under --all-features: the collinear-walls census probe still expects the edit door to refuse a tangent junction that PR 7116 moved to evaluation
status: open
opened: 2026-11-19
priority: P1
cost: E
refs: [7116, 7075]
---


Nightly 97 (run 44464826302, sha `8c12a734277c`) fails one row in `suite under --all-features (default eps)`: `editor-core::all m10_7_r1_census_probe::r1_collinear_walls_are_unauthorable_so_cosurface_stays_a_non_identity`. It panics on the `drive(..).unwrap()` in the test's `Ok(doc)` arm (`crates/editor-core/tests/m10_7_r1_census_probe.rs`) with `DriveRefusal::WitnessDoesNotBuild { cause: "profile loop 0 refused at step 2: this junction is tangent … (turn margin 0 m on a 1 m arm)" }`. The job ran 16356 rows and failed this one. Nightly 96 (run 44243918484, sha `db86602a558e`) had the job green.

**Cause, read from the diff and the tree, not reproduced by a run.** PR 7116 (merge `cb05ad1ff`, building `[recipe.doors]` from PR 7075) changed `check_written_node` in `crates/editor-core/src/edit.rs` from a check at current values to the value-free `ProfileProgram::lattice()`. The test's `split_rectangle` inserts a chain with a collinear junction through `editor_core::apply(InsertNode)` and expected the door to refuse it. The door now admits it, so the same `PathError::JunctionTangent` (`junction_check`, `crates/profile/src/path.rs`) arrives at the drive's witness evaluation instead. The move is deliberate. PR 7116 re-pinned the default-feature sibling (`coincidence_door::a_profile_junction_on_one_carrier_refuses_at_evaluation`) and not this row, whose file is `#![cfg(feature = "probe")]`: the per-PR gate runs tests at default features, and its `--all-features` use is clippy, which compiles the row and does not run it.

**Re-pin.** The row asserts that the document is admitted and that evaluation carries the tangent-junction refusal, so "collinear walls cannot be built" stays pinned. The docs of `split_rectangle` and of the test say the door refuses. The file header is stale twice: it says no test in the file is executed by CI (the nightly's all-features job runs it), and it cites `work/sym/symbolic-tier-census.md` (the file is `work/tier/symbolic-tier-census.md`). TIER's census wording about the chain door refusing may want the same move.

**Reproduce:** `cargo nextest run -p editor-core --features probe --test all -E 'test(=m10_7_r1_census_probe::r1_collinear_walls_are_unauthorable_so_cosurface_stays_a_non_identity)'`.

**The class.** No per-PR job runs probe-gated tests, so a door or refusal that moves can red this nightly job with no PR having seen it. `work/green/tests-red-under-all-features-never-run-by-ci.md` (closed) is the precedent.

The same nightly's other red, `full suite, build once`, was its runner shutting down about 26 minutes in, while compiling `viewer`; it is not this row's.

(ONEROUND orchestrator, from a read-only triage of nightly 97, 2026-11-19)
