---
id: split-gates-its-operand-on-null-edges-not-on-tier-2
kind: issue
title: The split's operand gate checks null edges by hand, not tier 2, so a strut-bearing operand is not refused at its gate
status: dispatched
opened: 2026-10-02
priority: P1
cost: E
rides_with: split-answers-an-inside-out-operand-with-two-inside-out-halves
branch: cleave/split-operand-gate
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

## Built (branch cleave/split-operand-gate)

Folded into the finished-body door
(`split-answers-an-inside-out-operand-with-two-inside-out-halves`):
the operand gate is tier 2's own verdict, then check 7 per solid, where
no verdict rides the operand; a finished operand already holds both.
`SplitReduceError::ScaffoldingOperand` carries tier 2's findings
(`errors: Vec<ValidationError>`), and its text names no key and states
the recourse, as the Boolean's does. The hand-written null-edge arms in
`classify::gate_operand` and `classify::insert_crossings` are invariants
(`unreachable!`): every door that reaches them took a finished operand.
`classify_neighborhood`, a public read over any body, keeps a typed
refusal for a null edge at its vertex, now carrying tier 2's
`NullEdgeAtRest` finding.

Measured on main `575b309d`: the slit dome (`pole_slit_window.rs`'s
`slits`) through `split` refuses `CurvedBooleanUnsupported` (its sphere
face) for a plane through the dome, at `f64`, `Interval` and `Dual64`;
for a plane clear of it (y = 2) the whole slit lands on one side and
refuses at the result gate, `ResultInvalid` with the strut tip, worded as
a kernel defect ("report it"). On the branch: the slit does not finish at
`f64` or `Interval`, and at `Dual64` every plane refuses
`ScaffoldingOperand` at the door with tier 2's verdict
(`pole_slit_window::a_slit_operand_refuses_at_the_split_door_at_a_dual`).
A reduced body handed back in refuses at every door with its null edges
(`split_operand_gate::a_reduced_body_refuses_at_every_door_with_its_null_edges_at_a_dual`).

Sweep of the blind spot (doors that read an input body with a
hand-written check or none), measured on main with the same two
fixtures: the blend doors answer the inside-out wedge with an inside-out
blended body (filed: `work/band/blend-doors-answer-an-inside-out-operand-with-an-inside-out-body.md`);
`shell` answers it with a valid-looking body of the wrong volume and
`replace_face_offset` answers it inside-out (evidence added to
`work/shell/shell-answers-for-the-complement-of-an-inside-out-operand.md`);
the STEP writer refuses a null edge by hand and reads nothing else
(filed: `work/exch/exchange-writers-take-a-body-no-at-rest-gate-read.md`).
