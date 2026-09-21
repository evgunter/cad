---
id: chart-coherence-lane-absence-unstated
kind: issue
title: ChartCoherenceLane's None is the f64-only class, and its arms say nothing
status: open
opened: 2026-09-21
priority: P3
cost: E
refs: [H5]
---

## What

`ChartCoherenceLane::examine_chart_coherence`
(`crates/editor-core/src/checks.rs`, the trait around `:244`) answers
`Some` at `f64` (`:250`) and `None` at `Probe` (`:257`) and `Interval`
(`:267`). **Neither refusing arm carries a word of reason** — no doc
comment, no inline note — while every sibling lane in the file's
neighbourhood states its own (`ShellLane`'s probe arm,
`crates/editor-core/src/verbs/shell.rs` around `:187`: *"The recording
scalar is `f64` with a sink attached, so it carries exactly what `f64`
carries"*).

The reason is readable from the tree and it is not a certification
fact: `topo::examine_chart_coherence` is **written at `f64`**
(`crates/topo/src/coherence.rs`, the door around `:672`, signature
`&Body<f64>`). So this is the LANE-0 class — *"the derivation is not
written at this scalar"* — and not *"this scalar may not certify"*
(`docs/DUAL-DESIGN.md` DL1, the missing `CertifiedEnclosure` impl).
The probe arm is the sharp one: `Probe` IS `f64` with a sink, and every
other per-scalar seam in editor-core says so and delegates
(`Lane::NAME`, `SectionScalar::pinned_f64` returning `Some(self.0)`,
`AxisScalar::axis`, `ShellLane::run_shell`). Reading the arms alone, a
reader cannot tell which of the two absences this is, and the probe
arm reads as a demotion nobody intended.

A third shape rides with it: the trait has **three** impls and no
`Sym` or `Dual` arm at all, so those scalars cannot instantiate the
resident's pass (`checks.rs` around `:1211`) — statically excluded
rather than answering `None`, which is a different contract again and
is also unstated.

## Why it is not fixed where it was found

Found by SCALAR's LANE-0 sweep (`docs/LANE-0-SPEC.md` §4, the absence
table). `crates/editor-core/src/checks.rs` is FIX's ground and outside
that unit's fence, which claims no editor-core paths.

## The shape of the fix

Either state the reason on each arm (cheap, and enough), or — if
`H5`'s ruling 3 reaches editor-core's seven lane-shaped traits — give
the absence the same treatment LANE-0 gave the offset fit: one door
value (`geom_brep::OffsetFitLane`'s shape), `Some` from the `f64`
seam, `None` everywhere else, and the `Sym`/`Dual` gap closed by an
arm that says what it means rather than by a missing impl.
