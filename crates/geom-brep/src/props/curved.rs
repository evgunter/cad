//! Closed-form flux/area for the curved M2 surfaces, over every loop
//! of a face, holes included ([`curved_face_loops`]): cylinder and
//! torus faces by their chart Green forms, cone faces by their
//! boundary's vector area, and sphere faces bounded by circle arcs by
//! Gauss–Bonnet (see [`super`] module docs for the formulation and the
//! stored-data discipline).
//!
//! **Not everything public here serves that lane, and neither door may
//! be cited by it.** This module hosts two structural predicates beside
//! the flux arms: [`require_iso_rectangle`], the shape door, and
//! [`require_one_chart_branch`], the BRANCH door, both `mesh`'s. The
//! flux lane reads no chart extent on the sphere and measures a
//! pole-crossing arc EXACTLY (rows of
//! `geom-brep/tests/cert1_sphere_polar.rs`), so citing the branch door
//! from [`curved_face`] or from `mass_properties` would retract those
//! rows.
//!
//! Classification vocabulary: a boundary edge of a revolution surface
//! is a **rim** (iso-`v` circle about the surface axis — carrier axis
//! parallel or antiparallel to the surface axis) or a **meridian**
//! (iso-`u`: an axial/generator line, a great circle through the
//! sphere's poles, or a torus minor circle — carrier axis
//! perpendicular to the surface axis). Every decision goes through the
//! crate's recording funnel with a `props_*` predicate name; a
//! definite-nonzero consistency residual or an out-of-inventory shape
//! is a typed [`PropsError`].

use geom::Curve3;
use geom::Surface;
use geom_core::period::{Exact, Extent, NotForward, TurnRate, TurnRows};
use geom_core::{
    Band, Decide, Indeterminate, InfSpeed, LeveredUnitError, Margin, Point3, Real, RigidMotion,
    Sign, UNIT_DIRECTION_ARM, UnitVec3, UnitVec3Error, Vec3,
};

use super::closure::{self, ChartLoop, EdgeTurn, Joint};
use super::{FaceContribution, LoopEdge, PropsCheck, PropsError, loop_vector_area};
use crate::dihedral::decide;
use crate::implicit::cone_offset;
use crate::pcurve_cache::{BranchMiss, Period};

/// The flux and area of a curved face from its **outer** loop:
/// [`curved_face_loops`] over that one loop. `band` is the run's
/// linear band, built once at operation entry.
///
/// `sense` is the face's orientation BIT (`topo::Face::sense`): `true`
/// where the surface's chart normal already points out of the
/// material, `false` where the face reverses it. It is
/// **deliberately not applied to every term**: `A⃗` and every chart
/// Green form are recovered from the face's STORED LOOP TRAVERSAL,
/// which the interior-left rule already ties to the outward normal —
/// `revert` reverses loops and flips `sense` together, so signing
/// those terms by the sense as well would double-count and negate the
/// volume twice. `sense` is consumed by the sphere alone
/// ([`sphere_face`]), whose radial term and Gauss–Bonnet area are read
/// against the outward normal the bit gives, after
/// [`sphere_loop_side`] has held the bit against the boundary.
///
/// # Errors
///
/// [`PropsError`] — unimplemented kinds, out-of-inventory boundary
/// shapes, definite consistency failures, escalated classifications.
pub fn curved_face<T: Decide>(
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    sense: bool,
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    curved_face_loops(surface, &[outer], sense, band)
}

/// The material side a curved face's **boundary traversal** encodes —
/// [`boundary_material_sign`]'s answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialSign {
    /// The boundary encodes the side: `Sign::Positive` ⇔ the stored
    /// traversal places the material where a `sense: true` face's
    /// outward normal (`+chart_normal`) claims. Definite
    /// (`Positive`/`Negative`) by construction: every factor is a
    /// definite `classify` outcome — the `Zero` arms refuse typed
    /// before a sign is minted.
    Encoded(Sign),
    /// The boundary does not encode the side, so the face's bit is its
    /// one encoding and nothing cross-checks it — the documented
    /// residual of the curved sense gate. Two one-loop sphere faces
    /// answer this ([`sphere_loop_side`]):
    ///
    /// * one with both chart poles on its loop (a lune, a hemisphere
    ///   split at its poles), whose loop bounds the face and its
    ///   complement alike with neither holding a pole. Tier 3 reads
    ///   such a face through the region it forms with its seam
    ///   siblings, the faces of its surface it reaches across shared
    ///   edges, where that region's boundary encodes one
    ///   ([`sphere_region_material_sign`]). A region whose boundary
    ///   carries both poles stays unread: for a region of one face that
    ///   is no defect, as the flipped bit describes the complement,
    ///   another valid body;
    ///   and, the same answer by the band rather than by the geometry,
    ///   one with a pole off its loop but within the ambiguity band of
    ///   a great (or near-great) carrier's plane: its antipode is the
    ///   same distance from that plane, so both readings escalate and
    ///   neither pole is read, though the face may hold neither
    ///   (`props_sphere_pole_side.rs`,
    ///   `a_pole_in_the_band_of_a_great_carriers_plane_leaves_the_loop_unread`);
    /// * one whose loop has a pole off it on EACH side (a rim-only cap
    ///   or a tilted cap holding a pole). Under either sense such a
    ///   face holds a pole, which breaks the rule that a chart
    ///   singularity inside a face is a vertex of it; the reading
    ///   carries nothing about the bit, and tier 3's chart-singularity
    ///   check, not the side reader, is what refuses the face
    ///   ([`crate::chart_singularity`]).
    Unencoded,
}

/// The flux derivations' material-side sign, factored public (M6-6):
/// which side of the chart normal the face's **stored loop traversal**
/// says the material lies on — the boundary's own encoding of the
/// orientation fact `Face::sense` (M5 S10) also encodes. Tier 3's
/// curved check-6 arm compares the two encodings. Over the outer loop
/// alone; [`boundary_material_sign_loops`] takes a face's rings too,
/// and is where each kind's reading is stated.
///
/// # Errors
///
/// [`PropsError`] exactly as the flux lanes: unimplemented kinds
/// (NURBS), out-of-inventory boundary shapes (including conic-trimmed
/// faces the quadrature lane owns), escalated classifications,
/// degenerate faces. Gating callers MUST treat an error as exempt,
/// never as disagreement (the check-7 posture).
pub fn boundary_material_sign<T: Decide>(
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    band: Band,
) -> Result<MaterialSign, PropsError> {
    boundary_material_sign_loops(surface, &[outer], band)
}

/// [`boundary_material_sign`] over a face's outer loop AND its rings,
/// read per kind off the quantity that kind's flux integrates:
///
/// - **cylinder** and **torus**: the sign of the chart Green form's
///   area ([`cylinder_chart`], [`torus_chart`]), every loop included,
///   metered as a mean width (`props_chart_area_side`);
/// - **cone**: the sign of its area against the chart normal, from the
///   axial vector area its flux reads ([`cone_material_sign`],
///   `props_cone_area_side`);
/// - **sphere**: [`sphere_loop_side`] — the sign of the Gauss–Bonnet
///   area for a face with holes, the pole-free side of the loop for a
///   one-loop face.
///
/// None rests on an iso-rectangle premise: a notched or ringed face
/// encodes its side as a rectangle does.
///
/// # Errors
///
/// As [`boundary_material_sign`].
pub fn boundary_material_sign_loops<T: Decide>(
    surface: &Surface<T>,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<MaterialSign, PropsError> {
    match *surface {
        Surface::Plane { .. } => Err(PropsError::NotIsoRectangle {
            what: "boundary_material_sign called on a plane (planar orientation is the \
                   check-6 Newell winding, not a props derivation)",
        }),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => cylinder_material_sign(origin, axis, radius, loops, band),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => cone_material_sign(apex, axis, half_angle, loops, band),
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => sphere_loop_side(center, radius, axis, loops, band),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => torus_material_sign(center, axis, major_radius, minor_radius, loops, band),
        // As `curved_face_loops`: no closed form for a spline or for an
        // offset description over one.
        Surface::Nurbs(_) | Surface::Approx(_) => Err(PropsError::Unimplemented),
    }
}

/// **The iso-rectangle SHAPE door** — *is this curved face's domain an
/// iso-parameter rectangle?* — answered by the S58 single-home
/// predicate (`require_rims_at_extremes`, decided as `props_rim_level`)
/// and its sense-free companion (`unanimous_rim_side`: the rims all
/// encode one material side), on top of the same per-kind boundary
/// classification the flux lane and [`boundary_material_sign`] parse
/// with, and by nothing else: no flux, no area, and no material side
/// DERIVED for the face — the second predicate asks the rims to agree
/// with each other, which takes no `Face::sense` and answers nothing
/// about where the material is. A consumer whose own lane rests on
/// the premise — `mesh`'s swept-rectangle walk is the first — cites
/// this door itself rather than inheriting the refusal transitively
/// through a mass-properties call: no consumer keeps a transitive
/// floor, so when the certified-quadrature lane learns notched
/// domains, each door's line changes visibly instead of a floor
/// silently vanishing.
///
/// **What it decides, per kind.** The boundary parse certifies every
/// edge's CARRIER as a rim (a coaxial iso-`v` circle, incident on the
/// surface) or a meridian carrier (an axial line, a generator through
/// the apex, a great circle through the poles, a minor circle in an
/// axial plane) — a carrier that is neither refuses there, on every
/// kind (an oblique sphere section on `props_rim_axis_parallel`, a
/// torus Villarceau circle on `props_rim_fit`) — and the predicate
/// then requires
/// every rim to sit at one of the face's two extreme levels: the
/// cylinder's, cone's and sphere's from `min_max` over every level the
/// parse touched (the sphere's with each meridian arc's span-derived
/// pole extremes folded in), the torus's from its anchor meridian's
/// stored span. One home and one band: the margins are `RimArms`-
/// levered through the same named decides as the flux lane
/// (`props_circle_axis_class`, `props_rim_fit`,
/// `props_cylinder_circle_*` and `props_cone_circle_*`,
/// `props_rim_axis_parallel`, `props_rim_center_on_axis`, the
/// `props_meridian_*` decides, `props_rim_level`), so a
/// `NotIsoRectangle` from here carries the `what` the flux lane would
/// report for the same face.
///
/// **Carrier membership is not arc membership; this door decides the
/// former and [`require_one_chart_branch`] decides the latter** (issue
/// 1571). A certified meridian CARRIER can carry an arc that leaves
/// one chart meridian: a great circle contains both poles, so a sphere
/// meridian arc may cross a pole mid-edge, where the chart's `u` jumps
/// by π; a generator is a line through the apex, so a cone generator
/// segment may run through it. This parse folds the singularity into
/// the face's extent (the closed form is right about the area) and
/// says nothing about the arc's chart image — deliberately, because
/// the flux lane must keep measuring such a face. A consumer whose
/// walk assumes each edge stays on one iso curve — `mesh`'s — no
/// longer inherits that premise: it cites the branch door beside this
/// one, and gets it.
///
/// **It inherits the extent derivations, and says so.** The door
/// decides shape from rim structure against the extremes each kind
/// derives; where a derivation mis-read the extent, the answer would
/// be a false "not at an extreme", not a shape verdict. Each kind
/// derives its extremes from spans certification bounded per edge:
/// the linear kinds' are `min_max` over endpoint levels, the sphere's
/// fold each arc's pole extremes in (after re-deciding the arc's span
/// against those bounds, `props_meridian_span_forward` and
/// `props_meridian_span_winding`, inside the pole helper), and the
/// torus's is its anchor meridian's whole stored span — the pieces of a split edge folded
/// into that meridian first, under the same per-edge invariants
/// re-decided on the span the fold reconstructs
/// ([`fold_torus_meridians`]). What no derivation sees is a meridian
/// an importer states as several edges on one curve entity: those
/// carry no split lineage, stay several meridians, and the torus
/// refuses the far rim by `props_rim_level`.
///
/// **A rimless sphere band is a chart rectangle and PASSES.** A lune
/// between two meridians is `[u0, u1] × [−π/2, π/2]` whatever
/// `u1 − u0` is; the predicate is vacuous on it (no rim to place) and
/// this door says so.
///
/// **The flux lane does not ask this door's question on the sphere.**
/// [`sphere_face`] reads any loop of circles by Gauss–Bonnet, so a
/// face this door refuses can still measure, and a face it admits is
/// measured by a formula that does not rest on it. The door's sphere
/// arm serves `mesh`'s walk alone. It cannot tell the L-shaped
/// complement of a half-cap from the half-cap — one rim at an extreme
/// either way, and the door takes no sense bit — and `mesh` walks the
/// boundary it is given, which bounds the L-shaped face itself. What
/// the door DOES take is the sense-free residue of the side question —
/// every rim encodes the same side ([`unanimous_rim_side`], via
/// [`linear_rims_at_extremes`]) — which catches a multi-rim
/// contradiction without a bit.
///
/// **A zero-extent face PASSES too.** Two rims at one level joined by
/// zero-length meridians have every rim at an extreme (`lo == hi`);
/// the flux lane refuses `DegenerateFace` at its own `require_extent`
/// first. Extent is not a shape question — a degenerate rectangle is a
/// rectangle — so this door does not ask it: a consumer that cannot
/// mesh a zero-area face refuses it on its own terms (the walk's and
/// the CDT's), and one that can is not told otherwise by a shape
/// predicate. The unanimity companion is vacuous there for the same
/// reason ([`linear_rims_at_extremes`]): with no extreme to sit at, no
/// rim encodes a side for another to contradict. Pinned beside the
/// lune in `tests/r2_mesh7_door_probes`.
///
/// **A plane is not its question.** A planar face's loop is arbitrary
/// by design (rings, polygons, splines); asked anyway, this refuses
/// typed exactly as [`curved_face`] does, rather than answer "yes" for
/// a domain no chart rectangle describes.
///
/// # Errors
///
/// [`PropsError::NotIsoRectangle`] naming the failed structural
/// expectation; [`PropsError::Escalated`] when a classification lands
/// in the ambiguity band (escalate, never guess);
/// [`PropsError::Unimplemented`] for a NURBS carrier or surface.
///
/// **There is no recourse lane for a genuinely notched iso domain, and
/// a line here used to say there was.** It named the certified
/// quadrature; `topo::props`' per-face dispatch routes structurally,
/// on the carrier KIND — only an `Ellipse`/`Nurbs`-trimmed boundary or
/// a spline chart enters `quad(…)` — so an iso-bounded face refused by
/// the closed form is refused, final, with no enclosure to fall back
/// to. D2 addendum row 2 (valid input, lane not built) is still the
/// classification; what it costs is the whole answer, not a `pad > 0`.
/// The recourse a caller has is to STATE the face as iso-rectangles:
/// split the notch out with a meridian, which is what the three-face
/// sphere does where the two-face one cannot be measured at all.
pub fn require_iso_rectangle<T: Decide>(
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    band: Band,
) -> Result<(), PropsError> {
    match *surface {
        Surface::Plane { .. } => Err(PropsError::NotIsoRectangle {
            what: "require_iso_rectangle called on a plane (a planar loop is not a chart rectangle)",
        }),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let b = cylinder_boundary(origin, axis, radius, outer, band)?;
            linear_rims_at_extremes(&b, band)
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (sin_a, cos_a) = half_angle.sin_cos();
            let b = cone_boundary(apex, axis, sin_a, cos_a, outer, band)?;
            linear_rims_at_extremes(&b, band)
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => {
            let b = sphere_boundary(center, radius, axis, outer, band)?;
            linear_rims_at_extremes(&b, band)
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let p = torus_parse(center, axis, major_radius, minor_radius, outer, band)?;
            torus_rims_at_extremes(&p, center, axis, major_radius, minor_radius, band)
        }
        // As `curved_face`: no rim inventory for a spline or for an
        // offset description over one.
        Surface::Nurbs(_) | Surface::Approx(_) => Err(PropsError::Unimplemented),
    }
}

/// **The BRANCH predicate**: every boundary edge's traversed ARC lies
/// on ONE branch of the chart — its stored parameter span is monotone
/// in the chart and contains no chart singularity in its interior.
///
/// [`require_iso_rectangle`] certifies each edge's CARRIER as an iso
/// curve and the face's rim structure as a rectangle. Carrier
/// membership is not arc membership: a great circle contains both
/// poles, so a certified sphere meridian carrier can carry an arc that
/// runs over a pole, where the chart's `u` jumps by π mid-edge; a cone
/// generator is a line through the apex, so a certified generator
/// carrier can carry a segment that runs through the apex, where `u`
/// jumps to the mirror nappe. This predicate is the second question,
/// with its own home and its own name.
///
/// **Two predicates, not two answers — and the flux lane must not cite
/// this one.** The extent derivations FOLD the singularity in and are
/// right about the area of such a face: `geom-brep/tests/
/// cert1_sphere_polar.rs`'s `a_pole_crossing_meridian_arc_measures_
/// the_half_cap_exactly` and `the_rimless_hemisphere_split_off_its_
/// poles_still_measures` are faces whose meridian arcs contain a pole
/// in their interior and whose closed form is asserted EXACT. A pole
/// refusal placed in the shared parse, or in [`curved_face`], would
/// retract that. (What the parse DOES refuse, and this door with it,
/// is a span outside certification's per-edge bounds —
/// [`require_span_within_period`] — which is not an arc
/// either fold could stand on.) What such a face
/// breaks is a consumer that reads ONE chart coordinate per edge —
/// `mesh`'s boundary walk, whose `topo::chart_iso::mid_azimuth` reads
/// the carrier's midpoint through `Chart::u_of` and lands on the far
/// branch. So `mesh` cites this door and `mass_properties` does not,
/// and both are right about the same face.
///
/// **The quiet side is the walk's own inclusive pole rule.** Only a
/// definite `Positive` refuses. `Zero`, the indeterminate band and a
/// poisoned margin all ADMIT, because an arc that ENDS at a pole is
/// exactly the shape every sphere cap in the inventory has, and
/// because this door's disposition must not contradict the extent
/// fold's on the same margin ([`sphere_meridian_pole_margins`], the
/// one home): the fold takes everything but `Negative`, this takes
/// only `Positive`, and the gap between them is the arc that ends at
/// the singularity.
///
/// **The floor is [`Band::escalate`], not ε, and the distance is a
/// factor of K.** A definite sign needs `|m| ≥ escalate`, which at the
/// ratified `K = 10` is TEN coincidence widths; between `zero` and
/// `escalate` the classification is indeterminate and this door
/// admits. So the refusal begins at `10·ε` of point deviation at the
/// arc's own lever, not at `ε` — a singularity `5·ε` inside a span is
/// admitted, deliberately and by the rule above, and any statement of
/// this door's threshold that says "the band" or "ε" understates it by
/// K. The rows that pin it bracket `escalate` from both sides
/// (`0.99×` admits, `1.01×` refuses) and walk the ladder
/// `0.25·zero → zero → the indeterminate midpoint → escalate →
/// 4·escalate`.
///
/// **Per kind.**
///
/// * **cylinder — immune, and it is geometry, not a check.** The chart
///   has no singularity to cross: the axis is not on the surface, `v`
///   is the axial coordinate and a generator is a line parallel to the
///   axis (monotone in `v`, constant `u` at every parameter), and a
///   rim is a coaxial circle whose `v` is constant over the whole
///   circle however far the span runs. There is no branch to leave, so
///   this arm decides nothing rather than deciding `Ok` at a band.
/// * **cone — the apex.** A generator's stored span may contain the
///   apex; `props_cone_apex` is the signed distance from the apex's
///   line parameter to the nearer span end, which IS metres (a line's
///   `t` is arc length on a unit `dir`, the same dimensional argument
///   `props_meridian_generator` makes). Rims are coaxial circles,
///   immune as the cylinder's are.
/// * **sphere — the poles.** Decided on
///   [`sphere_meridian_pole_margins`]. Rims are not measured against
///   the poles at all: a rim's `v` is constant over its whole circle,
///   and the pole-membership arithmetic is a MERIDIAN's (it reads the
///   great circle's own parameterization), so running it on a rim
///   would refuse an equatorial full circle for nothing. The
///   rim/meridian split is `props_circle_axis_class`, the parse's own
///   name and the parse's own margin.
/// * **torus — immune on a ring torus, dormant otherwise.** Rims are
///   major circles at constant `v`; meridians are minor circles in an
///   axial plane, and on a ring torus (`major > minor`) such a circle
///   never meets the axis, so its `u` is constant over the whole
///   circle and no span can leave the branch. `major ≤ minor` — the
///   horn and spindle tori, whose minor circle DOES cross the axis —
///   is dormant rather than checked here, the same dormancy
///   `mesh`'s walk records for `Chart::poles` being empty on a torus:
///   `revolve` refuses both at construction and `topo::validate`'s
///   tier-3 `DegenerateTorus` covers the import door. This arm's
///   blind spot, stated so it reads as a decision.
/// * **plane — not its question**, refused typed exactly as
///   [`require_iso_rectangle`] refuses it.
///
/// **Layering, and the one classify each arm repeats.** This door
/// decides ARC membership on carriers the per-kind parse has
/// certified; it does not re-certify them. A caller that has not run
/// [`require_iso_rectangle`] first is asking the second question
/// without the first, and on a carrier that is no iso curve at all the
/// answer here is not meaningful. `mesh`'s
/// `curved::require_iso_rectangle_face` asks them in order.
///
/// Each arm nonetheless repeats the one parse classification it
/// cannot do without — `props_circle_axis_class` on the sphere,
/// `props_meridian_apex` on the cone — because it must know WHICH
/// edges the per-kind question is about, and a public door may not
/// rest on a caller having run another one. The sphere's span bounds
/// are not repeated by this door at all: [`sphere_meridian_pole_margins`]
/// decides them itself before it forms a margin, so the fold and this
/// door cannot disagree about them. Each repeat is a duplicate of a
/// sample already in the stream, never a new margin VALUE, so the
/// large-K lint (which lints margins against K, not counts) cannot
/// move on it.
///
/// # Errors
///
/// [`PropsError::NotOneChartBranch`] naming the offending edge and the
/// branch boundary its span crosses (valid input, unbuilt lane — D2
/// addendum row 2: the recourse is to state the side as two edges
/// meeting at the singularity, which every consumer reads);
/// [`PropsError::NotIsoRectangle`] naming `props_meridian_span_forward`
/// or `props_meridian_span_winding` for a sphere meridian span outside
/// certification's per-edge bounds `0 < Δt ≤ τ` (decided inside the
/// pole helper, so both doors answer one name), or on a plane; [`PropsError::Escalated`] when the rim/meridian
/// classification or the span bound lands in the ambiguity band
/// (escalate, never guess); [`PropsError::Unimplemented`] for a NURBS
/// surface.
pub fn require_one_chart_branch<T: Decide>(
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    band: Band,
) -> Result<(), PropsError> {
    match *surface {
        Surface::Plane { .. } => Err(PropsError::NotIsoRectangle {
            what: "require_one_chart_branch called on a plane (a planar chart has no \
                   singularity and no branch for an arc to leave)",
        }),
        Surface::Cylinder { .. } => Ok(()),
        Surface::Cone { apex, .. } => {
            for (i, e) in outer.iter().enumerate() {
                let Curve3::Line { origin, dir } = e.carrier else {
                    continue;
                };
                // GENERATORS only, by the parse's own generator margin
                // — the cone's mirror of the sphere arm's
                // rim/meridian filter. A line that misses the apex is
                // no generator of this cone, has no apex in its span
                // to cross, and must not be refused here: without this
                // the arm read EVERY `Line` carrier as a generator and
                // answered `NotOneChartBranch` on a line the shape
                // door refuses.
                if classify(
                    "props_meridian_apex",
                    Margin::norm3((apex - e.p0()).cross(dir)),
                    band,
                    PropsCheck::Exact,
                )? != Sign::Zero
                {
                    continue;
                }
                // The apex's own parameter on the generator's line
                // (`dir` unit — `props_meridian_generator` certifies
                // the direction and the parse mints it so — hence `t`
                // is metres); the margin is its signed distance to the
                // nearer span end, positive exactly when the apex is
                // interior to the span.
                let t_apex = (apex - origin).dot(dir);
                let m = (t_apex - e.t0).min(e.t1 - t_apex);
                if matches!(
                    decide("props_cone_apex", Margin::of(m), band),
                    Ok(Sign::Positive)
                ) {
                    return Err(PropsError::NotOneChartBranch {
                        edge: i,
                        what: "a cone generator whose stored span runs through the apex, the \
                               chart singularity: the azimuth this edge holds constant flips \
                               to the mirror nappe there",
                    });
                }
            }
            Ok(())
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => {
            for (i, e) in outer.iter().enumerate() {
                let Curve3::Circle {
                    axis: n_c,
                    radius: r_c,
                    ..
                } = e.carrier
                else {
                    continue;
                };
                // Meridians only (see the fn docs): the parse's own
                // rim/meridian split, on the parse's own margin.
                if classify(
                    "props_circle_axis_class",
                    Margin::levered(n_c.dot(axis), r_c),
                    band,
                    PropsCheck::Inventory,
                )? != Sign::Zero
                {
                    continue;
                }
                // The helper decides the span against certification's
                // bounds before it forms a margin, so this door — which
                // does not run the parse — refuses such a span under
                // the parse's own names.
                for (m, _) in sphere_meridian_pole_margins(e, center, radius, axis, n_c, band)? {
                    if matches!(
                        decide("props_meridian_pole", Margin::levered(m, radius), band),
                        Ok(Sign::Positive)
                    ) {
                        return Err(PropsError::NotOneChartBranch {
                            edge: i,
                            what: "a sphere meridian arc whose stored span contains a pole, \
                                   the chart singularity: the azimuth this edge holds \
                                   constant jumps by π there",
                        });
                    }
                }
            }
            Ok(())
        }
        Surface::Torus { .. } => Ok(()),
        Surface::Nurbs(_) | Surface::Approx(_) => Err(PropsError::Unimplemented),
    }
}

/// The shape door's predicate on a linearly-leveled parse: the face's
/// extremes from `min_max` over every level the boundary touches,
/// lifted into the rims' own representation, with **every rim required
/// to encode the same material side** ([`unanimous_rim_side`]).
///
/// The second half needs no sense bit: it says the rims agree with
/// EACH OTHER, which is a fact about the boundary alone, and catches a
/// face whose rims contradict one another — where reading any one rim
/// would answer a ±1 decided by which rim the owning body's loop walk
/// handed over first. A one-rim face has nothing to compare, so the
/// L-shaped complement of a half-cap passes as the half-cap does.
///
/// **Vacuous where it has nothing to decide**, in the door's own two
/// senses of that word. A rimless parse (the sphere band) has no rim
/// to place and none to compare. A ZERO-EXTENT parse has no extreme
/// for a rim to sit at, so "which extreme is this rim at" — and with
/// it "do the rims agree" — is undefined rather than violated:
/// [`rim_side`] answers that with `DegenerateFace`, and this door
/// admits it, because extent is not a shape question (a degenerate
/// rectangle is a rectangle) and the flux lanes refuse a face of no
/// area on their own terms.
fn linear_rims_at_extremes<T: Decide>(b: &LinearBoundary<T>, band: Band) -> Result<(), PropsError> {
    let (lo, hi) = min_max(&b.levels)?;
    match unanimous_rim_side(b, (lo, hi), band) {
        Ok(_) | Err(PropsError::DegenerateFace) => Ok(()),
        Err(e) => Err(e),
    }
}

/// A torus face's parse with the anchor meridian's chart orientation,
/// the prologue of [`require_iso_rectangle`]'s torus arm (the flux and
/// side readers take the Green form instead, [`torus_chart`]). The
/// anchor is the FIRST meridian in loop
/// order after [`torus_boundary`] has folded the pieces of a split
/// edge into the meridian they carry, so its span is the meridian's
/// whole span however many edges carry it.
struct TorusParse<T: Real> {
    rims: Vec<Rim<T>>,
    anchor: TorusMeridian<T>,
    orient: Sign,
}

fn torus_parse<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<TorusParse<T>, PropsError> {
    let (rims, mut meridians) = torus_boundary(center, axis, major, minor, edges, band)?;
    if meridians.is_empty() {
        return Err(PropsError::NotIsoRectangle {
            what: "torus face without a meridian",
        });
    }
    let anchor = meridians.swap_remove(0);
    let orient = torus_meridian_orient((anchor.n_c, anchor.c_c), center, axis, minor, band)?;
    Ok(TorusParse {
        rims,
        anchor,
        orient,
    })
}

/// The torus's two extreme minor angles from the anchor meridian's
/// stored span ([`torus_ends`]) with the iso-rectangle premise decided
/// against them.
fn torus_rims_at_extremes<T: Decide>(
    p: &TorusParse<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    band: Band,
) -> Result<(), PropsError> {
    let (s0, c0, s1, c1) = torus_ends(&p.anchor, center, axis, major, minor, p.orient);
    require_rims_at_extremes(
        &p.rims,
        (RimLevel::Unit(s0, c0), RimLevel::Unit(s1, c1)),
        torus_arms(major, minor),
        band,
    )?;
    Ok(())
}

// ---------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------

/// ±1 as a scalar from a **definite** sign (`Zero` maps to 0 — callers
/// only reach this with definite outcomes).
fn t_sign<T: Real>(s: Sign) -> T {
    match s {
        Sign::Positive => T::one(),
        Sign::Negative => -T::one(),
        Sign::Zero => T::zero(),
    }
}

/// The rim's discrete `u`-traversal direction: a definite circle
/// axis class, reversed when the loop traverses the edge backward.
/// Definite in ⇒ definite out (`flip` fixes only `Zero`).
fn rim_dir(s: Sign, forward: bool) -> Sign {
    if forward { s } else { s.flip() }
}

/// Funnel wrapper: classify, mapping an escalation to the typed
/// [`PropsError::Escalated`] under the `check` whose ending it carries.
///
/// `name` is the recording channel (the K stream, and
/// `docs/predicate-dimension-audit.md`'s rows); `check` is what the
/// decision was, which is what the refusal's recourse follows. Two
/// arguments and not one: the same name is one check on one surface and
/// another on another, so a table keyed by name could not say which.
///
/// `check` comes LAST, after the decide's own `(name, margin, band)`,
/// because it is about the REFUSAL and not the decision — and because
/// `tools/k-lint`'s `predicate_roster` reads a mint's margin as the
/// funnel call's second argument, deliberately by spelling.
fn classify<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
    check: PropsCheck,
) -> Result<Sign, PropsError> {
    decide(name, margin, band).map_err(|cause| PropsError::Escalated { cause, check })
}

/// **Which premise a consistency residual checks** — the split
/// [`require_zero`] takes at every raising site, because the two
/// readings of a nonzero residual are not one fact and no consumer can
/// tell them apart from the variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Premise {
    /// The edge lies on the face's own surface. No valid body violates
    /// it, so a miss is [`PropsError::OffSurface`] — a defect.
    OnSurface,
    /// The face's boundary is one the closed-form inventory folds. A
    /// valid face can violate it, so a miss is
    /// [`PropsError::NotIsoRectangle`] — a lane not built.
    Inventory,
}

