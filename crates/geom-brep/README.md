# geom-brep: curved geometry and offsets

`geom-brep` is the layer between the evaluators (`crates/geom`: analytic
surfaces, NURBS curves and surfaces) and the arena store (`crates/topo`:
the B-rep, a solid represented by its boundary faces, edges and
vertices). It defines what an edge's geometry *is* (an `EdgeDescription`,
never a bare curve), how a concrete curve earns the right to stand in for
it (an `EdgeCurve` is constructible only through certification against
its description, so an uncertified carrier is unrepresentable), and the
geometric classifiers topology stands on: dihedral wedges, tangency jets,
the surface-pair intersection table, surface–surface intersection (SSI)
by marching, per-half-edge pcurves, certified mass properties, and the
analytic and fitted offset surfaces that `shell` consumes. The thesis,
stated once: exact closed forms where they exist (DESIGN.md D3),
intensional descriptions with certified caches where they do not (D2/D4),
and every completeness claim backed by an enclosure rather than by an
algorithm's diligence. Marching proposes; certification decides; nothing
a marcher does is trusted. Every decision is a named margined predicate
through `geom_core::Decide` (DESIGN.md Q1): definite, coincident, or an
escalated typed refusal, never a raw comparison.

## Where in the code

| Decisions | Lives in |
|---|---|
| C1 locus ladder | `crates/geom/src/curves.rs` (`Curve3`: Line, Circle, Ellipse, Spiric, Nurbs); `crates/geom-brep/src/intersect.rs` (`Rung`); `crates/geom-brep/src/intersect/conic_arc.rs` (the plane×cone parabola and hyperbola arcs) |
| C2, C3 SSI and its certificate | `crates/geom-brep/src/ssi.rs` + `ssi/{march,certify,exhaust,enclose,jet,system,boundary,section,ends,pair,four}.rs`; `crates/geom-brep/src/intersect/section_bounds.rs` (the bounds a classified pair's section states, which retire the tube) |
| C4 pcurves | `crates/geom-brep/src/pcurve_cache.rs` (value, certificate), `pcurve.rs` (conic constructors), `crates/topo/src/pcurves.rs` (storage, minting, branch walk); description form in `description.rs` |
| C5 dispatch table | `crates/geom-brep/src/intersect.rs` (`route`, the section functions); for curve pairs `crates/geom-brep/src/curve_pair.rs` (`method`, `meet`) and `curve_pair/{span,pose,polish,walk}.rs` |
| C6 structure certified for every member, values in the scalar, generic certification | `crates/geom-core/src/spline/`, `crates/geom/src/curves/fit.rs` |
| C7 tangency | `crates/geom-brep/src/tangent.rs`, `enters.rs`, `conic_bend.rs` (a conic edge's side of a plane one order down), `locus.rs` (the closed-form tangent locus, the `Tangent` witness lane); marks in `crates/topo/src/validate.rs` (`ContactMark`) |
| C8 fillets | `crates/sweep/src/blend/` (see `crates/sweep/README.md`) |
| C9 certification arithmetic | `crates/geom-core/src/interval/certification.rs` (the certification doors), `interval.rs`, `spline/hull.rs`, `spline/compose/{tensor,patch}.rs`; the importers in `scripts/gates/certification-doors.sh` |
| C10 BVH | `crates/bvh` |
| C11 NURBS substrate | `crates/geom/src/{curves,surfaces}/nurbs.rs`, `curves/fit.rs`, `*/projection.rs`; lofts in `crates/sweep/src/{loft,skin}.rs` |
| C12 consumers | `crates/topo/src/splitting/`, `boolean/`, `merge_faces.rs`; `crates/mesh/src/curved.rs`; `crates/geom-brep/src/props/quad/`; `crates/geom-core/src/linalg/{svd,lsq}.rs`; where a chart singularity lies against a face, `crates/geom-brep/src/chart_singularity.rs` |
| O1 analytic offset mint | `crates/geom-brep/src/offset.rs` |
| O2 approximating surface | `crates/geom/src/surfaces/approx.rs` (`Surface::Approx`) |
| O3 meters and fit | `crates/geom-brep/src/offset_meters.rs`, `offset_fit.rs`, `patch_bound.rs` |
| O4 shell | `crates/topo/src/shell.rs`, `boolean/voids.rs`, `replace_face.rs`, `offset_together.rs`, `offset_axial.rs` |
| O5 validator posture | `crates/topo/src/validate.rs` (`OffsetFitLane::recertify`) |

## Curved geometry (CURVED-DESIGN C1–C12)

### The locus ladder and its certificate

**C1 — Intersection loci live on a three-rung ladder.** The *carrier*
of an `Intersection` edge (the 3-D curve cached against the intensional
description `{pair, selector}`, D2) is, by surface-kind pair and most exact
first: rung 1, closed-form `Line`/`Circle`; rung 2, the exact conic
`Curve3::Ellipse` (tilted plane×cylinder, tilted plane×cone,
equal-radius cylinder×cylinder)
and the exact quartic `Curve3::Spiric` (the axis-parallel plane×torus
section, one oval in the torus's own minor angle — minted by
`plane_torus_section`'s axis-parallel arm, which the offset-axial
door's rim mint calls; a plane through the axis cuts the more exact
`Circle`, and `Curve3::spiric` refuses it); rung 3,
a pair with no closed form (a spline operand), whose carrier is
evidence of any kind — a fitted cubic `Curve3::Nurbs` where the kernel
marched it, a closed form where a file stated one or recognition
promoted one — carrying the C2 certificate. The rung is the pair's, read off the C5
table, never the carrier's: what a carrier owes (C2) depends on its
pair's pose and its selector (D2), and not on its kind or on who minted it. The kind inventory is closed at the
kinds above: a section that has a closed form and no kind is minted
exactly into the NURBS container. A plane×cone parabola or hyperbola is
such a section, minted from the pair in closed form as a
rational-quadratic arc (weights `(1, w, 1)`, `w = 1` the parabola,
`w > 1` the hyperbola), never marched. The line, circle and ellipse
carriers stay kinds; they round-trip to rational-quadratic NURBS only as
an export/tessellation form. There is no polyline rung.

**C2 — The certificate of a witness-selected `Intersection` carrier has three limbs.**
A structural selector (D2: a curve known in advance, such as a row or
column of a swept or skinned wall that is the edge on both surfaces)
owes no limb 3: it is certified through each surface's chart-image
certificate, its on-locus residual and sup-norm bound read per surface.
Limbs 1 and 2 are owed by every carrier, of every kind, however it was
minted. Limb 3 is owed unless the pair's section function, for this
pose, states the two numbers the tube would prove: a lower bound on the
sine of the surfaces' crossing angle over the carrier's stretch of the
section, and the clearance of the tube of radius 8ε over that sine
around the stretch, out to 16ε over it, from every other piece of the
two surfaces' ε-overlap (the section's
other components, and any near-miss region where the surfaces approach
without crossing) (`intersect::tube_bounds`, read by
`edge_nurbs::analytic_rung3`). Limb 2 within ε of both surfaces then puts the
carrier on the named component, which the witness mid-pin identifies.
(1) On-locus residual at the fixed `CERT_SAMPLES` schedule: `|f(C(t))|`
in metres for an analytic operand (`implicit.rs`); for a NURBS operand
`|C(t) − S(u*,v*)|` at the projected foot point; any point of the
surface bounds the distance from above, so a foot is owed no limb of
its own.
(2) Sup-norm honesty between samples, by control-coefficient hull bounds
in certification arithmetic (C9). `geom_core::spline::compose` composes
the implicit form with the carrier, and `compose::tensor` encloses
`S(P(t)) − C(t)` as one composite for a NURBS operand so the
cancellation that is the whole content of the claim survives into the
bound. The implicit form converts to metres by a constant factor for
plane, cylinder and sphere. On the cone and the torus the metres form
is a root of a polynomial, which no polynomial composite carries, so
the conversion is a per-span lever instead: a scalar bound on the
carrier's distance from the axis, whose one square root is of a bound,
in the scalar's own outward-rounded arithmetic. The edge certificate
reads it per part of the carrier's knot spans, through one reader for
every carrier (`pcurve_cache::limb2_offset_sup`): the carrier's net
(`carrier_net::framed_carrier_net`: a spline is its own net, a closed
form its enclosing net), framed at each operand in the scalar's
arithmetic (C9's backend at the numeric channel), so the bound rounds at the part's own scale and never at
its distance from the world origin,
and on the cone a part the lever does not decide positive (one reaching
the apex) reads `√|f|` instead: `f` is the product of the distances
from the meridian half-plane's two generator lines, both on the implicit
cone. The search's limbs convert the torus per span of the carrier
too, bounding the residual its limb 1 reads,
`m = F/(2r·(A + 2Rρ))` for the quartic `F = A² − 4R²ρ²` with
`A = |q|² + R² − r²`, by `sup|F∘C|` over `2r` times
`inf A∘C + √(inf A∘C² − sup F∘C)`, `A∘C` its own composite over the
same spans and floored by `R² − r²` (`ssi::certify`); the cone has no
such conversion at the search, which is why its rung-3 arms are
unretired. At a
parameter-family scalar the limb is decided by the scalar like any
margin (E12): its symbolic identity is the residual of the carrier's own
description against the operand, per span, with the carrier's parameter
left free, formed by the limb from the operands' own values, and its
numeric channel is the composite's enclosure `[floor, upper]`; an
identity certifies every member of the box at every parameter of the
carrier. A curved closed form is read first on the dyadic spans of
its angle parameter (multiples of `τ/2ᵈ`), each enclosed alone at a
higher Taylor degree and composed alone, an uncleared span halved
into its dyadic halves (`pcurve_cache::limb2_quick`). A span may
reach past the edge's ends, since a bound over a span holding a piece
of the window bounds that piece. A span past them that still halves
its bound keeps halving, its halves past the ends dropped; one whose
halving no longer halves its bound is read again clipped to the ends,
and halved clipped from then on. That reading only passes, and only on
cleared parts that cover the edge's outer ends with no gap; a span it
cannot halve further ends it. Where it does not pass, the rounds below
read the carrier as though it had not been taken, and every refusal is
theirs. A span's reading is a function of the carrier, the operand,
the chart (a cone's nappe, taken from the window's middle) and the
span, so edges sharing a carrier, an operand and a nappe read their
shared spans once. Where the bound does
not clear the band, the composite's uncleared spans are halved round by
round, each piece cut from its Bézier segment's own row so no cut
compounds another's width, and the rounds stop (`round_floored`) once
the bound is within the stop's factor (`ROUND_FLOOR_FACTOR`, 16) of the
net's own rounding (`CarrierNet::rounding`, its
controls' width before any span's remainder, which no halving
removes), that width is itself that factor above the band, and a
round no longer halves the bound (the rounding is the whole net's, not
the window's parts', so the stop can end the rounds early: a cost in
liveness, never in soundness, since every stop reads the lowest hull):
the limb passes once the bound
clears;
refuses on the composite's certified value at a break where that lies
past the band's zero, since no bound over a span holding the break can
clear; and otherwise refuses on the lowest bound any round reached
where the crate's stop rule ends the rounds (`crate::descent`, in its
own units: `SSI_HULL_WORK` spans recomputed over the rounds,
`SSI_HULL_CUTS` breaks added) — so a first composite already over
`SSI_HULL_WORK` spans is never subdivided and refuses on its first
hull, and a later round whose bound is NaN stops on the lowest certified
bound before it rather than escalating. At a point scalar, limb 2's
reading is an upper bound, and an upper bound shows no miss: a bound that ends above the band with no
measured miss past it refuses as a kernel approximation limit
([tolerance.recourse]; `AnalyticRung3Refusal::KernelLimit`), carrying the bound, the extent it was read over
and the carrier's own enclosure width there, so the one story can name
the input's extent where the bound is near that width (within
`KERNEL_LIMIT_ROUNDING_FACTOR`, a constant of the story's own, not the
stop's) and the width is
the extent's own rounding, and claims no cause otherwise; it is never
an ambiguity of the input and never a definite miss. Only the measured
value at a break refuses as a miss. A line against a plane is distant
from it by a function affine along it, whose sup is at an end, so a
reading that neither clears nor shows a miss there is decided on the
two ends' measured distances, `(p − o)·n` over `|n|` in certification
arithmetic at every scalar (the data and the ends' parameters lifted
through their certified brackets), read by whether an end's enclosure
excludes zero. An end whose floor is above zero is a measured,
certified non-zero distance: within the zero band it passes, a floor
past `Kε` is a miss, and otherwise — in the ambiguity band or
straddling its edge — it escalates on that measured value (`Escalated`
on `HullValue`), the input ambiguous at the tolerance. An end whose
floor is at or below zero (it cannot exclude the carrier lying on the
plane) passes if its upper end is within the band, and otherwise is
only an upper bound, which says nothing about the input: it adds
nothing to the composite's stall, which is the verdict (the kernel
limit at a point scalar, `Stalled` at an enclosure one). Both ends
passing certify the edge; a miss, then an escalation, refuses. At an enclosure scalar a bound that
stalls is undecided at that input (`AnalyticRung3Refusal::Stalled`),
and over a parameter box the drive bisects it (E6); the refusal carries
the enclosure and claims no cause and no terminality. A floor measured
inside the band is not a stall: it escalates on the measured value, the
classifier's terminal bit kept. Against
an analytic operand
the value at a break is limb 1's residual, a measured miss: at the edge
certificate it is the carrier's distance from the surface, bounded
below by `|f|` over an upper bound on the factor that divides it to
metres, read off the break's point. Against a NURBS operand it is the
bound's own quantity at a point, a bound on the miss.
(3) The uniqueness tube: over a chain of boxes of certified radius
around the carrier, the enclosure of `(∇f₁ × ∇f₂)·e` excludes zero, so
by a mean-value argument each slice holds at most one solution, and each
connected piece of the solution set in a box ends on the box's boundary
at two points. The solution set in the chain is one arc, and it spans
the carrier; the proof is the same at every door, a search's and an
edge's at rest. For a carrier between two analytic surfaces the three
limbs are the edge certificate's own (`edge_nurbs::analytic_edge_limbs`,
through the scalar's certified lane): limb 2 against each operand, its
distance from that surface over the edge's interval, whatever faces
store pcurve rows (a planar face stores none), which implies limb 1;
and the tube over the same interval, or over the whole carrier where
the carrier is rational with unequal weights (no piece of it has `f64`
weights at an enclosure scalar). Certification is a property of the
scalar's type, never an argument a door may omit. An edge door is two
functions: minting the canonical description is construction, generic
over any scalar that decides; certifying it (the sampled checks and
limbs 1–3) is bounded on the certification trait and takes its lane as a
plain argument. The scalar's policy names what its edges carry: a
certifying scalar's door mints and certifies; a tangent-transport
scalar's door (DUAL-DESIGN DL1, DL3) only mints, and no code path at that
scalar can call a certifying function. A dual body carries no
certificate: what a dual measures is read against the input-identical
`f64` evaluation, which holds it (DUAL-DESIGN DL3). At `f64` the door certifies in full and rest
re-derives the same certificate. An edge with an operand outside the
tube's enclosure roster (`ssi::enclose::has_implicit_enclosure`) refuses
the tube by naming that operand (`AnalyticRung3Refusal::NoTubeEnclosure`)
once limb 2 holds and the pair's section states no bounds for the pose. Each box, cut to the wall's knot rectangle (and to the
ℝ³ slab where a search clips to one), holds exactly one piece: two
simple solutions on its boundary, or a stretch of its boundary on a
side of the wall's domain that the boundary pass reads within ε of the
plane with no piece of it clear (the side's own section and the clear
test C3 decides a side by), whose cover puts every solution in the box
within ε of the side. That piece meets the slice through every knot
two boxes share and the slices through both ends of the carrier: an
arc's end counting also where a zero of the locus is certified within ε
of the carrier's end, on a slice beside it, and a side's where the end,
moved across onto the side, lands on its stretch at a point the
boundary pass reads within ε of the plane and not clear. So the arc
spans the carrier (chart edges walked in runs of one sign or monotone,
ℝ³ faces by Krawczyk). A
rung whose chain is a graph but not one arc gives way to a narrower one.
With none certified, the narrowest rung probed speaks: its band verdict
where it straddled, `SsiError::TubeNotOneArc` with what it found where
it was a graph but not one arc. For
plane×NURBS the chain is the
wall pcurve's per-span windows, padded along each chart axis by the
radius over that axis's chart speed (minted once over the wall's domain,
refusing a zero or non-finite axis by name), and the enclosure is the
chart form `∇φ·e⊥ / ‖chart stretch‖`. For NURBS×NURBS the chain is
of boxes in the ℝ⁴ state space, each the product of the two pcurves'
padded windows over a run of consecutive spans (at most
`PAIR_TUBE_BOXES` of them, the same windows hulled), and the enclosure
is the determinant of the 3×4 Jacobian of `S₁(u₁,v₁) − S₂(u₂,v₂)`, each
column scaled by its chart speed, stacked over the run's scaled unit
tangent `e` (orthonormal charts crossing at θ read `√2 sin θ`); each
box, cut to the product of the two knot rectangles, holds one piece
where its eight faces, three equations in three unknowns each, hold
exactly two solutions, each isolated by Krawczyk (a face piece is first
cleared by the mean-value form along the coordinate axes and each
chart's normal), and an end is reached where the slice through it holds
a solution or a box about it, `ε/8` per axis at that axis's chart speed,
holds one on its boundary (`ssi/four.rs`). At rest the edge certificate
derives both charts' images of the carrier from the fixed foot schedule,
decides each interior sample's two normals transversal, and certifies
the pair with this tube (`edge_nurbs::nurbs_pair_limbs`). The certificate records the tube
by kind, a radius in metres or the per-axis chart pad (`SsiTube`), and
the exhaustiveness accounting banks exactly the region it records.
A component outside the chain is C3's exhaustiveness obligation, a
separate theorem. Refusal
is typed, never a retry loop: an enclosure that does not clear the
band at any rung escalates (`ssi_tube_transversality`,
`SsiError::TubeStraddles`). Two branches passing within the band of
each other is a genuine sliver of the operand pair, and escalation is
correct for it, F6's ladder speaking. The enclosure's own remaining
slack can straddle too, and escalates the same way. Hull bounds are an entry requirement: no schedule-max-only
certificate ever reaches an at-rest body, and the tube is required for
every fitted `Intersection`, not only where several branches were found.
The witness is `carrier(mid)`, minted from the cache the schedule sees.

**A check's name is the taxonomy's to say, not a rendering each sentence
picks.** `CertCheck` carries a `Display` on its declaring row, written
as an exhaustive match so a check the taxonomy gains has no word until
someone writes one, and `CertifyError`'s three check-naming arms render
through it. Each word is the phrase a person would write ("the
between-samples sag bound"), not the variant identifier: these rows are
not doors anyone calls, so the identifier buys a reader nothing that the
typed field does not already give a program, and every neighbouring arm
of the same `Display` is English prose. The word also carries the KIND
of quantity the check meters, because the sentence cannot: not every
check that reaches the definite arm meters a residual (a sup bound,
`CertCheck::bounds_a_miss`; a parallelism defect), and the exhaustive
match is where each one's kind is written. The
censuses beside the taxonomy hold the words apart and hold each away
from its identifier: two checks saying one phrase makes a refusal
ambiguous about what it refused, and a phrase that IS the identifier is
the rendering this contract replaced.

### Surface–surface intersection

**C3 — March, then certify; the stepper is trusted for nothing.** No
per-step predicate can prove "no other branch within reach" from local
data, so none is asked to. `ssi/march.rs` is a candidate generator
(Hoffmann §6.2: third-order local approximant from the underdetermined
jet system solved by SVD, Frenet choice of free coefficients, Newton
refinement; all f64, libm-only, fixed iteration order). Each side
enters the section through the one exact description it has: its
implicit residual where it has one in metres (plane, cylinder, sphere,
torus), else its chart (NURBS). The pair is one system on
`ℝ³ × ∏ Dᵢ`, the point eliminated wherever a chart side exists, so the
trace shape is decided by the sides: two residuals, an implicit pair in
ℝ³ (2×3 SVD; `cylinder_sphere_ssi`); a residual and a chart, one
equation `f∘S = 0` on the chart's knot rectangle (1×2); two charts, a
parametric pair in ℝ⁴ on `G₁(u₁,v₁) − G₂(u₂,v₂) = 0` (3×4 SVD;
`ssi::Section`), from which the 3-D curve and both pcurves fall out as
projections of one traced object on one shared parameter. The trace's
**domain** is the compact part of its state space: a chart side
contributes its knot rectangle; two residual sides have no compact set
of their own, so the pose's ball where one is proved, cut by the
operation's region, bounds them, and the region alone where none is. No face's trim sizes a domain. A
component is closed, or ends on the domain's boundary (a rim end on a
rectangle's side, or a cut end on the region's), or ends at a node. A branch jump is a certificate refusal. The
stepper guards the step where it mints it: no step is longer than the
march domain's diagonal, and a march speed that is not positive and
finite, a step that is not finite or does not move the state
(`SsiError::StepUnusable`), or one that collapses into the band
(`StepCollapsed`) refuses naming the speed. The step's rungs read the
curvature at the step's start against ε, and no extent caps them. Each
step's first try is at most twice the last step kept, and a try is kept
where its predicted state's residual is at most ε plus the settle
tolerance; a try whose end leaves the domain (the wall's rectangle, or
the caller's ℝ³ slab) has its midpoint, and its last predicted state
inside the domain, on the locus to the same reach instead. Otherwise
the step halves and predicts again, down to the band, where
`ssi_step_progress` refuses. Every try, kept or halved, counts against
`SSI_MAX_STEPS`, and a step whose halving would pass it refuses there.
The residual test and the restart set how many samples the march gives,
with the rungs; the certificate decides how many more a carrier gets:
where limb 1 or 2 refuses the fitted
carrier, definitely or in band, on a margin that is not poisoned, it
names the spans it refused, every gap between samples that a refused
span meets is halved with one gap on each side of it, the new sample
settled onto the locus wherever it settles, and the carrier is refitted
and certified again (`ssi/refine.rs`). A midpoint that does not settle
is read by the march's transversality decision at the gap's chord
midpoint, the state Newton started from: in the band the surfaces are
near tangent there (`SsiError::TransversalityBand`), undecided the
decision escalates (`SsiError::Escalated` on `ssi_transversality`), and
either is the refusal, with the clearer angle's lever; clear of it, the
gap is one refinement cannot halve. Where the refused residual stops
falling over two consecutive rounds (two definite margins, the later no
smaller, or two in the band, whatever their values), limb 3 is asked
once of the carrier, and its refusal stands: a carrier across two
branches is one no halving answers. The door that returns an
uncertified triple does not ask it, and refines as long as limbs 1 and
2 locate a refusal. A gap is halved only while half
of it clears the
band, and no round gives the branch more steps (gaps between samples)
than the march may take (`SSI_MAX_STEPS`, one wall per branch on its
steps, whether marched or split by refinement: a named resource wall
like the cell budget, not a derivation); where refinement can go no
further, the certificate's refusal stands, naming where refinement
stopped (gaps whose half falls in the band, midpoints that did not
settle, midpoints that settled outside the domain) and the limb and
margin each earlier round refused (`SsiError::RefinementExhausted`). At
the wall, a refused residual that stopped falling over the last two
rounds ends in the tolerance as the arithmetic's floor; one still
falling ends as the curvature-held march's does.
A certified ℝ³ analytic branch is refined once more where limb 3
certifies it narrowly (`refine_for_tube`): the tube's rung is the
whole chain's, so where a chain box wider than the ladder's next rung
up is no graph padded by it, the gaps that box meets are halved the
same way, the carrier is refitted and certified again, and the finer
carrier is kept only where its tube is wider. These rounds keep a
certified carrier throughout and refuse nothing: they stop where no box
holds the tube, nothing halves, the step wall would be passed, the
finer tube is no wider, or the finer carrier does not certify; each
kept round climbs a rung, so there are at most `SSI_TUBE_RUNGS`.
Before any march, the
plane × NURBS lane decides its own domain boundary, the wall's knot
rectangle, against the plane, one side at a time
(`geom_brep::boundary_section`: plane × one boundary curve of the
wall). A side
either lies within the band of the plane, or meets it at isolated
crossings, each found to the sweep floor and decided transversal along
the side, or refused as a graze, the locus tangent to the side, naming
the side (`SsiError::BoundaryGraze`). A side within the band is
decided over a strip beside it where the wall's slope across it is
one-signed: nothing where the strip is clear of the plane (its plane
distance one-signed and the wall moving further that way inward; a
piece of the side whose Bernstein hull straddles zero is halved until
one-signed, the side reads in band at opposite signs or a halving that
does not narrow a hull, and it refuses past its halvings,
`SsiError::SideSignBudget`), a `Side` region where the locus is
coincident with the side (below); where that
slope does not clear the band the surfaces may be tangent along the
side, and it refuses toward C7 (`SsiError::BoundaryTangent`); where no
strip has it one-signed, or none holds the locus's certified zero set
inside it, the side's own crossings decide it. A corner within the band
on no side decided over a strip is classified by the plane distance's
two inward partials over a corner cell: a branch starts at it where
they are of opposite inward sign; where they are of one sign the locus
leaves the domain there, and the corner is a `Corner` region where the
locus is coincident with it, nothing where the corner's distance has
that sign too, and otherwise no region, its roots ordinary crossings.
In the band the
pass does not pick a side: a region asserts no topology. A `Corner`
region certifies that its cell's solution set is at most one arc lying
within `reach` of the corner, a `Side` region that its strip's solution
set lies within `reach` of the side. A region is reported exactly where
the locus is coincident with its corner or side: its certified zero set
stays inside its cell, so an arc in it ends on the domain's sides, and
is certified to lie within ε of the corner or side, its `reach` that
certified distance plus ε. Otherwise no region is reported, and the
roots decide, the arc traced between them as any branch is. A side
whose strip holds the locus's certified zero set but not within ε of it
keeps its interior roots, each decided along it, and leaves a root at
one of its corners to the other side through that corner; a corner
both of whose sides are such is classified as a corner on no decided
side, and their roots there are its. On a wall narrower than the
widest strip every side within the band whose slope across it clears
the band is such a side, its strip the whole domain. A reported
region's cell holds no zero beyond its certified zero set, so a root in
the cell is the region's, and every other root is kept. Whether a
vertex lies on a face stays the
consumer's decision, and the exact empty answer stands outside the
domain. Every root is settled onto both surfaces, or refuses
`SsiError::EndNotOnLocus`. The crossings are the only ends a branch has
on this lane, and between them the simplest candidate is tried first:
from a crossing, the Hermite cubic to the nearest crossing not yet
used, through their tangents, one span exact at both ends, its two
states taking the march's transversality decision. Where anything
refuses it (that decision at either end, the march tolerance, or the
certificate), a march runs from the crossing to the unused
crossing on the side it leaves, the one nearest where its last step
meets that side, its step the curvature's against ε, the carrier's and
the bend of the branch's own chart path alike, kept by its residual as
every step is; a
march that leaves where no crossing matches refuses as the march's
limit (`SsiError::CrossingUnmatched`). The plane's window must hold the
wall's image, or the door refuses (`SsiError::WindowShortOfWall`), so a
march ends only at the knot rectangle. The ℝ³ lane still ends an open
branch at the caller's slab by its boundary search
(`ssi_branch_open_end`), and the slab is not
geometry (`work/ssiedge/ssi-r3-slab-is-not-geometry.md`). Neither candidate
is trusted, its pairing of crossings included: the certificate decides
each on the chart lane, so limb 3 proves its tube one arc in the knot
rectangle, and a cubic to another branch's crossing either leaves the
locus, which limbs 1 and 2 refuse, or holds more than one arc in its
tube, which limb 3 refuses. The Hermite is one span, not a polyline of
samples, so refinement has nothing to halve in it: a refused Hermite is
marched, and the march's carrier is refined as any is. The fit is given
the cubic's four samples where the march gave fewer, by halving gaps at
their midpoints settled onto the locus, read where they do not settle
as refinement reads them. Where nothing halves and every gap's half
falls in the band, the polyline is a sized refusal in its length
(`SsiError::ShortBranchUncertified`), naming the lever of what bounds
the branch, the wall or the caller's slab. Otherwise it stops with
nothing to halve, counting what each gap's midpoint did: on the ℝ³
lane as `SsiError::RefinementExhausted`, the slab's limit as its
refusal (`SsiError::TraceUnresolved`), on the plane × NURBS lane as
`SsiError::MarchShortOfFit`, the march's limit. Where neither candidate
certifies, the march's refusal stands, with two exceptions. Where the
march's own states were too short to halve, their sized refusal
carries the Hermite's. Where the march refused for want of step, its
step in the band, the Hermite's refusal on the transversality decision
at an end stands where it refused there (the march refuses at its own
first state as the Hermite does at that end, so the Hermite refused at
the far end, which the march never reaches). Otherwise a branch whose
ends' distance over five falls in the band (`ssi_short_branch`) is the
sized refusal in that distance, carrying the Hermite's; a longer one
may be long and the step is the short quantity, and the refusal is the
step's (`SsiError::MarchStepInBand`), carrying the Hermite's, its
levers the bend and the tolerance below which the step clears the
band. The extent sizes no realized
step; it is the point decisions' clamp, the region decisions' lever,
the seeding floor and the tube
ladder's widest rung.
Exhaustiveness is an in-op obligation (`ssi/exhaust.rs`): every cell of
the bounded domain is *excluded* (an implicit residual bounded away from
zero by enclosure, the boundary pass's mean-value enclosure of a strip
or corner cell the plane misses included), *accounted* (contained in a found branch's tube,
or in a boundary contact's certified region), or refined to the named
floor, where the op refuses
`SsiError::ExhaustivenessInconclusive`. Where the pose counts the
section's components (C5), exhaustiveness closes by that count: the
march, seeded from the counted cut ends and witnesses, returns
certified branches matched one to one to them, else a desync; the cell
sweep runs where no pose counts. For a pair with a ruled side (a
cylinder or cone against a plane, cylinder, cone or sphere) the count
is the ruling sweep's, and a `debug_assert` re-derives it: each cell of
the ruling angle has its rulings' meetings with the partner (the roots
of a quadratic, in closed form) outside the domain the march ends at or
inside a found branch's tube (DESIGN's error taxonomy, row 5). Each floor is minted once over
the domain it bisects (`SweepFloor`), and a floor that domain cannot
resolve refuses `SsiError::FloorUnresolvable` before any sweep runs:
one that is not a positive finite width, or one narrower than the
finest cell bisection can cut at the domain's largest coordinate. On
the chart lane the refusal names the rate that crossed the floor, so
the cell budget never answers for a floor no cell can reach (an
attainable floor can still spend the budget by cell count). Before
any of it, every SSI door refuses a domain whose centre is not finite
or whose half-extent, feature extent or floor scale is not positive and
finite (`SsiError::DomainUnusable`). The receipt and that refusal
state their lengths in the units their own lane subdivides in and carry
an `ExhaustLane` saying which — metres on the ℝ³ lane, chart units plus
the certified `SupSpeed` that crossed them on the chart lane — so metres
come from `floor_meters()` rather than from a caller holding the rate.
The op does not return until every branch is found or it refuses; the
subdivision doubles as the seed generator, so finding never depends on
luck. Closure of a trace and loop
topology are named trileans on parameter-space distances. Near-tangential
configurations refuse toward C7, each decision by what it reads. A
decision at a point reads `sin θ · min(ρ, E)`, its arm from
`ssi::point_arm`: `E` the extent, and `ρ` the reciprocal of the larger
principal curvature of either surface there, read on the plane × NURBS
lane from each chart's first and second fundamental forms
(`shape_operator`) and on the ℝ³ lane from each implicit form's
Hessian (`implicit_max_normal_curvature`), a plane's being zero; no
chart's parameter lines enter it. The point decisions are the march's
states, a Hermite candidate's two ends and refinement's unsettled chord
midpoint, which refuse `SsiError::TransversalityBand`, and the at-rest
per-sample check in `plane_nurbs_limbs`, which refuses
`PlaneNurbsRefusal::NotTransverse`; each names which of `ρ` and `E`
its arm was (`PointLever`). A decision over a region (the boundary
strip, and limb 3's tube on both lanes) reads the certified least
`sin θ` over it, levered by `E` alone.
The NURBS × NURBS lane is one complete operation, the full solution
set over the product of the two knot rectangles, read through a handle
(`ssi::Section::of`). Its boundary pass runs once per handle, at the
first reading that needs its crossings: each side of either rectangle
is a face of the product, the other surface against that side, and the
pass walks all eight faces as limb 3 walks a box's
(`ssi::four::face_roots`), each solution it isolates settled onto both
surfaces with its face's coordinate held, or refused
`SsiError::EndNotOnLocus`; those are the crossings, the only ends an
open branch has. The lane reports the two boundary regions the
plane × NURBS lane does, each built from a solution handed to the
reading as known, because the lane has no parameter inverse to find a
side's image on the other chart:
- **A `Side` region** comes from a shared row: a row of both charts
  that the caller hands in with its two certified images. The region
  is the row's cover, a chain of boxes along the row, each wide enough
  across it to hold the accounting floor's cells beside it, over which
  the pair's crossing reading (`graph_margin`, `ssi::row_crossing` at
  that width) is zero-free. Each slice across the row then holds at
  most one solution, and the row is that solution. Its cells are
  contact cells in the accounting.
- **A `Corner` region** comes from a shared vertex at a corner of both
  rectangles. The reading certifies the crossing reading zero-free
  over the corner cell, and reads the signs of the direction the
  section leaves along there, as the plane lane reads a corner's
  inward partials: either the section leaves the domain (an isolated
  point), or a branch starts there.

A face the walk does not resolve within its pieces and that no handed-in
row or vertex covers (the section touching or running along a side, or
through a corner, of either rectangle) refuses
`SsiError::PairBoundaryUnresolved`. Its two readings are one contract. `branch_at(seed)` settles the seed's feet on both charts
onto the section by Newton (outside either rectangle it refuses
`SeedOffDomain`; where the surfaces are tangent, by the march's
transversality decision); a seed inside a tube already certified is
that branch, any other is traced to the crossings it reaches as an
interior seed is, fitted, refined and certified, and the answer is the
branch whose certified tube holds the settled seed, or
`SsiError::SeedOutsideTube`. What is proved is that the answer is
the branch through the state Newton settled on. Newton's convergence to
the component nearest the seed is not proved, and the tube check does
not catch a seed that settled onto another component (the branch is
traced through the settled state, so its tube holds it); a seed far
from the section gets whichever branch Newton reaches, with no distance
gate. The backstops are the caller's: the offset door's seed lies
within `d / sin(φ/2)` of the moved seam, and a wrong branch
disagrees with the edge's moved vertices (`VertexDisagreement`). `all()` traces every open branch between the
crossings no certified branch already ends at, every closed loop from
the pair lane's seeds (cells of the product whose two charts' boxes are
not certified apart, at the seeding floor), and accounts every cell of
the product, excluded or inside a tube, at the accounting floor, in
chart units at the fastest of the four chart speeds
(`exhaust::account_pair`). Exhaustiveness is `all()`'s obligation.
The plane × NURBS lane is read through a handle of the same shape and
contract (`ssi::PlaneSection::of`, which refuses a window short of the
wall before any reading; `plane_nurbs_ssi` is its `all()`). Its
boundary pass, the side-by-side pass above, runs once per handle at the
first reading that needs it. `branch_at(seed)` settles the seed's feet
on the plane's chart and the wall as the pair lane's does (outside the
wall's rectangle it refuses `SeedOffDomain`), and what it proves, and
its backstops, are the pair lane's. It answers what holds the settled
seed (`ssi::SectionReading`): a `Side` or `Corner` region the pass
reported, or the branch whose certified chart tube holds it.
`all()` traces every open branch between the crossings no certified
branch already ends at and every closed loop from the chart lane's
seeds, and accounts every cell of the wall's knot rectangle, excluded,
in a tube, or in a region.
Hoffmann §6.5's tracing through singular points is deliberately not
adopted. Subdivision is recursive bisection with a linear scan over
tubes; the C10 tree is not wired in.

### Pcurves

**C4 — Pcurves are per-half-edge certified caches, certified in metres
through the map.** A *pcurve* is an edge's image in a face's `(u,v)`
chart. Its home is the half-edge (`Body::pcurves`, a
`SecondaryMap<HalfEdgeKey, PcurveCache>`): a row is the edge's image,
a function of the edge and the chart alone, plus the half-edge's
**joint element**, the integer (whole periods, a torus's second period,
a sphere's twin bit) that carries its image onto the end of the
half-edge before it in its loop, or a reset marker at a pole or apex
(and on a spline chart where the net's own `u` stretch is not decided
past the band).
On a periodic channel the image is placed over the edge's span in the
chart's principal window, so where a carrier's parameter happens to
start moves neither the stored image nor any joint element. A chart
coordinate is a coordinate in the chart's cover and never wraps. An
angle read off the geometry becomes one only through a stated
reference; an azimuth's is the side of the principal window's cut line
its direction lies on, decided in metres (`pcurve_chart_cut_side`), so
the read jumps only across the band of that line at the window's start,
on its angle-0 side, a band as wide in angle as the band is against the
direction's length. For an arm whose phase is the direction's angle that
is the window's own cut. For an arm whose offset moves the angle along
the edge (a parallel, a rim, a focal section, a torus tube's `σ·m`, a
Villarceau `t0`) it is a generic angle no corpus direction is on, and
placement's marks name the copy either way. A sphere's latitude is read
the same way in its own window, `(1 − π, 1 + π]`, whose cut is generic
for the latitude read too: a far-half meridian reads its polar angle
anywhere on `(π/2, 3π/2)`, so the cut cannot be `atan2`'s at `π`.
Every whole-period question uses the channel's own period, which the
chart owns: by renaming where no reader tells the copies apart,
through the marks where one does. No whole-period integer is read off
a value.
A loop's lift is derived by summing elements from its `first`, so no
stored byte depends on which half-edge is `first`. A seam edge has both
half-edges on one surface with one image and two joint elements, so no
coarser key works; this holds on every closed channel, each with its
own period (τ in azimuth, a torus's τ in its second angle, the knot
domain's length on a spline net decided closed in that direction). Its parameter *is* the carrier's
`he_plus`-forward parameter; traversal sense per face is derived, never
stored. `PcurveCache::certify` is the only constructor. The certified
statement is `|S(P(t)) − C(t)| ≤ ε`, a 3-D displacement over the whole
edge, bounded by an envelope whose own statement the certificate names
(`EnvelopeStatement`, whose variants carry their own derivations).
Where `S ∘ P` has a closed form, the envelope alone is the certified
statement: it bounds the displacement over the whole span, spelled in
the carrier's own coefficients so that a minted row's identity is a
theorem rather than a trig round trip, and a sampled check adds
nothing it does not already prove. For `Pcurve::Harmonic` (both sides
in `span{1, cos t, sin t, t}`, so a corruption hiding between samples
is unrepresentable) that spelling is the carrier's incidence with the
chart (centre on the axis, radius, orientation, axial line, per chart
arm, and the chart frame's own unit and orthogonality defects) plus the
stored image's fidelity to the image `certify` re-derives from the
carrier, metered through the chart's stretch; the shared schedule is
then the closed-form tables' cross-check, run on the point lane (the
f64 witness replay, the `Witness::Inexact` scalars) and as a property
test of `chart_image_harmonic ∘
chart_pcurve = carrier_harmonic` over the covered classes, not in a
certificate over a parameter box. Where no closed form exists, the
image is the one the chart holds exactly. A fitted image on a NURBS
chart is bounded between the shared schedule's samples by hull bounds,
whatever the carrier's kind: a spline carrier enters the composite as
its own net, a line as its degree-1 net, and a circle, ellipse or
spiric as a per-span enclosure in certification arithmetic (C9).
On an analytic chart, a carrier with no closed-form image (a spline
carrier, a sphere's general circle) stores the *projected image*, the
chart's own inverse applied to the carrier, `P(t) = ψ(C(t))`: the
carrier's net in the chart frame, plus a partition of the span into
pieces that each stay in one `atan2` sector. On the plane it is the
carrier's net mapped through the affine chart; on the cone its second
channel is the coordinate of the carrier point's foot on the
generator. `S(ψ(x))` is the point of the chart nearest `x`, so the
envelope bounds `|S(P(t)) − C(t)|` over the whole span as the
carrier's distance from the chart (the composite of the chart's
implicit form along the carrier, in metres through a per-span lever),
plus the stored net's fidelity to the re-derived one through the
projection's Lipschitz bound and the frame's defect; per piece, the
sector condition is decided, and a span reaching a pole or the apex
refuses. Every condition is decided over the pieces and spans the
edge's own interval overlaps: the net's geometry past the edge's ends
is not the edge's. The period gate reads the image's sweep on a cover
finer than its pieces, graded toward its two ends, so a whole turn
certifies and a sweep past one refuses. The row reads no mate surface: an intersection carrier's
uniqueness tube is the edge's certificate (C2), not the row's. No
UV-space tolerance appears in any certified statement; the chart's
stretch is the lever arm. A row certifies against its carrier and chart alone, on one branch (a τ jump
is unrepresentable in `Harmonic`'s `α + β·t`, and the projected
image's sector condition refuses one); no caller's window enters
it. On a periodic chart a harmonic image's winding is structure the
row stores, never decided in metres: the row holds its windings (`β`,
and `σ` where the second channel is an angle) as the derivation's own
values, `certify` compares them with the image it re-derives exactly,
and the image's sweep is the carrier's own span, which the edge
certificate decides at the carrier's rate. The envelope is the row's
one statement in metres. A row is its placed image: a lifted copy is
the row together with a deck element and is never certified again.
The face's rows lift its loops, and `topo::pcurves` certifies the
loop: at each joint the walk decides the deck element (the whole number
of periods on each periodic channel, and on a sphere the involution
twin) as an integer, by the half-period marks read outward from zero,
however many periods that is, with no cap and with
half the step to the next point of the joint's orbit as room: half a
period on a cylinder, cone or torus, and a quarter period on a sphere,
whose twin sits half a period over in azimuth. Where two points as far
from the axis as the joint's chart ends can be (the vertex's distance
less the two ε that bound each end) and that half step apart in azimuth
are farther apart than the joint bound, the integer names the joint's
own orbit point; nearer a pole or a narrow cone's apex it may name
another lift of the same point. The joint's
3-D coincidence is not decided again in the chart, since it follows
from the two rows' envelopes and the edge certificate's pinning of each
carrier's ends to the vertex. A loop's winding is its elements composed
once around, read off the invariants a change of starting half-edge
leaves alone: off the twin, at most one period on each periodic
channel; through the twin, an odd number of half turns of azimuth, at
most three, and no bound on the second channel, whose periods are not
such an invariant there; and no azimuth winding counted across a reset.
A closed chart polygon is built only where the winding is zero and
every joint's integer was decided with room past the joint bound. A
vertex on the chart's singular set (a sphere's pole, a cone's apex) is
decided as 3-D incidence of the vertex on that set; a chart with no
singular set decides nothing there. On a spline chart,
where a net can fold and a 3-D coincidence does not name the sheet, each
joint also states its chart-space gap. Planar faces store nothing; `chart_pcurve` derives on
demand. On every other chart the row is mandatory at rest: every
half-edge of the face stores its certified row, and tier 3 reports a
missing row as a finding, saying why it is missing by re-deriving the
face (never minted, or the derivation refuses), and re-certifies every
stored row, a half-minted face's included. A topology door may drop
rows mid-surgery; every public producer ends with a full mint, so
validity is judged on what the producer returns. Every class of carrier
a chart can hold has a route into a certified row, and a face no route
covers refuses at the producer rather than reaching rest uncached.
A closed-form class gate is a lane choice (D3): in band or definite,
the carrier takes a lane whose certificate does not rest on the class,
its projected image where the chart has one. Whether the carrier lies
on the chart at all is a residual, not a class gate: a definite miss
is a contradiction between what the edge states and its geometry,
ended by the door that reads it (D4 ¶1 (iv)), and a read in band, at a
pose or undecided, ends in the decision's lever at every door. Which
sketch entity an edge came from (`EdgeAuthority`) is provenance, and no
ending reads it. A
carrier whose route refuses (a spline or a circle whose projected image
reaches a pole) refuses at the producer like any other refusal: no
class goes unrouted, so the exemption excuses only a scalar that holds
no certified lane, at which a net's projected row cannot read its hull. The
lanes: `Harmonic`, `IsoLine`, `IsoArc`, `Spiric` (the
plane-cap and torus-wall images of a `Curve3::Spiric`, on its own torus
and on that torus's mirror through the cutting plane, data-free and
closed from the carrier's own parameter), `FocalSection` (a conic
whose projection onto the plane normal to the chart's axis has its
focus on the axis, in two instances: a tilted plane×cone ellipse on its
cone, its second channel the harmonic slant, and a Villarceau circle on
its torus, its second channel the tube angle, linear in the parameter;
the azimuth is the projection's Kepler true anomaly, and, as for
`Harmonic`, the envelope alone is the certified statement — the
harmonic closed form of the Kepler decomposition plus its remainder,
drift and frame terms — with the schedule its cross-check), `Projected`
(the projected image above, on an analytic chart), `Fitted` and `General`
(on a spline chart; the general curve-in-UV at the honest fitted grade). Carrier-primary
stands: the 3-D carrier is the authoritative machinery and the edge's
parameter stays chart-neutral. A two-surface edge is `EdgeDescription::Intersection { pair, selector }`,
a curve of the pair's intersection picked by a structural selector where
the curve is known in advance (a closed form of the pair, or a curve
chosen in a region of 2-D overlap; stated by a construction, recovered
by an import; certified by its own meter and incidence on both surfaces, each surface's chart-image certificate, the `|S(P(t)) − C(t)|`
statement its pcurve carries) or by a witness where it must be derived
numerically (certified by its transversality or order-k jet margin);
`EdgeDescription::Chart { surface, pcurve,
wrap }` is a locus one chart defines (wraps, iso lines, splits whose two
faces share one surface), with `EdgeAuthority` recording who determined
the locus. Which form an edge between two faces takes is decided in one
home, `topo::rest_locus` (`RestLocus`): one surface
record is one surface; two records the carrier-equality ladder reads
as one carrier over the faces' region are one surface too, since
incidence on both is incidence on one, and the edge rests in the first
record's chart; two distinct carriers, or a kind outside the ladder,
are the pair, carrying a structural selector where the pair's curve has
a closed form and a witness otherwise; a coincidence in band
is refused at the structural act that made the two faces adjacent, and
a rung that cannot decide or a region that does not read refuses typed,
naming the lane. The chart collapse and its fence are
`docs/PCURVE-UNIFY-DESIGN.md`, not restated here. Volume and
area read a face carrying a `General` pcurve through the trimmed
quadrature lane (`crates/topo/src/props/quad_lane.rs`), and tessellation through the
mesher's trimmed path; a rational `General` image refuses typed there.

### Dispatch

**C5 — One total kind-pair table, no runtime fallback.**
`intersect::route(SurfaceKind, SurfaceKind) -> PairRoute { rung,
implemented, note }` is an exhaustive match with no wildcard arm, so
adding a `SurfaceKind` breaks the build (D3). "Try closed-form, else
march" does not exist: an arm's rung is a documented decision, and an
unimplemented arm refuses typed naming its routing and what it lacks.
Within-pair degeneracies are trileans run before any rung (axis
parallelism at derived angular thresholds, centre/axis distances against
radii): definitely generic goes to the arm's rung, exactly degenerate to
the closed form, in-band to `SectionError::Escalated`. Equal cylinder
radii, and a sphere's centre on a cylinder's axis, are decided by their
margins like every other coincidence (D10). A pair of analytic surfaces
decides its tangency and coincidence once, on its pose, before any rung,
and every reader (the pair's SSI door, the section value, the section
certificate) reads that one decision: a closed-form margin where the
pose has one, and otherwise the pair's **critical gap**, a certified
sweep of a one-parameter distance between the two surfaces' spines,
never a root formula; in band, it escalates. A pair of a NURBS face and a
surface with an implicit residual `f` (plane, cylinder, sphere, torus;
the cone once its residual is enclosed) decides its tangency and
coincidence once, in its classification, on the composite `φ = f∘S`
over the NURBS face's knot rectangle, before any march, and every
reader reads that verdict: the hull of `φ` decides apart and
coincident; the boundary pass decides each side (a side the boolean's
records declare is read from them); one regularity sweep of the
rectangle decides the interior, where an isolated critical point of `φ`
with a nondegenerate Hessian and its value decided in the zero band is
a tangency decided Zero (a point touch where the Hessian is definite, a
node where it is indefinite, its arcs ending at it), and a tangency
along an interior curve, or anything in band, escalates. A side of the
rectangle collapsed to one point not decided clear of the partner
refuses at the door, before the sweep, naming the side. The
interior-loop guard's loop-free stage is that sweep's verdict. A pair
of two spline sides (NURBS, or `Approx` read as its fit) is classified
the same way over both rectangles; until that is built, its tangency is
its certificate's (C2). Tangential outcomes (`TangentLine`, `TangentPoint`) are
classification data, refused as carriers. `SurfaceKind::Approx` is its
own kind, and as an intersection operand it is its fit: `(Plane,
Approx)` and `(Approx, Plane)` route to the plane×NURBS arm over
`approx.fit()`, and every other `Approx` pair routes as the fit's kind
does. Nothing is composed into the edge's bound: the fit's distance from
its description is the face's claim (O3, re-derived at rest by O5). Such
an edge's carrier lies within `2ε / sin θ` of the described section
along the wall, `θ` the angle between the plane and the wall, the same
conditioning every certified `Intersection` edge already has; that is
stated here and not enforced.
The same rule governs curve pairs. Two carrier spans (a carrier and a
parameter interval) meet through one exhaustive `(CurveKind, CurveKind)`
table, and every candidate it proposes is settled against each span by
the point-on-span reading, which composes geom's one-carrier readings
(the point's foot, and whether it lies on the carrier) with the span's
reading of its ends, and decides nothing of its own; a point is decided
off a carrier only on a lower bound on its distance to the whole
carrier.
Line, circle and ellipse pairs are closed forms that solve in one conic's
angle and measure in metres; a
pair with a spiric or spline side is the certified bisection walk, which
is also the closed forms' differential test oracle. A candidate generator's
in-band decision escalates and never prunes, and a candidate admitted under
a `Zero` reading is placed by the exactly degenerate formula, never by the
generic one. Two spans meet along stretches. A *stretch* is a longest
interval of one span which is nowhere definitely off the other; a
*meeting* is a longest interval of one span which is everywhere
definitely on the other, a single point for a crossing or a touch and a
longer interval for a shared run. Each span's ends are read against the
other span before anything else. A stretch holding exactly one meeting
is reported as that meeting, which takes the whole stretch; a stretch
holding no meeting, or more than one, escalates, since between two
meetings in one stretch the spans are nowhere definitely apart and
whether they are two crossings or one run is undecided. Two curves may
cross any number of times, and each crossing in a stretch of its own is
its own meeting; a candidate nearer a found meeting than the band's
escalation width cannot be decided distinct from it, so it is not read
again.
A closed form that already holds a meeting divides that meeting's root out
of its equation and solves only for the others. The reader is given no
topology: two edges joined at a vertex meet there because their ends meet.
Implemented: plane×plane, plane×sphere, sphere×sphere (rung 1);
plane×cylinder, plane×cone (the parabola and hyperbola in closed form,
each arc minted per window, `ConicBranch::arc`),
axis-aligned plane×torus (its axis-parallel pose off the axis the
`Spiric`) (rung 2); cylinder×cylinder in every pose (its parallel
rulings and its bisector ellipses closed form, every other crossing pose
marched over its ball cut by the region), cylinder×sphere,
sphere×torus, cylinder×torus, torus×torus, plane×NURBS and plane×`Approx`,
NURBS×NURBS and every pairing of `Approx` with NURBS or `Approx` (rung 3). The offset door reads
plane×NURBS through `PlaneSection::branch_at` and NURBS×NURBS through
`Section::branch_at`, each at the old edge's middle. The boolean reads
plane × NURBS and NURBS × NURBS (and `Approx` as its fit) through the
section value (`pair_section`, through its march door), built
from the whole-section reading `all()`:
- each closed branch is a loop component, with its witness and carrier;
- each open branch, `Side` region and `Corner` region is a component
  that reaches its chart's boundary;
- a refusal maps by name, tangency and grazes to tangent, and an
  unclosed accounting to intractable, carrying its cause.

A component that reaches its chart's boundary is not interior to the
face. A point of the rectangle's side that lies in the face lies on one
of the face's pcurves (C4), which is an edge of the face. So the
interior-loop guard clears such a component as it clears an unbounded
one.
A plane × spline pair is classified before `all()` is read, in the
order above: the hull decides it apart or coincident, or escalates, at
every scalar; a collapsed side not decided clear refuses at the door;
the boundary pass, run once and handed on to `all()`, refuses a graze
by name and finds where the section ends; and the regularity sweep's
loop-free verdict, every component open with its ends on those sides
(`Components::Reaching`), is the value's, with no `all()`. The guard
clears it where each of those sides bounds the face, and otherwise
reads it whole. Where the sweep declines, the value is `all()`'s.
The boolean reads the rung-3 arms cylinder×sphere and the three torus
pairs through the section value (`pair_section`), classified, then
carried: the classification decides the pair's pose once and, where a
closed form counts them, its components (`Components::Counted`); it
costs no march and needs no lane. Carrying marches and fits the loops
at `f64`, and lists the components: a counted section's, the count the
proof and the march matched to it one to one (each boundary crossing
the end of exactly one branch, each listed witness on exactly one
branch, each claimed winding held, else a desync), and
otherwise the march's own closed branches (`Components::Marched`), each
winding about an operand as the fitted loop's turns about the
operand's axis and core circle, read off the loop in ℝ³, say
(`pair_section::winds`); a point is read on a carried loop within the
band padded by the loop's pad (`CarriedComponent::pad`, where its
closeness and uniqueness are argued). A reader that needs a list no closed form
counts carries; a site's component is its foot on the carried section,
its one reading. A pair traced in ℝ³ is read over its own ball where
its pose proves one, cut by one region per operation, as at one band:
the ball about the overlap of the two operands' boxes, padded, which
holds every section point the operation can use. No face's or site's
extent enters it, so two faces on one carrier read one value. A
component is closed, or open with both ends on the domain's boundary,
or ends at a node (Edge descriptions): a node is
decided on the pose as a Zero tangency whose section still has
carriers, placed in closed form, and a site at it reads as the node.
Two cylinders decide their pose on closed-form margins: parallelism as
`sin θ · L` at the region's radius `L`; then the gap `e` along the
common perpendicular against `r₁ + r₂` and `|r₁ − r₂|`, or the axis
distance where parallel. A pair with a ruled side reads its pose on the
sweep of the discriminant along that side's rulings: its double root
the tangency, decided on the pose, the arcs where it is positive the
components, a cone's apex a closed-form margin, and for two cylinders
the margins above its extremes. Each torus pair's pose
(`SphereTorusPose`, `CylinderTorusPose`, `TorusTorusPose`) guards its
operands, reads the partner's symmetry on the section certificate's
lever and pivot margins over the torus's own ball `|x − c| ≤ R + r`
(which holds the whole section; the smaller ball for two tori), and decides the
**critical gap**, the one place a torus pair's tangency is decided. Two
tori are decided one surface or not before any of it
(`pair_section::torus_coincidence`): one where the bound on their
distance, the core circle's move plus the tube difference, decides
`Zero`, apart where a datum alone decides past the band; the bound holds
at every point of both, so it reads no ball, and the Boolean's carrier
ladder reads the same decision for two torus faces it reads undeclared. A
sphere, a cylinder and a torus are the points at distance `ρ` from a
spine (a point, a line, a core circle), and two of them are tangent
exactly where a critical value of the spine-to-spine distance equals
`ρ₁ + ρ₂` or `|ρ₁ − ρ₂|`. A coaxial partner's spine distance is
constant, and its gap is the closed-form tube row read across the
coaxial slack. A sphere's spine is a point, whose critical distances
from the core circle are its two meridian ones, so its gap is closed
form too, read across its rounding. Every other pose's is a certified
sweep of the torus's core angle (`pair_section::gap`), two tori's on
either core, the second where the first escalates, so the verdict does
not depend on the order: a cell clears where the distance `D` stands
past the band from both levels (it moves at most `R·w` over a cell) or
where `D²` is smooth and monotone on it (a Taylor bound on `(D²)′` with
a bound on `(D²)‴`); a turn of `D` whose window bound reads within the
zero band, or the least distance and `|ρ₁ − ρ₂|` both within it (the
tube-circle tangency), is `Zero`; anything else at the floor escalates.
Each margin is a length, less its rounding: a slope's sign is certified
on `(D²)′` shrunk by its own rounding before anything divides it, so a
slope within its rounding of zero reads no sign however small `D` is,
and what is left, over the most `D` reaches, bounds the sine of the
crossing, levered by the core radius. Where the core crosses a torus
partner's axis `D` has a kink, and no cell holding one clears on its
slope: a tangency on a kink always escalates, unless it is the least
distance at the level `|ρ₁ − ρ₂|`. Over a transverse gap
the symmetric poses (coaxial, parallel, square) count their components
in closed form, the coaxial ones as circles and the rest the general
rung's, marched by `carry` over a ball that holds the whole section
(`PairPose::ball`: the sphere, else the torus's `|x − c| ≤ R + r`, the
smaller of two tori's), every branch closing inside it; a general
sphere × torus pose is the sphere arm's, marched; a general cylinder or
torus pose has no closed-form count of its components, and its
components are the march's (`Components::Marched`), which the general
rung's door (`ssi::cylinder_torus_ssi`, `ssi::torus_torus_ssi`) marches
past a transverse gap. `implemented` covers the whole operation.
Every other pair refuses: the cone pairs blocked on the cone's metres
conversion (C2 limb 2).

The same rule governs a curve against a face. A carrier span and a face
meet through one exhaustive `(CurveKind, SurfaceKind)` table, in the
curve-pair vocabulary (stretches, meetings, an in-band decision
escalating and never pruning), and the boolean's root doors are its
arms: a spline span against an analytic face and an analytic span
against a spline face are two arms of the one table. The pair is
decided from the strongest evidence first, and each rung reads only
what the rungs before it left undecided: what the operands' topology
already states; then the span's ends; then the span against each of the
face's boundary edges, through the curve-pair table; then the face's
interior. Topology decides by incidence. A span that the curve-pair
table shows to run the whole of an edge of the face's own body, end to
end, is that edge. It then meets the body's faces only at the vertices
and edges they share, and nothing is read against their interiors. This
holds under the boolean's precondition that its operands are valid
bodies, which it assumes and does not certify per call.
A shared run along a boundary edge is a coincidence, decided at the
coincidence door (DESIGN `[coincidence]`) and never by surface
arithmetic, and a point meeting there is a meeting on the face's
boundary. An interior arm with no closed form settles
each piece of the span against each cell of the surface by enclosures:
clear when the two are definitely apart; exactly one transverse meeting
when an interval Newton (Krawczyk) contraction on the piece and cell
succeeds, its Jacobian enclosure definitely invertible; otherwise
subdivided down to the tangency floor, where it escalates. A surface
with no signed residual is given none: a point is decided on it by a
certified foot within the band, and off it by a lower bound on its
distance past the band. A meeting inside the chart is placed in or out
of the face by the face's trim.
Implemented: the table (`curve_face::route`), the boundary pass
(`curve_face::boundary`), which the boolean's sweep reads ahead of the
carrier for a spline face and through which its on-carrier crossing
reading reads every face's boundary, and the spline span × plane arm
(`curve_face::composite`, the boolean's planar lane reading it). That
arm composes the face's canonical residual along the span (the ring
composite) and reads it piece by piece, a piece never straddling a
knot: clear where the residual, metered, is definitely past the band
over the piece; one crossing where it is monotone and its ends lie
definitely on opposite sides, bisected and read on the surface; and
otherwise cut where it is definitely past the band. A piece within the
band that no cut parts, a meeting within the band of a crease and two
crossings within the band of each other refuse. The composite reads the curved analytic faces too, but the
curved lane does not dispatch a spline span to it. The span ×
spline face arm is built for an analytic or a NURBS span
(`curve_face::spline_face`, the boolean's spline-face door reading it
past the boundary pass): it settles boxes of the state `(t, u, v)` on
`C(t) − S(u, v)`, a displacement read against the band with no meter,
clear where the box's enclosure is past the band and one meeting where
the Krawczyk test holds, which also isolates the meeting's
neighbourhood. A NURBS span's point and tangent over a parameter window
are its control hull's and its hodograph's, read as the chart's are. The
span and the chart are cut at their creases and no box touching one
carries a proof; a box within the band at the floor that no
meeting holds refuses, wherever it lies, so a meeting within the band of
a crease, on the chart's domain edge (a closed chart's seam included) or
within the band of another refuses, and nothing in band is dropped. An
end of the span is the end vertex's meeting: the walk reads past it as
far as the carrier runs, and a box holding the end's state on which
every solution lies within the band of the end, by the mean value
theorem, is accounted for by it, so an end at the carrier's domain end
or on the chart's edge is read without reading past either. A spiric
span against a spline face is unbuilt. The analytic
arms are the sweep's existing root doors. A meeting inside a spline
face's chart is placed by the trim at the chart point it was found at,
on its own sheet (`topo::boolean::spline_region::spline_chart_placement`);
a point in space has its placement built
(`topo::boolean::spline_region`, the spline arm of the boolean's
point-in-face door): on the carrier by a foot decided within the band,
off it where every Bézier cell of the chart is decided past it; then in
where such a foot lies in the trim, read from the face's lifted pcurve
loops, and out where a foot exists and every cell not decided past the
band lies wholly outside the trim, each reading clear of the trim by
more than the band along the chart.

### Fitted-cache structure

**C6 — Structure is combinatorial and certified for every member;
values are the scalar's; certification is scalar-generic.** A carrier's
*structure* is its combinatorial part: degree, knot and control counts,
knot multiplicities, the order of its breakpoints, and every decision a
fitting loop or a skin takes (knot count, sub-arc count, merged knots).
Its *values* are knot positions, weights, control points and the
combination coefficients the knot algebra derives from them. Values are
held in the scalar, each entered where it is made as what it is: a
constant of the construction's form (π as `T::pi()`, never as the
arithmetic that computes it) or of an imported file through `from_f64`;
a numerical routine's output (a fit's knots, weights and poles) once, at
the fit, through `Real::from_computed`; a value the model determines
computed in the scalar. At an enclosure scalar a carrier stands for the
family of its members, so its knot positions and weights may be
enclosures as its poles are. Structure is decided once for every member:
at a point scalar it is read off the values; at an enclosure scalar it
is certified to hold for every member — consecutive knot runs ascend,
every weight is positive, and a parameter's span read hulls every span
the parameter can lie in for some member. A structure decision whose
members may disagree refuses typed, naming the decision, and carries
its margin so that a refinement of the inputs can lift it; a decision
that was only a shortcut (equal weights, the integral test) takes the
general route instead. No decision rule breaks where documents sit: the
sub-arc count allows one sub-arc to reach `(π/2)(1+η)`, η a named
structure constant, so an arc whose true sweep is a multiple of π/2
counts the same for every member. η is `1/16`: it keeps a sweep of
`k·π/2` at `k` sub-arcs for every `k ≤ 4`, a full turn included (which
needs `η < 1/3`), it is exact in binary, and a sub-arc's
tangent-intersection point stays well conditioned
(`cos(Δ/2) ≥ cos((π/4)(1+η))`). The arc's other decisions (that its sweep
is not zero, whether it ends where it starts, and that it is short of a
turn between two points or one turn back to its start) are certified over
their brackets the same way. A fit's structure (knot vectors,
degrees and control counts) is the deterministic `f64` pipeline's, and
the certificate re-evaluates against it at any `Real`, so the interval
lane proves what the f64 lane chose. A refinement made inside a
certificate chooses nothing: its refined weights are generally not
`f64`, so it is held only as homogeneous enclosures `(w·P, w)`, and the
projective knot algebra has no meaning at the certification scalar; the
knot algebra a construction runs at that scalar (a skin's compatibility
step) is that homogeneous algebra too. No topology-determining predicate
reads knot counts, spans or fitted coefficients except through named
certified margins; the name table is a function of recipe structure and
verdicts only.

