# profile

`profile` is the kernel's planar sketch layer. A profile is a set of
closed loops on a `SketchPlane`; a loop is a vertex chain in which each
segment is a **carrier** plus a signed interval on it — a line between
its two vertices, or a circular arc stored as centre, radius and signed
sweep Δθ with |Δθ| ≤ 2π, so a full circle is one segment at one vertex.
Vertices are stored verbatim and are authoritative; validation verifies
that they lie on their carriers. The **bulge** b = tan(Δθ/4) (b = 0 a
line) is the `arc_to(Bulge { p, b })` mode's input, lowered to the
carrier form once, at the algebra. Loops are authored through
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
and not the other; the row also carries the word the verb is CALLED,
which is `Verb`'s `Display` — the authoring spelling, for a sentence
about the step a person wrote (`Debug` is the variant identifier, which
is what the table-coordinate sentence renders). The typed surface records as it lowers: a closing
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
side/winding tags, the `circle_split` count) stays literal; no slot
edit changes it. An expression is addressed
`SlotId::Profile { loop_, step, arg: StepArg }`, `StepArg` being the
closed per-verb role enum; step indices are stable under every slot
edit because structure changes only by `DocEdit::SetProgram`, which
replaces a live profile's program whole, says which old step each new
step keeps by its minted id, and reports every name on a step it drops
(`crates/editor-core/REFERENCES.md` DM7; *ruled by Ev on EDIT's `[ev]`
PR #2904, 2026-09-20, and on #3193, 2026-09-25*). Evaluation resolves the program at f64
(`ProfileProgram::resolve`), replays it, embeds the loops into the lane
scalar and validates there. Structure (junction classes, fillet fits and
candidate picks, loop orientation, loop roles) is selected once, at f64,
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
content key that hashes the program's structure, its steps' minted ids
and its resolved values (and the lane-resolved values under `Guided`),
never persisted, rebuilt on load; D9 makes the rebuild bit-exact. Every
verb that consumes a profile — a loft's sections included — iterates
its CANONICAL positions, and the canonical form keeps what the author
wrote wherever validity allows: `validate` orients each loop (outer
counterclockwise, holes clockwise) and keeps its AUTHORED start vertex
and the authored hole order, so canonical segment `k` is the author's
segment `k` for a loop authored in its canonical sense and segment
`n − 1 − k` for one authored against it. A canonical position is not a
name. Profile-entity names (`ProfileEdgeRef`/`ProfileVertexRef`) spell
the piece a position is — its step's minted id and its role in that
step's fixed list (`crates/editor-core/src/names/README.md`, "N1, the
profile pieces") — which the replay records per segment
(`ReplayStructure::pieces`), so no value edit, loop-role change or sense
flip moves a name. `eval/anchor.rs` recovers each loop's reversal as a
`LoopAnchor` by bit-matching the canonical loop against the replayed
one, and pairs each canonical segment and vertex with its piece
(`ProfilePieces`). A step `SetProgram` drops takes its id with it, and
its names resolve Vanished.

*Record: the canonical numbering was ruled by Ev on PR 3102's thread
(2026-09-23); that names spell minted step ids rather than canonical
positions was ruled by Ev on #3193 (2026-09-25), with the role lists on
#3202.*

**V4 — The stored form, chain-only.** `Node::Profile` carries
`ProfileProgram { plane: RecipeNodeId, loops: Vec<LoopProgram>, ids: Vec<Vec<StepId>> }`;
`LoopProgram` is `Chain(Vec<ProgramStep>)`, `Circle { centre, radius }`
or `CircleSplit { centre, radius, n, phase }`, the carrier forms being
one-step programs whose form is structural. There is one wire
vocabulary: no raw vertex-table loop exists at rest (VQ1). The wire
shape is `WireProfile { plane, loops }` with `deny_unknown_fields`,
which denies an unknown KEY and nothing else — a `plane` written in the
pre-node shape refuses at `plane_ref`'s own visitor instead; the
format carries no schema version and no migration, and a file this
build cannot read refuses `PersistError::Unreadable` with the regenerate
recourse. `plane` references a `Datum::Frame` node, so a profile has a
DAG input; evaluation resolves the frame at f64 for structure selection
(`eval/wire.rs::profile_plane_f64`). Raw loop data stays kernel
vocabulary through the `RawLoop` trait (`new`, which takes the
canonical form, `polygon`, `with_tangent_joints`) and the
`test_support::bulge_loop` helper, which hands a bulge chain to the
lowering; both are omitted from the `pncad::profile` façade.
`ProfileLoop`'s fields are private, so outside this crate a loop exists
only through the lattice, the `map_scalar` materialization door, or
those fixture doors — the trait's item is declared `pub(crate)` and the
helper's module does not exist in any build satisfying neither `test`
nor `test-support` (`ProfileLoop`'s own docs are the one home for the
door list). `continue_to` is a lattice verb
the document vocabulary does not spell yet
(`RecordedProgramError::VerbNotInDocumentVocabulary`).

**V5 — The v1-form → program lift is a development tool.** `profile::lift`
mints a chain- or carrier-vocabulary program from a lowered loop (its
vertices and the bulge each segment was lowered from) with declared
joints: declared junctions become `.tangent()`, every
other junction a sharp `line_to`/`arc_to`, the seam rotated to the first
undeclared joint — and when there is none, seamed at 0 with the closing
target carrying joint 0's declaration (`Start.arrives_tangent()`); no
director is ever emitted, so no
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
continuation is identity and is legal declared or undeclared, fit
gating; a `ValidatedProfile` is minted by the validate doors
(`validate`, `validate_recording`, `validate_guided`) on segments and by
`ValidatedProfile::lift_onto` from an `f64` one, and
extrude/revolve/fillet/loft/sweep never see a program. Junction
predicates classify at replay exactly as at
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

## The fillet door never mints a joint the verify layer refuses

A fillet's declared tangency is a claim about the carriers the loop
STORES, and a loop stores an arc as its chord and a bulge. Two things can
happen to that claim between the door's arithmetic and the stored form,
and both are refused at the door rather than left for validation. A
fillet whose sagitta `r(1 − cos(θ/2))` sits at or below ε is read back as
a straight segment, so the carrier is simply not there
(`PathError::FilletArcFlattenedInStorage`; the levers are a larger turn
or a larger radius). A fillet whose stored arc IS an arc can still lose
its joint to arithmetic: a carrier clearance is a difference of lengths
at the scene's own magnitude, so it resolves only to about that
magnitude times 2⁻⁵², and past the radius or the distance from the origin
where that floor is coarser than ε the joint cannot be classified at all
(`PathError::FilletCarrierBelowSceneResolution`; the levers run the other
way — a smaller radius, or the geometry nearer the origin). Both read
back through `seg::build_seg` and `seg::joint_tangency`, the verify
layer's own classifications, so the door's answer is the verify layer's
answer and no new predicate name enters the K stream. The in-band twin of
either is relayed as `PathError::Escalated` naming the same
classification. The pins are `tests/fillet_stored_tangency.rs` and the
three sentences' rows in `tests/fillet_recourse_followability.rs`.

**Every fillet gate's in-band verdict carries that gate's own recourse.**
`PathError::Escalated`'s `Display` asks `validate::fillet_recourse_for`
for the sentence belonging to the escalation's predicate name — the one
map from the nine `fillet_*` names to the six `FILLET_*_RECOURSE`
sentences, several names sharing a sentence because they share a user
situation (D4 ¶1's addendum) — and renders the site it was resolving
with that sentence and no coincidence tail: a fillet the caller asked for
has no joint they declared, so "declare the coincidence" names a
declaration that does not exist. The fillet names are asked first, ahead
of the stored-form classifications and the junction keys, and are
disjoint from both. `tests/fillet_recourse_followability.rs` censuses the
nine against the `decide("fillet_…")` call sites across `src`, so a gate
added anywhere in the crate without a sentence is a red row, and so is a
gate that disappears — the census is an equality with the tree.

Six of the nine take an in-band verdict from a request a caller can
author. Of the other three, `fillet_corner_arm` is shadowed by
magnitude — its margin IS the lever arm, so putting it in the band puts a
length-shaped path gate in the band on the same request — and
`fillet_leg_fit` and `fillet_leg_reach` classify against the exact-order
band, which no representable `f64` lies inside: that last is a statement
about `f64` alone, and the interval lane already drives `fillet_leg_fit`
in band. Two of the six are levered or scene-scaled rather than radius-
scaled, which is why their sentences name the leg extent and the bound on
the lever's own window.

**What stays.** `Leg::tangent_point`'s antipodal flip (the ρ < 0 tangent
point) remains as the closed form's sign rule, unit-pinned and
unreachable by any door. No construction is known to reach
`NoCornerReason::NoCornerSideCandidate` since the class refuses earlier;
the item that owned that reading was deleted with its program's tracker
directory and is recoverable in git history. The pins are
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
are the whole of the answering channel, never a pick from it, and the
list is therefore NOT every corner the pair derives.

The reason the two channels are not merged is that the unit's spec asks
for a one-entry envelope where only one crossing sits in the windows.
It is not that merging them would re-rank a gate: nothing branches on
entry order, and both channels yield the same variant, so a merged list
would rank nothing. What a merged list would add is a sentence about a
corner the author did not ask about, next to the answer about the one
they did.

**A refusal that names no corner outranks the envelope.** The pair-level
conditions (`NoCornerForFillet`) and the M8 conditioning gate
(`FilletOffsetLeverTooShort`) are facts about the pair and about the
run; a per-corner sentence instead of one of them would be a smaller
and weaker claim about a situation the whole pair is in. Nothing is
discarded silently — the entries such a refusal outranks are statements
about corners of a pair that has already been refused as a pair.

**Order is presentation, not truth.** Entries are sorted by the sum of
the distances from the corner to the two bracketing anchors, ascending,
ties on enumeration order — the first sentence is the corner the author
most plausibly meant. The sort key is an `f64` enclosure read of a
quantity nothing decides on; nothing in the kernel branches on the
order, and no entry outranks another. The pins are
`tests/fillet_refusal_envelope.rs`.

**One level down, the candidate.** At an arc-carrier corner the offset
carriers admit up to two candidate circles, and an anchor-fit entry
(`AnchorOutsideTrimmedExtent`) can have both of them round the corner
and overrun a leg. The entry's numbers are the candidate nearest to
fitting IN THE SETBACK METRIC — `fillet_select::nearest_candidate`'s
ladder over the candidates' setback pairs, the one home of "the nearest
candidate at one corner" — on that candidate's worse leg, the one whose
setback outruns its extent by more (ties name the incoming leg; a
candidate tie is unreachable by geometry, one candidate being shallower
on both legs). The construction (`sugar::arc_fillet_trims`) carries
every overrunning candidate out at the scalar and compares nothing; the
pick is the door's (`path::arc_fillet::map_refusal`), an `f64` enclosure
read off the diagnostic channel like the sort key above, and nothing
branches on it. What the numbers are NOT: a radius amount.
`setback − available` is the leg's overrun in the setback metric; the
recourse "reduce the radius or move the anchor" is un-metered and true,
and a setback does not scale 1:1 with the radius on either leg kind.
The pins are `tests/fillet_overrun_nearest_fit.rs` and the two review
probe suites beside it (`review_fillet_overrun_nearest_fit_r1_probes`,
`_r2_probes`), whose grid-A recourse census says which way reading the
number as a radius reduction errs.