impl Premise {
    /// The refusal a definitely-nonzero residual of this premise is.
    fn refusal(self, what: &'static str) -> PropsError {
        match self {
            Self::OnSurface => PropsError::OffSurface { what },
            Self::Inventory => PropsError::NotIsoRectangle { what },
        }
    }

    /// The check an escalation of this residual names.
    fn check(self) -> PropsCheck {
        match self {
            Self::OnSurface => PropsCheck::Exact,
            Self::Inventory => PropsCheck::Inventory,
        }
    }
}

/// Require a consistency residual of `premise` to be coincident with
/// zero.
fn require_zero<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
    premise: Premise,
) -> Result<(), PropsError> {
    match classify(name, margin, band, premise.check())? {
        Sign::Zero => Ok(()),
        Sign::Positive | Sign::Negative => Err(premise.refusal(name)),
    }
}

/// Require a definitely-positive extent (degenerate ⇒ typed error).
fn require_extent<T: Decide>(margin: Margin<T>, band: Band) -> Result<(), PropsError> {
    match classify("props_face_extent", margin, band, PropsCheck::Extent)? {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(PropsError::DegenerateFace),
    }
}

/// **Whether an on-surface circle is the iso rim** the parse reads a
/// level off its centre, with `w` the rim centre minus the surface's
/// axis origin:
///
/// - carrier axis **parallel** to the surface axis
///   (`props_rim_axis_parallel`): `‖n_c × â‖·r_c`, the tilt metered at
///   the rim radius;
/// - rim centre **on the axis line** (`props_rim_center_on_axis`):
///   `‖w − â(w·â)‖`.
///
/// These are the circle's deviation from the iso rim, not its distance
/// from the surface, so a miss is the inventory's: every caller has
/// decided incidence at the true distance first, and an oblique sphere
/// section, a torus Villarceau circle, or a circle tilted within band
/// of a cylinder or cone lies on the surface and is not iso.
fn require_rim_incidence<T: Decide>(
    w: Vec3<T>,
    n_c: Vec3<T>,
    r_c: T,
    axis: Vec3<T>,
    band: Band,
) -> Result<(), PropsError> {
    require_zero(
        "props_rim_axis_parallel",
        Margin::levered(n_c.cross(axis).norm(), r_c),
        band,
        Premise::Inventory,
    )?;
    require_zero(
        "props_rim_center_on_axis",
        Margin::norm3(w - axis * w.dot(axis)),
        band,
        Premise::Inventory,
    )
}

/// A rim's iso-level, carrying its own DIMENSION so every grouping
/// comparand is metered per kind rather than by one uniform
/// expression. The ratified ε semantics (D4) make every `classify`
/// comparand a LENGTH — the point deviation the difference induces —
/// and the level payload's dimension varies by surface kind, so the
/// metering choice is forced here at the constructor, not left to
/// convention at the comparison site. (The uniform `× arm` this
/// replaces turned a cone's already-length level difference into an
/// AREA — two lengths multiplied — and shrank the mm-scale
/// `cone_trunc` rim separation into the ambiguity band of the sweep
/// leg that found it (`CAD_TOLERANCE_EPS=1e-7`, not the compiled
/// default): the project's first in-band K landing, #89.)
#[derive(Clone, Copy)]
enum RimLevel<T: Real> {
    /// Cylinder/cone: the level is the axial/slant arc length `v`
    /// itself, in meters — a difference is ALREADY the point
    /// deviation and reaches `classify` bare.
    Length(T),
    /// Sphere/torus: the dimensionless direction pair
    /// `(sin v, cos v)`. Two of them are
    /// compared by the CHORD `√(Δs² + Δc²)` at the level lever arm
    /// ([`RimArms::level`], meters) — on both kinds that chord IS the
    /// point deviation, everywhere on the surface (an axial-only pair
    /// would shrink by `cos v̄` toward the sphere's poles and merge
    /// genuinely distinct rims). **That claim is the chord's**, not
    /// every sphere margin's: the side test in [`rim_side`] reads the
    /// primary component alone (`lo + hi − 2s`, at the same arm), and
    /// still shrinks by `cos v̄` near the poles, in the REFUSING
    /// direction only (audit note N8).
    Unit(T, T),
}

/// A classified rim: `u`-traversal direction (`d_u_sign`), parameter
/// span (`dt`, the face's `Δu` candidate), and its iso-level payload.
struct Rim<T: Real> {
    /// The traversal direction in `u` as a **discrete** definite sign:
    /// sign(carrier axis · surface axis) × the stored traversal bool,
    /// both discrete at origin (a definite `classify` outcome and a
    /// bool), so every consumer of it compares exact ±1s rather than
    /// scalars — [`unanimous_rim_side`]'s agreement and
    /// [`unanimous_rim_dir`]'s alike. The direction has ONE representation here: a
    /// `T` copy beside it fed a `Margin` over two values that are ±1
    /// by construction, which is a discrete comparison wearing a
    /// tolerance's costume.
    d_u_sign: Sign,
    /// Carrier parameter span `t1 − t0` (angle-true; `Δu`), decided
    /// within certification's per-edge bound ([`Rim::decided`]).
    dt: Extent<T>,
    /// The rim's iso-level, dimension carried by the variant (see
    /// [`RimLevel`] and the per-surface call sites).
    level: RimLevel<T>,
}

impl<T: Decide> Rim<T> {
    /// **The one door a rim is minted through, on every kind**: the
    /// stored span is decided ([`require_rim_span`], at the rim
    /// circle's radius `r_c` — certification's lever), so every reader
    /// of [`Rim::dt`] — the cylinder and torus charts' `Δu` sums,
    /// `du_of_rims`, [`span_sum`] — rests on that bound. A
    /// full-period rim sits AT it and is admitted.
    fn decided(
        e: &LoopEdge<T>,
        d_u_sign: Sign,
        level: RimLevel<T>,
        r_c: T,
        band: Band,
    ) -> Result<Self, PropsError> {
        let dt = require_rim_span(e, r_c, band)?;
        Ok(Self {
            d_u_sign,
            dt,
            level,
        })
    }
}

/// The two lever arms a rim decision meters at, kept apart because on
/// the torus they are different radii — `minor` and `major`, ~4× apart
/// on a real donut and 1000× apart on a gasket.
///
/// A `RimLevel::Unit` difference is a difference of DIRECTIONS; the
/// point deviation it induces is that difference at the radius the
/// direction turns about, which is the **level** arm (the sphere's `R`,
/// the torus's `minor`). A Δu angle difference is azimuthal, and the
/// point deviation it induces is at the **azimuthal** arm (the
/// sphere's `R`, the torus's `major`). The two coincide on every kind
/// but the torus, which is why one scalar was enough until it was not.
#[derive(Clone, Copy)]
struct RimArms<T> {
    /// The lever a [`RimLevel::Unit`] difference turns about.
    /// **Never consumed for a [`RimLevel::Length`] kind** — an axial or
    /// slant level difference is already the point deviation and
    /// reaches the funnel bare.
    level: T,
    /// The lever an azimuthal difference (Δu) turns about.
    azimuth: T,
}

impl<T: Real> RimArms<T> {
    /// The three surfaces whose level and azimuth turn about the same
    /// radius: the cylinder and sphere (`R`) and the cone (its first
    /// rim's own radius, [`cone_arm`]).
    fn uniform(r: T) -> Self {
        Self {
            level: r,
            azimuth: r,
        }
    }

    /// The torus: a minor-circle direction pair moves a point at
    /// `minor`, an azimuthal angle moves one at `major`.
    fn split(level: T, azimuth: T) -> Self {
        Self { level, azimuth }
    }
}

/// **Whether a rim level is one of the face's two extremes**, for
/// [`require_rims_at_extremes`], on three axes:
///
/// * **the metric.** A `Unit` pair's deviation is the CHORD
///   `√(Δs² + Δc²)` between the two directions, at `arms.level`.
///   Deciding the two components separately decides a different
///   quantity (it admits a pair up to `√2` further apart). **On both
///   `Unit` kinds that chord IS the distance a point moves**: on the
///   torus the Hausdorff distance between the two rim circles, on the
///   sphere the direction separation at `R` (~the geodesic rim
///   separation). The sphere's pair carries its `cos v` for exactly
///   this — an axial-only `(sin v, 0)` pair shrinks by `cos v̄`
///   toward the poles and merges genuinely distinct near-polar rims
///   in the ACCEPTING direction (retired audit note N7). The
///   retirement is the CHORD's: the sphere's `props_rim_side` and
///   `props_face_extent` margins still meter axial sine differences
///   and understate near the poles in the refusing direction — open
///   as audit note N8, deliberately untouched here.
/// * **the lever.** The chord is metered at [`RimArms::level`], which
///   on the torus is `minor`. Metering it at `major` overstates by
///   `major / minor` — audit note N1, now retired: the arms are
///   separate fields, so the azimuthal margins keep `major` without
///   the level margin borrowing it.
/// * **the fail direction.** A mixed representation is structurally
///   impossible (one surface builds every rim of a face AND both its
///   ends), and it REFUSES here — [`mixed_levels`].
///
/// The NEARER extreme is what is decided (`props_rim_level`), so a rim
/// sitting exactly on one extreme is not made to escalate by the other.
fn level_coincides<T: Decide>(
    level: RimLevel<T>,
    (lo, hi): (RimLevel<T>, RimLevel<T>),
    arms: RimArms<T>,
    band: Band,
) -> Result<bool, PropsError> {
    let name = "props_rim_level";
    let (Some(a), Some(b)) = (level_gap(level, lo), level_gap(level, hi)) else {
        return Err(mixed_levels(name));
    };
    let gap = a.min(b);
    let margin = match level {
        RimLevel::Length(_) => Margin::of(gap),
        RimLevel::Unit(..) => Margin::levered(gap, arms.level),
    };
    Ok(classify(name, margin, band, PropsCheck::Inventory)? == Sign::Zero)
}

/// How far apart two rim levels are, **in the level's own units**:
/// meters for [`RimLevel::Length`] (already a point deviation) and the
/// dimensionless direction CHORD for [`RimLevel::Unit`] (a point
/// deviation once levered). `None` for a mixed pair, which
/// [`level_coincides`] turns into a refusal.
fn level_gap<T: Real>(a: RimLevel<T>, b: RimLevel<T>) -> Option<T> {
    match (a, b) {
        (RimLevel::Length(la), RimLevel::Length(lb)) => Some((la - lb).abs()),
        (RimLevel::Unit(sa, ca), RimLevel::Unit(sb, cb)) => {
            // powi(2), not x*x: the interval square is tight and
            // nonnegative, so the sqrt stays fully in-domain even when
            // the difference encloses zero (an x*x interval product has
            // a negative lower bound there, and the domain-clamped
            // sqrt's decoration would poison the margin — found live on
            // the interval-lane donut).
            Some(((sa - sb).powi(2) + (ca - cb).powi(2)).sqrt())
        }
        _ => None,
    }
}

/// The refusal a mixed-representation level pair gets: typed, named
/// for the predicate that asked, and **not routed through the
/// funnel** — the pair has no comparand, so there is nothing to
/// decide and no verdict to spend.
///
/// The state is kernel-bug-only (one surface builds every rim of a
/// face AND both its ends), and this file answers that class two other
/// ways — [`torus_meridian_orient`]'s `unreachable!` and
/// [`unreachable_zero`]'s poison return. Neither is adopted here:
/// choosing between them is the file-wide census
/// `props-curved-carries-two-readings-of-d9-unreachable-vs-poison`
/// asks for, and a decision taken at one site would pre-empt it.
fn mixed_levels(name: &'static str) -> PropsError {
    PropsError::NotIsoRectangle { what: name }
}

/// **The iso-rectangle predicate**: every rim sits at one of the
/// face's two extreme `v`-levels.
///
/// This is the one named test of *"this face's domain is an
/// iso-parameter rectangle"* — the premise the closed forms in this
/// module integrate against (`super`'s module docs; `cylinder()`'s
/// `area = r·Δu·(hi − lo)`). Before S58 it existed on **one** arm,
/// inside `torus()`, for a periodicity reason rather than as a
/// decision about how the property should be tested; the other three
/// kinds tested rim-group span SUMS instead, and a sum is not a shape
/// (#649: a cross-shaped domain passed, and `topo::mass_properties`
/// certified a 19%-low volume with `volume_pad = 0.0`).
///
/// **Why the level rule gives `w ≡ Δu`.** Let `w(v)` be the total
/// `u`-measure of the domain at height `v`. Between consecutive rim
/// levels the boundary consists of meridians only — iso-`u` curves,
/// which move no `u`-endpoint — so `w` is constant there, and it can
/// change ONLY at a level carrying a rim. If every rim sits at `lo` or
/// at `hi`, then `w` is constant on the whole open interval
/// `(lo, hi)`: `w ≡ Δu`, which is what the closed forms assume ABOUT
/// `w`.
///
/// **What this does NOT establish.** The rule gives `w ≡ Δu` on
/// `(lo, hi)` and nothing more. `area = r·Δu·(hi − lo)` needs a
/// SECOND premise, which is not this one: that the `(lo, hi)` handed
/// in really is the face's `v`-extent. That premise is each kind's
/// own derivation: the torus's ends are the anchor meridian's STORED
/// span; the cylinder's and cone's are `min_max` over edge ENDPOINT
/// levels, exact because their meridians are lines — monotone in `v`,
/// with no interior extremum to miss; the sphere's meridians are
/// great-circle arcs whose latitude peaks at a pole the arc may
/// contain in its INTERIOR, so its fold also carries each arc's
/// span-derived pole extremes ([`sphere_meridian_span_levels`] — the
/// torus's stored-span move, in fold form). A face whose extent were
/// understated would still pass THIS predicate correctly, at margin
/// 0; do not read a pass here as "the closed form's preconditions are
/// checked".
///
/// **Deliberately a little stricter than necessary.** An interior
/// level carrying matching `+`/`−` groups would leave `w` unchanged
/// and is refused here anyway. Erring strict is the right direction
/// for a precondition: the refusal is D2-addendum row 2 (valid input,
/// lane not built — [`PropsError::NotIsoRectangle`]), and the
/// capability answer for such a domain is the certified-quadrature
/// lane, not a wider closed form.
///
/// `ends` are the two extreme levels in the SAME representation the
/// rims carry ([`RimLevel`]) — the torus's from its anchor meridian's
/// stored span, the linearly-leveled kinds' from `min_max`. Every
/// decision below is [`level_coincides`], including the fail
/// direction.
///
/// **The margin is not always exactly zero.** On the ordinary
/// `Length`-leveled domain it is: `min_max` folds the rim levels
/// among the rest, so each rim's own level IS one of the extremes and
/// the difference is bitwise 0. A `Unit` pair's margin carries a
/// rounding-scale second-component residual even then (the lift
/// recomputes the extreme's cosine from its sine, the rim reads its
/// own off stored data). It is a different expression whenever a
/// MERIDIAN endpoint sets `lo` or `hi` — then the margin is the rim's
/// disagreement with that endpoint, a real quantity this predicate
/// decides. That case is subsumed upstream: a rim wobbled a nanometre
/// off the vertex its meridian starts at is already refused by
/// `certify`'s `carrier_endpoint_start`, a length residual at the
/// same band.
fn require_rims_at_extremes<T: Decide>(
    rims: &[Rim<T>],
    ends: (RimLevel<T>, RimLevel<T>),
    arms: RimArms<T>,
    band: Band,
) -> Result<(), PropsError> {
    for rim in rims {
        if !level_coincides(rim.level, ends, arms, band)? {
            return Err(PropsError::NotIsoRectangle {
                what: "props_rim_level",
            });
        }
    }
    Ok(())
}

/// A linearly-leveled face's boundary, parsed once — **and the
/// metering choice that goes with it**.
///
/// The rims and levels are the parse; `arms` and `cosines` are the
/// decisions about *how a level is compared*, and they live here
/// because S81 was two sites making that decision independently and
/// drifting apart. The flux lane and [`boundary_material_sign`] each
/// need all four, and a kind's lever arms are not a thing two callers
/// may each choose: change a kind's arms here and both move.
struct LinearBoundary<T: Real> {
    /// The face's rims, in traversal order.
    rims: Vec<Rim<T>>,
    /// Every level the face's boundary touches — rim levels, meridian
    /// ENDPOINT levels and, on the sphere, each meridian arc's
    /// span-derived pole extremes — the list [`min_max`] folds.
    levels: Vec<T>,
    /// The kind's lever arms ([`RimArms`]).
    arms: RimArms<T>,
    /// On the sphere, each level's latitude cosine, beside it in
    /// `levels` and read where the level is found (a rim's `r_c/R`, a
    /// meridian end's distance from the axis over `R`, a pole's zero):
    /// a cosine completed from the sine as `√(1 − s²)` carries `√(2u)`
    /// near a pole. Empty for the cylinder and the cone, whose levels
    /// are lengths.
    cosines: Vec<T>,
}

impl<T: Decide> LinearBoundary<T> {
    /// The face's extreme levels `(lo, hi)` lifted into the rims' own
    /// [`RimLevel`] representation: `Length` for the cylinder and cone,
    /// the latitude `Unit` pair for the sphere, its cosine the one
    /// carried beside the level at that extreme (selected by sine; the
    /// interval scalar hulls the cosines of levels its sines cannot
    /// tell apart, which near a pole are one latitude's).
    fn ends(&self, (lo, hi): (T, T)) -> (RimLevel<T>, RimLevel<T>) {
        let (Some((&s0, rest)), Some(&c0)) = (self.levels.split_first(), self.cosines.first())
        else {
            return (RimLevel::Length(lo), RimLevel::Length(hi));
        };
        let at = |toward: T| {
            let (mut s, mut c) = (s0, c0);
            for (&si, &ci) in rest.iter().zip(&self.cosines[1..]) {
                // `≤ 0` when `si` is at least as far toward the extreme.
                let d = (si - s) * toward;
                (s, c) = (d.select_le_zero(si, s), d.select_le_zero(ci, c));
            }
            c
        };
        (
            RimLevel::Unit(lo, at(T::one())),
            RimLevel::Unit(hi, at(-T::one())),
        )
    }
}

/// `s_f` from `rim`'s level and the face's level range: the interior
/// lies toward the opposite extreme, read off the signed offset
/// `lo + hi − 2v` (positive when the rim sits at `lo`). Metered by the
/// level's own dimension ([`RimLevel`]) — bare for `Length` (meters
/// already), `× arms.level` for the dimensionless `Unit` primary
/// component. The offset reads ONE rim and the face's own extent, so
/// nothing about which rim a loop walk hands over first reaches it.
///
/// Returns the **discrete** sign, definite by construction: a definite
/// `props_rim_side` outcome × the rim's definite traversal direction.
///
/// **Not a door.** It carries no premise of its own; its one home is
/// [`unanimous_rim_side`], which reads every rim under
/// `props_rim_level`.
fn rim_side<T: Decide>(
    rim: &Rim<T>,
    lo: T,
    hi: T,
    arms: RimArms<T>,
    band: Band,
) -> Result<Sign, PropsError> {
    let margin = match rim.level {
        RimLevel::Length(v) => Margin::of(lo + hi - v - v),
        RimLevel::Unit(s, _) => Margin::levered(lo + hi - s - s, arms.level),
    };
    match classify("props_rim_side", margin, band, PropsCheck::Inventory)? {
        Sign::Positive => Ok(rim.d_u_sign),
        Sign::Negative => Ok(rim.d_u_sign.flip()),
        Sign::Zero => Err(PropsError::DegenerateFace),
    }
}

/// **The side EVERY rim encodes, required unanimous** — the iso
/// shape door's sense-free companion ([`linear_rims_at_extremes`]).
///
/// `s_f` is one fact about a face, so every rim's reading of it must
/// be the same fact. Reading the first rim alone answers a DEFINITE ±1
/// on a face whose rims disagree, and which one depends on where the
/// owning body's loop walk started: executed on a sphere face carrying
/// a rim at `lo` and a rim at `hi` traversed the SAME way, the lower
/// rim first read `Positive` and the upper rim first `Negative` (the
/// staircase face of `r2_sph_probes`). Requiring unanimity answers an
/// EXEMPTION there instead. It needs no sense bit: it says the rims
/// agree with EACH OTHER, not that they agree with the face's stored
/// orientation.
fn unanimous_rim_side<T: Decide>(
    b: &LinearBoundary<T>,
    (lo, hi): (T, T),
    band: Band,
) -> Result<Option<Sign>, PropsError> {
    require_rims_at_extremes(&b.rims, b.ends((lo, hi)), b.arms, band)?;
    let mut answer: Option<Sign> = None;
    for rim in &b.rims {
        let s = rim_side(rim, lo, hi, b.arms, band)?;
        match answer {
            // Equality is symmetric, so "every rim agrees with some
            // rim" is one statement about the SET and does not depend
            // on which rim the walk hands over first.
            Some(a) if a != s => {
                return Err(PropsError::NotIsoRectangle {
                    what: "props_rim_side",
                });
            }
            _ => answer = Some(s),
        }
    }
    Ok(answer)
}

// ---------------------------------------------------------------------
// Cylinder
// ---------------------------------------------------------------------

/// **A cylinder face's flux and area over ALL its loops**, in closed
/// form, by Green's theorem on the chart ([`cylinder_chart`]).
///
/// `p·n = R` on the face and the chart's area element is `R du dv`, so
/// with `A` the signed chart area the flux is `R·(R·A) + origin·A⃗` and
/// the area `R·|A|` — `R²·(−Σ∮ v du) + origin·A⃗`, each ring summed with
/// its own winding, no sense bit read (the traversal carries the side).
fn cylinder_face<T: Decide>(
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let chart = cylinder_chart(origin, axis, radius, loops, band)?;
    let mut va = Vec3::new(T::zero(), T::zero(), T::zero());
    for edges in loops {
        va = va + loop_vector_area(edges, origin)?;
    }
    let flux = radius.powi(2) * chart.area + (origin - Point3::origin()).dot(va);
    Ok(FaceContribution {
        flux,
        area: radius * chart.area.abs(),
    })
}

/// A cylinder face's signed chart area and boundary length
/// ([`cylinder_chart`]).
struct CylinderChart<T> {
    /// `A = −Σ∮ v du`, in rad·m: positive exactly when the stored
    /// traversal winds the region about the chart normal `+r̂`.
    area: T,
    /// The boundary's length in metres: `R·|Δu|` per rim, the length of
    /// each ruling.
    length: T,
}

