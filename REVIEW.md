IN PROGRESS — dual review r1 of PR 4038 at frozen head e2e114be1431737cc3f761cf0be0f098e483e095

Interim (pushed against container loss):
- r1b battery (`crates/sweep/examples/r1b_pinch_probes.rs`): 28 512 planar lines (rotated L's, Lmid edge-interior, notch327, U inner corner), base 8793177b → head: 1 110 refusal→SOUND, 0 refusal→BAD, 0 SOUND→refusal, 51 → PinchUncrossed.
- cylinder (curved pierced face) 684 lines: 24 ∩ refusal→BAD, all operand=false only (Containment(VolumeUncertified), filed `at-infinity-probe-measures-in-closed-form-only`); base ships 294 same-class.
- PR mutants reproduce 5/3/1/2/1; own: anysurf red(4); single, noB, lastsite green.
