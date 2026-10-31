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
| Loft and path sweep bodies: extrude's topology over NURBS walls, slab by slab at v-degree 1 | `crates/sweep/src/loft.rs` (`loft_body`, `loft_profiles`, `sweep_body`, `slab_walls`) |
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
| The carve that takes a feature wholly inside a straight band's material, removed or filled: the grouping into features, and the walk that removes one | `crates/sweep/src/blend/consume.rs` |
| The open bands: the plane–plane band with its trihedral corners, the ruled band with its transverse cut-off | `crates/sweep/src/blend/open/planar.rs`, `crates/sweep/src/blend/open/ruled.rs` |
| Birth records (`BlendNaming`) the document layer turns into names | `crates/sweep/src/blend/naming.rs` |

## The band's reach (predicate 2)

A band changes the material between its supports and its blend surface:
a convex band removes it, a concave one adds it. The blend's result is
that change as a Boolean (C8): `S − R` under a convex band and `S ∪ F`
under a concave one, the region read from the supports' carriers and
the band's ends, so a feature in or across the region has the result
set algebra gives it whether or not the surgery builds it. Predicate 2
meters that region — the band's REACH — against every face of the body that is not
a support of the chain, in any shell, and reads each face as clear of
it, wholly inside it, or crossing it. A feature wholly inside the reach
goes with that material, as a feature wholly inside a Boolean's tool goes
with the tool: the carve that consumes it takes its faces, edges and
rings, and the verdict is logged in the blend node's verdict log (under
`fillet3_face_clearance` and `fillet3_ring_clearance`), so that a later
reader of one of its names resolves `Vanished` with the blend's verdict
as the cause (`PredicateFlip`). The carve is built at a straight
band's cut-off: the ruled fillet's, and the plane–plane fillet's and
chamfer's (below), a through-bore under a convex band and a bar
between the end walls under a concave one; and on the plane–plane band
a cup on one end face, a blind bore under a convex band and a pin under
a concave one
(`band_one_cycle_members::a_blind_bore_and_a_pin_go_with_the_band_at_every_scale`),
and, where the band is a whole chain of one link cut off at both ends,
a cup on one support's strip, a dimple under a convex band and a boss
under a concave one
(`band_strip_mouth_cups::a_dimple_and_a_boss_go_with_the_band_on_either_support_at_every_scale`).
The strip's cup is read in three steps. The ring carry-through pass
reads its mouth against the strip itself, across the edge, as a cap's
cycle is read against its sliver: wholly inside it, the ring joins the
request's verdict as a cycle the band may take; any other ring is
metered against the trimline as before, so one crossing it refuses
`RingClearance`. The clearance screen stands down on a ring that
reading reads inside or crossing, and only on its pairs with the edge
whose strip holds it, so the verdict decides it. The reach reads the
cup's faces in the band's exact core less that support's side, their
other edges in the whole core, so the cup lies on the material's side
of the support, as its mouth does. A wall ruled along the support's
normal pays nothing for its run along it where that section is convex
(the chamfer's) or the face lies on the support's side of the fillet
ball's centre, within which each line along the normal meets each of
the section's deeper sets in one segment; elsewhere it pays its run
across the spine, so a cup reaching past the ball's centre at an acute
dihedral refuses as a bound
(`band_dihedral_cups::an_acute_strip_cup_whose_wall_crosses_the_kept_disk_refuses_as_a_bound`).
Every other inside case refuses as one the rule defines and the surgery
does not build, at the meter that reads it: a void or an island with
no cap cycle `BlendError::WhollyInside`; a cup on the ruled band, whose
far faces have no exact core to be read in
(`band_one_cycle_members::a_pin_inside_a_concave_ruled_creases_fill_refuses_as_a_bound`),
or a hole whose mouth a concave fill covers, a bound where its wall
meets the end face's plane, as is a cup on a strip of the band's
opposite sign (a boss under a convex band, a dimple under a fill:
`band_strip_mouth_cups::a_cup_of_the_bands_opposite_sign_on_its_strip_refuses_as_a_bound`);
and a ring on a support that the strip meter reads neither inside nor
crossing (one touching the trimline), one beside a band ending at a
mitre, a turn or a corner, or a notch across an end wall, a boundary
feature of the support, at the clearance screen
(`FaceClearanceUncertified`), nearer the edge than the band sets back.

A face crossing the reach refuses as an unbuilt case too: building it
needs the band face cut against the faces that cross it, which the
surgery has no step for, and most of those surface pairs are outside
the Boolean's intersection table. Such a feature meets the clearance
screen (`FaceClearanceUncertified`), the reach (`FaceClearance`) or a
ring meter (`RingClearance`), and those recourses
(`FILLET3_CLEARANCE_RECOURSE`, `FILLET3_RING_RECOURSE`) name the
reorder beside a smaller blend and moving the feature clear: a feature
that came after the edge is applied after the blend instead, so
`fillet(subtract(body, hole))` becomes `subtract(fillet(body), hole)`.
The blend's edges are then re-picked on the earlier body, since a
selection written on the feature's output names the feature's node
(`From { read, of }`, N1 in `crates/editor-core/src/names/README.md`)
and does not resolve upstream. Where the feature did not split the
edge, the reorder gives the same body for a feature of the band's own
sign (a hole under a convex band, a protrusion under a fill), set
difference and union each commuting with themselves, and the other
body for the opposite sign. An edge the feature split re-picks as its
whole parent edge, so the band runs the whole edge rather than the
piece picked, and the reorder is a different request. A feature born
with the edge (a hole in an extruded profile) has no reorder, and the
sentence's condition says so. The reordered request builds where the
Boolean serves the band against the feature, a chamfer's plane strip
against a bore and either band against a bore parallel to its edge
(`blend_recourse_followability::the_ring_recourses_reorder_leg_builds_on_a_parallel_bore`),
and refuses at the Boolean where it does not yet, a fillet's cylinder
against a skew bore
(`blend_recourse_followability::the_reorder_leg_builds_where_the_boolean_serves_the_band`).

The supports' own
boundary features are judged first, by the battery's screen and the
surgery's exact ring and boundary meters; the reach runs after them,
before any mutation. A face not certified clear is read against each
region that is a band's material exactly (plane supports, or a whole
rim over a plane and a cylinder). It crosses the region when one point
of its own boundary lies inside and another outside, and refuses
`BlendError::FaceClearance` as a measurement (`bounded: false`). It is
wholly inside when every cell of its box that may hold a point of its
surface lies inside. Anything else is a bound. The readings, with the
cut-off end-face meter's below, make one verdict for the request, in
whatever order they are read: a crossing anywhere refuses as a crossing,
then a bound refuses `BlendError::FaceClearance` (or the end-face
meter's `RingClearance`) with `bounded: true`, and only a request whose
every face and end-face cycle not clear is read wholly inside refuses
`WhollyInside` (a tie on the margin names the least entity, so the
refusal is the same in every read order). The cell walk never reads
inside a face that touches the region's boundary, so on an exact core a
through-feature's wall, which meets the window's end plane, reads as a
bound whatever its rings read at the cap. A straight band's
reach reads such a wall through its boundary instead: the ruled band's
always, and the plane–plane band's for a wall its exact core read as a
bound (that boundary reading runs first, so a wall with a point in
kept material still refuses as a crossing). Each edge of the face is
certified inside the prism the band removes or fills one of three
ways: an edge of a cap cycle read inside the sliver, its depth
projected along the spine; an edge read inside the band's exact core
piece by piece, each piece of a line, circle or ellipse inside the box
its carrier's speed bounds it to (a plane–plane band's only, the ruled
band's bounds enclosing its material rather than being it); or a line
between two vertices so certified, which strays from the spine by its
run times its tilt. A face on a plane not parallel to the spine
projects one-to-one onto the section's plane, onto the region its
boundary encloses there, and the section is simply connected, so the
face lies inside at its boundary's least depth less the lines' stray;
a face on any other surface ruled near the spine (a cylinder's axis, a
plane's direction nearest it) lies inside when that depth also exceeds
how far its rulings stray over its extent (`reach::prism_inside`).
Where the caps' slivers are read as a chord end's exact triangle (the
chamfer's) the section is convex and the reading owes no slack: every
point of such a face is a convex combination of points at least the
least depth inside, and the distance from a convex region's boundary
is concave inside it, so the least depth is the reading. The prism's
slice in a cap's plane is the sliver the cap meter reads, so the two
readings agree on what dies by construction. A face the cell walk
reads inside a straight band's exact core lies in its prism too. A
request whose verdict is then inside groups the faces read inside into
features, joined by the edges they share, and the carve takes each
whole or refuses `WhollyInside` (`blend/consume.rs`). What it takes is
a through-feature between one band's two caps, an annulus of faces
(`V − E + F − R = 0`, `R` their rings), or a cup on one cap, a disk
(`V − E + F − R = 1`), each face read inside that band's material and
joined only to the others or to the cycles read inside on the caps; a
face's rings are bridged to its outer cycle first. The faces, edges
and vertices it takes are recorded as retired (`BlendNaming::dead`).

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
predicate, or at a turn to the turn's verdict and the mitre, where the
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
At v-degree 1 through three sections or more it builds one wall per
segment per slab (the stretch between two consecutive sections): the
skinned net's control rows are its sections, so slab `s`'s wall is rows
`s` and `s + 1`, a single ruled span in `v`, and the loft is `k − 1`
two-section lofts glued at the shared sections. A degree-1 wall through
three sections or more is not C¹ at its interior sections by its knot
structure, so the loft cuts there (DESIGN [topology]: "a wall keeps the
cuts its author wrote: the kernel adds none, except where a face or edge
carrier is not C¹ by its knot structure").
Each interior section is a ring of edges on the placed sketch segments,
the same carriers as the cap rims, described in the row of the slab wall
below; each strut is one straight boundary column per slab. An arc
whose row the other sections' sub-arc joints refine has no arc-rim image
there, and the loft refuses it (`LoftError::SectionRim`, recourse
v-degree 2;
`work/isotail/a-loft-whose-arc-bulge-varies-per-section-refuses-its-arc-rim.md`).
Slab walls lie on distinct spline surfaces, which no merge glues (a spline pair is
outside the carrier ladder's inventory), so a translated polygon's slab
walls stay distinct though coplanar. At v-degree 2 or more one wall
spans every section, its sections interior knots of a smooth net.

## The path sweep's frame `[sweep.frame]`

A path sweep places the same section at `n` stations sampled at
uniform shares of the path's parameter, and the frame each station
takes is the author's start frame carried along the path by parallel
transport: the rotation-minimizing frame, which turns about no tangent.
The author owns the start placement and nothing else; the carry is the
kernel's one law, because Frenet's frame is undefined on a straight run
and a fixed reference direction would read the world axes.

- **The discretization is double reflection** (Wang, Jüttler, Zheng
  and Liu, "Computation of Rotation Minimizing Frames", ACM TOG 2008).
  The step from station `i` to `i + 1` reflects in the plane normal to
  the chord `pᵢ₊₁ − pᵢ`, then in the plane that carries the reflected
  tangent onto `tᵢ₊₁`. Its error is fourth order in the step: on a
  helix of the tour coil's proportions (radius 0.14, pitch 0.08) the
  end roll after six turns is 9.2e-7 rad off the exact frame at 16
  stations a turn, 5.7e-8 at 32 and 3.5e-9 at 64.
- **The carry is a function of the `f64` path and the station count.**
  It is computed once, its decisions taken there, and every scalar lane
  composes it onto its own start plane; station 0 is the start plane
  itself, and a step whose two tangents are equal bit for bit is the
  identity with no rounding (the exact law's answer: its second
  reflection is its first).
- **The pivot is the path's start point `p₀`.** Station `i` is the
  start frame turned about `p₀` by the carried rotation `Cᵢ`, then
  moved onto the path: its origin is `pᵢ + Cᵢ·(o − p₀)` for `o` the
  start frame's origin, so where the sketch origin sits does not
  change the body.
- **A step refuses where it has no reflection.** Coincident stations
  have no chord (`SkinError::PathStationsCoincide`). Where the path
  folds back across the chord between two stations — the arrival
  tangent is the departure tangent mirrored in the plane normal to the
  chord, so the path leaves along the chord and comes back against it
  — the reflected tangent is opposed to the next one and the second
  reflection has no plane (`SkinError::PathFoldsBack`). A smooth fold
  sweeps with more stations; a kink that doubles back on itself (a
  hairpin corner) refuses at every station count, and only smoothing
  it cures it. Each is decided under the band, with its in-band twin
  `SkinError::PathUndecided`. A path whose end tangent is its start
  tangent reversed is not one of these: a planar U-turn sweeps, its
  end frame the start frame turned a half turn about the plane's
  normal, and so does a semicircle between two stations.
- **A section sits at its station's path parameter**: station `i` at
  `v = i / (n − 1)`, which no spelling of the profile and no roll of
  the start frame enters. The path parameter is not arc length on a
  rational path, and the skin's fit pays for it: the quarter-arc elbow
  (`sweep::test_support`, a rational quarter circle, 9 stations) falls
  1.03e-5 short of its Pappus volume, where stations at its chord
  lengths fell 3.8e-6 short.

What the law makes of a path is physics, not a choice per path: on a
planar path the frame turns only about the plane's normal, and closes
on a closed one; on a helix of angle `α` the section rolls against the
helix's axis by `2π·sin α` a turn — a rod bent without twist — about
196° over the tour coil's six turns. A section held to a direction
(a spring whose wire keeps a face toward its axis) is a frame held to
something other than the path, which is placement vocabulary and not
this law.

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
every blend trimline in closed form (but a plane–plane link's
trimline, against which a ring carrying a plane–plane link's
requested edge is metered at the strip instead, and a ring it reads
wholly inside such a link's strip joins predicate 2's verdict as a
cup's mouth), every other
outer-boundary edge
of a closed rim's supports — one requested in the same call at its own
trim — against that support's trim, and every edge a cut-off leaves
on its end face, on either side, against the sliver it removes; every
edge a planar band's local carve leaves on a support's outer cycle, or
on a ring carrying a plane–plane link's requested edge, is metered
against the strip it removes under predicate 2's
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
wrap edge), but each rim's two arcs then meet at a vertex nothing else
does, which tier 3 refuses at rest, so no blend door takes it
(`fillet_h5_r2_probes::a_curved_single_face_carrying_both_arcs_is_construction_state`).

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
  the shared support, whose axes therefore meet). When the two bands'
  feet on the unrequested edge L coincide, the mitre runs from the
  trimlines' crossing on the shared face down to L, and L ends there.
  For a fillet that is the isosceles trihedron: equal dihedrals at the
  two requested edges, the same fact as equal face angles at the vertex.
  A chamfer's foot lies `d / sin φ` along L, so its feet also coincide at
  supplementary face angles. Otherwise the band whose foot is further,
  `k`, is cut off past the mitre by the other band's face of L. This is
  the OVERRUN: band `k`'s own cut-off at that face (same carrier, line or
  circle or ellipse picked as for any cut-off, and same sliver), running
  from the mitre's end on the other band's trimline down to `k`'s foot on
  L. Every vertex there is trivalent, and the region that face loses
  beyond the other band's strip is metered as band `k`'s cut-off sliver
  (arm (c)). Which holds is decided on two closed-form readings, both
  per verb and both signed by which foot is further: the gap between
  the feet on L, and the overrun's chord from the mitre's end to the far
  foot, which is the one new short edge. Neither alone suffices for a
  chamfer: its chord is the gap times a ratio the corner fixes, at least
  sin φ of the near band's face angle but unbounded as the turn flattens
  toward a T, where a gap in band can carry a definite chord. Both Zero
  builds the mitre on L; both definite builds the overrun on the side
  the sign picks; anything else escalates, with a recourse that makes the
  feet meet (equal, or for a chamfer supplementary, face angles), moves
  them clearly apart (a larger setback, a more asymmetric corner), or
  requests L too for the patch. The overrun's section escalates as any
  cut-off's does: near a box corner, where the far band's cut-off face
  is tilted just off square to its axis, a fillet's ellipse stays in
  band out to `sec θ = 1 + escalate / r`, so moving the feet apart there
  hands on to that ellipse's own recourse; where the face is square the
  section is a circle and the overrun builds once the feet are apart. A fillet's overrun arc is tangent to the
  mitre at its end and to L at the far foot, so band `k`'s boundary runs
  G1 from the crossing to L. A Zero verdict is a coincidence
  decided from values, the two feet being one point, so the verdict
  records it (`BatteryVerdict::coincidences`, D10), and
  `Blended::coincidences` carries the rows to the document's coincidence
  door. No ball rests at a turn, so the mitre is G0 between the bands;
  it never competes with the patch, because the request count decides.

Chain G1 (predicate 4) is a classifier at a junction of two plane–plane
links: Zero, one band through a joint; definite, two chain ends at a
turn; sliver, escalated. Tangency is what lets one band run through a
junction, not a condition on the request. It keeps its name and its
place in the order, and still refuses at a junction involving a curved
link. The planar band carves locally, as the ruled band does: the end
face's rim edges split at the feet, the end curve `mef`'d across it, one
trimline per support; at a turn, L split at its foot, a strut out to the
trimlines' crossing on the shared face, and the mitre `mef`'d across one
band, the triangle it cuts off folded into the other. At an overrun L is
split at the far band's foot, the far band's cut-off `mev`'d from there
out to the mitre's end on the near band's face, and the mitre `mef`'d
across the near band, the quadrilateral it cuts off (the triangle and
the overrun) folded into the far one. A
convex cut-off takes a through-feature wholly inside the material it
removes, between its two end faces, as the ruled cut-off does
(predicate 2, above, and the ruled cut-off, below), under both verbs,
and a concave cut-off between two end walls absorbs a bar between
them wholly inside the material it fills, its cycles buried with the
walls' slivers, at `ΔV = +(A − A_bar)·L`
(`band_concave_bar::a_bar_inside_a_concave_bands_fill_is_absorbed_at_every_lean`).
A cup on one end face goes the same way: a blind bore at
`ΔV = −(A·L − V_bore)`, a pin at `ΔV = +(A·L − V_pin)`, an undercut or
a headed pin whose head stands off the end face included
(`band_one_cycle_members`). A cup's face that meets the end face is
read through the mouth: a plane or a cylinder (a countersunk, conical
mouth refuses as a bound), charged under the fillet its lines' slack
and, ruled, its box's diagonal times its lean off the spine, so a cup
leaning more than its depth pays refuses as a bound
(`band_one_cycle_members::a_leaning_blind_bore_goes_where_its_lean_is_paid`).
A cup with a handle, or one feature with two mouths on one cap,
refuses `WhollyInside`: the carve takes a disk on one cap or an
annulus between two
(`band_one_cycle_members::a_cup_with_a_handle_and_a_u_tube_refuse_wholly_inside`).
The fillet's sliver is the ruled cut-off's, between the arc and the
old vertex; the chamfer's is the triangle of the old vertex and the
two feet, and a cycle lies inside it when each of its edges stays
inside each of the three sides, its depth the least of those
clearances (`CapSliver`'s triangle; a point past a side is a witness
outside it). A bore or a square slot along the edge whose rings lie
inside each end face's sliver goes at `ΔV = −(A − A_feature)·L`, at a
perpendicular end face and at oblique ones, and its wall's name
resolves `Vanished` on the blend's clearance flip after a size edit
puts it inside
(`band_planar_consume::a_bore_inside_the_planar_bands_sliver_goes_with_it_at_every_lean`,
`edit_planar_carve::a_bore_a_size_edit_puts_inside_a_planar_sliver_vanishes_on_the_blends_verdict`).
A feature through a turn's overrun is not taken: the overrun's sliver is read through the far band's
triangle, a bound larger than the overrun, so its wall refuses as a
bound
(`band_turn_overrun_chamfer::a_slot_through_the_overrun_is_never_consumed`).
Nor is a void inside the union of a mitre's two bands: "wholly inside"
is read band by band, against each band's uncut prism rather than its
mitre-cut region, so a void inside both prisms refuses `WhollyInside`
(`band_planar_mitre::a_void_straddling_the_mitre_inside_both_prisms_refuses_wholly_inside`)
and one inside only their union crosses each and refuses as a crossing
(`band_planar_mitre::an_l_void_inside_only_a_mitres_union_refuses_as_a_crossing`).

The battery's other Zero verdicts are coincidences decided from values
too, and record the same way: a junction chain G1 reads tangent is a
`Tangent` row between its two links' edges, arriving then leaving, and
a curved pair's `fillet3_support_coaxiality` is a row between its two
supports: `Coaxial` for the axis a torus band is minted on, `CoRuled`
for the ruling a cylinder band is. A self-closed
link's own seam is one cell, not two, and records nothing.

What still refuses, typed `UnsupportedRunOut` with a detail naming the
shape: a curved end face (a fillet against a cylinder meets it in a
quartic with no stored carrier); a foot that lands inside a support
rather than on a rim edge (a band running into a wall or a step, and the
inner corner of an L-shaped rim, where the shared face's sector is
reflex); two cut-offs at the two ends of one rim whose feet on it cross
or coincide, metered before any mutation by `fillet3_cut_off_feet`
(feet apart only within the band escalate); an overrun's far foot past
L's far end, as for any cut-off; an
overrun whose near requested edge ends before band `k`'s trimline on the
shared face reaches it, the other foot of `k`'s sliver landing inside a
face (a short near edge at an obtuse shared-face angle, which refuses
where the overrun itself fits); an end vertex of valence other than
three. A vertex whose three edges do not round
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
feet coincide or not. Where they stand apart the extra curve is that
band's `EndArc` and its end `FootVertex`; both resolve
`Vanished` when the feet coincide, and an edit that carries the corner
across a coincidence moves them to the other band. Where the feet
coincide the turn foot has four edges, so blending L in a later call
refuses. At an overrun the turn foot is trivalent and L's later band
ends in band `k`'s face: a chamfer's is a plane, and it builds only
where L's foot lands on the overrun edge, which takes a much smaller
second setback; a fillet's is a cylinder, a curved end face. Every such
refusal's recourse is to request L in the same call, which builds the
patch.

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
(`Curve3::ellipse`). The departure is the one decision of circle
against ellipse: a definite one is the exact ellipse however
near-circular, its axes' difference `r·(sec θ − 1)` second order in
the tilt (`RunOutPolicy::CutOffAtEndFace`; `CornerConfig::policy` maps
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
concave side; a through-feature between the two caps, wholly inside
the material the band removes or fills (a bore on the convex side, a
bar between end walls on the concave side), goes with it, its cycles
leaving the caps and its faces the body (predicate 2, above; a blind
bore or a pin, whose far faces the ruled band has no exact core to read
in, refuses as a bound), and every other edge of the cap stays where it
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
`fillet3_ring_clearance`. The region contains the sliver, so it answers
only "clear": a cycle with an edge not definitely clear of it is read
against the sliver itself, bounded by the arc and the two rim pieces
(on the plane–plane chamfer's chord end, the triangle, above). A cycle with a sampled point certainly inside and
another certainly outside crosses, and refuses `RingClearance` as a
measurement, the inside point's depth (the cut cycle always has points
outside, its rims running on past the feet). A cycle other than the cut
one that misses the arc and has a point inside lies wholly inside, as
predicate 2's verdict reads it; anything else is a bound, `RingClearance` with `bounded: true`
(`crates/sweep/tests/band_ruled_cap_ring.rs`). A bore wholly inside the
D-rod's sliver, or a keyhole crease's, goes with it at
`ΔV = −2·A·L + π·a²·L`
(`band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_the_closed_form`,
`review_band_ruled_ring_probes::a_bore_in_a_keyhole_creases_removed_sliver_goes_with_it_at_the_closed_form`),
and its wall's name, carried by the fillet before a radius edit that
puts it inside, resolves `Vanished` on the face-clearance flip after
(`editor-core` `edit_ruled_carve`). A
curved end face refuses typed.
Consumer: the rod with a flat milled along it (`cylinder ∖ box`), both
creases in one call, at the prism closed form `ΔV = A_section · L`
(`crates/sweep/tests/fillet_h7_transverse_cap.rs`), and the same rod
cut off by a tilted plane, each crease removing its section over the
length at the section's centroid
(`an_oblique_cap_cuts_the_ruled_band_off_in_an_ellipse`). A body whose
band an ellipse trims, of either band, measures through the certified
quadrature, tessellates, and takes a boolean beside, through or wholly
apart from the band (`crates/sweep/tests/band_planar_oblique_fillet.rs`). The
CONCAVE ruled band — the material-adding side, the fill covering the region
under the arc — is pinned through the extrude door too: a rod's section
standing on a block's top edge (the sunk rod,
`crates/sweep/tests/review_fillet_h7_r1_probes.rs`, `ΔV = +2·A·L`). The
`CylinderCylinderCylinder` consumer — two parallel cylinders unioned at
a common ruling — has no body yet: the union refuses at the boolean's
curved-pierce door, and so does a block ∪ cylinder at its join lane;
that is the boolean's ground, not the band's.

## Conventions every sweep keeps

Moved from DESIGN `[topology]` on 2026-10-17, verbatim. Bracketed ids (`[topology]`, `[coincidence]` …) are `docs/DESIGN.md`'s clauses.

- **The minimal sphere at rest is V2/E2/F2**: tier 2's valence-1 ban
  makes a one-band wire sweep unrepresentable, so axis-touching full
  revolves sweep two π-bands and poles have valence 2. The split is
  owed only by CURVED walls, whose pole or apex must keep valence 2. A
  full revolve emits each PLANAR wall as one face: a disc's centre is
  interior to the face, not a vertex, and no revolve returns two
  adjacent faces on one plane.
- **The extrude/full-revolve hole asymmetry is structural** (moved from DESIGN):
  extruded holes are cap-to-cap tunnels
  (one shell, genus); full-revolve holes, and the gaps between a full
  revolve's axis runs, are cavities (a shell each, through
  `topo::insert_void`); partial revolve is extrude-shaped and carries
  holes in its one shell. Sweeps emit single-shell primary boundaries.