/// **The one home of a cylinder face's chart Green form.**
///
/// `v du` is a global 1-form on the cylinder (the azimuth is periodic,
/// its differential is not), so the face's signed chart area is
/// `A = −Σ_loops ∮ v du` whatever the region's shape: a notched wall, a
/// wall with holes, a full-period band bounded by two whole rims. A
/// rim at level `v` traversed through `Δu` contributes `−v·Δu`; a
/// ruling has no `du` and contributes nothing. The stored traversal is
/// interior-left about the outward normal and the chart is right-handed
/// about the outward radial (`∂u × ∂v = R·r̂`), so `A` is positive
/// exactly when the outward normal is `+r̂`.
///
/// **Closure is checked, not assumed** ([`cylinder_windings`]): every
/// loop closes on the chart and the loops wind it as a face's boundary
/// does — each at most once, all of them zero times together. That
/// catches a face handed without a ring that WINDS the cylinder (a
/// band's second rim); a contractible ring left out leaves a closed
/// boundary of its own and cannot be seen here — the callers hand every
/// loop. A contractible ring wound the same way as its face refuses
/// (`props_ring_winding`, [`require_holes_wound_against`]). The zero
/// total winding is what makes the sum anchor-free, and
/// the sum is formed against the anchor `v_m` at the middle of the
/// face's level range — `−Σ (v − v_m)·Δu`, equal to `−Σ v·Δu` in exact
/// arithmetic — so a short face far along the axis keeps the
/// conditioning of `Δu·(v₁ − v₀)` instead of cancelling `v₀·Δu`
/// against `v₁·Δu`.
///
/// Every edge is admitted by [`cylinder_boundary`]'s decisions (an
/// axial line on the surface, a rim on it); the level extent must be
/// definite, and a face with no rim bounds no area.
fn cylinder_chart<T: Decide>(
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<CylinderChart<T>, PropsError> {
    let mut rims = Vec::new();
    let mut levels = Vec::new();
    let mut length = T::zero();
    // Per loop: its first rim's index in `rims`, and its line length.
    let mut spans: Vec<(usize, T)> = Vec::with_capacity(loops.len());
    for edges in loops {
        let first_rim = rims.len();
        let length_before = length;
        // The carriers first: an edge off the surface, or one this
        // closed form has no arm for, refuses as itself.
        let b = cylinder_boundary(origin, axis, radius, edges, band)?;
        for e in edges.iter() {
            if let Curve3::Line { .. } = e.carrier {
                length = length + (e.p1() - e.p0()).norm();
            }
        }
        for rim in &b.rims {
            let RimLevel::Length(v) = rim.level else {
                return Err(PropsError::NotIsoRectangle {
                    what: "a cylinder rim carried a non-length level",
                });
            };
            rims.push((v, t_sign::<T>(rim.d_u_sign) * rim.dt.get()));
        }
        levels.extend(b.levels);
        spans.push((first_rim, length - length_before));
    }
    let windings = cylinder_windings(origin, axis, radius, loops, &rims, &spans, band)?;
    let (lo, hi) = min_max(&levels)?;
    require_extent(Margin::of(hi - lo), band)?;
    if rims.is_empty() {
        return Err(PropsError::NotIsoRectangle {
            what: "curved face without a rim (non-sphere)",
        });
    }
    let anchor = lo + (hi - lo) * T::from_f64(0.5);
    // One loop's anchored chart area and boundary length.
    let loop_chart = |rims: &[(T, T)], lines: T| {
        rims.iter().fold((T::zero(), lines), |(a, l), &(v, du)| {
            (a - (v - anchor) * du, l + radius * du.abs())
        })
    };
    let (area, length) = loop_chart(&rims, length);
    if spans.len() > 1 {
        let holes = cylinder_holes(&rims, &spans, &windings, loop_chart, radius);
        require_holes_wound_against(
            "props_chart_area_side",
            (radius * area, length),
            &holes,
            band,
        )?;
    }
    Ok(CylinderChart { area, length })
}

/// **A cylinder face's loops close on its chart and wind it as a face's
/// boundary does** ([`closure::closed_form_windings`]): per loop, the
/// azimuth each edge turns between its own ends, and at each junction the turn about the axis
/// from the edge before's end to the edge after's start, metered at the
/// radius, beside the gap in level. Returns each loop's turns.
fn cylinder_windings<T: Decide>(
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    loops: &[&[LoopEdge<T>]],
    rims: &[(T, T)],
    spans: &[(usize, T)],
    band: Band,
) -> Result<Vec<[i64; 1]>, PropsError> {
    // The axis inside each rim carried to it, read at the rim's own
    // radius (the least of them decides for all): the surface's may be
    // up to `ε` less, and a cylinder narrower than `Kε` whose rims the
    // axis class admits holds no such premise in doubt.
    let least_rim = loops
        .iter()
        .flat_map(|l| l.iter())
        .filter_map(|e| match e.carrier {
            Curve3::Circle { radius: r, .. } => Some(r),
            _ => None,
        })
        .reduce(T::min);
    if let Some(r) = least_rim {
        closure::require_axis_inside(r, band)?;
    }
    let chart_loops: Vec<ChartLoop<T, 1>> = loops
        .iter()
        .enumerate()
        .map(|(i, edges)| -> Result<ChartLoop<T, 1>, PropsError> {
            let start = spans[i].0;
            let end = spans.get(i + 1).map_or(rims.len(), |&(next, _)| next);
            // Every edge's turn about the axis between its own ends: a
            // rim's stored turn about its own centre, carried to the axis
            // at each end; a ruling's short turn, which a ruling certified
            // axial may still make by `ε/R`.
            let mut rim_turns = rims[start..end].iter().map(|&(_, du)| du);
            let pieces = edges
                .iter()
                .map(|e| {
                    let (a, b) = e.traversal_ends();
                    let arms = turn_arms(axis, a - origin, b - origin);
                    let sweep = match e.carrier {
                        Curve3::Circle { center, .. } => {
                            let du = rim_turns.next().ok_or(PropsError::NotIsoRectangle {
                                what: "a cylinder loop with more circles than parsed rims",
                            })?;
                            du + about_axis(axis, center, origin, b)
                                - about_axis(axis, center, origin, a)
                        }
                        _ => arms.1.atan2(arms.0),
                    };
                    Ok([EdgeTurn::new(sweep, arms, radius)])
                })
                .collect::<Result<_, PropsError>>()?;
            let joints = edges
                .iter()
                .zip(edges.iter().cycle().skip(1))
                .map(|(e, next)| {
                    let a = e.traversal_ends().1 - origin;
                    let b = next.traversal_ends().0 - origin;
                    Joint {
                        turn: [(turn_about(axis, a, b), radius)],
                        flat: (b - a).dot(axis),
                    }
                })
                .collect();
            Ok(ChartLoop {
                edges: pieces,
                joints,
            })
        })
        .collect::<Result<_, _>>()?;
    closure::closed_form_windings(&chart_loops, band)
}

/// How much farther round the unit `axis` (through `origin`) the point
/// `p` sits than round a circle's own centre `center`: the turn from
/// `p − center` to `p − origin`. A stored turn about the circle's centre
/// plus its value at the end less its value at the start is the turn
/// about the axis. It is read clear of the branch cut while the axis
/// passes inside the circle, less than a quarter turn from any point of
/// it: decided for the face by [`closure::require_axis_inside`].
fn about_axis<T: Real>(axis: Vec3<T>, center: Point3<T>, origin: Point3<T>, p: Point3<T>) -> T {
    turn_about(axis, p - center, p - origin)
}

/// The signed angle about the unit `axis` from the offset `a` to the
/// offset `b`, both from a point of the axis, the short way round.
fn turn_about<T: Real>(axis: Vec3<T>, a: Vec3<T>, b: Vec3<T>) -> T {
    let (dot, cross) = turn_arms(axis, a, b);
    cross.atan2(dot)
}

/// [`turn_about`]'s cosine and sine arms: the dot and cross products about
/// the unit `axis` of the offsets `a` and `b` less their parts along it.
fn turn_arms<T: Real>(axis: Vec3<T>, a: Vec3<T>, b: Vec3<T>) -> (T, T) {
    let (a, b) = (a - axis * a.dot(axis), b - axis * b.dot(axis));
    (a.dot(b), a.cross(b).dot(axis))
}

/// A cylinder face's holes: each ring that does not wind the cylinder
/// ([`cylinder_windings`]), as its chart area in m² (`R·A`) and
/// boundary length. A ring that winds it is a band's rim, whose chart
/// area alone depends on the anchor, so it is no hole and is passed
/// over: the face's loops winding zero times together makes the face's
/// whole anchor-free. On such a band the first loop stored is a hole
/// too when it does not wind, so it is read as one.
fn cylinder_holes<T: Decide>(
    rims: &[(T, T)],
    spans: &[(usize, T)],
    windings: &[[i64; 1]],
    loop_chart: impl Fn(&[(T, T)], T) -> (T, T),
    radius: T,
) -> Vec<(T, T)> {
    let hole = |i: usize| {
        let (start, lines) = spans[i];
        let end = spans.get(i + 1).map_or(rims.len(), |&(next, _)| next);
        let (a, l) = loop_chart(&rims[start..end], lines);
        (radius * a, l)
    };
    let contractible = |i: &usize| windings[*i] == [0];
    let band_face = (1..spans.len()).any(|i| !contractible(&i));
    // On a band every loop that does not wind is a hole, the first one
    // stored included.
    (1..spans.len())
        .chain(band_face.then_some(0))
        .filter(contractible)
        .map(hole)
        .collect()
}

/// **Every hole winds against its face**, the meter the cylinder's,
/// the torus's and the cone's ringed faces share. A hole's boundary is
/// traversed opposite to the region it is cut from, so each hole's
/// signed area must have the sign opposite to the face's `whole`
/// (`props_ring_winding`), each read as its mean width `2·A/P`: an
/// `(area, length)` pair in m² and m, the length an upper bound on the
/// boundary's. A hole wound the same way would be ADDED to the face's
/// area, silently ([`PropsError::RingWoundWithFace`]); a whole or a
/// hole of no width is a degenerate face.
///
/// The cylinder quadrature and the spiric lane read their rings
/// through this same decision at the interval scalar
/// (`quad::cylinder_cut_face_rounds`,
/// `spiric::require_rings_wound_against`), where an enclosure that
/// straddles the band escalates rather than takes a side.
///
/// Which rings are holes is the caller's to say, and the kinds differ
/// on purpose: the cylinder passes over a ring that winds it
/// ([`cylinder_holes`]), whose face is guarded by its zero-winding
/// check, and so does the cone, as a band's rim ([`require_cone_holes`]);
/// the torus's every loop already closes in its lift. The whole is
/// read under `whole_name`, the predicate its kind meters a side with.
pub(super) fn require_holes_wound_against<T: Decide>(
    whole_name: &'static str,
    whole: (T, T),
    holes: &[(T, T)],
    band: Band,
) -> Result<(), PropsError> {
    require_holes_against(mean_width_side(whole_name, whole, band)?, holes, band)
}

/// [`require_holes_wound_against`] for a whole whose side is already
/// decided.
fn require_holes_against<T: Decide>(
    whole: Sign,
    holes: &[(T, T)],
    band: Band,
) -> Result<(), PropsError> {
    for &hole in holes {
        let side = mean_width_side("props_ring_winding", hole, band)?;
        if side == Sign::Zero || whole == Sign::Zero {
            return Err(PropsError::DegenerateFace);
        }
        if side == whole {
            return Err(PropsError::RingWoundWithFace);
        }
    }
    Ok(())
}

/// The sign of a region's mean width `2·A/P` under `name`: its signed
/// `area` (m²) over its boundary `length`.
fn mean_width_side<T: Decide>(
    name: &'static str,
    (area, length): (T, T),
    band: Band,
) -> Result<Sign, PropsError> {
    classify(
        name,
        Margin::over_lever(area * T::from_f64(2.0), length),
        band,
        PropsCheck::Inventory,
    )
}

/// The material side a cylinder face's loops encode: the sign of its
/// chart area ([`cylinder_chart`]), metered as the mean width `2·R·A/P`.
fn cylinder_material_sign<T: Decide>(
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<MaterialSign, PropsError> {
    let chart = cylinder_chart(origin, axis, radius, loops, band)?;
    chart_area_side(radius * chart.area, chart.length, band)
}

/// The material side a chart Green form's area encodes: its sign,
/// metered as the mean width `2·A/P` of the face's `area` (in m²)
/// over its boundary `length`. A face of no width encodes no side.
fn chart_area_side<T: Decide>(area: T, length: T, band: Band) -> Result<MaterialSign, PropsError> {
    match mean_width_side("props_chart_area_side", (area, length), band)? {
        Sign::Zero => Err(PropsError::DegenerateFace),
        side => Ok(MaterialSign::Encoded(side)),
    }
}

/// The flux and area of a curved face over its outer loop AND its
/// rings, dispatched on the surface kind: a cylinder or torus face takes
/// every loop into its chart Green form ([`cylinder_face`],
/// [`torus_face`]), a cone face into its boundary's vector area
/// ([`cone_face_closed_form`]), and a sphere face into Gauss–Bonnet
/// ([`sphere_face`]).
///
/// # Errors
///
/// As [`curved_face`].
pub fn curved_face_loops<T: Decide>(
    surface: &Surface<T>,
    loops: &[&[LoopEdge<T>]],
    sense: bool,
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    match *surface {
        Surface::Plane { .. } => Err(PropsError::NotIsoRectangle {
            what: "curved_face called on a plane (planar faces take the loop route)",
        }),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => cylinder_face(origin, axis, radius, loops, band),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => cone_face_closed_form(apex, axis, half_angle, loops, band),
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => sphere_face(center, radius, axis, loops, sense, band),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => torus_face(center, axis, major_radius, minor_radius, loops, band),
        // The volume/area inventory is closed-form per analytic kind;
        // a spline has no entry and neither does an offset description
        // over one. `Approx` refuses here rather than answer as its
        // fitted kind would.
        Surface::Nurbs(_) | Surface::Approx(_) => Err(PropsError::Unimplemented),
    }
}

/// Classify a cylinder face's boundary into (rims, iso-levels) — the
/// shared parse consumed by both the flux closed form and
/// [`boundary_material_sign`].
fn cylinder_boundary<T: Decide>(
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<LinearBoundary<T>, PropsError> {
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut levels: Vec<T> = Vec::new();
    for e in edges {
        match e.carrier {
            Curve3::Line { dir, .. } => {
                // Incidence at the true distance from the cylinder over
                // the span: the ends' radial residuals, and how far the
                // segment sags inside between them ([`radial_sag`]).
                let radial = |p: Point3<T>| {
                    let w = p - origin;
                    w - axis * w.dot(axis)
                };
                let (q0, q1) = (radial(e.p0()), radial(e.p1()));
                let (f0, f1) = (q0.norm() - radius, q1.norm() - radius);
                require_zero(
                    "props_meridian_on_surface",
                    Margin::of(f0.abs().max(f1.abs()).max(radial_sag(q0, q1) - f0.min(f1))),
                    band,
                    Premise::OnSurface,
                )?;
                // Whether the on-surface line is a ruling: its deviation
                // from the ruling through its start, which a line tipped
                // in the tangent plane leaves to first order while it
                // leaves the cylinder to second.
                require_zero(
                    "props_meridian_axial",
                    Margin::levered(dir.cross(axis).norm(), e.t1 - e.t0),
                    band,
                    Premise::Inventory,
                )?;
                levels.push((e.p0() - origin).dot(axis));
                levels.push((e.p1() - origin).dot(axis));
            }
            Curve3::Circle {
                center,
                axis: n_c,
                radius: r_c,
                ..
            } => {
                let s = classify(
                    "props_circle_axis_class",
                    Margin::levered(n_c.dot(axis), r_c),
                    band,
                    PropsCheck::Inventory,
                )?;
                if s == Sign::Zero {
                    return Err(PropsError::NotIsoRectangle {
                        what: "cylinder boundary circle is not a rim",
                    });
                }
                require_zero(
                    "props_rim_fit",
                    Margin::of(r_c - radius),
                    band,
                    Premise::OnSurface,
                )?;
                // Incidence at the true distance from the cylinder: a
                // circle tilted by `θ` is off it by `r·(1 − cos θ)`, and
                // one whose centre is `o` off the axis by `o`.
                require_zero(
                    "props_cylinder_circle_tilt",
                    Margin::levered(T::one() - n_c.dot(axis).abs(), r_c),
                    band,
                    Premise::OnSurface,
                )?;
                let w = center - origin;
                require_zero(
                    "props_cylinder_circle_off_axis",
                    Margin::norm3(w - axis * w.dot(axis)),
                    band,
                    Premise::OnSurface,
                )?;
                require_rim_incidence(w, n_c, r_c, axis, band)?;
                let v = (center - origin).dot(axis);
                // The axial arc length itself — meters.
                rims.push(Rim::decided(
                    e,
                    rim_dir(s, e.forward),
                    RimLevel::Length(v),
                    r_c,
                    band,
                )?);
                levels.push(v);
            }
            // An ellipse arc on a wall boundary (a curved cut, M5
            // PR 5) breaks the iso-rectangle patch shape THIS pass
            // requires. The PR 11 quadrature lane handles it — but it
            // needs the body's stored pcurves, so it lives one layer
            // up (`topo::mass_properties` routes conic-trimmed
            // cylinder faces there BEFORE this closed form runs); a
            // direct key-free call keeps the typed refusal.
            Curve3::Ellipse { .. } => {
                return Err(PropsError::NotIsoRectangle {
                    what: "cylinder boundary carries an ellipse arc (curved cut) — route \
                           through topo::mass_properties, whose quadrature lane consumes \
                           the stored pcurves this key-free pass cannot see",
                });
            }
            // A spiric lies on no cylinder or cone: refused beside the
            // spline, typed.
            Curve3::Nurbs(_) | Curve3::Spiric { .. } => return Err(PropsError::Unimplemented),
        }
    }
    // The cylinder's level and azimuth turn about the same radius, and
    // its levels are axial arc length — already meters, so the lift is
    // `Length` and `arms.level` is never consumed.
    Ok(LinearBoundary {
        rims,
        levels,
        arms: RimArms::uniform(radius),
        cosines: Vec::new(),
    })
}

/// Fold a nonempty level list to (min, max).
fn min_max<T: Real>(levels: &[T]) -> Result<(T, T), PropsError> {
    let Some((&first, rest)) = levels.split_first() else {
        return Err(PropsError::NotIsoRectangle {
            what: "curved face with an empty boundary",
        });
    };
    let mut lo = first;
    let mut hi = first;
    for &l in rest {
        lo = lo.min(l);
        hi = hi.max(l);
    }
    Ok((lo, hi))
}

// ---------------------------------------------------------------------
// Cone
// ---------------------------------------------------------------------

/// **A cone face's flux and area, in closed form** — no quadrature, and
/// the one home of both for every cone face on one nappe, whatever its
/// boundary's shape, with or without rings.
///
/// Every point of a cone lies on a generator through the apex, and the
/// generator lies in the tangent plane, so `(p − apex)·n = 0` on the
/// whole face. The flux is therefore `∮ p·n dA = apex·∮ n dA = apex·A⃗`,
/// with `A⃗` the boundary's vector area (Stokes; [`loop_vector_area`]),
/// summed over every loop. The normal makes the constant angle
/// `n·axis = ∓sin α` with the axis on each nappe, so the area is
/// `|axis·A⃗| / sin α`. The traversal's winding orients `A⃗`, so no sense
/// bit is read.
///
/// Both read `A⃗` and nothing else, so the form is exact for any region
/// on one nappe — a notched or stepped wall as much as a rectangle, and
/// a region whose closure holds the apex, where `n` is undefined on a
/// set of no area. What it needs, each decided by name:
///
/// - **edges on the cone** — generators through the apex, coaxial
///   sections, plane sections (the incidence premises,
///   [`require_on_cone`]), each conic's span within a period
///   ([`require_cone_conic_spans`]);
/// - **one nappe** — the boundary does not cross the apex
///   (`props_cone_nappe`, in [`cone_loops`]);
/// - **closed loops** that bound the region **once**, decided as on the
///   periodic charts ([`closure`]): every junction closes, its gap read
///   in space (in [`cone_loops`]); each loop off the apex
///   winds a whole number of turns, read off its edges' own turns about
///   the axis ([`cone_loop_winding`]), and the face's loops wind as a
///   boundary does, the apex winding back the balance of a face that
///   holds it — a cap, a band of two rims, never two turns; a loop
///   through the apex sweeps at most a turn
///   (`props_cone_apex_loop_sweep`) and its rings wind nothing; no loop
///   meets itself at a vertex ([`require_cone_loop_vertices_distinct`]);
///   and every loop that does not bound the face (a cap's rim, a
///   band's two, a loop through the apex) is a hole, wound against
///   those that do (`props_ring_winding`) and, together, inside them
///   ([`require_cone_holes`]);
/// - **a side** — the area `A⃗` encodes is not zero
///   (`props_cone_area_side`, in [`cone_loops`]), the side tier 3's
///   check 6 compares with `Face::sense`.
///
/// **Not decided: a loop that crosses itself between vertices.** Two
/// edges of one loop that cross at interior points bound lobes of
/// opposite sign, and the vector area is their difference. No closed
/// form here decides edge-edge crossing — the sphere's Gauss–Bonnet
/// shares the residual — so it is the body validity's; nor is a hole
/// decided to lie inside the loops that bound the face, beyond the sum
/// of the holes.
///
/// # Errors
///
/// [`PropsError::Unimplemented`] for a spline or spiric edge;
/// [`PropsError::OffSurface`] for an edge off the cone;
/// [`PropsError::NappeSpanning`] for a face on both nappes;
/// [`PropsError::DegenerateFace`] for a face of no area;
/// [`PropsError::ChartLoopOpen`] for a loop that does not close;
/// [`PropsError::LoopsWindChart`] for loops that wind as no boundary
/// does;
/// [`PropsError::ConeLoop`] naming the decide for a loop through the
/// apex sweeping past a turn, one that meets itself at a vertex, a ring
/// that reaches the apex, and an ellipse arc's span not forward or past
/// a period;
/// [`PropsError::RingWoundWithFace`] for a ring wound with the face;
/// [`PropsError::HolesExceedFace`] for holes that cover more than the
/// loops that bound the face; [`PropsError::Escalated`] for an in-band
/// decision.
pub fn cone_face_closed_form<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let read = cone_face_read(apex, axis, half_angle, loops, band)?;
    Ok(FaceContribution {
        flux: (apex - Point3::origin()).dot(read.va),
        area: (axis.dot(read.va) / half_angle.sin()).abs(),
    })
}

/// **A loop through the apex sweeps at most a turn**: its net azimuth
/// `total` (signed, in the loop's sense) at the arm `arm`, decided
/// definitely nonzero (`props_cone_apex_loop_net`; a loop sweeping none
/// bounds nothing and refuses) and not over a turn
/// (`props_cone_apex_loop_sweep`, the turn rule), each refusing as
/// [`PropsError::ConeLoop`].
fn apex_loop_sweep<T: Decide>(total: T, arm: T, band: Band) -> Result<(), PropsError> {
    let escalated = |cause| PropsError::Escalated {
        cause,
        check: PropsCheck::Inventory,
    };
    let net = "props_cone_apex_loop_net";
    let (_, extent) = Extent::decide_signed(net, total, InfSpeed::new(arm), band).map_err(
        |cause: geom_core::Indeterminate| match cause.margin.rejected_sign() {
            Some(Sign::Zero) => PropsError::ConeLoop { what: net },
            _ => escalated(cause),
        },
    )?;
    let sweep = "props_cone_apex_loop_sweep";
    match Period::new(T::tau()).not_over(TurnRows::exact(sweep), extent, Exact::new(arm), band) {
        Ok(true) => Ok(()),
        Ok(false) => Err(PropsError::ConeLoop { what: sweep }),
        Err(undecided) => Err(escalated(undecided.diag)),
    }
}

/// **A cone face's loops, read and decided to bound its region once** —
/// the one reading the flux arm ([`cone_face_closed_form`]) and the
/// material sign ([`cone_material_sign`]) share, so the side the gate
/// answers is one the flux lane would measure: [`cone_loops`], then the
/// loops' windings as a boundary's (a loop through the apex sweeping
/// once round, and the apex the puncture the others wind on balance),
/// every loop's vertices apart, and every loop that does not bound the
/// face a hole within those that do ([`require_cone_holes`]).
fn cone_face_read<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<ConeLoops<T>, PropsError> {
    let Some((&outer, rings)) = loops.split_first() else {
        return Err(PropsError::NotIsoRectangle {
            what: "curved face with an empty boundary",
        });
    };
    let read = cone_loops(apex, axis, half_angle, loops, band)?;
    let w = cone_loop_winding(apex, axis, outer, AtApex::AlongGenerator, band)?;
    let rings = rings
        .iter()
        .map(|ring| cone_loop_winding(apex, axis, ring, AtApex::Refuse, band))
        .collect::<Result<Vec<_>, _>>()?;
    let ring_charts = || rings.iter().map(|r| &r.chart);
    let windings = if w.through_apex {
        // A loop through the apex has no winding: the azimuth it sweeps
        // is a sector's, a slit cap's whole turn admitted.
        apex_loop_sweep(w.total, w.arm, band)?;
        let mut windings = vec![[0]];
        windings.extend(closure::closed_loop_turns(ring_charts(), band)?);
        // A ring that winds the axis beside a loop through the apex is not
        // read: such a face can bound one region (a band whose inner
        // boundary is a slit cap, a cap less a sector at the apex), but
        // these windings are no closed boundary's on the chart, and the
        // closed form refuses them rather than read them
        // (`work/conearm/cone-apex-loop-beside-a-winding-rim-refuses-a-region-it-bounds`).
        if windings.iter().any(|&k| k != [0]) {
            return Err(PropsError::LoopsWindChart);
        }
        windings
    } else {
        let windings =
            closure::closed_loop_turns(core::iter::once(&w.chart).chain(ring_charts()), band)?;
        // The apex is the chart's puncture: a face whose loops wind it on
        // balance holds it, and it winds back.
        let total: i64 = windings.iter().map(|k| k[0]).sum();
        let mut with_apex = windings.clone();
        if total != 0 {
            with_apex.push([-total]);
        }
        closure::require_boundary_windings(&with_apex)?;
        windings
    };
    require_cone_loop_vertices_distinct(apex, axis, &w, band)?;
    for r in &rings {
        require_cone_loop_vertices_distinct(apex, axis, r, band)?;
    }
    require_cone_holes(&read, &windings, band)?;
    Ok(read)
}

/// A cone face's loops, read once for its flux and its side.
struct ConeLoops<T: Real> {
    /// The vector area of every loop, summed.
    va: Vec3<T>,
    /// The side the loops encode, definite.
    side: Sign,
    /// Each loop's signed area and an upper bound on its length, the
    /// area signed as `side` reads the whole's, in the order stored.
    per_loop: Vec<(T, T)>,
}

/// **The material side a cone face's loops encode**, and the vector
/// area its flux reads ([`cone_face_closed_form`]): the sign of its
/// area against the chart normal, off the axial vector area.
///
/// Every edge must lie on the cone, decided as incidence
/// ([`PropsError::OffSurface`], [`require_on_cone`]), and a line must be
/// a generator besides, the inventory's question. Every circle and ellipse span is decided within a period
/// ([`require_cone_conic_spans`]) before any reader takes it.
///
/// The chart normal makes `n·axis = −sin α` on the `v > 0` nappe and
/// `+sin α` on the other, so a face whose traversal places it left of
/// its loops under the chart normal has `axis·A⃗ = ∓sin α·Area`. The
/// encoded side is therefore the sign of `−ν·axis·A⃗ / sin α`, with `ν`
/// the nappe's sign, metered as the mean width over an upper bound on
/// the boundary length (`props_cone_area_side`); a face of no width
/// encodes none and is degenerate.
///
/// The nappe is read off the edges' endpoint slants
/// (`props_cone_nappe`), which bound each edge for the carriers a cone
/// face carries: a generator runs between its ends, a rim holds one
/// height, and a plane section that is an ellipse lies on one nappe
/// whole. A spline or spiric edge could cross the apex between its
/// ends, so it refuses. A boundary with ends on both nappes crosses the
/// apex; one whose ends are all on it bounds nothing.
///
/// A vector area is the region's only for a closed loop, so every loop
/// must close, under the periodic charts' closure decision with its gap
/// read in space ([`PropsError::ChartLoopOpen`] otherwise).
fn cone_loops<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<ConeLoops<T>, PropsError> {
    let (sin_a, cos_a) = half_angle.sin_cos();
    let slant = |p: Point3<T>| (p - apex).dot(axis) / cos_a;
    for &edges in loops {
        for e in edges {
            require_on_cone(apex, axis, sin_a, cos_a, e, band)?;
        }
        require_cone_conic_spans(edges, band)?;
    }
    let (mut ends, mut length, mut axial, mut va) =
        (Vec::new(), T::zero(), T::zero(), Vec3::zero());
    let mut per_loop = Vec::with_capacity(loops.len());
    for &edges in loops {
        // The cone's chart is periodic only in how its azimuth is read:
        // in space a junction's gap has no period to take off, and is
        // read there, the apex with it.
        for (e, next) in edges.iter().zip(edges.iter().cycle().skip(1)) {
            closure::require_gap_closed(
                (e.traversal_ends().1 - next.traversal_ends().0).norm(),
                band,
            )?;
        }
        let mut loop_length = T::zero();
        for e in edges {
            let l = cone_edge_length(e)?;
            length = length + l;
            loop_length = loop_length + l;
            ends.push(slant(e.p0()));
            ends.push(slant(e.p1()));
        }
        let loop_va = loop_vector_area(edges, apex)?;
        let loop_axial = axis.dot(loop_va);
        axial = axial + loop_axial;
        va = va + loop_va;
        per_loop.push((loop_axial, loop_length));
    }
    let (lo, hi) = min_max(&ends)?;
    let nappe = match (
        classify("props_cone_nappe", Margin::of(lo), band, PropsCheck::Exact)?,
        classify("props_cone_nappe", Margin::of(hi), band, PropsCheck::Exact)?,
    ) {
        (Sign::Negative, Sign::Positive) => return Err(PropsError::NappeSpanning),
        (_, Sign::Positive) => T::one(),
        (Sign::Negative, _) => -T::one(),
        _ => return Err(PropsError::DegenerateFace),
    };
    let side = match mean_width_side(
        "props_cone_area_side",
        (-(nappe * axial) / sin_a, length),
        band,
    )? {
        Sign::Zero => return Err(PropsError::DegenerateFace),
        side => side,
    };
    Ok(ConeLoops {
        va,
        side,
        per_loop: per_loop
            .iter()
            .map(|&(a, l)| (-(nappe * a) / sin_a, l))
            .collect(),
    })
}

/// **An edge lies on the cone**, each incidence decided at the true
/// distance from the cone of the edge's farthest point, every one
/// through the cone's residual ([`cone_offset`]):
///
/// - a line lies on it, and is a generator as the inventory's question
///   ([`require_cone_generator`]), under the iso parse's names;
/// - a circle is coaxial (`props_circle_axis_class`) at the cone's
///   radius for its height (`props_rim_fit`, the residual at the radius
///   and the centre's axial height), and neither tilted
///   (`props_cone_circle_tilt`) nor off the axis
///   (`props_cone_circle_off_axis`) by more than the band. With the
///   other two exact, a circle tilted by `θ` is off the cone by at most
///   `r·(sin θ·sin α + (1 − cos θ)·cos α)` and one whose centre is `o`
///   off the axis by at most `o·cos α`, both attained. The iso parse
///   asks besides whether it is the iso rim ([`require_rim_incidence`]);
/// - an ellipse arc is bounded off the cone over its whole span
///   ([`require_cone_ellipse`]).
fn require_on_cone<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    sin_a: T,
    cos_a: T,
    e: &LoopEdge<T>,
    band: Band,
) -> Result<(), PropsError> {
    match e.carrier {
        Curve3::Line { dir, .. } => require_cone_generator(apex, axis, sin_a, cos_a, dir, e, band),
        Curve3::Circle {
            center,
            axis: n_c,
            radius: r_c,
            ..
        } => require_cone_circle(apex, axis, sin_a, cos_a, center, n_c, r_c, band).map(|_| ()),
        Curve3::Ellipse { .. } => require_cone_ellipse(apex, axis, sin_a, cos_a, e, band),
        Curve3::Nurbs(_) | Curve3::Spiric { .. } => Err(PropsError::Unimplemented),
    }
}

/// [`require_on_cone`]'s circle arm, shared with the iso parse
/// ([`cone_boundary`]), which reads the returned axis class as the
/// rim's sense.
#[allow(clippy::too_many_arguments)]
fn require_cone_circle<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    sin_a: T,
    cos_a: T,
    center: Point3<T>,
    n_c: Vec3<T>,
    r_c: T,
    band: Band,
) -> Result<Sign, PropsError> {
    let s = classify(
        "props_circle_axis_class",
        Margin::levered(n_c.dot(axis), r_c),
        band,
        PropsCheck::Inventory,
    )?;
    if s == Sign::Zero {
        return Err(PropsError::OffSurface {
            what: "props_circle_axis_class",
        });
    }
    let w = center - apex;
    require_zero(
        "props_rim_fit",
        Margin::of(cone_offset(r_c, w.dot(axis).abs(), sin_a, cos_a)),
        band,
        Premise::OnSurface,
    )?;
    let cos_t = n_c.dot(axis).abs();
    require_zero(
        "props_cone_circle_tilt",
        Margin::levered(
            n_c.cross(axis).norm() * sin_a + (T::one() - cos_t) * cos_a,
            r_c,
        ),
        band,
        Premise::OnSurface,
    )?;
    require_zero(
        "props_cone_circle_off_axis",
        Margin::levered(cos_a, (w - axis * w.dot(axis)).norm()),
        band,
        Premise::OnSurface,
    )?;
    Ok(s)
}

