Result for part (d), item 2 (the last one). Nothing is still running, and nothing has been pushed; the head is still 3494faec95.

Tess-budget sweep: release, `--sizing-only`, every tour scene. Main 6396c45978 against the frozen head 3494faec95, the two trees alternated, two runs each, same container:
- main: 345 s and 342 s
- branch: 335 s and 341 s

All four CSVs have the same 1,678 rows, byte for byte (only the `# tess-budget-cut` provenance line differs).

Replacement for the PR body's sweep cost line:
"- Tess-budget sweep (release, every tour scene, `--sizing-only`), measured 2026-11-17 on main 6396c45978 against this head, alternated: main 345 s and 342 s, branch 335 s and 341 s. The meter's 1678 rows are byte-identical."

Still open, waiting for your go: merging origin/main (PR 7043 conflicts in mesh/src/tessellate.rs and .config/nextest.toml).
