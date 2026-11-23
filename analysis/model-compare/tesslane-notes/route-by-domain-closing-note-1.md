Step 1 note, on 68873a44b5. Nothing pushed.

**Finished on 68873a44b5:**
- gates, lint, doc-gate: all 0.
- clippy workspace (`--all-features -D warnings`) and viewer `--features app`: both 0.
- geom-brep + topo + step-import whole: 7386/7387. The one red is topo `square_search`'s random-seed fuzz row, which is main's (replayed red on main, filed honeroot).
- mesh whole with the slow set at all three ε: 517/517 each, run on c541c7195a, which differs from this head by the slow-set line only.

**NOT finished.** The container has restarted three times (≈18:40Z, ≈22:09Z, ≈22:43Z). Each restart killed the detached follow-on script during mutant M1, and no result came out:
- the mutant script (M1–M11) on 68873a44b5: no result;
- the armed mesh differential on 68873a44b5: not run;
- the viewer slow-set rows and sweep's meshing modules on 68873a44b5: not run. Last results, on 650e2ce14e: viewer 38/38, sweep 578/578;
- the tour: not run on 68873a44b5. Last result, the armed tour on 650e2ce14e: 106/106, 0 differences.

The tree is clean. I am not restarting anything long now. I'll wait for your pushed sha and run step 2's list there; the mutant script and the armed differential will follow on that head if you want them.