/// **An ellipse arc lies on the cone over its whole span**, bounded,
/// not sampled (`props_cone_ellipse_on_cone`).
///
/// With `w` a point's offset from the apex, `h = w·axis` and `ρ` its
/// distance from the axis, the cone's quadric residual
/// `Q = ρ²·cos²α − h²·sin²α` — formed from the radial and axial parts
/// apart, which do not cancel against each other toward a needle as
/// `cos²α·|w|² − h²` does — factors as
/// `(ρ·cos α − |h|·sin α)·(ρ·cos α + |h|·sin α)`: the elevation times a
/// second factor. So the elevation is at most `|Q|` over a lower bound
/// on that factor, metres, and that bound is taken over the
/// span in 128 equal pieces, each about its midpoint `t_m`, and decided
/// once at its largest.
///
/// On a piece, with `δ = t − t_m`, the carrier is
/// `W + A·x + B·y`, where `x = cos δ − 1`, `y = sin δ`, `W` the
/// midpoint's offset, and `A`, `B` the ellipse's axis vectors turned to
/// `t_m`. Since `x² + 2x + y² = 0`, `Q` is
/// `k₀ + k_x·x + k_y·y + k_xx·x² + k_xy·x·y`, whose five coefficients are
/// unique — all zero for an ellipse on the cone — and are read off the
/// quadratic form at the midpoint, so they carry the rounding of points
/// near the face, not of a carrier that may reach far past it; `k₀`,
/// which no small factor scales, is `Q` at the midpoint taken as its
/// two factors, the first the midpoint's own elevation. Over the
/// piece `|x| ≤ X = 1 − cos δ_max` and `|y| ≤ Y = sin δ_max`, which
/// bound `|Q|`. The piece's `|h|` is bounded below by the larger of
/// `|h_m| − |A·axis|·X − |B·axis|·Y` and the whole carrier's minimum,
/// `|h_m − A·axis| − ‖(A·axis, B·axis)‖`; its least over the pieces is
/// decided positive first (`props_cone_ellipse_nappe`), since an
/// ellipse on the cone lies on one nappe clear of the apex.
///
/// The pieces are many because the parameter is slow where a section
/// is sharp: near the vertex nearest the apex an eccentric section's
/// `A` is its major semi-axis while `|h|` is small, so a piece must be
/// short for `|A|·X` — and the rounding the coefficients carry at that
/// scale — to stay small against `|h|`.
///
/// The second factor is bounded below on a piece by
/// `ρ_low·cos α + |h|_low·sin α`, `ρ_low` the midpoint's distance from
/// the axis less what `A` and `B` can move it over the piece. The bound
/// over-reads the elevation by the spread of `ρ` and `|h|` across a
/// piece and by the triangle inequality over the five terms.
fn require_cone_ellipse<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    sin_a: T,
    cos_a: T,
    e: &LoopEdge<T>,
    band: Band,
) -> Result<(), PropsError> {
    let Curve3::Ellipse {
        center,
        axis: n,
        semi_u,
        semi_v,
        u_ref,
    } = e.carrier
    else {
        return Err(PropsError::Unimplemented);
    };
    let (a, b) = (u_ref * semi_u, n.cross(u_ref) * semi_v);
    let (cos2, sin2) = (cos_a.powi(2), sin_a.powi(2));
    let perp = |p: Vec3<T>| p - axis * p.dot(axis);
    let m = |p: Vec3<T>, q: Vec3<T>| perp(p).dot(perp(q)) * cos2 - p.dot(axis) * q.dot(axis) * sin2;
    let piece = (e.t1 - e.t0) / T::from_f64(128.0);
    let half = (piece / T::from_f64(2.0)).abs();
    let x_max = T::one() - half.min(T::pi()).cos();
    let y_max = half.min(T::pi() / T::from_f64(2.0)).sin();
    let mut middle = T::from_f64(0.5);
    let (mut h_low, mut elevation) = (T::zero(), T::zero());
    for k in 0..128 {
        let (st, ct) = (e.t0 + piece * middle).sin_cos();
        let (am, bm) = (a * ct + b * st, b * ct - a * st);
        let w = (center - apex) + am;
        let (hm, ha, hb) = (w.dot(axis), am.dot(axis), bm.dot(axis));
        let h = (hm.abs() - ha.abs() * x_max - hb.abs() * y_max)
            .max((hm - ha).abs() - (ha.powi(2) + hb.powi(2)).sqrt());
        let mbb = m(bm, bm);
        let rho = perp(w).norm();
        let q_max = (cone_offset(rho, hm.abs(), sin_a, cos_a) * (rho * cos_a + hm.abs() * sin_a))
            .abs()
            + (m(w, am) - mbb).abs() * T::from_f64(2.0) * x_max
            + m(w, bm).abs() * T::from_f64(2.0) * y_max
            + (m(am, am) - mbb).abs() * x_max.powi(2)
            + m(am, bm).abs() * T::from_f64(2.0) * x_max * y_max;
        let rho_low = (rho - perp(am).norm() * x_max - perp(bm).norm() * y_max).max(T::zero());
        let piece_elevation = q_max / (rho_low * cos_a + h * sin_a);
        (h_low, elevation) = if k == 0 {
            (h, piece_elevation)
        } else {
            (h_low.min(h), elevation.max(piece_elevation))
        };
        middle = middle + T::one();
    }
    if classify(
        "props_cone_ellipse_nappe",
        Margin::of(h_low),
        band,
        PropsCheck::Inventory,
    )? != Sign::Positive
    {
        return Err(PropsError::OffSurface {
            what: "props_cone_ellipse_nappe",
        });
    }
    require_zero(
        "props_cone_ellipse_on_cone",
        Margin::of(elevation),
        band,
        Premise::OnSurface,
    )?;
    Ok(())
}

/// **A line segment lies on the cone, and is a generator.**
///
/// Incidence first (`props_meridian_on_surface`), at an upper bound on
/// the segment's distance from the cone, which at a point is `|f|`,
/// `f = ρ·cos α − |h|·sin α` ([`cone_offset`]), with `ρ` its distance
/// from the axis and `h` its height above the apex. Along the segment
/// `h` is affine and `ρ` convex, so on each piece of it either side of
/// the apex's plane `f` is convex: greatest at the piece's ends — the
/// segment's, and where it crosses the plane, `f = ρ·cos α` — and below
/// the chord of theirs by at most `cos α` times the radial sag
/// ([`radial_sag`]). The ends and the crossing are exact.
///
/// Then whether the on-surface line is a generator, the inventory's
/// question: at the generator angle to the axis
/// (`props_meridian_generator`) and through the apex
/// (`props_meridian_apex`, the apex's distance from the line, `dir`
/// unit). A line through the apex at angle `β` to the axis is off the
/// cone by `s·sin(β − α)` at distance `s` from the apex, and
/// `sin(β − α)` is the cone's residual at the unit direction
/// ([`cone_offset`] of `|dir × axis|` and `|dir·axis|`), so the angle is
/// levered by the edge's reach from the apex, the farther of its ends.
/// A line tipped in the tangent plane misses the apex to first order and
/// leaves the cone to second.
fn require_cone_generator<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    sin_a: T,
    cos_a: T,
    dir: Vec3<T>,
    e: &LoopEdge<T>,
    band: Band,
) -> Result<(), PropsError> {
    let radial = |p: Point3<T>| {
        let w = p - apex;
        w - axis * w.dot(axis)
    };
    let (q0, q1) = (radial(e.p0()), radial(e.p1()));
    let (h0, h1) = ((e.p0() - apex).dot(axis), (e.p1() - apex).dot(axis));
    let f0 = cone_offset(q0.norm(), h0.abs(), sin_a, cos_a);
    let f1 = cone_offset(q1.norm(), h1.abs(), sin_a, cos_a);
    // Over a piece on one side of the apex's plane: the larger end, and
    // the depth inside.
    let piece = |qa: Vec3<T>, fa: T, qb: Vec3<T>, fb: T| {
        fa.max(fb).max(radial_sag(qa, qb) * cos_a - fa.min(fb))
    };
    let whole = piece(q0, f0, q1, f1);
    // Split where the segment crosses the plane, at radial `qc`.
    let qc = (q0 * h1 - q1 * h0) * (T::one() / (h1 - h0));
    let fc = qc.norm() * cos_a;
    let split = piece(q0, f0, qc, fc).max(piece(qc, fc, q1, f1));
    let span = h0.abs() + h1.abs();
    let worst = (h0 * h1).select_le_zero(span.select_le_zero(whole, split), whole);
    require_zero(
        "props_meridian_on_surface",
        Margin::of(worst),
        band,
        Premise::OnSurface,
    )?;
    require_zero(
        "props_meridian_generator",
        Margin::levered(
            cone_offset(dir.cross(axis).norm(), dir.dot(axis).abs(), sin_a, cos_a),
            (e.p0() - apex).norm().max((e.p1() - apex).norm()),
        ),
        band,
        Premise::Inventory,
    )?;
    require_zero(
        "props_meridian_apex",
        Margin::norm3((apex - e.p0()).cross(dir)),
        band,
        Premise::Inventory,
    )
}

/// **How far a segment sags inside the chord of its ends' radii**, with
/// `q0`, `q1` the ends' radial vectors from an axis and `ρ = ‖q‖`: its
/// radius at `λ ∈ [0, 1]` is at least `(1 − λ)·ρ0 + λ·ρ1` less this.
///
/// That chord `c` meets `c² − ρ(λ)² = λ(1 − λ)·N`, with
/// `N = (ℓ − δ)(ℓ + δ)`, `ℓ = ‖q1 − q0‖`, `δ = |ρ1 − ρ0|`, so the sag is
/// `λ(1 − λ)·N / (c + ρ(λ)) ≤ λ(1 − λ)·N / c ≤ N / (√ρ0 + √ρ1)²`, the
/// last the greatest over `λ` of the one before, and so within a factor
/// 2 of the greatest sag. Read as the least of that, `2·N / (ℓ + δ)`
/// and `2·(ℓ − δ)` (equal in reals; `(ℓ + δ) / (√ρ0 + √ρ1)² ≤ 2`, as
/// `ℓ + δ ≤ 2·max ρ`): the last has no denominator to vanish on a piece
/// of a generator that ends at the apex, whose ends both read on the
/// axis.
///
/// `N` is formed without the cancellation `ℓ − δ` carries:
/// `N = ℓ² − δ² = 2·(ρ0·ρ1 − q0·q1)`, and where `q0·q1 ≥ 0` that is
/// also `2·‖q0 × q1‖² / (ρ0·ρ1 + q0·q1)` (Lagrange's identity), the
/// lesser of the two read. A generator's
/// ends are radially parallel, so its `N` is rounding squared; read as
/// the difference of two norms, the interval scalar loses their
/// dependence and encloses a sag of the norms' own width (`≈ 1e-12` on
/// a wall 410 out, past `ε = 1e-12`).
fn radial_sag<T: Real>(q0: Vec3<T>, q1: Vec3<T>) -> T {
    let (r0, r1) = (q0.norm(), q1.norm());
    let (l, d) = ((q1 - q0).norm(), (r1 - r0).abs());
    let two = T::one() + T::one();
    let (rr, dot) = (r0 * r1, q0.dot(q1));
    let tiny = T::from_f64(f64::MIN_POSITIVE);
    // Both forms enclose `N`, so the lesser does: the second is tight
    // where the first cancels, and the first holds the second where an
    // end within rounding of the axis leaves its denominator, floored,
    // near nothing.
    let direct = two * (rr - dot);
    let n = dot.select_le_zero(
        direct,
        direct.min(two * q0.cross(q1).norm_squared() / (rr + dot).max(tiny)),
    );
    let roots = (r0.sqrt() + r1.sqrt()).powi(2).max(tiny);
    // Three bounds on the one sag, the least taken: `2·(ℓ − δ)`, which
    // has no denominator to vanish where both ends are on the axis but
    // carries the cancellation, and the two quotients of `N`.
    (n / roots)
        .min(two * n / (l + d).max(tiny))
        .min(two * (l - d))
}

/// **Every conic span the cone reads, decided.** The cone's flux lane
/// and material sign integrate each edge's stored span through
/// [`loop_vector_area`] and [`cone_edge_length`], and its winding reads
/// it through [`cone_loop_winding`], on every loop, rings included,
/// and none of them mints a [`Rim`]. A circle
/// on a cone is a coaxial section, so its span is a rim's and is decided
/// under the rim's names at its own radius ([`require_rim_span`]). An
/// ellipse arc is no rim: its span is decided under its own names,
/// `props_cone_ellipse_span_forward` / `props_cone_ellipse_span_winding`,
/// at certification's ellipse rates (the speed floor forward, and the inf
/// and sup over the turn's defect for the turn), refusing as
/// [`PropsError::ConeLoop`].
fn require_cone_conic_spans<T: Decide>(
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<(), PropsError> {
    for e in edges {
        match e.carrier {
            Curve3::Circle { radius, .. } => {
                require_rim_span(e, radius, band)?;
            }
            ref ellipse @ Curve3::Ellipse { .. } => {
                let Some(floor) = geom::carrier_speed::speed_floor(ellipse) else {
                    unreachable!("`speed_floor` answers an ellipse")
                };
                let rows = TurnRows::bounded(
                    "props_cone_ellipse_span_winding",
                    "props_cone_ellipse_span_winding_straddle",
                );
                require_span_within_period(
                    (e.t0, e.t1),
                    floor,
                    |extent| {
                        let rate = geom::carrier_speed::turn_defect_rate(ellipse, e.t0, extent);
                        (rate, rows)
                    },
                    "props_cone_ellipse_span_forward",
                    band,
                    |what| PropsError::ConeLoop { what },
                )?;
            }
            _ => {}
        }
    }
    Ok(())
}

/// An upper bound on a cone boundary edge's length: a conic's speed is
/// at most its major semi-axis.
fn cone_edge_length<T: Decide>(e: &LoopEdge<T>) -> Result<T, PropsError> {
    match e.carrier {
        Curve3::Line { .. } => Ok((e.p1() - e.p0()).norm()),
        Curve3::Circle { radius, .. } => Ok(radius * (e.t1 - e.t0).abs()),
        Curve3::Ellipse { semi_u, semi_v, .. } => {
            Ok(geom::Curve3::semi_axes_by_size(semi_u, semi_v).0 * (e.t1 - e.t0).abs())
        }
        Curve3::Nurbs(_) | Curve3::Spiric { .. } => Err(PropsError::Unimplemented),
    }
}

/// The material side a cone face's loops encode, read by the flux
/// lane's own decides ([`cone_face_read`]), so the side it answers is
/// one the flux lane measures. No iso premise: a notched or trimmed face encodes its side as a
/// rectangle does.
fn cone_material_sign<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<MaterialSign, PropsError> {
    Ok(MaterialSign::Encoded(
        cone_face_read(apex, axis, half_angle, loops, band)?.side,
    ))
}

/// **A ringed cone face's holes lie inside it.** The face lies inside
/// the loops that bound it — a band's two rims, which wind the axis
/// opposite ways; the one loop that winds the axis about the apex the
/// face holds; or else the first loop stored — and every other loop is
/// a hole. Each hole is wound against those loops together
/// (`props_ring_winding` against their side, `props_cone_outer_side`,
/// read off their signed areas summed), and the face's side
/// (`props_cone_area_side`) is theirs: holes that cover more than they
/// do — overlapping one another or leaving the face — turn the whole's
/// sign, and that is [`PropsError::HolesExceedFace`], not a ring wound
/// with the face.
///
/// Tier 3 reads a cone face's side off its outer loop alone
/// ([`boundary_material_sign_loops`]), so this is the one check of a
/// cone ring's orientation a body at rest gets.
fn require_cone_holes<T: Decide>(
    read: &ConeLoops<T>,
    windings: &[[i64; 1]],
    band: Band,
) -> Result<(), PropsError> {
    let wound: Vec<usize> = (0..windings.len())
        .filter(|&i| windings[i] != [0])
        .collect();
    let bounds = if wound.is_empty() { vec![0] } else { wound };
    let holes: Vec<(T, T)> = (0..windings.len())
        .filter(|i| !bounds.contains(i))
        .map(|i| read.per_loop[i])
        .collect();
    if holes.is_empty() {
        return Ok(());
    }
    let outer = bounds.iter().fold((T::zero(), T::zero()), |(a, l), &i| {
        let (ai, li) = read.per_loop[i];
        (a + ai, l + li)
    });
    let outer = match mean_width_side("props_cone_outer_side", outer, band)? {
        Sign::Zero => return Err(PropsError::DegenerateFace),
        side => side,
    };
    require_holes_against(outer, &holes, band)?;
    if read.side == outer {
        Ok(())
    } else {
        Err(PropsError::HolesExceedFace)
    }
}

/// **A cone loop meets itself at no vertex**
/// (`props_cone_loop_vertex`). A loop that passes a vertex twice — a
/// pinched figure-eight, whose vector area is the difference of its
/// lobes, or a loop stated twice, whose is double — bounds no region
/// once. Every two non-adjacent vertices (each edge's traversal start)
/// are decided apart in space; two that coincide are the same vertex
/// unless the loop has gone round the axis between them, which is a
/// seam's two copies of one vertex, a period apart on the chart
/// (`props_cone_loop_vertex_sheet`: the azimuth the loop turns between
/// them, a whole number of turns, against half a turn, levered by the
/// vertex's radius). A vertex on the apex has no azimuth, and a loop
/// that passes the apex twice is pinched there.
fn require_cone_loop_vertices_distinct<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    w: &ConeWinding<T>,
    band: Band,
) -> Result<(), PropsError> {
    let refuse = || PropsError::ConeLoop {
        what: "props_cone_loop_vertex",
    };
    let n = w.vertices.len();
    for i in 0..n {
        for j in i + 2..n {
            if i == 0 && j == n - 1 {
                continue;
            }
            let ((p, a), (q, b)) = (w.vertices[i], w.vertices[j]);
            if classify(
                "props_cone_loop_vertex",
                Margin::of((p - q).norm()),
                band,
                PropsCheck::Inventory,
            )? == Sign::Positive
            {
                continue;
            }
            let (Some(a), Some(b)) = (a, b) else {
                return Err(refuse());
            };
            let r = p - apex;
            let rho = (r - axis * r.dot(axis)).norm();
            if classify(
                "props_cone_loop_vertex_sheet",
                Margin::levered((a - b).abs() - T::pi(), rho),
                band,
                PropsCheck::Inventory,
            )? != Sign::Positive
            {
                return Err(refuse());
            }
        }
    }
    Ok(())
}

/// **A cone loop's junctions on the cone's chart**, for its winding: the
/// turn about the axis from the edge before's end to the edge after's
/// start, metered at the after's distance from the axis. Read only for a
/// loop whose ends are all decided off the axis; its junctions' gaps are
/// decided in space ([`cone_loops`]).
fn cone_joints<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    edges: &[LoopEdge<T>],
) -> Vec<Joint<T, 1>> {
    let radial = |p: Point3<T>| {
        let w = p - apex;
        w - axis * w.dot(axis)
    };
    edges
        .iter()
        .zip(edges.iter().cycle().skip(1))
        .map(|(e, next)| {
            let (a, b) = (
                radial(e.traversal_ends().1),
                radial(next.traversal_ends().0),
            );
            Joint {
                turn: [(turn_about(axis, a, b), b.norm())],
                flat: T::zero(),
            }
        })
        .collect()
}

/// What [`cone_loop_winding`] does with an edge end on the apex, where
/// the azimuth is undefined.
#[derive(Clone, Copy, PartialEq, Eq)]
enum AtApex {
    /// Refuse (`props_cone_ring_off_apex`): a ring is read off the apex.
    Refuse,
    /// Read a generator's azimuth at its apex end off its other end
    /// (`props_cone_loop_off_apex`): an outer loop may reach the apex.
    AlongGenerator,
}

/// A cone loop's turn about the axis ([`cone_loop_winding`]).
struct ConeWinding<T: Real> {
    /// The azimuth the loop turns through, summed.
    total: T,
    /// Its widest radius.
    arm: T,
    /// Whether an edge end lies on the apex.
    through_apex: bool,
    /// Each edge's traversal start, with the azimuth the loop has
    /// turned from its first vertex to it — `None` on the apex.
    vertices: Vec<(Point3<T>, Option<T>)>,
    /// The loop on the cone's chart, for the closure decision
    /// ([`closure::closed_loop_turns`]): each edge's turn about the axis
    /// and the angle by which it misses its end, metered at its nearest
    /// radius — none for a generator's short turn or an ellipse's
    /// thirds, which agree with their ends by construction, and a rim's
    /// read about its own centre ([`closure::EdgeTurn::carried`]); each
    /// junction's turn, metered at the after's radius. Its
    /// junctions' gaps are decided in space ([`cone_loops`]), so they
    /// carry none. Empty for a loop through the apex, which has no
    /// winding.
    chart: ChartLoop<T, 1>,
}

/// **The azimuth a cone loop turns through about the axis**: the sum,
/// piece by piece over the cycle of its edges' ends in traversal order,
/// of each edge's turn and each joint's:
///
/// - a segment on the cone is a generator, turning through nothing;
/// - a joint closes within the band ([`cone_loops`]) between ends
///   off the axis, so it turns through less than a quarter turn — the
///   principal angle between its ends' radial directions; a joint at
///   the apex, where the azimuth is undefined, is skipped;
/// - a circle on a cone is a rim, whose parameter IS the azimuth about
///   its own centre, in the sense its axis makes with the cone's
///   (`props_cone_arc_sense`): it turns through its stored span, a whole
///   turn or more included, carried to the axis at each end
///   ([`about_axis`]: its centre is within the band of the axis and its
///   ends are decided off it, so the axis passes inside it);
/// - an ellipse on a cone is a plane section that meets every generator
///   of its nappe once, so its azimuth is monotone along it, in that
///   sense too — but its parameter is not the azimuth. It is read in
///   thirds of its parameter span, so a whole turn closed by one edge is
///   no principal angle of zero; each third turns through the principal
///   angle between its ends, taken the long way round when that angle
///   runs against its sense (`props_cone_arc_turn`). The long way is
///   reachable: a third of a tilted section's parameter near the vertex
///   the axis pierces closest to subtends more than half a turn.
///
/// Every step is congruent, mod a whole turn, to the azimuth change
/// between its ends, and the ends close up, so the sum over a loop off
/// the apex is a whole number of turns up to the rounding of a few
/// angles, far below half a turn: its winding, which the closure
/// decision reads ([`closure::closed_loop_turns`]) off the loop's chart
/// ([`ConeWinding::chart`]).
///
/// A loop through the apex has no winding — the apex joint is skipped —
/// and sums the azimuth its edges sweep: a sector's angle, a whole turn
/// for the slit cap that goes round to its own generator, and more only
/// for a loop that covers part of the nappe twice. Its caller decides
/// that sweep against a whole turn, admitting the turn itself
/// (`props_cone_apex_loop_sweep`).
fn cone_loop_winding<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    edges: &[LoopEdge<T>],
    at_apex: AtApex,
    band: Band,
) -> Result<ConeWinding<T>, PropsError> {
    let refuse = |what| Err(PropsError::ConeLoop { what });
    let radial = |p: Point3<T>| {
        let w = p - apex;
        w - axis * w.dot(axis)
    };
    let off = |name, rho: T| {
        classify(name, Margin::of(rho), band, PropsCheck::Inventory).map(|s| s == Sign::Positive)
    };
    let mut arm = T::zero();
    let mut through_apex = false;
    // Each edge's step, and its radial directions (and whether on the
    // apex) at its traversal start and end.
    let mut steps = Vec::with_capacity(edges.len());
    let mut pieces = Vec::with_capacity(edges.len());
    for e in edges {
        let (mut a, mut b) = (radial(e.p0()), radial(e.p1()));
        let (mut a_apex, mut b_apex) = (false, false);
        let rho = a.norm().min(b.norm());
        match at_apex {
            AtApex::Refuse => {
                if !off("props_cone_ring_off_apex", rho)? {
                    return refuse("props_cone_ring_off_apex");
                }
            }
            AtApex::AlongGenerator => {
                let line = matches!(e.carrier, Curve3::Line { .. });
                match (
                    off("props_cone_loop_off_apex", a.norm())?,
                    off("props_cone_loop_off_apex", b.norm())?,
                ) {
                    (true, true) => {}
                    (false, true) if line => (a, a_apex) = (b, true),
                    (true, false) if line => (b, b_apex) = (a, true),
                    _ => return refuse("props_cone_loop_off_apex"),
                }
            }
        }
        through_apex = through_apex || a_apex || b_apex;
        arm = arm.max(a.norm()).max(b.norm());
        let sense = |n: Vec3<T>, rho: T| match classify(
            "props_cone_arc_sense",
            Margin::levered(n.dot(axis), rho),
            band,
            PropsCheck::Inventory,
        )? {
            Sign::Positive => Ok(T::one()),
            Sign::Negative => Ok(-T::one()),
            Sign::Zero => Err(PropsError::ConeLoop {
                what: "props_cone_arc_sense",
            }),
        };
        let mut lever = rho;
        let mut own_turn = T::zero();
        let step = match e.carrier {
            // The stored turn about the rim's own centre, carried to the
            // axis at each end.
            Curve3::Circle {
                axis: n, center, ..
            } => {
                own_turn = sense(n, rho)? * (e.t1 - e.t0);
                own_turn + about_axis(axis, center, apex, e.p1())
                    - about_axis(axis, center, apex, e.p0())
            }
            Curve3::Ellipse { axis: n, .. } => {
                let third = (e.t1 - e.t0) / T::from_f64(3.0);
                let m1 = radial(e.carrier.eval(e.t0 + third));
                let m2 = radial(e.carrier.eval(e.t1 - third));
                let rho = rho.min(m1.norm()).min(m2.norm());
                lever = rho;
                let sense = sense(n, rho)?;
                let mut thirds = T::zero();
                for (x, y) in [(a, m1), (m1, m2), (m2, b)] {
                    let principal = turn_about(axis, x, y);
                    thirds = thirds
                        + match classify(
                            "props_cone_arc_turn",
                            Margin::levered(principal * sense, rho),
                            band,
                            PropsCheck::Inventory,
                        )? {
                            Sign::Positive => principal,
                            Sign::Negative => principal + sense * T::tau(),
                            Sign::Zero => return refuse("props_cone_arc_turn"),
                        };
                }
                thirds
            }
            _ => turn_about(axis, a, b),
        };
        let ends = ((a, a_apex), (b, b_apex));
        let (step, ends) = if e.forward {
            (step, ends)
        } else {
            (-step, (ends.1, ends.0))
        };
        // A generator's short turn and an ellipse's thirds agree with
        // their ends by construction; a rim's carried turn misses them by
        // what its own turn misses them about its centre.
        pieces.push([match e.carrier {
            Curve3::Circle { center, .. } => {
                let (ca, cb) = e.traversal_ends();
                let own = if e.forward { own_turn } else { -own_turn };
                EdgeTurn::carried(step, own, turn_arms(axis, ca - center, cb - center), lever)
            }
            _ => EdgeTurn::exact(step, lever),
        }]);
        steps.push((step, ends));
    }
    // A loop through the apex has no winding, and no chart to read.
    let joints = if through_apex {
        Vec::new()
    } else {
        cone_joints(apex, axis, edges)
    };
    let mut total = T::zero();
    let mut vertices = Vec::with_capacity(edges.len());
    for (k, e) in edges.iter().enumerate() {
        let (step, ((_, start_apex), (end, end_apex))) = steps[k];
        vertices.push((
            e.traversal_ends().0,
            if start_apex { None } else { Some(total) },
        ));
        total = total + step;
        let ((start, next_apex), _) = steps[(k + 1) % steps.len()].1;
        if !end_apex && !next_apex {
            total = total + turn_about(axis, end, start);
        }
    }
    Ok(ConeWinding {
        total,
        arm,
        through_apex,
        vertices,
        chart: ChartLoop {
            edges: pieces,
            joints,
        },
    })
}

/// Classify a cone face's boundary into (rims, signed slant levels) —
/// the iso parse [`require_iso_rectangle`] reads — **with the apex
/// folded in where the boundary needs it** ([`cone_apex_level`]): an
/// apex cap is the rectangle `[0, v] × [0, 2π)`.
///
/// **A folded extent carries its guard with it**, so there is no flag
/// to return and no caller that can forget one: the fold refuses a rim
/// that does not close around the apex ([`require_rim_only_closed`])
/// rather than folding and leaving the check to whoever asked.
fn cone_boundary<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    sin_a: T,
    cos_a: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<LinearBoundary<T>, PropsError> {
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut levels: Vec<T> = Vec::new();
    let mut generators = false;
    for e in edges {
        match e.carrier {
            Curve3::Line { dir, .. } => {
                generators = true;
                require_cone_generator(apex, axis, sin_a, cos_a, dir, e, band)?;
                levels.push((e.p0() - apex).dot(axis) / cos_a);
                levels.push((e.p1() - apex).dot(axis) / cos_a);
            }
            Curve3::Circle {
                center,
                axis: n_c,
                radius: r_c,
                ..
            } => {
                let s = require_cone_circle(apex, axis, sin_a, cos_a, center, n_c, r_c, band)?;
                require_rim_incidence(center - apex, n_c, r_c, axis, band)?;
                let v = (center - apex).dot(axis) / cos_a;
                // The signed slant arc length itself — meters.
                rims.push(Rim::decided(
                    e,
                    rim_dir(s, e.forward),
                    RimLevel::Length(v),
                    r_c,
                    band,
                )?);
                levels.push(v);
            }
            Curve3::Ellipse { .. } => {
                return Err(PropsError::NotIsoRectangle {
                    what: "cone boundary carries an ellipse arc (a tilted-section cut) — \
                           not a rim or a generator, so this iso parse has no reading of \
                           it (`cone_face_closed_form` measures such a face by its \
                           vector area)",
                });
            }
            // A spiric lies on no cylinder or cone: refused beside the
            // spline, typed.
            Curve3::Nurbs(_) | Curve3::Spiric { .. } => return Err(PropsError::Unimplemented),
        }
    }
    // Cone levels are the signed SLANT arc length — `Length`, bare —
    // and the arm is the first rim's own radius ([`cone_arm`]).
    let arms = RimArms::uniform(cone_arm(&rims, sin_a));
    let mut b = LinearBoundary {
        rims,
        levels,
        arms,
        cosines: Vec::new(),
    };
    if !generators && !b.rims.is_empty() {
        cone_apex_level(&mut b, edges, band)?;
    }
    Ok(b)
}

