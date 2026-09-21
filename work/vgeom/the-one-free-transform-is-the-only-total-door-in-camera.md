---
id: the-one-free-transform-is-the-only-total-door-in-camera
kind: issue
title: camera's header promises typed refusal and the transform it just adopted is total, and the placement prose is the same length it replaced
status: dispatched
opened: 2026-09-06
refs: [2089]
priority: P4
cost: E
branch: vgeom/f32-seam
---



Found by the style review of #2089.

## Every other door in `camera` refuses typed; this one is total

`crates/viewer/src/camera.rs:30-34` states the module's discipline:

> Operations carry plain `f64` and are therefore constructible with
> values that are not navigation moves at all (a NaN drag, a
> zero-scale dolly). Those are **refused typed** by [`apply`], never
> clamped and never silently dropped

and the file keeps it well past `apply`: `Camera::new` (`:397-404`),
`Camera::fitted` (`:461`), `Camera::projection_matrix` (`:638`) and
`Camera::ray_through` (`:717-720`, plus the length guard at `:759`)
all call `finite(…)` before doing arithmetic, and `ray_through`
explicitly refuses rather than "hand back a poisoned ray as if it were
an answer".

`cursor_projection` (`camera.rs:908-922`) checks nothing. A NaN cursor
poisons the returned matrix, `viewport_px = [0.0, 0.0]` returns a
degenerate one, and both come back as an answer. That was consistent
where it lived before — `marks` is total by construction, a mark is a
value — and it is the move that puts it under a header promising the
opposite. The header's new section (`camera.rs:36-43`) does not mention
it.

I am not sure the fix is a `Result`; the caller is `gpu.rs:595` on a
per-hover path and has nothing to do with a refusal. But "the one door
in this module that answers for any input" is worth one sentence
somewhere, and right now the module doc says the reverse.

## The placement prose did not get shorter

The stated benefit of the move is that `marks.rs` stops apologising for
a lodger. Measured, the apology was replaced by an assertion of the
same size:

| | before (`bc44531e1`) | after |
|---|---|---|
| module-header section about where it lives | 13 lines (`marks.rs`, *"is here for want of a home"*) | 10 lines (`camera.rs`, *"# The one free transform"*) |
| paragraph inside its own doc arguing placement | 6 lines | 9 lines |
| **total** | **19** | **19** |

The function's own rustdoc went from 16 lines to 19 for a 12-line body,
and the added sentence is *"It is HERE because its subject is this
module's"* — a second statement of what the new module header already
says eight hundred lines above. A home that is right usually needs less
argument than one that is wrong, not the same amount in a different
tone.

## What I would have done

`Camera::cursor_projection`, or a named `projection` submodule holding
`view_projection`, `project`, `ray_through` and this. A free `pub fn`
that has to explain in nine lines why it is a neighbour is the shape
that moves again.

## Confidence

`likely` on the totality point — the counter-argument (a per-frame
transform on a hover path should not allocate a refusal) is real, and
the finding is that the module doc does not say so. `sure` on the line
counts. `unsure`, deliberately, on the `Camera::cursor_projection`
suggestion; it is taste.

## Closed

Closed by `vgeom/f32-seam`, which took this row with
`the-point3-to-gpu-corner-cast-is-at-three-sites`,
`the-viewport-and-position-lanes-narrow-to-f32-with-no-door` and
`the-one-free-transform-is-the-only-total-door-in-camera` as **one
question**: where the `f64` → `f32` conversion lives in this crate,
and whether it refuses.

**The answer: one home, `crate::narrowing::Narrow`, and it refuses.**
A single trait with a single method, implemented for `f64`, for
`[T; N]` where `T` narrows (which covers a pair, a triple and a
column-major 4x4 matrix in one impl) and for `Point3<f64>`. It
answers `None` when the RESULT is not a finite `f32` — a test on the
narrowed value rather than on the input, because `f32::MAX` is about
`3.40e38` and a finite `f64` is what turns into an infinity. The
module holds the crate's one `as f32`.

No second door was minted. `Camera::view_projection_f32` and
`SceneMesh::build` do not re-decide the conversion; they call it and
say what a refusal means where they stand.

**The module doc no longer says the reverse**, which is the minimum
this row asked for. `camera.rs`'s *# The one free transform* is now
*# The one free transform, and the one door that does not refuse*, and
it states the totality as a decision with its reason rather than
leaving the header's *refused typed by `apply`, never clamped and
never silently dropped* to be read as covering `cursor_projection`.

**The totality survives, and it is now earned rather than left
over.** The counter-argument the row called real — a per-frame
transform on a hover path should not allocate a refusal — is the one
that won, but the reason it is safe changed: every `f32` it receives
is now produced by `Narrow`, which refused anything that is not a
finite `f32`, and a viewport with no area is refused by
`ViewportSize::ndc_of` before a query is built. There is nothing left
for a `Result` to carry. `viewport_px = [0.0, 0.0]` is still finite
and still gives a degenerate matrix, and `aspect()` is still what
stops it; that is stated at the header rather than implied.

**The `Camera::cursor_projection` suggestion was not taken**, and the
row marks it `unsure` and taste. What DID move is the half of it that
was not taste: `camera` now has a door that produces the transform's
own argument (`Camera::view_projection_f32`), so the module is no
longer *"the home of a function it cannot feed from its own doors"*.

**The placement measurement is re-derived, and the row's own figure is
stale.** Measured here by reading the lines, not by shifting a
number — heading through the last body line in each case:

| | before (`bc44531e1`) | row, 2026-09-06 | now (`origin/main`, `5cc1db9d`) |
|---|---|---|---|
| module-header section about where it lives | 13 | 10 | **8** |
| paragraph inside the fn's own doc arguing placement | 6 | 9 | **6** |
| **total** | **19** | **19** | **14** |

So the row's *"the same amount in a different tone"* was true when
written and stopped being true before this unit touched it: the
function's rustdoc is 16 lines again, not 19, and the sentence the row
quotes — *"It is HERE because its subject is this module's"* — is
already gone. This unit adds to the header, deliberately: the section
is longer now because it states a decision (why `f32`, why total) that
was previously an absence. **That is the claim this row's second half
should be read against, and it rests on a measurement with no
mechanical guard.** It can have none cheaply — a line count over a
prose section is not a property any gate here computes — so the reason
is recorded at the claim site rather than only here: the table above
is in the PR body with the command that took it (`sed -n`, read by
eye), and a successor re-measures rather than quoting.
