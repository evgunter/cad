---
id: a-profile-cannot-refer-to-its-own-earlier-parts
kind: issue
title: A profile cannot refer to its own earlier parts, so a bore's centre is written twice
status: open
opened: 2026-11-02
priority: P1
cost: X
design: true
points: 30
refs: [the-rectangle-and-circle-templates-reopen-as-paths, a-profile-step-cannot-refer-to-its-own-earlier-points]
---

Filed by the VSEAM orchestrator from Ev's comment on `[ev]` PR 6143
(2026-11-02): "it generally seems useful to let part of a profile refer
to previous parts of the same profile (not just the ones the user has
written but also … things like the centers of arcs)".

## The gap

A profile's program is loops of steps, and each step's arguments are
written values: typed numbers, or expressions over document variables.
Nothing in one loop or step can read anything earlier in the same
profile, with one exception: a leg aimed at `Start`, the loop's own
first point. So:

- **A bore is two `circle` loops, each with its own centre.** That is
  one stated relation stored as two numbers, the shape
  `[recipe.forms]` and `[intent.checked]` name. Moving one centre leaves
  the bore off-centre. The template designers on PR 6143 agreed this is
  a defect (sure).
- **A sharing variable must be named.** An unnamed variable may have
  only one reader (`[variables]`, VR2/VR7/VR9, `Doc::shared_unnamed_vars`),
  so the only way to share a value today is for the person to name a
  document variable.
- **Derived values cannot be read at all.** An arc's centre, a leg's
  end, a fillet's tangent point, a loop's centroid: none of these is a
  stated number anywhere, so nothing can read them.

## What the designers weighed on 6143, as context and not as the option set

- An in-program address, "the centre of loop k", resolved in program
  order, as `Start` is. It holds no variable, so it is no second reader.
- Each loop defining outputs (its centre) as an operation defines output
  variables. Ev asked whether that is "the same way as bodies creating
  variables".
- Facts from that round: `profile::replay` returns one loop per step
  list; PP2 keeps one record per loop; PATHS-DESIGN §1 reads loops at
  LOOP granularity. Profiles already hold any number of loops, and
  containment decides which are holes.

## What needs deciding

What can be referred to (written arguments only, or derived points and
directions too). How a reference is spelled and stored. Whether it is a
variable, an address, or an output. What it may reach, both within the
profile and from outside it. How it interacts with step ids, names
(N1), the slot walk and range probing. And what it refuses.

## 2026-11-02 — in front of Ev (`[ev]` PR, fork-log row 236)

The designer pair converged on the core: a by-address read
`Of { step, role }` of an earlier step's emitted geometry, holding no
slot and lowered at evaluation. A third designer was dispatched after
three crossovers on two sub-questions: direction reads now or later,
and the spelling of a centre at the entry. The diff states the answer
in `docs/PATHS-DESIGN.md` §1b, `[variables.poses]` and VQ5.

## 2026-11-02 — ruled (Ev, `[ev]` PR 6289): B, 2-D pose variables in the profile's coordinates

The design is `docs/PATHS-DESIGN.md` §1b and `[variables.poses]` as
merged. A profile's points and directions are `Point2`/`Direction2`
variables in the profile's own coordinates. Written points are
anonymous `Point2`s (name one to share it). Derived values are
definitions reading an earlier piece by N1 locator (`Centre`, `Vertex`,
`Heading`), in program order. `AlongTrace` folds in as a `Direction2`
read off a traced face. A value lifts to a 3-D pose only where the
profile is drawn on a frame. Ev's reason for B: "the center of the arc
on the face at the end of the extrude" is a 3-D read off the body with
its own z, a different value from the profile's 2-D centre.

**Priced X, 30 points.** It crosses `crates/editor-core` (variables,
kinds, definitions, VR3's step-granular order rule, the slot walk,
persist), `crates/profile`, `pncad-py` and the viewer (the editor's pick
mode; it needs
`the-profile-editor-holds-kernel-steps-not-the-authored-program`
first). Cut it into units on the owning programs before dispatch. The
two mechanics the designers left unchecked are pinned in the build:
that no later step moves a piece's start once drawn, and that a read
ray is stored verbatim.

## 2026-11-02 — cut into six units; a design residue first

