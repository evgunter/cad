---
id: a-profile-on-a-face-holds-that-faces-outline-or-a-hole
kind: unit
title: A profile drawn on a face holds that face's outline, or the hole through a named edge, lowered at each evaluation and its pieces named by source edge (FACELOOP U2)
status: parked
opened: 2026-11-22
priority: P1
cost: H
parent: a-faces-loop-or-closed-trace-cannot-become-a-profile-loop
blocked_on: [a-faces-loop-reads-each-joint-off-the-body]
---


The build of the ruling itself (`work/faceloop/plan.md`), on U1's joint
read. One unit, because the loop form, its lowering and the names of
the walls it sweeps cannot land apart: a sweep of a profile holding the
form reaches `eval/anchor.rs`'s piece walk, which today asserts one
minted id per authored step and one kernel step per piece
(`unreachable!` there), and a positional name landed first would be a
persisted name that has to break later (N1).

## The ruled text

`docs/DESIGN.md` `[variables.kinds]` (PR 7284): "A profile drawn on a
face's chart may also hold that face's boundary loops as loops of its
own: the outline, or the hole through a named edge. Such a loop stores
names and no number, and is the face's boundary as it is at each
evaluation, as a shell's open face is." The agreed state is the parent
row's "Weighed (2026-11-21)" section.

## Done when

**The form.** `LoopProgram` (`crates/editor-core/src/program.rs`) gains
`Boundary(Outer | Through(edge))`. It is a loop of a profile whose
frame is a `PoseDef::FaceChart` (`crates/editor-core/src/pose.rs`); the
face is that chart's `face`, so the form stores no face of its own.
`Through` stores one `Edge` read. `authored_steps` is 1, so
`ProfileProgram::ids` mints one `StepId` for it. It persists, loads,
and replays bit-identically (D9).

**The lowering.** At each evaluation, and at each lane scalar
(`f64`, `Interval`, and the `Sym` replay), `LoopProgram::resolve_with`
walks the loop on the face's body as that evaluation built it and
lowers it to kernel steps the lattice already has, read through the
chart's map into sketch coordinates (`eval/wire/trace.rs`'s
`body_to_sketch`):

- one closed circular edge: `Step::Circle`, centre and radius off the
  edge's carrier (`topo::readback::edge_pose`);
- a straight edge after a corner: `line_to` its end vertex
  (`topo::readback::vertex_point`);
- a circular edge after a corner: `arc_to(Center { c, winding, p })`,
  `c` the carrier's centre, `winding` from the edge's sense against the
  face's outward normal;
- at a joint U1 reads **tangent**: `.tangent()` then the continuation
  the lattice admits after it (today `continue_to(p)` and
  `tangent_arc_to(p)`; after PR 7251's build, `line_to(p)` and
  `arc_to(End { p })`), the close at `Start` likewise.

**Tests a bug could break:**

- Ev's elbow by the API: a pipe (radii `R`, `r` variables, extruded),
  a profile drawn on its end cap holding `Outer` and `Through(bore
  rim)`, revolved a quarter turn about `(3, 0)` heading −π/2: builds.
  Change `r` and re-evaluate: the elbow's bore follows. Heading +π/2
  refuses today, and that refusal is LATHE's row, not this unit's
  (below); assert the building heading only.
- The same profile extruded: its walls lie on the pipe's own carriers.
- A box top's `Outer`, extruded: four walls. Fillet the box's vertical
  edges upstream: eight pieces, the joints tangent where U1 reads them
  tangent, and no wall renamed that was not touched.
- **Names.** Each lowered piece is named by the loop's `StepId` and
  the stable name of the edge it came from, never by position: a new
  `ProfileEdgeRef` variant and its `ProfileVertexRef` twin
  (`crates/editor-core/src/names/role.rs`), on the wire and in words
  (`names/words.rs`). A test: fillet one corner of the box upstream,
  and every wall name off the other three corners still resolves to
  the same wall.
- **Refusals, each typed, each naming the loop and its step:** a frame
  that is not a face's chart (`NotOnAFace`); the face's name lost or
  ambiguous (the selection's own N5 refusal); the hole's edge lost, or
  not on the face, or on its outer loop (the outline is `Outer`, one
  spelling per act, `[one-way]`); an edge whose carrier the profile
  vocabulary lacks (an ellipse, a NURBS curve, a spiric), naming the
  ovoid row's profile ellipse arc (`an-ovoid-cannot-be-authored`) as the
  recourse; U1's refusals (valence, in-band joint, slit), passed up;
  and a resolution outside an evaluation, as the trace forms refuse
  (`crate::trace::TraceStepFault::Unevaluated`).
- `tests/switch_program_vocabulary.rs` and `LoopProgram::ALL_NAMES`
  carry the new form.

## Where, by name, and how big

- `crates/editor-core/src/program.rs`: `LoopProgram`,
  `authored_steps`, `resolve_with`, `loop_shape`, `shape`, `drawn`,
  `partition`, `structure`, `pieces`; 11 `LoopProgram::Rectangle`
  match sites, so about as many arms.
- `crates/editor-core/src/step_handle.rs` (1 site),
  `crates/viewer/src/pane/profile.rs` (1) and `crates/viewer/src/sketch.rs`
  (3): exhaustive matches that break at compile. In the viewer the
  arms show the loop and offer no handle on its pieces; the click is
  U3.
- `crates/editor-core/src/eval/wire/trace.rs`: `drawn_in` reads the
  frame's face only as a plane today (`eval/wire/pose.rs`'s
  `on_frame`, through `read_off(.., K::Plane, ..)`); the lowering needs
  the face's body and key at the lane (`select_one`, `read_named_body`,
  as the traced faces are read there), carried in `Drawn` and in
  `crate::trace::Traces`.
