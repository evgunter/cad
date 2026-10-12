---
id: chord-join-serves-lines-where-the-whole-turn-reach-refuses
kind: issue
title: the agreement-gate fuzz finds chord_join serving parallel lines where the whole-turn reach refuses (eps 1e-12, seed 0x80e3604cc0517565)
status: open
opened: 2026-10-09
---

Filed by SHELL's `shell/inverted-body-at-rest` lane (PR 4403), whose
CI drew this seed. That diff does not touch `boolean/join.rs` or
`chord_join.rs`.

## Measured

`topo boolean::join::frame_dispatch_tests::the_agreement_gate_never_serves_on_the_span_alone`
(`crates/topo/src/boolean/join.rs`) fails at `CAD_TOLERANCE_EPS=1e-12`.
It reproduces locally on main as of 2026-10-09:

```
CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x80e3604cc0517565 CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p topo -E 'test(the_agreement_gate_never_serves_on_the_span_alone)'
```

```
chord_join: head serves lines where main reads refused
(0.00001323942935269089 rad × 2.9762215196352457 on r = 1,
 sin β = 0.0000000000003360182154077116)
```

The rim patch is short, an arc of 1.3e-5 rad with height ≈ 3r on the
unit wall, and the plane is tilted ≈ 3.4e-13 off the class boundary.
At that point `chord_join::wall_section` serves `ParallelLines`, while
the whole-turn reading (`face_reach_round_from`) refuses. That is the
span-alone service the row exists to rule out.

## Shape of a fix

This is a real counterexample to the row's property, not a flake. It
should be pinned as a deterministic row beside the fix. The fix makes
the head's line reading escalate wherever the whole-turn reach refuses
on a tilt this close to the band.