/// **A generator-free cone boundary's missing extreme is the APEX** —
/// level `0`, pushed into the levels so the face has the extent its
/// rims alone cannot state.
///
/// A cone face bounded by one rim circle and nothing else: the levels
/// hold one slant, `min_max` answers `lo == hi` and [`require_extent`]
/// would refuse `DegenerateFace` for a face that plainly has an area.
/// A cone is bounded on the apex side ONLY, so there is a single
/// candidate for the missing extreme and no bit to read. (A cylinder
/// is unbounded both ways along its axis and has no candidate at all,
/// which is why it has no fold.)
///
/// **Unanimous traversal direction is required** ([`unanimous_rim_dir`]).
/// What it excludes is the true zero-extent patch — rims at one level
/// traversed opposite ways, which point at opposite sides and contain
/// no apex — and that face keeps its `DegenerateFace`.
///
/// The extent question is [`require_extent`]'s own comparand asked one
/// step earlier, under `props_rim_only_extent`: the bare slant
/// difference, as `require_extent`'s cone call reads it.
fn cone_apex_level<T: Decide>(
    b: &mut LinearBoundary<T>,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<(), PropsError> {
    let (lo, hi) = min_max(&b.levels)?;
    if classify(
        "props_rim_only_extent",
        Margin::of(hi - lo),
        band,
        PropsCheck::Extent,
    )? != Sign::Zero
    {
        return Ok(());
    }
    if unanimous_rim_dir(&b.rims).is_none() {
        return Ok(());
    }
    let Some(du) = span_sum(&b.rims) else {
        unreachable!("a unanimous direction is read off at least one rim")
    };
    require_rim_only_closed(edges, du, b.arms.azimuth, band)?;
    b.levels.push(T::zero());
    Ok(())
}

/// **The apex is interior to a rim only if the rim CLOSES around it**
/// (`props_rim_only_closed`), decided on the `Δu` the rim spans sum to
/// — [`cone_apex_level`]'s fold reads a traversal DIRECTION, which
/// says nothing about how far the rim goes. Without it a lone half rim
/// answered half the cap's area, and the
/// same full rim stated twice double.
///
/// **A SUM is not a COVER.** The question is the turn rule's whole turn
/// ([`geom_core::period`]), the headroom `(τ − Δu)` at the rim radius
/// ([`RimArms::azimuth`]): the arc the spans fail to total a turn by.
/// It admits any multiset of same-direction arcs whose spans
/// total `τ` — the same HALF rim stated twice covers half the circle —
/// so the arcs must also TILE: `props_rim_only_join` decides, for each
/// edge in the loop's own order, that its traversal END is the next
/// one's traversal START, a point deviation in metres. Every arc runs
/// the same way and its span is non-negative, so arcs that chain and
/// total a turn cover the circle exactly once; three arcs of `2τ/3`
/// chain and double-cover (the sum refuses them), and a half rim
/// stated twice totals a turn and covers half (the joins refuse it).
/// The joins are asked cyclically, so the verdict does not move under
/// a re-anchoring of the loop.
///
/// The cone's other traversal of the rim bounds the rest of the nappe,
/// which runs to infinity and is no finite face; what catches that face
/// is tier 3's check 6, against the side [`cone_material_sign`] reads.
///
/// **Its precondition is the fold's own**: every edge of `edges` is a
/// rim of the one group, in loop order.
fn require_rim_only_closed<T: Decide>(
    edges: &[LoopEdge<T>],
    du: Extent<T>,
    arm: T,
    band: Band,
) -> Result<(), PropsError> {
    let name = "props_rim_only_closed";
    match Period::new(T::tau()).whole_turn(TurnRows::exact(name), du, Exact::new(arm), band) {
        Ok(true) => {}
        Ok(false) => return Err(Premise::Inventory.refusal(name)),
        Err(undecided) => {
            return Err(PropsError::Escalated {
                cause: undecided.diag,
                check: Premise::Inventory.check(),
            });
        }
    }
    // The joins, in the loop's own order: edge `i`'s traversal END is
    // edge `i + 1`'s traversal START, cyclically. A point deviation in
    // metres.
    for (i, e) in edges.iter().enumerate() {
        let next = &edges[(i + 1) % edges.len()];
        require_zero(
            "props_rim_only_join",
            Margin::norm3(e.traversal_ends().1 - next.traversal_ends().0),
            band,
            Premise::Inventory,
        )?;
    }
    Ok(())
}

/// The traversal direction EVERY rim of a boundary shares, or `None`
/// where the set is empty or divided.
fn unanimous_rim_dir<T: Real>(rims: &[Rim<T>]) -> Option<Sign> {
    let (first, rest) = rims.split_first()?;
    rest.iter()
        .all(|r| r.d_u_sign == first.d_u_sign)
        .then_some(first.d_u_sign)
}

/// The `Δu` a one-group rim set spans — the plain sum of the stored
/// spans, asked only where a fold has already established one group
/// (one level, one direction); `None` for no rims. It decides nothing
/// and records nothing; the deciding is [`require_rim_only_closed`]'s.
fn span_sum<T: Real>(rims: &[Rim<T>]) -> Option<Extent<T>> {
    rims.iter().map(|r| r.dt).reduce(|acc, dt| acc + dt)
}

/// The cone's azimuthal lever arm: the first rim's own radius
/// `|v|·sin α` (cone rims are `Length`-leveled, so the arm meters only
/// the `Δu` margin of [`require_rim_only_closed`]).
///
/// **The `T::one()` fallback is never metered against.** It is
/// reachable — [`cone_boundary`] computes the arm before it knows
/// whether there is a rim — but the fold that reads it runs only on a
/// non-empty rim list, and [`unanimous_rim_side`] reads no arm on an
/// empty one.
fn cone_arm<T: Real>(rims: &[Rim<T>], sin_a: T) -> T {
    match rims.first() {
        Some(Rim {
            level: RimLevel::Length(v),
            ..
        }) => v.abs() * sin_a,
        _ => T::one(),
    }
}

// ---------------------------------------------------------------------
// Sphere
// ---------------------------------------------------------------------

/// **A sphere face bounded by circle arcs, holes included** — closed
/// form, no chart. The radial flux term needs the face's area, and on a
/// sphere of radius `R` Gauss–Bonnet gives it from the boundary alone:
///
/// ```text
/// Area / R²  =  2π(1 − r)  −  Σ_loops ( Σ_arcs ∫κ_g ds  +  Σ_vertices ε )
/// ```
///
/// for a face of one outer loop and `r` rings (`χ = 1 − r`), with `κ_g`
/// the geodesic curvature against the face's OUTWARD normal
/// `N = σ·(p − c)/R` (`σ` the sense bit) and `ε` the signed turning
/// angle at each vertex, both read by [`sphere_loop`]. The flux is
/// `σ·R·Area + c·A⃗`, with `A⃗` summed over every loop.
///
/// Every face the sphere carries is bounded by circles — a rim, a
/// meridian great circle, a section tilted against the chart — so this
/// one form measures the lune, the zone, the cap, a face no chart
/// rectangle describes (the L-shaped complement of a half-cap), and a
/// meridian that arrives in lineage pieces, which turns by zero at each
/// joint and needs no fold.
///
/// The premises are the formula's own, each a named decide: every edge
/// is a circle on the sphere, every loop closes and no junction is a
/// cusp ([`sphere_loop`]); the region left of the outer loop has solid
/// angle strictly inside `(0, 4π)` (`props_sphere_loop_area`, the
/// smaller of it and its complement's); each ring's hole has positive
/// solid angle (`props_sphere_hole_area`); and the face's own solid
/// angle, the region less its holes, is positive
/// (`props_sphere_face_area`, decided once, by [`sphere_loop_side`],
/// for a face with holes — a one-loop face's is its region's). Those
/// see degeneracy only: a loop read under the wrong bit measures a
/// complement, which is in range too. What reads the bit against the
/// boundary is [`sphere_loop_side`], first; a bit it contradicts
/// refuses, because the radial term reads the bit. Every solid angle
/// is read by [`solid_angle`], which keeps a small region's digits.
///
/// # Errors
///
/// [`PropsError::SphereLoop`] naming the failed premise,
/// [`PropsError::SenseContradicted`] for a bit a one-loop face's
/// boundary contradicts and [`PropsError::SenseInverted`] for one a
/// holed face's does ([`sphere_loop_side`]),
/// [`PropsError::Unimplemented`] for a spline or spiric edge,
/// [`PropsError::Escalated`] in the band.
fn sphere_face<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    loops: &[&[LoopEdge<T>]],
    sense: bool,
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let gauss = sphere_loops(center, radius, loops, band)?;
    let Some((outer, rings)) = gauss.split_first() else {
        return Err(PropsError::SphereLoop {
            what: "sphere face with an empty boundary",
        });
    };
    if let MaterialSign::Encoded(side) = side_of_sphere_loops(center, radius, axis, &gauss, band)?
        && (side == Sign::Positive) != sense
    {
        // A holed face's side is its area's sign, which no pole enters.
        return Err(if rings.is_empty() {
            PropsError::SenseContradicted
        } else {
            PropsError::SenseInverted
        });
    }
    // The face lies left of its loops under the OUTWARD normal, which
    // is the chart normal's left exactly when the bit is set.
    let region = solid_angle(center, outer, sense);
    require_solid_angle(
        "props_sphere_loop_area",
        region.min(solid_angle(center, outer, !sense)),
        radius,
        band,
    )?;
    let mut solid = region;
    for ring in rings {
        let hole = solid_angle(center, ring, !sense);
        require_solid_angle("props_sphere_hole_area", hole, radius, band)?;
        solid = solid - hole;
    }
    let area = radius.powi(2) * solid;
    let va = gauss.iter().fold(Vec3::zero(), |va, g| va + g.vector_area);
    let radial = radius * area;
    let radial = if sense { radial } else { -radial };
    Ok(FaceContribution {
        flux: radial + (center - Point3::origin()).dot(va),
        area,
    })
}

/// **The solid angle of the region on one side of a loop**, left of it
/// under the chart normal when `left`, right of it otherwise — read
/// without the cancellation Gauss–Bonnet suffers on a small region.
///
/// Gauss–Bonnet gives the left region as `2π − turning` (the right as
/// `2π + turning`), whose error is a few ulps of `2π` however small
/// the region: a cap of height `h` keeps only `h/R` of the digits. The
/// same region is also a fan about an apex `o` inside it, one piece
/// per arc: the geodesic triangle `(o, A, B)` over the arc's ends
/// (Van Oosterom–Strackee, [`triangle_solid_angle`]), plus the segment
/// between that geodesic and the arc — the sector the arc sweeps about
/// its left pole `Q`, `Δt·(1 − d)` with `d` the carrier plane's offset
/// along `Q` over `R`, less the triangle `(Q, A, B)`. Every term is the
/// size of the region, so the fan keeps the region's own digits, and
/// `1 − d` is read without cancellation as `ρ²/R²/(1 + d)` where `d`
/// is positive. Each arc is fanned in thirds, so no triangle spans a
/// half-turn about its apex, where its sign would be undecided. The
/// apex is the region's own vector area `∫n dA` ([`SphereLoop`]'s, or
/// its negation for the right region), which points into a small
/// region; triangle solid angles are homogeneous in their apex, so it
/// needs no normalising.
///
/// The fan is exact modulo `4π` for an apex anywhere off the loop, so
/// it is read against Gauss–Bonnet's value: shifted by the whole turns
/// that bring it nearest, and taken where it then agrees to within
/// [`FAN_AGREEMENT_SR`]. Where it does not (an apex at the loop, or at
/// the centre for a loop whose vector area vanishes), the answer
/// fades to Gauss–Bonnet's by twice that. The blend is the fan less a
/// residue `r(gap)` of the gap between the two forms, built from
/// `min`/`max` so that it is EXACTLY zero while the gap is inside the
/// agreement band: an enclosure then carries the fan's own width, not
/// Gauss–Bonnet's, and a value carries the fan's bits. It is taken
/// only for a region of at most [`FAN_REGION_SR`]
/// ([`Real::select_le_zero`], the door that picks the well-conditioned
/// candidate).
///
/// The fan's error has an absolute floor, a few ulps of each
/// triangle's own size: on a region far smaller than a steradian it
/// keeps a few ulps of the region where the fan's apex sits on an
/// axis the region is symmetric about, and some `1e-17` sr in a
/// general pose (`sweep/tests/a_thin_cap_keeps_its_digits.rs` pins
/// both).
fn solid_angle<T: Decide>(center: Point3<T>, lp: &SphereLoop<T>, left: bool) -> T {
    let tau = T::tau();
    let gauss = if left {
        tau - lp.turning()
    } else {
        tau + lp.turning()
    };
    let pieces = T::from_f64(3.0);
    // Each arc's points at its thirds, from the sphere's centre.
    let thirds: Vec<[Vec3<T>; 4]> = lp
        .arcs
        .iter()
        .map(|arc| {
            let u = arc.start - arc.centre;
            let w = arc.axis.cross(u);
            let at = |phi: T| (arc.centre - center) + u * phi.cos() + w * phi.sin();
            let delta = arc.extent.get() / pieces;
            // The arc's own end points, as its edge evaluates them, so
            // the faces either side of an edge fan over the same points.
            let (first, last) = if arc.forward {
                (arc.from, arc.at)
            } else {
                (arc.at, arc.from)
            };
            [first - center, at(delta), at(delta + delta), last - center]
        })
        .collect();
    // The apex: the centroid of those points, a sum of vectors of the
    // sphere's radius that leans into a small region, so its direction
    // is read to the points' own precision.
    let apex = thirds
        .iter()
        .flat_map(|ends| ends.iter())
        .fold(Vec3::zero(), |sum, &p| sum + p);
    let mut fan = T::zero();
    for (arc, ends) in lp.arcs.iter().zip(&thirds) {
        let delta = arc.extent.get() / pieces;
        // The carrier plane's offset along its (unit) axis, over the
        // sphere's radius as the carrier states it, and `ρ²/R²`.
        let offset = (arc.centre - center).dot(arc.axis);
        let reach = offset.powi(2) + arc.radius.powi(2);
        let d = offset / reach.sqrt();
        let fit = arc.radius.powi(2) / reach;
        // `1 − d`, accurate for either sign of `d`.
        let one_less = |d: T| {
            let below = d.min(T::zero());
            fit / (T::one() + d.max(T::zero())) + below.powi(2) - below
        };
        // The side the stored axis points into is left of the stored
        // direction; the region's side of a traversed arc is that one
        // when its traversal and the region's side agree.
        let stored_left = arc.forward == left;
        let d = if stored_left { d } else { -d };
        let k = one_less(d);
        let (s, c) = (delta / T::from_f64(2.0)).sin_cos();
        // The segment `Δt·(1 − d) − tri(Q, A, B)` per third, as
        // `2(1 − d)·g − d·f` with `g = atan2(d·sin(Δt/2), cos(Δt/2))`
        // and `f = tri(Q, A, B) = 2·atan2(k·s·c, 1 − k·s²)`: each term
        // carries the factor that vanishes with the segment, `d` on a
        // near-great circle and `1 − d` on a small one, so neither
        // cancels across the region's own size.
        let g = (d * s).atan2(c);
        let f = T::from_f64(2.0) * (k * s * c).atan2(T::one() - k * s.powi(2));
        let segment = T::from_f64(2.0) * k * g - d * f;
        for k in 0..3 {
            let wedge = triangle_solid_angle(apex, ends[k], ends[k + 1]);
            fan = fan + segment + if stored_left { wedge } else { -wedge };
        }
    }
    // Close the fan across the rounding gap at each vertex, where one
    // edge's evaluated end is not quite the next one's start.
    for (i, arc) in lp.arcs.iter().enumerate() {
        let next = &lp.arcs[(i + 1) % lp.arcs.len()];
        let gap = triangle_solid_angle(apex, arc.at - center, next.from - center);
        fan = if left { fan + gap } else { fan - gap };
    }
    let four_pi = tau + tau;
    // The fan's branch, by floor, and deliberately not decided: no
    // decision and no identity reads `turns`, only the numeric blend
    // below. Where `gauss − fan` straddles a half period — the interval
    // scalar's enclosure of the solid-angle sum can be wider than one —
    // the floor's enclosure spans both branches and the blend encloses
    // the true value, where a decision would fail on an enclosure wider
    // than a period. At an `f64` tie either branch leaves a gap of `±2π`,
    // far past the agreement band, so either blends to Gauss–Bonnet's
    // value exactly. At `Sym` the blend is numeric whatever `turns` is.
    // (`props-torus-lift-and-gauss-bonnet-fan-pick-a-branch-by-floor`, closed by PR 5375.)
    let turns = ((gauss - fan) / four_pi + T::from_f64(0.5)).floor();
    let fixed = fan + four_pi * turns;
    let gap = fixed - gauss;
    // The residue the fan gives up to Gauss–Bonnet: zero for
    // `|gap| ≤ reach` ([`FAN_AGREEMENT_SR`]), the whole gap for
    // `|gap| ≥ 2·reach`, linear between — `shrink + clamp(shrink)`, with
    // `shrink` the gap less `reach` toward zero. Both halves are
    // `min`/`max` of the gap against constants, so an enclosure inside
    // the band yields `[0, 0]` exactly.
    let reach = T::from_f64(FAN_AGREEMENT_SR);
    let shrink = (gap - reach).max(T::zero()) + (gap + reach).min(T::zero());
    let residue = shrink + shrink.max(-reach).min(reach);
    let blended = fixed - residue;
    (gauss - T::from_f64(FAN_REGION_SR)).select_le_zero(blended, gauss)
}

/// **How far the fan form may stand from Gauss–Bonnet's value and
/// still be taken** ([`solid_angle`]), in steradians: far wider than
/// either form's rounding (a few ulps of `2π`, `~1e-15`, summed over a
/// loop's terms) and far narrower than any region. A fan further off
/// than this is one whose apex sits at the loop or at the centre, and
/// the answer fades to Gauss–Bonnet's by twice it.
const FAN_AGREEMENT_SR: f64 = 1e-12;

/// **The largest region the fan form serves** ([`solid_angle`]), in
/// steradians. Below it Gauss–Bonnet's `2π ∓ turning` cancels to a
/// region whose own digits it cannot keep; above it Gauss–Bonnet's
/// rounding is already a few ulps of the region, and the region's
/// vector area — the fan's apex — can vanish (a hemisphere's does),
/// where an enclosure's triangles are undefined.
const FAN_REGION_SR: f64 = 1.0;

/// The signed solid angle of the geodesic triangle `(q, a, b)` seen
/// from the sphere's centre, positive when the three run
/// counterclockwise seen from outside (Van Oosterom–Strackee). The
/// form is homogeneous in each vertex, so none needs normalising.
fn triangle_solid_angle<T: Real>(q: Vec3<T>, a: Vec3<T>, b: Vec3<T>) -> T {
    let (nq, na, nb) = (q.norm(), a.norm(), b.norm());
    let across = nq * na * nb + q.dot(a) * nb + q.dot(b) * na + a.dot(b) * nq;
    T::from_f64(2.0) * q.dot(a.cross(b)).atan2(across)
}

/// Require a solid angle (sr) to be definitely positive, levered by the
/// sphere radius; a refusal is [`PropsError::SphereLoop`] named by the
/// predicate.
fn require_solid_angle<T: Decide>(
    name: &'static str,
    angle: T,
    radius: T,
    band: Band,
) -> Result<(), PropsError> {
    match classify(
        name,
        Margin::levered(angle, radius),
        band,
        PropsCheck::Inventory,
    )? {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(PropsError::SphereLoop { what: name }),
    }
}

/// One loop of a sphere face read against the CHART normal
/// `(p − c)/R`: its arcs in traversal order, the geodesic curvature
/// they integrate, the turning at its vertices, and its vector area
/// `(1/2)∮(p − c)×dp` ([`loop_vector_area`]), which is `∫n dA` over
/// the region left of it.
struct SphereLoop<T: Real> {
    arcs: Vec<SphereArc<T>>,
    curvature: T,
    corners: T,
    vector_area: Vec3<T>,
}

impl<T: Real> SphereLoop<T> {
    /// `Σ∫κ_g ds + Σε` against the chart normal.
    fn turning(&self) -> T {
        self.curvature + self.corners
    }
}

/// One circle arc of a sphere face's loop: its carrier, its stored
/// start and span, and its traversal ends with the unit traversal
/// tangent at each.
struct SphereArc<T: Real> {
    centre: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    start: Point3<T>,
    extent: Extent<T>,
    forward: bool,
    from: Point3<T>,
    depart: Vec3<T>,
    at: Point3<T>,
    arrive: Vec3<T>,
}

/// [`sphere_loop`] over every loop of a face.
fn sphere_loops<T: Decide>(
    center: Point3<T>,
    radius: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<Vec<SphereLoop<T>>, PropsError> {
    loops
        .iter()
        .map(|edges| sphere_loop(center, radius, edges, band, Cusp::Band))
        .collect()
}

/// **One loop's Gauss–Bonnet terms**, against the chart normal
/// `N = (p − c)/R`. A circle arc on the sphere has constant geodesic
/// curvature: with carrier centre `C`, unit axis `â` and a traversal of
/// `Δt` in the direction `s = ±1` (the `forward` bit),
///
/// ```text
/// ∫κ_g ds  =  s·((C − c)·â / R)·Δt
/// ```
///
/// (the circle's curvature vector is `−(p − C)/ρ²`, and its component
/// along `N × T` is `s·((C − c)·â)/(ρR)` since `p − C ⊥ â`). The
/// turning angle at a vertex is the angle from the arriving traversal
/// tangent to the departing one about `N` there, in `(−π, π)`. Every
/// quantity is stored data: the arcs' certified spans and carriers.
///
/// Verified first: each edge is a circle ON the sphere — its centre
/// offset parallel to its axis (`props_sphere_circle_on`) and its
/// radius fitting (`props_sphere_circle_fit`) — whose stored span is
/// one certification produces, `0 < Δt ≤ τ`
/// ([`require_span_within_period`], at the circle's radius);
/// each arc ends where the next begins
/// (`props_loop_closed`, refused as [`PropsError::SphereLoop`]:
/// Gauss–Bonnet reads a closed curve, so closure is checked rather
/// than trusted from the vertex tags). It is decided in space, not as
/// the periodic-chart closure ([`closure`]): the sphere has no global
/// chart whose periods a loop can wind, since its azimuth is undefined
/// at the poles, and Gauss–Bonnet needs only the curve closed; and no
/// junction is a cusp ([`require_cusp_free`]). Gauss–Bonnet reads a
/// SIMPLE loop, and simplicity is not decided here: a loop that
/// crosses itself, or visits a vertex twice (two whole circles joined
/// at a point), answers the formula's value for it.
fn sphere_loop<T: Decide>(
    center: Point3<T>,
    radius: T,
    edges: &[LoopEdge<T>],
    band: Band,
    cusp: Cusp,
) -> Result<SphereLoop<T>, PropsError> {
    if edges.is_empty() {
        return Err(PropsError::SphereLoop {
            what: "sphere face with an empty boundary",
        });
    }
    let normal = |p: Point3<T>| (p - center) / radius;
    let mut curvature = T::zero();
    let mut arcs = Vec::with_capacity(edges.len());
    for e in edges {
        let Curve3::Circle {
            center: c_c,
            axis: n_c,
            radius: r_c,
            ..
        } = e.carrier
        else {
            return Err(match e.carrier {
                Curve3::Nurbs(_) | Curve3::Spiric { .. } => PropsError::Unimplemented,
                _ => PropsError::SphereLoop {
                    what: "sphere boundary edge is not a circle",
                },
            });
        };
        let w = c_c - center;
        require_on_loop(
            "props_sphere_circle_on",
            Margin::of(w.cross(n_c).norm()),
            band,
        )?;
        require_on_loop(
            "props_sphere_circle_fit",
            Margin::of((w.norm_squared() + r_c.powi(2)).sqrt() - radius),
            band,
        )?;
        let extent = require_circle_span(
            (e.t0, e.t1),
            r_c,
            band,
            [
                "props_sphere_arc_span_forward",
                "props_sphere_arc_span_winding",
            ],
            |what| PropsError::SphereLoop { what },
        )?;
        let (from, at) = e.traversal_ends();
        let span = extent.get();
        // `N·â` is the same at every point of the arc.
        let k = normal(from).dot(n_c) * span;
        curvature = if e.forward {
            curvature + k
        } else {
            curvature - k
        };
        let tangent = |p: Point3<T>| {
            let d = n_c.cross(p - c_c);
            let d = d / d.norm();
            if e.forward { d } else { -d }
        };
        arcs.push(SphereArc {
            centre: c_c,
            axis: n_c,
            radius: r_c,
            start: e.p0(),
            extent,
            forward: e.forward,
            from,
            depart: tangent(from),
            at,
            arrive: tangent(at),
        });
    }
    let mut corners = T::zero();
    for (i, arc) in arcs.iter().enumerate() {
        let next = &arcs[(i + 1) % arcs.len()];
        require_on_loop(
            "props_loop_closed",
            Margin::of((next.from - arc.at).norm()),
            band,
        )?;
        match cusp {
            Cusp::Band => require_cusp_free(arc.arrive, next.depart, radius, band)?,
            Cusp::Rounding => require_turn_read(arc.arrive, next.depart)?,
        }
        corners = corners
            + normal(arc.at)
                .dot(arc.arrive.cross(next.depart))
                .atan2(arc.arrive.dot(next.depart));
    }
    Ok(SphereLoop {
        arcs,
        curvature,
        corners,
        vector_area: loop_vector_area(edges, center)?,
    })
}

/// Require a premise of [`sphere_loop`] to read zero, refusing as
/// [`PropsError::SphereLoop`] named by the predicate.
fn require_on_loop<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<(), PropsError> {
    match classify(name, margin, band, PropsCheck::Inventory)? {
        Sign::Zero => Ok(()),
        Sign::Positive | Sign::Negative => Err(PropsError::SphereLoop { what: name }),
    }
}

/// How [`sphere_loop`] reads a junction near a cusp.
#[derive(Clone, Copy)]
enum Cusp {
    /// At the band ([`require_cusp_free`]): props' premise.
    Band,
    /// Against the tangents' rounding alone ([`require_turn_read`]):
    /// the chart-singularity reading's, whose question is the sign of
    /// the junction's turn, which a sharp corner keeps.
    Rounding,
}

/// A junction whose turn's sign survives the tangents' rounding:
/// `chart_singularity_cusp`, the chord between the departing tangent and
/// the reversed arriving one held against the rounding of two unit
/// vectors.
fn require_turn_read<T: Decide>(arrive: Vec3<T>, depart: Vec3<T>) -> Result<(), PropsError> {
    match decide(
        "chart_singularity_cusp",
        Margin::sign_of((depart + arrive).norm() - T::from_f64(16.0 * f64::EPSILON)),
        Band::enclosure_sign(),
    ) {
        Ok(Sign::Positive) => Ok(()),
        _ => Err(PropsError::SphereLoop {
            what: "chart_singularity_cusp",
        }),
    }
}

/// A loop junction whose arriving and departing tangents are
/// antiparallel turns by ±π, which no signed angle decides:
/// `props_sphere_loop_cusp`, the chord between the departing tangent
/// and the reversed arriving one, metered at the sphere radius.
fn require_cusp_free<T: Decide>(
    arrive: Vec3<T>,
    depart: Vec3<T>,
    radius: T,
    band: Band,
) -> Result<(), PropsError> {
    match classify(
        "props_sphere_loop_cusp",
        Margin::levered((depart + arrive).norm(), radius),
        band,
        PropsCheck::Exact,
    )? {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(PropsError::SphereLoop {
            what: "props_sphere_loop_cusp",
        }),
    }
}

/// **Which side of its loops a sphere face lies on, read without its
/// sense bit** — the boundary's own encoding of the side, which the
/// flux arm ([`sphere_face`]) and tier 3's check 6
/// ([`boundary_material_sign_loops`]) both hold the bit against.
/// `Encoded(Positive)` says the face is the region left of its loops
/// under the CHART normal, the side a `sense: true` face claims.
///
/// * **A face with holes** is read by which side of its loops has
///   positive area, with no chart pole involved
///   (`props_sphere_face_area`, the face's solid angle under each
///   reading, [`solid_angle`]). Under the true side that area is the
///   face's, positive; under the other it is `4π(1 − r)R² − Area`,
///   negative for any `r ≥ 1`, so at most one reading is positive.
///   Neither is when the holes overlap or reach past the outer loop's
///   region, and the loops then bound no face: that is the one home of
///   the premise that the face's area is positive, which a one-loop
///   face meets through `props_sphere_loop_area` ([`sphere_face`]).
/// * **A one-loop face** has no such asymmetry — a loop bounds the face
///   and its complement alike — and is read as the side of its loop
///   that holds no chart pole ([`pole_free_side`]).
///
/// # Errors
///
/// [`sphere_loop`]'s premises; [`PropsError::SphereLoop`] for a ringed
/// face of positive area under neither reading.
fn sphere_loop_side<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<MaterialSign, PropsError> {
    let gauss = sphere_loops(center, radius, loops, band)?;
    side_of_sphere_loops(center, radius, axis, &gauss, band)
}

/// [`sphere_loop_side`] over loops already read.
fn side_of_sphere_loops<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    gauss: &[SphereLoop<T>],
    band: Band,
) -> Result<MaterialSign, PropsError> {
    match gauss {
        [] => Err(PropsError::SphereLoop {
            what: "sphere face with an empty boundary",
        }),
        [one] => pole_free_side(center, radius, axis, one, band),
        [_, _, ..] => {
            for (left, side) in [(true, Sign::Positive), (false, Sign::Negative)] {
                if classify(
                    "props_sphere_face_area",
                    Margin::levered(claimed_area(center, gauss, left), radius),
                    band,
                    PropsCheck::Inventory,
                )? == Sign::Positive
                {
                    return Ok(MaterialSign::Encoded(side));
                }
            }
            Err(PropsError::SphereLoop {
                what: "props_sphere_face_area",
            })
        }
    }
}

/// **The area, in steradians, that `loops` bound on their `left`
/// side** under the chart normal: the first loop's side less each
/// other's opposite side (`Ω₀ − Σ (4π − Ωᵢ)`, each `Ω` a
/// [`solid_angle`]), the Gauss–Bonnet area of a sphere with that many
/// holes. Under the wrong side of a face with holes it is
/// `4π(1 − r) − Area`, negative for `r ≥ 1` holes. The one home of the
/// reading, for a face's side ([`side_of_sphere_loops`]) and a seam
/// region's claims ([`sphere_region_cover`]) alike.
fn claimed_area<T: Decide>(center: Point3<T>, loops: &[SphereLoop<T>], left: bool) -> T {
    let Some((outer, rings)) = loops.split_first() else {
        return T::from_f64(2.0) * T::tau();
    };
    rings
        .iter()
        .fold(solid_angle(center, outer, left), |s, ring| {
            s - solid_angle(center, ring, !left)
        })
}

/// **The side of a one-loop sphere face's loop that holds no chart
/// pole.** A chart singularity inside a face is a vertex of it, so a
/// face lies on the side of its loop that holds neither pole; each
/// pole off the loop says which side that is ([`pole_side`]).
///
/// The answer is `Encoded` when the poles off the loop that read agree
/// on a side, and `Unencoded` when both poles lie on the loop (a lune,
/// a hemisphere split at its poles: the loop alone does not say), when
/// neither reading is decided, or when the poles off the loop read on
/// OPPOSITE sides. That last loop has a pole on each side, so whichever
/// side is material holds a pole and the reading says nothing about
/// the bit; it is not this reader's to refuse ([`MaterialSign`]).
fn pole_free_side<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    lp: &SphereLoop<T>,
    band: Band,
) -> Result<MaterialSign, PropsError> {
    agreed_side(axis, |pole| pole_side(center, radius, pole, lp, band))
}

