Result for part (d), item 1. Nothing has been pushed; the head is still 3494faec95.

**Frozen-head re-run.** geom-brep, topo, mesh and step-import together: 7,854 tests, 7,852 green, 2 red.
- step-import `split_iso_side` is green.
- Both reds are fuzz rows run under random seeds, in code this branch does not touch. Replayed with the same seed on main 6396c45978, each fails there too:
  - geom-brep `curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance`, `CAD_FUZZ_SEED=0x4948ab2ef4fca5cd`. A ring-torus convention escalates in band. Already filed: work/splice/an-ellipse-torus-fuzz-seed-draws-a-torus-whose-ring-convention-is-in-band.md and work/splice/the-torus-rows-door-answer-panics-on-a-ring-torus-convention-refusal.md.
  - topo `boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`, `CAD_FUZZ_SEED=0x9aa29ef6ff51a6d8`, 2 mismatches. Already filed under work/honeroot/ (section-cert-square-wall…, the-square-wall-arm-reads-a-tangent…).

**Main has moved, and a merge now conflicts.** origin/main e4b01d0b7f includes PR 7043 (tesslane/lattice-clearance). `git merge-tree` reports content conflicts in crates/mesh/src/tessellate.rs and .config/nextest.toml; the other mesh files auto-merge.
- I have not merged. Merging needs a push after the frozen head, so I am waiting for your go.
- Once you say go, I will merge, resolve, re-run mesh at all three ε plus the gates, and report the new sha.

The tess-budget sweep timing is still to follow.
