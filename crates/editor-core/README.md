# editor-core

`editor-core` is the headless document layer between the kernel op crates
(`profile`, `sweep`, `topo`) and any client. A document (`Doc<P>`, `src/doc.rs`)
is a plain value: the recipe (the feature DAG of `Node`s, data only, DESIGN.md
D8) plus named parameters, the recorded tolerance ε, per-node witness data,
appearance attributes and metadata. All mutation goes through the typed
`DocEdit` vocabulary and the pure `apply(doc, edit, tol) -> Applied`
(`src/edit.rs`), which returns a new document; undo is keeping the prior value.
The evaluation service (`src/eval/`), `evaluate(doc, prior, cancel, opts, tol)
-> Evaluation<T>`, runs the live nodes in a deterministic topological order,
reuses prior results by content key (`src/eval/memo.rs`: bit-exact inputs plus
ε hashed, so equal key implies equal output under D9), poisons only a failed
node's descendants, and is generic over the scalar lane `T` (the `f64` build
lane; the `Interval` and `Dual` analysis lanes). Persistence is `src/persist/`.

Elsewhere: the three-layer split and the GUI, `crates/viewer/README.md`;
persistent naming, `crates/editor-core/src/names/README.md`; assemblies and
mates, `crates/editor-core/ASSEMBLY.md`; the error-propagation lane (analysis,
distributions, duals, stackups), `docs/ERROR-DESIGN.md` and
`docs/DUAL-DESIGN.md`. This page holds the witness mechanism (W1–W9), the group
boolean, and the profile-parameter lift (PP1–PP6).

## Where in the code

| Decisions | Modules |
|---|---|
| Witness W1–W9 | `src/witness.rs` (`WitnessDatum`, `BranchCertification`, `WitnessBifurcation`); `src/edit.rs` (`ReWitness`, `ReWitnessBulk`); `src/doc.rs` (`Doc::witness`); `src/eval/mod.rs` (`NodeErrorKind::WitnessBifurcation`, `content_key`); `src/resolve/mod.rs` (`Diagnosis::WitnessBifurcation`) |
| Group boolean | `src/node.rs` (`Node::PlacedUnion`, `PatternKind::Explicit`, `Node::placement_rule_fault`); `src/eval/wire.rs` (`wire_placed_union`); `src/names/emit.rs` (`name_placed_union`); `crates/topo/src/separation.rs` (`Separation`); `crates/topo/src/instance.rs` (the graft doors) |
| Profile lift PP1–PP6 | `src/eval/wire.rs` (`prepare_profile`, `lane_profile`, `section_of`); `src/eval/mod.rs` (`ProfileLift`, `EvalOptions`, `content_key`); `src/eval/anchor.rs` (`derive_naming`); `src/program.rs` (resolution at `T`); `crates/profile/src/structure.rs`; `crates/profile/src/path/program.rs` (`replay_recording`, `replay_guided`); `crates/profile/src/validate.rs` (`validate_recording`, `validate_guided`); consumers `src/drive.rs`, `src/stackup.rs` |

## The witness mechanism (W1–W9)

A constrained sketch has a discrete solution set; the *witness* is the
recipe-stored datum saying which solution the user meant (branch selection, not
the rigidity-probing "witness configuration" of the constraint-solving
literature). Selection is `solution(constraints, params, witness)`, pure in
exactly those three; continuation along the edit path is banned. The sketch
solver is not implemented: what exists is the document contract below.
`WitnessBifurcation` is never constructed today and the `solver_branch_margin`
predicate does not exist yet.

**W1 — The witness is the committed solved assignment.** The full solved
coordinate assignment (f64, kernel units) recorded at the last committed sketch
edit, with the parameters it solved under. Discrete invariants (chirality,
tangency side, arc orientation) are derived from it, never stored. At the
document layer it is opaque: `WitnessDatum { schema, bytes }`, kept per node in
`Doc::witnesses`, persisted bit-exactly as hex, accepted only on `Node::Profile`
(the sketch-bearing node; `EditError::WitnessOnNonSketch` otherwise).