/// **The side the poles' readings agree on.** `read` reads the side at
/// the pole in direction `pole`, or `None` for a pole it does not read;
/// an in-band reading leaves that pole unread too, and never refuses.
/// `Encoded` when the readings made agree, `Unencoded` when none is made
/// or two disagree.
fn agreed_side<T: Decide>(
    axis: Vec3<T>,
    read: impl Fn(Vec3<T>) -> Result<Option<Sign>, PropsError>,
) -> Result<MaterialSign, PropsError> {
    let up = axis / axis.norm();
    let mut reading: Option<Sign> = None;
    // The sphere's singular points, from their one home, as directions.
    let origin = Point3::new(T::zero(), T::zero(), T::zero());
    for (_, pole) in crate::chart_singularity::sphere_poles(origin, up, T::one()) {
        let pole = pole - origin;
        let side = match read(pole) {
            Ok(side) => side,
            Err(PropsError::Escalated { .. }) => None,
            Err(other) => return Err(other),
        };
        match (reading, side) {
            (Some(a), Some(b)) if a != b => return Ok(MaterialSign::Unencoded),
            (None, side) => reading = side,
            _ => {}
        }
    }
    Ok(reading.map_or(MaterialSign::Unencoded, MaterialSign::Encoded))
}

/// **A sphere seam region's loops, read once** for both region reads
/// ([`sphere_region_material_sign`], [`sphere_region_cover`]): its
/// boundary, chained into loops, and every loop of every member, each
/// parsed by [`sphere_loop`] against the sphere's chart.
pub struct SeamLoops<T: Real> {
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    boundary: Vec<SphereLoop<T>>,
    members: Vec<SphereLoop<T>>,
}

/// [`SeamLoops`] of the region on `surface` whose boundary is `boundary`
/// and whose members' loops are `members`.
///
/// # Errors
///
/// [`sphere_loop`]'s premises over the boundary and the members;
/// [`PropsError::SphereLoop`] for a surface that is not a sphere.
pub fn seam_loops<T: Decide>(
    surface: &Surface<T>,
    boundary: &[&[LoopEdge<T>]],
    members: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<SeamLoops<T>, PropsError> {
    let Surface::Sphere {
        center,
        radius,
        axis,
        ..
    } = *surface
    else {
        return Err(PropsError::SphereLoop {
            what: "a seam region read on a surface that is not a sphere",
        });
    };
    Ok(SeamLoops {
        center,
        radius,
        axis,
        boundary: sphere_loops(center, radius, boundary, band)?,
        members: sphere_loops(center, radius, members, band)?,
    })
}

/// **The side of a seam region of one sphere's faces**, read off the
/// region's own boundary. A face whose loop holds both chart poles
/// encodes no side alone ([`pole_free_side`]), but the faces on its
/// surface that it reaches across shared edges tile one region, the
/// loops' traversals agree across every edge they share, and that
/// region encodes a side wherever its boundary does.
///
/// * **No boundary**: the members close the sphere. `Unencoded`: covered
///   once, they are a shell whose orientation tier 3's volume checks
///   read; their overlap is the cover census's ([`sphere_region_cover`]),
///   not the side's.
/// * **Two or more boundary loops**: the sign of the region's
///   Gauss–Bonnet area, as for a face with holes.
/// * **One boundary loop**: read at each pole on an edge interior to
///   the region, on some member's loop and off the boundary. The region
///   holds that pole, so it is the side of the boundary that holds it.
///   A pole on no member's loop is not read: the region is the side
///   without it only where no face holds a chart singularity off its
///   loop, which tier 3 refuses ([`crate::chart_singularity`]) and this
///   reader does not assume. `Unencoded` when no pole
///   is read, among them a boundary carrying both poles: that region
///   and its complement are both legal, and for a region of one face,
///   flipping its bit describes the other one, a valid body and not a
///   defect. For a region of two faces or more, all flipped, the faces
///   overlap, which the cover census refuses.
///
/// # Errors
///
/// As [`boundary_material_sign_loops`].
pub fn sphere_region_material_sign<T: Decide>(
    region: &SeamLoops<T>,
    band: Band,
) -> Result<MaterialSign, PropsError> {
    let SeamLoops {
        center,
        radius,
        axis,
        ..
    } = *region;
    let [one] = region.boundary.as_slice() else {
        return if region.boundary.is_empty() {
            Ok(MaterialSign::Unencoded)
        } else {
            side_of_sphere_loops(center, radius, axis, &region.boundary, band)
        };
    };
    agreed_side(axis, |pole| {
        let p = center + pole * radius;
        for arc in region.members.iter().flat_map(|m| &m.arcs) {
            if pole_against_arc(arc, p, band)?.1 {
                return Ok(pole_side(center, radius, pole, one, band)?.map(Sign::flip));
            }
        }
        Ok(None)
    })
}

/// **How many times a seam region's faces cover it**, as their sense
/// bits claim ([`sphere_region_cover`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeamCover {
    /// The members claim the region once: no overlap.
    Once,
    /// The members claim the region `times` times (`times ≥ 2`): their
    /// faces overlap. A region whose members all carry the wrong bit
    /// claims it `times = Σ χ_f − χ_R + 1` times
    /// ([`sphere_region_cover`]'s count), among them:
    ///
    /// * three or more faces covering the sphere twice over, with no
    ///   boundary (`times = 2`);
    /// * `n ≥ 2` one-loop faces bounded by `m ≥ 1` loops, all flipped
    ///   (`times = n + m − 1`): a both-poles pair is `2`;
    /// * `n ≥ 3` lunes of a whole ball, all flipped (`times = n − 1`):
    ///   an inside-out ball of two lunes claims the sphere once and is
    ///   the volume checks', one of three lunes or more is refused here.
    Over {
        /// How many times the members claim the region.
        times: usize,
    },
    /// A threshold read in the band: the cover cannot be certified, so
    /// the census refuses rather than read `Once`. A whole census
    /// stands `2πR` metres from both thresholds, past every span read
    /// its loops passed ([`sphere_region_cover`]), so no region of a
    /// body whose loops read reaches it: it is the census's loud end.
    Unresolved,
    /// The members' sense bits disagree, so the claims are no cover
    /// and the census says nothing. A member whose bit differs from a
    /// neighbour's across a shared edge is a lamina there, check 4's to
    /// refuse; the census does not read it.
    MixedSense,
}

/// **The cover census of a seam region**: how many times its members
/// claim it under their sense bits. `claims` pairs each member's sense
/// bit with the run of `region`'s member loops that are its own.
///
/// **The census.** Each member claims the side of its loops its bit
/// names, of area [`claimed_area`]. The members' half-edges agree across
/// every edge they share, so the number of claims over a point jumps by
/// one across the region's boundary and nowhere else: it is the
/// boundary's claimed side plus a whole number `c` of spheres, and the
/// census `S = Σ claims − boundary's claim` is `4π·c` steradians exactly
/// (with no boundary, the boundary's claim is the whole sphere, `4π`).
/// The members claim the region `c + 1` times. Read under its true bits
/// a region reads `c = 0`, whatever its poles and whatever its boundary
/// carries; no pole is read.
///
/// **The count under the wrong bits.** A member of `L` loops and true
/// area `A` claims `4π(2 − L) − A` under its flipped bit, and a boundary
/// of `m` loops likewise claims `4π(2 − m) − A_R`, so with every
/// member's bit wrong
/// `c = Σ_f (2 − L_f) − (2 − m) = Σ_f χ_f − χ_R`, the Euler
/// characteristics of the members and the region as spheres with holes.
/// For `n` one-loop members, `c = n + m − 2`. It is never negative:
/// counting the region's cells, `χ_R = Σ_f χ_f + V − E` over its
/// vertices and edges, and its boundary loops hold as many of each, so
/// `c = E° − V°` over those off the boundary; each vertex off it ends
/// two such edges or more, so `2E° ≥ 2V°`. Under any bits, then, a seam
/// region's census is `0` or `Σ χ_f − χ_R ≥ 0`.
///
/// **Reading `c`.** `c` is read against the half-sphere thresholds
/// `(c ± ½)·4π` with the band's decide (`props_sphere_region_cover`),
/// each a solid angle levered by the radius `R`, as
/// [`side_of_sphere_loops`] reads a face's area. A whole census stands
/// `2πR` metres from both thresholds. Every arc of the region's loops
/// passed its own span read (`props_sphere_arc_span_forward`), a span
/// levered by the arc's radius `ρ ≤ R`, so `2πR ≥ span·ρ` exceeds a
/// band each of them cleared: where the loops read, a whole census is
/// never in the band, and a sphere too small for it refuses at its
/// loops first. A threshold read in the band refuses
/// [`SeamCover::Unresolved`].
///
/// What it does not read:
///
/// * a ringed member whose bit contradicts its own area reading claims
///   a negative "area" (`4π(1 − r) − Area`). Signed Gauss–Bonnet area
///   is additive, so that claim can keep `c = 0`; the member's own face
///   read ([`sphere_loop_side`]) refuses it;
/// * members whose bits disagree ([`SeamCover::MixedSense`]).
///
/// # Errors
///
/// [`PropsError::SphereLoop`] for a region with no member, and for a
/// census decided below `−½` or past `members + ½`
/// (`props_sphere_region_cover_untiled`): no seam region's count lies
/// there (the count above), so the loops handed in tile no region, an
/// internal error rather than a reading of a body.
pub fn sphere_region_cover<T: Decide>(
    region: &SeamLoops<T>,
    claims: &[(bool, core::ops::Range<usize>)],
    band: Band,
) -> Result<SeamCover, PropsError> {
    let untiled = PropsError::SphereLoop {
        what: "props_sphere_region_cover_untiled",
    };
    let Some((sense, _)) = claims.first() else {
        return Err(PropsError::SphereLoop {
            what: "a seam region with no member",
        });
    };
    if claims.iter().any(|(s, _)| s != sense) {
        return Ok(SeamCover::MixedSense);
    }
    let sphere = T::from_f64(2.0) * T::tau();
    let mut census = -claimed_area(region.center, &region.boundary, *sense);
    for (_, run) in claims {
        let loops = region
            .members
            .get(run.clone())
            .ok_or(PropsError::SphereLoop {
                what: "a seam region's claim names loops it does not hold",
            })?;
        census = census + claimed_area(region.center, loops, *sense);
    }
    // Above the threshold `c` spheres, below it, or in its band.
    let above = |c: f64| match classify(
        "props_sphere_region_cover",
        Margin::levered(census - T::from_computed(c) * sphere, region.radius),
        band,
        PropsCheck::Inventory,
    ) {
        Ok(Sign::Positive) => Some(true),
        Ok(Sign::Negative) => Some(false),
        Ok(Sign::Zero) | Err(_) => None,
    };
    match above(-0.5) {
        Some(true) => {}
        Some(false) => return Err(untiled),
        None => return Ok(SeamCover::Unresolved),
    }
    // Each member's claim is at most the whole sphere, so the members
    // claim the region at most `members + 1` times.
    for c in 0..=claims.len() {
        match above(c as f64 + 0.5) {
            Some(false) if c == 0 => return Ok(SeamCover::Once),
            Some(false) => return Ok(SeamCover::Over { times: c + 1 }),
            Some(true) => {}
            None => return Ok(SeamCover::Unresolved),
        }
    }
    Err(untiled)
}

/// Where the point `p` on the sphere stands against `arc`: the side of
/// its carrier plane (`props_sphere_pole_plane`, in metres), and whether
/// it lies on the arc itself, in that plane and within the arc's span
/// (`props_sphere_pole_on_arc`, [`arc_point_margin`]).
fn pole_against_arc<T: Decide>(
    arc: &SphereArc<T>,
    p: Point3<T>,
    band: Band,
) -> Result<(Sign, bool), PropsError> {
    let plane = classify(
        "props_sphere_pole_plane",
        Margin::of(arc.axis.dot(p - arc.centre)),
        band,
        PropsCheck::Inventory,
    )?;
    let on = plane == Sign::Zero
        && classify(
            "props_sphere_pole_on_arc",
            arc_point_margin(arc, p),
            band,
            PropsCheck::Inventory,
        )? != Sign::Negative;
    Ok((plane, on))
}

/// **Which side of the loop the pole in direction `pole` lies on**:
/// `Some(Positive)` when it is right of the loop under the chart
/// normal (so the pole-free side is the left one), `Some(Negative)`
/// when it is left, `None` when the pole lies on the loop.
///
/// The pole is on the loop when it lies in some arc's carrier plane
/// (`props_sphere_pole_plane`, in metres) and within that arc's span
/// (`props_sphere_pole_on_arc`, [`arc_point_margin`]).
///
/// Otherwise the loop is projected stereographically from the pole onto
/// the plane through the centre normal to it, oriented by `−pole` so
/// the projection preserves the chart normal's orientation. The pole
/// goes to infinity, so the region left of the loop is the bounded one
/// exactly when the pole is not in it, and that is when the image turns
/// by `+2π` rather than `−2π`. The image's turning is the sum of:
///
/// * the vertex turning angles, which the projection keeps (conformal
///   and orientation-preserving): [`SphereLoop::corners`] as read;
/// * each arc's image, an arc of a circle (or of a line, when the pole
///   is on its carrier but off the arc), whose sweep is twice the angle
///   from its tangent at the start to its chord. That angle lies in
///   `(0, π)` along the image's circulation, which is positive exactly
///   when the cap left of the arc's carrier does not hold the pole; it
///   is read through a quarter-turn shift so its branch cut sits half a
///   turn from both ends of that range. A whole circle
///   (`props_sphere_pole_arc_full`) sweeps a whole turn along its
///   circulation.
///
/// The sum is a whole turn up to rounding and is decided by its sign,
/// levered by `R` (`props_sphere_pole_winding`). It is a winding about a
/// point, which side of the loop the pole is on, and not a loop's turns
/// about a periodic direction: [`closure`]'s face-boundary facts have no
/// sphere analogue to check.
fn pole_side<T: Decide>(
    center: Point3<T>,
    radius: T,
    pole: Vec3<T>,
    lp: &SphereLoop<T>,
    band: Band,
) -> Result<Option<Sign>, PropsError> {
    let p = center + pole * radius;
    let mut circulations = Vec::with_capacity(lp.arcs.len());
    for arc in &lp.arcs {
        let (plane, on) = pole_against_arc(arc, p, band)?;
        if on {
            return Ok(None);
        }
        circulations.push(Some(circulation(arc, plane)));
    }
    let Some(turning) = pole_turning(center, pole, lp, &circulations, band)? else {
        unreachable!("every circulation is read, so the turning is")
    };
    Ok(
        match classify(
            "props_sphere_pole_winding",
            Margin::levered(turning, radius),
            band,
            PropsCheck::Inventory,
        )? {
            Sign::Positive => Some(Sign::Positive),
            Sign::Negative => Some(Sign::Negative),
            Sign::Zero => None,
        },
    )
}

/// An arc's image's circulation about the pole, `+1` or `−1`, from the
/// side of its carrier plane the pole lies on, `plane` ([`pole_side`]):
/// `−1` exactly where the cap left of the arc holds the pole.
fn circulation<T: Real>(arc: &SphereArc<T>, plane: Sign) -> T {
    // The cap left of a forward arc is the one its axis points into.
    let left_holds_pole = if arc.forward { plane } else { plane.flip() };
    if left_holds_pole == Sign::Positive {
        -T::one()
    } else {
        T::one()
    }
}

/// **The turning of a sphere loop's stereographic image from the pole
/// in direction `pole`** ([`pole_side`]'s sum), each arc's circulation
/// given: `None` for an arc whose circulation is not read, which the
/// sum does not need where the arc's image sweeps less than half a
/// turn (its tangent at the start leads its chord), since the sweep
/// `c·2·(π/2 + atan2(−d·chord, c·cross))` is then `2·atan(cross /
/// d·chord)` for either `c`. `Ok(None)` where such an arc's image
/// sweeps half a turn or more, or closes on itself.
///
/// A point's image divides by `1 − w·pole`, which for `w` near the pole
/// is the difference of two numbers near one; it is read as
/// `|w − pole|² / 2`, its value for unit vectors, `w` normalized, so a
/// loop passing near the pole keeps its image's digits.
///
/// # Errors
///
/// [`pole_arc_full`]'s escalation.
fn pole_turning<T: Decide>(
    center: Point3<T>,
    pole: Vec3<T>,
    lp: &SphereLoop<T>,
    circulations: &[Option<T>],
    band: Band,
) -> Result<Option<T>, PropsError> {
    let unit = |x: Point3<T>| {
        let w = x - center;
        w / w.norm()
    };
    let half = T::from_f64(0.5);
    let image = |x: Point3<T>| {
        let w = unit(x);
        (w - pole * w.dot(pole)) / ((w - pole).norm_squared() * half)
    };
    // The image of a tangent `t` at `x`, up to a positive factor.
    let push = |x: Point3<T>, t: Vec3<T>| {
        let w = unit(x);
        (t - pole * t.dot(pole)) * ((w - pole).norm_squared() * half)
            + (w - pole * w.dot(pole)) * t.dot(pole)
    };
    let mut turning = lp.corners;
    for (arc, c) in lp.arcs.iter().zip(circulations) {
        let full = pole_arc_full(arc.extent, arc.radius, band)?;
        let sweep = match (full, *c) {
            (true, Some(c)) => c * T::tau(),
            (true, None) => return Ok(None),
            (false, c) => {
                let d = push(arc.from, arc.depart);
                let chord = image(arc.at) - image(arc.from);
                let cross = -pole.dot(d.cross(chord));
                let lead = d.dot(chord);
                match c {
                    Some(c) => {
                        let half = T::pi() * T::from_f64(0.5) + (-lead).atan2(c * cross);
                        c * (half + half)
                    }
                    None => {
                        if !matches!(
                            decide(
                                "props_sphere_pole_arc_leads",
                                Margin::sign_of(lead),
                                Band::enclosure_sign()
                            ),
                            Ok(Sign::Positive)
                        ) {
                            return Ok(None);
                        }
                        let a = cross.atan2(lead);
                        a + a
                    }
                }
            }
        };
        turning = turning + sweep;
    }
    Ok(Some(turning))
}

/// What [`sphere_pole_left`] reads of one loop.
pub(crate) enum PoleLeft {
    /// Whether the loop's left side, under the chart normal, holds the
    /// pole.
    Read(bool),
    /// The loop is not one this closed form reads: an edge that is not
    /// a circle on the sphere, an open loop ([`sphere_loop`]'s premises,
    /// refused or in band), or a junction whose turn's sign is lost in
    /// the tangents' rounding ([`Cusp::Rounding`]).
    NoClosedForm,
    /// The reading does not clear its own rounding: a full arc whose
    /// carrier plane holds the pole within it, or a turning that is no
    /// whole turn.
    Straddle,
    /// A band decision of the reading is in band (a span within the band
    /// of a whole turn, [`pole_arc_full`]).
    Banded(Indeterminate),
}

/// **Whether the left side of one sphere loop holds the pole `p`**, for
/// a pole its caller has decided OFF the loop
/// past its band (`crate::chart_singularity`, on the pole's distance to
/// each vertex and edge): [`pole_side`]'s turning, with no on-loop test
/// and each arc's circulation read off the pole's height over the arc's
/// carrier plane as an exact order against that height's rounding
/// (`chart_singularity_pole_plane`), not against a band. A pole clear of
/// the arc that the band would place in its plane is a pole off a small
/// circle on its tangent side: which cap holds it is still the plane's
/// side, and a height lost in the rounding leaves the circulation unread,
/// which [`pole_turning`] needs only for an arc whose image sweeps half a
/// turn or more. A sharp junction is read as the corner it is, against
/// the tangents' rounding rather than the band ([`Cusp::Rounding`]): a
/// lens a band wide turns by nearly half a turn at each tip, and the
/// side its loop leaves a pole on is still its turning's. The turning, a
/// whole turn up to rounding, is read against half a turn: positive
/// leaves the pole right of the loop.
pub(crate) fn sphere_pole_left<T: Decide>(
    center: Point3<T>,
    radius: T,
    p: Point3<T>,
    edges: &[LoopEdge<T>],
    band: Band,
) -> PoleLeft {
    let Ok(lp) = sphere_loop(center, radius, edges, band, Cusp::Rounding) else {
        return PoleLeft::NoClosedForm;
    };
    let pole = (p - center) / radius;
    // The height's rounding: the difference's, at its coordinates'
    // magnitude, and the dot product's, at its own, twice over.
    let ulp = T::from_f64(2.0 * f64::EPSILON);
    let origin = Point3::new(T::zero(), T::zero(), T::zero());
    let circulations: Vec<Option<T>> = lp
        .arcs
        .iter()
        .map(|arc| {
            let w = p - arc.centre;
            let height = arc.axis.dot(w);
            let rounding = ulp
                * arc.axis.norm()
                * ((p - origin).norm()
                    + (arc.centre - origin).norm()
                    + T::from_f64(3.0) * w.norm());
            let clear = decide(
                "chart_singularity_pole_plane",
                Margin::sign_of(height.abs() - rounding),
                Band::enclosure_sign(),
            );
            let side = decide(
                "chart_singularity_pole_plane",
                Margin::sign_of(height),
                Band::enclosure_sign(),
            );
            match (clear, side) {
                (Ok(Sign::Positive), Ok(plane @ (Sign::Positive | Sign::Negative))) => {
                    Some(circulation(arc, plane))
                }
                _ => None,
            }
        })
        .collect();
    let turning = match pole_turning(center, pole, &lp, &circulations, band) {
        Ok(Some(turning)) => turning,
        Ok(None) => return PoleLeft::Straddle,
        Err(PropsError::Escalated { cause, .. }) => return PoleLeft::Banded(cause),
        Err(_) => return PoleLeft::NoClosedForm,
    };
    let whole = decide(
        "chart_singularity_pole_turning",
        Margin::sign_of(turning.abs() - T::pi()),
        Band::enclosure_sign(),
    );
    let sign = decide(
        "chart_singularity_pole_turning",
        Margin::sign_of(turning),
        Band::enclosure_sign(),
    );
    match (whole, sign) {
        (Ok(Sign::Positive), Ok(Sign::Positive)) => PoleLeft::Read(false),
        (Ok(Sign::Positive), Ok(Sign::Negative)) => PoleLeft::Read(true),
        _ => PoleLeft::Straddle,
    }
}

/// Whether a sphere arc of `extent` on a circle of `radius` is a whole
/// circle (`props_sphere_pole_arc_full`, the turn rule's whole turn).
fn pole_arc_full<T: Decide>(extent: Extent<T>, radius: T, band: Band) -> Result<bool, PropsError> {
    let rows = TurnRows::exact("props_sphere_pole_arc_full");
    Period::new(T::tau())
        .whole_turn(rows, extent, Exact::new(radius), band)
        .map_err(|undecided| PropsError::Escalated {
            cause: undecided.diag,
            check: PropsCheck::Inventory,
        })
}

/// Whether the point `p`, on `arc`'s carrier, lies within its stored
/// span, as a margin ([`span_membership`], levered by the circle
/// radius).
fn arc_point_margin<T: Decide>(arc: &SphereArc<T>, p: Point3<T>) -> Margin<T> {
    let w0 = arc.start - arc.centre;
    let rr = arc.radius.powi(2);
    let q = p - arc.centre;
    let (ps, pc) = (q.dot(w0) / rr, q.dot(arc.axis.cross(w0)) / rr);
    Margin::levered(span_membership(ps, pc, arc.extent.get()), arc.radius)
}

/// **The rim span door**: [`require_circle_span`] at a coaxial
/// circle's own radius `r_c`, as `props_rim_span_forward` /
/// `props_rim_span_winding`, refusing [`PropsError::NotIsoRectangle`].
/// Asked by [`Rim::decided`] at every rim mint, and by
/// [`require_cone_conic_spans`] wherever the cone reads a circle's span
/// without minting a rim.
fn require_rim_span<T: Decide>(
    e: &LoopEdge<T>,
    r_c: T,
    band: Band,
) -> Result<Extent<T>, PropsError> {
    require_circle_span(
        (e.t0, e.t1),
        r_c,
        band,
        ["props_rim_span_forward", "props_rim_span_winding"],
        |what| PropsError::NotIsoRectangle { what },
    )
}

/// **The meridian span door**: [`require_circle_span`] at the
/// meridian circle's radius — the sphere's `R`, the torus's minor
/// radius — as `props_meridian_span_forward` /
/// `props_meridian_span_winding`, refusing
/// [`PropsError::NotIsoRectangle`]. Asked by
/// [`sphere_meridian_pole_margins`] and by [`torus_classify`] on every
/// torus meridian edge.
fn require_meridian_span<T: Decide>(
    e: &LoopEdge<T>,
    radius: T,
    band: Band,
) -> Result<(), PropsError> {
    require_circle_span(
        (e.t0, e.t1),
        radius,
        band,
        ["props_meridian_span_forward", "props_meridian_span_winding"],
        |what| PropsError::NotIsoRectangle { what },
    )
    .map(|_| ())
}