Cut by the VSEAM orchestrator from a read-only planning pass over main
(after PRs 6302 and 6306, with 6340's `rectangle` step in view).
Owners are as `work.py territory` gives them. Each unit names its
crossings. Each unit's scope is below, and its row points here. Every
unit is P1.

| unit | program | cost | blocked on |
|---|---|---|---|
| U1 `a-profile-point-argument-holds-one-point2-variable` | doctail | H | 6340; the residue |
| U2 `the-prefix-replay-reports-each-drawn-pieces-start-centre-and-ray` | profile | M | the residue (Q3, Q13) |
| U3 `a-profile-point-reads-an-earlier-pieces-centre-or-vertex` | doctail | H | U1, U2 |
| U4 `a-profile-director-holds-one-direction2-and-reads-a-straight-pieces-heading` | doctail | H (M without the lift) | U3 |
| U5 `python-writes-a-shared-point-and-reads-a-pieces-centre-vertex-and-heading` | lib | M | U3, U4 |
| U6 `the-profile-editor-picks-an-earlier-piece-as-a-points-or-directors-value` | vseam | M | U3, U4 |

Points: 8+3+8+8+3+3 = 33, which is 28 with the lift deferred (Q11).

**U1. Representation only.** Its sole new behaviour is that two steps
may share a named point.
- `VarKind::Point2` (`var.rs:56`), and a separate `Pose2Def::Written { x, y }`,
  rather than arms on the 3-D `PoseDef` (`pose.rs:242`).
- Every `[S; 2]` point field in `program.rs` becomes one read:
  `ProgramTarget::Point` (:180), `At`/`FarEndTo` (:238), the arc `Via` q
  and `Center` c (:336), and the circle, split-circle and rectangle centres
  (:405).
- The authored form is a new `Operand` seat (`operand.rs:21`), minted at
  `edit.rs:1327`.
- The role tables (`program.rs:1316`/`:1380`/`:1411`, `node.rs:1060`)
  move point coordinates to `read_rows` (:2701).
- VR7's reader counts (`doc.rs:1460`) and VR8's `param_source.rs`
  expansion.
- Persist and load checks (`persist/check.rs:609`/`:964`/`:1905`).
- Crossings are mechanical: pncad, pncad-py, viewer `sketch.rs`
  `HeldStep`, and `switch_program_vocabulary.rs`. Keep the authored
  constructors' signatures, with `From<[Formula;2]>`.
- Test: a bore as two circles reading one named point; an anonymous
  point's second reader refused at both doors; the save/load/replay
  identity.

**U2. A public prefix door** beside `pen` (`profile/src/path/program.rs:3006`),
through the one `walk` (:3324).
- Per drawn `(step, PieceRole)` it reports the start, the arc centre and
  the straight ray; an undrawn piece is reported `NotYetEmitted`.
- **Carries the first pinned mechanic**: no later step moves a drawn
  piece's start. Test it in `tests/prefix_emission.rs`, keyed on
  `Verb::ALL`: every prefix of the differential corpus, at f64 and at
  Interval guided.

**U3. Definitions that read a piece**: `Pose2Def::Centre`/`Vertex { piece }`.
- The step-granular VR3 order rule, written into `VARIABLES.md`, without
  a node self-cycle (`edit.rs:6716`, `doc.rs:1377`/`:1619`/`:1783`/`:2150`,
  `persist/check.rs:856`).
- They resolve inside the profile's own resolution, from U2's record
  (`program.rs:1872`/`:1931`). This crosses into trace's
  `trace.rs:255`/`:301`, and into wire/tally's `eval/wire/pose.rs:232`
  with one arm.
- Refusals are typed at both doors. A dropped step leaves its read
  unresolved, never re-pointed.

**U4. `VarKind::Direction2`.**
- `Toward` holds one (`program.rs:243`).
- `Pose2Def::Heading { piece, sense }`.
- `AlongTrace` (`program.rs:307`, lowering at `trace.rs:369`) folds into
  a `Direction2` read off a traced face.
- The optional lift through `DrawnOn.frame`.
- **Carries the second pinned mechanic**: a read ray is stored verbatim,
  bit-equal to U2's ray. `Against` gives the exact negation.

**U5. pncad and pncad-py.**
- `at`/`toward`/`circle`/`circle_split` take a 2-D pose or a tuple.
- `centre`/`vertex`/`heading` on `Piece` (`py/step.rs:116`), and the
  matching stub, tags and census rows.

**U6. The editor's pick mode** (`pane/profile.rs`, `pickindex.rs:2593`).
- A click on a piece of the profile under edit writes a read into the
  field.
- `HeldStep` (`sketch.rs:433`) and `ProfileEdit` (`drafts.rs:360`) hold
  the read: a crossing into how a field holds a read, and no further.
- It claims `a-profile-pick-names-no-profile-piece`.

**The design residue, which blocks every unit.** It goes to a designer
pair before any unit dispatches.
- Q1. A written `Direction2`: VQ4 stores `toward(dx, dy)` as two Scalars
  verbatim, but `[variables.poses]` says a 2-D direction is an angle.
- Q2. Where "a read ray is stored verbatim" applies.
- Q3. The seam-fillet close (`.to(Start)`, `profile/src/path.rs:2882`)
  re-trims the first piece's start after it is drawn. When is a fillet's
  incoming leg "drawn"?
- Q4. Where a written, named `Point2` that reads no piece is scoped. May
  two free profiles share one?
- Q5. Whether a read stores its profile node, and which DAG walks the
  step-granular rule exempts.
- Q6. Whether `SlotId::Profile { arg: PointX }` and the coordinate
  `StepArg`s survive as addresses into the `Point2`.
- Q7. Whether a self-read resolves outside an evaluation:
  `ProfileProgram::check`, `pieces`, the load walk, the preview.
- Q8. "Loop order": authored, or canonical after containment.
- Q9. A kept step id on a different verb, or a `SetProgram` that moves
  the read piece after its reader.
- Q10. A 2-D slide (vertex + offset), the other half of
  `work/forms/a-profile-step-cannot-refer-to-its-own-earlier-points`.
- Q11. Who reads a lift, and whether the revolve's axis migrates now.
- Q12. How the folded `AlongTrace` is addressed, and whether it stays a
  step.
- Q13. A read of a role a `Zero` fit leaves undrawn at these values.
- Q14. Addressing a piece of a program that is on its way in and holds
  no step ids yet, which needs a placeholder lowered at insert.
- Q15. VR8 coincidence tokens for a derived read.

**Owner load.** doctail is `ready` with no orchestrator, at load 111
against a budget of 45. U1, U3 and U4 add 19 points to it.

## 2026-11-02 — the residue, weighed (designer pair, five rounds; fork-log row 241)

The pair agrees on all fifteen. One of them, Q1, changes ratified text,
so it waits for Ev on the `[ev]` PR. The rest are build-level: the
answers are implied by the ruled text, and builders follow them.

- **Q1.** A written `Direction2` holds no components. It is one `Angle`
  from the profile's +x, lowered exactly at a written fraction of a turn
  through `geom_core::Angle`. Today only the revolve's heading reads
  through that; the profile director's `Dir::from_angle` must be wired
  to it. `toward(dx, dy)` leaves the document and stays the kernel's
  ray door. "30° off an edge" is a `Spin { of: read, by }`, admitted
  when a reader asks. Ev's residual preference: whether the profile's
  axes also get a numberless `Axis { x | y, sense }` form.
- **Q2.** Nothing of a read is stored. The pin is that a `Heading`'s
  value is the read piece's ray, unchanged, at every scalar.
  `Against` negates it exactly.
- **Q3.** Only the seam-fillet close moves an emitted start, and it
  moves the entry vertex. A same-loop `Vertex` read of a chain's first
  piece refuses; the recourse is to name the `at` point.
- **Q4.** All of a point's readers share one coordinate system: one
  profile, or profiles drawn on one frame, plus nodes that read those.
  This is checked at the save and load door.
- **Q5 and Q7.**
  - A read stores its locator only; the node is derived from the step
    id.
  - The node DAG drops a profile's edge to itself.
  - Step order is checked through everything a definition reaches.
  - A read resolves wherever its profile resolves (check, pieces,
    preview, load). The one exception is a read of a piece after a
    trace step, which resolves only in an evaluation.
- **Q6.** A point argument is one slot. The coordinate `StepArg`s go,
  and the definition's own `Length`s are edited by id.
- **Q8.** Order is the stored order.
- **Q9.** These refuse at the `SetProgram` door, because §1b's
  refusals are save and load invariants (`validate_snapshot`):
  - a `SetProgram` that reorders a read piece after its reader;
  - one that keeps a step id on a verb lacking the read role.

  Only a dropped step strands its read.
- **Q10.** A 2-D `Slide { point, along: Direction2, by: Length }`, with
  `p.offset(w, 0)` as façade sugar.
- **Q11.** The revolve's axis line migrates to `Point2`/`Direction2` now,
  as its own unit. The lift waits for a reader.
- **Q12.** `TraceTangent { piece, sense }`, a `Direction2` definition;
  the `AlongTrace` step goes.
- **Q13.** A read of a role the values do not draw resolves from the
  construction (§1b "constructed"), so U2 reports each role from the
  trims. A name on that role still vanishes.
- **Q14.** An authored address for a step the same edit adds, lowered to
  its id when minted.
- **Q15.** A read lowers as a leaf `(kind, step id, role)`. `Centre` of a
  verb that holds its centre as a `Point2` expands to that point.

**Cut corrections.**
- U1 changes addressing as well as representation, and carries Q4's
  check.
- U4 folds `Angle` and `Toward` into one `Direction2` director, adds
  `Spin`, `Slide` and `TraceTangent`, and wires the director through
  `geom_core::Angle`. It is cost M without the lift.
- A U7 for the revolve's axis follows U4. It is filed when Ev rules.

**Still unverified by either designer**, for U2's pin to test:
- that nothing but the seam close moves an emitted start;
- that the entry is always `at(p)`.

## 2026-11-02 — Q1 restated after Ev's comments on 6346

Ev's comments on 6346:
- "isn't this going to be inexact in the cases that we had toward for?"
- "the design that's supposed to be here is another case of a family of
  different representations with a shared api, like the arcs".

The kernel already built it that way: one director slot whose payload is
an angle or a ray (§2a item 5). The document holds the two forms as two
steps, `ProgramStep::Angle` and `Toward`.

The designers had read `[variables.poses]`' "a 2-D direction is an
angle" as binding. Under `[recipe.forms]` it is the sentence that
needed amending, and the director does not. A `Direction2` is one
value, its unit ray, written as an angle or as a ray, and each form is
kept as written. This is both designers' round-1 alternative.

So U4 folds `Angle` and `Toward` into one director slot that holds a
`Direction2` written in either form, plus the reads. The axis form and
exact degree literals are not needed for exactness, because the ray
form keeps it.
