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
| C1 locus ladder | `crates/geom/src/curves.rs` (`Curve3`: Line, Circle, Ellipse, Nurbs); `crates/geom-brep/src/intersect.rs` (`Rung`) |
| C2, C3 SSI and its certificate | `crates/geom-brep/src/ssi.rs` + `ssi/{march,certify,exhaust,enclose,jet,system}.rs` |
| C4 pcurves | `crates/geom-brep/src/pcurve_cache.rs` (value, certificate), `pcurve.rs` (conic constructors), `crates/topo/src/pcurves.rs` (storage, minting, branch walk); description form in `description.rs` |
| C5 dispatch table | `crates/geom-brep/src/intersect.rs` (`route`, the section functions) |
| C6 f64 structure vs generic certification | `crates/geom-core/src/spline/`, `crates/geom/src/curves/fit.rs` |
| C7 tangency | `crates/geom-brep/src/tangent.rs`, `enters.rs`; marks in `crates/topo/src/validate.rs` (`ContactMark`) |
| C8 fillets | `crates/sweep/src/blend/` (see `crates/sweep/README.md`) |
| C9 interval ring | `crates/geom-core/src/ring_interval.rs`, `spline/hull.rs`, `spline/compose/{tensor,patch}.rs` |
| C10 BVH | `crates/bvh` |
| C11 NURBS substrate | `crates/geom/src/{curves,surfaces}/nurbs.rs`, `curves/fit.rs`, `*/projection.rs`; lofts in `crates/sweep/src/{loft,skin}.rs` |
| C12 consumers | `crates/topo/src/splitting/`, `boolean/`, `merge_faces.rs`; `crates/mesh/src/curved.rs`; `crates/geom-brep/src/props/quad.rs`; `crates/geom-core/src/linalg/{svd,lsq}.rs` |
| O1 analytic offset mint | `crates/geom-brep/src/offset.rs` |
| O2 approximating surface | `crates/geom/src/surfaces/approx.rs` (`Surface::Approx`) |
| O3 meters and fit | `crates/geom-brep/src/offset_meters.rs`, `offset_fit.rs`, `patch_bound.rs` |
| O4 shell | `crates/topo/src/shell.rs`, `boolean/voids.rs`, `replace_face.rs`, `offset_together.rs`, `offset_axial.rs` |
| O5 validator posture | `crates/topo/src/validate.rs` (`recertify_approx`) |

## Curved geometry (CURVED-DESIGN C1–C12)

### The locus ladder and its certificate

**C1 — Intersection loci live on a three-rung ladder.** The *carrier*
of an `Intersection` edge (the 3-D curve cached against the intensional
description `{s1, s2, witness}`) is, by surface-kind pair and most exact
first: rung 1, closed-form `Line`/`Circle`; rung 2, the exact conic
`Curve3::Ellipse` (tilted plane×cylinder, equal-radius cylinder×cylinder),
whose residual is zero by construction; rung 3, a fitted cubic
`Curve3::Nurbs` carrying the C2 certificate. Parabola and hyperbola are
outside the inventory by decision: a generic-tilt plane×cone routes to
rung 3 permanently. Conics round-trip to rational-quadratic NURBS only as
an export/tessellation form, never as the kernel carrier (the axes and
centre are what dispatch consumes). There is no polyline rung.

**C2 — A fitted carrier's certificate has three limbs, all mandatory.**
(1) On-locus residual at the fixed `CERT_SAMPLES` schedule: `|f(C(t))|`
in metres for an analytic operand (`implicit.rs`); for a NURBS operand
`|C(t) − S(u*,v*)|` at a certified foot point whose orthogonality
residual is banded too, so a bad projection cannot launder a bad cache.
(2) Sup-norm honesty between samples, by control-coefficient hull bounds
in the C9 ring: `geom_core::spline::compose` composes the implicit form
with the carrier (converted to metres exactly for plane, cylinder and
sphere; cone and torus need a root the ring lacks, which is why their
rung-3 arms are unretired), and `compose::tensor` encloses
`S(P(t)) − C(t)` as one composite for a NURBS operand so the
cancellation that is the whole content of the claim survives into the
bound. (3) The uniqueness tube: over a chain of boxes of certified radius
around the carrier, the enclosure of `(∇f₁ × ∇f₂)·e` (or the chart form
for plane×NURBS) excludes zero, so by a mean-value argument each slice
holds at most one solution and the solution set in the chain is one arc.
The tube says nothing about a disjoint component at other `e`-levels;
that is C3's exhaustiveness obligation, a separate theorem. A straddling
enclosure is a genuine sliver of the operand pair and escalates
(`ssi_tube_transversality`, `SsiError::TransversalityBand`), never a
retry. Hull bounds are an entry requirement: no schedule-max-only
certificate ever reaches an at-rest body, and the tube is required for
every fitted `Intersection`, not only where several branches were found.
The witness is `carrier(mid)`, minted from the cache the schedule sees.

