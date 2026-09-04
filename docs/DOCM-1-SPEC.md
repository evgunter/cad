# DOCM-1 — the derived sketch frame: `Datum::FaceFrame`, the sense beside the pose, the carrier-kind read (spec)

**Program:** DOCM (`work/docm/plan.md`), unit `DOCM-1`
(`work/docm/DOCM-1.md`). **Ratified design:**
`docs/DOCM-REFERENCES-DESIGN.md` DM1, DM1a, DM1b, DM2 — read them
first; this spec binds the build and does not re-open them.
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass,
record-at-merge; §Review below).
**Pre-draw fields, logged before the draw:** difficulty **L**,
task-class **STRUCTURAL**.

- **L** — a new datum variant reaches every exhaustive `Datum`/`Node`
  match (`node.rs`, `eval/`, `persist/wire.rs`, `persist/check.rs`,
  `refactor.rs`, `resolve/`, the viewer's `session.rs`, `sketch.rs`,
  `tree.rs`, `scene.rs`; the Python mirror is LIB's and forced edits
  are disclosed, not designed), plus a kernel read-back door in `topo`
  and its `StableName` twin, plus the first datum with an N5 failure
  mode.
- **STRUCTURAL** — no new numeric decision: the frame's axes go through
  the existing `frame_axes` door (`eval/wire.rs:791`), the planarity
  gate is a tag comparison (DM2), and the spin is a rotation, not a
  predicate.

## What the unit builds

**1. `Datum::FaceFrame { at: RecipeNodeId, face: StableName, spin: Expr }`**
(`node.rs`, beside `Frame`). `at` is a DAG edge to a body-denoting node
(`Node::inputs` returns `[at]`, exactly as `AxisInPlane` returns its
`plane`); `face` is a frozen face name resolved through `at`'s value
(`Node::payload_names` lists it, so `InsertNode`'s liveness check and
`Rebind` cover it — the `Fillet` selection precedent, `node.rs:1985`,
`:2010`); `spin` is a continuous angle slot (`SlotId::Spin`, new,
angle dimension) — the authored rotation of sketch +x about the
normal. The variant's doc states DM1's three sentences: derived not
frozen, the failure mode is the fillet's, the normal is the outward
one.

**2. Evaluation** (`eval/wire.rs`, a `wire_datum` arm): read `at`'s
value through `body_operand` (`wire.rs:457`; a non-body refuses
`WrongOperand` as every body door does); resolve `face` through that
value's name table under the N5 ladder (`ladder::live` → `landing` →
`resolve`, the `resolve_selection` shape at `wire.rs:1574`), a
non-`Face` key refusing typed; failures are
`NodeErrorKind::FaceFrameResolve { error: Box<ResolveError> }`,
`BlendSelectionResolve`'s twin. Then the carrier-kind read (item 4):
anything but `SurfaceKind::Plane` refuses
`NodeErrorKind::FaceFrameNotPlanar { carrier: SurfaceKind }` (DM1b).
Then the pose with its sense (item 3): outward normal `n = sense ·
axis`; sketch +x `u = rotate(u_ref, n, spin)`; `v = n × u`; origin the
carrier's origin. The value is **`DatumValue::Frame { origin, u, v }`**
— the same value a `Datum::Frame` yields, produced through the same
`frame_axes` door so a degenerate pair refuses at the same site —
which is what makes every reader of a frame (`ProfileProgram::plane`,
`AxisInPlane::plane`, `axis_frame`, `stackup.rs:852`, the viewer's
sketch and scene) work unchanged BY VALUE. Content key: a new node tag
(the next free under the `node_tag_space_is_injective` census; do not
reuse), hashing the spin slot, the upstream key, and the face name the
way a blend's selection is hashed.

**3. The sense beside the pose** (DM1a; `crates/topo/src/readback.rs`,
`crates/editor-core/src/names/interrogate.rs`). `Pose<T>` gains
`sense: bool`, the face's orientation sense copied out
(`entity.rs:277`), documented as the second fact `axis` deliberately
does not fold in; `face_pose` fills it and `names::interrogate::face_frame`
forwards it. `axis` keeps its chart meaning. `edge_pose`/`edge_frame`
carry no sense (an edge has none); say so at the field. The mate tool
reads `face_frame` today (`crates/viewer/src/matetool.rs:518`) and is
unaffected by an added field; do not change what it does.

**4. The carrier-kind read** (DM2). `topo::readback::face_carrier_kind(body,
face) -> Result<SurfaceKind, ReadbackError>` — the `SurfaceKind` tag
(`crates/geom-brep/src/intersect.rs:89`) copied out, refusing only the
dangling arms; and `names::interrogate::face_carrier_kind(ev, node,
name)` as its `StableName` twin through the same node ladder
`face_frame` uses (`interrogate.rs:231`). Rule 1's text at
`readback.rs:11-17` and its mirror at `interrogate.rs:22-30` is
tightened to say NUMERIC predicates are what no door decides, with
"is this face planar" moved to the example of a tag read that IS
answered. `select_where`'s `SurfaceKind` filter (`names/geompred.rs:124`)
is the precedent to cite.

