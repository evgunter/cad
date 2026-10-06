# `sweep` — solids from profiles, and the edge blends

`sweep` turns a `profile::ValidatedProfile` into a closed B-rep solid
(faces on stored surfaces, edges on stored curves, glued by half-edges)
and blends the edges of one. The sweep verbs are extrude, revolve, loft
and path sweep, all driving `topo`'s certified Euler operators over one
shared lowering. The blend family is the constant-radius rolling-ball
fillet and the equal-setback chamfer: one validity battery run over the
inputs before any surface is minted, one table of analytic arms, one
in-place composition surgery that splits the supports along stored
trimlines and grafts the band in, one birth record per minted entity,
one verb-neutral refusal vocabulary. Nothing is sampled or approximated.

The fillet design proper (the battery's six predicates in binding
order, the arm table, the corner-configuration scope) is in
`crates/geom-brep/README.md` under CURVED-DESIGN C8. What is not yet
built (mid-curve run-outs, the canal-surface blend, curved-support
chamfers) is registered in `docs/KERNEL-VERBS.md`; the canal blend is
`docs/DESIGN.md` frontier (f).

## Where in the code

| Concern | Module |
|---|---|
| Extrude: translational sweep along the sketch normal; caps, walls, hole rings, rim upgrades | `crates/sweep/src/extrude.rs` |
| Revolve: axis and angle conventions, partial wedge and full ring, seam meridians, the `tube_along_arc` torus door | `crates/sweep/src/revolve/` (`mod.rs`, `axis.rs`, `partial.rs`, `full.rs`, `chain.rs`, `surfaces.rs`, `upgrade.rs`, `tube.rs`) |
| Loft and path sweep bodies: extrude's topology over NURBS walls | `crates/sweep/src/loft.rs` (`loft_body`, `sweep_body`) |
| Skinned and swept NURBS surfaces (the definitional geometry, not an approximation) | `crates/sweep/src/skin.rs` |
| The lowering every profile sweep shares: traversal order, carriers, cosurface decisions | `crates/sweep/src/swept.rs` |
| The two blend doors, one per verb; each attaches its `BlendKind` once | `crates/sweep/src/fillet.rs`, `crates/sweep/src/chamfer.rs` |
| Shared blend vocabulary: `BlendKind`, `BlendRefusal`, `BlendError`, `CornerConfig`, `RunOutPolicy`, recourse sentences | `crates/sweep/src/blend/mod.rs` |
| Validity battery, per-verb predicate gating, arm dispatch (`coaxial_arm`), `is_seam_vertex` | `crates/sweep/src/blend/battery.rs` |
| Analytic arms: sheet derivation, `BlendArm`, chamfer strip, corner ball | `crates/sweep/src/blend/arms.rs` |
| Admission tokens (holding the value is the fact) | `crates/sweep/src/blend/admit.rs` |
| Assembly front doors, `Blended` result type, octant charts | `crates/sweep/src/blend/build.rs` |
| In-place composition surgery: the door, the plans, the ring check, the description pass, the one `kef` door; the closed-rim walks (ladder rims, annulus rims across seams) | `crates/sweep/src/blend/surgery.rs` |
| The open bands: the plane–plane band with its trihedral corners, the ruled band with its transverse cut-off | `crates/sweep/src/blend/open/planar.rs`, `crates/sweep/src/blend/open/ruled.rs` |
| Birth records (`BlendNaming`) the document layer turns into names | `crates/sweep/src/blend/naming.rs` |

## Walls: one per run

A *run* is a maximal chain of adjacent profile pieces the lowering's
cosurface verdict puts on one carrier — collinear lines, cocircular
same-turn arcs — however the author wrote them (a declared straight
continuation, a station kept on a side, a raw collinear polygon). Extrude
and revolve build ONE wall per run on every carrier kind, so no sweep mints
a same-key adjacency for a merge to undo. The one exception is a run that
is the whole closed loop (a circle): it keeps its canonical cut (C12.5).
A station inside a run has no entity in the body: a cap carries the run as
one rim edge, as the wall is one face (`docs/DESIGN.md`, maximal edges). It
stays in the profile, where `ProfileVertexRef` names it.
Loft builds one wall per corresponding segment pair: across sections
nothing declares two walls one surface, and the station pins the ruling.

## Blend vocabulary (BLEND-VOCAB-DESIGN V1–V4)