### Tangency

**C7 — `TangentIntersection` and second-order sector classification.**
`EdgeDescription::TangentIntersection { pair, witness }` mirrors
`Intersection` one differential order up. Its jet (`tangent.rs`,
`TangentJet { sin_theta, kappa_rel }`) is certified per sample, one
reading per equation of the jet system, each a length at its own lever:
the two surfaces' separation along the shared normal within ε (the
touch itself, a fact about the pair), the witness within ε of each
surface, normal parallelism within the derived angle at lever arm
`1/κ_rel` (the witness's offset across the locus), and the relative
transverse normal curvature
`κ_rel = κ₁ − κ₂` bounded away from zero (the IFT denominator of the jet
system), with C2's hull bounds and tube between samples. Where
first-order sector ranking ties exactly, classification descends one
order: a surface pair through `enters_material_order2` (consumed by
`topo::boolean::sectors`), a conic edge against a plane through
`conic_bend_side`, its lift certified from the carrier's curvature
bounds less the first-order residue (consumed by
`topo::splitting::neighborhood` and `topo::boolean::sectors` alike); an
in-band second-order tie escalates. Readings of different equations are never
summed: the witness's offset along the shared normal and across the
locus are perpendicular, and a residual is a defect of its equation, not
a distance to the locus (as for `Intersection`). Two terms are summed
only when both move one reading, as a residual and its span sag do. Tier 3 has two levels: every
definitely-tangent edge carries a recorded `ContactMark` (`Tangent` when
jet-determinate, `SmoothUnderdetermined` when the surfaces
under-determine the locus, e.g. a G2 sketch join), and the must-carry
rule fires only on `ContactMark::Tangent`, refusing a conventional
description there (`OnPair`, or a `Chart` that is not a wrap, declared
or derived) `TangentNotIntrinsic`. Tier 3 reads a body at the ε it was
built at, the run's. An edge whose stations read definitely
transverse at one and definitely smooth at its neighbour brackets an
unsampled in-band point and is refused (`MixedDihedral`, naming the two
stations); `must_carry_over_edge` answers it `MustCarryVerdict::Mixed`,
and every constructor refuses it typed but one: the blend's contact
edge, routed smooth by its carrier's construction, ends a mixed reading
as `SurgeryInvariant`, the surgery contradicting its own routing, since
the rolling ball's exact contact cannot read a corner anywhere along
it. NURBS-
and `Approx`-adjacent edges are exempt by kind (`Unmarked`). Curved
contact census is CONTACT-DESIGN's, at `crates/topo/README.md`.

### Fillets

**C8 — Fillet validity is reified predicates, evaluated before any
construction; blends are analytic-first.** Implemented in
`crates/sweep/src/blend/`; `crates/sweep/README.md` is the reference.
What binds from here: the six named margined predicates over the inputs
(radius vs `1/κ_max` of each support, face clearance, spine regularity,
chain G1, convexity-sign consistency, corner configuration) all answer
before anything is built, which is what lets an interval replay certify
validity over a parameter box. They run in that order, except face
clearance's reach arm (every band against every face of the body it
does not blend), which needs the plan's feet: it runs in the surgery
after predicate 6 and before any mutation.

