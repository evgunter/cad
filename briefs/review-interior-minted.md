# Review: PR 7832 (`heldrecords/interior-minted`), base main — single FULL review

**Freeze first.** Freeze the head with `git rev-parse origin/heldrecords/interior-minted` and name that SHA in your report.

**Probes:** only on `heldrecords/review-interior-minted-<6 hex>`.

**Read first:**
- `CLAUDE.md`
- `docs/prompts/reviewer-style-lane.md`, IN FULL
- the row `work/heldrecords/a-divisions-minted-cells-name-cells-on-the-cells-boundary.md` on the head, including its Outcome
- the review of PR 7825, whose probes are on `heldrecords/review-editor-fold-red-5f04e9`
- `crates/topo/src/boolean/lineage.rs:62-64` (minted means interior)

**Diff:** `git diff origin/main...<head>`.

**What it builds:**
- `Incidence::chords` counts a section cell as interior only when every one of its twins bounds only the divided face's fragments. A twin is a clone cell on the same point keys, across the null faces.
- The `fate_of` face filter is retired in favour of a debug assertion.
- Normalisation has one home, `records::minted_read`, which `fate_of`, `place_found` and `landings` all use.
- New `ci` pin: `a_tall_flush_blocks_division_names_no_edge_on_the_plate_faces_boundary`.

**Claims to falsify:**
1. **The twin rule is exact.**
   - Every truly interior section cell stays minted. Attack an interior section whose twin bounds a fragment of the *same* face across a seam, and a section on a periodic face's seam.
   - Every boundary section is excluded. Attack a boundary section with no twin, for example where the neighbouring face is not cut.
   - Dropping a truly interior cell would lose a structure or removed landing, so check for silent consumes. Re-run PROBE_GAP face/edge/recut and the carry mutants from PR 7825's probes, and compare the counts.
2. **The debug assertion holds** wherever the old filter fired. Run the slow sets with debug assertions on.
3. **No valid body moves.**
   - The lane's evidence: `ci` at 1e-12 over three crates; topo and editor-core at full default; the sweep slow set by the reach method, 160 hits paired.
   - Re-check the 160 paired sweep rows yourself, plus any slow sweep row you can reach in the time. Main's sweep slow rows are 10–20× slow since INSIDE's PR 7801.
   - Built→refused must be empty.
4. **S4, the four kept-pair `?` reachers.** Is each in-lane reading honest? Is the out-of-lane one, the fillet's conic-trim pair, truly unrelated to minted cells?

**Gate:** the mesh-budget job hangs on main (`quadric_pair_sections`, filed on INSIDE). Do not count it against this PR.

**Rules:**
- Own `CARGO_TARGET_DIR` outside the checkout, per crate.
- Foreground rule: never end a turn with background work.

**Report:** ≤150 lines:
- verdict;
- claims held or broken;
- MAJOR / MINOR / NOTE;
- `## Style`, each item sure / likely / unsure;
- probe rows.

Send it by `mcp__claude-code-remote__send_message` to `session_018u3gVKp4QiofNJ9Z1mzgD8`, then stop. No email or personal identifier anywhere; no model names in commits.
