---
id: tangent-escalations-end-with-the-lever-alone
kind: issue
title: chord_join's and split_edge's tangent escalations end through diag.ending, offering a tolerance arm for an angle decision
status: parked
opened: 2026-10-12
priority: P3
cost: E
blocked_on: [4766]
---


## Finding

Raised by ENCL's note in `work/tang/log.md` (2026-10-12). Four TANG refusal
arms end through `Indeterminate::ending`, which carries the valued tolerance
arm:
- `topo::chord_join` `TangentChordEscalated` and `TangentChordBendEscalated`,
  through `diag.ending(JOIN_RECOURSE)`;
- `topo::euler` `SplitTangentEscalated` and `SplitTangentBendEscalated`,
  through `diag.ending(crate::split::TANGENT_PIECE_LEVER)`.

Each margin is a tangency decision: the dihedral angle between the faces, or
their second-order bend, along the edge. It is not a length the user chose,
so a "change the tolerance" recourse names no knob the user holds.

## Ruling (TANG orchestrator)

Under the settled undecided-wording rule (ENCL `too-close-to-call-remainder`;
`recourse.rs`'s Unsized), these four arms end with the lever alone, through
geom-core `Indeterminate::lever_ending`. That ending lands in PR 4766, so this
row is parked on it. When it lands, sweep the shape: every
`MustCarryEscalation` / `DihedralReading` refusal that ends through
`.ending(`. The sweep covers `splitting::finish`'s `Describe*Escalated`
(through `split_lever_ending`) and the sweep and blend sliver arms. Put the
hit list and its dispositions in the PR.

