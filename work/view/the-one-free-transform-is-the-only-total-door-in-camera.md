---
id: the-one-free-transform-is-the-only-total-door-in-camera
kind: issue
title: camera's header promises typed refusal and the transform it just adopted is total, and the placement prose is the same length it replaced
status: open
opened: 2026-09-06
refs: [2089]
---



Found by the style review of #2089.

## Every other door in `camera` refuses typed; this one is total

`crates/viewer/src/camera.rs:30-34` states the module's discipline:

> Operations carry plain `f64` and are therefore constructible with
> values that are not navigation moves at all (a NaN drag, a
> zero-scale dolly). Those are **refused typed** by [`apply`], never
> clamped and never silently dropped

and the file keeps it well past `apply`: `Camera::new` (`:362-369`),
`Camera::fitted` (`:426`), `Camera::projection_matrix` (`:603`) and
`Camera::ray_through` (`:682-685`, plus the length guard at `:724`)
all call `finite(…)` before doing arithmetic, and `ray_through`
explicitly refuses rather than "hand back a poisoned ray as if it were
an answer".

`cursor_projection` (`camera.rs:882-896`) checks nothing. A NaN cursor
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
