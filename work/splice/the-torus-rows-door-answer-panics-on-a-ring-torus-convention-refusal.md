---
id: the-torus-rows-door-answer-panics-on-a-ring-torus-convention-refusal
kind: issue
title: ellipse_torus's torus fuzz row panics at ε 1e-6 where the door refuses a torus whose ring convention is undecided in the band: the row's door_answer takes that typed refusal as outside the decision
status: open
opened: 2026-11-15
priority: P3
cost: E
---


Filed by GREEN's `green/nightly-1115` lane, from nightly 43680833019's
ε 1e-6 row.

`geom-brep curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance`
panics in `curve_face::circle_roots::door_answer` ("the door refused
outside its decision") on
`CAD_TOLERANCE_EPS=1e-6 CAD_FUZZ_SEED=0x296907b8296cc745 CAD_FUZZ_EFFORT=1`.
The payload:

`Torus(TorusRefusal { convention: Ring, verdict: Undecided(Indeterminate { margin: 7.427460446460076e-6, band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 }, predicate: Some("ring_torus_convention") }) })`

So the search drew a torus whose ring margin (major minus minor radius)
sits inside the escalation band at ε 1e-6, the door refused it typed,
and the row's helper accepts only `DoorRefusal::Escalated`. No certified
answer is involved: this is a refusal the row has no arm for, not a
wrong root. Reproduced on main `3e7a8c93b` at `opt-level = 2`.

**Owed.** Decide whether the row's poses should stay clear of the
convention band at the run's ε or whether `door_answer` should count a
convention refusal with the escalations, and say which in the row's doc.

Reproduce:
`CAD_TOLERANCE_EPS=1e-6 CAD_FUZZ_SEED=0x296907b8296cc745 CAD_FUZZ_EFFORT=1 cargo nextest run -p geom-brep -E 'test(certified_torus_answers_hold_against_the_true_distance)'`.
