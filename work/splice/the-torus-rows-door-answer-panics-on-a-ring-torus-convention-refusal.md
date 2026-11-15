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

**The pose, so the case does not depend on the row's draw order**
(ε 1e-6 band, family `millimetre`, draw 75; dumped by replaying the
seed):

`Ellipse { frame: Frame { origin: (0, 0, 0), axis: (0.5011117488650869, 0.8647888067648851, -0.032052064575313476), u_ref: (0.6153071327271677, -0.3821016026333749, -0.6894893020780511) }, semi_u: 0.0012671770703822389, semi_v: 0.00017107773178168394 }`
against
`Torus { center: (-0.0005635725753506218, 0.0003338208691673246, 0.0007773907680392707), axis: (-0.8206617627591943, 0.31402430454476943, -0.47739188022016643), major_radius: 3.699336782089947e-5, minor_radius: 2.956590737443939e-5, u_ref: (-0.5028274340603643, 0.0, 0.8643868182453212) }`
on `[2.0969731187580827, 2.7684830342291]`, band `Band::new(1e-6, 1e-5)`.
The major radius exceeds the minor by 7.43e-6, which is the margin the
convention reading could not decide. Not pinned as a deterministic row
here: `ellipse_torus_roots` on this pose returning
`DoorRefusal::Torus(..)` is the whole of it.

**Owed.** Decide whether the row's poses should stay clear of the
convention band at the run's ε or whether `door_answer` should count a
convention refusal with the escalations, and say which in the row's doc.

Reproduce:
`CAD_TOLERANCE_EPS=1e-6 CAD_FUZZ_SEED=0x296907b8296cc745 CAD_FUZZ_EFFORT=1 cargo nextest run -p geom-brep -E 'test(certified_torus_answers_hold_against_the_true_distance)'`.
