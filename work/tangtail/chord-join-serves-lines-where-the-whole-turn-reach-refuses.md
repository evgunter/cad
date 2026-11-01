---
id: chord-join-serves-lines-where-the-whole-turn-reach-refuses
kind: issue
title: the agreement-gate fuzz finds chord_join serving parallel lines where the whole-turn reach refuses (eps 1e-12, seed 0x80e3604cc0517565)
status: open
priority: P3
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

## Another seed (2026-10-13)

PR 4962's CI (`band/turn-overrun-fillet`, which touches no file under
`crates/topo`) drew `CAD_FUZZ_SEED=0x42ba32f9ed63baf9` at `1e-12` and
failed the same row with the conic arm: `chord_join: head serves conic
where main reads refused (0.0666318355989892 rad × 1.761587807094113 on
r = 1, sin β = 0.000000000005677081873502136)`. The property is the same
and the reading is a different class, so a fix for the line arm alone
would not close it.

## 2026-10-13: once at the default eps (EMIT, branch `emit/naming-refusals`)

The fuzz failed once in a local `cargo nextest run -p topo --profile ci`
at the default eps, on a branch whose topo diff touches only
`boolean::ops::apply_cut_ins` and its caller. The seed was not kept;
15 reruns passed.

## Another seed (CLEAVE, `cleave/fate-cut-ends-at-a-slit-tip`, 2026-10-21)

At the default ε, a local run on that branch (whose diff does not
reach `chord_join`) drew a conic case:

```
CAD_FUZZ_SEED=0x6fc0592555a860af CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p topo -E 'test(the_agreement_gate_never_serves_on_the_span_alone)'
```

```
chord_join: head serves conic where main reads refused
(0.00004901376726445554 rad × 7620.053854752845 on r = 1000,
 sin β = 0.0000000000013123654088462953)
```

## Another seed (2026-10-27)

ATREST's `atrest/a-crossing-touch-is-interference` CI (PR 5823, head
`0bfb067e`) drew a second seed at the DEFAULT eps. That diff does not
touch `boolean/join.rs` or `chord_join.rs`:

```
chord_join: head serves lines where main reads refused
(0.000012485095070953775 rad × 3873.1882640132785 on r = 1000,
 sin β = 0.00000000000025822144679227736)
CAD_FUZZ_SEED=0x90ac89fbe5ecb537 CAD_FUZZ_EFFORT=1
```
