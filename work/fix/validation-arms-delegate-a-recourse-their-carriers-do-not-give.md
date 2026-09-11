---
id: validation-arms-delegate-a-recourse-their-carriers-do-not-give
kind: issue
title: four ValidationError arms delegate their recourse to a carrier whose own messages give none — Class B of the census recourse sweep
status: open
opened: 2026-09-11
---


(FIX orchestrator) From the fix pass of PR 2354
(`census-flattens-the-typed-chart-region-declines`), which was told to
sweep the class after its own instance of it was found by review.

## The class

A `ValidationError` arm whose `Display` is a thin wrapper — sometimes
literally `"tier 3: {error}"` — over a **carried** error, on the
assumption that the carrier's own message names a repair. When the
carrier does not, the message a user reads states a condition and
stops.

That is the defect PR 2354 introduced and then removed at the census
door: it dropped a blanket recourse tail because "the cause supplies
the recourse", and **nine of thirteen causes did not**. The unit fixed
its own carriers and made the claim enforceable
(`chart_region::tests::every_chart_region_arm_names_a_recourse`, which
found the ninth arm a reading by eye had missed). These four arms make
the same assumption about four other carriers.

| arm | whole message | carrier | literals with a recourse clause |
| --- | --- | --- | --- |
| `Band { error }` | `"tier 3: {error}"` | `BandError`, `crates/geom-core/src/predicate.rs` | **0 of 2** |
| `VolumeUncomputable { source }` | — | `MassPropsError`, `crates/topo/src/props.rs` | **0 of 5** |
| `Pcurve { finding }` | `"tier 3: {finding}"` | `PcurveMintError`, `crates/topo/src/pcurves.rs` | **1 of 9** |
| `ApproxCertification { error }` | — | `OffsetFitError`, `crates/geom-brep/src/offset_fit.rs` | **1 of 8** |

`Band` and `Pcurve` are the sharp ones: the arm contributes four words
and the carrier contributes everything else, so whatever the carrier
fails to say is simply absent.

Six further arms carrying `Indeterminate` are **sound** and are not on
this list — `Indeterminate`'s `Display` ends in `COINCIDENCE_RECOURSE`.

## The blind spot, stated

The counts come from a verb-vocabulary match over string literals. **It
is a signal to read the arm, not a verdict on it** — a carrier can name
a repair in words the match does not know, and a match can fire on a
sentence that merely contains a verb. Every row above wants reading
before it is fixed.

## What this needs

Per carrier, the same choice PR 2354 faced: give the arms their
recourse (which makes the wrapper's assumption true and is what that
unit did), or have the wrapper supply one (which was what it removed,
because a blanket tail is wrong for some causes). The enforcement row
is the part worth copying either way — a claim of the form *every arm
names its recourse* is cheap to make mechanical and was worth a ninth
arm within minutes of being written.

## Fences — four carriers, four owners

`crates/geom-core/src/predicate.rs` and
`crates/geom-brep/src/offset_fit.rs` are **PROPS's**;
`crates/topo/src/pcurves.rs` is **TRIM's**; `crates/topo/src/props.rs`
is Track M's, which was S-CERT's and S-CERT is closed — that ground
wants an owner named before the row is taken. Homed here because it is
one class with one repair shape and no single program owns the four.
