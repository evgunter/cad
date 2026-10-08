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
| Predicate 2's reach: every band against every face of the body that is not a support of its chain, in any shell, before any mutation | `crates/sweep/src/blend/reach.rs` |
| The open bands: the plane–plane band with its trihedral corners, the ruled band with its transverse cut-off | `crates/sweep/src/blend/open/planar.rs`, `crates/sweep/src/blend/open/ruled.rs` |
| Birth records (`BlendNaming`) the document layer turns into names | `crates/sweep/src/blend/naming.rs` |

## The band's reach (predicate 2)

A band changes the material between its supports and its blend surface:
a convex band removes it, a concave one adds it. Predicate 2 meters that
region — the band's REACH — against every face of the body that is not
a support of the chain, in any shell: a face there would be cut through
or buried, and the surgery has no step for either. The supports' own
boundary features are judged first, by the battery's screen and the
surgery's exact ring and boundary meters; the reach runs after them,
before any mutation, and refuses `BlendError::FaceClearance` — a
measurement (`bounded: false`) when a point of the face's own boundary
lies inside a region that is the band's material exactly (plane
supports, or a whole rim over a plane and a cylinder), and a bound
(`bounded: true`) whenever the face could not be certified clear.

Each reach is an intersection of 1-Lipschitz bounds: per link, the
cross-section between the supports, the sector the ball's arc subtends
and the ball (the chamfer's triangle), run along a straight spine over
the edge's window and closed by the plane of the face each end runs
into (at a mitre the other band's support, which the band's section
passes no later than the mitre's plane of symmetry), or revolved
over the whole turn of a circular one; per corner patch, the three
supports, the three band-end planes and the ball. The faces are pruned
by their certified boxes (`topo::FaceBoxes`); a face on a surface of
revolution about a circular band's axis is metered in the band's
meridian sheet over its own extent along its trace (read off its
boundary: the azimuth it winds, and on a sphere the sign of
`∮ (z − z_c) dθ`, say which singular points it covers), every other face
on cells of its box in space, and a face's margin is the least lower
bound over its cells. A link's reach skips its chain's supports, the
faces at its own two vertices that its window ends in (the faces the
band runs into, which predicate 6 and the surgery judge: a plane the
window is capped by, or a corner patch's support; a face at another
link's end is metered like any other), and any face on a support's own
stored surface (which can touch the reach only on its boundary). Two bands of one request are read together: a support of
the other chain is metered by what survives that chain's band, the other
band's new surface is metered as a face (but a concave band's surface
against a convex band's reach, whose removed material the concave region
overlapped), and two chains that meet at a vertex are left to the corner
predicate, or at a turn to the isosceles verdict and the mitre, where the
bands meet by construction.

Consumers: an island standing in a filleted cavity, on one shell and as
a second solid, and a thin revolved wall whose convex inner fillet
leaves through the far wall, all refused; the teapot lid's dome, a
sphere zone beside the flange's band, clear
(`crates/sweep/tests/blend_band_reach.rs`); an island filleted in the
same request as its cavity, its convex round receding from the void's
concave one, clear (`blend_per_shell_carry`); a void under a box's
convex mitre and an island beside a cavity floor's concave ones,
refused inside the bands and built clear of them
(`band_planar_mitre`).

## Walls: one per run

A *run* is a maximal chain of adjacent profile pieces the lowering's
cosurface verdict puts on one carrier — collinear lines, cocircular
same-turn arcs — however the author wrote them (a declared straight
continuation, a station kept on a side, a raw collinear polygon). Extrude
and revolve build ONE wall per run on every carrier kind, so no sweep mints
a same-key adjacency for a merge to undo. The one exception is a run that
is the whole closed loop of k ≥ 2 pieces (a circle split at authored
stations): it keeps its authored cuts (C12.5). A one-piece closed loop is
one wall whose strut is its wrap edge (D1), in every verb: extrude's
cylinder wraps `u` across it, a partial revolve's torus wraps `v`, a
loft's spline wall wraps `u`, and a full revolve's torus is one face
closed both ways, its meridian and its latitude circle each a wrap edge.
A station inside a run has no entity in the body: a cap carries the run as
one rim edge, as the wall is one face (`docs/DESIGN.md`, maximal edges),
and a partial revolve's run of on-axis segments is one axis edge. It
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
on WHERE the rim sits in its host's loop structure (a ring of several
arcs is the ladder, the face's own outer cycle is this), and each crossing's host
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
trim — against that support's trim, and every edge a cut-off leaves
on its end face, on either side, against the sliver it removes; every
outer-boundary edge a planar band's local carve leaves on a support is
metered against the strip it removes under predicate 2's
`fillet3_face_clearance`, the closed form of its sampled screen). A
merged cap that is an ANNULUS
therefore carves on both its rims, one call each. A full revolve's plane
wall is such a host as built: one face with no seam, a ONE-EDGE rim its
outer cycle or, at an annulus's inner circle, its ring, whose trim then
replaces that ring. Its one crossing takes the strut, and the trim is
minted so the host keeps its key (`lone_host_trim`); both rims of such
an annulus are annulus rims, and carve in one call. A CURVED single face
carrying every arc is authorable through `topo`'s `kef_describing` (a
cylinder wall merged over one seam meridian, the other restated as its
wrap edge), finishes, and refuses at the half-band gate on both routes
(`fillet_h5_r2_probes::a_finished_curved_single_face_carrying_both_arcs_refuses_at_the_half_band_gate`).

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
  D3). The end face gains the curve and loses the sliver between the
  curve and the old vertex on either side, cut away on the convex side
  and covered by the fill on the concave side; the two unrequested
  edges end at the feet; the old vertex goes. The ruled
  band's cut-off below ends the same way at its caps.
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
  down to L. Which holds is a margined verdict (`fillet3_turn_isosceles`,
  the two face angles at the vertex compared, and the two bands' feet on
  L): Zero builds
  the first, definite the second, the sliver band refuses. A Zero
  verdict is a coincidence decided from values, so the verdict records
  it (`BatteryVerdict::coincidences`, D10). No ball rests at a turn,
  so the mitre is G0 between the bands; it never competes with the
  patch, because the request count decides.

