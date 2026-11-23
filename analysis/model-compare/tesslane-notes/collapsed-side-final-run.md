tesslane/collapsed-side — FINAL EVIDENCE RUN on the closing head 2213678c3c8688c0b1b00a291bfd5c45c87d93a9: no red. I checked it out detached, committed nothing and pushed nothing. It ran detached and was polled to completion.

1. `cargo nextest run -p mesh` (default profile, slow set included), default ε: 556 tests run, 556 passed (9 slow), 4 skipped.
   - The collapsed rows plus `d9_mesh_goldens` (`--ignore-default-filter -E 'test(collapsed_rows) | test(d9_mesh_goldens)'`): at ε 1e-6, 11 run, 11 passed; at ε 1e-12, 11 run, 11 passed.
2. `sweep`, default ε, under `--profile ci` (the per-PR default suite minus the slow set), restricted to `binary_id(sweep::all) & test(/^(M)::/)`. M is the 127 suites in `crates/sweep/tests` whose source matches `mesh::|loft_body|loft_profiles|sweep::loft`; `pinned_corners_build` and `pinned_corners_at_the_loft_door` are among them. Result: 734 tests run, 734 passed, 3147 skipped (outside the filter). This filter is mine, not the review's; stage 2 ran no sweep subset.
3. `bash scripts/run-gates.sh`: rc 0.

No red, so there was nothing to compare against origin/main. Signing off on this unit.