The fillet and the chamfer are one request over the same bodies, judged
by the same predicates and carved by the same surgery; only the band
differs. They share one error type; these decisions say how a shared
refusal names the verb that raised it.

**V1 — the verb crosses as one wrapper at the door.** `fillet_edges`
and `chamfer_edges` return `Result<Blended, BlendRefusal>`, where
`BlendRefusal { verb: BlendKind, error: BlendError }` is minted once by
the door's `map_err` and nowhere below. The recipe layer's
`NodeErrorKind::Blend { verb, error }` (`crates/editor-core/src/eval/`)
is the same wrapper one layer up, filled from `BlendRefusal::verb`. One
discrimination point per layer: no verb field on the variants, no
per-verb enum.

**V2 — inner prose is verb-neutral; the wrapper supplies the verb.**
`BlendError`'s `Display` and the shared `FILLET3_*_RECOURSE` constants
never name a verb; `BlendRefusal`'s `Display` prefixes `"fillet: "` or
`"chamfer: "`. A recourse is a claim about a second request, so every
shared sentence is true under both verbs; a door one verb lacks is
conditioned in the sentence (the closed-chain clauses of
`FILLET3_ASSEMBLY_RECOURSE` and `FILLET3_SEAM_VERTEX_RECOURSE` say a
chamfer has no band). Ball-only arms keep ball language because no
chamfer run reaches them: `RadiusHeadroom` and `SpineIrregular` are
metered only when `run_battery_for` sees `BlendKind::Fillet`, and
`SpineUnsupported` sits behind the chamfer's early return
`ChamferArmUnsupported`.

**V3 — the shared machinery is blend-named; per-verb doors are thin.**
The module is `sweep::blend` (`BlendRequest`, `BlendError`,
`BlendNaming`, `blend::surgery::blend_surgery`); `sweep::fillet` and
`sweep::chamfer` re-export their door and result alias and nothing
else; ball-specific identifiers (`corner_ball`, spine and torus
language) keep their names. Three fences stay fillet-named on purpose:
the `fillet3_*` predicate names (a K-corpus family; renaming would fuse
telemetry buckets), the persisted `RoleSeg`/`RimSupport` role vocabulary
in `crates/editor-core/src/names/`, and `OpGroup::Fillet` there, whose
name under-describes what it groups (the chamfer reuses the same roles;
the minting node tells the two apart).

**V4 — no parallel enum.** There is no `ChamferError`; minting one is
not an admissible way to discriminate the verbs.

Settled choices: "edge blend" is the neutral generic noun; `Blended<T>`
is the one result type of both doors, `Filleted<T>` and `Chamfered<T>` its call-site aliases.

## Fillet arms and the seam vertex (ARMS3-DESIGN A3-1…A3-3)