- Names: `ProfileEdgeRef` / `ProfileVertexRef` sites in
  `names/role.rs` (3), `names/words.rs` (2), `names/nest.rs` (1),
  `refactor.rs` (3), `eval/anchor.rs` (1), `eval/mod.rs` (1),
  `coincide/stated.rs`, `resolve/mod.rs`, and
  `crates/pncad-py/src/py/step.rs` (1, PYDOOR's ground, an arm only).
  `eval/anchor.rs`'s walk maps each replayed piece to its authored step
  by index (`steps.get(p.step)`); a `Boundary` step draws many pieces,
  so the lowering must hand that walk the source edge of each.
- Persist: `crates/editor-core/src/persist/wire.rs` and `check.rs`.
- `crates/editor-core/src/names/README.md` N1 gains the locator; the
  `DrawnOn` and `ProfileProgram` docs in `program.rs` say a profile on a
  face's chart may hold its loops. Both describe code this unit lands.

## First checks, before building

1. **A rim's name through a keyway.** The ruled example is that a
   keyway cut later is in the loop. For `Outer` that holds by reading.
   For a hole named `Through(rim)`, a keyway through the bore splits the
   rim edge: does the `Edge` read then resolve to one successor, or
   refuse N5 `Ambiguous`? Build it first. If it refuses, the ruled
   sentence "a lost face name or hole edge refuses typed" still holds,
   but Ev's worked example (the elbow keyed through its bore) does not:
   **stop and report**; it is a question for Ev, not a naming change to
   improvise.
2. **The lane pass reads the f64 pass's loop.** The structure is chosen
   at `f64` and the lane replays guided (`eval/wire/trace.rs` module
   docs). Check that the lane's body walks to the same edges in the same
   order (by stable name, not by `EdgeKey`), and that the `Sym` replay's
   vertex points and carrier centres reach the lowering without a
   `from_f64` site. If the lane's walk can differ from the witness's,
   stop and report.
3. **The static doors.** `ProfileProgram::shape`, `partition`,
   `structure` and `pieces` answer without an evaluation, and a
   `SetProgram` that keeps named steps calls `partition` (`edit.rs`,
   the `Reshaped` door). A `Boundary` loop's pieces are not known
   without the body. How those doors answer for it is listed as an open
   question in `work/faceloop/plan.md`; settle it with the orchestrator
   before writing them, do not pick one.

## Left for later units

- The click: `a-click-on-an-edge-of-the-sketched-face-writes-its-loop`
  (VSEAM, U3).
- Python: `a-faces-loop-has-no-python-spelling` (PYDOOR).
- Part of an outline, or a section where no edge exists:
  `part-of-an-outline-or-a-section-where-no-edge-exists-has-no-profile-form`
  (PROFILE, `design`).
- The revolve's refusal for one heading:
  `work/lathe/a-revolve-refuses-a-profile-on-one-side-of-its-axis-as-across-it.md`.
  The union of pipe and elbow:
  `work/heldrecords/a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md`.

## Ground crossed, and who is told

`python3 scripts/work.py territory --files -` on the list above:
`program.rs`, `persist/check.rs` (DOCTAIL, SELFREF); `edit.rs`,
`persist/wire.rs`, `resolve/mod.rs` (DOCTAIL); `eval/mod.rs`,
`eval/anchor.rs` (BORDERS, TALLY); `eval/wire/trace.rs` (TALLY);
`names/role.rs` (DOCTAIL, REFS); `names/words.rs` (REFS);
`names/README.md` (BORDERS, REFS); `coincide/stated.rs` (COINCIDE);
`viewer/src/*` (CHROME, VSEAM); `pncad-py` (PYDOOR). Coupled, so owed
an announcement while live: **SELFREF**, which adds document reads to
the same resolve hop (`resolve_with`, `Traces`); **PROFILE**, whose
PR 7251 build deletes `Step::ContinueTo` and `Step::TangentArcTo`,
which this lowering writes (whichever lands second re-spells the other's
sites; the lowering adds about four); **REFS** and **DOCTAIL** for the
locator variant; the away channel for the persist schema.

## Rows it closes or changes

With U3, it closes the parent row. It makes the elbow a body;
LATHE's and HELDRECORDS's rows above stay open and are what still
refuses.

## Review

**Dual**, difficulty **H**: a persisted form and a persisted name
variant (names are forever), and a lowering read at three lanes whose
joints must never be re-decided.
