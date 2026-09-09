---
id: no-door-mints-mate-frame-from-face
kind: issue
title: No door mints a mate's alignment frame from a selected face
status: open
opened: 2026-08-23
github: 944
refs: [938]
needs_ev: true
---

## From GitHub issue 944

Opened 2026-08-23; 0 comments.

Found by the ASM-DEMO exit walk (#938).

## The shape of it

A11 makes the constructive solve structural on purpose — "no geometry inspection, no numerics beyond decided predicates" — so `Alignment`'s two `MateFrame`s are numbers the author writes. That is the right design for the SOLVE. The consequence at the authoring door is that a mate's frame and the geometry it is meant to sit on are two independent sources of truth, with nothing tying them together.

Concretely, in `demos/tour/src/assembly.rs` the stand's mates carry the post's cap height as a literal (`POST_SEAT`, built from `POST_HEIGHT`) and the shelf's seating points as literals. Edit the post document so its top cap moves, and the mate keeps pointing where the face used to be. Nothing warns; the assembly simply stops fitting, and the only signal is the at-rest gate refuting the declaration afterwards — which the demo's update walk provokes on purpose and reports.

## What is missing

`face_frame(&ev, node, &name)` already answers a named face with a `Pose`. Nothing consumes a `Pose` into an `Alignment`/`MateFrame`. So the natural authoring gesture — "mate THIS face to THAT face" — has no spelling; the author reads the pose (or knows the model's numbers) and retypes them.

## Mitigation a user learns, and why it is not a fix

Model each part from the datum it mates on, so the mated face sits at the part origin and a size change never moves it. The demo says this at `stops`' doc comment. It only helps for the parts you control and the one face you chose.

## Note on scope

Deriving a frame from a face at AUTHORING time does not weaken A11: the solve still reads authored data. What would need deciding is whether the derived numbers are frozen at authoring (a materialized selection, matching the `select` doors' own materialize-then-store rule) or re-derived — the second reintroduces geometry into the solve and is presumably not wanted.

— Claude (ASM-DEMO lane)

## Home

S-MATE's `keep_out` names this issue by number as its own, to be taken with LIB's hand-off; `crates/editor-core/src/mate.rs` is in its territory.

## Question for Ev (2026-09-09, LIB orchestrator; `[ev]` PR)

The authoring gesture "mate THIS face" has no spelling: `face_frame`
answers a named face with a `Pose` (origin, axis, an optional `u_ref`;
`crates/topo/src/readback.rs:88`), `MateFrame` is three authored
vectors (origin, axis, reference; `crates/editor-core/src/mate.rs:115`),
and nothing maps one into the other. Two decisions, then a shape.

**1. Frozen or re-derived?** A11 makes the solve structural — it reads
authored numbers and inspects no geometry. A door that derives a frame
from a face can either MATERIALIZE the pose into numbers at authoring
time (the `select` doors' own materialize-then-store rule: the mate
stores three vectors exactly as if the author had typed them, and the
solve is untouched), or record the face and re-derive the frame at
solve time (geometry re-enters the solve, which A11 forbids and the
item already presumes is not wanted). Recommendation: **frozen**. It
removes the retyping, not the drift: a face that moves after an edit
still leaves the stored frame behind, and the at-rest gate remains the
signal, as today.

**2. The reference axis when the face gives none.** `placement`
refuses a `reference` parallel to `axis`, and a `Pose` carries
`u_ref` only for faces with a canonical in-plane direction. So the
door needs a rule for the rest: **(i)** take an explicit `reference`
argument that is used when `u_ref` is absent and refused with a typed
error when both are missing — recommended; **(ii)** invent one (any
perpendicular), which makes the clocking arbitrary and silent.

**3. The shape**, given 1 and 2(i): **(A)** one kernel door,
`MateFrame::from_pose(pose, reference: Option<[f64; 3]>)` in
`editor_core::mate`, plus the composition at the façade and in Python
(`MateFrame.from_face(evaluation, node, name, reference=None)`),
refusing as a typed `MateFrameError` when neither source gives a
reference — recommended, mechanical once ruled; **(B)** the same door
that also records the face's `StableName` on the `MateFrame` as
provenance, so a later check can compare the frozen frame against the
face's current pose and report drift — a new field on a persisted
struct and a new advisory check, which is a second unit, not a
refinement of the first; **(C)** leave it: the mitigation (model each
part from the datum it mates on) stays the documented answer.

**4. Whose.** The Home line names S-MATE, which has exited; its
residue program is MSOLVE (`work/msolve/`, paths `mate.rs` and
`mate/*`, and it says the census-attribution consumer in
`assembly.rs` stays DOCM's). The kernel half of (A) is one function
in `mate.rs`, in MSOLVE's paths; the façade and Python halves are
LIB's. Recommendation: LIB takes (A) as one mechanical unit and
announces the `mate.rs` touch on MSOLVE's tracker, the way lanes
announce seams today — or, if you would rather keep `mate.rs` edits
with MSOLVE, LIB takes only the façade and Python halves once MSOLVE
lands the kernel door.

### (F), added 2026-09-09 after Ev asked whether materializing stores logically duplicate numbers

It does: a frozen frame is three vectors the evaluation can re-derive
from the face, stored as if authored. Measured: the duplicate exists
today, typed by hand (the tour's `POST_SEAT`), so (A) moves its source
to a door and neither adds nor removes it; `face_pose` is EXACT — a
plane/cylinder/cone/sphere/torus frame read straight off the surface's
parameters, no tolerance, and a NURBS face refuses
(`ReadbackError::NoCanonicalFrame`); and `solve_document(doc, tol)`
takes the document alone — no evaluated body enters the solve, which
is what "the frames are authored data" means in practice.

**(F): the alignment stores the face's `StableName`** — as a fillet
stores `selection: Vec<StableName>` — **and the frame is derived at
evaluation.** No duplicate, and the drift the issue was filed about is
fixed rather than reported: edit the post and the mate follows the
face. The cost is the solve's INPUTS, not its numerics: `solve_document`
would take, or run, the mated parts' evaluations, so a solve depends on
every upstream node of each part. That revises A11's "nothing here
reads geometry" (`crates/editor-core/src/mate.rs` module docs) — a
DESIGN.md conversation and MSOLVE's kernel, not a mechanical unit.

Recommendation, conditional: **(F)** as the direction if the duplicate
is the objection and A11 can be revised to "the document plus its
parts' evaluations", with LIB taking the façade and Python halves over
whatever the kernel then stores; **(A)** if A11 stays as ratified. Not
(B): frozen numbers plus the name stores both and buys only a report.

### Can the solve avoid reading geometry forever? Asked 2026-09-09; measured: no

`crates/editor-core/src/mate.rs` carries no `Expr` and no `ParamName`:
`MateFrame` is three `[f64; 3]`, `PlanarRest.offset` is `f64`, the
clocking rider `Option<f64>`. A mate is plain numbers, so a document
parameter change in a part stales every mate on it silently until the
at-rest gate refutes it, and the only cure under "authored numbers
forever" is a person re-deriving frames — the part's modelling
restated in numbers beside it. So the solve must read geometry
eventually, and A11 reads as drawn around the solve ALGORITHM (coset
intersection over decided predicates, no numeric fitting), not around
where the frames come from.

**(F), shaped so A11's algorithm claim stays and only its "inputs are
the document alone" sentence moves:** `MateFrame` gains an arm —
authored vectors as today, or `FromFace { face: StableName,
reference: Option<[f64; 3]> }` — resolved at EVALUATION through the
exact `face_pose` readback (analytic surfaces off their parameters; a
NURBS face refuses typed and keeps taking authored vectors), with
`reference` used when the face has no `u_ref` and refused when
neither does; `solve_document` runs over resolved frames, the solve
itself unchanged. The drift disappears rather than being reported;
nothing is stored twice. MSOLVE's kernel and a DESIGN.md revision of
A11's wording — a design item, not a mechanical unit; LIB's half (the
re-export, `MateFrame.from_face` in Python) follows the arm. (A)
becomes unnecessary: freezing is materializing the arm.

Recommendation: **(F) now**, in that shape.
