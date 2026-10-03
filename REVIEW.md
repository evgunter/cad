IN PROGRESS

# Review: JOIN-2 fix pass 1 (PR 3880, delta 17c254c99d..4c18c2cbe)

Interim (containers restart):
- Head suite topo + sweep: 4214/4215 at 4c18c2cbe (the one red is mine: the battery file was added mid-run, `every_suite_file_is_aggregated`).
- Battery `crates/sweep/tests/join2_d_probes.rs` (own shape, 1680 lines), main 82b9ceb2b vs head: 0 SOUND->refusal, 0 BAD either tree, 56 refusal->SOUND.
- Mutant M1 (old ring-first order, `>= 1`): two non-ignored rows red.