/// [`require_span_within_period`] on a circle of radius `radius`, whose
/// speed is its radius everywhere: the forward half at it, the turn at
/// [`Exact`].
fn require_circle_span<T: Decide>(
    span: (T, T),
    radius: T,
    band: Band,
    [forward, winding]: [&'static str; 2],
    refuse: fn(&'static str) -> PropsError,
) -> Result<Extent<T>, PropsError> {
    require_span_within_period(
        span,
        InfSpeed::new(radius),
        |_| (Exact::new(radius), TurnRows::exact(winding)),
        forward,
        band,
        refuse,
    )
}

/// **Certification's per-edge span bounds, re-decided at the parse**:
/// the stored span is definitely forward and does not definitely
/// exceed one period, certification's check 2 (`interval_span_forward`,
/// then the turn rule, [`geom_core::period`]) at the linear band. Every
/// span read here rests on that bound; a loop built without a body
/// ([`LoopEdge::hand_built`]) can state any span, so both halves are
/// decided under the asker's own names with certification's rates and
/// band, hence its dispositions: the forward half admits only a
/// definite `Positive` span (certification's `IntervalNotForward`), the
/// turn is asked "not over" ([`Period::not_over`]), refusing an extent
/// over a turn; an undecided reading escalates. The rate is the asker's,
/// read off the decided extent (a bounded rate's sup is over the turn's
/// defect); the extent is returned for the asker that reads it again.
///
/// Each asker brings its own rates and names:
///
/// * a meridian, through [`require_meridian_span`];
/// * a rim of any kind, and every circle the cone reads, through
///   [`require_rim_span`];
/// * every ellipse arc the cone reads, forward at certification's speed
///   floor and turned between it and the sup over the turn's defect,
///   `props_cone_ellipse_span_forward` /
///   `props_cone_ellipse_span_winding` (straddle
///   `props_cone_ellipse_span_winding_straddle`), refusing as
///   [`PropsError::ConeLoop`] ([`require_cone_conic_spans`]);
/// * every arc of the sphere's flux lane ([`sphere_loop`]) at the arc's
///   own radius, `props_sphere_arc_span_forward` /
///   `props_sphere_arc_span_winding`, refusing as
///   [`PropsError::SphereLoop`].
///
/// The torus decides the same pair for a span it reconstructs across
/// pieces (`props_meridian_pieces_forward`,
/// `props_meridian_pieces_winding`, [`fold_chain`]).
fn require_span_within_period<T: Decide, R: TurnRate<T>>(
    (t0, t1): (T, T),
    forward_rate: InfSpeed<T>,
    rate: impl FnOnce(Extent<T>) -> (R, TurnRows<R::Straddle>),
    forward: &'static str,
    band: Band,
    refuse: fn(&'static str) -> PropsError,
) -> Result<Extent<T>, PropsError> {
    let escalated = |cause| PropsError::Escalated {
        cause,
        check: PropsCheck::Inventory,
    };
    let extent = Extent::forward(forward, t0, t1, forward_rate, band).map_err(|n| match n {
        NotForward::Undecided(cause) => escalated(cause),
        NotForward::Zero(_) | NotForward::Reversed(_) => refuse(forward),
    })?;
    let (rate, rows) = rate(extent);
    if Period::new(T::tau())
        .not_over(rows, extent, rate, band)
        .map_err(|undecided| escalated(undecided.diag))?
    {
        Ok(extent)
    } else {
        Err(refuse(rows.turn))
    }
}

/// The two poles' span-membership margins for a sphere meridian arc,
/// each with the latitude sine it would carry — **one home for the
/// test, two dispositions**.
///
/// [`sphere_meridian_span_levels`] FOLDS on this margin (everything
/// but a definite `Negative` pushes the pole's latitude into the
/// face's extent, so the closed form measures a pole-crossing arc
/// exactly); [`require_one_chart_branch`] REFUSES on it (a definite
/// `Positive` is a pole strictly inside the span, where the chart's
/// `u` jumps by π mid-edge). The two answers differ only on the
/// `Positive` side, which is exactly the arc that crosses; an arc
/// ENDING at a pole is `Zero` and both doors admit it. Both doors
/// decide the margin `Margin::levered(m, radius)` — metres — through
/// the funnel under the same name, because it is the same quantity.
///
/// Along the meridian the latitude sine is `λ(θ) = sa·cosθ + ca·sinθ`
/// for `θ` measured from `t0`, with `sa = λ(t0)` and `ca = dλ/dt(t0)`
/// read off stored data (`dP/dt = n_c × (P − center)` for the stored
/// circle parameterization). Over the full circle the extremes are
/// `±r0` with `r0 = √(sa² + ca²)` (= 1 up to the certified
/// `props_meridian_great` / `props_circle_axis_class` residuals),
/// attained at the two poles; the arc attains one exactly when that
/// pole's angular offset from `t0` lands inside the span.
///
/// The margin is `props_meridian_pole`: the chord from the pole's
/// span-relative direction to the nearer span endpoint, carrying the
/// membership sign, levered at the sphere radius — the point
/// deviation of moving the pole onto the span boundary.
///
/// **The span is forward and at most one period, decided here before
/// any margin is formed** ([`require_span_within_period`],
/// the reason this helper takes the band and can refuse). The
/// membership test below is `⟨P, M⟩ − cos(dt/2)`, whose zero set on
/// the parameter circle is exactly the two span endpoints for
/// `0 < dt ≤ 2π`; an admitted span exceeds `2π` by at most `zero/R`
/// radians — certification's own coincidence band. On such a span
/// `cos(dt/2)` is within `(zero/2R)²/2` of `−1`, and the only pole
/// directions that difference can reclassify lie within `zero/2R` of
/// the span's endpoint, whose chord is inside the band, where the
/// fold folds and the door admits whatever the sign. Pinned in
/// `tests/mesh12_saturated_span.rs`.
///
/// The pole is located relative to the STORED span, as directions —
/// no chart inversion at all, so this is not the wedge-unwrap trap
/// the module docs forbid (two endpoint inversions differenced,
/// which loses the winding); the interval stays the stored
/// `t1 − t0`.
fn sphere_meridian_pole_margins<T: Decide>(
    e: &LoopEdge<T>,
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    n_c: Vec3<T>,
    band: Band,
) -> Result<[(T, T); 2], PropsError> {
    require_meridian_span(e, radius, band)?;
    let w0 = e.p0() - center;
    let sa = w0.dot(axis) / radius;
    let ca = n_c.cross(w0).dot(axis) / radius;
    // powi(2), not x*x (the one argument lives at `level_gap`).
    let r0 = (sa.powi(2) + ca.powi(2)).sqrt();
    let dt = e.t1 - e.t0;
    // The north pole (λ = +r0) sits at the span-relative direction
    // `(sa, ca)` on the parameter circle; the south pole (λ = −r0)
    // at its antipode.
    Ok([(sa, ca, r0), (-sa, -ca, -r0)]
        .map(|(ps, pc, extreme)| (span_membership(ps, pc, dt), extreme)))
}

/// **Whether a point on a circle lies in an arc's span, as a signed
/// chord.** `(ps, pc)` is the point's direction on the unit circle
/// relative to the span's start (`ps` along the start, `pc` a quarter
/// turn on), and `dt` the span, which its caller has decided is one
/// certification produces, `0 < dt ≤ τ`.
///
/// Sign: the point lies in the closed span iff its direction is within
/// `dt/2` of the span's midpoint direction — one dot test,
/// `⟨P, M⟩ − cos(dt/2)`, whose zero set on the circle is exactly the two
/// span endpoints. Magnitude: the CHORD to the nearer span endpoint,
/// the point deviation of moving the point onto the span boundary once
/// levered by the radius. Direction arithmetic, no `atan2` and no range
/// reduction: an angle extraction is wide at its branch cut (an arc
/// anchored at a pole put an interval enclosure exactly there, live on
/// the die-fillet corpus) and a mod-2π `floor` spans its integer step
/// at a period boundary, either widening the margin to the whole period
/// and forcing an escalation the scalar lane does not have.
fn span_membership<T: Real>(ps: T, pc: T, dt: T) -> T {
    let (sd2, cd2) = (dt * T::from_f64(0.5)).sin_cos();
    let (sdt, cdt) = dt.sin_cos();
    let f = ps * cd2 + pc * sd2 - cd2;
    // powi(2), not x*x (see `level_gap`).
    let chord_a = ((ps - T::one()).powi(2) + pc.powi(2)).sqrt();
    let chord_b = ((ps - cdt).powi(2) + (pc - sdt).powi(2)).sqrt();
    // `copysign` transfers the membership sign onto the chord; at an
    // interval scalar a sign enclosure straddling zero yields the
    // two-sided hull `±chord`, which is tight exactly where it happens
    // — the point at a span endpoint, chord ≈ 0.
    chord_a.min(chord_b).copysign(f)
}

/// Push the latitude-sine extremes a sphere meridian arc attains over
/// its **stored parameter span** — the span-derived `v`-extent, the
/// torus's own derivation carried to the sphere. Endpoint latitudes
/// alone are not the arc's extent: a great circle contains both poles,
/// so an arc whose span crosses one reaches latitude ±1 in its
/// INTERIOR, where an endpoint fold never looks.
///
/// The membership test is [`sphere_meridian_pole_margins`]; what is
/// here is this lane's DISPOSITION of it. `Negative` = outside,
/// nothing to push; **everything else folds** — `Positive`, `Zero`,
/// and the indeterminate band alike. At or near a span end the
/// endpoint latitude already sits within band² of the pole's (the
/// latitude is quadratic at its extremum), so the fold choices agree
/// far inside any honest tolerance, the folded extent is continuous
/// across the decision, and an indeterminate margin carries no
/// information a refusal could honestly report. The margin still
/// records through the funnel like any decide.
fn sphere_meridian_span_levels<T: Decide>(
    e: &LoopEdge<T>,
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    n_c: Vec3<T>,
    (levels, cosines): (&mut Vec<T>, &mut Vec<T>),
    band: Band,
) -> Result<(), PropsError> {
    for (m, extreme) in sphere_meridian_pole_margins(e, center, radius, axis, n_c, band)? {
        // Decided through the funnel — the margin is RECORDED like
        // any other — but the indeterminate outcome FOLDS instead of
        // escalating. In-band, the pole sits within the band of a
        // span end, where the two fold choices differ by ~band²/2 in
        // latitude — sub-band in every downstream quantity — so an
        // indeterminate carries no information about the answer, and
        // refusing on it would refuse a solid whose area is not in
        // doubt (executed: a split vertex 1e-6 rad off the pole
        // flipped certify-exactly into an import refusal). A POISONED
        // margin lands in the same arm and folding stays loud: sa/ca
        // poison makes the folded `±r0` poison too, which the extent
        // and level decides downstream refuse typed; a poisoned span
        // is refused upstream by certification before this parse.
        match decide("props_meridian_pole", Margin::levered(m, radius), band) {
            Ok(Sign::Positive | Sign::Zero) | Err(_) => {
                levels.push(extreme);
                cosines.push(T::zero());
            }
            Ok(Sign::Negative) => {}
        }
    }
    Ok(())
}

/// Classify a sphere face's boundary into rims and latitude-sine levels
/// — [`require_iso_rectangle`]'s sphere parse, which `mesh`'s walk
/// cites. The flux lane and the side reader read circles by
/// Gauss–Bonnet instead ([`sphere_face`]) and do not parse.
fn sphere_boundary<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<LinearBoundary<T>, PropsError> {
    let mut rims: Vec<Rim<T>> = Vec::new();
    let (mut levels, mut cosines): (Vec<T>, Vec<T>) = (Vec::new(), Vec::new());
    // A point's latitude, its sine and cosine each off its own offset
    // from the centre: along the axis, and out from it.
    let latitude = |p: Point3<T>| {
        let w = p - center;
        let h = w.dot(axis);
        (h / radius, (w - axis * h).norm() / radius)
    };
    for e in edges {
        let Curve3::Circle {
            center: c_c,
            axis: n_c,
            radius: r_c,
            ..
        } = e.carrier
        else {
            return Err(match e.carrier {
                // A spiric lies on no sphere: refused beside the
                // spline, typed.
                Curve3::Nurbs(_) | Curve3::Spiric { .. } => PropsError::Unimplemented,
                _ => PropsError::NotIsoRectangle {
                    what: "sphere boundary edge is not a circle",
                },
            });
        };
        let s = classify(
            "props_circle_axis_class",
            Margin::levered(n_c.dot(axis), r_c),
            band,
            PropsCheck::Inventory,
        )?;
        match s {
            Sign::Positive | Sign::Negative => {
                let w = c_c - center;
                require_zero(
                    "props_rim_fit",
                    Margin::of((w.norm_squared() + r_c.powi(2)).sqrt() - radius),
                    band,
                    Premise::OnSurface,
                )?;
                // Incidence: the fit above only fixes ‖w‖; the offset
                // must also point ALONG the axis (w ∥ â) with the
                // carrier axis parallel — together they place the
                // circle on the sphere as the iso-v rim.
                require_rim_incidence(w, n_c, r_c, axis, band)?;
                let sin_v = w.dot(axis) / radius;
                let cos_v = r_c / radius;
                // Dimensionless latitude DIRECTION pair, both
                // components from stored data (`w·â/R`, `r_c/R`). The
                // chord between two of these is the geodesic separation
                // at R everywhere on the sphere; the axial component
                // alone shrinks by `cos v̄` toward the poles and merges
                // distinct near-polar rims.
                let level = RimLevel::Unit(sin_v, cos_v);
                rims.push(Rim::decided(e, rim_dir(s, e.forward), level, r_c, band)?);
                levels.push(sin_v);
                cosines.push(cos_v);
            }
            Sign::Zero => {
                // Meridian great circle: centered at the sphere center
                // with the sphere's radius.
                require_zero(
                    "props_meridian_great",
                    Margin::of((c_c - center).norm().max((r_c - radius).abs())),
                    band,
                    Premise::Inventory,
                )?;
                for p in [e.p0(), e.p1()] {
                    let (sin_p, cos_p) = latitude(p);
                    levels.push(sin_p);
                    cosines.push(cos_p);
                }
                // The arc's extent is its stored span's, not its
                // endpoints': fold in the pole latitude(s) the span
                // contains (see `sphere_meridian_span_levels`); the
                // pole helper decides first that the span is one the
                // fold may read.
                sphere_meridian_span_levels(
                    e,
                    center,
                    radius,
                    axis,
                    n_c,
                    (&mut levels, &mut cosines),
                    band,
                )?;
            }
        }
    }
    // Sphere levels are latitude SINES; the rims carry the full
    // `(sin v, cos v)` direction pair, so each level carries its cosine
    // beside it, read where it is found ([`LinearBoundary::ends`]).
    // Metered at the sphere radius, which is also its azimuthal arm.
    Ok(LinearBoundary {
        rims,
        levels,
        arms: RimArms::uniform(radius),
        cosines,
    })
}

// ---------------------------------------------------------------------
// Torus
// ---------------------------------------------------------------------

/// The torus's two lever arms. The ONE kind whose level and azimuth
/// turn about different radii, which is what [`RimArms`] exists for:
/// `minor` is the exact lever for a minor-circle direction pair,
/// `major` for the `Δu` angle and the ±1 traversal difference.
fn torus_arms<T: Real>(major: T, minor: T) -> RimArms<T> {
    RimArms::split(minor, major)
}

/// The face's two extreme minor angles as `(s0, c0, s1, c1)` — the
/// INCREASING interval `[v0, v1]`, from the anchor meridian's stored
/// span and its chart orientation, never from endpoint `atan2`.
///
/// The torus's `v` is periodic, so its extremes cannot come from
/// `min_max` over endpoint levels the way the linearly-leveled kinds'
/// do; the sphere's fold carries the same stored-span derivation per
/// meridian arc ([`sphere_meridian_span_levels`]).
fn torus_ends<T: Real>(
    m0: &TorusMeridian<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    orient: Sign,
) -> (T, T, T, T) {
    // Anchor latitude from the meridian's t0 endpoint.
    let wa = m0.anchor - center;
    let ha = wa.dot(axis);
    let rho_a = (wa - axis * ha).norm();
    let (sin_a, cos_a) = (ha / minor, (rho_a - major) / minor);
    // Rotate the anchor latitude by the signed span where needed.
    let (sd, cd) = m0.dt.sin_cos();
    match orient {
        // orient Negative ⇒ dv/dt = +1: anchor is v0.
        Sign::Negative => {
            let s1 = sin_a * cd + cos_a * sd;
            let c1 = cos_a * cd - sin_a * sd;
            (sin_a, cos_a, s1, c1)
        }
        // orient Positive ⇒ dv/dt = −1: anchor is v1.
        Sign::Positive => {
            let s0 = sin_a * cd - cos_a * sd;
            let c0 = cos_a * cd + sin_a * sd;
            (s0, c0, sin_a, cos_a)
        }
        Sign::Zero => unreachable_zero(),
    }
}

/// A torus face's chart Green form ([`torus_chart`]): its signed
/// flux and area integrals and its boundary length.
struct TorusChart<T> {
    /// `−Σ∮ F(v) du` with `F′ = r(R + r cos v)(R cos v + r)`: the flux
    /// of `(p − c)` through the face along the chart normal, in m³.
    flux: T,
    /// `−Σ∮ G(v) du` with `G′ = r(R + r cos v)`: the chart area, in
    /// m², positive exactly when the stored traversal winds the region
    /// about the chart normal.
    area: T,
    /// The boundary's length in metres.
    length: T,
}

/// One torus boundary edge on the loop's lift: a rim at lifted level
/// `v` turning `Δu`, or a meridian turning `Δv`.
#[derive(Clone, Copy)]
enum TorusStep<T> {
    Rim { sin_v: T, cos_v: T, du: T, rho: T },
    Meridian { dv: T },
}

/// One boundary edge as [`torus_chart`] walks it: its step, and what
/// [`fold_torus_pieces`] needs to rejoin the pieces of one split edge —
/// the edge, and the ±1 its stored span turns into its step.
struct TorusPiece<'a, T: Real> {
    edge: &'a LoopEdge<T>,
    step: TorusStep<T>,
    sign: T,
}

/// The loop's steps with the pieces of one split edge rejoined: loop-
/// adjacent pieces with one [`CarrierId`](super::CarrierId), one
/// traversal direction, and intervals that ABUT exactly (the split's
/// own `t`, decided at the exact-order band) are one step over the span
/// they assemble ([`loop_runs`]), so a split edge reads as the edge it
/// was, bit for bit. The rejoin is for that stability alone: the Green
/// form is linear in each step's `Δu` and `Δv` and takes no `sin`/`cos`
/// of a span, so pieces left apart sum to the same integral, and
/// nothing here is refused or re-decided.
fn fold_torus_pieces<T: Decide>(pieces: Vec<TorusPiece<'_, T>>, minor: T) -> Vec<TorusStep<T>> {
    let joins = |a: &TorusPiece<'_, T>, b: &TorusPiece<'_, T>| {
        let (ea, eb) = (a.edge, b.edge);
        let same = ea.forward == eb.forward
            && matches!((ea.carrier_id, eb.carrier_id), (Some(x), Some(y)) if x == y)
            && matches!(
                (&a.step, &b.step),
                (TorusStep::Rim { .. }, TorusStep::Rim { .. })
                    | (TorusStep::Meridian { .. }, TorusStep::Meridian { .. })
            );
        let gap = if ea.forward {
            eb.t0 - ea.t1
        } else {
            ea.t0 - eb.t1
        };
        same && matches!(
            decide(
                "props_torus_pieces_meet",
                Margin::levered(gap, minor),
                exact_band()
            ),
            Ok(Sign::Zero)
        )
    };
    let runs = match loop_runs(pieces, joins) {
        Ok(runs) => runs,
        Err(pieces) => return pieces.into_iter().map(|p| p.step).collect(),
    };
    runs.into_iter()
        .map(|run| {
            let (first, last) = (&run[0], &run[run.len() - 1]);
            if run.len() == 1 {
                return first.step;
            }
            let span = if first.edge.forward {
                last.edge.t1 - first.edge.t0
            } else {
                first.edge.t1 - last.edge.t0
            };
            match first.step {
                TorusStep::Rim {
                    sin_v, cos_v, rho, ..
                } => TorusStep::Rim {
                    sin_v,
                    cos_v,
                    du: first.sign * span,
                    rho,
                },
                TorusStep::Meridian { .. } => TorusStep::Meridian {
                    dv: first.sign * span,
                },
            }
        })
        .collect()
}

/// **The one home of a loop's run grouping.** `items` is a boundary
/// loop in traversal order; `joins(a, b)` says `b` continues `a` (the
/// next piece of one split edge). The loop is walked from an item no
/// run continues into, so no run is cut at the seam of the `Vec`, and
/// handed back as its maximal runs in loop order, each non-empty.
/// A loop every item of which continues the one before has no such
/// start: it comes back whole as the `Err`, for the caller to decide
/// what a loop of one run is.
fn loop_runs<I>(mut items: Vec<I>, joins: impl Fn(&I, &I) -> bool) -> Result<Vec<Vec<I>>, Vec<I>> {
    let n = items.len();
    let Some(start) = (0..n).find(|&i| !joins(&items[(i + n - 1) % n], &items[i])) else {
        return if n == 0 { Ok(Vec::new()) } else { Err(items) };
    };
    items.rotate_left(start);
    let mut runs: Vec<Vec<I>> = Vec::new();
    for item in items {
        match runs.last_mut() {
            Some(run) if run.last().is_some_and(|last| joins(last, &item)) => run.push(item),
            _ => runs.push(vec![item]),
        }
    }
    Ok(runs)
}

/// `atan2(y, x)` with its branch cut kept away from the arguments:
/// the principal branch for `x ≥ 0`, the `[0, 2π)` branch for `x < 0`,
/// so an interval enclosure never straddles the cut. Any branch serves
/// [`torus_chart`]: a lift moved by whole periods shifts `F` and `G`
/// by constants, which a loop with `Σ Δu = 0` sums to nothing.
fn torus_angle<T: Decide>(sin_v: T, cos_v: T, band: Band) -> T {
    match decide("props_torus_lift_branch", Margin::of(cos_v), band) {
        Ok(Sign::Negative) => (T::zero() - sin_v).atan2(T::zero() - cos_v) + T::pi(),
        Ok(Sign::Positive | Sign::Zero) | Err(_) => sin_v.atan2(cos_v),
    }
}

/// **The one home of a torus face's chart Green form.**
///
/// The torus's integrands depend on the tube angle `v` alone, by
/// symmetry about the axis: with `(p − c)·n_chart = R cos v + r` and
/// the area element `r(R + r cos v) du dv`, the flux is
/// `∬ F′(v) du dv` and the area `∬ G′(v) du dv`, so Green's theorem on
/// the chart gives `−∮ F(v) du` and `−∮ G(v) du` over the boundary.
/// Neither `F` nor `G` is periodic, so each loop is read on its
/// **lift**: walked from its first rim, a meridian moves the running
/// `v` by its own signed span (`dv/dt = −orient`,
/// [`torus_meridian_orient`]), and each rim is placed on the branch of
/// its level that the running `v` reaches.
/// A wrap edge's two halves land a period apart that way, as the loop
/// walk places them.
///
/// **Each loop must close and wind the torus not at all**
/// ([`torus_chart_loop`]): then it bounds a region of the lifted chart
/// and the sums are exact. Loops that do not close, or wind as no
/// boundary does, are defects ([`closure`]); a loop that winds the torus
/// as a boundary may (a seamless band's rim) has no region under it,
/// and which way round the torus the face runs between two winding
/// loops is not a fact about either loop, so it refuses as the
/// inventory's. A ring wound the same way as its face refuses
/// (`props_ring_winding`).
///
/// Each loop's sums are formed against the anchor `v_m` at the middle
/// of its rims' lifted levels, `−Σ (F(v) − F(v_m))·Δu`, equal to the
/// plain sum because `Σ Δu = 0`.
fn torus_chart<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<TorusChart<T>, PropsError> {
    let (big, r) = (major, minor);
    let half = T::from_f64(0.5);
    let f = |v: T| {
        let (sv, cv) = v.sin_cos();
        r * ((big.powi(2) + r.powi(2)) * sv + big * r * v + big * r * half * (v + sv * cv))
    };
    let g = |v: T| r * (big * v + r * v.sin_cos().0);
    let mut total = TorusChart {
        flux: T::zero(),
        area: T::zero(),
        length: T::zero(),
    };
    let mut per_loop: Vec<(T, T)> = Vec::with_capacity(loops.len());
    let mut loop_pieces: Vec<Vec<TorusPiece<T>>> = Vec::with_capacity(loops.len());
    closure::require_axis_inside(minor, band)?;
    let mut chart_loops: Vec<ChartLoop<T, 2>> = Vec::with_capacity(loops.len());
    for edges in loops {
        // Every edge is certified rim or meridian first, so a face this
        // inventory has no arm for refuses as itself.
        let mut pieces: Vec<TorusPiece<T>> = Vec::with_capacity(edges.len());
        let classified = torus_classify(center, axis, major, minor, edges, band)?;
        for (edge, e) in classified.into_iter().zip(edges.iter()) {
            let (step, sign) = match edge {
                TorusEdge::Rim(rim) => {
                    let RimLevel::Unit(sin_v, cos_v) = rim.level else {
                        return Err(PropsError::NotIsoRectangle {
                            what: "a torus rim carried a non-angle level",
                        });
                    };
                    let sign = t_sign::<T>(rim.d_u_sign);
                    let step = TorusStep::Rim {
                        sin_v,
                        cos_v,
                        du: sign * rim.dt.get(),
                        rho: big + r * cos_v,
                    };
                    (step, sign)
                }
                TorusEdge::Arc(arc) => {
                    let orient =
                        torus_meridian_orient((arc.n_c, arc.c_c), center, axis, minor, band)?;
                    let sign = t_sign::<T>(orient.flip())
                        * if arc.edge.forward {
                            T::one()
                        } else {
                            -T::one()
                        };
                    let step = TorusStep::Meridian {
                        dv: sign * (arc.edge.t1 - arc.edge.t0),
                    };
                    (step, sign)
                }
            };
            pieces.push(TorusPiece {
                edge: e,
                step,
                sign,
            });
        }
        chart_loops.push(torus_chart_loop(
            center, axis, major, minor, edges, &pieces, band,
        )?);
        loop_pieces.push(pieces);
    }
    // The torus's real question, which lift a band's second rim takes,
    // is one this form does not answer: a winding loop refuses here as
    // the inventory's, once the closure decision has passed it as a
    // boundary.
    let windings = closure::closed_form_windings(&chart_loops, band)?;
    if windings.iter().any(|&k| k != [0, 0]) {
        return Err(PropsError::NotIsoRectangle {
            what: "a torus face whose loops wind the torus (a seamless band): its chart \
                   Green form folds no winding loop",
        });
    }
    for pieces in loop_pieces {
        let mut steps = fold_torus_pieces(pieces, minor);
        let Some(first) = steps
            .iter()
            .position(|s| matches!(s, TorusStep::Rim { .. }))
        else {
            return Err(PropsError::NotIsoRectangle {
                what: "curved face without a rim (non-sphere)",
            });
        };
        steps.rotate_left(first);
        // The lift. Each rim's level is its own branch-canonical angle
        // (`torus_angle`, read off its carrier alone) plus the whole
        // number of turns the walk reached it on, so a rim's lifted
        // level depends on the loop's geometry and not on where the
        // walk began: the turns are counted from the loop's lowest.
        let mut rims: Vec<(T, i32, T)> = Vec::new();
        let mut running = None;
        let mut length = T::zero();
        for step in &steps {
            match *step {
                TorusStep::Rim {
                    sin_v,
                    cos_v,
                    du,
                    rho,
                } => {
                    let angle = torus_angle(sin_v, cos_v, band);
                    // The running `v` picks the branch, not the level.
                    let at = running.unwrap_or(angle);
                    let (turns, v) = torus_lift(at, angle, minor, band)?;
                    running = Some(v);
                    length = length + rho * du.abs();
                    rims.push((angle, turns, du));
                }
                TorusStep::Meridian { dv } => {
                    running = running.map(|at| at + dv);
                    length = length + r * dv.abs();
                }
            }
        }
        let base = rims.iter().map(|&(_, turns, _)| turns).min().unwrap_or(0);
        let lifted: Vec<(T, T)> = rims
            .iter()
            .map(|&(angle, turns, du)| (angle + Period::new(T::tau()).times(turns - base), du))
            .collect();
        let (lo, hi) = min_max(&lifted.iter().map(|&(v, _)| v).collect::<Vec<_>>())?;
        let anchor = lo + (hi - lo) * half;
        let (f_m, g_m) = (f(anchor), g(anchor));
        let (flux, area) = lifted
            .iter()
            .fold((T::zero(), T::zero()), |(fl, ar), &(v, du)| {
                (fl - (f(v) - f_m) * du, ar - (g(v) - g_m) * du)
            });
        per_loop.push((area, length));
        total.flux = total.flux + flux;
        total.area = total.area + area;
        total.length = total.length + length;
    }
    if per_loop.len() > 1 {
        require_holes_wound_against(
            "props_chart_area_side",
            (total.area, total.length),
            &per_loop[1..],
            band,
        )?;
    }
    Ok(total)
}

/// **A rim's tube angle lifted onto a torus loop's running one**: the
/// whole turns `k` the running angle `at` is from the rim's own `angle`
/// (`props_torus_lift_turns`), decided as structure — the branch of
/// `at − angle` through the half-turn marks, metered at the minor
/// radius ([`Period::decide`]), an integer and never a value's floor —
/// and the lifted level `angle + k·τ`, its shift a literal at every
/// scalar. Every junction of the loop is decided closed and every
/// edge's turn whole within its slack ([`closure::closed_form_windings`])
/// before the lift is read, so `at − angle` is a whole number of turns
/// to within `≈ Kε/minor`, half a turn clear of either mark, and the
/// marks decide; a gap on a mark is no rim's.
fn torus_lift<T: Decide>(at: T, angle: T, minor: T, band: Band) -> Result<(i32, T), PropsError> {
    let name = "props_torus_lift_turns";
    let period = Period::new(T::tau());
    period
        .decide(
            name,
            at - angle,
            |g| Margin::metered(g, InfSpeed::new(minor)),
            band,
        )
        .map(|k| (k, angle + period.times(k)))
        .map_err(|stop| match stop.miss {
            BranchMiss::Undecided(cause) => PropsError::Escalated {
                cause,
                check: PropsCheck::Inventory,
            },
            BranchMiss::OnMark => PropsError::NotIsoRectangle { what: name },
        })
}

/// One torus loop for the closure decision ([`closure::closed_form_windings`]):
/// the azimuth and the tube angle each piece turns between its ends,
/// and at each junction the turn about the axis from the edge before's
/// end to the edge after's start, metered at the after's distance from
/// the axis, beside the turn about the tube, metered at the minor
/// radius.
fn torus_chart_loop<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    edges: &[LoopEdge<T>],
    pieces: &[TorusPiece<T>],
    band: Band,
) -> Result<ChartLoop<T, 2>, PropsError> {
    // A point's place on the tube: its distance out from the core
    // circle and its height, the tube angle's cosine and sine arms.
    let tube = |w: Vec3<T>| {
        let h = w.dot(axis);
        ((w - axis * h).norm() - major, h)
    };
    // The tube angle's arms from `a` to `b`: their dot and cross.
    let tube_arms = |a: Vec3<T>, b: Vec3<T>| {
        let ((xa, ya), (xb, yb)) = (tube(a), tube(b));
        (xa * xb + ya * yb, xa * yb - ya * xb)
    };
    // The short turn on a pair of arms.
    let short = |(dot, cross): (T, T)| cross.atan2(dot);
    // How much farther round the tube a point sits about the core circle
    // than about a meridian's own centre: the turn, in the meridian's
    // plane, from `d` (the point less the meridian's centre) to `w` (the
    // point less the torus's centre), read on the tube's arms. The plane
    // is the meridian's own, out from the axis toward its centre `m`,
    // not the point's: a point near the axis reads its own out-direction
    // to `u·|w|/ρ`, which the core circle would carry `major` times over.
    let about_core = |m: Vec3<T>, d: Vec3<T>, w: Vec3<T>| {
        let radial = m - axis * m.dot(axis);
        let out = radial * (T::one() / radial.norm());
        let (xd, yd) = (d.dot(out), d.dot(axis));
        let (xw, yw) = tube(w);
        (xd * yw - yd * xw).atan2(xd * xw + yd * yw)
    };
    // Each piece's turn between its own ends: a parallel's stored azimuth
    // turn about its own centre carried to the axis at each end, and its
    // short tube turn; a meridian's stored tube turn about its own centre
    // carried to the core circle at each end, and its short azimuth turn.
    let turns = pieces.iter().map(|p| {
        let (a, b) = p.edge.traversal_ends();
        let Curve3::Circle {
            center: centre,
            radius: r_c,
            ..
        } = p.edge.carrier
        else {
            return Err(PropsError::NotIsoRectangle {
                what: "a torus piece that is no circle",
            });
        };
        let (wa, wb) = (a - center, b - center);
        let (azimuth, tube) = (turn_arms(axis, wa, wb), tube_arms(wa, wb));
        Ok(match p.step {
            TorusStep::Rim { du, .. } => {
                closure::require_axis_inside(r_c, band)?;
                [
                    EdgeTurn::new(
                        du + about_axis(axis, centre, center, b)
                            - about_axis(axis, centre, center, a),
                        azimuth,
                        r_c,
                    ),
                    EdgeTurn::new(short(tube), tube, minor),
                ]
            }
            TorusStep::Meridian { dv } => {
                let near = |w: Vec3<T>| (w - axis * w.dot(axis)).norm();
                [
                    EdgeTurn::new(short(azimuth), azimuth, near(wa).min(near(wb))),
                    EdgeTurn::new(
                        dv + about_core(centre - center, b - centre, wb)
                            - about_core(centre - center, a - centre, wa),
                        tube,
                        r_c,
                    ),
                ]
            }
        })
    });
    let edges_turns = turns.collect::<Result<Vec<_>, PropsError>>()?;
    let joints = edges
        .iter()
        .zip(edges.iter().cycle().skip(1))
        .map(|(e, next)| {
            let a = e.traversal_ends().1 - center;
            let b = next.traversal_ends().0 - center;
            Joint {
                turn: [
                    (turn_about(axis, a, b), (b - axis * b.dot(axis)).norm()),
                    (short(tube_arms(a, b)), minor),
                ],
                flat: T::zero(),
            }
        })
        .collect();
    Ok(ChartLoop {
        edges: edges_turns,
        joints,
    })
}

/// A torus face's flux and area over all its loops, from its chart
/// Green form ([`torus_chart`]): no sense bit read, the traversal
/// carries the side.
fn torus_face<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let chart = torus_chart(center, axis, major, minor, loops, band)?;
    let mut va = Vec3::new(T::zero(), T::zero(), T::zero());
    for edges in loops {
        va = va + loop_vector_area(edges, center)?;
    }
    Ok(FaceContribution {
        flux: chart.flux + (center - Point3::origin()).dot(va),
        area: chart.area.abs(),
    })
}

