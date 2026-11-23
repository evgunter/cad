# Reviewer probes, PR 7394 at 539a76e190f

- `crates/geom-core/tests/review_decide_path_r1.rs`: rows p1..p12 (each prints its reading).
- `crates/geom-core/src/predicate.rs`: `review_r1_the_straddling_twin_unproven_and_the_excluding_one_proven` (prints).
- `crates/editor-core/tests/review_decide_path_r1.rs`: the instrument over named corpus documents (`--ignored`).
- `mutants.py`: the PR's nine named mutants and four reviewer mutants over `geom-core`, run in a
  separate worktree and target directory (paths at its top).
- Two env-switched mutants were built into a throwaway tree and are not committed:
  `R1MUT_SIN2` (`algebra::one_minus_cos_squared` returns `-cos²`) and `R1MUT_WALK` = 1 | 2
  (`Sym::decide_sign`'s definite arm asks `discharge_in(node, Path::Algebra)`, first attempt only;
  2 keeps those walks' freezes out of the drive memo).
