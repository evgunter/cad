# profile

`profile` is the kernel's planar sketch layer. A profile is a set of
closed loops on a `SketchPlane`; a loop is a vertex chain in which each
vertex carries a **bulge** b = tan(θ/4) for the segment leaving it (b = 0
a line, otherwise a circular arc of signed included angle θ), so every
segment lies on a line or circle **carrier**. Loops are authored through
the PATHS algebra, a typestate lattice whose closing verbs return both
the lowered `ProfileLoop` and the **program** that produced it (the verb
sequence as data). In a document the program is the profile's
definition and the loop is a derived value replayed from it.
`Profile::validate` is the one gate to a `ValidatedProfile`, the only
form sweeps consume; the plane's rigid placement lifts the validated 2-D
data into 3-space (`SketchPlane::to_world`). The algebra itself (binding
lattice, verbs, junction rules, the fillet family) is specified in
`docs/PATHS-DESIGN.md` and is not restated here.

## Where in the code

| Topic | Module |
|---|---|
| Loop data, `RawLoop` doors, `SketchPlane` | `crates/profile/src/lib.rs` |
| Typed authoring surface, `PathError` | `crates/profile/src/path.rs`, `path/family.rs`, `path/verbs.rs` |
| Step vocabulary, transition table, replay driver (V1) | `crates/profile/src/path/program.rs` |
| Arc-carrier fillets, candidate selection, enclosing tangency | `crates/profile/src/path/arc_fillet.rs`, `sugar.rs`, `fillet_select.rs` |
| Structure record for guided replay (V2, V3) | `crates/profile/src/structure.rs` |
| Validation ladder, canonical form (V6) | `crates/profile/src/validate.rs`, `seg.rs` |
| The v1-form → program lift tool (V5) | `crates/profile/src/lift.rs` |
| Expr-bearing program, slots, authoring-time check (V2, V4) | `crates/editor-core/src/program.rs`, `node.rs` |
| Persisted form (V4) | `crates/editor-core/src/persist/wire.rs` |
| Evaluation: resolve, replay, validate, naming anchor (V2, V3) | `crates/editor-core/src/eval/mod.rs`, `eval/wire.rs`, `eval/anchor.rs` |

## Profiles as programs