**5. Every consumer that matches on the NODE variant gains the arm.**
The viewer's `NodeKindWanted::Frame` (`session.rs:428`) accepts a
`FaceFrame` — a profile can be drawn on it through `SessionOp::AddProfile`
— and `tree.rs` labels it; `program.rs`'s `plane_input` doc and any
`Datum::Frame { .. }` match that means "a frame node" (`wire.rs:622`,
`:1037`, `sketch.rs`, `scene.rs`) decide, per site, whether they mean
the variant or the value, and say which. The persist wire mirror,
`check.rs`, `refactor.rs`'s node and name remaps, `resolve`'s
names-in-node walk, and `pncad-py`'s exhaustive mirrors take their
arms (the Python-side edits are the mechanical minimum the mirrors
force; no Python door — `Datum.face_frame` is LIB's, filed in the PR
body).

## Acceptance

- **A1 — derived, not frozen.** A box, a `FaceFrame` on its top face
  (spin 0), a profile on that frame, an extrude of the profile. Raise
  the box's height by `SetParam`: re-evaluate with the previous
  evaluation as `prior`, and the frame's origin, the profile and the
  second extrude all move with the face; the frame node recomputes and
  the box's untouched siblings reuse.
- **A2 — the normal is outward.** On a face whose stored sense is
  `false` (find or mint one; `Body::flipped_face_sense_for_tests`
  exists for exactly this), the frame's normal `u × v` is `−axis`, and
  on a `true` face it is `+axis`; a row per sense, asserting against
  `face_pose`'s own `axis` and `sense`.
- **A3 — spin.** With `spin = θ`, `u` is `u_ref` rotated by `θ` about
  the outward normal and `v = n × u`, right-handed; `spin` is a
  continuous slot (`SetParam` reaches it, `slots()` lists it, the
  dimension check refuses a length).
- **A4 — the fillet's failure mode.** Edit the body so the named face
  no longer exists (a cut through it, or a `Rebind` to a name the
  table lacks): the frame refuses `FaceFrameResolve` with the N5 arm
  the situation warrants, and everything above it is poisoned, never
  silently re-anchored. `Rebind` to a live face repairs it.
- **A5 — non-planar refuses typed.** A `FaceFrame` on a revolved
  body's band face refuses `FaceFrameNotPlanar { carrier: Cylinder }`
  (or `Sphere`, per the body), naming the kind.
- **A6 — the two read doors.** `face_carrier_kind` answers `Plane`,
  `Cylinder` and `Sphere` on faces known to be those, the `StableName`
  twin agrees, and both refuse `Dangling` on a stale key / `NoSuchName`
  on a stale name. `face_pose`'s `sense` matches `entity.rs`'s stored
  flag on every face of a corpus body.
- **A7 — by value.** A `FaceFrame` serves as the plane of a profile AND
  as the `plane` of an `AxisInPlane`; a revolve about that axis
  evaluates. The viewer's frame seat accepts it (`require_kind`), and
  the tree names it.
- **A8 — wire.** A document carrying a `FaceFrame` saves, loads and
  replays bit-identical; the corpus gains one document that carries
  one (the tour's own door, per the demo rule: the natural spelling of
  "sketch on this face").
- **A9 — rule 1's text.** `readback.rs` and `interrogate.rs` say
  numeric predicates; the sentence that listed "is this face planar"
  among the refusals is gone.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record; poll it in the foreground; never end
  a turn with background work active.
- **Blinding: NO `Co-Authored-By` or `Claude-Session` trailer in lane
  commits** (the A/B experiment's rule overrides the harness
  convention; if one lands in a pushed commit, note it in the PR body
  and carry on — never rewrite history).
- Merge-only: no rebase, no force-push, no squash. Push early and often.
- Private `CARGO_TARGET_DIR` and private scratch directory, both
  outside the worktree. Read `git status` before every `git add`;
  never `git add -A`.
- The kernel is serde-free; `Pose`'s new field is not persisted
  (evaluations never are). A format change in editor-core is a corpus
  regeneration and nothing else (`persist/mod.rs` module doc).
- Do not touch `resolve/vdiff.rs`, `crates/profile/*`, the analysis
  lane, the `product.rs` Dual arms, `mate.rs`/`matetool.rs` behaviour,
  or `crates/pncad-py` beyond what its exhaustive mirrors force.
- Do not add a frozen-frame door, a planarity PREDICATE (a tag read is
  the whole gate), a derived `MateFrame`, or a chrome affordance; each
  is a finding for the PR body.

## Out of scope

The add-profile chrome (`work/chrome/add-profile-mints-no-frame`,
`add-profile-placement-on-picked-face-frame`); the Python surface
(LIB); the mate side (frozen by A11, unchanged).

## Review

v6 dual on the frozen head, claims to falsify (the reviewers get these
verbatim plus `docs/prompts/reviewer-style-lane.md` by path):

- **C1** The frame is derived: A1 moves with the face and the memo
  recomputes exactly the cone (the reviewer builds a different body
  and edit).
- **C2** The normal is `sense · axis` on both senses (A2), and `spin`
  rotates about THAT normal, right-handed (A3) — the reviewer checks
  the handedness with a spin the implementer did not choose.
- **C3** A vanished or non-planar face refuses typed through the N5
  ladder and DM1b, never re-anchors, and `Rebind` repairs (A4, A5).
- **C4** `face_carrier_kind` reads a tag and decides nothing; rule 1's
  text now says numeric; no door anywhere in the diff compares a
  number to decide planarity (grep the diff for a tolerance read).
- **C5** Every reader of a frame VALUE is unchanged and every match on
  the frame VARIANT gained its arm with no wildcard; the new node tag
  passes the census; the wire round-trip replays bit-identical.
- **C6** The corpus document renders through the tour's own door and
  no baseline moved for a numeric reason.
