# MSOLVE-9 — A mate frame that names a face and resolves at evaluation (spec)

Unit of the `msolve` program. Item `work/msolve/MSOLVE-9.md`. Answers
`work/msolve/mate-frames-resolve-from-a-face-at-evaluation.md` — Ev's
ruling (F) on `[ev]` PR 2256 (2026-09-09), read the item and the PR's
comments in full; LIB's `work/lib/no-door-mints-mate-frame-from-face.md`
is parked on it for the façade and Python half. **Track:** a design
unit — it revises one sentence of a ratified page (`crates/editor-core/
ASSEMBLY.md` A11 rule 5, the solve's INPUTS; the ALGORITHM claim
stays), which lands only with Ev's sign-off on the `[ev]` PR that
carries this spec; then a kernel change to what a mate frame may be
and where it is resolved. One style review plus a correctness arm
(§Review). No A/B row. Sequenced last: it rides MSOLVE-6's reach road
and MSOLVE-8's frame witness.

## What the tree says now

1. **A mate frame is three authored vectors.** `MateFrame { origin,
   axis, reference: [f64; 3] }` (`mate.rs`), `deny_unknown_fields`,
   turned into a placement by `point_at` at the solve's frame read
   (`mate_coset`, through `MateFrame::placement` — MSOLVE-8 makes that
   the frame witness's affine). `Alignment` carries one per side. The
   module doc, the type doc and the Python docstring all say
   "authored data, not geometry read back". A document parameter
   change in a part stales every mate on it silently until the
   at-rest gate refutes it (the PR's measurement: `mate.rs` holds no
   `Expr` and no `ParamName`, so a mate cannot follow even a
   parameter).

2. **The face pose already exists, exact.** `topo::readback::face_pose
   (body, key) -> Pose { origin, axis, u_ref: Option, sense }` answers a
   plane, cylinder, cone, sphere or torus off its surface parameters
   with no tolerance, and refuses a NURBS face `NoCanonicalFrame`.
   `names::interrogate::face_frame(ev, node, name)` reads it through
   the name table at a node of an evaluation. The viewer's mate tool
   (`viewer/src/matetool.rs`) reads the picked face's pose that way
   at AUTHORING time, pulls it back through the solved placement's
   inverse into part coordinates, and stores the three vectors —
   the drift the item was filed about.

3. **The solve already holds the parts' evaluations.** Since MSOLVE-6
   `solve_document` takes `&dyn MateReach`, answered by the
   evaluation's `CacheReach` over its `PartCache` (a cached
   `PartValue { body, names, … }` is the part's own product in the
   part's own coordinates with the part's own name table) and, at the
   doors with no run, by `PartReach::with_resolver`; the edit door's
   maintenance takes the reach; `RefusingReach` refuses typed. The
   road the face pose needs is this one, and it ends in the part's
   coordinates — no placement pull-back.

## What the unit builds

**1. The arm.** `MateFrame` becomes a closed enum with two arms:

```rust
pub enum MateFrame {
    /// Three authored vectors in the part's coordinates (as today).
    Authored(AuthoredFrame),               // { origin, axis, reference }
    /// A face of the part, resolved at evaluation.
    FromFace(FaceFrame),                   // { face: StableName, reference: Option<[f64; 3]> }
}
```

`face` is the PART-LOCAL stable name — a row of the part's own
table, the spelling the part's document authored, never the qualified
spelling a head carries in the assembling document. `reference` is
used when the face's pose carries no `u_ref` and refused, typed, when
neither is present; when the face carries one, an authored
`reference` is refused as a second spelling of one fact (so the arm
never stores a duplicate). On the wire: EXTERNALLY tagged —
`{"Authored": {…}}` / `{"FromFace": {…}}` — each inner struct
`deny_unknown_fields`, so a stray key on either arm refuses and an
untagged frame refuses as the pre-arm shape it is. No reader accepts
the old bare-vector frame: every tracked document that carries a mate
is regenerated with the repo's own tooling in this unit, and what it
moved is said (discipline §3). `MateFrame::authored(origin, axis,
reference)` is the constructor every present literal moves to.

