# topo

`topo` is the B-rep topology crate. A **B-rep** (boundary representation)
describes a solid by its boundary — faces on surfaces, edges on curves,
vertices at points — linked by a **half-edge** structure (each edge is two
antiparallel half-edges, one per bounding face). `Body<T>` holds that in
typed generational arenas: scalar-free topology plus `T`-valued geometry
arenas, built only through the Euler operators (`docs/DESIGN.md` `[topology]`).
Above the arenas the crate carries the validation tiers, plane splitting,
the booleans and their declared-contact lanes, the at-rest coincidence
census, the contact vocabulary, shell/offset surgery and the kernel query
seat. Orientation conventions are stated once, in `src/entity.rs`.
Determinism (DESIGN `[determinism]`): arena iteration is slot-index order, no hash container
feeds a decision, every walk is bounded.

## Where in the code

| Area | Modules |
|---|---|
| Arenas, entities, provenance | `src/body.rs`, `src/entity.rs`, `src/geometry.rs`, `src/provenance.rs`, `src/live.rs` |
| Euler operators | `src/euler.rs` (make), `src/euler_kill.rs` (kill duals), `src/euler_ring.rs` (rings/genus), `src/split.rs`, `src/movefac.rs`, `src/revert.rs`, `src/attach.rs` |
| Validation tiers 1–3, 3′ | `src/validate.rs` (`validate`, `validate_closed`, `validate_geometric`, `validate_pseudomanifold`), `src/face_normal.rs`, `src/sector_face.rs`, `src/sector_shape.rs`, `src/coherence.rs` |
| Coincidence census, at rest | `src/census.rs` (`census_and_certify`: the BVH pre-filter `Candidates`, the sweeps, the backing rungs, the confirm pass, the cross-solid backstop) |
| Contact vocabulary | `src/contact.rs` (`ContactClass`, `DeclaredContact`, `ContactVerdict`, `ContactRefusal`, `ContactFinding`, `CONTACT_RECOURSE`, `FIT_DEFERRAL`) |
| Contact verification (Door 1) | `src/boolean/contact_verify.rs` (`contact_pair_verdict`), `src/boolean/carrier_eq.rs` (the kind-generalized carrier ladder), `src/boolean/plane_eq.rs` (its planar arm) |
| Chart-region overlap (Door 2) | `src/chart_region.rs` (`chart_region_overlap`, `declared_pair_overlap`, `world_carrier`, `cylinder_pair_overlap`, `interior_witness`), `src/chart.rs`, `src/chart_iso.rs`, `src/pcurves.rs` |
| Chart-boundary description | `src/chart_bound.rs` (`ChartBound`, `ChartLoop`, `ChartEdge`, `assembled`, `metred`, `MetredBound::certifies_outside`, `MetredRect`), `src/pcurves.rs` (`chart_boundary`) |
| Plane splitting | `src/splitting/` (`classify`, `neighborhood`, `rules`, `insert`, `order`, `join`, `finish`, `section`, `containment`), `src/chord_join.rs`, `src/ring_path.rs`, `src/null.rs` |
| Booleans | `src/boolean/mod.rs` (reduction, classification, `ReductionContacts`, `BooleanInputs`, the glue), `records.rs` (`ContactRecords`, `Contact`), `reduce.rs`, `vtxfac.rs`, `sectors.rs`, `recl.rs`, `tables.rs`, `insert.rs`, `join.rs`, `finish.rs`, `zip.rs`, `ops.rs`, `carrier_pair.rs`, `combine.rs`, `voids.rs`, `boxes.rs`, `contain.rs`, `solid_contain.rs`, `surface_group.rs`, `rim_wedge.rs` |
| Instances, separation | `src/instance.rs` (disjoint graft), `src/separation.rs` (certified no-touch), `src/transform.rs` (rigid placement) |
| Shell and offset surgery | `src/shell.rs`, `src/replace_face.rs`, `src/offset_general.rs`, `src/offset_together.rs`, `src/offset_axial.rs`, `src/merge_faces.rs` — decisions in `crates/geom-brep/README.md` (OFFSET-DESIGN). `shell` and `shell_open` take a finished body (`AtRestBody`): their thickness is measured into the solid's material. `replace_face_offset`, `replace_faces_offset`, `offset_surfaces_together`, `offset_planes_together` and `offset_charts_together` take construction state (`Body`), tier 2 in and tier 2 out: their argument is a distance along each chart's stored normal, which no face's sense decides, and `shell` runs them over a clone mid-construction. Their result becomes finished only through `AtRestBody::validate`, where an inside-out result refuses `NegativeVolume`. A move that carries a face through its neighbours inverts the body too, and refuses there on the rings it leaves outside the moved face (`RingOutsideOuter`) |
| Queries, flush detection, read-back | `src/query.rs` (the EXACT/DECIDED atoms and `rim_of`; `crates/verbs/README.md` S1), `src/flush.rs`, `src/readback.rs`, `src/props.rs` (mass properties, `AtRestPolicy`), `src/ray_walk.rs` |

The census's sweeps and backstop examine only pairs whose padded boxes
overlap. The pad is at least escalate + 2·zero, so a pair the boxes
prove apart is decided apart by the box answer alone: no carrier-stage
escalation or refusal is raised about it.

## Contact census and declared contact (the CONTACT-DESIGN clauses, C1–C8)

These ids are the contact-design clauses; the curved-geometry design in
`crates/geom-brep/README.md` has its own C-numbers, always cited
qualified (`CURVED-DESIGN C7`).

Setting: two bodies whose boundaries touch, at rest or as boolean
operands. Three objects carry the design:

- the **census** (tier 3′, `validate_pseudomanifold`) finds every
  cross-entity coincidence;
- a **declaration** is recipe data asserting a contact class on a named
  face pair;
- a **decision** (`Coincidence`, `crate::coincidence`) is a
  coincidence an operation decided from values: which two of its
  inputs' cells, what relation, where, and the margin read;