**What a blend does with the body's other features is Boolean
semantics.** A convex band's result is `S − R` and a concave band's
`S ∪ F`, where the region `R` (or `F`) is the material between the band
surface and its supports' strips over the band's extent, closed by its
end faces, read from the supports' carriers and the band's ends and
never from the features near it. Every case follows by set algebra: a
feature wholly inside `R` goes with it, as a feature wholly inside a
Boolean's tool goes with the tool, a void and a free solid inside it
included; a protrusion or an island wholly inside `F` is absorbed, the
island's voids, which lie in `F`, filled with it; a hole whose mouth lies wholly on
the face `F` covers is sealed, a through-hole turning blind and a blind
hole a void; and a feature crossing the band surface has a result too.
What is built of it: face clearance reads each face three ways (clear
of the region, wholly inside it, or crossing it), and the carve takes a
feature wholly inside the material a straight band's cut-off removes or
fills, attached to the rest of the body only by cycles on the band's
caps or on its supports' strips: a through-feature between the two
caps on the ruled fillet and the plane–plane fillet and chamfer (a bore
under a convex band, a bar between the end walls under a concave one),
a cup on one cap on the plane–plane band (a blind bore under a convex
band, a pin under a concave one), each of whose faces that meet the cap
is a plane or a cylinder, and a cup on one support's strip on a
plane–plane band that is a whole chain of one link, cut off at both
ends (a dimple under a convex band, a boss under a concave one). Every
other case refuses, a case the rule defines and the surgery does not
build, and it refuses at the meter that reads it, not by which case
it is. The clearance screen and the exact ring meters read the
supports' own boundary features and the end faces' cycles
(`FaceClearanceUncertified`, `RingClearance`); the reach reads every other face, a crossing as a
measurement and a face it cannot certify either way as a bound
(`FaceClearance`); and a request whose every reading is inside but
that no carve takes refuses `WhollyInside` (a void or an island with no
cap cycle). So a cup on the ruled band, whose far faces have no exact
region to be read in, or a hole whose mouth a concave fill covers,
refuses as a bound where its wall meets the end face's plane. A ring on
a support that the strip meter reads crossing its strip refuses at the
ring meter (`RingClearance`); one it reads neither inside nor crossing
(one touching the trimline, one it cannot certify) and one beside a
band ending at a mitre, a turn or a corner are read at the screen,
nearer the edge than the band sets back, and refuse there whether or
not they lie inside. A crossing refuses as an unbuilt case too, not
as an exception to the rule, as the Boolean refuses the surface pairs
it does not serve; the clearance and ring recourses name the reorder,
blending before a feature that came after the edge
(`crates/sweep/README.md`, predicate 2). The band and its supports are
in-place surgery, not a literal Boolean against a body for the region:
that tool is cusp-edged along both trimlines, and most band × feature
surface pairs are outside the intersection table (C5).

