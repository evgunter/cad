IN PROGRESS

# Review r2 of PR 4050 (frozen head 0e0d871a)

Interim notes (executed so far, probes in crates/sweep/tests/review_sixx_r2_probes.rs, `--test sixx`):
- six-crossing cube grid, 16 shapes incl. mirrored: 39 738 runs PairingMismatch→SOUND, 0 BAD.
- shape-vs-shape n>=8: 4 616 PairingMismatch→ClassificationInvariant (strut held by strut held by fan, depth 2).
- near-tangent (d=±1e-7, ±1e-9 rad): 1 187 PairingMismatch→BAD, all CensusEscalated, exact volume, t2/cert/operand true.
- pinch probe: 85 SharedVertexCrossings at head (nested plan at a shared vertex), reached.