**W2 — Selection is certification, not search.** Deterministic Newton iteration
from the witness proposes a candidate; the decision is a Krawczyk /
Hansen–Sengupta containment `K(X) ⊆ int X` on an ε-inflated box enclosing
witness and candidate, proving at once that the box holds exactly one root (the
witness lies in the returned root's uniqueness region) and that the interval
Jacobian is regular throughout (the distance-to-singularity margin). There is
no nearness metric in the semantics. The margin is a named k_stats trilean,
`solver_branch_margin`; a small margin escalates instead of selecting. The
interval lane runs the same containment from the f64 witness and contracts; it
never solves. Refusal after the bounded inflation schedule is typed (W3), never
a retry loop.

**W3 — `WitnessBifurcation`, the typed refusal.** `kind` (`FoldProximity`: the
regularity margin entered the sliver band; `AmbiguousBasin`: inflation
swallowed several roots; `ResidualFailure`: the committed assignment failed
ε-certification), `margin` (`BranchMarginEvidence`, the certified bound against
its band), `implicated` (entities by stable name, constraints by index, from
the interval Jacobian's near-nullspace), `witness_age` (params solved under vs.
now). Layer 2 of the two-layer DOF diagnosis: never rendered as
over/under-constrained (layer 1, structural) and never as "did not converge".
It reaches the DAG as `NodeErrorKind::WitnessBifurcation`; a name failing to
resolve through such a node carries it as `Diagnosis::WitnessBifurcation`.

**W4 — Witness update: commits only, repair explicit.** The witness changes at
a committed sketch edit and at the recorded `DocEdit::ReWitness { node,
witness }`; parameter-edit rebuilds never write it back (continuation smuggled
through the document). `ReWitnessBulk { entries, certification }` records many
adoptions and is legitimate exactly when the W2 certificate proves each old
witness and new solution share one uniqueness region, which changes no
predicate outcome; `apply` validates shape only (live sketch nodes, no
duplicates, non-empty) and the checker consuming `BranchCertification` is not
implemented. A drag is a user-supplied homotopy and may author the proposal:
fold-free, it identifies the endpoint branch with no dialog; crossing the wall
where the margin vanishes is explicit recorded intent. Undo/redo and
`SetTolerance` need no special cases.

**W5 — Composition with the result DAG.** The sketch node's value is the
certified assignment; `WitnessBifurcation` poisons descendants only. The
witness datum is an input to the node's content key, so a witness change moves
the key and the pure triple is the cache key's correctness proof.

**W6 — Certification schedule.** At commit: residuals ≤ ε on the committed
assignment, typed on failure. At solve: the W2 containment, its failure W3. At
the interval lane: the same containment at `Interval`; an indeterminate joins
the subdivision driver's posture. Residuals use `powi(2)`, never `x·x`, for
possibly-zero quantities.

**W7 — Certification, not the iterator, is the contract.** The solver engine
(DESIGN.md Q3) is audited for bit identity: libm-only transcendentals, no
hash-map or pointer-identity iteration order, bit-identical results across
builds and platforms, a documented termination schedule. An engine failing the
audit is demoted to a seed proposer outside `build`, with a small deterministic
Newton polish from the witness inside it; W2 carries correctness either way.

**W8 — Mates: contract verbatim, mechanism per manifold.** The contract
(witness = branch selection, purity, certified selection, typed bifurcation
with margin) transfers to mates; the datum is per-node and opaque so a mate
node can store points of ∏SE(3) with no document change. The interface is
"certified-unique in a chart-box centered at the witness", the chart a
deterministic function of the witness (exp-map chart, quaternion double cover
quotiented). The mechanism is not designed; `src/mate/` places instances by a
constructive coset fold with no witness.

**W9 — Structural layer, interface pinned.** Layer 1 of the DOF diagnosis
(combinatorial decomposition, exact and float-free) is a pure function of the
constraint graph: it may key caches but never consults coordinates;
generic-configuration rigidity probes live there. Not implemented.

## The group boolean: `PlacedUnion`

`Node::PlacedUnion { input, count, kind }` is a Pattern that fuses: one
prototype (a body-denoting node), a placement rule, one body out — the union of
the prototype placed at each placement. It is its own node kind, not a
`PatternKind` of `Node::Pattern`: Pattern's N-bodies-unfused contract stays,
and a result type forked on a variant is the dispatch trap D3 forbids. The two
nodes share the rule vocabulary and slot map.

- **Placement rule.** `PatternKind::Linear`, `Circular`, or
  `Explicit(Vec<Frame>)`: absolute frames in listed order, the index
  D8-structural, so appending changes no existing index. "How many placements"
  has one spelling: a stepped rule carries the `Count` slot, `Explicit` carries
  none. `placement_rule_fault`, read by `apply`, the snapshot check and
  evaluation, refuses a count spelled twice, an empty list, and a non-finite or
  improper frame. Face-tied placements and a placement-list edit arm are not
  implemented. Heterogeneous groups are `Node::Union`'s: an n-ary fuse over an
  arbitrary member list, named by member and not by position, edited with
  `DocEdit::SetMembers`.
- **Naming does not grow.** Per-instance discrimination is `RoleSeg::Instance
  { i, of }`, the pattern node's wrapper; each instance's rows are re-keyed onto
  the one output body through that instance's graft bridge, and the body is
  minted as this node's output body (`name_placed_union`).
- **Disjointness is certified, never declared.** `topo::Separation::of` builds
  one BVH of padded conservative face boxes over the prototype; `certify(maps)`
  tests each pair `(i, j)` in the prototype's frame through `M_i⁻¹ ∘ M_j`. Box
  separation is real separation, and the test is sufficient-not-necessary: a
  touching-box but genuinely disjoint arrangement refuses typed
  (`NodeErrorKind::PlacementsUncertified`, the first pair in index order). The
  certificate runs before any body is placed.
- **Lowering.** In placement order (D9): placement 0 goes through
  `graft_disjoint_all_keyed`, minting the destination solids; each later
  placement grafts onto those solids through `graft_disjoint_all_onto_keyed`,
  so the result is the one-solid, N-shell union the pairwise `Boolean(Union)`
  chain produces — the only shape the seamed boolean path accepts as an
  operand. No new kernel op or naming record; `BooleanNaming` stays two-operand.

## The profile-parameter lift (PP1–PP6)

A profile is a program (`crates/profile/README.md`); *replay* elaborates its
resolved steps into a loop and *validation* canonicalizes the loops. Structure
(fillet candidate, fit signs, corner gates, loop roles, canonical start) is
selected once at f64, identically for every lane (C6); geometry at the lane
scalar is the lift.

**PP1 — Guided replay: the f64 elaboration is the witness.** Pass 1
(`prepare_profile`): resolve at f64, `replay_recording`, `validate_recording`,
`derive_naming`, emitting the structure record. Pass 2 (`lane_profile`, under
`ProfileLift::Guided`): resolve the same program at `ParamEnv<T>`;
`replay_guided` and `validate_guided` consume every discrete decision from the
record and re-run its predicate at `T`. Agreement proceeds; an indeterminate
refuses `StructureRefusalKind::Indeterminate` (the E6 driver's cue to bisect);
a definite disagreement refuses `Flipped { recorded, found }` (the binding
provably left the nominal's structure, the `FlipCrossing` shape). Both surface
as `NodeErrorKind::ProfileLaneReplay`. No lane ever selects: the fillet ladder
never ranks at `T` (in a hairline-asymmetric lens two lanes may legally pick
different pockets, so re-ranking is a second choice, not a check).

**PP2 — The structure record.** `profile::ProfileStructure`: per loop a
`ReplayStructure` (each fillet's corner-gate outcomes, survivor count, chosen
candidate index, fit signs) and a `CanonicalStructure` (per loop: role,
containment row, representative vertex, reversal, canonical start, per-segment
shapes, tangent-joint set). Derived, content-keyed, never persisted.

**PP3 — Canonicalization is pinned.** `validate_guided` takes rotation and
reversal from the record instead of re-running `lex_min` and the orientation
decide (ulp-wide bands: total at f64, indeterminate at `Interval` on
essentially every input); it verifies the value channel they induce (segment
shapes, declared joints) and re-runs the containment forest, an ordinary
decided predicate, against the record.

**PP4 — Naming stays f64.** `derive_naming` runs on pass 1 only; names are
program-structural indices and the lane pass takes them verbatim. `T`-valued
geometry changes no name because the canonical permutation is pinned (PP3);
the two decisions are one commitment.

**PP5 — Content keys.** The f64-resolved program stream remains the structural
identity; when pass 2 runs, the `T`-resolved program's continuous arguments
also feed the key through `ContentBits::feed` (both channels of a dual,
DUAL-DESIGN DL2), so a seeded or widened profile parameter cannot alias the
nominal memo entry. Keys never persist.

**PP6 — Scope and the bit-identity fence.** `ProfileLift::Pinned` is the
default and the build path: profile geometry is the f64 elaboration embedded
through `from_f64`, unchanged. `Guided` is set by the E6 driver (`src/drive.rs`)
and the E4 sensitivities (`src/stackup.rs`). Guided replay at `f64` reproduces
plain replay bitwise (`tests/m10_p_fence.rs`,
`crates/profile/tests/scalar_channels.rs`). The sweep/loft ladder is the same
function: `section_of` calls `prepare_profile` and runs pass 2 as a gate only —
a section's geometry stays f64 (the skinned surface's structure must be
lane-identical), so a seed on a parameter the section reads refuses
`SeedPinnedSection` rather than arriving as a zero. **The sketch plane, by
frame kind.** An AUTHORED frame's plane stays f64 under every lift
(`profile_plane_f64`, read from the document's own slots), so its
profile's placed coordinates are exact points at every scalar. A DERIVED
frame (`Datum::FaceFrame`, DOCM-REFERENCES-DESIGN DM1) has no document
elaboration — its value is read off the evaluated body — so its profile
is placed at the lane scalar through `frame_plane_lane` under every
lift, and its placed coordinates at `Interval` are enclosures, which is
what a derived frame is at that scalar; the 2-D structure record (PP1)
is f64-pinned exactly as before, the placement is the one thing that
moves to `T`. A loft or sweep SECTION on a derived frame refuses typed
at any scalar but f64 (`DerivedFrameSection`, the `SeedPinnedSection`
shape), since a section's geometry stays f64. Out of scope: naming,
persistence, the sketch solver.

## Open

- The sketch solver: build vs. bind (DESIGN.md Q3).
- The mate-witness mechanism (W8; `ASSEMBLY.md`, A1).