Every constant-radius arm mints a torus or a cylinder (the envelope of
equal spheres over a circle or a line spine); a cone belongs to the
variable-radius family.
Trimlines are stored as `TangentIntersection`. Scope: a straight band
(plane–plane, or the ruled band about a straight spine) ends at a
trivalent vertex of one convexity between planes as the request decides
— by how many of the vertex's three edges it names. All three: the
corner patch (the sphere octant; the chamfer's plane through the three
feet). One: the cut-off — the band ends in the end face's plane section
of it, a chord for a chamfer, and for a fillet a circle where the end
face is perpendicular to the edge and an ellipse otherwise
(`CornerConfig::EndFace`, `RunOutPolicy::CutOffAtEndFace`;
`fillet3_cap_transverse` picks the kind). Two: the mitre — each band is
cut off by the other's support, so the two meet along their
intersection, a line for a chamfer and a planar ellipse for a fillet
(`CornerConfig::Turn`, `RunOutPolicy::Mitre`). Chain G1 classifies a
junction of two plane–plane links rather than refusing it: Zero, one
band runs through a joint; definite, the chain breaks into two ends at
a turn; the sliver band escalates. It still refuses at a junction that
involves a curved link. Every other end refuses — a curved end face, a
foot landing inside a support rather than on a rim edge, an end vertex
of valence other than three, mixed convexity — with a `CornerConfig`
tag and the `RunOutPolicy` that would handle it, refusal-payload names
only. A
spine that is neither line nor circle refuses `SpineUnsupported`: the
canal-surface blend, an approximating surface per O2, is not
implemented.