**2. The resolution, on the reach road.** `MateReach` gains a second
method, `face_pose(&self, part: &DocRef, face: &StableName) ->
Result<Pose<f64>, FacePoseRefusal>` (name yours), answered by
`CacheReach` and `PartReach` from the cached `PartValue`: the name
resolved in `names` to a `FaceKey` (the kind asked first, as
`interrogate::read` does), then `readback::face_pose(&body, key)`;
`RefusingReach` refuses `PartUnresolved`. `FacePoseRefusal` is a
closed enum in the resolver's and the readback's own voice — the part
fault unaltered, `NoSuchName`, `NotAFace { found }`, `Ambiguous`,
`NoCanonicalFrame { carrier }`, `NoReference`, `ReferenceRefused`
(an authored reference beside a carried one) — and the solve wraps
it as one new `MateFault` arm naming the mate, the side, the
instance and the part, the way `Unleverable` wraps `LeverRefusal`.
`mate_coset` resolves each side's frame before it reads it:
`Authored` → the frame witness as MSOLVE-8 left it; `FromFace` → the
pose, then the same witness ladder over `(origin, axis, u_ref or
reference)`, so both arms meet `point_at`'s refusals identically. The
pose's `sense` is NOT folded into the axis: the frame's axis is the
chart's, as the readback documents, and the mate's `AxisSense` says
which way the sides point — state that at the arm.

**3. The lever.** `Alignment::lever_arm` reads `‖origin‖` off the
RESOLVED frames, so it moves to the solve's side beside the reach
read (the datum term is formed once per mate, after resolution;
`fold_pair` already forms the sum there). `Alignment::is_finite`
checks authored vectors only — a `FromFace` arm has none. The three
`EvalOptions` doors that build a `PartReach` are unchanged: the
resolver they hold already answers a part's product.

**4. The memo key.** A mate's content key (`eval`'s `feed_alignment`)
feeds the authored vectors today. A `FromFace` side feeds the face
name AND the part's content pin the instantiate node is keyed by, so
a part edit that moves the face moves the mate's key (MSOLVE-4's
invariant: a mate's key carries what the solve's answer depends on).
Pin it: edit the part so the face moves, re-evaluate, the mate's
result is recomputed and the solved pose follows the face; an edit
that leaves the face where it was leaves the key alone.