### Surface–surface intersection

**C3 — March, then certify; the stepper is trusted for nothing.** No
per-step predicate can prove "no other branch within reach" from local
data, so none is asked to. `ssi/march.rs` is a candidate generator
(Hoffmann §6.2: third-order local approximant from the underdetermined
jet system solved by SVD, Frenet choice of free coefficients, Newton
refinement; all f64, libm-only, fixed iteration order). Two trace shapes
are compile-time decisions per table arm: an implicit pair in ℝ³ (2×3
SVD; `cylinder_sphere_ssi`) and a parametric pair in ℝ⁴ on
`G₁(u₁,v₁) − G₂(u₂,v₂) = 0` (3×4 SVD; `plane_nurbs_ssi`), from which the
3-D curve and both pcurves fall out as projections of one traced object
on one shared parameter. A branch jump is a certificate refusal.
Exhaustiveness is an in-op obligation (`ssi/exhaust.rs`): every cell of
the bounded domain is *excluded* (an implicit residual bounded away from
zero by enclosure), *accounted* (contained in a found branch's tube), or
refined to the named floor, where the op refuses
`SsiError::ExhaustivenessInconclusive`. The op does not return until
every branch is found or it refuses; the subdivision doubles as the seed
generator, so finding never depends on luck. Closure of a trace and loop
topology are named trileans on parameter-space distances. Near-tangential
configurations (the transversality band along the trace) refuse toward
C7; Hoffmann §6.5's tracing through singular points is deliberately not
adopted. Subdivision is recursive bisection with a linear scan over
tubes; the C10 tree is not wired in.

### Pcurves

**C4 — Pcurves are per-half-edge certified caches, certified in metres
through the map.** A *pcurve* is an edge's image in a face's `(u,v)`
chart. Its home is the half-edge (`Body::pcurves`, a
`SecondaryMap<HalfEdgeKey, PcurveCache>`): a seam edge has both
half-edges on one surface with two chart images (`u = α` and
`u = α + 2π`), so no coarser key works. Its parameter *is* the carrier's
`he_plus`-forward parameter; traversal sense per face is derived, never
stored. `PcurveCache::certify` is the only constructor. The certified
statement is `|S(P(t)) − C(t)| ≤ ε`, a 3-D displacement at the shared
schedule, plus a between-samples envelope whose own statement the
certificate names (`EnvelopeStatement`): closed-form over the whole span
for `Pcurve::Harmonic` (both sides in `span{1, cos t, sin t, t}`, so a
corruption hiding between samples is unrepresentable), hull-bounded for
fitted images on NURBS charts, and only the carrier's incidence with the
chart surface (`OnLocusHull`) for a fitted image on a periodic analytic
chart, where `S ∘ P` is transcendental. No UV-space tolerance appears in
any certified statement; the chart's stretch is the lever arm. Domain
validity is part of the certificate: one branch pinned at the start (a
τ jump is unrepresentable in `Harmonic`'s `α + β·t`; the branch per face
is chosen once by the loop walk in `topo::pcurves` and certified by loop
continuity) and trim containment against the caller's `ChartWindow`
(`TrimEscape`). Planar faces store nothing; `chart_pcurve` derives on
demand. The lanes: `Harmonic`, `IsoLine`, `IsoArc`, `Fitted`, `General`
(the general curve-in-UV at the honest fitted grade). Carrier-primary
stands: the 3-D carrier is the authoritative machinery and the edge's
parameter stays chart-neutral. The description form every conventional
edge takes is `EdgeDescription::Chart { surface, pcurve, seam }`, with
`EdgeAuthority` recording who declared the locus; that collapse and its
fence are `docs/PCURVE-UNIFY-DESIGN.md`, not restated here. Volume, area
and tessellation still refuse typed on a face carrying a `General`
pcurve.

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
radii are structural or declared (`RadiusEvidence`), never inferred from
values. Tangential outcomes (`TangentLine`, `TangentPoint`) are
classification data, refused as carriers. `SurfaceKind::Approx` is its
own kind, not `Nurbs`: a locus claim against an approximating surface is
a claim about the fit, and `Approx × anything` refuses because composing
the fit's precision claim with the SSI limbs is not a ratified rule.
Implemented: plane×plane, plane×sphere, sphere×sphere, axis-aligned
plane×torus (rung 1); plane×cylinder, exact-degenerate plane×cone,
declared-equal cylinder×cylinder (rung 2); cylinder×sphere and
plane×NURBS (rung 3). Every other pair refuses, most blocked on the cone
and torus metres conversion (C2 limb 2).

### Fitted-cache structure

**C6 — Cache structure is an f64-lane artifact; certification is
scalar-generic.** Knots, weights, degrees and every combination
coefficient are `f64` structure (`geom_core::spline`); control points are
the only generically typed data; the fitting loops (`curves/fit.rs`)
take `f64` points. The certificate re-evaluates against the pinned
structure at any `Real`, so the interval lane proves what the f64 lane
chose. No topology-determining predicate reads knot counts, spans or
fitted coefficients except through named certified margins; the name
table is a function of recipe structure and verdicts only.

### Tangency

**C7 — `TangentIntersection` and second-order sector classification.**
`EdgeDescription::TangentIntersection { s1, s2, witness }` mirrors
`Intersection` one differential order up. Its jet (`tangent.rs`,
`TangentJet { sin_theta, kappa_rel }`) is certified per sample: surface
coincidence within ε, normal parallelism within the derived angle at
lever arm `1/κ_rel`, and the relative transverse normal curvature
`κ_rel = κ₁ − κ₂` bounded away from zero (the IFT denominator of the jet
system), with C2's hull bounds and tube between samples. Where
first-order sector ranking ties exactly, classification descends one
order (`enters_material_order2`, consumed by
`topo::splitting::neighborhood` and `topo::boolean::sectors`); an in-band
second-order tie escalates. Tier 3 has two levels: every
definitely-tangent edge carries a recorded `ContactMark` (`Tangent` when
jet-determinate, `SmoothUnderdetermined` when the surfaces
under-determine the locus, e.g. a G2 sketch join), and the must-carry
rule `TangentNotIntrinsic` fires only on `ContactMark::Tangent`. NURBS-
and `Approx`-adjacent edges are exempt by kind (`Unmarked`). Curved
contact census is CONTACT-DESIGN's, at `crates/topo/README.md`.

### Fillets

**C8 — Fillet validity is reified predicates, evaluated before any
construction; blends are analytic-first.** Implemented in
`crates/sweep/src/blend/`; `crates/sweep/README.md` is the reference.
What binds from here: the six named margined predicates over the inputs
run in order before any ball exists (radius vs `1/κ_max` of each
support, face clearance, spine regularity, chain G1, convexity-sign
consistency, corner configuration), which is what lets an interval
replay certify validity over a parameter box. Every constant-radius arm
mints a torus or a cylinder (the envelope of equal spheres over a circle
or a line spine); a cone belongs to the variable-radius family.
Trimlines are stored as `TangentIntersection`. Scope: the
three-convex-edge sphere-octant corner is in; every other corner refuses
with a `CornerConfig` tag and the `RunOutPolicy` that would handle it
(`RunOutStopAtVertex`, `RunOutFeather`), refusal-payload names only. A
spine that is neither line nor circle refuses `SpineUnsupported`: the
canal-surface blend, an approximating surface per O2, is not
implemented.

### Arithmetic substrate and the BVH

**C9 — Enclosures run on an in-house interval ring.** Every enclosure
certification needs is transcendental-free (implicit residuals are
polynomial, de Boor is ring arithmetic, hull bounds are convexity facts),
so `geom_core::RingInterval` provides `±`, `×`, `÷` and integer powers
with unconditional outward ulp-widening, always compiled, MIT-clean, not
a `Real`. It is certification substrate; the evaluation scalar
`geom_core::Interval` (behind the `interval` feature, backend the in-repo
`interval-transcendentals` crate) is a `Real` instantiation for replay.
No copyleft dependency exists in any build configuration. Certification
code reads brackets through the `Bounds`/`Enclosure` traits.

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
multiplicity), point projection with certified orthogonality residuals,
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
curved-cut faces are certified quadrature in the ring (`props/quad.rs`):
harmonic pcurve boundaries, polynomial and rational patch flux; rational
pcurve channels refuse `QuadratureUnsupported`; exhaustion is
`QuadratureBudget`, never a silent Gaussian. (8) In-house SVD and
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
shift). Refusals are named predicates over the *realized* stored float,
decided before any mint: `offset_radius_floor` (margin `radius + d`;
`OffsetError::RadiusFloor`) and `offset_torus_ring` (margin
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
not built); `SurfaceSpec { description, fit, window, tolerance }` is the
uncertified input; `ApproxSurface::certify(spec, certifier)` is the sole
door and its fields are private, so an uncertified approximating surface
is unrepresentable. The certifier is injected because the derivation
lives one crate up (`offset_fit.rs`) and is `f64`-only. The base is an
owned `Arc`, not an arena key (layering, and `Surface` values travel
without an arena), and it is NURBS by type: analytic bases mint exactly
under O1 and never reach this door. Storage is the seventh variant
`Surface::Approx(Arc<ApproxSurface>)`, so every dispatch site must say
what it does with one (most delegate to the fit; kind-indexed tables
treat it as its own kind, C5).