### Arithmetic substrate and the BVH

**C9 — Enclosures run on the in-repo interval backend.** Every enclosure
certification needs is transcendental-free (implicit residuals are
polynomial, de Boor is ring arithmetic, hull bounds are convexity facts),
so certification arithmetic is IEEE-754's correctly rounded operations —
`±`, `×`, `÷`, integer powers and `√` — and no transcendental, over
`geom_core::Interval` — the evaluation scalar itself, a newtype over
`interval-transcendentals`' `DInterval`, outward-rounded where the
operation is inexact, always compiled, MIT-clean. Its refusal is the
backend's decoration (`dec < Def`), read as `!is_certified()`: a bracket
that may not certify carries ordinary endpoints, so a consumer asks the
refusal by name, and the certification doors refuse it whatever its
endpoints say. The doors are the `Certification` trait's
(`geom_core::interval::certification`), sealed to `Interval` and
imported by name: a file that imports them has no `Real` in its
production code, so a value typed `Interval` there reaches neither
evaluation's weaker `is_poison` nor a transcendental, and
`scripts/gates/certification-doors.sh` holds that over its list of
importers. Generic code over a lane scalar in the same file (`Bounds`,
`CertifiedBounds` and `Decide` are `Real` subtraits) evaluates on that
lane, at `T = Interval` too, and crosses into certification arithmetic
only through `Interval::from_certified` — inherent, since the crossing
is written where a lane scalar is held — which carries the certified
door's verdict as a `Def`/`Trv` cap on the decoration. No copyleft
dependency exists in any build configuration. Certification code reads
brackets through `Bounds` and asks admission through
`CertifiedEnclosure`.