- a **record** (`ContactRecords`, one list of unordered pairs of the
  body's cells) is a touch a result body carries, citing the decisions that back it (`Cites`):
  rows of the producing operation's own coincidences
  (`Backing::Decided`), or a record of an input it carries in
  (`Backing::Carried`, which cites its own). A record cannot be built
  without one (D1 (ii)).

The coincidence ladder (D10): at the Boolean, a margin decided Zero
glues, and the decision is recorded where the two faces meet, at a
point, an edge or an area (`crate::coincidence`), for the document's
`unproven-coincidence` lint; a margin in band refuses, with nothing to
bridge it; structure is decided at the document's door, not here. At rest,
certification runs both ways: a record with no witness is
`StaleContactDeclaration`, and a touch no record backs is
`UnrecordedContact` inside one group of the body (one op's result, one
copy) — and between two groups (two copies, two imported instances)
the census records it, a row of its own Zero and a record citing it,
which its next pass confirms like any other (`record_at_rest`, D1
(iii)).

**C1 — Classification of the pair germ.** At p ∈ ∂A ∩ ∂B interior to a
face of each, write each boundary as a graph h_X over its tangent plane
along the outward normal n_X (material below) and II_X := Hess h_X(0).
The classes:

- *transverse crossing* — n_A, n_B independent; interference at rest;
- *aligned tangency* — n_A = n_B; always local material overlap, so
  containment or interference, never contact;
- *opposed tangency* — n_A = −n_B, with separation s = h_B − h_A and
  relative form II_rel = Hess s = −(II_A + II_B), sub-classed as
  *point touch* (II_rel ≻ 0), *curve touch* (rank-1 kernel, s ≡ 0 along
  a witnessed curve, κ_rel the positive eigenvalue), *conformal* (s ≡ 0
  on a patch), *crossing touch* (indefinite; interference) and
  *degenerate residue* (in-band κ_min with distinct carriers; escalates);
- *interference* — the regional class.

Every class boundary is a trilean at a stated order (1: normal
independence; 2: sign/rank of II_rel; ∞: conformality, decided
structurally); nothing is classified by proximity. The second-order arm
in code is the transverse κ_rel of the jet schedule
(`contact_verify::tangent_pair_relation`, `sectors.rs`'s second-order
lump). No test of II_rel's indefiniteness over all tangent directions
exists, so a tangency indefinite only off the sampled direction is
missed, not bridged.

**C2 — Representation boundary.** *Identity lemma:* for the analytic
kinds, two surfaces agreeing on an open patch agree as loci, so every
true conformal contact is same-carrier contact. For piecewise-rational
carriers the guarantee is per knot-span, and a span-partial coincidence
escalates. Conformality is decided by the carrier ladder's margins over
the consumed extent — Zero is one carrier, in band escalates unless a
declaration bridges it — never by sampled proximity. Identity is at the
locus level: two descriptions of one locus may differ as charts
(`u_ref`, seam), which C3's chart authorities answer for.

Face-pair procedure: exclusion, structural rung, declared rung, definite
separation/crossing by geometry, in-band ⇒ escalate. The exclusion step
is `census::sweep_cross_solid_backstop`. A cross-solid pair with a curved
side clears only

- on a definitely-positive separation margin between certified reach boxes
  (`face_reach_in`), read along each world axis and then along the
  directions that turn with the pair: the axis between the two faces'
  anchors, and either face's normal where it is planar
  (`boolean::separating::apart`), or
- on a definitely-positive gap in depth (the distance from an axis
  line, a centre or a circle that either face's carrier or a conic on
  its boundary proposes) between one face's least depth and the
  other's greatest, each bounded through rings and segments that cover
  the face or its convex hull (`census::depth_gap`): the nested walls,
  bores and plugs no direction separates, or
- where the pair meets, by the touch analysis, which reads every curved
  face of a touch's star through that box (its corners' signed distances
  from the candidate plane): a box on its side certifies the Rest, and
  anything else refuses — it never decides a crossing;

and refuses `CensusUndecidable` otherwise. Same-solid distinct-key curved
pairs are not detected; they are their constructor's obligation.