**O3 — The certificate is C2 lifted, on two meters the fit needs.** The
claim is `sup_(u,v) ‖S_fit − (S + d·n)‖ ≤ ε_precision`, pointwise in the
base's own chart parameters. Meters (`offset_meters.rs`, read off
`patch_bound.rs`'s per-cell control-hull enclosures): the *regularity
floor*, a certified lower bound on `‖S_u × S_v‖` (three assemblies, the
largest wins: componentwise mignitude, fixed-direction projection, and
the Gram determinant `EG − F²`), classified by `offset_normal_floor` with
the patch's faster chart speed as lever, deliberately not `|d|`, since
whether the normal degenerates does not depend on `d`; and the
*collapse headroom*, principal curvatures `[κ_lo, κ_hi]` from the closed
form of the two fundamental forms, refusing through
`offset_curvature_headroom` when `|d|` reaches `1/κ` on the folding side.
Both bound conservatively (they can refuse a regular patch, never accept
a degenerate one). The fit (`offset_fit.rs`) is the NURBS Book's A9.4
grid interpolation at the base's own parameters, then a
certify-and-insert loop that refines the cells carrying the sup until
every cell certifies or `BudgetExhausted` refuses carrying the achieved
bound; A9.10's downward knot-removal compression is not built.
`OffsetCertificate` has two limbs: `on_locus_max`, a sampled residual
that steers, and `hull_sup`, the certified bound via
`spline::compose::patch` over the rationalized composites
`X = Ẽ·Ẽ − d²·w̃²` and `Y = Ẽ × M̃`, which are small exactly when the
fit is good. A rational base refuses typed.