**V1 — The stored program is the constructor-call sequence as data.** A
loop program is an ordered list of `Step`s, one per algebra verb, each
holding only authored data (the verb tag plus the author's arguments;
`Via`/`Center` arcs keep their authored points and derive the bulge at
replay). The lattice markers are a compile-time property of the
authoring surface; the stored form is the erased step list, re-armed by
**replay**: `profile::replay(&[Step<T>], tol)` holds the in-flight tip
as `DynTip`, an enum over the lattice states each carrying the typed
`PartialPath` value, and applies a step by a match on (state, verb)
whose arm can only call the one typed binder well-typed there. Typed
method, driver arm, `Step` variant and `Verb` tag are projected from one
`transition_table!` row, so a transition cannot exist in one surface
and not the other. The typed surface records as it lowers: a closing
verb returns `ClosedLoop { loop_, program, structure }`. Replay is the
only path from steps to geometry; serde (in `editor-core`) is transport,
never a constructor. Two refusal classes (`ReplayErrorKind`):
`Transition` (a verb or arc mode ill-typed at the tip's `TipState`; no
authoring surface produces it, so it is the corrupt-or-hand-edited-file
class, refused typed at load) and `Path(PathError)` (a well-typed chain
whose geometry refuses under this binding; legal at rest, surfacing as
the node's typed evaluation error). Record → replay bit-identity is
pinned for every closing verb the suites author
(`tests/common/mod.rs::pinned`).

**V2 — Expression binding.** In the document form (`editor-core`'s
`ProgramStep`, the mirror of `Step`) every continuous scalar is an
`Expr`: coordinates, lengths and radii are Length; `angle`, `turn` and
the `circle_split` phase are Angle; bulges and `toward` director
components are Scalar. Structural data (the verb tag, `Start`,
side/winding tags, the `circle_split` count) stays literal; changing it
is re-authoring. An expression is addressed
`SlotId::Profile { loop_, step, arg: StepArg }`, `StepArg` being the
closed per-verb role enum; step indices are stable because structure
changes only by re-authoring. Evaluation resolves the program at f64
(`ProfileProgram::resolve`), replays it, embeds the loops into the lane
scalar and validates there. Structure (junction classes, fillet fits and
candidate picks, canonical start, loop roles) is selected once, at f64,
identically for every scalar lane (the rule the code cites as C6), which
is why profile expressions are f64-pinned while node magnitude slots are
lane-live. Under `ProfileLift::Guided` the same program is also resolved
at the lane scalar and replayed through `replay_guided`, consuming and
re-verifying the f64 pass's `ReplayStructure` instead of re-deciding it
(the profile-parameter lift, `crates/editor-core/README.md`). Junction
checks re-run under every binding, and every declared tangency is
re-verified by `validate`, never trusted. `ProfileProgram::check` runs
resolve + replay + validate under the current parameter environment at
the edit door (`ProgramRefusal::{Resolve, Transition, Geometry,
Validate}`); evaluation re-runs the same ladder per binding.

**V3 — Caches and provenance.** Replayed segments, the structure record
and the naming anchor are derived values: memoized per node under a
content key that hashes the program's structure and resolved values
(and the lane-resolved values under `Guided`), never persisted, rebuilt
on load; D9 makes the rebuild bit-exact. Profile-entity names
(`ProfileEdgeRef`/`ProfileVertexRef`) for program loops index
program-structural positions: `eval/anchor.rs` recovers each loop's
canonical rotation and reversal as a `LoopAnchor` by bit-matching the
canonical loop against the replayed one and remaps emitted names
canonical → program order, so nothing geometric enters an index and a
continuous edit cannot renumber. `validate` still canonicalizes
(lex-min start, outer counterclockwise) for downstream geometry.
Structural edits may renumber; stale selections then refuse Vanished.

**V4 — The stored form, chain-only.** `Node::Profile` carries
`ProfileProgram { plane: RecipeNodeId, loops: Vec<LoopProgram> }`;
`LoopProgram` is `Chain(Vec<ProgramStep>)`, `Circle { centre, radius }`
or `CircleSplit { centre, radius, n, phase }`, the carrier forms being
one-step programs whose form is structural. There is one wire
vocabulary: no raw vertex-table loop exists at rest (VQ1). The wire
shape is `WireProfile { plane, loops }` with `deny_unknown_fields`; the
format carries no schema version and no migration, and a file this
build cannot read refuses `PersistError::Unreadable` with the regenerate
recourse. `plane` references a `Datum::Frame` node, so a profile has a
DAG input; evaluation resolves the frame at f64 for structure selection
(`eval/wire.rs::profile_plane_f64`). Raw loop data stays kernel
vocabulary through the `RawLoop` trait (`new`, `polygon`,
`with_tangent_joints`), omitted from the `pncad::profile` façade;
`ProfileLoop`'s fields are private, so outside this crate a loop exists
only through the lattice or that trait. `continue_to` is a lattice verb
the document vocabulary does not spell yet
(`RecordedProgramError::VerbNotInDocumentVocabulary`).

**V5 — The v1-form → program lift is a development tool.** `profile::lift`
mints a chain- or carrier-vocabulary program from a vertex+bulge loop
with declared joints: declared junctions become `.tangent()`, every
other junction a sharp `line_to`/`arc_to`, the seam rotated to the first
undeclared joint (a fully declared loop refuses
`LiftRefusal::AllJointsDeclared`); no director is ever emitted, so no
`sin_cos` quantization enters a lifted program; fillets are not
recovered (un-trimming a corner is inference, not a flag read).
Structural walls are `LiftRefusal`; geometric walls are the driver's own
`ReplayError` through `LiftOutcome::ReplayRefused`. `lift_checked`
lifts, replays and compares, reporting `Fidelity::BitIdentical` or
`ValueEqual`. It never runs at load: a v1-form document refuses at the
persistence header door.

**V6 — What programs do not change.** The verify layer runs unchanged on
replayed output under every binding: flags verified-never-trusted
(`UndeclaredTangency`, `TangencyContradicted`), same-carrier
continuation is identity, fit gating; `ValidatedProfile` is minted only
by `validate` on segments, and extrude/revolve/fillet/loft/sweep never
see a program. Junction predicates classify at replay exactly as at
typed authoring. Replay is deterministic (libm-pure, no ordering
effects). A chain's seam still sits at a junction or fillet, never
mid-carrier (PQ4); `circle` and `circle_split` author no seam (their
split is a private lowering), so they are program forms, not a PQ4
relaxation. `LoopBuilder` no longer exists; the differential suite
(`tests/path_differential.rs`) compares against recorded fixtures and
independent closed-form oracles.

**V7 — Question ledger.**

- **VQ1 — Chain-only.** One program vocabulary; adding a raw vocabulary
  later would be additive, removing one is not, so none ships.
- **VQ2 — Derived segments are not persisted.** See V3.
- **VQ3 — Edit addressing is (loop, step, argument role).** The Expr
  sub-path mechanism (`Expr::descend`) is reused unchanged.
- **VQ4 — Exact directors.** `toward(dx, dy)` stores the normalized ray
  verbatim; `angle(θ)` stays for genuinely angular authoring.
- **VQ5 — Steps are core verbs only.** Anything built above the table
  expands to table steps at authoring; no sugar step exists on the wire.
- **VQ6 — One tolerance.** Replay-time junction checks and both
  validations run under the run's `Tolerance::get()`, reached through
  the `Tol` witness; there is no per-call tolerance value.
- **VQ7 — NURBS legs are not implemented.** `ProfileLoop` has no NURBS
  segment; the program form carries them the day the segment layer does.
- **VQ8 — Plane placement is not an expression slot.** It is a frame
  node reference (V4), stored apart from the loop programs.
- **VQ9 — Checks bind twice.** At the edit door under the current
  environment (fail-loud early) and at every evaluation (V2).

**V8 — Acceptance shape.** The record → replay differential pin (V1),
plus a corpus scene with a real parameter driven from geometry into a
typed refusal naming the step
(`crates/editor-core/tests/corpus/plate_param.rs`).

## Enclosing tangency

A fillet rounds the corner between two legs with a blend arc tangent to
both leg carriers. For a circular leg of radius R the blend circle's
centre lies on the leg's **offset carrier**, the concentric circle of
signed radius ρ = R − σ·τ·r, where σ is the corner's turn sense and τ
the leg's sweep sense (`sugar::offset_radius`). ρ > 0 is the ordinary
tangency. ρ < 0, which is exactly σ·τ = +1 with r > R, is the
**enclosing** tangency: the only circle of radius r tangent to the
carrier with those senses contains the carrier whole, hence the corner,
which is a point of it, so the arc cannot touch the corner it was asked
to round. ρ < 0 on one leg forces ρ < 0 on its partner unless the corner
is degenerate.

**Decision.** The enclosing class is permanently unreachable: no door,
shipped or future, emits it, and a request whose radius demands it is
answered by a typed refusal, not by construction. An arc that cannot
reach the corner is not a fillet of that corner.

**Where it refuses.** `sugar::arc_fillet_trims` classifies ρ for every
circular leg (`fillet_enclosing_carrier`, linear band) as soon as σ is
decided, before any candidate centre exists; both legs are classified so
the bound named is the tightest. Negative refuses
`ArcTrimRefusal::EnclosesLegCarrier`, surfaced as one entry of the
refusal envelope below — `CornerReason::EnclosesLegCarrier { side,
carrier_radius, offset_radius, largest_tangent_radius }`: `side` is
`None` when both carriers are swallowed (the ordinary case),
`carrier_radius` is the class bound (necessary, never sufficient), and
`largest_tangent_radius` = (R₁ + R₂ − d)/2 is the existence bound the
recourse endorses when the corner's two circular carriers define one.
Zero (r within the band of R) escalates with the enclosing recourse. In
`path::arc_fillet::resolve` the refusal rides the construction channel
with the other corner refusals rather than aborting, because the carrier
pair's other crossing turns the other way and may serve the same radius
as an ordinary tangency. It is its own reason, not laundered into a "no
corner" one: the corner exists; a fillet of it at this radius does not.

**What stays.** `Leg::tangent_point`'s antipodal flip (the ρ < 0 tangent
point) remains as the closed form's sign rule, unit-pinned and
unreachable by any door. No construction is known to reach
`NoCornerReason::NoCornerSideCandidate` since the class refuses earlier
(`work/issues/nocornersidecandidate-has-no-producer.md`). The pins are
`tests/review_s2.rs`'s `the_lattice_door_never_emits_an_enclosing_tangency`,
`enclosing_fillet_swallows_both_leg_carriers` and
`an_enclosing_leg_forces_an_equally_enclosing_partner`. The 3-D blend
verbs in `crates/sweep` are outside this decision.