**C10 — One deterministic AABB tree, conservative-superset contract.**
`crates/bvh`: arena-order build, median split on the longest centroid
axis with total tie-breaks, no hash iteration; a query may only prune
pairs whose padded boxes definitely do not interact, so results stay a
function of exact predicates (D9), pinned by an idealized/realized
differential suite. Boxes of curved entities are certified-conservative
(analytic extents closed-form, NURBS from control hulls). Wired: the
boolean edge×face sweep, placement separation, viewport picking. Not
wired: SSI seeding/exhaustiveness (C3).

### NURBS scope

**C11 — The NURBS substrate, bounded.** Clamped knot vectors only
(periodic and unclamped forms are a designed absence); strictly positive
weights enforced at construction (the convex-hull property every hull
bound stands on); evaluation and derivatives generic over `Real` by de
Boor in fixed order. Algorithms: knot insertion, refinement, removal and
degree elevation on curves and surfaces (`split_at` is insertion to full
multiplicity), point projection with certified residuals,
and the fitting stack (interpolation, column-wise collocation for
skinning, the bounded approximation loop). Lofts and sweeps
(`crates/sweep`) are *definitional* surfaces: the produced NURBS is the
definition, with no residual obligation; only items derived from them
carry certificates. Absent by decision: scattered-data surface fitting,
degree reduction, UV-space booleans beyond trim loops.

### The seams the curved work touched

