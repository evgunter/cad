# Review probes, PR 7394 at 539a76e1 (merge base 0a62bae4)

- `head-mutant-gate-R2_MUT.patch`: applied to the head for the corpus-lane mutants
  (`R2_MUT=sub`: a constant non-zero difference's form reads zero; `R2_MUT=retry`: the first
  offered retry attempt answers zero). Both turn `contradiction_corpus` red (338/129/150/434/478
  and 1047/548/891/1508/1673 found on the five documents).
- `base-S1-gate-R2_S1.patch`: applied to the merge base; `R2_S1=1` skips the definite arm's
  `door_zero` and contradiction walks (the PR's change in isolation).
- `geom-core-mutants.py`: the PR's named mutants plus M10 (enclosure check taken at attempt 0 only).
- `crates/editor-core/tests/review_decide_path_r2_drive_probes.rs`: the slab race drives at
  8/4 and 8/3, memo on and off, printing certified counts and frozen.
- `crates/geom-core/tests/review_decide_path_r2_probes.rs`: the enclosure-check probes.