## The refusal envelope: every refusing crossing, named

**A refusal about a corner names the corner, and a refusal about a pair
names every corner it tried.** A carrier pair derives 0, 1 or 2 corners.
If any of them takes the fillet the resolve succeeds, so a refusal that
names a corner at all is `PathError::NoCornerOfPair { radius, corners }`
— one `CornerRefusal { at, reason }` per corner that refused, the point
beside the reason, and the deixis of every sentence is "this corner".

`CornerReason` has four arms, and each carries the payload its retired
variant carried, field for field: `OutsideAnchors(CornerWindow)` (the
advance and reach windows), `NoTangentCircle(NoCornerReason)`,
`AnchorOutsideTrimmedExtent { side, carrier, setback, available }` and
`EnclosesLegCarrier { side, carrier_radius, offset_radius,
largest_tangent_radius }`. The requested radius is named once, on the
envelope.

Two refusals stay outside it, for the same reason in both cases — they
are not about a corner. `NoCornerForFillet { reason, radius }` carries
the pair-level conditions that name no corner to be about
(`PathNoCornerReason`: `CarriersParallel`, `CarriersDoNotMeet`), and
`FilletOffsetLeverTooShort` aborts the resolve where it fires, because a
lever the band cannot support at one corner is a conditioning fact about
the run rather than a fact about the pair.

**Which corners are entries.** The resolve keeps two channels — corners
the anchor windows discarded, and corners that passed them and then
failed to admit a tangent circle — and the construction's channel
answers when it is non-empty. So a corner the author did not bracket is
never listed beside the answer about the corner they did; the entries
are the whole of the answering channel, never a pick from it.

**Order is presentation, not truth.** Entries are sorted by the sum of
the distances from the corner to the two bracketing anchors, ascending,
ties on enumeration order — the first sentence is the corner the author
most plausibly meant. The sort key is an `f64` enclosure read of a
quantity nothing decides on; nothing in the kernel branches on the
order, and no entry outranks another. The pins are
`tests/fillet_refusal_envelope.rs`.
