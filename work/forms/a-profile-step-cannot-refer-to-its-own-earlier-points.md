---
id: a-profile-step-cannot-refer-to-its-own-earlier-points
kind: issue
title: A profile step cannot refer to the profile's own earlier points: no target is 'vertex k', or 'vertex k plus an offset'
status: open
opened: 2026-10-31
priority: P1
design: true
---


Reported by Ev (chat, 2026-10-31), wanting to write a step's point as
"point x − (w, 0)", where x is a point the profile already has, and
willing to name `(w, 0)` first as a vector variable. Banded P1 here
(verb breadth, and an annoyance Ev reported, not a block); `design`
because any fix touches a ratified rule (below).

## What a step can name today

A target-taking verb ends at a `Target`
(`crates/profile/src/path/program.rs:269`): an authored absolute
`Point` (`:271`), or `Start` / `StartArriving`, the loop's entry
vertex — which only closes the loop. The other verbs move from the tip
(`Toward { dx, dy }`, `:921`; `Line`, `Turn`, `Angle`). So a step can
reach the PREVIOUS vertex (by moving from it) and the FIRST one (by
closing onto it), and no other: there is no target "vertex k" and no
"vertex k + (dx, dy)".

## The workaround, and where it stops

Each coordinate of an absolute `Point` is a `Length` slot. In a
committed profile's edit door a slot's variable can be named
(`SessionOp::SetSlotVariable`, offered from the naming field,
`crates/viewer/src/pane/properties.rs:1162`), and a later slot can read
it in a formula: name step 2's x `ax`, write step 5's x as `ax - w`.
That works only:

- coordinate by coordinate: a point is two names and two formulas;
- after the profile is committed (the create form takes numbers:
  `work/forms/the-add-profile-form-takes-numbers-and-no-formula.md`);
- for a vertex written as an absolute point. A vertex the profile
  reached by a relative move (`Toward`, `Line` after an `Angle`) has
  no coordinate slot to name, so nothing downstream can
  read where it is.

## Why this is a design question

DESIGN `[variables.poses]` (`docs/DESIGN.md:1201`) rules that "a 2-D
value (a profile's step, a revolve's axis line) lives in the node that
holds its frame, as length and angle slots, and is never a variable of
a 2-D kind". So Ev's "(w, 0) as its own vector variable" is outside
the ratified kinds. A reference to the profile's own vertex need not
be: it could be a target form inside the profile program (a `Vertex`
form naming an earlier step, with an optional offset of two `Length`
slots), which stays within "lives in the node that holds its frame".
Which of those — a new target form, a 2-D kind, or something else —
is a fork for the designer lanes and Ev (`docs/prompts/designer.md`),
not for an implementer.
