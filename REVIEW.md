IN PROGRESS

Review r2 of PR 4026 at 93ecd145 (base e4a0a0d = the head's merge parent on main).

Interim (not final):
- probes in crates/sweep/tests/join_pierce_r2_probes.rs (shapes, holed block, ball, row tilts under mutants).
- holed battery 3 359 lines: 0 BAD on either tree; 306 refusal->SOUND, 582 refusal->refusal, 0 SOUND->refusal.
- row tilts under env mutants on head: the mutant table reproduces (fixed facing, ∩ end/walk, one-pierce guard, hole without kfmrh -> BAD).
- weld refusals ("two fragments ... meet one pinch", "copies divide a face") reached by holed ∖ cube, edge-run family, unfiled.
