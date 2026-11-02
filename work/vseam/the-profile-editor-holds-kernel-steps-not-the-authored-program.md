---
id: the-profile-editor-holds-kernel-steps-not-the-authored-program
kind: issue
title: The profile editor holds kernel Step<f64>, not the authored LoopProgram, so it cannot hold a trace, AtX or reference program
status: open
opened: 2026-11-02
priority: P1
cost: H
refs: [a-profile-cannot-refer-to-its-own-earlier-parts]
---

Found by a designer weighing `a-profile-cannot-refer-to-its-own-earlier-parts`
(2026-11-02); filed by the VSEAM orchestrator.

`viewer::sketch::HeldLoops` holds the program as kernel `profile::Step<f64>`,
the resolved, lowered form, not the authored `LoopProgram<Formula>`. So
`edit_profile_ui` cannot hold a program that reads a face's trace
(`OnTrace`/`AlongTrace`) or the pen (`AtX`/`AtY`), and falls back to
slot rows (`HeldRefusal::Trace`/`Resolve`). It also could not hold an
in-profile reference, which is the shape the designers recommend there.
That is two representations of one program, against `[recipe.forms]`
and VQ5 as restated on PR 6143. PR 6144 already had to graft driven
expressions back onto the held kernel steps (`ProfileEdit::read_drivers`),
which is the symptom.

The editor should hold the authored program, which
`resolve_written_loops` already speaks, and lower it only to draw.
P1: two implementations of one representation, and an editor that
cannot hold every stored profile.