A declaration licenses a coincidence, never a side. Every meeting between
two solids — found undeclared by a sweep, named by a record, or backed by
a declared face pair — is read by the one touch analysis, and the pair
clears only when each meeting reads Rest. A record's certificate licenses
only what it checked (a patch record's opposed conformal interface; a
vertex record's coincidence), so no record excuses a meeting or a face
from being read.

Refusals are typed with `CONTACT_RECOURSE`: declare the class or move
the geometry. There is no tolerance arm, since ε cannot supply intent.
Invariant: no flag, mode or setting moves a glue — the Boolean glues
what its margin decides Zero under every setting, and the lint reads
what it glued.

**C3 — Record granularities.** Every granularity cites its backing
(the preamble's decision); a vertex record cites the vertex identity
the reduction decided (`DecisionSite::VertexFusion`), and a pair a
chain of such identities implies cites every one on each shortest
chain.

A record's kind is its two cells' dimensions; the face-level kinds
are edge–face and face–face pairs, and the census reads which touch a
pair is from the two cells' carriers:

- A **curve touch** is certified by the jet schedule along its locus
  (coincidence within ε, normal opposition within ε·κ_rel, κ_rel
  definitely positive, hull bounds between samples). For an edge lying
  in a face the locus is the edge's carrier; for a face–face pair on two
  carriers the census derives it from the carriers, on the closed-form
  lane the deciding operation used, and refuses typed where no such lane
  exists. Its endpoints are bounded by vertex records or the locus's
  closure; an unbacked bound is `UnrecordedContact`.
- A **point touch** is a pair of any two cells whose interiors meet at
  isolated points without crossing (a circle on a plane, a sphere on a
  plane, two circles kissing). For a cell on a face, or two spheres,
  the census finds each point on the closed-form lane of the two
  carriers, refusing typed where none exists, and certifies there
  coincidence within ε, the point strictly inside both cells, and
  κ_rel definitely positive; in band escalates. Two edges kissing,
  one of them curved, are found by the curved-edge lane's bisection
  search over both spans, which certifies a point on both edges'
  interiors.
  Whether the solids cross at the point is the touch analysis's
  reading of the faces around it.
- A **patch** is a face–face pair on one carrier, backed by a
  `SameOpposite` decision (for a mate's patch, the at-rest
  census's `CensusAtRest` row; the mate places and never checks):
  carrier identity by the carrier ladder (a margin
  decided Zero, or a declaration bridging one in band), senses opposed
  (aligned coincidence is contradicted), and definitely-positive trim
  overlap in a shared chart — exact on the planar trim inventory
  (`chart_region.rs`), typed elsewhere (`NonPlanarTrim`, `ArmUnbounded`,
  `SeamBranch`). Empty overlap ⇒ stale; in-band ⇒ escalate.

The shared chart comes from one of three authorities, tried in fixed
order (`declared_pair_overlap`):

1. one chart (`declared_chart`: one key, or two bit-identical
   descriptions);
2. for a declared **planar** pair, the shared world carrier: one plane
   description taken as the representative frame. This is legitimate by
   the frame-invariance lemma at `world_carrier` (both chart maps are
   isometries, so every quantity the area machinery consumes is
   Euclidean-invariant), and is gated by `carrier_agreement`, which
   meters the descriptions' disagreement over the pair's own boundary
   vertices;
3. for a declared **cylinder** pair, the certified everywhere-within-ε
   enclosure (`cylinder_pair_overlap`): one description's trims carried
   across the exact affine chart relation, the angle folded to one period
   through `periodic_branch`, gated by the `chart_region_cyl_*` rows.

Every other cross-description declared pair refuses `ChartDivergence`.
The claim earned is *certified everywhere within ε*, never exact
(`Ok(Zero)` means `|m| ≤ zero`). Invariant: certification strength
equals its skeleton; a contact of order k > 1 has no record type and
refuses. Area sampling is rejected (it can miss a trim hole).

**Continuation.** Two faces, one from each operand, on one carrier with
their senses ALIGNED are a *continuation*: one surface carried on,
whether the two abut along a boundary curve or overlap on a patch. The
kernel derives exactly which.

A continuation is not a contact and not a `ContactClass`: a mate (and every
record and census reading) speaks `ContactClass`, and a boolean glues a
pair as a `GlueClass`, which is `Rest | Continuation | Tangent`. A mate
cannot state a continuation, and at rest two flush walls carry nothing
to verify. A glued `Tangent` is the tangency of two distinct carriers
along a locus the witness lane certifies, and says nothing of the
materials: whether the pair is a contact (a void crescent between the
sheets), an overlap (a crescent both materials fill), a seam or a nested
touch is read where it is consumed, by the lump and by the wedge of the
result's junction (C7). `ContactClass::Tangent`, the record's and the
mate's class, keeps its material claim, and a shared crescent
contradicts it.

Verification is `Rest`'s carrier rung with the sense bit reversed:
carrier non-contradiction through the kind ladder, senses aligned as an
exact bit. Opposed senses contradict it, as aligned senses contradict
`Rest` at every door. A union merges a continuation its margin decides:
the merge stage reads the same ladder (D10), keeping the
description of the group's arena-first face that lies in no other
member's hole. That is ordinarily operand A's, and B's where A's face
plugs a hole of B's.

**Seam.** A *seam* is a junction of the result whose material wedge is
π: two faces on distinct carriers joining G1 along a curve. It is read
on the result, not glued on the operands: the boolean glues the two
faces' tangency (`Tangent`), and the op and the membership decide which
edge the result carries on the locus. One tangent pair can leave a seam
under one op, a knife edge under another and no edge under a third.
Rim routing by material wedge (C7) decides the rim, and the zip mints a
seam carrying `TangentIntersection`. A G1 joint authored inside one
profile is the structural form of the same fact.

**The crossing layer's one-sided cover.** An edge touching the other
operand's carrier — a root at an end, its interior uncertified — is
recorded at its endpoints only when the edge's parent carrier is
certified to lie in one closed side of that carrier.

*Sources.* That certificate comes from exactly these, on either operand,
to a face verified as one carrier with the target: a verified `Rest`
(residual ≡ 0), a verified continuation, a glued `Tangent` (along a line
from the witness lane, a global side by the lane's own contract; along a
rim, as below), or a structural tangency (an edge described
`TangentIntersection`).

*Which tangencies count.* A `Tangent` along a rim and a structural
tangency are tangencies along a curve, and the condition asks for a
certified side, not a local touch, so they count only where such a
tangency is a global side for the two carriers' kinds
(`boolean::tangency_certifies_side`). A rim tangency counts for
plane–cylinder, cylinder–cylinder, cylinder–sphere,
plane–torus and sphere–torus; a structural tangency only for
plane–cylinder. A torus with a cylinder or with a torus does not count.
The exclusion is conservative because the condition is read per pair of
KINDS: a cylinder coaxial with a torus (radius `R ± r`, tangent along an
equator) does lie on one side of it, but a straight tube leaving a
torus's end meridian G1 crosses its continuation, and the kinds cannot
tell the two apart.

*Reading the side.* Which side the certificate holds is read off the
parent face, and an endpoint off the carrier is eventless only on that
side. It is never read from values: a root verdict of "tangent" is a
band decision, so a graze within the band refuses, and an exact tangency
is reached only through structure or a declaration.

*Touches inside the edge.* A covered LINE touching a cylinder or a
sphere inside the edge is read again on the edge's fragments once both
sweep directions have run: where the other operand has a vertex at the
touch, whether it builds does not depend on which operand is A, and with
no vertex there it refuses in both orders. A covered arc, and a covered
line against a torus or a cone, keep the refusal where the touch is inside the
edge: their residual is not convex, so a fragment's ends need not see
every touch.

*Edges lying on the carrier.* An edge decided to lie identically on the
other operand's carrier is an ON event under the reduction's trilean
(exactly-on ⇒ ON, in-band ⇒ escalate), as an edge lying in a partner's
plane is — provided every surface of a face it bounds is decided
distinct from that carrier by the carrier ladder. Where the ladder does
not decide a parent distinct, the cosurface question keeps its door.

**C5 — The signed gap.** For a declared pair on same-kind carriers with a
shared mating frame, g is the carrier-relative signed offset:

- parallel planes: the material separation along the outer face's
  outward normal;
- concentric spheres: R − r − ‖Δc‖;
- coaxial cylinders: r_b − r_p − d (skew axes refuse).

**g > 0 clearance, g = 0 contact, g < 0 interference.** The census
classes are the strata of g's zero set: g = 0 with structural frame
sharing is `Rest`; g = 0 with an offset frame is an internal point/curve
touch. So the conformal limit is reached by structure, never by g
drifting to zero. Under structural frame sharing g is linear in the
radii; with independent frames it carries the norm kink ‖Δc‖ at Δc = 0
(Clarke subdifferential the closed unit ball; the `Dual<Interval>`
straddle-hull treatment). Built as the document layer's
`MeasurePrimitive::Gap { outer, inner }`
(`crates/editor-core/src/measure.rs`, evaluated in
`crates/editor-core/src/eval/measure.rs`), argument order the mating
role, authored rather than inferred.

**C6 — Interference fits.** `Fit { gap: g₀ }` asserts a nonzero nominal
gap; g₀ = 0 is rejected (zero gap is `Rest`). A verified interference fit
makes the disjointness/containment/extent gates skip the pair as a
*recorded* verdict naming the declaration; assembly mass properties
refuse by default, with an explicit opt-in subtracting closed-form
overlap volumes; booleans are unchanged; STEP export drops the
declaration.

Not implemented: the variant lands with its first consumer. The
backstop's containment arm decides the nested-instance class by a
material test — a nested placement sharing no material clears; an
instance with a vertex inside another's material is
`ValidationError::InstanceInterference` — and no declaration admits an
interference. Between two copies at rest the editor's gate reads that
verdict, a pierce and a same-side crossing as interference evidence
(ASSEMBLY A5 *Interference.*), quieted by a one-sided `Gap` assertion at
the overlap, not by a declaration. Invariant: inside one body an interference is always a
typed error, and at rest between two copies it is always a finding
(ASSEMBLY A5 *Interference.*), loud unless an assertion at its site
quiets it; no blanket "disable interference checking" exists.

**C7 — The join lane.** At the curved coplanar-lump sites (`vtxfac.rs`,
`recl.rs`) a tangent pair the witness lane decides (the
glue door, `boolean/glue.rs`), whichever side its materials lie on, or a
`Rest` its margin decides, descends to second order (`sectors.rs`: the
sector's relative transverse curvature signed against the other face's
outward normal, the pair's sense bit bridging an exact zero). The join
discards each side of a conformal contact whole, on any carrier the
ladder certifies, and its finish fuses the two solids along the seam
once, so at full engagement union volume is the operands' sum.

A rim with a determinate G1 jet carries `TangentIntersection`. Rim
routing reads the material wedge of a junction, an edge of the result
that both tangent faces bound (`docs/MATE-7-TANGENCY-DESIGN.md`): π ⇒
smooth seam; 0/2π ⇒ the cusp family. A wedge-0 junction along a line
locus builds through the ON-edge lane and passes tier 3 on its jet
(κ_rel determinate); `BooleanError::RimCuspArmUnbuilt` remains where no
locus arm exists. Two operands whose materials overlap beside a tangent
locus (an overlapping tangency) leave a π junction under ∖, a wedge-0
junction under ∩, and none under ∪, where the locus is interior. Two
operands that touch only along a tangent line leave no junction: each
keeps its own edge on the line, the membership decides every op, ∖
returns the operand, ∩ is empty, and ∪ is two solids (D1's pieces) whose
touch is recorded as any pair whose interiors meet — an edge–edge record
of the two edges on the line, citing the glue's tangency decision and
confirmed by reconstruction from its bounding vertex records. Where the
line runs through a face's interior the touch is a face–face curve
touch, recorded the same way and bounded by the records of the cells
its ends lie in; where the two sides are one piece (the operands also
overlap elsewhere, or a later op joins them), the result is one solid
pinched along the line, carrying the same record. A pinch is a record,
never a junction.

The same substrate is the at-rest door: `validate_pseudomanifold` with
mate declarations landed in `ContactRecords`, no boolean, no zip.

**C8 — Ratification boundary.** Refusals are typed per class and retire
per table arm, never wholesale; `CensusUnsupported` and
`CurvedBooleanUnsupported` name the C4 recourse vocabulary. A tangent
pair the witness lane has a locus for is glued, so it never meets
`CurvedBooleanUnsupported`, which remains the refusal outside the lane. Still open:
NURBS↔analytic same-locus recognition; kinematics (records are
geometric; mate solving is not here).

## The body, its operators and its tiers

How `topo` keeps DESIGN `[topology]`. This is the text DESIGN held
before the rules and the mechanics were split (2026-10-17), verbatim
except for its profile, sweep and parameterization paragraphs, which
went to those crates' pages, and the wired-arm list, which went to
`docs/KERNEL-VERBS.md`. Bracketed ids (`[topology]`, `[coincidence]` …) are `docs/DESIGN.md`'s clauses.

- Topology entities (`Solid / Shell / Face / Loop / HalfEdge / Edge /
  Vertex`) live in generational arenas and reference each other by
  typed IDs — never pointers. A B-rep is a plain value: cheaply
  cloneable, serializable, diffable, validatable. Realized as Mäntylä's
  half-edge structure in typed arenas: an `Edge` is two antiparallel
  half-edges, the mate computed and never stored; the empty loop is a
  typed `LoopBoundary::Empty | Cycle`, so every half-edge field is
  non-optional and the sole `Option` in topology is the vertex's
  emanating half-edge. A face's outer loop is excluded from its ring
  list (`outer ∉ rings`), so rings coincide with the Euler–Poincaré
  r-term.
- **Manifold solids only.** Non-manifold (radial-edge) topology is added
  only if sheet/wire bodies demand it.
- Topology is built **exclusively through Euler operators**: a small
  closed set of primitives that provably preserve the Euler–Poincaré
  invariant. Each operator debug-asserts its postcondition — a per-call
  instance of the soundness theorem, never a semantic gate on
  intermediate states. The postcondition has two halves and they are
  paid at different rates: the operator's declared arena delta is O(1)
  and is checked at **every call**, while the whole-body tier-1
  re-derivation is O(body) and is checked **once per public door**, at
  the door's end, over the state the caller will see
  (`work/perf/d1-per-op-tier1-sweep-price`, Ev, PR 2305). An operator a
  consumer calls directly is itself a door and sweeps there; one inside
  a composing door's surgery scope leaves the sweep to that door. The
  guarantee is unchanged — every public mutation path preserves tier 1,
  checked at every observable boundary — and an opt-in
  `topo/per-op-postcondition` feature restores the per-operator sweep
  to name the operator behind a door-level failure. **One class needs
  the scalpel rather than merely benefiting from it**: a corruption an
  operator introduces and a later operator in the same door repairs is
  invisible at the door, because the state the door hands back is
  sound. The door-level check is a claim about that state, not about
  every state the door passed through, and the ruling's "caught one
  door later" is exactly that trade. "Exclusively" is
  realized: the operator set is the only public construction path; raw
  insertion is crate-internal test scaffolding.
- **Touching via distinct entities is a typed success** validating at
  tier 3′ ([topology]'s representability boundary). Silent splitting into separate manifold bodies is rejected as
  inexplicit; any future split behavior is an explicit ratified
  operation the user invokes, never a fallback.

**Topology conventions.**
One rule, from which everything else is a
corollary: walking any loop in `next` order with the face's outward
normal toward the viewer, the face interior lies to the **left** of
every half-edge. Corollaries: outer loops run counterclockwise viewed
from outside and rings clockwise; an edge's two half-edges are
antiparallel (`end(he) = start(mate(he))`); `Edge::he_plus` defines the
edge's intrinsic direction, and curve geometry MUST agree — increasing
curve parameter runs from `start(he_plus)` to `end(he_plus)`, pcurves
and per-face traversal senses derived from that, never stored as peers;
the vertex-orbit step `next(mate(he))` visits a vertex's outgoing
half-edges **clockwise** viewed from outside. Transcription hazard:
GWB/Mäntylä's diagrams orient face boundaries clockwise viewed from
outside — mirrored relative to us — so figures, argument orders and
traversal idioms from the book are rederived from the interior-left
rule and pinned by construction tests, never transcribed. The normative
derivations live in `crates/topo/src/entity.rs`.

**Face orientation sense.** A face carries `Face::sense: bool`: `true`
iff the face's material side agrees with its surface's chart normal, so
the outward normal at a point is `n(u, v)` where `sense` is `true` and
`−n(u, v)` where it is `false`. The analytic
chart normals admit no reversal by reparameterization (cylinder, cone
and torus normals are odd in the radius; the sphere's is even and
outward under the `radius > 0` convention — a negative-radius sphere is
rejected as a representation), so the bit on the face is the
representation of reversal. Normative consequences:
- The interior-left rule is stated against the sense-signed chart
  normal.
- Orientation reversal is **exact structure**, never a numeric decide:
  `revert` flips `sense` on every face carried by a non-plane surface,
  negates the stored normal of `Plane`-carried faces, and inverts every
  joint element, moving it onto its source predecessor (a joint's
  element read from the other side is its inverse — `topo`'s
  `JointElement`); no loop's anchor moves. The two normal encodings are
  exclusive by surface kind, so every outward normal is negated exactly
  once, inverting an element is exact integer arithmetic, and
  `revert ∘ revert` is bit-identical at every scalar backend.
- A face's `sense` is decided where its chart is decided. An Euler
  operator minting a face on its parent's chart (`same_chart`: one
  key, or keys sharing one payload) derives the bit from its own
  topology: a `mef` fragment is a piece of the parent's region and
  takes the parent's bit; the ring `mfkrh` promotes was wound
  clockwise about the parent's outward normal and now bounds its face
  counter-clockwise, so that face takes the parent's bit negated
  (`kfmrh` demotes the other way). A face minted or re-charted onto
  any other chart carries the bit its caller states beside the
  surface (`FaceSurface::New`, `Shared`); a stated bit that
  contradicts the derived one on the parent's chart is refused, typed,
  before mutating. Never a numeric compare, and never a default. Where
  the parent bounds no region yet, its bit and so the derived one are
  provisional, and the constructor states the honest bit when it
  charts the face.
- Every "which way is out" consumer (tier gates, mass-properties flux,
  boolean classification, tessellation and export winding) reads the
  signed normal, or documents in place why it is sense-invariant.
- The bit is exactly STEP's `advanced_face.same_sense`, so the exporter
  consumes it rather than deriving it. Bodies are not serialized ([determinism]),
  so persistence is unaffected.

**The operator set.** `[topology.operators]`
Ten operators in five make/kill pairs —
`mvfs`/`kvfs`, `mev`/`kev`, `mef`/`kef`, `kemr`/`mekr`, `kfmrh`/`mfkrh`
— plus the `ring_move` reparenting helper, deliberately **not** an
Euler operator (`mef` does not reclassify rings, after GWB's `ringmv`).
Addressing is by half-edge key plus per-op **site enums** whose
variants are the degenerate cases (`MevSite::{Fan, Lone}`): degenerate
sites live in the argument types, not behind null checks. GWB's id-scan
layer is dropped; arena keys are the stable O(1) handles. The uniform
per-op contract: **atomic** (typed-error preconditions fully resolve
before an infallible mutation phase; a failed op consumes no key
slots), **deterministic minting order** (documented per op — [determinism]
lineage replay), and a **debug-asserted postcondition** — the declared
arena delta at every call, the whole-body tier-1 re-derivation once per
public door ([topology]).
Association convention: **the given/first half-edge's side is the new
or affected thing** — `mef`'s `he1` side becomes the new face's outer
loop, `kemr`'s `he1` side becomes the ring, `kef` kills the given
half-edge's face, `kev` the vertex it points at. Cross-shell `kfmrh` is
the shell-fusion form (same solid), `mfkrh` its inverse; `ring_move`
reparents only within one shell (`EulerOpError::CrossShell`).

**Validity tiers.** `[topology.tiers]`

1. **Tier 1 "euler-valid"** — the structural invariant of every
   Euler-reachable state, construction scaffolding included (empty
   loops, struts, self-loop edges, laminae are mandatory
   intermediates); what each operator debug-asserts. The checklist:
   referential integrity across all arenas (orphan geometry is an
   error); half-edge chain coherence; mate involution/antiparallelism;
   vertex anchoring (every vertex is referenced by ≥ 1 half-edge XOR is
   the lone vertex of exactly one Empty loop); vertex-orbit closure
   (manifoldness — watertightness is structural in the half-edge
   form); the ownership/back-pointer partition; shell-partition/
   edge-adjacency coherence; arity floors; bidirectional [identity] provenance;
   and the **component-aware per-shell Euler–Poincaré**: per connected
   component of a shell's incidence complex, v − e + f − r = 2(1 − g)
   with g a non-negative integer, summing per shell to 2(c − Σgᵢ) over
   its c components. The naive per-body form is wrong for tier-1
   bodies — `mfkrh` on a detached ring disconnects a shell's surface
   while a single shell entity remains.
2. **Tier 2 "closed solid"** (`validate_closed`) — tier 1 plus: no
   empty loops, no valence-1 vertices, and c = 1 per shell (the third
   ban is independent: a promoted detached cycle ring disconnects a
   shell with neither an empty loop nor a strut).
3. **Tier 3 "geometric"** — [tolerance.residuals] residual certification, plus the
   **material wedge-angle predicate**: at every edge the material wedge
   ∈ (0, 2π), bounded away from the ends by θ = ε/r; wedge = π is the
   legal smooth-seam case; and the ends carry a **second-order arm**:
   wedge = 0 (a cusp) and wedge = 2π (a knife slit, an isolated cusp's
   `revert` image — legal together or not at all) are legal iff the tangency is
   **jet-determinate**: quadratic transverse separation with κ_rel
   bounded away from zero — `TangentIntersection`'s own margin, so the
   cusp edge's honest description IS `TangentIntersection` wherever the
   tangent certificate's lane reaches. The ends are derived from the
   body exactly as the π seam is: a cusp and a smooth seam are the same
   tangent junction on one shared edge, told apart only by the sign of
   the two faces' outward normals, which the body already certifies.
   The shared edge is the structural record. A tangency is constructed
   (the profile's cusp joint, a tangency construction, [coincidence]) or decided
   Zero by its margin where an op meets it, and a tangency decided from
   values is recorded and checked by the `unproven-coincidence` lint
   like any coincidence. In-band κ_rel escalates; osculation
   refuses (`LaminaWedge`). The
   arm admits no laminae, so zero-volume bodies stay geometric defects.
   Where more than two sheets meet on a line, the material decides the
   edges, as the manifold boundary of the material rather than of its
   surface: each creased material sector between two consecutive
   sheets is one edge, bounding that wedge alone; a sector of exactly π
   is a face continuing, with no edge there; and the material sectors
   on one line are disjoint, so no face passes through an edge's
   material wedge. So a doubled cusp (two material wedges on one
   tangent line) and a reflex edge lying in one of its own faces are
   two coincident distinct edges, each classifying separately; an edge
   touching a face from outside its wedge stays one edge, in contact
   with that face; and where no edge lies on the line, the wedges are a
   touch between faces, recorded like any touch, and no op cuts edges
   to make a junction of it. Where the line ends inside a face, that
   face carries its edges as a zero-area ring, which bounds no hole and
   whose winding is exempt; where it reaches the face's boundary, as a
   slit of that loop. The complement pairs the sheets differently, so
   a door that complements material across such a line re-pairs there.
   Tier 3′ reads every touch for sides, the coincident edges on one
   line included: touching cells' material lies on opposite sides of
   each other. Consumers with no wedge-0/2π answer (fillet, offset,
   mesh sizing, sector classification), and none for a body's
   curve-touch records, refuse typed at the consumer.
   Also at tier 3: **maximal edges** (a joinable vertex at rest is a
   construction that stopped half-way, `JoinableVertexAtRest`; a
   vertex whose regularity reading lands in the sliver band, so that
   whether it is joinable is undecided, refuses too, with the
   tighten-the-tolerance recourse: a size the run's ε cannot tell from
   a pole or apex), **prefer-intrinsic enforcement** ([edges]) and the
   **orientation invariant** (a solid's faces leave infinity outside
   it, read by the reader of a shell's role, [coincidence.booleans]: a ray
   aimed at one of its faces, the certified signed volume only where no
   face offers an aim; definitely inside-out ⇒ invalid, and a solid
   whose side no reading decides refuses — an orientation probe, not a
   thinness gate, at every door). Laminae live here, not at
   tier 2: two faces glued along their whole shared boundary is a
   two-hemisphere ball's incidence structure, so a zero-volume lamina
   is a geometric defect, not a topological one. Global
   self-intersection / minimum clearance is the interval clearance
   engine's: its body-level half — cell subdivision over
   `Body<Interval>` — lives in `editor-core` today and moves into
   `topo` (SHELL-3, ruled at #1737), with the
   parameter-box outer half above it in `editor-core`, so a verb that
   must certify a boundary embedded (`shell`'s cavity clone) runs the
   same engine at a certifying scalar and refuses typed at the door.

   A **finished body** (`AtRestBody`) passes tier 3′ against its own
   contact records: none, for a body that touches nothing, where the
   census must find no coincidence at all. Every door that returns or
   consumes one pays that gate once, at the door that built it.
   The Boolean door's result meets tier 3 without the census: its
   contact records, each citing the coincidence that backs it, ride
   beside it (`BooleanBody::contacts`), and the
   census over them is parked on CONTACT's cross-solid curved lane
   (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`).
   Construction state (tier 1, or tier 2 without geometric
   certification) is what Euler operators hand back, and becomes a
   finished body only through the at-rest gate.
4. **Tier 3′ "pseudomanifold"** (`validate_pseudomanifold`) — the
   honest at-rest tier for boolean results that *touch*: contacts
   limited to entirely-coincident-but-distinct edges, edge-on-face,
   vertex-on-face, vertex-on-edge/vertex — touching allowed, proper
   self-intersection not. Composition: tier 3's local battery
   verbatim (shared extraction) plus a **global coincidence census**
   plus **two-directional contact-record certification**:
   - The census is exact on the planar inventory and admits every
     carrier kind: same-key opposed-sense curved pairs certify through
     the conformal arm, face–face records through the patch
     certifier on one carrier or two planes and the jet schedule
     along the closed-form tangent line on two other carriers,
     cross-solid pairs with a curved
     side in reach refuse as undecidable, and same-solid distinct-key
     curved pairs stay undetected until C9/C6 (`topo::census` module
     docs). A record or candidate outside a certifier's lane refuses
     typed `CensusUnsupported`, never samples. Every comparison is a
     named trilean ([verdicts]); indeterminates surface as typed
     `CensusEscalated`, never a silent skip.
   - A census finding decides one number per pair of cells (two edges,
     an edge and a face, a vertex and a face, two faces), read between
     the cells themselves, never between the infinite lines or planes
     they lie on, and signed where a face gives a side. For cells that
     share nothing it is their minimum distance. Cells that share a
     vertex, a point key or an edge meet there by construction, and the
     question is whether they meet anywhere else: the number is their
     largest distance over the shorter cell, beyond what they share. For
     straight cells it falls at the shorter cell's far end, its length
     times the sine of the angle between them ([tolerance.epsilon]'s d = r·θ, with
     the cell's own extent as the arm). The verdict's three arms ([verdicts]) read it: a
     definite number is a legal configuration, however thin; one in the
     sliver band refuses; a Zero is a coincidence (the cells are one
     over their extent), which the op that made the body glues and
     records ([coincidence]), so a census Zero no record backs is that op's
     defect. Near what two cells share, every pair is within the band
     whatever its angle. That neighbourhood is not a coincidence, and
     its extent is not a number the census reads.
   - Certification runs **both directions and never scans-to-bless in
     either**: a census finding with no backing record is never
     silently blessed ((iii) below); a record with no geometric witness
     is stale and refuses typed. Structural sharing (same surface or
     point key) needs no record. Vertices an op knows or decides to be
     one point share one point key, whether they stay in one solid or
     lie in two: an op that cuts one vertex into copies hands each copy
     the original's point, as it hands a cut face's fragments one
     surface; vertices whose gap an op decided Zero share the key it
     keeps, and the op records that decision (a `VertexFusion` row,
     cited by the pair's vertex-vertex record) for the lint, not for
     the census. A Zero-gap vertex pair on two point keys in a finished
     body is its producer's defect.
   - A contact record is a pair of cells, one from each touching
     shell (or each side of a shell's pinch, whose two sides touch
     within one shell, recorded as between shells), whose interiors meet, plus its backing: a
     coincidence an op decided Zero, recorded ([coincidence]). The record is
     that pair and nothing else: an unordered pair of the body's own
     cells, in canonical order, in one list a citation addresses by
     position. It carries no operand label, no stored granularity and
     no witness. Its kind (vertex–vertex, vertex–edge, vertex–face,
     edge–edge, edge–face, face–face) is the two cells' dimensions, and
     whether it touches at a point, along a curve or over a patch is
     read from the two cells' geometry by the census (CONTACT-DESIGN
     C3), whose rungs back a subordinate vertex event. An operation's
     own working vocabulary (the boolean's operand-tagged cells) stays
     inside it and is dropped at its carry. Every pair whose interiors
     meet is recorded, an edge lying in a face or along an edge
     included; only how the census confirms a record depends on its
     geometry. A straight edge in a plane, or two collinear lines, is
     confirmed by reconstruction from its bounding vertex records
     (between two backed bounds, two lines sharing two points are one
     line; a missing bounding record is an unbacked contact, never
     inferred), a curved one as a curve touch; a crossing of two edge
     interiors has no bound, and its edge-edge record backs it. Every op carries the records through
     one lineage, which states for each cell what the op made of it:
     kept; substituted by a cell that holds its interior (a merge's
     face, a join's edge, a seam); divided, into its pieces and the
     cells the op minted between them, the division exhaustive or with
     its remainder removed; or removed. One carry, for the boolean and
     every surgery verb alike, lands a record whose two cells the op kept
     or substituted on their images by lineage, unread, their geometry
     unchanged and the record read where it was minted; it puts each
     pair drawn from a divided cell's images to the census's record
     confirm, and the record lands on every such pair whose interiors
     meet, each landing citing it. Two
     cells an op minted on a record's two cells and that meet at a point
     meet where the record puts them when one cut made both, as the
     split's one plane does, and the landing cites the record alone; an
     op that cuts with many surfaces (the boolean's tool) decides every
     such meeting at its door ([coincidence]), and the landing cites the decision
     beside the record; any other op refuses one. A record
     that lands on no pair, every pair read definitely, is consumed
     where one of its cells was removed or divided with its remainder
     removed, and is stale otherwise; a cell the lineage does not cover
     refuses. A cell that replaces another without holding its interior
     (a blend face for the edge it rounds) takes no record from it.
   - **Certification strength equals its skeleton**: a curve touch is
     certified at its jet samples plus hull bounds, a patch by
     definitely-positive region overlap in the shared chart, a
     vertex-granularity area contact via its corner/segment records.
     Nested-shell pure containment (a void, zero coincidences) is
     census-invisible and certifies — the voids story below, not a
     gap.

   **Touching is always recorded**: (i) operand coincidences are
   decided by margined verdicts ([coincidence]) — a Zero margin is a coincidence,
   and a sliver-band one is an escalated typed error, never contact;
   (ii) result-side touching arises only from those coincidences
   propagated through the op's lineage, from the op's own decision
   where two cells it minted on one meet at a point, or from an op's
   copies of one vertex at such a coincidence, which share its point; the result carries
   machine-checkable contact records (the ON-set survivors, carried
   across seam-zip/merge mints by a descendant map, never re-derived),
   each citing the coincidence ([coincidence]) that backs it, which the
   `unproven-coincidence` lint checks; (iii) a contact the
   census discovers that no record backs is never silently blessed:
   between copies at rest it is an `unproven-coincidence` finding
   ([coincidence]), and inside one op's result it is that op's defect, refused
   typed.

   **Validity class rides the result wrapper, never a mutable `Body`
   field**: a boolean result is `BooleanBody` — body + contacts — whose
   at-rest gate is `validate_pseudomanifold(&body, &contacts)` over its
   own contact list, empty or not; with the list empty, the census must
   find nothing.

   **Representability boundary**: pseudomanifold touching via
   *distinct* entities (two vertices at one point, two edges on one
   segment) is representable in the half-edge structure and is a typed
   *success* carrying its 3′ records. A single edge with >2 faces, or a
   wedge fan sharing one entity, assembles faces into something that is
   not a solid; the half-edge structure does not represent it.

**Structural conventions.**

- **Sweeps emit single-shell primary boundaries; every CAVITY is born
  through the shared void-insertion door.** A cavity's boundary is a
  disconnected interior shell, and its bookkeeping — orientation,
  census participation, containment evidence — has exactly one home:
  `topo::insert_void`, the door the boolean owns, callable without the
  SSI pipeline for provably-no-crossing cases with caller-certified
  containment. Four producers: boolean subtraction (its probe
  verdicts); `shell`'s sealed hollow (its offset margin, OFFSET-DESIGN
  O4); the full revolve of a holed profile, DEFINED as
  `revolve(outer) − revolve(hole-as-outer)` and executed through the
  degenerate no-crossing arm (the profile's validated 2-D margins); and
  a concave blend's fill sealing a blind hole's mouth, DEFINED by the
  blend's Boolean semantics (`S ∪ F`) and executed by its surgery (the
  mouth's cycle on the face the fill covers).
  Recipe-layer sugar may wrap any of these; the door stays the one
  birthplace. `UnsupportedToroid` is permanent: a [surfaces] ring-torus
  boundary — spindle tori have no representation.
- **∅, disjoint, and voids are typed results.** ∅ is a typed success
  value (`BooleanResult::Empty`), not an error. **A solid is one piece
  of material**: the `Outer` shell around it and the `Void` shells of
  the cavities in its material, which together bound that material and
  nothing else. A cavity belongs to the piece whose material surrounds
  it, so an island inside a cavity is a solid of its own, and a cavity
  inside that island is the island's. Pieces that only touch (at a
  corner, along an edge, or across a face that contact records hold
  apart) are distinct solids, because touching is between distinct
  entities (tier 3′). A piece that touches itself stays one solid. A
  body is any number of solids, so a disjoint union is a body of
  several solids. Booleans, `shell` and `split` take bodies and return
  bodies, and each sorts its result into solids, so every output is an
  operand. A product (the copies its world placements define, [placement]) is
  not a boolean operand: the editor refuses it, naming the explicit union
  of the copies that makes them one body
  (`crates/editor-core/ASSEMBLY.md`, A2). Which solids are one part stays recipe
  structure, never body state (`crates/editor-core/ASSEMBLY.md`, A2).
  The extrude/full-revolve
  hole asymmetry is structural: extruded holes are cap-to-cap tunnels
  (one shell, genus); full-revolve holes are cavities (a second shell,
  through the door); partial revolve is extrude-shaped and carries
  holes in its one shell.
- **Coincidence discipline in the reduction.** Every
  reduction/classification comparison is a trilean ([verdicts]): definitely-off
  ⇒ clean side, decided Zero ⇒ ON, in-band ⇒ escalated typed error (a
  genuine sliver: the operand pair is ill-conditioned at this ε). An
  ON verdict that only places topology is derived; one that would make
  pieces of one result touch (a split vertex whose orbit has two or
  more runs on one side) is a coincidence decided from values, built,
  recorded and reported by the `unproven-coincidence` lint unless it is
  structural ([coincidence]). No snapping beyond the Zero verdict: a merged face
  keeps one operand's description, and nothing in the sliver band is
  moved. Booleans on independently modeled nearly-touching bodies fail
  loudly rather than guess — the design thesis; the resolution is one
  construction or an explicit [import]-style repair/adoption op.
- **Maximal-faces precondition and the merge stage.** Booleans
  precondition no two adjacent coplanar faces (`NonMaximalFaces`); the
  explicit opt-in normalization op is `merge_coplanar_faces` (merging
  is never silent), and boolean *outputs* run it as a documented final
  stage of the op's contract — the seam zip manufactures coplanar
  pairs by construction; the recipe records one boolean node, not
  hidden healing. A sweep mints no same-key adjacency to merge: it
  builds one wall per run of profile pieces on one carrier
  (`crates/sweep/README.md`, "Walls"). Merge glues on a Zero verdict
  between the carriers ([coincidence]), never on bit-equality: a boolean merges a
  cross-operand coplanar pair its margins decide a continuation, records
  it for the `unproven-coincidence` lint, and keeps one description. A boolean whose output
  stage cannot glue a planar group its verdicts decided a continuation refuses
  the step with the merge's own typed reason, so every boolean output
  is a legal boolean operand; only a curved group's skip is recorded
  and shipped. Every op's output also has **maximal edges**: no
  *joinable* vertex, meaning valence 2 with two distinct edges on one
  structural carrier between the same two faces. Two edges are on one
  structural carrier when both lie on one locus of the faces' surface
  pair, or on one iso family of one surface's chart, and that locus is
  one regular curve at their shared vertex: the surfaces are transverse
  there (tangent arms one order up), or the chart is regular there.
  Distinct components of one locus never share a point, so the vertex
  decides the branch, and nothing is compared between the two edges. A
  pole, a cone's apex or any point where the locus is not one curve is
  never joinable; a reading of the vertex in the margin band refuses
  typed. A closed edge the join makes keeps one *conventional vertex*:
  a vertex whose only edge is that one closed edge, at both its ends.
  Its position, and the edge's parameter origin and witness that hang
  on it, are conventional data the kernel owns, like a seam's place.
  The vertex has no identity of its own. It mints no name a reference
  binds to; a reference or a pick there resolves to the edge, and a
  record, census reading or later cut at its point reads the edge's
  interior. A sweep builds one rim
  edge per run, as it builds one wall, and a boolean's output stage joins
  every joinable vertex after the merge, whatever drew it. A body is then
  the unique complex with maximal faces and maximal edges over its face
  partition, up to the position of each closed edge's conventional
  vertex, so a union's body does not depend on its member order, and
  the form is checked at rest on the result alone, with no history, by
  the one predicate the join takes a vertex by: every door that
  finishes a body, import included, ends with the join, and a vertex
  whose regularity reading lands in the sliver band refuses at rest,
  naming the tolerance below which it would be decided.
  Load-bearing dependency: `merge_coplanar_faces` **never fuses two
  vertices into one**. A contact record is a pair of cells, one from each
  touching shell (vertex, edge or face), whose interiors meet, plus its
  backing. A vertex resting on an edge's interior is the record
  (vertex, edge). The output stage deletes a vertex in two shapes, both
  recorded in `killed_vertices`:
  - the free end of a seam edge the glue left dangling inside the
    merged face, which goes together with its edge;
  - a joinable vertex, whose two edges become one.

  The join carries every record citing the three cells it replaces to
  the edge it makes, by substitution with its backing unchanged, so
  records are written by the op that changes their entities and never
  re-derived at the gate (F1). Tier 3′'s strict record-drop rule (a
  contact record whose vertex pair fused into one vertex is consumed and
  drops) is correct *because* nothing is fused. Any future fusing
  re-opens the record-carriage class.

## Until the intent refactor lands

These clauses describe declared contact as the code still has it: a
mate's `ContactClass`, its minted declaration and the declared rungs
that back census events. DESIGN `[coincidence]` and `[assertions]`
replace them, and INTENT stage 4 (a mate's contact class) and stage 5
(the at-rest check) delete them as they land. C2's and C3's mentions of
a declared rung and a declared pair belong to the same model.

**C4 — Declared contact as data.** A declaration names two faces by
stable name on the mate that declares it and asserts a `ContactClass`:

- `Rest` — same carrier, opposed senses, gap ≡ 0;
- `Tangent` — curve or point touch, non-crossing;
- `Fit { gap }` — carrier-parallel at a signed nominal gap; specified,
  not built (`FIT_DEFERRAL`).

A declaration is verified, never trusted. Each class states a list of
facts that must verify DEFINITE, the triggers that contradict it, and a
bridged residue; the declaration bridges only the residue
(`ContactVerdict::{Definite, Bridged}`; `ContactRefusal::{Contradicted,
Escalated, Undeclared, NotCertifiable}`).

- *`Rest`.* Must verify: carrier non-contradiction through the kind
  ladder (`carrier_eq`: plane, sphere, cylinder, torus), the pair read as
  one displacement over the consumed extent — position data at a pivot,
  plus the angular data levered from it to the extent's far reach, plus
  the radius differences; senses opposed as an exact bit; overlap
  definitely positive on C3's chart authority. Bridged only where that
  displacement's upper bound is in band at every consumed point.
  Contradicted by a consumed point (a face vertex, or the whole extent)
  definitely off the other carrier, by aligned senses, or by definite
  separation on the patch. Escalated between.
- *`Tangent`.* Must verify: the two surfaces' separation along the
  shared normal within ε at every sample (the touch), first-order
  tangency along the witnessed locus, and the witness within ε of each
  surface (C7's jet readings). Definite separation and definite
  crossing are read on the separation; a witness off a surface is a bad
  witness, not a gap. Contradicted by definite
  normal independence, definite crossing (as far as the sampled κ_rel
  sees it, C1), or definite separation. Bridged: in-band κ_rel
  *including exact zeros at isolated points* (a G1 tube chain's neutral
  meridians) — deliberately weaker than a jet certificate.

The two doors are coupled: Door 2 receives Door 1's verdict. The planar
interior-witness rung runs only on `Definite` (a precondition may not be
discharged by the claim under test), and the cylinder enclosure halves
its zero band on `Bridged`.

A boolean's face pairs are the glue its margins decide (`boolean::glue`),
or, at a step of an n-ary union's fold, the verdicts that union's pairwise
judgement decided (`BooleanInputs::given`, each pair made from a
`GlueVerdict` some boolean returned, which nothing ties to the operands
it is handed to); declarations live on mate
nodes alone (`crates/editor-core/ASSEMBLY.md`). Bodies carry only verified records,
in the `BooleanBody` wrapper, never persisted. Replay is scalar-generic;
an indeterminate verification at an interval scalar aborts.

Failures, all typed: `UnrecordedContact` (the census);
`ContactContradicted` (at use and
at rest); `ContinuationContradicted` (at use);
`StaleContactDeclaration`; `CensusEscalated`. Invariant: every definite
verdict wins over every declaration.

### At-rest census identity (CENSUS-REST-CLOSURE)

**Vertex-on-edge at rest.** The boolean lane refines every vertex-on-edge
event to vertex-vertex before records exist; at rest nothing refines. So
`sweep_vertex_edge`, the asymmetric arm of `ee_bound_backed` and the
edge-on-face bounds read two backings: the event's own `(vertex, edge)`
record (a vertex–edge pair, which a join writes from the v-v record whose
vertex it joined away), and `Declared::ve_face_backed`, a declared face
pair that holds the vertex on one boundary and names a face the edge
bounds. With neither, the event is a defect
(`CensusContact::VertexOnEdge` as `UnrecordedContact`). Both are written
before the census runs, never read off the geometry's agreement with
itself.

**World-carrier Door 2 for declared planar pairs.** Two instances of one
part placed apart carry descriptions whose bits differ, and a `Rest`
pair's charts mirror, so one chart is unreachable across instances.
Door 2 therefore answers a declared planar pair on the verified shared
carrier (C3's planar authority) with the one-body parity walk; the claim
is certified within ε at the pair's own extent, not exact.

**Cross-instance curved `Rest`.** The sanctioned shape is a certified
everywhere-within-ε overlap enclosure on the shared curved carrier. The
cylinder arm is built (C3); sphere, cone and torus keep the typed
divergence, with that shape recorded at the refusal site.

**Attribution at the assembly layer.** Each live `Rest` mate is minted as
a face–face record (a patch). Findings attribute to mates as Declined
or Refuted (`StaleContactDeclaration`), or to no mate; all-Declined is
the `Uncertified` frontier, and any other is a hard `AtRest` error
(`crates/editor-core/ASSEMBLY.md`). A touch between two copies that no
mate relates is no finding there: the gather's census records it
(`record_at_rest`).

### Crossing backability (MATE-4B-CROSSING)

A **crossing** here is the census event `EdgeEdgeCross`: two coplanar
boundary edges crossing at a point. A *backing rung* decides whether a
declared face pair answers for an event.

**Unified backing strength.** A declared pair answers exactly for its
verified interface: the overlap region, with material opposition being
what "interface" means for a crossing. Unifying down — crossings at
structural strength — would bless transverse interpenetration.

**The `EdgeEdgeCross` rung** (`census::ee_cross_backed`, planar-first) is
the first rung at that strength. A declared pair backs a crossing iff
all of:

- the crossing point lies on both carriers inside both closed trims, and
  both edges lie in both carriers (`pair_holds_point`,
  `pair_holds_edges`);
- the side test answers `OppositeSides`;
- the pair verifies through both doors in either frame order
  (`pair_region_verified`).

Confinement is by carrier, not incidence: nothing requires the crossing
edges to bound the declared faces.

**Three-valued side verdict** (`CrossingSideVerdict`). `OppositeSides`
backs. `SameSide` refuses, naming the verdict in the refusal's typed
`side` field; it is interpenetration evidence, which the assembly layer
reads as an interference between two copies at rest, so no bool may
stand there. `Undecided` escalates
`CensusEscalated`. The side is read by handing both faces' `Face::sense`
bits to `geom_brep::classify_material_pairing` after `classify_dihedral`
establishes the smooth precondition; the census is otherwise
sense-invariant.

**Grandfathered rungs.** `vv_face_backed`, `vf_face_backed`,
`ve_face_backed`, `ee_face_backed` (which stands in for a collinear
overlap's edge–edge record), and the face-pair arms of `ee_bound_backed`
confine by structural incidence only, so they can back an event outside
the pair's overlap region. Each migrates to the unified strength one at a time,
measured. `ef_bound_backed`'s face-pair arms hold the unified strength
(`Declared::vf_face_backed_confined`): a declared pair incident to the
bound backs it only where the pair passes Door 1 and Door 2 reaches
`PositiveArea`. Incidence puts the bound in the closed intersection of
the two regions, the licence the crossing rung reads; a curved pair
answers nothing there.

The edge-on-face lane cuts an edge at every boundary vertex on it and
every place a boundary edge crosses it, each placed in metres along
both (a crossing within the band of an arc's end is that end's vertex),
and refuses a face whose spiric or spline edge it cannot clear, so no
cell spans the face's boundary on a decided reading.

**`EdgeFacePierce` stays categorical.** A transverse dive is
interpenetration, and no declaration backs it. Inside one body the
recourse is separating the solids or making the crossing a boolean's
working state; at rest between two copies it is interference evidence,
reported as ASSEMBLY A5's interference finding.

**The `interior_witness` schedule.** A flush seat's trims share a
boundary, so the region walk refuses `TouchingBoundary`; the witness rung
turns that refusal into `PositiveArea` by certifying one point strictly
interior to both trims (`contfp`, both faces' rings). Candidates are
uncertified hints in two stages — the trims' own landmarks, then the cell
centres of the vertical decomposition of both boundaries
(`decomposition_witness`) — each certified at use, so the schedule
affects only what declines, never what certifies. A schedule stopped by
one of its two caps refuses typed, one face per cap:
`WitnessSegmentCapExceeded` (over `WITNESS_SEGMENT_CAP`, nothing probed)
and `WitnessCellCapExceeded` (`WITNESS_CELL_CAP` probes made, cells
left).

## Related pages

`docs/DESIGN.md` (`[topology]`, `[determinism]`); `crates/verbs/README.md`
(VERB-SEAT-DESIGN, query doors at topo); `docs/MATE-7-TANGENCY-DESIGN.md`
(rim tangency routing); `crates/editor-core/CHECKS.md`;
`crates/geom-brep/README.md` (curved geometry, offsets, shelling);
`crates/editor-core/ASSEMBLY.md` (mates and the at-rest door);
`docs/guide/assembly.md`.

## Open

- Inside one body the `EdgeFacePierce` arm (issue 973) refuses typed;
  no declaration admits it (C6).
- The declared-cusp wedge-0/2π arm is defined, unbuilt (941).
- Sphere, cone and torus cross-description declared pairs refuse
  `ChartDivergence`; the C9 exclusion ring for same-solid distinct-key
  curved pairs is unbuilt.
