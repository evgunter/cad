---
id: opening-a-mouth-with-its-whole-belly-refuses-at-the-lift-as-a-together-axial-edge
kind: issue
title: Opening the vessel's mouth together with its whole belly refuses at the rim lift as TogetherAxialEdge, untested and undiagnosed
status: open
opened: 2026-11-22
priority: P2
cost: M
refs: [shell-opens-a-wrapped-face-only-as-two-branches-meeting-at-a-pole]
---


Found by the LATHE shell-one-seam-pole lane (2026-11-22), measured on
`main` and on `lathe/shell-one-seam-pole`; not diagnosed.

## What

`crates/editor-core/tests/lib_g17_shell_node.rs`'s
`the_refusals_are_typed_and_their_texts_pinned`, case (d), designates
the vessel's mouth and its belly (`vessel::SEG_BELLY`, a sphere zone
between the foot and the neck circle). It pinned
`ShellError::OpenFaceChartPartial` because a full revolve built the
belly as two π-bands and the piece named one. With the one-band full
revolve (LATHE, PR 7414) the piece names the whole chart, so that gate
no longer fires, and the shell refuses at the rim stage's lift:

`Lift { face: FaceKey(3v1), error: TogetherAxialEdge { edge: EdgeKey(1v3),
what: "an edge whose moved ends read as one point of its curve, or in
reverse, so it has no forward span left" } }`

The same designation on `main`, naming BOTH belly bands (`band` and
`band_pi`, the whole chart under two-band), refuses the same way
(`EdgeKey(11v3)`), so it is not the one-band build's: the half-chart gate
masked it. The mouth alone opens (`the_vessel_opens_its_mouth_into_one_rim`).

## Open

Whether opening two adjacent designated charts (here plane and sphere,
meeting along the neck circle) is a shape the rim stage should build,
in which case this is a defect in the lift
(`crates/topo/src/offset_axial.rs`, the `TogetherAxialEdge` arms, reached
through `shell::lift_to`), or one it should refuse typed by name before
the lift. Case (d) also needs a new half-chart fixture: a revolved
sphere chart is one face now, so a partial chart needs an authored cut
(two profile segments on one circle, or a meridian split).