**A3-1 — the sphere×sphere arm is a row of the coaxial family.** Two
spheres on distinct centres always meet in a circle whose axis is the
line through the centres, so the pair is coaxial by construction and
`fillet3_support_coaxiality` is zero there rather than measured.
`coaxial_arm` maps `(Sphere, Sphere)` to `BlendArm::SphereSphereTorus`;
the centre is the crossing of the two offset circles in the meridian
sheet through the rim (`Meridian::trace`), material sides read from
each face's stored sense bit folded with the chain's convexity verdict
(the ball rests on the material side of each support on a convex
chain and in the void on a concave one — S10/S11, the one fold every
arm spells, `plane_plane_blend`'s `signed` its precedent). A
tangential pair poisons the spine radius and escalates at predicate 3
(`spine_regularity`). The band carves on either material side: a
lentil's convex equator loses material to it, a two-sphere snowman's
waist gains it, through one surgery.

**A3-2 — a valence-4 seam vertex is not a corner.** Where a chart seam
(the `u = 0` meridian of a revolved wall) crosses an otherwise smooth
latitude rim, the vertex has valence four: two rim arcs carrying one
support pair and two co-surface seam meridians with dihedral zero. The
surface is smooth through it, with no wedge and no ball-rest
configuration, so no run-out policy applies. A chain ending there refuses
`BlendError::UnsupportedCorner { corner: CornerConfig::SeamVertex,
policy: None }`; `CornerConfig::policy` is the single tag-to-policy map,
so a payload cannot disagree with its tag. `battery::is_seam_vertex`
reads pure incidence, never convexity, which fixes the rule: **a
recourse must be true at every site its tag can fire**.
`FILLET3_SEAM_VERTEX_RECOURSE` therefore names the REQUEST (ask for the
rim whole, every arc the seam split it into), and the closed-rim
surgery serves it on either material side: it takes that multi-link
closed chain as one annulus (`AnnulusRim::crossings`, one
`SeamCrossing` per arc, each side's support several faces of one
surface), removing material on a convex rim and adding it on a concave
one, so the sentence conditions on nothing. A pole-touching body with
merged caps (`merge_coplanar_faces` — the repair every boolean consumer
runs) hosts every arc on ONE plane face, in that face's own outer
cycle. The same annulus serves that too: `resolve_rim` routes it there
on WHERE the rim sits in its host's loop structure (a ring is the
ladder, the face's own outer cycle is this), and each crossing's host
foot is minted by the LADDER's strut (`HostFoot::Strut`) because the
merge consumed the host's seam and left the crossing TRIVALENT. The tag
does not fire at such a crossing — there is no seam there to make a
seam vertex — and the subset request that does refuse names the whole
rim, which carves. **One condition on that host, and it is what the
recourse states**: the rim is its WHOLE outer cycle. A RING of the host
is not a second condition but a clearance — the band's host trim
becomes that face's new outer boundary, so a ring carries through
exactly when the trim CONTAINS it, metered before any mutation under
`fillet3_ring_clearance` (`blend/surgery.rs`'s ring carry-through
pass, which meters every ring of every touched support face against
every blend trimline in closed form, every other outer-boundary edge
of a closed rim's supports — one requested in the same call at its own
trim — against that support's trim, every edge a convex cut-off
leaves on its end face against the sliver it removes, and every
outer-boundary edge a planar band's local carve leaves on a support
against the strip it removes). A merged cap that is an ANNULUS
therefore carves on both its rims, one call each. A CURVED single face
carrying every arc is authorable through `topo`'s `kef` and refuses at
the half-band gate on both routes
(`work/blend/curved-single-host-rim-refuses-at-the-half-band-gate.md`).

**A3-3 — the genuine mid-curve run-out is named and not implemented.**
Stopping a band part-way along a smooth rim, at a station with no
vertex, has two honest shapes: a ball-cap stop (the ball at rest at the
final spine station caps the band with a sphere patch; new surgery, no
new surface kinds) and a feather-out (the radius tapers to zero toward
the station; variable-radius machinery). Neither has a constructor
(`RunOutStopAtVertex` and `RunOutFeather` are refusal-payload
vocabulary only); the ball-cap is the presumptive first pick when a
consumer arrives.

**Where a straight band ends, and where it turns.** One rule: a band
ends where it meets the face it runs into. At a trivalent vertex of one
convexity between planes the request decides which face that is, by how
many of the vertex's three edges it names, and every answer is exact on
stored kinds (plane, cylinder, line, circle, ellipse).

- *All three* — the corner patch: the ball at rest (a sphere octant),
  or the chamfer's plane through the three feet.
- *One* — the CUT-OFF: the band ends in the end face's plane section of
  it (`CornerConfig::EndFace`, `RunOutPolicy::CutOffAtEndFace`). A
  chamfer ends in a chord at any angle; a fillet in a circle when the
  end face is perpendicular to the edge and an ellipse otherwise,
  `fillet3_cap_transverse` deciding which (one kind per configuration,
  D3). The end face gains the curve and loses (convex) or gains
  (concave) the sliver between it and the old vertex; the two
  unrequested edges end at the feet; the old vertex goes. The ruled
  band's transverse cut-off below is the perpendicular case.
- *Two* — the MITRE (`CornerConfig::Turn`, `RunOutPolicy::Mitre`): each
  band is cut off by the other band's support, the two regions overlap,
  and the bands meet along their intersection — a line for a chamfer, a
  planar ellipse for a fillet (two cylinders of one radius tangent to
  the shared support, whose axes therefore meet). When the trihedron is
  isosceles about the unrequested edge L — equal dihedrals at the two
  requested edges, the same fact as equal face angles at the vertex —
  the mitre runs from the trimlines' crossing on the shared face down to
  L, and L ends there. Otherwise the band that reaches further is cut off
  past the mitre by the other band's far support, one more short curve
  down to L. Which holds is a margined verdict: Zero builds the first,
  definite the second, the sliver band refuses. No ball rests at a turn,
  so the mitre is G0 between the bands; it never competes with the
  patch, because the request count decides.

Chain G1 (predicate 4) is a classifier at a junction of two plane–plane
links: Zero, one band through a joint; definite, two chain ends at a
turn; sliver, escalated. Tangency is what lets one band run through a
junction, not a condition on the request. It keeps its name and its
place in the order, and still refuses at a junction involving a curved
link. The planar band carves locally, as the ruled band does: the end
face's rim edges split at the feet, the end curve `mef`'d across it, one
trimline per support.

What still refuses, typed `UnsupportedRunOut` with a detail naming the
shape: a curved end face (a fillet against a cylinder meets it in a
quartic with no stored carrier); a foot that lands inside a support
rather than on a rim edge (a band running into a wall or a step, and the
inner corner of an L-shaped rim, where the shared face's sector is
reflex); an end vertex of valence other than three. A turn whose two
edges round opposite ways refuses `UnsupportedCorner { MixedConvexity }`.

Naming: the cut-off curve is `EndArc { vertex, edge }` and the feet
`FootVertex { vertex, support }`, as the ruled cut-off names them; the
trimlines' crossing at a turn is `FootVertex { vertex, shared support }`.
A turn adds two roles, keyed by the vertex the bands end at — a
trivalent vertex has at most one turn, and the request fixes its edges:
`Mitre { vertex }` for the edge where the bands meet, and
`TurnFoot { vertex }` for the point where L ends, one name whether the
trihedron is isosceles or not. In the non-isosceles case the extra
curve is that band's `EndArc` and its end `FootVertex`; both resolve
`Vanished` at equality. In the isosceles case the turn foot has four
edges, so blending L in a later call refuses, and the recourse is to
request it in the same call, which builds the patch.

**What IS built beside it is a different termination — the ruled
band's TRANSVERSE CUT-OFF (FILLET-H7, Ev's ruling on PR 1736).** A
ruled link (`CylinderPlaneCylinder`, `CylinderCylinderCylinder`: a
cylinder band about a straight spine, both trimlines lines along the
ruling) ends where its supports do, at a vertex whose two unrequested
edges lie in one plane face perpendicular to the ruling —
`CornerConfig::EndFace`, decided by `fillet3_cap_transverse`
(the cap normal's departure from the ruling, in meters at the link's
own extent, the lever the shared-ruling hypothesis is metered at). The
band ends in that plane's section of it, an exact stored arc of the
band's radius about the spine's crossing
(`RunOutPolicy::CutOffAtEndFace`; `CornerConfig::policy` maps
the tag). The carve (`blend/open/ruled.rs`, beside the planar band's
`blend/open/planar.rs`, both cutting off through `blend/open/end_face.rs`;
the rim phases stay in `blend/surgery.rs`)
mints no strut: the cap's two
rim edges are split at the trimlines' feet, the arc is `mef`'d across
the cap, one trimline `mef` per support carves its strip along the
ruling, and the crease's `kef` with two `kef`/`kev` pairs folds the
slivers in and retires the old vertices — the trimlines described as
the band's tangent contact with a curved support, the arcs as its
transverse intersection with the cap, on either material side. On the
convex side the cut removes the sliver between the arc and the old
vertex from the cap, and leaves every other edge of the cap where it
was — the edges of its other cycles (a bore's ring, or the outer cycle
where the cut runs in a ring) and those of the cut cycle other than
the two rims it shortens (a notch in the outline). Each is metered
before any mutation, over its own window, against a region that
encloses the sliver: the annulus about the spine's crossing from the
band's radius out to the farthest the sliver reaches, cut down to the
half-plane towards the old vertex that the sliver lies in. The meter is
the same ring carry-through pass under the same
`fillet3_ring_clearance`; an edge not definitely clear of the region
refuses `RingClearance` at the cap
(`crates/sweep/tests/band_ruled_cap_ring.rs`). A
curved end face refuses typed.
Consumer: the rod with a flat milled along it (`cylinder ∖ box`), both
creases in one call, at the prism closed form `ΔV = A_section · L`
(`crates/sweep/tests/fillet_h7_transverse_cap.rs`). The CONCAVE
ruled band — the material-adding side, the cap gaining the region
under the arc — is pinned through the extrude door too: a rod's section
standing on a block's top edge (the sunk rod,
`crates/sweep/tests/review_fillet_h7_r1_probes.rs`, `ΔV = +2·A·L`). The
`CylinderCylinderCylinder` consumer — two parallel cylinders unioned at
a common ruling — has no body yet: the union refuses at the boolean's
curved-pierce door, and so does a block ∪ cylinder at its join lane;
that is the boolean's ground, not the band's.