**O4 — What shell is.** `shell(B, t) := B − offset_inward(B, t)` by
definition, boolean-family; its execution never runs the crossing
pipeline. An open shell is unrepresentable by construction (every edge
has exactly two half-edges; D1 is manifold-first), and shell does not
need one. *Sealed* (`topo::shell`): one clone with every boundary face
moved to its inward offset (all-planar bodies through
`offset_planes_together`, which solves each corner against all moved
planes at once; planes meeting revolved walls through
`offset_charts_together`; anything else chart by chart through
`replace_faces_offset`, whose oblique corners refuse
`ReanchorOffCarrier`), then inserted through the shared void-insertion
door `boolean::voids::insert_void` with the construction's own
d-vs-reach margins carried as `VoidContainment::Carried` evidence; the
door never derives containment. The result is a two-shell solid, and the
invariant this preserves is that every cavity is born through that one
door (three producers: boolean subtraction, shell, the full revolve of a
holed profile). *Opened* (`shell_open`): the sealed construction, then
each designated chart's cavity counterpart offset back outward onto it
and the pair reduced by rim surgery (`canonicalize_chart`, `kfmrh`) to
one annular rim face. Nothing opens; the result is closed and
single-shell, and the invariant is closure, not genus (one opening is a
cup, genus 0). Refusals: a wall past a curved face's reach at O1's floor,
inverted cavity walls at edge re-attachment. A NURBS-walled body still
cannot be shelled: `Approx × anything` has no C5 arm, so the
face-replacement door refuses on a fitted face's intrinsically described
boundary.

**O5 — The validator re-derives per face, as it does per edge.** Tier 3
never trusts a stored certificate: `validate.rs` re-runs the O3
derivation on every `Approx` face on every call through
`PropsQuadLane::recertify_approx` (`ApproxCertification` on failure;
`ApproxLaneUnsupported` on a scalar lane that cannot derive it). The
stored `OffsetCertificate` is provenance, kept for reporting. `Approx`
faces inherit the NURBS-adjacent exemption from dihedral marks (C7).

**O6 — Sequencing and the demo gates.** The units are landed in the
order O1 → O3 meters and fit → O2 storage → face replacement and shell.
Live gates, each named where it refuses: a multi-shell curved solid
refuses STEP export (`CurvedShellClassification`, the outward/void
classifier is planar-only); the area enclosure is unmetered; rational
pcurve quadrature refuses typed; the Klein bottle's BULB wall pairs wait on
the `cylinder × torus` and `cone × torus` arms — its own two rims. The
bulb has no cone-abutting-cylinder adjacency at all: its flare is
bracketed by two tori, by construction.

## Open

- Shell's curved wall clearance: two facing curved walls closer than
  `2t` shell to a self-intersecting cavity that tier 3 does not catch
  (`wall_clearance` gates planar operands only); the general clearance
  certificate over a parameter box belongs to the error-propagation
  lane (`docs/ERROR-DESIGN.md`).
- `replace_face::mint_offset` does not discharge `ConeOffset`'s
  mirror-nappe obligation (`offset_axial::nappe_signed` does).
- Corner taxonomy: the uniformly concave trihedron is carved but has no
  `CornerConfig` tag; the finer run-out taxonomy (per-end assignment,
  setback parameters) is reserved for the design that implements
  run-outs.