Chain G1 (predicate 4) is a classifier at a junction of two plane–plane
links: Zero, one band through a joint; definite, two chain ends at a
turn; sliver, escalated. Tangency is what lets one band run through a
junction, not a condition on the request. It keeps its name and its
place in the order, and still refuses at a junction involving a curved
link. The planar band carves locally, as the ruled band does: the end
face's rim edges split at the feet, the end curve `mef`'d across it, one
trimline per support; at a turn, L split at its foot, a strut out to the
trimlines' crossing on the shared face, and the mitre `mef`'d across one
band, the triangle it cuts off folded into the other.

What still refuses, typed `UnsupportedRunOut` with a detail naming the
shape: a curved end face (a fillet against a cylinder meets it in a
quartic with no stored carrier); a foot that lands inside a support
rather than on a rim edge (a band running into a wall or a step, and the
inner corner of an L-shaped rim, where the shared face's sector is
reflex); two cut-offs at the two ends of one rim whose feet on it cross
or coincide, metered before any mutation by `fillet3_cut_off_feet`
(feet apart only within the band escalate); a turn whose trihedron
`fillet3_turn_isosceles` decides definitely not isosceles
(`TURN_NOT_ISOSCELES`), which includes a chamfer at supplementary face
angles, whose two feet on L coincide; an end vertex
of valence other than three. A vertex whose three edges do not round
one way refuses `UnsupportedCorner { MixedConvexity }` whatever the
request names there: an edge cut off where its unrequested edges round
the other way, a turn whose two edges round opposite ways, or one whose
L does.

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
edges lie in one plane face — `CornerConfig::EndFace`, its section
picked by `fillet3_cap_transverse` (the cap normal's departure from
the ruling, in meters at the link's own extent, the lever the
shared-ruling hypothesis is metered at): Zero where the cap is
perpendicular to the ruling, definite where it is oblique. The band
ends in that plane's section of it, an exact stored arc about the
spine's crossing — of the circle of the band's radius, or of the
ellipse whose minor semi-axis that radius is and whose major is the
radius over the tilt's cosine, built through the ellipse door
(`Curve3::ellipse`), whose own verdict on its axes is the second
decision: the near-perpendicular tilts whose axes it cannot tell apart
escalate as `CapEllipse` on that door's `ellipse_axes_distinct`, with
the axes' difference `r·(sec θ − 1)` for margin
(`RunOutPolicy::CutOffAtEndFace`; `CornerConfig::policy` maps
the tag). An oblique cap whose three face normals are dependent — a
cap that nearly contains the ruling — refuses
`UnsupportedCorner { DependentNormals }` under
`fillet3_corner_independence`, as a plane–plane end does. The carve
(`blend/open/ruled.rs`, beside the planar band's
`blend/open/planar.rs`, both cutting off through `blend/open/end_face.rs`;
the rim phases stay in `blend/surgery.rs`)
mints no strut: the cap's two
rim edges are split at the trimlines' feet, the arc is `mef`'d across
the cap, one trimline `mef` per support carves its strip along the
ruling, and the crease's `kef` with two `kef`/`kev` pairs folds the
slivers in and retires the old vertices — the trimlines described as
the band's tangent contact with a curved support, the arcs as its
transverse intersection with the cap, on either material side. On
either side the cap loses the sliver between the arc and the old
vertex, cut away on the convex side and covered by the fill on the
concave side, and every other edge of the cap stays where it
was — the edges of its other cycles (a bore's ring, or the outer cycle
where the cut runs in a ring) and those of the cut cycle other than
the two rims it shortens (a notch in the outline). Each is metered
before any mutation, over its own window, against a region that
encloses the sliver: the disc about the spine's crossing out to the
farthest the sliver reaches, less the inside of the band's section
(the circle, or the ellipse), cut down to the half-plane towards the
old vertex that the sliver lies in and, on a round end, to the box the
sliver spans in the section's own axes — so a tilted section's long
major axis reaches no edge the sliver does not. A straight edge is
read point by point, at every scalar: it is clear when each of its
points is clear of one face of the region or another, the least over
the segment taken at its ends, the centre's foot and the faces'
pairwise crossings, all closed-form, with an ellipse's inside read
through the disc of its minor semi-axis. A crossing whose divisor's
enclosure meets zero is skipped rather than spread over the edge.
A circle or ellipse edge must clear one face whole.
The meter is
the same ring carry-through pass under the same
`fillet3_ring_clearance`; an edge not definitely clear of the region
refuses `RingClearance` at the cap
(`crates/sweep/tests/band_ruled_cap_ring.rs`). A
curved end face refuses typed.
Consumer: the rod with a flat milled along it (`cylinder ∖ box`), both
creases in one call, at the prism closed form `ΔV = A_section · L`
(`crates/sweep/tests/fillet_h7_transverse_cap.rs`), and the same rod
cut off by a tilted plane, each crease removing its section over the
length at the section's centroid
(`an_oblique_cap_cuts_the_ruled_band_off_in_an_ellipse`). A body whose
band an ellipse trims, of either band, measures through the certified
quadrature, tessellates, and takes a boolean beside or through the
band; with an operand wholly apart the boolean's containment door
refuses it (`work/contact/at-infinity-probe-measures-in-closed-form-only.md`,
pinned in `crates/sweep/tests/band_planar_oblique_fillet.rs`). The
CONCAVE ruled band — the material-adding side, the fill covering the region
under the arc — is pinned through the extrude door too: a rod's section
standing on a block's top edge (the sunk rod,
`crates/sweep/tests/review_fillet_h7_r1_probes.rs`, `ΔV = +2·A·L`). The
`CylinderCylinderCylinder` consumer — two parallel cylinders unioned at
a common ruling — has no body yet: the union refuses at the boolean's
curved-pierce door, and so does a block ∪ cylinder at its join lane;
that is the boolean's ground, not the band's.
