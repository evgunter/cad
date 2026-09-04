# topo

`topo` is the B-rep topology crate. A **B-rep** (boundary representation)
describes a solid by its boundary — faces on surfaces, edges on curves,
vertices at points — linked by a **half-edge** structure (each edge is two
antiparallel half-edges, one per bounding face). `Body<T>` holds that in
typed generational arenas: scalar-free topology plus `T`-valued geometry
arenas, built only through the Euler operators (`docs/DESIGN.md` D1).
Above the arenas the crate carries the validation tiers, plane splitting,
the booleans and their declared-contact lanes, the at-rest coincidence
census, the contact vocabulary, shell/offset surgery and the kernel query
seat. Orientation conventions are stated once, in `src/entity.rs`.
Determinism (D9): arena iteration is slot-index order, no hash container
feeds a decision, every walk is bounded.

## Where in the code

| Area | Modules |
|---|---|
| Arenas, entities, provenance | `src/body.rs`, `src/entity.rs`, `src/geometry.rs`, `src/provenance.rs`, `src/source.rs` (`GeomSource`, a description's recipe identity), `src/live.rs` |
| Euler operators | `src/euler.rs` (make), `src/euler_kill.rs` (kill duals), `src/euler_ring.rs` (rings/genus), `src/split.rs`, `src/movefac.rs`, `src/revert.rs`, `src/attach.rs` |
| Validation tiers 1–3, 3′ | `src/validate.rs` (`validate`, `validate_closed`, `validate_geometric`, `validate_pseudomanifold`), `src/face_normal.rs`, `src/sector_face.rs`, `src/sector_shape.rs`, `src/coherence.rs` |
| Coincidence census, at rest | `src/census.rs` (`census_and_certify`: the sweeps, the backing rungs, the confirm pass, the cross-solid backstop) |
| Contact vocabulary | `src/contact.rs` (`ContactClass`, `DeclaredContact`, `ContactVerdict`, `ContactRefusal`, `ContactFinding`, `CONTACT_RECOURSE`, `FIT_DEFERRAL`) |
| Contact verification (Door 1) | `src/boolean/contact_verify.rs` (`contact_pair_verdict`), `src/boolean/carrier_eq.rs` (the kind-generalized carrier ladder), `src/boolean/plane_eq.rs` (its planar arm) |
| Chart-region overlap (Door 2) | `src/chart_region.rs` (`chart_region_overlap`, `declared_pair_overlap`, `world_carrier`, `cylinder_pair_overlap`, `interior_witness`), `src/chart.rs`, `src/chart_iso.rs`, `src/pcurves.rs` |
| Plane splitting | `src/splitting/` (`classify`, `neighborhood`, `rules`, `insert`, `order`, `join`, `finish`, `section`, `containment`), `src/chord_join.rs`, `src/null.rs` |
| Booleans | `src/boolean/mod.rs` (reduction, classification, `ContactRecords`, `BooleanDeclarations`), `reduce.rs`, `vtxfac.rs`, `sectors.rs`, `recl.rs`, `tables.rs`, `insert.rs`, `join.rs`, `finish.rs`, `zip.rs`, `ops.rs`, `combine.rs`, `voids.rs`, `boxes.rs`, `contain.rs`, `solid_contain.rs`, `surface_group.rs`, `rim_wedge.rs` |
| Declared-REST zip (C7 join lane) | `src/boolean/rest.rs` |
| Instances, separation | `src/instance.rs` (disjoint graft), `src/separation.rs` (certified no-touch), `src/transform.rs` (rigid placement) |
| Shell and offset surgery | `src/shell.rs`, `src/replace_face.rs`, `src/offset_together.rs`, `src/offset_axial.rs`, `src/merge_faces.rs` — decisions in `crates/geom-brep/README.md` (OFFSET-DESIGN) |
| Queries, flush detection, read-back | `src/query.rs` (`docs/VERB-SEAT-DESIGN.md`), `src/flush.rs`, `src/readback.rs`, `src/props.rs` (mass properties, `AtRestPolicy`), `src/ray_parity.rs` |

## Contact census and declared contact (the CONTACT-DESIGN clauses, C1–C8)

These ids are the contact-design clauses; the curved-geometry design in
`crates/geom-brep/README.md` has its own C-numbers, always cited
qualified (`CURVED-DESIGN C7`). Setting: two bodies whose boundaries
touch, at rest or as boolean operands. The **census** (tier 3′,
`validate_pseudomanifold`) finds every cross-entity coincidence; a
**declaration** is recipe data asserting a contact class on a named face
pair; a **record** (`ContactRecords`: `VvContact`, `VfContact`,
`CurveContact`, `PatchContact`) is the verified form a result body
carries. The coincidence ladder: structural (shared key or same
`GeomSource`) is intent by construction; declared is intent plus
non-contradiction; value equality never glues. Certification runs both
ways — a finding with no backing declaration is `UndeclaredContact`, a
declaration with no witness is `StaleContactDeclaration` — and the
census never blesses what it discovers.

**C1 — Classification of the pair germ.** At p ∈ ∂A ∩ ∂B interior to a
face of each, write each boundary as a graph h_X over its tangent plane
along the outward normal n_X (material below) and II_X := Hess h_X(0).
Classes: *transverse crossing* (n_A, n_B independent; interference at
rest); *aligned tangency* (n_A = n_B; always local material overlap, so
containment or interference, never contact); *opposed tangency*
(n_A = −n_B) with separation s = h_B − h_A and relative form
II_rel = Hess s = −(II_A + II_B), sub-classed as *point touch*
(II_rel ≻ 0), *curve touch* (rank-1 kernel, s ≡ 0 along a witnessed
curve, κ_rel the positive eigenvalue), *conformal* (s ≡ 0 on a patch),
*crossing touch* (indefinite; interference) and *degenerate residue*
(in-band κ_min with distinct carriers; escalates); *interference* is the
regional class. Every class boundary is a trilean at a stated order (1:
normal independence; 2: sign/rank of II_rel; ∞: conformality, decided
structurally); nothing is classified by proximity. The second-order arm
in code is the transverse κ_rel of the jet schedule
(`contact_verify::tangent_pair_relation`, `sectors.rs`'s second-order
lump); an indefiniteness test of II_rel over all tangent directions is
not implemented, so a tangency indefinite only off the sampled
direction is missed, not bridged.

**C2 — Representation boundary.** Identity lemma: for the analytic kinds
two surfaces agreeing on an open patch agree as loci, so every true
conformal contact is same-carrier contact; for piecewise-rational
carriers the guarantee is per knot-span, and a span-partial coincidence
without structural or declared backing escalates. Conformality is thus
decided structurally, never numerically; identity is at the locus level,
and two descriptions of one locus may differ as charts (`u_ref`, seam)
and as sources — which is what the declared rung is for. Face-pair
procedure: exclusion, structural rung, declared rung, definite
separation/crossing by geometry, in-band ⇒ escalate. The exclusion step
as built is `census::sweep_cross_solid_backstop`: a cross-solid pair with
a curved side is cleared only on a definitely-positive separation margin
from certified reach boxes (`face_reach`) and refused `CensusUndecidable`
otherwise; same-solid distinct-key curved pairs are undetected (their
constructor's obligation). Refusals are typed with `CONTACT_RECOURSE`
(declare the class or move the geometry; no tolerance arm, since ε cannot
supply intent). Invariant: no flag, mode or tolerance glues value
equality.

**C3 — Record granularities.** `CurveContact { face_a, face_b, witness }`
is a certified curve touch: the jet schedule along the witness edge's
carrier (coincidence within ε, normal opposition within ε·κ_rel, κ_rel
definitely positive, hull bounds between samples); endpoints are bounded
by vertex records or the locus's closure, and an unbacked bound is
`UndeclaredContact`. `PatchContact { face_a, face_b }` is a certified
conformal patch: carrier identity by the structural or declared rung,
senses opposed (aligned coincidence is contradicted), definitely-positive
trim overlap in a shared chart — exact on the planar trim inventory
(`chart_region.rs`), typed elsewhere (`NonPlanarTrim`, `ArmUnbounded`,
`SeamBranch`); empty ⇒ stale, in-band ⇒ escalate. The chart authority is
one of three, in fixed order (`declared_pair_overlap`): the structurally
shared chart (`same_chart`); for a declared **planar** pair the shared
world carrier — one plane description taken as representative frame,
legitimate by the frame-invariance lemma at `world_carrier` (both chart
maps are isometries, so every quantity the area machinery consumes is
Euclidean-invariant) and gated by `carrier_agreement`, which meters the
descriptions' disagreement over the pair's own boundary vertices; for a
declared **cylinder** pair the certified everywhere-within-ε enclosure
(`cylinder_pair_overlap`: one description's trims carried across the
exact affine chart relation, the angle folded to one period through
`periodic_branch`, gated by the `chart_region_cyl_*` rows). Every other
cross-description declared pair refuses `ChartDivergence`. The claim
earned is *certified everywhere within ε*, never exact (`Ok(Zero)` means
`|m| ≤ zero`). Invariant: certification strength equals its skeleton; a
contact of order k > 1 has no record type and refuses. Area sampling is
rejected (it can miss a trim hole).

**C4 — Declared contact as data.** A declaration names two faces by
stable name on the consuming node and asserts a `ContactClass`: `Rest`
(same carrier, opposed senses, gap ≡ 0), `Tangent` (curve/point touch,
non-crossing) or `Fit { gap }` (carrier-parallel at a signed nominal gap;
specified, not built — `FIT_DEFERRAL`). Verified, never trusted: each
class states a must-verify-DEFINITE list, contradiction triggers and a
bridged residue, and the declaration bridges only the third
(`ContactVerdict::{Definite, Bridged}`; `ContactRefusal::{Contradicted,
Escalated, Undeclared, NotCertifiable}`). `Rest`: carrier
non-contradiction through the kind ladder (`carrier_eq`: plane, sphere,
cylinder, torus; angular margins levered at the consumed extent, length
margins at unit arm), senses opposed as an exact bit, overlap definitely
positive on C3's chart authority; contradicted by definitely distinct
carriers, aligned senses, definite separation on the patch. `Tangent`:
first-order tangency along the witnessed locus, locus on both surfaces
within ε; contradicted by definite normal independence, definite
crossing (as far as the sampled κ_rel sees it, C1), definite separation;
bridged: in-band κ_rel *including exact zeros at isolated points* (a G1
tube chain's neutral meridians), deliberately weaker than a jet
certificate. The two doors are coupled: Door 2 receives Door 1's
verdict; the planar interior-witness rung runs only on `Definite` (a
precondition may not be discharged by the claim under test) and the
cylinder enclosure halves its zero band on `Bridged`. Declarations live
in `BooleanDeclarations::coincident_faces` (`FacePairDeclaration`) and
on mate nodes (`crates/editor-core/ASSEMBLY.md`); bodies carry only
verified records in the `BooleanBody` wrapper, never persisted. Replay
is scalar-generic; an indeterminate verification at an interval scalar
aborts. Failures, all typed: `UndeclaredContact`, `ContactContradicted`
(at use and at rest), `StaleContactDeclaration`, `CensusEscalated`.
Invariant: every definite verdict wins over every declaration.

**C5 — The signed gap.** For a declared pair on same-kind carriers with a
shared mating frame, g is the carrier-relative signed offset: parallel
planes, the material separation along the outer face's outward normal;
concentric spheres, R − r − ‖Δc‖; coaxial cylinders, r_b − r_p − d (skew
axes refuse). **g > 0 clearance, g = 0 contact, g < 0 interference.**
The census classes are the strata of g's zero set: g = 0 with structural
frame sharing is `Rest`, g = 0 with an offset frame is an internal
point/curve touch, so the conformal limit is reached by structure, never
by g drifting to zero. g is linear in the radii under structural frame
sharing; with independent frames it carries the norm kink ‖Δc‖ at
Δc = 0 (Clarke subdifferential the closed unit ball; the `Dual<Interval>`
straddle-hull treatment). Built as the document layer's
`Measure::Gap { outer, inner }` (`crates/editor-core/src/eval/measure.rs`),
argument order the mating role, authored rather than inferred.

**C6 — Interference fits.** `Fit { gap: g₀ }` asserts a nonzero nominal
gap; g₀ = 0 is rejected (zero gap is `Rest`). A verified interference fit
makes the disjointness/containment/extent gates skip the pair as a
*recorded* verdict naming the declaration; assembly mass properties
refuse by default with an explicit opt-in subtracting closed-form overlap
volumes; booleans are unchanged; STEP export drops the declaration. Not
implemented: the variant lands with its first consumer, and until then
the nested-instance class (one instance's extent box inside another's)
refuses at the backstop. Invariant: an undeclared interference is always
a typed error; no blanket "disable interference checking" exists.

**C7 — The join lane.** At the curved coplanar-lump sites (`vtxfac.rs`,
`recl.rs`) an undeclared tangent pair refuses `CurvedBooleanUnsupported`;
a verified `Tangent`/`Rest` descends to second order (`sectors.rs`: the
sector's relative transverse curvature signed against the other face's
outward normal, the declaration bridging an exact zero). The zip
(`boolean/rest.rs`) removes conformal patches as interior on any carrier
the ladder certifies and mints each seam once, so union volume is exactly
additive at full engagement. A rim with a determinate G1 jet carries
`TangentIntersection`; rim routing by material wedge (π ⇒ smooth seam;
0/2π ⇒ the declared cusp family, defined but unbuilt,
`BooleanError::RimCuspArmUnbuilt`) is `docs/MATE-7-TANGENCY-DESIGN.md`.
The same substrate is the at-rest door: `validate_pseudomanifold` with
mate declarations landed in `ContactRecords`, no boolean, no zip.

**C8 — Ratification boundary.** Refusals are typed per class and retire
per table arm, never wholesale; `CensusUnsupported` and
`CurvedBooleanUnsupported` name the C4 recourse vocabulary. Still open:
NURBS↔analytic same-locus recognition; kinematics (records are
geometric; mate solving is not here).

## At-rest census identity (CENSUS-REST-CLOSURE)

**Face rung at rest.** The boolean lane refines every vertex-on-edge
event to vertex-vertex before records exist; at rest nothing refines. So
`sweep_vertex_edge` and the asymmetric arm of `ee_bound_backed` read
`Declared::ve_face_backed`: a declared face pair holding the vertex on
one boundary and naming a face the edge bounds backs the event; with no
such pair it is an undeclarable defect (`CensusContact::VertexOnEdge` as
`UndeclaredContact`). The rung consults declarations, never the
geometry's agreement with itself.

**World-carrier Door 2 for declared planar pairs.** Two instances of one
part carry `Placed { node, instance, .. }` sources that never equalize,
and a `Rest` pair's charts mirror, so structural chart identity is
unreachable across instances and provenance transfer cannot restore it.
Door 2 therefore answers a declared planar pair on the verified shared
carrier (C3's planar authority) with the one-body parity walk; the claim
is certified within ε at the pair's own extent, not exact.

**Cross-instance curved `Rest`.** The sanctioned closing shape is a
certified everywhere-within-ε overlap enclosure on the shared curved
carrier; the cylinder arm is built (C3); sphere, cone and torus keep the
typed divergence with that shape recorded at the refusal site.

**Attribution at the assembly layer.** Each live `Rest` mate is minted as
a `PatchContact`; findings attribute to mates as Declined, Unattributed
or Refuted (`StaleContactDeclaration`); all-Declined is the `Uncertified`
frontier, any Unattributed is a hard `AtRest` error
(`crates/editor-core/ASSEMBLY.md`).

## Crossing backability (MATE-4B-CROSSING)

**Unified backing strength.** A declared pair answers exactly for its
verified interface: the overlap region, with material opposition being
what "interface" means for a crossing. Unifying down (crossings at
structural strength) would bless transverse interpenetration.

**The `EdgeEdgeCross` rung** (`census::ee_cross_backed`, planar-first) is
that strength's first instance. A declared pair backs a crossing of two
coplanar boundary edges iff the crossing point lies on both carriers
inside both closed trims and both edges lie in both carriers
(`pair_holds_point`, `pair_holds_edges`), the side test answers
`OppositeSides`, and the pair verifies through both doors in either frame
order (`pair_region_verified`). Confinement is by carrier, not incidence:
nothing requires the crossing edges to bound the declared faces.

**Three-valued side verdict** (`CrossingSideVerdict`): `OppositeSides`
backs; `SameSide` refuses naming the verdict and is the future
declared-interpenetration hook (C6 consumes it as admission evidence, so
no bool may stand there; today it reaches the refusal only as rendered
witness text, not a typed field); `Undecided` escalates `CensusEscalated`.
The side is read via `Face::sense_sign` and
`geom_brep::classify_material_pairing` after `classify_dihedral`
establishes the smooth precondition; the census is otherwise
sense-invariant.

**Grandfathered rungs.** `vv_face_backed`, `vf_face_backed`,
`ve_face_backed`, and the face-pair arms of `ee_bound_backed` and
`ef_bound_backed` confine by structural incidence only and can back an
event outside the pair's overlap region. Each migrates one at a time,
measured; `ef_bound_backed` measured badly because the edge-on-face lane
cuts cells only at coincident boundary vertices (the D3 reach gap,
`census.rs` module docs), so its migration waits on boundary-crossing
cuts.

**`EdgeFacePierce` stays categorical.** A transverse dive is
interpenetration until a C6 vocabulary exists; the recourse is separating
the bodies or making the crossing a boolean's working state.

**The `interior_witness` schedule.** A flush seat's trims share a
boundary, so the region walk refuses `TouchingBoundary`; the witness rung
turns that refusal into `PositiveArea` by certifying one point strictly
interior to both trims (`contfp`, both faces' rings). Candidates are
uncertified hints in two stages — the trims' own landmarks, then the cell
centres of the vertical decomposition of both boundaries
(`decomposition_witness`) — each certified at use, so the schedule
affects only what declines, never what certifies.

## Related pages

`docs/DESIGN.md` (D1, D9, the tier ladder); `docs/VERB-SEAT-DESIGN.md`
(query doors at topo); `docs/MATE-7-TANGENCY-DESIGN.md` (rim tangency
routing); `docs/DISCIPLINES-DESIGN.md`; `crates/geom-brep/README.md`
(curved geometry, offsets, shelling); `crates/editor-core/ASSEMBLY.md`
(mates and the at-rest door); `docs/guide/assembly.md`.

## Open

- The `EdgeFacePierce` arm (issue 973) waits for the C6 interference era.
- `ef_bound_backed`'s migration waits on boundary-crossing cuts (1500).
- `interior_witness`'s budget-exhaustion decline is untyped (1478).
- The declared-cusp wedge-0/2π arm is defined, unbuilt (941).
- Sphere, cone and torus cross-description declared pairs refuse
  `ChartDivergence`; the C9 exclusion ring for same-solid distinct-key
  curved pairs is unbuilt.