**C12 — The refactor inventory, as it now stands.** (1) The
face-intersection seam consumes the C5 table; the curved-boolean
refusal retires arm by arm, never wholesale. (2) Sector classification
has the C7 second-order lane. (3) `EdgeCurveSpec::split_specs` splits a
NURBS carrier by knot insertion and a conic by parameter interval.
(4) The census is CONTACT-DESIGN's. (5) `merge_faces.rs` merges
cosurface runs through the same never-numeric ladder; a curved run that
closes its chart's full period refuses. (6) Curved tessellation
(`mesh/src/curved.rs`) takes iso-rectangle chart domains from the
boundary walk with certified chordal bounds from hull-bounded jets
(`nurbs_cert.rs`); general trimmed faces with pcurve-driven trim loops
are not implemented (`UnsupportedCurvedShape`). (7) Mass properties on
curved-cut faces are certified quadrature (C9, `props/quad/`):
harmonic pcurve boundaries, polynomial and rational patch flux; a cone
face needs none, its flux `apex·VA` and area `|axis·VA|/sin α` closed in
the boundary's vector area; rational
pcurve channels refuse `QuadratureUnsupported`. A measurement refines
where its enclosure is widest until its caller's predicate over the
enclosure resolves (a sign, a relation, or a resolution over the measured
surface, each stated by the caller), refinement stops paying, or the
budget its caller stated is spent. It returns the narrowest certified
enclosure it reached, with the reason it stopped (`[tolerance.measure]`):
never a silent Gaussian, and never a refusal for width alone. A plane cap or torus wall
bounded by a `Curve3::Spiric` takes `props/spiric.rs`: the cap's vector
area and the wall's chart Green form along the spiric are elliptic
integrals in the torus minor angle, enclosed by a fifth-order Taylor
rule (`props/taylor.rs`) that reads the spiric's integrands with no
operation past the ring ones but `√`, on the same schedule. A plane
cap bounded by a rational spline rim takes the same rule
(`props/rational_rim.rs`): its vector area is `(1/2)∫ B × B′/w²` per
Bézier span, `B` the homogeneous numerator about the reference, and the
closed form keeps its typed refusal for a door with no quadrature lane. A sphere face bounded
by circle arcs — rims, meridians and sections tilted against its chart
alike, holes included — has no conic or spline trim and is on the
closed-form lane, measured by Gauss–Bonnet over all its loops
(`props/curved.rs`, `sphere_face`). A chart
singularity in a face's closure is a vertex of it, read off the face's
region by one reading that STEP import and tier 3 both take
(`chart_singularity.rs`, `locate`); on a sphere loop of circle arcs that
reading takes a pole's side from this lane's own stereographic turning
(`sphere_pole_left`, beside `pole_side`), so a sphere face lies on
the side of its loop that holds no chart pole. The closed-form lane and
tier 3 read that side to cross-check the face's sense bit, and refuse a
contradiction, wherever at least one pole lies off the loop
(`sphere_loop_side`). A face whose loop holds both poles is read through
its seam region, the faces of its surface it reaches across shared
edges: a pole on an edge interior to the region lies inside it, so the
region is the side of its boundary that holds such a pole
(`sphere_region_material_sign`); a region with two or more boundary
loops is read by the sign of its area. A region with no boundary covers
the sphere and encodes no side: covered once, it is a shell whose
orientation checks 7 and 10 read. A region whose boundary carries both
poles, or whose poles lie on no member's loop, encodes no side: the
pole-free reading would assume the rule above, which tier 3 enforces
and this reader does not lean on. For a region of one face the flipped bit describes the
complement, another valid body. Where a region's faces overlap, tier 3
refuses it by its cover census (`sphere_region_cover`), read for every
seam region of two faces or more, whatever its members' own boundaries
encode: each member's Gauss–Bonnet area on the side its bit claims,
summed, less the area its boundary claims (the sphere, with no
boundary), is a whole number of spheres, so the members claim the region
a whole number of times, once for a region covered once. A cover of the
sphere twice over is claimed twice. A region whose members all carry the
wrong bit is claimed `Σ χ_f − χ_R + 1` times, `χ` the Euler
characteristic of each member and of the region as spheres with holes:
`n + m − 1` times for `n` one-loop faces bounded by `m` loops, so twice
for a both-poles pair, and `n − 1` times for a ball of `n` lunes turned
inside out, which the census refuses from three lunes on. The
measurement door reads the same seam regions without tier 3
(`topo/src/props.rs`, `seam_refusals`), so it refuses the regions tier 3
refuses, and a region whose faces' bits disagree, which tier 3 refuses
as check 4's lamina. A loop with a pole off it on each side
encodes no side: under either sense its face holds a pole, which is
validity's to refuse. Nor does a loop whose poles both read undecided: a
pole within the ambiguity band of a great or near-great carrier's plane
has its antipode there too, so a face whose loop passes that close to a
pole is measured under its bit alone. A face with holes reads its side
off the sign of its own Gauss–Bonnet area. (8) In-house SVD and
least-squares solvers with fixed elimination order
(`geom-core/src/linalg`). (9) The curvo audit is `docs/CURVO-AUDIT.md`
(it has no SSI); the stance is DESIGN.md Q5.

## Offsets and shell (OFFSET-DESIGN O1–O6)

The *offset* of a surface `S` at signed distance `d` is the normal
pushforward `S_d(u,v) = S(u,v) + d·n(u,v)` along the unit chart normal
(`∂u × ∂v` normalized; positive `d` is along the stored normal, the
face's `sense` bit takes no part). *Shelling* turns a solid into a
thin-walled one: offset the boundary inward by the wall thickness and
either keep the hollow closed (a cavity) or open it through designated
faces, leaving annular rims where the wall shows.

**O1 — Analytic offsets are minted by struct-update; degeneracies refuse
at the door.** The analytic kinds close under offset (`offset.rs`,
`offset_surface`): plane `origin + d·normal`; cylinder and sphere
`radius + d`; torus `minor + d`; cone with `axis`, `half_angle`, `u_ref`
verbatim and the apex slid by `−axis·(d/sin α)`, i.e. the pure parameter
shift `v ↦ v + d·cot α` (`ConeOffset` states the apex, the shift and the
pointwise displacement as one derivation, along the continuous extension
of the opening nappe's normal field, so nappe attribution follows the
shift; the turn a face-outward distance owes that convention is the
consumer's, discharged at `topo::offset_nappe::face_nappe` for every
door that needs it). Refusals are named predicates over the *realized*
stored float,
decided before any mint: `offset_radius_floor` (margin `radius + d`;
`OffsetError::RadiusFloor`) and `ring_torus_convention` (`geom::ring_torus`,
the convention's one home, on the realized minor: margin
`major − (minor + d)`; `TorusRing`). The cone has no door predicate
because nothing stored degenerates; whether a face's `v`-window crosses
the shifted apex is the consumer's question (`offset_apex_window` in
`topo/src/replace_face.rs`). NURBS refuses `NotClosedUnderOffset` into
the O2 lane. Self-intersection has no special case: where `d` reaches
the collapse threshold the door refuses (O3), never a silently looped
surface. Trimmed offsets and solved-distance offsets are not
implemented; nothing here forecloses them.

**O2 — The approximating surface lifts the `EdgeCurve` triple one
dimension.** The offset of a NURBS is not a NURBS (normalizing `n`
introduces a square root), so the kernel fits one and carries the
intent beside the fit (`geom/src/surfaces/approx.rs`):
`SurfaceDescription::Offset { base: Arc<NurbsSurface>, d }` is the
intensional layer and its only inhabitant (the canal blend is the next,
not built); `SurfaceSpec { description, fit, window }` is the
uncertified input; `ApproxSurface::certify(spec, certifier)` is the sole
door and its fields are private, so an uncertified approximating surface
is unrepresentable. The certifier is injected because the derivation
lives one crate up (`offset_fit.rs`) and is `f64`-only. The certifier
carries its own target, and every production certifier's is the run's
`Tol`, so the surface stores no tolerance and every re-derivation
classifies at the ε of the run that performs it (D4 ¶1). The base is an
owned `Arc`, not an arena key (layering, and `Surface` values travel
without an arena), and it is NURBS by type: analytic bases mint exactly
under O1 and never reach this door. Storage is the seventh variant
`Surface::Approx(Arc<ApproxSurface>)`, so every dispatch site must say
what it does with one: geometry questions (evaluation, boxes,
tessellation, pcurves, sections, C5) read the fit, O5's re-derivation
at rest reads the description, and the other kind-indexed questions
treat it as its own kind (dihedral classification answers `InBand` for
it as for `Nurbs`, the tangent span bounds decline it, the census keys
on the kind).

**O3 — The certificate is C2 lifted, on two meters the fit needs.** The
claim is `sup_(u,v) ‖S_fit − (S + d·n)‖ ≤ ε_precision`, pointwise in the
base's own chart parameters. Meters (`offset_meters.rs`, read off
`patch_bound.rs`'s per-cell control-hull enclosures): the *regularity
floor*, a certified lower bound on `‖S_u × S_v‖` (the area quotient's
least coefficient ratio along each piece's own direction, the
`PatchCell::m_inf` its sup is the companion of), classified by `offset_normal_floor` with
the patch's faster chart speed as lever (read from the norms of
coefficient vectors, D4 ¶2, as the floor's companion `sup‖S_u × S_v‖`
is), deliberately not `|d|`, since
whether the normal degenerates does not depend on `d`; and the
*collapse headroom*, principal curvatures `[κ_lo, κ_hi]` from the closed
form of the two fundamental forms, refusing through
`offset_curvature_headroom` when `|d|` reaches `1/κ` on the folding side.
Both bound conservatively (they can refuse a regular patch, never accept
a degenerate one). The fit (`offset_fit.rs`) is the NURBS Book's A9.4
grid interpolation at the base's own parameters, then a
certify-and-insert loop that refines the cells carrying the sup until
every cell certifies or the crate's stop rule (`crate::descent`)
refuses naming what stopped it — `SeedOverCap` before any round,
against the cap on a grid's total sample count; `WallReached`, naming
that cap or the cumulative budget on the samples fitted over all the
rounds (`OFFSET_FIT_WORK`); `RefinementStalled` on two idle rounds,
the second marked in both directions; the last two carry the last grid's bound
and the smallest any round reached (`BestBound`, whose doc states when a
request at it certifies); `BoundNotFinite`, carrying that smallest
finite bound or none, when the last grid's bound is not finite;
A9.10's downward knot-removal compression is not built.
`OffsetCertificate` has two limbs: `on_locus_max`, a sampled residual
that steers, and `hull_sup`, the certified bound via
`spline::compose::patch` over the rationalized composites
`X = Ẽ·Ẽ − d²·w̃²` and `Y = Ẽ × M̃`, which are small exactly when the
fit is good, plus the sign witness `D = Ẽ·M̃`. `D` does double duty:
it proves `E·n` carries `d`'s sign, and it floors `‖E‖` at
`|D|/(w̃·‖M̃‖)` — the reading of the three components TOGETHER, which
is what keeps a small-`|d|` bound near `|d|`'s own scale where the
componentwise mignitude of `Ẽ` collapses on a rotating normal. A
rational base is ordinary here: its weights decide only which terms
`M̃ = w³·m` carries, and the rational quarter cylinder certifies
through this door — `offset_surface` is where a NURBS base refuses
typed, `NotClosedUnderOffset` on every one of them, rational or not,
because normalizing the chart normal is what breaks rationality. This
fit is the approximating-surface route that refusal names.

**O4 — What shell is.** `shell(B, t) := B − offset_inward(B, t)` by
definition, boolean-family; its execution never runs the crossing
pipeline. An open shell is unrepresentable by construction (every edge
has exactly two half-edges; D1 is manifold-first), and shell does not
need one. *Sealed* (`topo::shell`): one clone with every boundary face
moved to its inward offset, every chart of a solid at once (a chart is
body-wide, and each solid's own faces are grouped by chart; all-planar
solids through `offset_planes_together`, which solves each corner
against all moved planes; planes meeting revolved walls through
`offset_charts_together`; anything else through
`offset_surfaces_together`, which derives each edge as the section of
its two moved surfaces, transporting it only where both sides' offset
actions agree on it, and each corner as a root of the moved surfaces
meeting it; the first two are its closed forms, and
`replace_faces_offset` is its one-chart spelling), then inserted
through the shared void-insertion door `boolean::voids::insert_void`
with the construction's own d-vs-reach margins carried as
`VoidContainment::Carried` evidence; the door never derives
containment. Every boundary thickens, the outer shell inward and each
void outward by the same signed distance (a void's faces are wound with
their material side outward), and each void moves with its dilated twin
into a solid of its own: for `k` voids the result holds `k + 1` solids
and `2(k + 1)` shells, every operand shell surviving under its key. The
invariant this
preserves is that every cavity is born through that one door (three
producers: boolean subtraction, shell, the full revolve of a holed
profile). *Opened* (`shell_open`): the sealed construction, then each
designated chart's cavity counterpart offset back outward onto it and
the pair reduced by rim surgery (`canonicalize_chart`, `kfmrh`) to one
annular rim face. Nothing opens; the result is closed, the designated
shell's thin solid is single-shell (a hollow operand's other thin
solids keep their two), and the invariant is closure, not genus (one
opening is a cup, genus 0). Refusals: a wall past a curved face's reach
at O1's floor, inverted cavity walls at edge re-attachment, two facing
planar walls closer than `2t` (`WallClearance`), and two planar walls at
an angle whose moved regions cross (`OffsetsCross`); curved walls
have no clearance gate yet (Open, below). A fitted
face's edge with a plane is their section over the fit through the old
edge's middle (`PlaneSection::branch_at`, C5): the branch there, the
fit's boundary row where a `Side` region holds the middle (the plane
holds that row within the band), a refusal by the corner where a
`Corner` region does, and `NoBranch` where the middle settles outside
the fit's window; and a row
of its fit whose sides both move with it — a seam the face shares with
itself, or a plane along whose normal every cross-row step of the old
net's two rows at that end runs, all one way, the two rows' weights
equal (which puts the fit's normal in the plane along the row; a plane
containing it otherwise takes the section) — is extracted from the new
fit, at the row its iso image names. Its corners are the planes' roots
along those edges, a section's sought from its end at the corner; the
fit itself is not rooted, and a corner no root of which lies on a moved
fit refuses (`CornerSection`, naming the corner's vertex and an edge of
the fit meeting it). A fitted face's edge with a spline face or another fit, moved or held,
is their NURBS × NURBS section through the old edge's middle
(`Section::branch_at`), in the old edge's sense; a refusal of that
trace is reported as from the old edge's middle (`FromMiddle`).
Measured: a crease
between two moved fits derives there (the shelled twisted loft's four
wall–wall seams at ε = 1e-6), and a seam where the two are tangent (a
smooth one) refuses by the section's transversality decision. A wall
moved alone beside a held spline wall refuses at that seam, typed by
whichever reading of the section meets it first: inward, the boundary
pass does not resolve the side the section runs along
(`PairBoundaryUnresolved`); outward, the old edge's middle settles
outside the fit's window, the moved fit no longer meeting the held wall
there (`NoBranch`, as at a plane), or, where the move is far inside ε, the
boundary pass refuses as inward or the trace loses the section at the
side (`CrossingUnmatched`, or the step-progress decision). A concave
crease's section lies outside both fits' windows; no row measures its
verdict. Its other boundary refuses: `Approx`
against a curved analytic face (`NeighborPairUnroutable`, naming
`Approx`); by
`FittedBoundaryUnsupported`, a curve on the fit that is no line of its
chart, or a line along a seam the face shares with itself that holds
neither parameter fixed, a scaffold edge, and a seam the face shares
with itself not described as an image on its own chart; a section
outside the fit's window (the section's own verdict); and a scalar with
no fit or section lane (`ApproxLaneUnsupported`,
`NurbsLaneUnsupported`).