/// The material side a torus face's loops encode: the sign of its
/// chart area ([`torus_chart`]), metered as the mean width `2·A/P`.
fn torus_material_sign<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    loops: &[&[LoopEdge<T>]],
    band: Band,
) -> Result<MaterialSign, PropsError> {
    let chart = torus_chart(center, axis, major, minor, loops, band)?;
    chart_area_side(chart.area, chart.length, band)
}

/// A torus minor-circle boundary meridian (iso-`u`) as the parse
/// consumes it: its carrier frame, the parameter span the meridian
/// covers on that carrier, and the anchor at the interval's `t0` end.
/// One meridian is carried by one edge, or by the pieces of one split
/// edge folded back into it ([`fold_torus_meridians`]).
struct TorusMeridian<T: Real> {
    n_c: Vec3<T>,
    c_c: Point3<T>,
    dt: T,
    anchor: Point3<T>,
}

/// One meridian ARC as one boundary edge carries it — the fold's
/// input: the edge itself, plus the two facts about its carrier the
/// classification certified (the minor circle's axis and centre).
struct TorusArc<'a, T: Real> {
    edge: &'a LoopEdge<T>,
    n_c: Vec3<T>,
    c_c: Point3<T>,
}

/// A torus boundary edge classified, in loop order, before the fold.
enum TorusEdge<'a, T: Real> {
    Rim(Rim<T>),
    Arc(TorusArc<'a, T>),
}

/// A torus face's boundary as the consumers read it: its rims, and
/// its meridians after the fold.
type TorusParts<T> = (Vec<Rim<T>>, Vec<TorusMeridian<T>>);

/// Classify a torus face's boundary into (rims, meridians) — the
/// shared parse consumed by the flux closed form,
/// [`boundary_material_sign`] and the shape door. Every edge is
/// certified rim-or-meridian in loop order first; the arcs that carry
/// one meridian are then folded into it.
fn torus_boundary<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<TorusParts<T>, PropsError> {
    fold_torus_meridians(
        torus_classify(center, axis, major, minor, edges, band)?,
        minor,
        band,
    )
}

/// Certify every edge of a torus face's boundary as a rim or a meridian
/// arc, in loop order.
fn torus_classify<'a, T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    edges: &'a [LoopEdge<T>],
    band: Band,
) -> Result<Vec<TorusEdge<'a, T>>, PropsError> {
    let mut classified: Vec<TorusEdge<T>> = Vec::with_capacity(edges.len());
    for e in edges {
        let Curve3::Circle {
            center: c_c,
            axis: n_c,
            radius: r_c,
            ..
        } = e.carrier
        else {
            return Err(match e.carrier {
                Curve3::Nurbs(_) => PropsError::Unimplemented,
                // The spiric rim of a partial revolve's hollowed torus
                // wall — a boundary edge that IS on the torus but is
                // neither rim nor meridian, so this parse has no arm
                // for it and says which edge kind it met. Named rather
                // than folded into the wildcard so a reader knows the
                // kind was considered. Reached through the closed-form
                // door (`curved_face`) directly; `topo`'s own face flux
                // never gets here for a spiric-bounded face — a loop
                // carrying one routes to the quadrature lane, whose
                // spiric arm measures it (`super::spiric`).
                Curve3::Spiric { .. } => PropsError::NotIsoRectangle {
                    what: "torus boundary edge is not a circle",
                },
                _ => PropsError::NotIsoRectangle {
                    what: "torus boundary edge is not a circle",
                },
            });
        };
        let s = classify(
            "props_circle_axis_class",
            Margin::levered(n_c.dot(axis), r_c),
            band,
            PropsCheck::Inventory,
        )?;
        match s {
            Sign::Positive | Sign::Negative => {
                let h = (c_c - center).dot(axis);
                let sin_v = h / minor;
                let cos_v = (r_c - major) / minor;
                require_zero(
                    "props_rim_fit",
                    Margin::levered((sin_v.powi(2) + cos_v.powi(2)).sqrt() - T::one(), minor),
                    band,
                    Premise::Inventory,
                )?;
                require_rim_incidence(c_c - center, n_c, r_c, axis, band)?;
                // Dimensionless minor-angle direction pair.
                let level = RimLevel::Unit(sin_v, cos_v);
                classified.push(TorusEdge::Rim(Rim::decided(
                    e,
                    rim_dir(s, e.forward),
                    level,
                    r_c,
                    band,
                )?));
            }
            Sign::Zero => {
                let w = c_c - center;
                let h = w.dot(axis);
                let rho = (w - axis * h).norm();
                require_zero(
                    "props_meridian_fit",
                    Margin::of((rho - major).abs().max(h.abs()).max((r_c - minor).abs())),
                    band,
                    Premise::Inventory,
                )?;
                // Incidence: the minor circle's plane must CONTAIN the
                // torus axis direction — its normal `n_c` has no radial
                // component (the definite `n_c·τ̂` orientation check
                // below only excludes n_c ⊥ τ̂). Margin
                // `n_c·(w − âh) = (n_c·ρ̂)·ρ`: the tilt metered at the
                // tube-center distance (lever arm ρ ≈ R, meters).
                require_zero(
                    "props_meridian_plane",
                    Margin::of(n_c.dot(w - axis * h)),
                    band,
                    Premise::Inventory,
                )?;
                // Certification's per-edge bound, at the minor radius:
                // the chart's lift and the meridian fold read this span.
                require_meridian_span(e, minor, band)?;
                classified.push(TorusEdge::Arc(TorusArc { edge: e, n_c, c_c }));
            }
        }
    }
    Ok(classified)
}

/// Fold the arcs that carry ONE meridian into it; rims pass through
/// in loop order.
///
/// Two loop-adjacent arcs are pieces of one meridian iff they carry
/// equal [`CarrierId`](super::CarrierId)s — pieces of one original edge, whose split
/// children keep its carrier and partition its interval — and are
/// traversed the same way. Identity is the whole membership test:
/// `None` matches nothing, and two arcs from distinct edges stay two
/// meridians however their stored circles compare as values (two
/// carriers meeting at a vertex are a corner, never a subdivision, and
/// the door then refuses the far rim as it always did). A meridian an
/// importer states as several edges on one curve entity carries no
/// split lineage and does not fold here.
///
/// **What the identity asserts is then enforced, not assumed**
/// ([`fold_chain`]): the pieces must MEET — adjacent pieces' intervals
/// abut exactly, `a.t1 == b.t0` in the traversal direction, which is a
/// structural fact of the split (one `t` is both children's boundary)
/// decided at the exact-order band rather than inferred at ε — and the
/// interval they assemble must be one certification could have
/// admitted, since this parse reads the span through `sin`/`cos`. A
/// loop of arcs that all continue one another (one closed minor circle
/// in pieces, no rim) has no run boundary ([`loop_runs`]) and refuses
/// by one name whatever rotation it arrives in.
///
/// The folded interval is `[lowest t0, highest t1]` over the chain —
/// on one parametrisation, the original edge's own stored interval,
/// bitwise, whatever the split fractions — and the anchor is the arc
/// at its `t0` end, so a meridian carried by one edge folds to exactly
/// the record that edge produces alone.
fn fold_torus_meridians<T: Decide>(
    edges: Vec<TorusEdge<'_, T>>,
    minor: T,
    band: Band,
) -> Result<TorusParts<T>, PropsError> {
    fn same_edge<T: Real>(a: &TorusArc<'_, T>, b: &TorusArc<'_, T>) -> bool {
        a.edge.forward == b.edge.forward
            && matches!((a.edge.carrier_id, b.edge.carrier_id), (Some(x), Some(y)) if x == y)
    }
    let joins = |a: &TorusEdge<'_, T>, b: &TorusEdge<'_, T>| match (a, b) {
        (TorusEdge::Arc(a), TorusEdge::Arc(b)) => same_edge(a, b),
        _ => false,
    };
    let Ok(runs) = loop_runs(edges, joins) else {
        return Err(PropsError::NotIsoRectangle {
            what: "torus meridian pieces close a loop with no rim",
        });
    };
    let mut rims = Vec::new();
    let mut meridians = Vec::new();
    for run in runs {
        let mut arcs = Vec::with_capacity(run.len());
        for e in run {
            match e {
                TorusEdge::Rim(r) => rims.push(r),
                TorusEdge::Arc(a) => arcs.push(a),
            }
        }
        if !arcs.is_empty() {
            meridians.push(fold_chain(arcs, minor, band)?);
        }
    }
    Ok((rims, meridians))
}

/// The exact-order band: the open interior `(min-subnormal,
/// 2·min-subnormal)` contains no representable `f64`, so a decision
/// against it is exact and total at `f64` — `Zero` means bit-level
/// coincidence — and at the interval scalar an enclosure straddling
/// the hairline escalates honestly. Profile's canonical-form band and
/// the split join's ordering decide against the same constants.
fn exact_band() -> Band {
    Band::least()
}

/// One chain of arcs — non-empty, loop-consecutive, one identity, one
/// traversal direction — as the meridian they carry. Traversal runs up
/// the parametrisation on a forward chain and down it on a reversed
/// one, so the `t0` end is the first arc or the last.
///
/// **Spans across edges are decided, never clamped — the class
/// statement, at its one home.** Certification bounds every EDGE's
/// stored span (`interval_span_forward`, `interval_span_winding`:
/// `0 < Δt ≤ τ`), and every per-edge span read in this module rests
/// on that bound — the sphere arm re-decides both halves per meridian
/// arc inside its pole helper ([`require_span_within_period`]),
/// because a hand-built loop can state a span no edge certifies. A
/// span this fold reconstructs
/// ACROSS edges was certified by nobody: a public door
/// (`set_edge_curve`) can restate one piece's interval on its own
/// carrier — shifted by a period, the identical arc, every piece
/// certifying — and the assembled interval then spans more than a
/// period, which `sin`/`cos` would silently fold back onto the
/// extremes and every consumer would answer for, twice over. So a
/// chain of two or more pieces re-decides three things, and refuses
/// typed on any of them, naming the decide — the pieces do not
/// partition one certified interval:
///
/// * `props_meridian_pieces_meet` — adjacent intervals abut exactly
///   (the exact-order band; a sub-ε shift is still not the split's
///   own `t`, and is refused, not absorbed);
/// * `props_meridian_pieces_forward` — the assembled span is definitely
///   positive, as certification requires of one edge;
/// * `props_meridian_pieces_winding` — it does not definitely exceed a
///   period, certification's winding bound, at the same band and lever.
///
/// A single edge's span — and each piece's — was decided at
/// [`torus_classify`] against certification's own per-edge bound
/// ([`require_span_within_period`]), under the sphere doors' names for
/// that question (`props_meridian_span_forward`,
/// `props_meridian_span_winding`) at the minor radius, so a chain of
/// one is already one certified interval.
fn fold_chain<T: Decide>(
    chain: Vec<TorusArc<'_, T>>,
    minor: T,
    band: Band,
) -> Result<TorusMeridian<T>, PropsError> {
    let (Some(first), Some(last)) = (chain.first(), chain.last()) else {
        unreachable!("a torus meridian chain is folded only when non-empty")
    };
    let forward = first.edge.forward;
    let pieces = chain.len() > 1;
    if pieces {
        let exact = exact_band();
        for pair in chain.windows(2) {
            let (a, b) = (pair[0].edge, pair[1].edge);
            let gap = if forward { b.t0 - a.t1 } else { a.t0 - b.t1 };
            if classify(
                "props_meridian_pieces_meet",
                Margin::levered(gap, minor),
                exact,
                PropsCheck::Inventory,
            )? != Sign::Zero
            {
                return Err(PropsError::NotIsoRectangle {
                    what: "props_meridian_pieces_meet",
                });
            }
        }
    }
    let (lo, hi) = if forward {
        (first, last)
    } else {
        (last, first)
    };
    let dt = hi.edge.t1 - lo.edge.t0;
    if pieces {
        require_circle_span(
            (lo.edge.t0, hi.edge.t1),
            minor,
            band,
            [
                "props_meridian_pieces_forward",
                "props_meridian_pieces_winding",
            ],
            |what| PropsError::NotIsoRectangle { what },
        )?;
    }
    Ok(TorusMeridian {
        n_c: lo.n_c,
        c_c: lo.c_c,
        dt,
        anchor: lo.edge.p0(),
    })
}

/// Chart orientation of the anchor meridian: `v` winds right-handed
/// about `−τ̂` at its minor center, so `dv/dt = −orient`. Definite by
/// construction (the `Zero` arm refuses typed).
///
/// `τ̂ = â × ρ̂` is the frame [`RigidMotion::from_aim_and_reference`]
/// mints about the torus axis with the meridian centre's offset as the
/// reference, so both lengths are decided before the orientation is:
/// the axis under `props_torus_axis`, the radial under
/// `props_meridian_radial`. A meridian centred on the axis has no
/// radial, and refuses as that.
fn torus_meridian_orient<T: Decide>(
    (n_c, c_c): (Vec3<T>, Point3<T>),
    center: Point3<T>,
    axis: Vec3<T>,
    minor: T,
    band: Band,
) -> Result<Sign, PropsError> {
    // The axis is a pure number, levered by the anchor meridian's reach
    // from the torus centre, where the frame it aims is consumed.
    let reach = (c_c - center).norm() + minor;
    let aim = UnitVec3::levered(axis, "props_torus_axis", band, reach).map_err(|e| match e {
        LeveredUnitError::Direction(e) => {
            torus_frame_refused(e, "props_torus_axis", "torus axis length not measurable")
        }
        LeveredUnitError::Arm(Some(cause)) => PropsError::Escalated {
            cause,
            check: PropsCheck::Inventory,
        },
        LeveredUnitError::Arm(None) => PropsError::NotIsoRectangle {
            what: UNIT_DIRECTION_ARM,
        },
    })?;
    let frame = RigidMotion::from_aim_and_reference(
        center,
        aim,
        c_c - center,
        "props_meridian_radial",
        band,
    )
    .map_err(|e| {
        torus_frame_refused(
            e.error,
            "props_meridian_radial",
            "torus meridian radial length not measurable",
        )
    })?;
    let tau = frame.v().get();
    let orient = classify(
        "props_meridian_orient",
        Margin::levered(n_c.dot(tau), minor),
        band,
        PropsCheck::Inventory,
    )?;
    if orient == Sign::Zero {
        return Err(PropsError::NotIsoRectangle {
            what: "torus meridian orientation degenerate",
        });
    }
    Ok(orient)
}

/// A torus frame leg's refusal under [`PropsError::NotIsoRectangle`]'s
/// convention: a length the funnel decided to zero is named by the
/// predicate that decided it (`decided`); a length refused before any
/// margin was decided — not a finite number, or underflowed out of the
/// format — gets the structural sentence (`unmeasured`).
fn torus_frame_refused(
    e: UnitVec3Error,
    decided: &'static str,
    unmeasured: &'static str,
) -> PropsError {
    match e {
        UnitVec3Error::Escalated(cause) => PropsError::Escalated {
            cause,
            check: PropsCheck::Inventory,
        },
        UnitVec3Error::Degenerate => PropsError::NotIsoRectangle { what: decided },
        UnitVec3Error::NonFiniteLength | UnitVec3Error::UnderflowedLength => {
            PropsError::NotIsoRectangle { what: unmeasured }
        }
    }
}

/// Documented-unreachable arm (the caller matched a definite sign);
/// returns poison values rather than panicking (D9).
fn unreachable_zero<T: Real>() -> (T, T, T, T) {
    let nan = T::from_f64(f64::NAN);
    (nan, nan, nan, nan)
}

// ADVERSARIAL REVIEW PROBE (authored on branch review/rim-dim,
// adopted by merge — authorship kept).
#[cfg(test)]
#[allow(clippy::expect_used)]
mod rim_level_review_probe {
    use super::*;
    use geom_core::Tol;

    /// The structurally-impossible mixed-kind arm must refuse typed,
    /// never panic and never answer false.
    #[test]
    fn mixed_kind_levels_refuse_typed() {
        let band = Band::linear(Tol::witness()).expect("band");
        let unit = RimLevel::Unit(0.5, 0.5);
        let got = level_coincides(
            RimLevel::Length(1.0_f64),
            (unit, unit),
            RimArms::uniform(1.0),
            band,
        );
        assert!(got.is_err(), "mixed kinds must refuse typed: {got:?}");
    }

    /// **One rule, so one fail direction** (#714's review asked for the
    /// two to point the same way; S81 gave them one home). Both call
    /// sites of [`level_coincides`] refuse a mixed representation, and
    /// the predicate's own entry is pinned here in both orders:
    /// whatever the poisoned classify does, a mixed-representation face
    /// never measures and never groups.
    #[test]
    fn mixed_representation_rim_and_ends_never_measure() {
        let band = Band::linear(Tol::witness()).expect("band");
        let rim = Rim {
            d_u_sign: Sign::Positive,
            dt: Extent::between(0.0, 1.0),
            level: RimLevel::Unit(0.5, 0.5),
        };
        // Rim is `Unit`, the ends are `Length`.
        let got = require_rims_at_extremes(
            std::slice::from_ref(&rim),
            (RimLevel::Length(0.0), RimLevel::Length(1.0)),
            RimArms::uniform(1.0),
            band,
        );
        assert!(got.is_err(), "mixed rim/ends must not measure: {got:?}");

        // And the other way round: `Length` rim, `Unit` ends.
        let rim = Rim {
            level: RimLevel::Length(0.5),
            ..rim
        };
        let got = require_rims_at_extremes(
            std::slice::from_ref(&rim),
            (RimLevel::Unit(0.0, 1.0), RimLevel::Unit(1.0, 0.0)),
            RimArms::uniform(1.0),
            band,
        );
        assert!(got.is_err(), "mixed ends/rim must not measure: {got:?}");
    }

    /// **The metric is the chord** (S81). A `Unit` rim level whose two
    /// components are each inside the coincidence band of an extreme but
    /// whose CHORD is not is not at that extreme.
    ///
    /// **The chord can only ever be `√2` × a component**, and the
    /// escalation multiplier K is larger than that on any sane run, so
    /// the honest outcome here is a **refusal, not a definite
    /// disagreement**: the pair lands in the ambiguity band and
    /// `classify` escalates. Both are errors, and that — *the component
    /// rule's answer is not this rule's answer* — is what the row
    /// asserts, so it holds at every ε and every K rather than at the
    /// one it was first written against.
    ///
    /// The offsets come from the run's own band, never from a literal.
    /// A literal here passed at ε = 1e-9 for the wrong reason (both
    /// components were outside the band too) and failed at ε = 1e-6.
    #[test]
    fn a_pair_the_component_rule_calls_one_level_is_not_one_level() {
        let band = Band::linear(Tol::witness()).expect("band");
        // Each component 0.8·zero (inside), chord 1.13·zero (outside).
        let d = band.zero() * 0.8;
        let rim = Rim {
            d_u_sign: Sign::Positive,
            dt: Extent::between(0.0, 1.0),
            level: RimLevel::Unit(d, d),
        };
        let got = require_rims_at_extremes(
            std::slice::from_ref(&rim),
            (RimLevel::Unit(0.0, 0.0), RimLevel::Unit(1.0, 0.0)),
            RimArms::uniform(1.0_f64),
            band,
        );
        assert!(
            got.is_err(),
            "components inside the band but the chord outside it is not one level: {got:?}"
        );
    }

    /// The floor for the row above: a pair whose CHORD is inside the
    /// band **is** one level, so "the chord rule" is not "refuse
    /// everything". `0.1·zero` per component puts the chord at
    /// `0.14·zero`, inside at every ε and every K.
    #[test]
    fn a_pair_inside_the_band_by_its_chord_is_one_level() {
        let band = Band::linear(Tol::witness()).expect("band");
        let d = band.zero() * 0.1;
        assert!(
            level_coincides(
                RimLevel::Unit(0.0, 0.0),
                (RimLevel::Unit(d, d), RimLevel::Unit(d, d)),
                RimArms::uniform(1.0_f64),
                band,
            )
            .expect("decides"),
            "a pair inside the band by its chord is one level"
        );
    }
}

/// **A torus rim's lift is its whole turns as a theorem over a box**
/// ([`torus_lift`]): at `Sym<Interval>`, a running angle `k` turns from
/// the rim's, both polynomials in a parameter over `[0.2, 0.4]` that are
/// not constants (`a = p² + p + k·τ`, `b = p·(p + 1)`, whose value
/// channels' difference is `≈ 0.64` rad wide with the dependency lost),
/// at `k ∈ {−1, 0, 1, 2}`: the lift decides `k`, and the lifted level
/// less `b + k·τ` is the zero polynomial, decided Zero by the symbolic
/// tier. A branch read off a floor carries an atom the tier cannot
/// fold, and a branch held constant misses every `k` but one.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod torus_lift_rows {
    use super::*;
    use geom_core::{Interval, ParamSymbol, Sym, SymBudget, Tol};

    #[test]
    fn a_rims_lift_is_its_whole_turns_as_a_theorem_over_a_box() {
        let band = Band::linear(Tol::witness()).expect("the witness band");
        let budget = SymBudget {
            max_terms: 64,
            max_degree: 8,
        };
        for k in [-1, 0, 1, 2] {
            let (out, counts) = geom_core::sym::with_session(budget, || {
                let p = Sym::param_over(
                    ParamSymbol::new(test_utils::symbol_id("torus_lift_box")),
                    Interval::from_bounds(0.2, 0.4),
                    0.2,
                    0.4,
                );
                let a = p * p + p + Sym::from_f64(f64::from(k)) * Sym::tau();
                let b = p * (p + Sym::<Interval>::one());
                let (turns, level) = torus_lift(a, b, Sym::from_f64(0.5), band)
                    .unwrap_or_else(|e| panic!("k = {k}: {e:?}"));
                assert_eq!(turns, k, "k = {k}: the lift's whole turns");
                decide(
                    "torus_lift_row",
                    Margin::of(level - (b + Sym::from_f64(f64::from(k)) * Sym::tau())),
                    band,
                )
            });
            assert!(
                matches!(out, Ok(Sign::Zero)),
                "k = {k}: the lifted level is the rim's plus {k} turns: {out:?}"
            );
            assert_eq!(
                counts.symbolic, 1,
                "k = {k}: the level less the rim's plus kτ is the zero polynomial"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod sphere_region_cover_rows {
    use super::*;
    use geom_core::Tol;

    /// The unit sphere about the origin, poled along `z`.
    fn sphere() -> Surface<f64> {
        Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// The circle of latitude at height `z` on a sphere of radius `r`
    /// about the origin, as one closed edge run counter-clockwise about
    /// `z` (`up`: the cap above it lies to its left) or clockwise.
    fn latitude(r: f64, z: f64, up: bool) -> LoopEdge<f64> {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, z),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: (r * r - z * z).sqrt(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            0.0,
            f64::tau(),
            up,
            0,
            0,
        )
    }

    /// The census of a region of the sphere of radius `r` at ε = `eps`:
    /// `loops` are one-edge loops, `boundary` indexes them, and each
    /// member is a bit and the loops it holds.
    fn census(
        r: f64,
        eps: f64,
        loops: &[LoopEdge<f64>],
        boundary: &[usize],
        members: &[(bool, &[usize])],
    ) -> Result<SeamCover, PropsError> {
        let band = Band::linear_at(Tol::witness(), eps).expect("band");
        let surface = match sphere() {
            Surface::Sphere {
                center,
                axis,
                u_ref,
                ..
            } => Surface::Sphere {
                center,
                radius: r,
                axis,
                u_ref,
            },
            other => other,
        };
        let one = |i: &usize| core::slice::from_ref(&loops[*i]);
        let boundary: Vec<_> = boundary.iter().map(one).collect();
        let mut held = Vec::new();
        let mut claims = Vec::new();
        for (sense, of) in members {
            let first = held.len();
            held.extend(of.iter().map(one));
            claims.push((*sense, first..held.len()));
        }
        let region = seam_loops(&surface, &boundary, &held, band)?;
        sphere_region_cover(&region, &claims, band)
    }

    /// The equator, north- and south-facing, and two latitudes at
    /// `z = ±½`, each both ways.
    fn circles(r: f64) -> Vec<LoopEdge<f64>> {
        vec![
            latitude(r, 0.0, true),
            latitude(r, 0.0, false),
            latitude(r, 0.5 * r, true),
            latitude(r, 0.5 * r, false),
            latitude(r, -0.5 * r, true),
            latitude(r, -0.5 * r, false),
        ]
    }
    const NORTH: usize = 0;
    const SOUTH: usize = 1;
    const UP_HIGH: usize = 2;
    const DOWN_HIGH: usize = 3;
    const UP_LOW: usize = 4;
    const DOWN_LOW: usize = 5;

    /// Two hemispheres close the sphere once under either common bit:
    /// both flipped, `Σ χ_f − χ_R = 1 + 1 − 2 = 0`, an inside-out ball of
    /// two faces that the volume checks read. Four close it twice.
    #[test]
    fn hemispheres_cover_the_sphere_once_and_four_cover_it_twice() {
        let c = circles(1.0);
        for sense in [true, false] {
            let two = [(sense, &[NORTH][..]), (sense, &[SOUTH][..])];
            assert_eq!(census(1.0, 1e-9, &c, &[], &two), Ok(SeamCover::Once));
        }
        let n: &[usize] = &[NORTH];
        let s: &[usize] = &[SOUTH];
        let four = [(true, n), (true, s), (true, n), (true, s)];
        assert_eq!(
            census(1.0, 1e-9, &c, &[], &four),
            Ok(SeamCover::Over { times: 2 })
        );
    }

    /// A member of two loops: the band between `z = ±½` and its two caps
    /// close the sphere once under either common bit, as the count says
    /// (`Σ χ_f − χ_R = 1 + 0 + 1 − 2 = 0`); claimed twice over, they
    /// cover it twice. The band alone, bounded by its own two
    /// loops, reads once under either bit.
    #[test]
    fn a_band_of_two_loops_and_its_caps_cover_the_sphere_once() {
        let c = circles(1.0);
        let band: &[usize] = &[DOWN_HIGH, UP_LOW];
        let caps: [&[usize]; 2] = [&[UP_HIGH], &[DOWN_LOW]];
        for sense in [true, false] {
            let three = [(sense, caps[0]), (sense, band), (sense, caps[1])];
            assert_eq!(census(1.0, 1e-9, &c, &[], &three), Ok(SeamCover::Once));
            assert_eq!(
                census(1.0, 1e-9, &c, &[DOWN_HIGH, UP_LOW], &[(sense, band)]),
                Ok(SeamCover::Once)
            );
        }
        let twice = [
            (true, caps[0]),
            (true, band),
            (true, caps[1]),
            (true, caps[0]),
            (true, band),
            (true, caps[1]),
        ];
        assert_eq!(
            census(1.0, 1e-9, &c, &[], &twice),
            Ok(SeamCover::Over { times: 2 })
        );
    }

    /// A lone hemisphere covers the region its boundary bounds once
    /// under either bit: its flip is the complement, another valid body.
    #[test]
    fn a_lone_face_reads_once_under_either_bit() {
        let c = circles(1.0);
        for sense in [true, false] {
            assert_eq!(
                census(1.0, 1e-9, &c, &[NORTH], &[(sense, &[NORTH][..])]),
                Ok(SeamCover::Once)
            );
        }
    }

    /// Members whose bits disagree are no cover: check 4's lamina.
    #[test]
    fn members_whose_bits_disagree_are_not_read() {
        let c = circles(1.0);
        let mixed = [(true, &[NORTH][..]), (false, &[SOUTH][..])];
        assert_eq!(
            census(1.0, 1e-9, &c, &[], &mixed),
            Ok(SeamCover::MixedSense)
        );
    }

    /// A sphere of radius `1e-13` (`2πR ≈ 6.3e-13`) at ε = 1e-12 refuses
    /// at its loops' span reads, typed, before the census
    /// ([`sphere_region_cover`]'s slack), which check 6 leaves to the
    /// flux lane's same refusal; at ε = 1e-15 the same sphere reads.
    #[test]
    fn a_sphere_below_the_bands_slack_refuses_at_its_loops() {
        let r = 1e-13;
        let c = circles(r);
        let two = [(true, &[NORTH][..]), (true, &[SOUTH][..])];
        assert_eq!(
            census(r, 1e-12, &c, &[], &two),
            Err(PropsError::SphereLoop {
                what: "props_sphere_arc_span_forward"
            })
        );
        assert_eq!(census(r, 1e-15, &c, &[], &two), Ok(SeamCover::Once));
    }

    /// A census on a half-sphere threshold refuses rather than read: one
    /// hemisphere with no boundary claims half the sphere, `−½` of a
    /// cover, which no seam region of a body reads.
    #[test]
    fn a_census_on_a_threshold_refuses() {
        let c = circles(1.0);
        assert_eq!(
            census(1.0, 1e-9, &c, &[], &[(true, &[NORTH][..])]),
            Ok(SeamCover::Unresolved)
        );
    }

    /// A census decided below no cover is no tiling's: the cap above
    /// `z = ½` with no boundary claims a quarter of the sphere, `−¾` of a
    /// cover.
    #[test]
    fn a_census_below_no_cover_is_no_seam_region() {
        let c = circles(1.0);
        assert_eq!(
            census(1.0, 1e-9, &c, &[], &[(true, &[UP_HIGH][..])]),
            Err(PropsError::SphereLoop {
                what: "props_sphere_region_cover_untiled"
            })
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod turn_rows;