**5. The doors.** The viewer's mate tool authors `FromFace` for a
picked face — the part-local name is the head's name with the
walk's qualification stripped (the member walk gives the chain; what
remains below the instance is the part's own row) — with `reference:
None`, refusing typed where the pose has no `u_ref` (today's
`MateToolError::NoReference`); the tool's frame arithmetic goes. The
`AddMate`/`SetAlignment` edit doors admit both arms; `apply`'s
maintenance resolves through the reach it already takes. `persist::
check` validates an `Authored` frame as today and a `FromFace` one
structurally (a name in the part-local spelling; nothing to check
numerically). Python: `MateFrame.from_face(face, reference=None)`
beside the vector constructor, the `.pyi`, the `mate_fault_tag`
census, the payload's new arm; LIB's façade half follows by
announcement on `work/lib/no-door-mints-mate-frame-from-face`.

**6. The docs.** `ASSEMBLY.md` A11 rule 5's inputs sentence as ratified
on this unit's `[ev]` PR (the text is in this PR's diff, verbatim);
`mate.rs` and `mate/solve.rs` module docs point at it; the `MateFrame`
type doc lists the two arms and the one thing the readback does not
fold in (`sense`); `pncad.pyi`'s `MateFrame` docstring; `docs/guide/
assembly.md`'s mate paragraph shows `FromFace` as the natural spelling
and the authored vectors as the NURBS fallback; the tour's stand
authors its mate `FromFace` and drops `POST_SEAT`'s derived literal
(the demo is evidence of the door; discipline §3).

**7. The rows.** `crates/editor-core/tests/msolve9_from_face.rs`
(registered in `tests/all.rs`): the item's own document — a post
mated on its cap face, the post's height edited, the mate follows
(the solved pose moves by exactly the height change, bit-exactly
where the arithmetic is exact); a planar, a cylindrical, a spherical
and a toroidal face each resolve to `face_pose`'s frame bit for bit
(compare against `interrogate::face_frame` on the part's own
evaluation); a NURBS face refuses `NoCanonicalFrame` typed at the
mate, side and part; a face with no `u_ref` (a sphere) refuses
`NoReference` without a reference and resolves with one; a carried
`u_ref` beside an authored reference refuses; a vanished name refuses
`NoSuchName`; an unresolvable part faults in the resolver's voice;
the memo-key row (§4); the wire round-trip of both arms and the
stray-key refusal on each; the pre-arm bare frame refusing; C5 over
the tracked documents (each loads; each that holds a mate was
regenerated and moved only by the tag). One
viewer row: the tool authors `FromFace` and the badge names the face
on a refusal. One Python row on `from_face` and the tag.

## Acceptance

- **A1** The item's document: edit the part, the mate follows, no
  gate refutation, nothing stored twice (the saved document holds the
  face name and no vectors for that side).
- **A2** Every analytic carrier resolves to `face_pose`'s frame bit
  for bit; NURBS refuses typed; the `sense` bit is not folded.
- **A3** No verdict moves for `Authored` frames: every mate row in the
  workspace passes unchanged after the literal sweep, and every
  tracked document loads; those that carry a mate are regenerated and
  differ from their prior bytes by the arm's tag alone (C5).
- **A4** The memo key moves iff the face moves.
- **A5** The A11 sentence, the module docs, the type doc, the `.pyi`,
  the guide and the tour agree; the Python surface and census agree.
- **A6** The item and LIB's parked item are closed with `## Closed`
  sections citing the rows (LIB's by announcement if its half
  remains); `work.py lint` clean.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record; poll it in the foreground; never end
  a turn with background work active. Four Cargo workspaces; the
  `MateFrame` change is public API and is checked in all of them
  before the push; `pncad-py`'s clippy with `--features python
  --all-targets` too.
- Merge-only; push early and often; the PR through the GitHub MCP
  tools. Private `CARGO_TARGET_DIR` outside the worktree; `git status`
  before every `git add`; never `git add -A`.
- **Dispatch gate:** this unit dispatches only after the `[ev]` PR
  carrying this spec and the A11 sentence is merged with Ev's
  sign-off. The lane then lands the code; the sentence is already
  ratified text and is not reworded by the lane.
- Fence: `crates/editor-core/src/mate.rs`, `mate/reach.rs`,
  `mate/solve.rs`, `eval/mod.rs` (`CacheReach`/`PartReach`, the mate
  key feed), `eval/parts.rs` (a read door only), `edit.rs`/`persist/
  check.rs` (the two doors' validation of the new arm), `viewer/src/
  matetool.rs`, `pncad`/`pncad-py` (façade re-export, `from_face`,
  `.pyi`, tags, payload, tests), `demos/tour` (the stand), `docs/guide/
  assembly.md`, tests, the items. Nothing in `topo` (the readback is
  taken as it is; a missing door there is filed on its owner's slate,
  and the arm refuses typed meanwhile), nothing in the walk, the
  coset table or the evaluator's placement.
- Comments state the invariant (discipline §4).
- **Stop clause.** If a part's cached product is not in the part's
  own coordinates for some door (a `FromFace` frame would then need
  the placement pull-back the viewer does today); if the memo key
  cannot be made to carry the part's pin without a second content
  channel; or if resolving `FromFace` at the edit door needs the
  maintenance to evaluate a part it does not already evaluate for the
  reach — STOP, write what you measured in the PR as a draft, and end
  your turn.

## Out of scope

A door that freezes a face's frame into authored numbers (Ev: not
built separately — freezing is materializing the arm); a drift
report for authored frames; the roll-reference convention question
half of `mate-clocking-has-no-gui-path` beyond the `reference` rule
above (the rest is CHROME's affordance); expressions or parameters on
mate offsets and the clocking rider (a further unit if it is wanted);
the head-vs-frame face coincidence as a rule (a frame may name a
different face of the same part than the head, and the spec does not
forbid it).

## Review

One style review plus a correctness arm, claims verbatim:

- **C1** A1 on the item's document and on the tour's stand: the mate
  follows the edited face and the saved document stores no vectors
  for that side.
- **C2** A2: for each of plane, cylinder, cone, sphere, torus the
  resolved frame equals `interrogate::face_frame` on the part's own
  evaluation bit for bit; NURBS refuses; `sense` is not folded (a
  face with `sense: false` resolves to the chart axis, and the mate's
  `AxisSense` alone decides the direction).
- **C3** A3: enumerate the mate suites and confirm no verdict moved;
  every tracked document loads, and a regenerated one differs from its
  prior bytes by the tag alone.
- **C4** A4: the mate's key moves under an edit that moves the face and
  is unchanged under one that does not; the solve is not re-run for
  the latter.
- **C5** The two refusal families (`FacePoseRefusal` and the frame
  ladder's) are typed at the mate, the side, the instance and the
  part, and every arm is reachable through a door or its
  unreachability is stated at the arm.

## Amendment (orchestrator, 2026-09-24)

§1's wire changed from untagged to externally tagged, and with it
A3, C3 and the rows' C5 clause. Two things moved under the old
sentence after this spec was written. Ev's ruling on PR 3123 retired
`LoggedEdit`'s two-shape untagged wire, the precedent the sentence
cited: backward compatibility with older files is a reason to remove
code, not keep it. And PR 2702 lands `scripts/gates/persist-no-backtracking.sh`,
which fails CI on any `#[serde(untagged)]` under
`crates/editor-core/src`, because the load door's structured refusal
rides a first-refusal-wins premise an untagged enum breaks. Filed by
PORT as `work/msolve/msolve-9-spec-prescribes-an-untagged-wire.md`,
which this unit closes.

