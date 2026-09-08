---
id: shell-mouth-chart-designated-in-full
kind: issue
title: A full revolve's mouth must be designated as both half-faces: the chart-completion friction, one coat later
status: open
opened: 2026-09-06
---

On a FULL revolve every profile segment is emitted as two faces on
one chart — the `[0, π)` band and the `[π, 2π)` band
(`crates/editor-core/src/names/emit_sweep.rs:204`, `:273`) — so a
vessel's mouth disc is two half-faces on one plane. The kernel's rim
surgery lifts a chart as a whole and refuses a partial designation
(`crates/topo/src/shell.rs:2094`, `ShellError::OpenFaceChartPartial`),
and the document layer completes no chart on the author's behalf (a
kernel rule and the seat's), so `Node::Shell`'s `open` names BOTH
halves — `corpus/vessel.rs` spells it as
`[band(pot, SEG_MOUTH), band_pi(pot, SEG_MOUTH)]`, and
`lib_g17_shell_node.rs::the_refusals_are_typed_and_their_texts_pinned`
(d) pins the refusal when only one is named.

Whether that is a natural spelling, measured: it is NOT — it is the
friction `demos/tour/src/teapot.rs`'s note 2 recorded, one coat later.
A user thinks "open the mouth"; the two names they must supply are an
artefact of the revolve emitter's `Band`/`BandPi` split, not of the
mouth. The Python spelling
(`tests/test_shell.py`) hides nothing: a selector for the mouth's
segment answers both halves, and both are handed over, but the caller
still has to know the mouth is two faces and that the FIRST named one
carries the rim's identity.

Two candidate resolutions, neither this unit's: a designation door
that takes a CHART (kernel/SEAT — `shell_open` grouping by surface
key itself, with the rim's identity then needing a rule other than
"first named"), or the revolve emitter naming the full-period disc as
ONE face (SWEEP — the `Band`/`BandPi` split is the `kemr`-free
ring-free representation, so this is a representation question).
Recorded here so the choice is made once, against both.
