---
id: an-ellipse-torus-fuzz-seed-draws-a-torus-whose-ring-convention-is-in-band
kind: issue
title: The ellipse x torus fuzz row panics on a seed that draws a torus whose ring convention is in band: the door's typed refusal is read as outside its decision
status: open
opened: 2026-11-11
priority: P2
cost: E
---


Filed by the SECTARMS plane-partial lane (branch
`sectarms/plane-partial-closed-form`, 2026-11-11). It turned up in that
branch's geom-brep slow set and reproduces on main.

## What

`curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance`
(`crates/geom-brep/src/curve_face/ellipse_torus.rs`) is a counterexample
search with a fresh seed each run. Seed `0x3ea80f26c3a065f6` panics at ε
1e-6, at `curve_face::circle_roots::door_answer`
(`crates/geom-brep/src/curve_face/circle_roots.rs`):

```
the door refused outside its decision: Torus(TorusRefusal { convention: Ring,
  verdict: Undecided(Indeterminate { margin: 8.878612850789409e-6, band (1e-6, 1e-5),
  predicate: "ring_torus_convention" … }) })
```

The pose generator drew a torus whose `R − r` lies in the band. The door
refuses that typed, on the torus's ring convention, before any root is
read. `door_answer` admits only `DoorRefusal::Escalated` as a refusal and
panics on any other.

## Reproduce

On main `9b4f0a7f47`:

```
CAD_FUZZ_SEED=0x3ea80f26c3a065f6 cargo nextest run -p geom-brep --all-features --lib \
  -E 'test(=curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance)'
```

It also failed once in a slow-set run on an unseeded draw, so roughly
one draw in a few dozen hits it.

## What the change could be

Either the pose generator keeps `R − r` past the band at the ε it runs
at, or the row reads a convention refusal in band as the escalation it
is. Which one depends on whether the door's typed refusal is meant to
be distinct from `Escalated`; that is SPLICE's call. Once it is fixed,
pin the seed as a deterministic row beside the fix.
