IN PROGRESS

Review r1 of PR #4139 at frozen head 7863f27b (main baseline: 897a2c24 + the
one-line `live::linked` import, i.e. the PR's merge base made to compile).

Interim (not a verdict):
- Claim 6 holds: pierce, pinch, corner-pairs, both reflex and rc_wide (12 shards)
  byte-identical main vs head.
- Claim 1: new probe `crates/sweep/examples/r1_4139_probes.rs` (kernel-free cone
  count). stair3 (three pinches on one face), islnotch (island + notch on one
  face): no wrong body; head fixes main's crossed bodies.
- Found (MAJOR candidate): `dbl` set (operand already holding two vertices on one point, cut by a cube face): 12 lines SOUND on main -> ResultInvalid{LoopRoleInverted} on head. Debug trace: split_cones groups the runs into 2 cones where exact + brute-force link count give 1, and splits the cube pierce vertex.