**O5 — The validator re-derives per face, as it does per edge.** Tier 3
never trusts a stored certificate: `validate.rs` re-runs the O3
derivation on every `Approx` face on every call through
`geom_brep::OffsetFitLane::recertify`, the door the pass takes as a
parameter (`ApproxCertification` on failure; `ApproxLaneUnsupported`
where the scalar's seam hands the pass no door). The
stored `OffsetCertificate` is provenance, kept for reporting. `Approx`
faces inherit the NURBS-adjacent exemption from dihedral marks (C7).

**O6 — Sequencing and the demo gates.** The units are landed in the
order O1 → O3 meters and fit → O2 storage → face replacement and shell.
Live gates, each named where it refuses: a hollow solid refuses STEP
export (`VoidShellUnsupported`, the writer has no `BREP_WITH_VOIDS`);
the area enclosure is unmetered; rational
pcurve quadrature refuses typed; the Klein bottle's BULB wall pairs wait on
the `cylinder × torus` and `cone × torus` arms — its own two rims. The
bulb has no cone-abutting-cylinder adjacency at all: its flare is
bracketed by two tori, by construction.

## Edge descriptions

How `geom-brep` keeps DESIGN `[edges]`: the text DESIGN held before its
rules and mechanics were split (2026-10-18), verbatim. Bracketed ids are
`docs/DESIGN.md`'s clauses.

**Background: pcurves.** A surface is a map `S(u,v) → ℝ³`. A face is a
region of that surface's parameter plane, and each boundary edge is
therefore also a curve `P(t) → (u,v)` in that plane — the *pcurve*. An
edge shared by two faces classically carries three representations: a
3-D curve plus one pcurve per adjacent face, with the consistency
requirement `Sᵢ(Pᵢ(t)) ≈ C(t)`. Pcurves are not optional — trimming,
tessellation and intersection marching happen in (u,v) — and the
redundancy among peer representations is a classic bug farm.

**Our rule:** an edge's geometry is stored as an *intensional
description* of what the locus **is**; every concrete representation
(the 3-D carrier, both pcurves) is a derived cache carrying a certified
residual bound against the described locus ([tolerance.residuals]). The type is
`EdgeDescription<T>` (`crates/geom-brep/src/description.rs`):

```text
EdgeDescription =
  | Intersection        { pair, selector }   -- a curve of S₁∩S₂, `pair` the set
                                             -- {S₁, S₂}, unordered; the selector
                                             -- picks which curve: a witness
                                             -- (the branch through a point, also
                                             -- the marching seed) or a structural
                                             -- selector (a construction's
                                             -- pushforward); whether the surfaces
                                             -- cross (read `Intersection`) or touch
                                             -- to order k (read `TangentIntersection`)
                                             -- is what the certificate reads, never
                                             -- a second stored record
  | Chart(ChartCurve)                        -- a curve ONE surface's chart defines:
                                             -- (surface, Pcurve) with a `wrap` flag
                                             -- (iso-lines, seams, same-surface splits)
  | Scaffold(MappedCurve)                    -- construction-time pushforward of a
                                             -- lower-dim entity; never at rest
```

A two-surface edge is always a curve of the two surfaces' intersection,
and its selector says which: no two-surface edge is conventional. The
selector follows one question, asked of the geometry and never of where
the body came from: **is the curve known in advance?** Where it is —
the surfaces' contact or crossing has a closed form (a ruling where a
plane meets a cylinder or a swept wall, a circle where coaxial surfaces
of revolution meet, a row or column of a swept or skinned wall), or the
curve was chosen to lie in a region where the two surfaces overlap in
2-D — the selector is **structural**: that closed form, picking the
curve exactly whatever the contact order, certified by its own meter and
incidence on each surface, each read through that surface's own
chart-image certificate (the one meter every pcurve carries). A
construction that knows the curve (a profile vertex swept, a section
placed, a section vertex skinned through a loft's sections) states that
closed form directly, and an import recovers the same closed form from
the same geometry, so a native body and its imported copy carry the same
selector: the closed form is a property of the surfaces, not of the
history. Where the curve must be derived numerically from the geometry
— a crossing or tangency with no closed form — the selector is a
**witness**, the branch through a point, admitted only where its margin
certifies: transversality for a crossing, the order-k jet system's
margin for a tangency of order k, each taken across the locus, over the
faces, never along the edge. A construction therefore never classifies
its own joins by measurement, and the sketch's joint class is the one
reading of their kind. A curve no lane can yet select refuses typed,
naming the lane, native or imported alike; a coincidence of two surfaces
in band is refused at the structural act that made them adjacent. A locus one chart defines
(parameterization seams — infinite-order contact, the seam's position
pure convention; iso-lines; splits whose two faces share one surface;
two records that read as one carrier) is `Chart`: one (surface,
`Pcurve`) form covers all of them (PCURVE-UNIFY-DESIGN); the exact
classes stay as certification lanes, not variants. A `MappedCurve`
survives as the `Scaffold` payload, as a structural selector and as the
`EdgeAuthority` record — one authoritative source, never two peer
representations needing cross-reconciliation.

**Deliberately omitted: an `Explicit` (extensional) variant.** Taken as
an unconditional challenge: every edge must have an intensional
description — there is no escape hatch, so it cannot be reached for
when not absolutely necessary. This holds for imported geometry ([import]):
the intrinsic variants are checkable properties of the geometry as it
now stands, and a closed-form selector is a property of the surfaces,
so extensional input is *adopted* by recovering the selector a native
body would carry, or refuses typed where no lane recovers one. What import pressure-tests is the completeness of the
taxonomy (imported fillets force `TangentIntersection`), not the need
for a fallback.

**Validity predicates with margins.** `Intersection` requires
*transversality*: normals of S₁, S₂ linearly independent along the
locus, so the implicit function theorem makes S₁∩S₂ locally a
1-manifold; the margin (angle between normals) governs the conditioning
of every derived cache. A branch may end at a *node*, a point where
it crosses another branch of the same pair and the two surfaces are
tangent (two equal cylinders with intersecting axes: their section is
two ellipses crossing at p ± r·unit(â₁×â₂)); there the section is one
connected set, so the witness selects the branch, and transversality
holds on the open edge, which is where certification samples it. A tangency's validity sits one
differential order up: surfaces coincident within ε and normal-parallel
within the derived angular threshold ε·κ_rel ([tolerance.epsilon]: lever arm
r = 1/κ_rel) *along* the locus, separating quadratically *transverse*
to it (relative normal curvature bounded away from zero — otherwise the
surfaces osculate over a patch and the "locus" is not a curve).
Reconstructing a tangency locus is well-conditioned *despite* the
tangency because its defining system includes the first-order
normal-alignment equations: the normal angle grows linearly with
transverse distance, and the curvature difference is the
implicit-function-theorem denominator for that jet system. Order-k
contact generalizes: defining equations from the k-jet, margin at order
k+1, so a curvature-continuous join (k = 3) is read as a tangency where
its third-order separation certifies, not left under-determined. In the intrinsic variants "the locus lies on both surfaces" holds
*by definition*; only the caches need certification. Vertices
generalize the same way (three surfaces / endpoint of a locus, with a
witness). A locus in the ambiguous band — a dihedral within a few
derived angular thresholds of tangent, certifiable as neither — fails
loudly at construction exactly as at import; a conventional description
is not an escape hatch from ill-conditioned geometry. An edge whose
stations read definitely transverse at some and definitely smooth at
others is such a locus: the dihedral is continuous along it, so the two
readings bracket an in-band point the schedule did not sample, and it
refuses typed, naming the two stations.

**Prefer-intrinsic rule, tier-3-enforced.** Every two-surface edge is
intrinsic, and its selector is decided by the geometry alone: a closed
form where one exists, a witness where the curve must be derived. A
fillet we build carries its rolling ball's contact rail; a sweep carries
the ruling its profile vertex sweeps out, whatever the joint's contact
order; an imported copy of either recovers the same closed form. A
closed form that selects the curve is description, not history; [identity]
provenance stays the record of who built the edge, and native and
imported bodies carry identical descriptions. At rest, a witness
selector must certify its reading:
a definitely-transverse locus as `Intersection` (`TransverseNotIntrinsic`
otherwise), a jet-determinate tangency as `TangentIntersection`
(`TangentNotIntrinsic`, where `geom_brep::tangent_certificate_lane`
admits the class). The rule needs two surfaces and a definite reading, so
a wrap edge (one surface) and an edge whose dihedral is not read
definitely (in band, or beside a spline face) fall outside it.
An unenforced preference drifts silently — exactly the failure shape this
document exists to kill.

**Witness contract.** "Selected by the witness point" is verifiable only
if the witness is *pinned*: the stored witness IS the carrier's
mid-parameter point, enforced by certification (`WitnessMidpoint`).
Pinning removes the aliasing freedom that let any point on the
component certify, including points encoding a wrong winding; residual
freedom is documented where it is geometrically invisible (circles:
whole-period translation). Every op that mints an `Intersection`
computes the witness as carrier(mid) in the certification schedule's
own association order.

This makes [identity]'s provenance load-bearing rather than bookkeeping: the
intensional description largely *is* the provenance.

## Open

- Shell's curved wall clearance: two facing curved walls closer than
  `2t` shell to a self-intersecting cavity that tier 3 does not catch
  (`wall_clearance` gates planar operands only). Ev ruled B on #1737:
  the clearance engine's body-level half moves into `topo` behind the
  interval scalar, and the curved gate at the shell door reads it.
  The engine has not moved yet (`editor_core::clearance`); the work is
  CLEAR's (`shell-curved-wall-clearance-window`, parked on
  `shell-curved-clearance-consumer`).
- Corner taxonomy: the uniformly concave trihedron is carved but has no
  `CornerConfig` tag; the finer run-out taxonomy (per-end assignment,
  setback parameters) is reserved for the design that implements
  run-outs.

## Parameterization conventions

Moved from DESIGN `[topology]` on 2026-10-17, verbatim. Bracketed ids (`[topology]`, `[coincidence]` …) are `docs/DESIGN.md`'s clauses.

- **Parameterization conventions** (authoritative text in the `geom`
  crate docs and its `curves`/`surfaces` modules): curve entities are
  complete loci; an edge's bounds derive from its vertices via the
  `he_plus`-forward contract (certification enforces forward, nonzero,
  ≤ one-period spans, the stored interval a certified cache reconciled
  against vertex authority). Shared azimuthal frame for all revolution
  surfaces (axis = +a₃, v_ref = axis × u_ref, seam at u_ref — for
  revolved bodies u = 0 IS the profile half-plane); sphere uses
  latitude; cone v = slant length with the apex a true chart
  singularity (poison normal, never sampled); normals are the chart's
  ∂u × ∂v with no "outward" contract — topology carries sense. A seam
  (a **wrap edge**) is a property of a face, not a place on its
  surface: an edge both of whose sides bound one face, across which
  that face's chart domain closes in one of its periodic directions.
  Where it sits is where the construction cut the wall — the profile
  vertex — read off the edge's carrier through the chart (a meridian
  wraps u, a torus parallel wraps v, a closed spline net's boundary
  column wraps u), so the mirror-nappe cone's chart u = 0 needs no
  spatial pin. A wall keeps the cuts its author wrote and the kernel
  adds none: a one-segment closed loop sweeps to one wall with one wrap
  edge, a k-piece loop to k walls on one surface key.
