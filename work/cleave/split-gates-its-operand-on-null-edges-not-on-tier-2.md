---
id: split-gates-its-operand-on-null-edges-not-on-tier-2
kind: issue
title: The split's operand gate checks null edges by hand, not tier 2, so a strut-bearing operand is not refused at its gate
status: open
opened: 2026-10-02
---


Found by the sweep of `a-strut-bearing-operand-passes-the-boolean-gates-and-refuses-at-the-join`,
whose fix gates the Boolean's operands on `validate_closed` (tier 2's
own verdict) in `reduce::gate_operand_pairs`.

The split carries the same hand-written half of that gate:
`crates/topo/src/splitting/classify.rs`'s operand edge pass refuses an
uncertified (null-scaffold) edge as `SplitReduceError::ScaffoldingOperand
{ edge }`, and nothing on the split's way in reads the rest of tier 2 —
a valence-1 strut vertex, an empty loop, a disconnected shell. A
strut-bearing operand (the slit dome of
`crates/sweep/tests/pole_slit_window.rs`'s `slits`) therefore enters
the split pipeline; what it refuses with, or whether it answers, is
**unmeasured**. The repair is the Boolean's: gate on `validate_closed`
and carry its findings in `ScaffoldingOperand`, so the null-edge arm
becomes an invariant.

Blind spot of the sweep that found this: it matched doors with a
hand-written scaffolding check. A door with no check at all cannot be
grepped for; of the public doors that read input bodies, only
`Body::merge_coplanar_faces` (`InputNotClosed`) and now the Boolean
gate their input on tier 2. The blend and offset doors' input posture
toward scaffolding is unmeasured.
