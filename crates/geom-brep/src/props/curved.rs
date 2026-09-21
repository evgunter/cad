//! Closed-form flux/area for the curved M2 surfaces (cylinder, cone,
//! sphere, torus) over structurally verified iso-parameter rectangles
//! (see [`super`] module docs for the formulation and the stored-data
//! discipline).
//!
//! **Not everything public here serves that lane, and one item must
//! NOT be cited by it.** This module hosts two structural predicates
//! beside the flux arms: [`require_iso_rectangle`], the shape door
//! every consumer of the iso-rectangle premise cites, and
//! [`require_one_chart_branch`], the BRANCH door, which is `mesh`'s
//! alone. The flux lane must not cite the branch door: its own extent
//! derivations fold a chart singularity into the extent on purpose and
//! measure a pole-crossing arc EXACTLY (three rows of
//! `geom-brep/tests/cert1_sphere_polar.rs` ride that fold), so citing
//! the branch door from [`curved_face`] or from `mass_properties`
//! would retract those rows. Two predicates for two consumers, one
//! module, and the sentence that keeps them apart lives here.
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
use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use super::{FaceContribution, LoopEdge, PropsError, loop_vector_area};
use crate::dihedral::decide;

/// The flux and area of a curved face from its **outer** loop (curved
/// M2 faces carry no rings — the owning body refuses ringed curved
/// faces before calling). Dispatches on the surface kind; `band` is
/// the run's linear band, built once at operation entry.
///
/// `sense` is the face's orientation BIT (`topo::Face::sense`): `true`
/// where the surface's chart normal already points out of the
/// material, `false` where the face reverses it. It is
/// **deliberately not applied to every term**: `A⃗` and the rim-derived
/// flux side are recovered from the face's STORED LOOP TRAVERSAL,
/// which the interior-left rule already ties to the outward normal —
/// `revert` reverses loops and flips `sense` together, so signing
/// those terms by the sense as well would double-count and negate the
/// volume twice. `sense` is consumed at exactly one place, the
/// **rimless** sphere face, for the reason [`SphereFluxSide::Sense`]'s
/// doc states (the one home of that fact).
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
    match *surface {
        Surface::Plane { .. } => Err(PropsError::NotIsoRectangle {
            what: "curved_face called on a plane (planar faces take the loop route)",
        }),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => cylinder(origin, axis, radius, outer, band),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => cone(apex, axis, half_angle, outer, band),
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => sphere(center, radius, axis, outer, sense, band),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => torus(center, axis, major_radius, minor_radius, outer, band),
        // The volume/area inventory is closed-form per analytic kind;
        // a spline has no entry and neither does an offset description
        // over one. `Approx` refuses here rather than answer as its
        // fitted kind would.
        Surface::Nurbs(_) | Surface::Approx(_) => Err(PropsError::Unimplemented),
    }
}

/// The material side a curved face's **boundary traversal** encodes —
/// [`boundary_material_sign`]'s answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialSign {
    /// The boundary encodes the side: `Sign::Positive` ⇔ the stored
    /// outer-loop traversal places the material where a `sense: true`
    /// face's outward normal (`+chart_normal`) claims. Definite
    /// (`Positive`/`Negative`) by construction: every factor is a
    /// definite `classify` outcome or a stored traversal bool — the
    /// `Zero` arms refuse typed before a sign is minted.
    Encoded(Sign),
    /// The boundary does not encode the side: a face whose flux sign
    /// the boundary cannot supply, a single encoding with nothing to
    /// cross-check against — the documented residual of the curved
    /// sense gate. Two sphere faces ANSWER this, and a third shape
    /// refuses rather than reaching it:
    ///
    /// * The **rimless sphere face**, two-band or wedge: no rim, so no
    ///   traversal to read a side off, and its flux sign is the sense
    ///   bit alone ([`SphereFluxSide::Sense`]'s doc is the one home of
    ///   that fact).
    /// * The **rim-only polar cap**: one latitude in the levels and no
    ///   meridian, so the extent the side would be read against is the
    ///   very thing the face's sense bit settles. The rim of a ball cut
    ///   by one plane bounds the cap when the chart normal is outward
    ///   and the ball-minus-cap when it is inward, with the same
    ///   traversal either way — two valid bodies the boundary alone
    ///   cannot tell apart, so an `Encoded` answer here would be a
    ///   guess the check-6 gate would read as disagreement.
    /// * A face whose rims encode DIFFERENT sides is neither: it is an
    ///   `Err` ([`unanimous_rim_side`]), i.e. exempt by the gating
    ///   caller's own posture. `Unencoded` says *the boundary is
    ///   silent*; that face's boundary speaks and contradicts itself,
    ///   and a derivation that answered it would be answering with
    ///   whichever rim the loop walk reached first.
    Unencoded,
}

/// The flux derivations' material-side sign, factored public (M6-6):
/// which side of the chart normal the face's **stored outer-loop
/// traversal** says the material lies on — the boundary's own encoding
/// of the orientation fact `Face::sense` (M5 S10) also encodes. Tier
/// 3's curved check-6 arm compares the two encodings; this fn re-runs
/// the exact sub-derivations the flux lanes consume
/// ([`linear_rim_side`] for cylinder/cone/rim-bearing sphere,
/// anchor-rim traversal × chart orientation for the torus) — and with
/// each of them **the iso-rectangle premise it rests on, on all four
/// kinds**. The torus is not exempt: its side cancels the anchor-end
/// choice against `dv/dt` only when the two rims FLANKING the anchor
/// meridian carry opposite `d_u`, which every corner of a rectangle
/// gives and a reflex corner does not. All four go through the same
/// already-length-metered
/// named decides (`props_rim_side`, `props_rim_level`,
/// `props_circle_axis_class`, `props_meridian_orient`, …) — no new
/// comparand, no new margin.
///
/// **This refuses faces it used to answer for**, and that is the
/// point: on a domain that is not an iso-rectangle the linearly-
/// leveled derivation returns a definite ±1 that depends on where the
/// loop flattening started, not on the face. Gating callers treat an
/// error as exempt, so what they get instead is an exemption.
///
/// The derivation is chart-generic: with `n_chart = ∂u × ∂v`, the
/// interior-left rule makes "traversal direction on the extreme rim"
/// determine the material side on ANY nappe/latitude — which is why
/// the cone arm needs no nappe correction (the `v < 0` chart normal
/// negation and the physical-azimuth reversal cancel).
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
        } => {
            let b = cylinder_boundary(origin, axis, radius, outer, band)?;
            let (lo, hi) = min_max(&b.levels)?;
            Ok(MaterialSign::Encoded(linear_rim_side(&b, (lo, hi), band)?))
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (sin_a, cos_a) = half_angle.sin_cos();
            // The apex cap ANSWERS here, where the sphere's rim-only
            // cap declines: the cone's missing extreme is the apex
            // whichever way the rim runs ([`cone_apex_level`]), so the
            // side the traversal encodes is a side, and the one
            // traversal that would bound the nappe's unbounded
            // complement disagrees with the face's bit instead of
            // measuring the cap.
            let (b, _folded_apex) = cone_boundary(apex, axis, sin_a, cos_a, outer, band)?;
            let (lo, hi) = min_max(&b.levels)?;
            Ok(MaterialSign::Encoded(linear_rim_side(&b, (lo, hi), band)?))
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => {
            let (b, meridian_axes) = sphere_boundary(center, radius, axis, outer, band)?;
            if b.rims.is_empty() {
                return Ok(MaterialSign::Unencoded);
            }
            let (lo, hi) = min_max(&b.levels)?;
            // The rim-only cap: the levels carry no extent, so "which
            // extreme is this rim at" has no answer and the side the
            // boundary encodes is not a side at all — the SAME rim
            // traversed the SAME way bounds the cap under one sense bit
            // and the ball-minus-cap under the other. The second
            // `Unencoded` face, decided on the predicate the flux lane
            // folds the pole on ([`sphere_rim_only_pole_level`]), at
            // the shared comparand so the two lanes agree about which
            // faces are rim-only.
            if meridian_axes.is_empty()
                && classify(
                    "props_rim_only_extent",
                    sphere_extent_margin(lo, hi, radius),
                    band,
                )? == Sign::Zero
            {
                return Ok(MaterialSign::Unencoded);
            }
            // EVERY rim, not the first one ([`unanimous_rim_side`]):
            // `s_f` is one fact about the face, and a face whose rims
            // read it differently gets an exemption rather than
            // whichever answer the walk's anchor happened to reach.
            Ok(MaterialSign::Encoded(unanimous_rim_side(
                &b,
                (lo, hi),
                band,
            )?))
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let p = torus_parse(center, axis, major_radius, minor_radius, outer, band)?;
            // The premise, on this arm too. The torus reads its side
            // from ONE corner — the anchor meridian's chart orientation
            // and the rim sharing that meridian's `t0` vertex — and a
            // corner is not enough: the anchor-end choice cancels
            // against `dv/dt` only when the two rims flanking the
            // meridian carry OPPOSITE `d_u`, which every corner of a
            // rectangle does and a REFLEX corner does not. On an
            // L-shaped domain the meridian at the notch is flanked by
            // two rims of the same `d_u`, the cancellation fails, and
            // the six rotations of one edge cycle answer +,+,−,−,+,+.
            torus_rims_at_extremes(&p, center, axis, major_radius, minor_radius, band)?;
            let rim_a = torus_anchor_rim(&p.rims, &p.anchor)?;
            Ok(MaterialSign::Encoded(sign_mul(
                rim_a.d_u_sign,
                p.orient.flip(),
            )))
        }
        // As `curved_face`: no closed-form rim inventory for a spline
        // or for an offset description over one.
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
/// `props_rim_axis_parallel`, `props_rim_center_on_axis`, the
/// `props_meridian_*` incidences, `props_rim_level`), so a
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
/// this door says so. [`curved_face`] measures the same face by its
/// own premises — `Δu = π` when the meridians lie on one great circle
/// and the loop runs it once (`props_band_coplanar`,
/// `props_band_opposite`), the azimuth between the two half-planes
/// otherwise (`props_wedge_azimuth`, [`sphere_wedge_azimuth`]) — and
/// refuses a rimless boundary that states no lune it can read (a slit,
/// a meridian in pieces, arcs short of the poles). Two questions, two
/// homes: this door's answer for a lune is the shape's and does not
/// depend on which lunes the flux lane measures.
///
/// **What this door does NOT ask, on the sphere, and what to read
/// instead.** The rectangle is one named predicate here and TWO on the
/// sphere's flux lane, which also requires every rim's interior side
/// to point into the folded extent (`props_rim_interior_side`,
/// `curved::require_rim_interior_sides`). This door cannot ask the
/// second: σ is the rim's traversal under the face's SENSE bit, and
/// this door takes no face — it is handed a surface and a loop,
/// deliberately, so that it answers a question about the boundary
/// alone. The executed divergence is the L-shaped complement of a
/// half-cap (issue 1598): its carriers are a certified rim and a
/// certified meridian great circle and its one rim sits at an extreme,
/// so this door answers `Ok(())`, while `curved_face` refuses it
/// `NotIsoRectangle { what: "props_rim_interior_side" }` — the same
/// error type from the other lane, on a face this door admits.
///
/// A consumer that needs the stronger premise — that the domain is the
/// rectangle the levels fold, not its complement — has to read the
/// flux lane's refusal, i.e. call `curved_face` (or
/// `topo::mass_properties`) and treat its `NotIsoRectangle` as this
/// door's. `mesh`'s walk is the live caller and does not need it: it
/// walks the boundary it is given and meshes the region that boundary
/// bounds, which is the L-shaped face itself. What this door DOES take
/// is the premise's sense-free residue — every rim encodes the same
/// side ([`unanimous_rim_side`], via [`linear_rims_at_extremes`]) —
/// which catches a multi-rim contradiction without a bit and leaves
/// the one-rim divergence above exactly where it was.
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
            let (b, _folded_apex) = cone_boundary(apex, axis, sin_a, cos_a, outer, band)?;
            linear_rims_at_extremes(&b, band)
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => {
            let (b, _meridian_axes) = sphere_boundary(center, radius, axis, outer, band)?;
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
            torus_rims_at_extremes(&p, center, axis, major_radius, minor_radius, band).map(|_| ())
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
/// [`require_meridian_span_within_period`] — which is not an arc
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
                // GENERATORS only, by the parse's own incidence margin
                // — the cone's mirror of the sphere arm's
                // rim/meridian filter. A line that misses the apex is
                // no generator of this cone, has no apex in its span
                // to cross, and must not be refused here: without this
                // the arm read EVERY `Line` carrier as a generator and
                // answered `NotOneChartBranch` on a line the shape
                // door answers `props_meridian_generator` for.
                if classify(
                    "props_meridian_apex",
                    Margin::norm3((apex - e.p0()).cross(dir)),
                    band,
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
/// The second half is the SENSE-FREE residue of the flux lane's
/// interior-side premise. The door cannot take that premise itself —
/// σ is the rim's traversal under the face's sense BIT and the door is
/// handed a surface and a loop with no face, deliberately — but
/// unanimity needs no bit: it says the rims agree with EACH OTHER,
/// which is a fact about the boundary alone. What it catches is a face
/// whose rims contradict one another, for which [`linear_rim_side`]
/// answers a definite ±1 decided by which rim the owning body's loop
/// walk handed over first.
///
/// **It does not close the door's documented divergence.** The
/// L-shaped complement of a half-cap has ONE rim, so there is nothing
/// for a unanimity rule to compare and no sense-free door can tell it
/// from the half-cap; that residue is `props_rim_interior_side`'s
/// alone and stays with the flux lane.
///
/// **Vacuous where it has nothing to decide**, in the door's own two
/// senses of that word. A rimless parse (the sphere band) has no rim
/// to place and none to compare. A ZERO-EXTENT parse has no extreme
/// for a rim to sit at, so "which extreme is this rim at" — and with
/// it "do the rims agree" — is undefined rather than violated:
/// [`rim_side`] answers that with `DegenerateFace`, and this door
/// admits it, because extent is not a shape question (a degenerate
/// rectangle is a rectangle) and the flux lane refuses it at its own
/// [`require_extent`] first.
fn linear_rims_at_extremes<T: Decide>(b: &LinearBoundary<T>, band: Band) -> Result<(), PropsError> {
    let (lo, hi) = min_max(&b.levels)?;
    if b.rims.is_empty() {
        return Ok(());
    }
    match unanimous_rim_side(b, (lo, hi), band) {
        Ok(_) | Err(PropsError::DegenerateFace) => Ok(()),
        Err(e) => Err(e),
    }
}

/// A torus face's parse with the anchor meridian's chart orientation:
/// the prologue every torus consumer runs before deciding anything —
/// the flux lane, [`boundary_material_sign`] and
/// [`require_iso_rectangle`] — in one place, so the refusal names and
/// their order are one. The anchor is the FIRST meridian in loop
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
    let orient = torus_meridian_orient(&anchor, center, axis, minor, band)?;
    Ok(TorusParse {
        rims,
        anchor,
        orient,
    })
}

/// The torus's two extreme minor angles from the anchor meridian's
/// stored span ([`torus_ends`]) with the iso-rectangle premise decided
/// against them — one call for the three torus consumers. Returns the
/// ends as `(s0, c0, s1, c1)` for the flux arm's closed form.
fn torus_rims_at_extremes<T: Decide>(
    p: &TorusParse<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    band: Band,
) -> Result<(T, T, T, T), PropsError> {
    let (s0, c0, s1, c1) = torus_ends(&p.anchor, center, axis, major, minor, p.orient);
    require_rims_at_extremes(
        &p.rims,
        (RimLevel::Unit(s0, c0), RimLevel::Unit(s1, c1)),
        torus_arms(major, minor),
        band,
    )?;
    Ok((s0, c0, s1, c1))
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

/// The product of two discrete signs (`Zero` absorbs — unreachable
/// here: every producer feeding this is definite, but the arm stays
/// total rather than panicking, D9).
fn sign_mul(a: Sign, b: Sign) -> Sign {
    match b {
        Sign::Positive => a,
        Sign::Negative => a.flip(),
        Sign::Zero => Sign::Zero,
    }
}

/// Funnel wrapper: classify, mapping an escalation to the typed
/// [`PropsError::Escalated`].
fn classify<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, PropsError> {
    decide(name, margin, band).map_err(|cause| PropsError::Escalated { cause })
}

/// Require a consistency residual to be coincident with zero.
fn require_zero<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<(), PropsError> {
    match classify(name, margin, band)? {
        Sign::Zero => Ok(()),
        Sign::Positive | Sign::Negative => Err(PropsError::NotIsoRectangle { what: name }),
    }
}

/// **The sphere's extent comparand, in one place.** The latitude-sine
/// span at the sphere radius — `(hi − lo)·R`, a length, the point
/// deviation of collapsing the face's `v`-extent.
///
/// Three sites read it and they must read the SAME thing:
/// [`require_extent`]'s sphere call, [`sphere_rim_only_pole_level`]'s
/// `props_rim_only_extent` (the same question asked one step earlier,
/// before a pole is folded in) and [`boundary_material_sign`]'s sphere
/// arm, which asks the second question so the gate and the flux lane
/// agree about which faces are rim-only. Two names over one comparand
/// is deliberate — the answers differ in what they DO, not in what
/// they measure — and a meridian-free rim-bearing face therefore
/// records both.
///
/// `props_rim_only_extent` is a name the cone shares
/// ([`cone_apex_level`]) and this comparand is not: a cone level is a
/// bare slant length, so its fold reads `require_extent`'s cone
/// comparand exactly as this reads the sphere's. One name, one
/// question, each kind's own metering — as `props_face_extent`
/// already is across all four.
fn sphere_extent_margin<T: Real>(lo: T, hi: T, radius: T) -> Margin<T> {
    Margin::levered(hi - lo, radius)
}

/// Require a definitely-positive extent (degenerate ⇒ typed error).
fn require_extent<T: Decide>(margin: Margin<T>, band: Band) -> Result<(), PropsError> {
    match classify("props_face_extent", margin, band)? {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(PropsError::DegenerateFace),
    }
}

/// Rim incidence on its surface of revolution (M2 PR 7 review F1:
/// shape checks alone — radius fit, definite axis class — accept
/// circles nowhere on the surface). Certifies, with `w` the rim center
/// minus the surface's axis origin:
///
/// - carrier axis **parallel** to the surface axis (the definite axis
///   class only pins `n_c·â ≠ 0`): margin `‖n_c × â‖·r_c` — the
///   tilt angle metered at the rim radius (lever arm `r_c`);
/// - rim center **on the axis line**: margin `‖w − â(w·â)‖`, the
///   center's perpendicular offset from the axis (already meters).
///
/// Together with the per-surface radius/level fit these pin the rim
/// circle pointwise onto the surface.
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
    )?;
    require_zero(
        "props_rim_center_on_axis",
        Margin::norm3(w - axis * w.dot(axis)),
        band,
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
    /// every sphere margin's: the side test in [`linear_rim_side`]
    /// reads the primary component alone (`lo + hi − 2s`, at the same
    /// arm), and `require_extent`'s sphere margin is the axial sine
    /// difference `(hi − lo)·R` — both still shrink by `cos v̄` near
    /// the poles, in the REFUSING direction only (audit note N8).
    Unit(T, T),
}

/// A classified rim: `u`-traversal direction (`d_u_sign`), parameter
/// span (`dt`, the face's `Δu` candidate), and its iso-level payload.
struct Rim<T: Real> {
    /// The traversal direction in `u` as a **discrete** definite sign:
    /// sign(carrier axis · surface axis) × the stored traversal bool,
    /// both discrete at origin (a definite `classify` outcome and a
    /// bool), so every consumer of it compares exact ±1s rather than
    /// scalars — [`boundary_material_sign`]'s material-side
    /// cross-check, [`rim_interior_side`]'s σ and [`du_of_rims`]'
    /// group key alike. The direction has ONE representation here: a
    /// `T` copy beside it fed a `Margin` over two values that are ±1
    /// by construction, which is a discrete comparison wearing a
    /// tolerance's costume.
    d_u_sign: Sign,
    /// Carrier parameter span `t1 − t0` (angle-true; `Δu`).
    dt: T,
    /// The rim's iso-level, dimension carried by the variant (see
    /// [`RimLevel`] and the per-surface call sites).
    level: RimLevel<T>,
    /// Traversal-order endpoint tags.
    tags: (u32, u32),
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

/// **The one spelling of "this rim level is one of these levels."**
///
/// Both consumers decide exactly this — [`du_of_rims`] against a
/// group's key, [`require_rims_at_extremes`] against the face's two
/// extremes — so they share the rule rather than each carrying one.
/// They carried two, and the two disagreed on all three of the axes
/// this fn now fixes:
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
///   ends), and it REFUSES here — [`mixed_levels`]. Answering
///   `Ok(false)` would let a caller that only groups carry on.
///
/// There are exactly two shapes and the signature says so: `other` is
/// a group key, or the face's `lo` extreme with `or` carrying `hi`.
/// The NEARER is what is decided, so a rim sitting exactly on one
/// extreme is not made to escalate by the other.
///
/// `name` is the funnel's recording channel, not a second rule — the
/// two call sites keep their own predicate names (`props_rim_level`,
/// `props_rim_level_group`) because their margin populations are
/// separately audited (`docs/predicate-dimension-audit.md`,
/// `docs/K-REPORT.md`), and one rule reported on two channels is not
/// two rules.
fn level_coincides<T: Decide>(
    name: &'static str,
    level: RimLevel<T>,
    other: RimLevel<T>,
    or: Option<RimLevel<T>>,
    arms: RimArms<T>,
    band: Band,
) -> Result<bool, PropsError> {
    let Some(mut gap) = level_gap(level, other) else {
        return Err(mixed_levels(name));
    };
    if let Some(second) = or {
        let Some(d) = level_gap(level, second) else {
            return Err(mixed_levels(name));
        };
        gap = gap.min(d);
    }
    let margin = match level {
        RimLevel::Length(_) => Margin::of(gap),
        RimLevel::Unit(..) => Margin::levered(gap, arms.level),
    };
    Ok(classify(name, margin, band)? == Sign::Zero)
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

/// The refusal a mixed-representation level pair gets: the typed
/// structural refusal, named for the predicate that asked.
///
/// **It is not routed through the funnel, and it used to be.** A mixed
/// pair has no comparand — the two representations are not two values
/// of one quantity — so there is nothing to decide, and the refusal
/// was manufactured by feeding `f64::NAN` into [`classify`] and
/// keeping whatever came back: a decision predicate used as a `throw`,
/// which spends a recorded verdict on a margin that measures nothing
/// and makes the error's TYPE depend on how the funnel happens to
/// treat a poisoned value. The outcome this arm owes its callers is
/// the one they always got — an `Err`, never `Ok(false)`, so a
/// mixed-representation face neither measures nor groups — and it is
/// stated here directly.
///
/// **Which reading of D9 this leaves.** The state is kernel-bug-only
/// (one surface builds every rim of a face AND both its ends), and
/// `props/curved.rs` answers that class two ways —
/// [`torus_meridian_orient`]'s `unreachable!` sibling and
/// [`unreachable_zero`]'s poison return. Neither is adopted here: this
/// arm keeps the typed refusal it already produced, because choosing
/// between the two is the file-wide census
/// `props-curved-carries-two-readings-of-d9-unreachable-vs-poison`
/// asks for and a decision taken at one site would pre-empt it.
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
/// `w`. (`du_of_rims` supplies the `Δu` value and, once this holds,
/// its span-sum agreement is a genuine rectangle test rather than a
/// proxy for one.)
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
        if !level_coincides(
            "props_rim_level",
            rim.level,
            ends.0,
            Some(ends.1),
            arms,
            band,
        )? {
            return Err(PropsError::NotIsoRectangle {
                what: "props_rim_level",
            });
        }
    }
    Ok(())
}

/// Check all rims agree on `Δu`; returns the face's `Δu`.
///
/// Two margins are metered here and they do not share a lever
/// ([`RimArms`]): the rim LEVEL difference that keys the grouping
/// turns about `arms.level`, the `Δu` angle difference about
/// `arms.azimuth`. On the torus those are `minor` and `major`. The
/// third half of the group key — the traversal direction — is
/// [`Rim::d_u_sign`], a discrete definite sign compared AS a sign. Both
/// sides are `±1` by construction, so the band around their difference
/// could only ever decide `Zero` on a difference of `0` or of `±2` —
/// and `==` agrees with it on the first. On the second it does not:
/// the old margin was `±2·arms.azimuth`, which lands inside the zero
/// band whenever `2·azimuth < K·ε`, so a gasket-scale torus merged
/// OPPOSITELY traversed rims into one group and `==` never does. The
/// sign compare is therefore strictly stricter, which is the direction
/// a premise may move in without re-deciding what it admits.
///
/// **This is the Δu VALUE, not the shape test.** Every caller runs
/// [`require_rims_at_extremes`] first, which is what makes the domain
/// a rectangle; with all rims at the two extreme levels the span-sum
/// agreement checked here says the two ends carry the same total
/// `u`-measure, i.e. it pins the value the rectangle's `w ≡ Δu`
/// already guarantees is constant. Standing alone — as it did before
/// S58 — it guaranteed only `w(v) ∈ {k·Δu}`, which is #649.
///
/// **`props_du_consistent`'s reachability is `unsure`** (recorded, not
/// settled — #714's review). With every rim pinned to an extreme and
/// every non-rim boundary edge a meridian, no loop has been
/// constructed whose extreme-level groups disagree on their span
/// sums, and nothing in the workspace asserts that refusal by name.
/// If it is in fact unreachable then this `require_zero` is a value
/// computation wearing a typed-refusal costume and should be an
/// `unreachable`-class site (D2 addendum row 4/5) instead. It is left
/// as a refusal because the argument for unreachability rests on the
/// predicate above being complete, and #723 was a live demonstration
/// that a premise about these domains can be one clause short.
fn du_of_rims<T: Decide>(rims: &[Rim<T>], arms: RimArms<T>, band: Band) -> Result<T, PropsError> {
    if rims.is_empty() {
        return Err(PropsError::NotIsoRectangle {
            what: "curved face without a rim (non-sphere)",
        });
    }
    // Group arcs by (rim LEVEL, traversal direction) first (M5 PR 9):
    // a re-merged or boolean-cut wall's rim legitimately arrives as
    // SEVERAL arcs per level (a rim run of two arcs after a strut
    // kev), so the face's Δu is the per-group SUM of spans, required
    // consistent ACROSS groups — the pre-PR-9 first-arc rule silently
    // undercounted multi-arc rims. Direction joins the key so the
    // degenerate zero-extent patch (both rims one level, opposite
    // traversal) keeps its M2 verdict downstream.
    let mut groups: Vec<(RimLevel<T>, Sign, T)> = Vec::new(); // (level, direction, dt sum)
    for rim in rims {
        let mut placed = false;
        for g in &mut groups {
            let same = level_coincides("props_rim_level_group", rim.level, g.0, None, arms, band)?;
            if same && rim.d_u_sign == g.1 {
                g.2 = g.2 + rim.dt;
                placed = true;
                break;
            }
        }
        if !placed {
            groups.push((rim.level, rim.d_u_sign, rim.dt));
        }
    }
    let total = groups[0].2;
    for g in &groups[1..] {
        require_zero(
            "props_du_consistent",
            Margin::levered(g.2 - total, arms.azimuth),
            band,
        )?;
    }
    Ok(total)
}

/// A linearly-leveled face's boundary, parsed once — **and the
/// metering choice that goes with it**.
///
/// The rims and levels are the parse; `arms` and `as_level` are the
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
    /// Lifts a scalar extreme into the rims' own [`RimLevel`]
    /// representation: `Length` for the cylinder and cone, the
    /// latitude-sine `Unit` pair for the sphere.
    as_level: fn(T) -> RimLevel<T>,
}

/// **The material-side sign of a linearly-leveled face** (cylinder,
/// cone, rim-bearing sphere), together with the premise it rests on.
///
/// The side is read off `lo + hi − 2v` — *which extreme is this rim
/// at* — and that is a material side only on a domain whose rims ALL
/// sit at `lo` or at `hi`. The two therefore travel together: the
/// derivation is **nested inside this fn**, so there is no
/// module-private door to it that a later arm could reach without the
/// premise.
///
/// **Why a caller cannot be left to run the premise itself.** Without
/// it the derivation reads the side off whichever rim the owning body's
/// loop flattening happens to put first, and answers a DEFINITE ±1
/// that is a property of the flattening rather than of the face — two
/// rotations of one edge cycle answer opposite signs.
/// [`boundary_material_sign`]'s callers must treat an error as exempt
/// (the check-7 posture), so pairing the premise with the side turns
/// that into the exemption they already handle. Unpaired it does the
/// opposite: tier 3's curved check 6 raises a `CurvedSenseInverted`
/// from the wrong ±1 and, check 7 being gated on `errors.is_empty()`,
/// SUPPRESSES the honest `NotIsoRectangle` the flux lane raises on the
/// same face.
///
/// **The answer is a DEFINITE side; `Zero` refuses here rather than
/// being returned** — and this is the one home of that claim, for
/// every consumer. The inner `side` answers `rim.d_u_sign` or its flip
/// on a definite `props_rim_side` outcome and `DegenerateFace` on
/// `Zero`, and [`Sign::flip`] fixes only `Zero`, so the outcome is
/// definite wherever the rim's own stored traversal direction is. The
/// `debug_assert` below is that claim's single guard: consumers
/// scalarize the answer with no zero arm of their own, and a future
/// arm that could answer `Zero` announces itself here rather than at
/// whichever consumer happened to keep a copy of the argument.
fn linear_rim_side<T: Decide>(
    b: &LinearBoundary<T>,
    (lo, hi): (T, T),
    band: Band,
) -> Result<Sign, PropsError> {
    require_rims_at_extremes(&b.rims, ((b.as_level)(lo), (b.as_level)(hi)), b.arms, band)?;
    let rim = b.rims.first().ok_or(PropsError::NotIsoRectangle {
        what: "curved face without a rim (non-sphere)",
    })?;
    let s = rim_side(rim, lo, hi, b.arms, band)?;
    debug_assert!(
        s != Sign::Zero,
        "linear_rim_side answers a definite rim traversal direction"
    );
    Ok(s)
}

/// `s_f` from `rim`'s level and the face's level range: the interior
/// lies toward the opposite extreme. Metered by the level's own
/// dimension ([`RimLevel`]) — bare for `Length` (meters already),
/// `× arms.level` for the dimensionless `Unit` primary component.
///
/// Returns the **discrete** sign, definite by construction: a definite
/// `props_rim_side` outcome × the rim's definite traversal direction.
/// Flux callers scalarize through [`t_sign`]; the material-side gate
/// consumes it combinatorially.
///
/// **Not a door.** It carries no premise of its own — the two homes
/// that pair it with one are [`linear_rim_side`] (the first rim, under
/// `props_rim_level`) and [`unanimous_rim_side`] (every rim, under the
/// same). A third caller would be a third strength, which is what S58
/// was about.
fn rim_side<T: Decide>(
    rim: &Rim<T>,
    lo: T,
    hi: T,
    arms: RimArms<T>,
    band: Band,
) -> Result<Sign, PropsError> {
    match classify(
        "props_rim_side",
        rim_offset_margin(rim.level, lo, hi, arms, Sign::Positive),
        band,
    )? {
        Sign::Positive => Ok(rim.d_u_sign),
        Sign::Negative => Ok(rim.d_u_sign.flip()),
        Sign::Zero => Err(PropsError::DegenerateFace),
    }
}

/// **The side EVERY rim encodes, required unanimous** — the
/// sense-free residue of the sphere's interior-side premise, and the
/// form [`boundary_material_sign`] can take.
///
/// `s_f` is one fact about a face, so every rim's reading of it must
/// be the same fact. [`linear_rim_side`] reads `b.rims.first()`, which
/// answers a DEFINITE ±1 on a face whose rims disagree — and which one
/// depends on where the owning body's loop walk started, so the
/// ANSWER, not merely the recorded verdict, is a fact about cycle
/// order. Executed on a sphere face carrying a rim at `lo` and a rim
/// at `hi` traversed the SAME way: `Encoded(Positive)` with the lower
/// rim first, `Encoded(Negative)` with the upper rim first (the
/// staircase face of `r2_sph_probes`). That is the failure
/// [`linear_rim_side`]'s own docs say the paired premise exists to
/// prevent, reached through the one rim the premise does not
/// constrain.
///
/// Requiring unanimity answers an EXEMPTION there instead, which is
/// what the gate's callers already handle (the check-7 posture). It
/// needs no sense bit, so it is available to a derivation that must
/// not read one: it says the rims agree with EACH OTHER, not that they
/// agree with the face's stored orientation, and comparing those two
/// is what tier 3's check 6 is.
fn unanimous_rim_side<T: Decide>(
    b: &LinearBoundary<T>,
    (lo, hi): (T, T),
    band: Band,
) -> Result<Sign, PropsError> {
    require_rims_at_extremes(&b.rims, ((b.as_level)(lo), (b.as_level)(hi)), b.arms, band)?;
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
    answer.ok_or(PropsError::NotIsoRectangle {
        what: "curved face without a rim (non-sphere)",
    })
}

/// **Where a rim sits in its face's level range, as a margin** — the
/// signed offset `lo + hi − 2v`, positive when the rim sits at `lo`,
/// pointed by `toward`.
///
/// Metered by the level's own dimension ([`RimLevel`]): bare for
/// `Length` (meters already), `× arms.level` for the dimensionless
/// `Unit` primary component. `toward` multiplies the offset by an
/// exact ±1 before the margin is formed, so
/// [`require_rim_interior_sides`] asks "does the interior side point
/// INTO the range" with the same comparand, lever and band that
/// [`linear_rim_side`] asks "which extreme is this rim at" with — one
/// margin, two questions, no second dimension analysis.
///
/// The offset reads ONE rim and the face's own extent. Nothing about
/// which rim a loop walk hands over first reaches it.
fn rim_offset_margin<T: Real>(
    level: RimLevel<T>,
    lo: T,
    hi: T,
    arms: RimArms<T>,
    toward: Sign,
) -> Margin<T> {
    let d = t_sign::<T>(toward);
    match level {
        RimLevel::Length(v) => Margin::of(d * (lo + hi - v - v)),
        RimLevel::Unit(s, _) => Margin::levered(d * (lo + hi - s - s), arms.level),
    }
}

// ---------------------------------------------------------------------
// Cylinder
// ---------------------------------------------------------------------

/// Cylinder face: rims are circles of the cylinder's radius centered
/// on the axis; meridians are axial lines. `v = (p − origin)·axis`
/// (meters), `Area = r·Δu·(v_hi − v_lo)`,
/// `∮(p−o)·n_chart dA = r·Area` ⇒ flux `= s_f·r·Area + o·A⃗`.
fn cylinder<T: Decide>(
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let b = cylinder_boundary(origin, axis, radius, edges, band)?;
    let (lo, hi) = min_max(&b.levels)?;
    require_extent(Margin::of(hi - lo), band)?;
    // The iso-rectangle premise, before anything integrates against it
    // (S58/#649), inside `linear_rim_side` with the side it underwrites.
    // A rim-free wall whose meridian endpoints all sit at one level
    // reports `DegenerateFace` (zero extent) rather than `du_of_rims`'
    // "curved face without a rim (non-sphere)": both are typed refusals
    // of the same input, and the second named the cause better.
    let s_f = t_sign::<T>(linear_rim_side(&b, (lo, hi), band)?);
    let du = du_of_rims(&b.rims, b.arms, band)?;
    let area = radius * du * (hi - lo);
    let va = loop_vector_area(edges, origin)?;
    let flux = s_f * (radius * area) + (origin - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
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
                require_zero(
                    "props_meridian_axial",
                    Margin::levered(dir.cross(axis).norm(), e.t1 - e.t0),
                    band,
                )?;
                // Incidence: the (certified-axial) line lies on the
                // cylinder iff one of its points does — radial distance
                // of the interval start from the axis vs the radius
                // (meters).
                let w0 = e.p0() - origin;
                require_zero(
                    "props_meridian_on_surface",
                    Margin::of((w0 - axis * w0.dot(axis)).norm() - radius),
                    band,
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
                )?;
                if s == Sign::Zero {
                    return Err(PropsError::NotIsoRectangle {
                        what: "cylinder boundary circle is not a rim",
                    });
                }
                require_zero("props_rim_fit", Margin::of(r_c - radius), band)?;
                require_rim_incidence(center - origin, n_c, r_c, axis, band)?;
                let v = (center - origin).dot(axis);
                rims.push(Rim {
                    d_u_sign: rim_dir(s, e.forward),
                    dt: e.t1 - e.t0,
                    // The axial arc length itself — meters.
                    level: RimLevel::Length(v),
                    tags: (e.start, e.end),
                });
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
        as_level: RimLevel::Length,
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

/// Cone face: rims are axis-centered circles of radius `|v|·sin α`;
/// meridians are generator lines (`|dir·axis| = cos α`). `v` is the
/// signed slant parameter `((p − apex)·axis)/cos α`; the face must not
/// definitely span both nappes. `Area = sin α·Δu·|v_hi² − v_lo²|/2`;
/// `(p − apex)·n_chart = 0` along generators, so the anchored term
/// vanishes and flux `= apex·A⃗` — no orientation sign is needed.
fn cone<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let (sin_a, cos_a) = half_angle.sin_cos();
    // A generator-free cone boundary of rims alone carries no extent of
    // its own: every level it touches is a rim's slant, and where those
    // coincide the face's missing extreme is the APEX
    // ([`cone_apex_level`], folded inside the shared parse so all three
    // doors read the same extent).
    let (b, folded_apex) = cone_boundary(apex, axis, sin_a, cos_a, edges, band)?;
    let (lo, hi) = min_max(&b.levels)?;
    require_extent(Margin::of(hi - lo), band)?;
    // The iso-rectangle premise (S58/#649). Cone levels are the signed
    // SLANT arc length — `Length`, so bare — and the arm (the first
    // rim's own radius) meters only the dimensionless margins
    // downstream. The premise alone here: a cone's flux needs no `s_f`
    // (generators run through the apex, so the anchored term
    // vanishes), and metering a side this lane does not read would be
    // a decide for nobody. `boundary_material_sign`'s cone arm, which
    // DOES read one, reaches it through `linear_rim_side` and gets the
    // premise with it.
    require_rims_at_extremes(&b.rims, ((b.as_level)(lo), (b.as_level)(hi)), b.arms, band)?;
    let du = du_of_rims(&b.rims, b.arms, band)?;
    // A folded apex is a claim that the rim CLOSES around it, and `Δu`
    // is the only place that claim can be read
    // ([`require_rim_only_closed`], at the cone's own azimuthal arm).
    if folded_apex {
        require_rim_only_closed(du, b.arms.azimuth, band)?;
    }
    // Single-nappe check: definitely-negative low AND definitely-positive
    // high would straddle the apex through both nappes.
    let s_lo = classify("props_cone_nappe", Margin::of(lo), band)?;
    let s_hi = classify("props_cone_nappe", Margin::of(hi), band)?;
    if s_lo == Sign::Negative && s_hi == Sign::Positive {
        return Err(PropsError::NappeSpanning);
    }
    let half = T::from_f64(0.5);
    let area = sin_a * du * ((hi.powi(2) - lo.powi(2)) * half).abs();
    let va = loop_vector_area(edges, apex)?;
    let flux = (apex - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
}

/// Classify a cone face's boundary into (rims, signed slant levels) —
/// the shared parse consumed by the flux closed form,
/// [`boundary_material_sign`] and [`require_iso_rectangle`] — **with
/// the apex folded in where the boundary needs it**
/// ([`cone_apex_level`]; the returned flag says whether it was).
///
/// The fold lives here rather than in the flux lane because the cone's
/// missing extreme needs no sense bit, so all three doors can and must
/// read the SAME extent: the gate's `Encoded` side for an apex cap is
/// what catches the inverted traversal at tier 3's check 6, and it can
/// only be read against an extent the parse supplies. The sphere's
/// pole fold is the other shape — it reads `Face::sense`, so it sits
/// in `sphere` and the gate arm declines to answer at all.
fn cone_boundary<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    sin_a: T,
    cos_a: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<(LinearBoundary<T>, bool), PropsError> {
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut levels: Vec<T> = Vec::new();
    let mut generators = false;
    for e in edges {
        match e.carrier {
            Curve3::Line { dir, .. } => {
                generators = true;
                require_zero(
                    "props_meridian_generator",
                    Margin::levered(dir.dot(axis).abs() - cos_a, e.t1 - e.t0),
                    band,
                )?;
                // Incidence: a line at the generator angle is a
                // generator iff it passes through the apex — the
                // apex-to-line distance `‖(apex − p) × dir‖` (`dir`
                // unit, so meters directly).
                require_zero(
                    "props_meridian_apex",
                    Margin::norm3((apex - e.p0()).cross(dir)),
                    band,
                )?;
                levels.push((e.p0() - apex).dot(axis) / cos_a);
                levels.push((e.p1() - apex).dot(axis) / cos_a);
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
                )?;
                if s == Sign::Zero {
                    return Err(PropsError::NotIsoRectangle {
                        what: "cone boundary circle is not a rim",
                    });
                }
                let v = (center - apex).dot(axis) / cos_a;
                require_zero("props_rim_fit", Margin::of(r_c - v.abs() * sin_a), band)?;
                require_rim_incidence(center - apex, n_c, r_c, axis, band)?;
                rims.push(Rim {
                    d_u_sign: rim_dir(s, e.forward),
                    dt: e.t1 - e.t0,
                    // The signed slant arc length itself — meters.
                    level: RimLevel::Length(v),
                    tags: (e.start, e.end),
                });
                levels.push(v);
            }
            Curve3::Ellipse { .. } => {
                return Err(PropsError::NotIsoRectangle {
                    what: "cone boundary carries an ellipse arc (a tilted-section cut) — the \
                           class has no cone-chart image at all (azimuth-non-harmonic, \
                           and no ring-computable fitted certificate either), so the \
                           quadrature lane has nothing to consume",
                });
            }
            // A spiric lies on no cylinder or cone: refused beside the
            // spline, typed.
            Curve3::Nurbs(_) | Curve3::Spiric { .. } => return Err(PropsError::Unimplemented),
        }
    }
    // Cone levels are the signed SLANT arc length — `Length`, bare —
    // and the arm is the first rim's own radius ([`cone_arm`]), which
    // meters only the dimensionless margins in `du_of_rims`.
    let arms = RimArms::uniform(cone_arm(&rims, sin_a));
    let mut b = LinearBoundary {
        rims,
        levels,
        arms,
        as_level: RimLevel::Length,
    };
    let folded_apex = !generators && !b.rims.is_empty() && cone_apex_level(&mut b, band)?;
    Ok((b, folded_apex))
}

/// **A generator-free cone boundary's missing extreme is the APEX** —
/// level `0`, pushed into the levels so the face has the extent its
/// rims alone cannot state.
///
/// A cone face bounded by one rim circle and nothing else is the shape
/// a ball cut by one plane gives on the sphere: the levels hold one
/// slant, `min_max` answers `lo == hi` and [`require_extent`] refuses
/// `DegenerateFace` for a face that plainly has an area. **Unlike the
/// sphere's, it needs no σ**: the sphere's missing extreme is one of
/// TWO poles and the rim's traversal under the face's sense bit picks
/// between them, while a cone is bounded on the apex side ONLY, so
/// there is a single candidate and no bit to read. (A cylinder is
/// unbounded both ways along its axis and has no candidate at all,
/// which is why it has no fold.)
///
/// **Unanimous traversal direction is required, and that is the
/// sphere's premise transposed.** [`sphere_rim_only_pole_level`] folds
/// only where every rim's σ agrees; σ is `d_u_sign` times the face's
/// one sense bit ([`rim_interior_side`]), so unanimity of σ IS
/// unanimity of `d_u_sign` and the cone can require it without the
/// bit. What it excludes is the true zero-extent patch — rims at one
/// level traversed opposite ways, which point at opposite sides and
/// contain no apex — and that face keeps its `DegenerateFace`.
///
/// The extent question is [`require_extent`]'s own comparand asked one
/// step earlier, under the name the sphere's fold asks it by
/// (`props_rim_only_extent`): the cone's is the bare slant difference,
/// as `require_extent`'s cone call reads it.
fn cone_apex_level<T: Decide>(b: &mut LinearBoundary<T>, band: Band) -> Result<bool, PropsError> {
    let (lo, hi) = min_max(&b.levels)?;
    if classify("props_rim_only_extent", Margin::of(hi - lo), band)? != Sign::Zero {
        return Ok(false);
    }
    let Some((first, rest)) = b.rims.split_first() else {
        return Ok(false);
    };
    if !rest.iter().all(|r| r.d_u_sign == first.d_u_sign) {
        return Ok(false);
    }
    b.levels.push(T::zero());
    Ok(true)
}

/// The cone's azimuthal lever arm: the first rim's own radius
/// `|v|·sin α` (cone rims are `Length`-leveled, so the arm meters only
/// the dimensionless direction/Δu margins in `du_of_rims`).
///
/// **The `T::one()` fallback is never metered against.** It is
/// reachable — both callers compute the arm before they know whether
/// there is a rim — but every route from here to a margin refuses on
/// the empty rim list first, and they are different routes: the flux
/// lane's is `du_of_rims`' opening `is_empty` refusal (nothing else
/// upstream of it consumes the arm, `require_rims_at_extremes` being
/// vacuous on no rims), the gate's is `linear_rim_side`'s
/// `rims.first()`. Stating one of them would be true at one call site
/// and false at the other; the invariant is what both establish.
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

/// Sphere face: rims are circles with axis ∥ the sphere axis
/// (`sin v = ((C − c)·axis)/R`); meridians are great circles through
/// the poles (center = sphere center, radius = R, axis ⊥ sphere axis).
/// Levels are `sin v` (latitude sines — all formulas need only these).
/// `Area = R²·Δu·(sin v_hi − sin v_lo)`,
/// `(p − c)·n_chart = R` ⇒ flux `= s_f·R·Area + c·A⃗`.
///
/// A face with **no rims** is bounded by meridian great-circle arcs
/// alone, and there are two such faces in the inventory. What the
/// rimless branch below establishes, and what it does not, stated
/// exactly — it is the ONE domain the iso-rectangle predicate is
/// exempt from, and an exemption is a claim about the arm, not a fact
/// the arm checks:
///
/// * **Established.** Every boundary edge classified `Zero` by
///   `props_circle_axis_class` is a meridian great circle centred on
///   the sphere centre at the sphere's radius (`props_meridian_great`).
///   Then EITHER every carrier axis is parallel to the first
///   (`props_band_coplanar`, decided `Zero` for each) — the arcs lie on
///   ONE great circle — and the loop CONTINUES through every junction
///   ([`require_band_opposite`], `props_band_opposite` decided `Zero`
///   at each shared vertex: the traversal tangent arriving equals the
///   one departing), so the arcs traverse that great circle once and
///   cut the sphere into two halves of azimuthal width π: the
///   **two-band face**, `Δu = π`. A coplanar loop that REVERSES at a
///   junction runs one half-plane there and back — a slit, or the ball
///   less a slit — and states no lune; it refuses typed there. OR some
///   axis is definitely not parallel to the first, and the face is the
///   **wedge**: two arcs, each pole to pole, on two great circles, and
///   `Δu` is the azimuth between their half-planes on the side the face
///   covers, read structurally by [`sphere_wedge_azimuth`] under
///   `props_wedge_azimuth` — which checks the pole-to-pole premise
///   itself. The coplanar decides run in the same order with the same
///   margins on both branches; only the disposition of a definite
///   nonzero differs.
/// * **Established separately: the `v`-extent.** `(lo, hi)` is
///   `min_max` over the meridians' endpoint latitudes AND each arc's
///   span-derived pole extremes ([`sphere_meridian_span_levels`]), so
///   the arcs need not run pole to pole for the extent to be theirs:
///   the same hemisphere split at two ordinary points instead of at
///   its poles still folds to `[−1, 1]`. The extent derivation is a
///   fact about the levels, not about this exemption — do not read
///   the exemption as "the domain is verified a rectangle".
///
/// **The rim-bearing branch's own premise, and why it is not in that
/// list.** The three bullets above are the RIMLESS branch's; this one
/// is the other branch's, established by the rims' TRAVERSAL: *the
/// extent is the face's own*. A level says a latitude the boundary
/// touches; it never says which side of that latitude the material is
/// on, and the two faces a rim separates touch the same levels. That
/// side is σ ([`rim_interior_side`]), the rim's own traversal
/// direction under the face's sense bit, and the arm decides it
/// against the folded extent per rim ([`require_rim_interior_sides`]):
/// a rim whose interior side points OUT of `[lo, hi]` bounds a face
/// the rectangle does not describe — the L-shaped complement of a
/// half-cap, refused `props_rim_interior_side`. Where the levels are
/// silent altogether the traversal is the only speaker and supplies
/// the missing extreme ([`sphere_rim_only_pole_level`]): a rim-only
/// polar cap measures `[v₀, +1]` or `[−1, v₀]`, and the claim that its
/// rim CLOSES around that pole is decided on `Δu`
/// ([`require_rim_only_closed`]). **What is still not established** is
/// that the domain has no notch away from the rims: rims at the
/// extremes and meridians between them is what the arm reads, and a
/// notch cut by an interior vertex chain is
/// `require_rims_at_extremes`' question, not this one.
///
/// **The two branches' arms are asked in the order the PARSE settles,
/// not in a priority order**, and nothing arbitrates between them: the
/// rim count does. A rim-free wedge and a rim-only cap are different
/// shapes and their guards are disjoint — the wedge and two-band arms
/// are inside `rims.is_empty()`, the pole fold requires a rim AND no
/// meridian — so no input reaches both and no ordering between them
/// can change an answer. What the order in the code DOES express is
/// each arm's relation to the extent: the pole fold runs FIRST because
/// it is the only step that MUTATES `b.levels`, so `min_max` and
/// `require_extent` must see the folded list or they would refuse the
/// cap they are about to admit; the rimless arms run AFTER
/// `require_extent` because they settle `Δu`, which the extent says
/// nothing about. A boundary with neither rim nor meridian reaches
/// neither and refuses on the empty level list.
///
/// **The flux side** is [`SphereFluxSide`]'s: the rimless face's is the
/// sense bit ([`SphereFluxSide::Sense`]'s doc is the one home of that
/// fact), the rimmed face's is [`linear_rim_side`]'s decided sign.
///
/// On the rimmed branch that decided sign is **no longer an
/// INDEPENDENT value**, and the reason belongs here because it is a
/// fact about this arm rather than about either type.
/// [`require_rim_interior_sides`] requires every rim's σ to point into
/// the folded extent, and σ is the rim's traversal under the sense
/// bit, so on every face that measures the boundary's reading and the
/// bit are equal — the `debug_assert` in the arm is that claim. What
/// is still different is the EVIDENCE: the rimmed branch DERIVES the
/// side and checks the bit against it, the rimless branch has nothing
/// to derive and carries the bit. Collapsing [`SphereFluxSide`] to the
/// bit would make the flux rest on a single encoding on both branches
/// and delete the derivation the premise is a check ON, so the two
/// stay different types up to the single term that consumes either.
/// Tier 3's check 6 compares the same two encodings one level out,
/// through [`boundary_material_sign`], which reads no bit at all.
fn sphere<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    edges: &[LoopEdge<T>],
    sense: bool,
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let (mut b, meridian_axes) = sphere_boundary(center, radius, axis, edges, band)?;
    // A boundary of rims alone can carry no extent of its own: every
    // level it touches is a rim latitude, and where those coincide the
    // face's missing extreme is the POLE its rims' traversals point
    // at ([`sphere_rim_only_pole_level`]).
    let folded_pole = !b.rims.is_empty()
        && meridian_axes.is_empty()
        && sphere_rim_only_pole_level(&mut b, sense, radius, band)?;
    let (du, side);
    let (lo, hi) = min_max(&b.levels)?;
    require_extent(sphere_extent_margin(lo, hi, radius), band)?;
    if b.rims.is_empty() {
        // Meridians only (the fn docs): the two-band face when every
        // carrier axis is coplanar with the first and the loop runs
        // the great circle once, Δu = π; the wedge when an axis is
        // definitely not coplanar.
        let Some((&first, rest)) = meridian_axes.split_first() else {
            return Err(PropsError::NotIsoRectangle {
                what: "sphere face with an empty boundary",
            });
        };
        let mut coplanar = true;
        for &n in rest {
            if classify(
                "props_band_coplanar",
                Margin::levered(n.cross(first).norm(), radius),
                band,
            )? != Sign::Zero
            {
                coplanar = false;
                break;
            }
        }
        du = if coplanar {
            require_band_opposite(edges, &meridian_axes, center, radius, band)?;
            T::pi()
        } else {
            sphere_wedge_azimuth(edges, &meridian_axes, center, radius, axis, sense, band)?
        };
        side = SphereFluxSide::Sense(sense);
    } else {
        // The iso-rectangle premise (S58/#649). Sphere rims carry the
        // `(sin v, cos v)` direction pair, so the scalar extremes are
        // lifted into the same representation (`as_level`) and metered
        // at the sphere radius.
        let s_f = linear_rim_side(&b, (lo, hi), band)?;
        // The premise the levels alone cannot state: every rim's
        // interior side points INTO `[lo, hi]`, so the extent the
        // levels folded is the face's own and not its complement's.
        require_rim_interior_sides(&b, (lo, hi), sense, band)?;
        // The premise makes the two encodings equal wherever both are
        // defined: a rim at `lo` needs σ = +1, i.e. `d_u_sign` = the
        // sense bit, and `s_f` IS `d_u_sign` there; at `hi` both
        // negate. A future weakening of the arm above shows up here
        // rather than as a silently wrong flux sign.
        debug_assert!(
            (s_f == Sign::Positive) == sense,
            "props_rim_interior_side makes the boundary-encoded side the face's sense bit"
        );
        side = SphereFluxSide::Rim(s_f);
        du = du_of_rims(&b.rims, b.arms, band)?;
        // A folded pole is a claim that the rim CLOSES around it, and
        // `Δu` is the only place that claim can be read.
        if folded_pole {
            require_rim_only_closed(du, radius, band)?;
        }
    }
    let area = radius.powi(2) * du * (hi - lo);
    let va = loop_vector_area(edges, center)?;
    let flux = side.signed(radius * area) + (center - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
}

/// Which way a sphere face's material faces, for the radial term of
/// its flux — the one quantity [`sphere`]'s two branches establish
/// from different evidence, kept apart so neither reads as the other.
///
/// **Not [`MaterialSign`], which forks on the same question one step
/// earlier.** That enum is what the BOUNDARY alone encodes, derived
/// without the face's bit precisely so tier 3 can compare the two
/// encodings against each other; this one is what the flux term
/// actually integrates, so where the boundary encodes nothing it
/// carries the bit rather than declining to answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SphereFluxSide {
    /// The rimless face, two-band or wedge: the face's `Face::sense`
    /// BIT is its flux side. **This is the one home of that fact.** It
    /// is the only flux sign in the props module that the boundary does
    /// not encode: with no rim there is no traversal to read a side off,
    /// and a rimless face's meridians are traversed the same way
    /// whichever side is material, so [`MaterialSign`] answers
    /// `Unencoded` for it and nothing cross-checks the bit. `Face::sense`
    /// (M5 S10) is exactly the missing bit — `true` where the chart
    /// normal already points out of the material — and an inward-facing
    /// rimless band is representable only through it. The wedge arm
    /// reads the same bit a second time, for WHICH azimuthal arc the
    /// face covers ([`sphere_wedge_azimuth`]): the same claim about the
    /// same bit, not a cross-check of it. A bit, not a `T` ±1, for the
    /// reason [`crate::enters::OutwardNormal::from_chart`]'s doc gives.
    Sense(bool),
    /// A rim-bearing face: the side the boundary itself encodes, as
    /// [`linear_rim_side`] decides it.
    Rim(Sign),
}

impl SphereFluxSide {
    /// The radial term `R·Area` signed by this side.
    ///
    /// [`Self::Rim`] carries a DEFINITE `Sign` — [`linear_rim_side`]
    /// states that and guards it — so [`t_sign`]'s `Zero => T::zero()`
    /// case, which is live for the torus's [`sign_mul`], is not a side
    /// this arm can hold and no rimmed face's radial term is metered
    /// as nothing.
    fn signed<T: Real>(self, radial: T) -> T {
        match self {
            Self::Sense(true) => radial,
            Self::Sense(false) => -radial,
            Self::Rim(s) => t_sign::<T>(s) * radial,
        }
    }
}

/// **The two-band face's loop runs its great circle once** — the
/// coplanar branch's second decide ([`sphere`]'s docs), asked after
/// `props_band_coplanar` has put every meridian on one great circle.
/// Coplanarity alone cannot tell OPPOSITE half-planes (the two-band
/// face, `Δu = π`) from COINCIDENT ones (two arcs on one half-plane,
/// there and back: a slit of no width, or the ball less a slit —
/// `Δu → 0` or `2π`); the loop's traversal can.
///
/// **`props_band_opposite`**: at every junction of the loop — arc `i`'s
/// traversal end and arc `i + 1`'s traversal start, the same vertex by
/// closure — the margin is the chord between the two unit traversal
/// tangents there, `Margin::levered(‖t_start − t_end‖, R)`: metres, the
/// distance between the two arcs' departure points scaled to the
/// sphere radius, at the run's linear band. A loop that goes AROUND
/// the circle continues through each junction (chord 0, `Zero`); a
/// loop that reverses there (a slit) turns back on itself (chord 2,
/// `Positive`). For two pole-to-pole arcs this is the chord between
/// the two meridians' equatorial departure directions; stated at the
/// junction it also holds for a great circle split at ordinary points
/// or into more than two arcs (CERT-1's rows), which no departure-pair
/// reading covers. `Zero` at every junction ⇒ the two-band face;
/// `Positive` ⇒ typed refusal; the ambiguity band escalates. The
/// traversal tangent of a stored arc at a point is `dP/dt = n × (P − c)`
/// for a forward traversal and its negation for a reversed one — the
/// forward bit and the carrier axis, nothing else.
///
/// # Errors
///
/// [`PropsError::NotIsoRectangle`] naming the slit, [`PropsError::Escalated`]
/// in the band.
fn require_band_opposite<T: Decide>(
    edges: &[LoopEdge<T>],
    meridian_axes: &[Vec3<T>],
    center: Point3<T>,
    radius: T,
    band: Band,
) -> Result<(), PropsError> {
    let inv_r = T::one() / radius;
    // Unit traversal tangents at an arc's traversal start and end.
    let tangents = |e: &LoopEdge<T>, n: Vec3<T>| {
        let (w0, w1) = (e.p0() - center, e.p1() - center);
        if e.forward {
            (n.cross(w0) * inv_r, n.cross(w1) * inv_r)
        } else {
            (w1.cross(n) * inv_r, w0.cross(n) * inv_r)
        }
    };
    let n = edges.len();
    for i in 0..n {
        let (_, t_end) = tangents(&edges[i], meridian_axes[i]);
        let (t_start, _) = tangents(&edges[(i + 1) % n], meridian_axes[(i + 1) % n]);
        if classify(
            "props_band_opposite",
            Margin::levered((t_start - t_end).norm(), radius),
            band,
        )? != Sign::Zero
        {
            return Err(PropsError::NotIsoRectangle {
                what: "a rimless sphere face whose coplanar meridians share one half-plane — \
                       a slit the flux lane does not measure",
            });
        }
    }
    Ok(())
}

/// **The wedge's azimuthal width, read from the loop's structure** —
/// the rimless arm's second branch ([`sphere`]'s docs), reached only
/// when `props_band_coplanar` has decided some meridian axis definitely
/// not parallel to the first.
///
/// **What the boundary states.** In a rimless parse every edge is a
/// meridian, so `meridian_axes[i]` is `edges[i]`'s carrier axis `n_i`
/// (the parse pushes one axis per meridian edge, in edge order). The
/// arm reads a TWO-edge boundary: one arc per meridian, each running
/// pole to pole — decided, not inherited, on the pole helper's own
/// margins ([`sphere_meridian_pole_margins`], `props_meridian_pole`
/// `Zero` at both poles: a forward span of at most one period with
/// both poles at its ends is one pole to the other). A meridian stated
/// in pieces is not folded here (the torus arm folds by lineage; the
/// sphere arm does not) and refuses under a `what` that says so. A
/// pole-to-pole arc lies in ONE half-plane of its great circle, and
/// that half-plane's direction is the stored parameterisation's
/// departure direction at its `t0` pole: `d_i = n_i × (P(t0) − c) / R`
/// — `dP/dt = n × (P − c)` on the stored circle, unit because the arc
/// is centred on the sphere at its radius (`props_meridian_great`) and
/// horizontal because `P(t0) − c` is along the axis at a pole.
///
/// **Which of the two azimuthal arcs the face covers is a structural
/// read of the loop, never a shorter-arc assumption and never a value
/// coincidence** — this paragraph is the derivation's one home.
/// Interior-left about the outward normal `N = ν·(P − c)/R` (`ν = ±1`
/// the face's sense bit) puts the face's interior at any point of arc
/// `A` along `N × T`, with `T` the traversal tangent
/// `f_A · n_A × (P − c)` (`f_A = ±1` the forward bit); the triple
/// `w × (n × w) = R²·n` for `w ⟂ n` collapses that to
///
/// ```text
/// I_A = ν · f_A · n_A
/// ```
///
/// — the carrier axis itself, signed by the sense bit and the forward
/// bit, and nothing else; the code is that expression, the sense bit
/// folded through [`crate::enters::OutwardNormal::from_chart`] (the one
/// door that may fold it — used here for its fold: the vector it
/// wraps is a carrier axis, not a normal, and is unwrapped at once).
/// The face covers the azimuthal arc that leaves `d_A` in the
/// direction `I_A` and ends at `d_B`, so with `(d_A, I_A)` an
/// orthonormal frame of the equatorial plane its width is the polar
/// angle of `d_B` in that frame, `φ = atan2(d_B · I_A, d_B · d_A)`,
/// and `Δu = φ` when `φ > 0` (the short arc between the planes) or
/// `φ + 2π` when `φ < 0` (the long one). Reversing the loop flips both
/// forward bits, flipping `I_A`, so the same two arcs traversed the
/// other way hand the face the complementary lune `2π − Δu`; flipping
/// the sense bit does the same. Loop closure (tier 1: `B` departs the
/// pole `A` arrives at) makes the read at `B` the negation of the read
/// at `A`, so one arc's read is the loop's and no second decide is
/// needed.
///
/// **`props_wedge_azimuth`** decides `Margin::levered(φ, R)` — metres:
/// the equatorial arc from meridian `A` into the face's interior to
/// meridian `B`, signed by whether that arc is the short one, at the
/// sphere radius as its lever, against the run's linear band.
/// `Positive` ⇒ `Δu = φ`; `Negative` ⇒ `Δu = φ + 2π`; `Zero` ⇒ the two
/// meridians coincide and the face refuses `DegenerateFace`, the
/// extent refusal's own disposition; the ambiguity band escalates
/// typed. **The `Zero` and indeterminate outcomes are this arm's D2
/// floor, structurally unreachable by the factor K**: the arm is
/// entered on `R·|sin φ| ≥ escalate = K·zero` (`props_band_coplanar`'s
/// margin, same band, same lever) and `|φ| ≥ |sin φ|`, so `R·|φ|` is
/// at least `K·zero` — K coincidence widths above the `Zero` edge and
/// on the definite side of its own band. The floor is kept as a typed
/// refusal because D2's inventory states every outcome of a decide;
/// it is not a rounding window. Coincident meridians themselves are
/// coplanar and are refused by [`require_band_opposite`].
///
/// **`atan2` here is the exception to the module's rule** (the props
/// module docs' stored-data discipline; [`sphere_meridian_pole_margins`]
/// refuses it because an arc anchored at a pole puts an interval
/// enclosure exactly on its branch cut). This angle's cut is
/// `d_B = −d_A` — the coplanar, opposite-half-plane pair — which
/// `props_band_coplanar` has just decided definitely NOT the case, so
/// every enclosure that reaches this call is bounded away from the cut
/// by at least the escalate width at the radius; the interval twin
/// (`bool5_wedge_arm.rs`) holds tight at every angle.
///
/// # Errors
///
/// [`PropsError::NotIsoRectangle`] naming the two-edge premise (a
/// meridian in pieces) or the pole-to-pole premise;
/// [`PropsError::DegenerateFace`] and [`PropsError::Escalated`] as
/// above.
fn sphere_wedge_azimuth<T: Decide>(
    edges: &[LoopEdge<T>],
    meridian_axes: &[Vec3<T>],
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    sense: bool,
    band: Band,
) -> Result<T, PropsError> {
    // The parse's own invariant: one axis per edge once no edge is a
    // rim. A mismatch cannot be reached by any input — the parse pushes
    // exactly one axis per edge it classified a meridian and refused
    // every other kind before returning — so it is a kernel bug, and a
    // kernel bug panics (D9): a typed refusal here would launder it
    // into a supported outcome, and a `zip` would truncate it silently.
    if edges.len() != meridian_axes.len() {
        unreachable!("a rimless sphere parse states one meridian axis per boundary edge");
    }
    let ([a, b], [n_a, n_b]) = (edges, meridian_axes) else {
        return Err(PropsError::NotIsoRectangle {
            what: "the wedge arm reads a two-edge boundary; a meridian in pieces is not folded \
                   on the sphere",
        });
    };
    // The premise, decided: each arc runs pole to pole — both poles at
    // its span's ends. The fold's own disposition on the same margin:
    // in-band and poisoned admit (near a span end the two readings
    // differ by ~band²; the extent fold has already taken the pole),
    // a definite Positive (pole inside the span) or Negative (pole
    // outside it) refuses.
    for (e, n) in [(a, *n_a), (b, *n_b)] {
        for (m, _) in sphere_meridian_pole_margins(e, center, radius, axis, n, band)? {
            if let Ok(Sign::Positive | Sign::Negative) =
                decide("props_meridian_pole", Margin::levered(m, radius), band)
            {
                return Err(PropsError::NotIsoRectangle {
                    what: "a rimless wedge's meridians run pole to pole",
                });
            }
        }
    }
    let inv_r = T::one() / radius;
    // Half-plane directions: the stored parameterisation's departure
    // direction at each arc's `t0` pole (fn docs).
    let d_a = n_a.cross(a.p0() - center) * inv_r;
    let d_b = n_b.cross(b.p0() - center) * inv_r;
    // `I_A = ν·f_A·n_A` (fn docs): the sense bit folded through the one
    // door that may fold it, the forward bit as the sign it is.
    let i_a = crate::enters::OutwardNormal::from_chart(*n_a, sense).vec()
        * if a.forward { T::one() } else { -T::one() };
    let phi = d_b.dot(i_a).atan2(d_b.dot(d_a));
    match classify("props_wedge_azimuth", Margin::levered(phi, radius), band)? {
        Sign::Positive => Ok(phi),
        Sign::Negative => Ok(phi + T::tau()),
        Sign::Zero => Err(PropsError::DegenerateFace),
    }
}

/// **Certification's per-edge span bounds, re-decided at the parse**
/// for a sphere meridian arc: its stored span is definitely forward
/// and does not definitely exceed one period. Certification enforces
/// `0 < Δt ≤ τ` on every edge — `interval_span_forward` (the span
/// `Δt·r`, definitely positive) and `interval_span_winding` (the
/// headroom `(τ − Δt)·r`, not definitely negative), both at the linear
/// band — and every span read in this module rests on that bound; the
/// pole fold in particular ([`sphere_meridian_pole_margins`]) is
/// stated for a forward span of at most one period and has no honest
/// answer outside it. A loop built without a body
/// ([`LoopEdge::hand_built`]) can state any span, so both halves are
/// decided here under their own names, with certification's margins,
/// band and lever, hence its dispositions. `props_meridian_span_forward`
/// admits only a definite `Positive` span and refuses `Zero` typed
/// (certification's `IntervalNotForward`); `props_meridian_span_winding`
/// admits `Zero` and `Positive` headroom (a span inside the coincidence
/// band above τ is one certification admits too) and refuses
/// definitely negative headroom typed; either decide's ambiguity band
/// escalates. The torus decides the same pair for a span it
/// reconstructs across pieces (`props_meridian_pieces_forward`,
/// `props_meridian_pieces_winding`, [`fold_chain`]); the sphere has no
/// fold and decides them per edge.
///
/// [`sphere_meridian_pole_margins`] runs this before it forms a margin,
/// so no consumer of the pole arithmetic — [`sphere_boundary`]'s fold
/// or [`require_one_chart_branch`]'s refusal — reaches it on an
/// undecided span. Rims are not decided here: a rim's span feeds the
/// `Δu` sum, a different premise.
fn require_meridian_span_within_period<T: Decide>(
    e: &LoopEdge<T>,
    radius: T,
    band: Band,
) -> Result<(), PropsError> {
    let dt = e.t1 - e.t0;
    if classify(
        "props_meridian_span_forward",
        Margin::levered(dt, radius),
        band,
    )? != Sign::Positive
    {
        return Err(PropsError::NotIsoRectangle {
            what: "props_meridian_span_forward",
        });
    }
    if classify(
        "props_meridian_span_winding",
        Margin::levered(T::tau() - dt, radius),
        band,
    )? == Sign::Negative
    {
        return Err(PropsError::NotIsoRectangle {
            what: "props_meridian_span_winding",
        });
    }
    Ok(())
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
/// any margin is formed** ([`require_meridian_span_within_period`],
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
    require_meridian_span_within_period(e, radius, band)?;
    let w0 = e.p0() - center;
    let sa = w0.dot(axis) / radius;
    let ca = n_c.cross(w0).dot(axis) / radius;
    // powi(2), not x*x (the one argument lives at `level_gap`).
    let r0 = (sa.powi(2) + ca.powi(2)).sqrt();
    let dt = e.t1 - e.t0;
    // The north pole (λ = +r0) sits at the span-relative direction
    // `(sa, ca)` on the parameter circle; the south pole (λ = −r0)
    // at its antipode. Everything below is direction arithmetic — no
    // `atan2`, no range reduction: an angle extraction is wide at its
    // branch cut (an arc anchored at a pole put an interval enclosure
    // exactly there, live on the die-fillet corpus), and a mod-2π
    // `floor` spans its integer step at a period boundary; either
    // widens the margin to the whole period and forces an escalation
    // the scalar lane does not have. (The one `atan2` in this module,
    // `sphere_wedge_azimuth`'s, is safe for the reason its doc gives:
    // its cut is a pair the coplanar decide has already excluded.)
    let half = T::from_f64(0.5);
    // The membership EDGE is the half-span's cosine: the span was
    // decided above, so `dt/2` is at most a half-turn plus the
    // coincidence band.
    let (sd2, cd2) = (dt * half).sin_cos();
    let (sdt, cdt) = dt.sin_cos();
    Ok([(sa, ca, r0), (-sa, -ca, -r0)].map(|(ps, pc, extreme)| {
        // Sign: the pole lies in the closed span iff its direction is
        // within `dt/2` of the span's midpoint direction — one dot
        // test, `⟨P, M⟩ − cos(dt/2)`, whose zero set on the circle is
        // exactly the two span endpoints.
        let f = ps * cd2 + pc * sd2 - cd2;
        // Magnitude: the CHORD to the nearer span endpoint — levered
        // by R below, that is the point deviation of moving the pole
        // onto the span boundary. powi(2), not x*x (see `level_gap`).
        let chord_a = ((ps - T::one()).powi(2) + pc.powi(2)).sqrt();
        let chord_b = ((ps - cdt).powi(2) + (pc - sdt).powi(2)).sqrt();
        // `copysign` transfers the membership sign onto the chord; at
        // an interval scalar a sign enclosure straddling zero yields
        // the two-sided hull `±chord`, which is tight exactly where
        // it happens — the pole at a span endpoint, chord ≈ 0.
        let m = chord_a.min(chord_b).copysign(f);
        (m, extreme)
    }))
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
    levels: &mut Vec<T>,
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
            Ok(Sign::Positive | Sign::Zero) | Err(_) => levels.push(extreme),
            Ok(Sign::Negative) => {}
        }
    }
    Ok(())
}

/// **σ — which side of its rim the face's interior lies on**, read
/// off stored data alone: `Positive` ⇔ the interior lies toward `+v`
/// (the `+axis` pole).
///
/// **One rule, two collapses, and this is the rim's.** The boundary is
/// traversed with the material on the LEFT of the traversal tangent
/// `T` as seen along the face's outward normal `N`, i.e. toward
/// `N × T`, and `N` is `n_chart` under the face's sense bit —
/// `+n_chart` when set, `−n_chart` when not, which is exactly
/// [`crate::enters::OutwardNormal::from_chart`]'s fold. On a sphere
/// `n_chart = ∂u × ∂v` is the outward radial `r̂` (the fn docs'
/// `(p − c)·n_chart = R`), so `(û, v̂, r̂)` is right-handed and
/// `r̂ × û = v̂`. For a RIM, `T = d_u_sign·û`, so
/// `N × T = ν·d_u_sign·v̂`: the interior lies toward `+v` exactly when
/// the rim's traversal direction and the sense bit agree, which is
/// what the code computes.
///
/// [`sphere_wedge_azimuth`] takes the SAME rule down the other
/// collapse — for a MERIDIAN, `T = f·n×(P−c)` and the triple product
/// gives `I = ν·f·n`, the carrier axis under the same two bits — and
/// spells the fold as the door because its quantity is a `Vec3` the
/// door can wrap. This one's is a discrete [`Sign`] naming a side in
/// `v`; it never holds a normal, so there is nothing for the door to
/// guard and the obligation is discharged by deriving it here from the
/// same `N × T`. Both read `ν` as `+1` under a set bit, which is
/// `from_chart`'s own arm.
///
/// **This is a per-rim, per-face fact and must stay one.** Every
/// factor is this rim's stored direction or the face's own bit: no
/// other rim is read, so the same body under two loop anchorings
/// yields the same σ on the same rim, and the verdict
/// [`require_rim_interior_sides`] records with it is a fact about the
/// face rather than about where a cycle happens to start. The two
/// predicates that are NOT — `props_rim_side` (whichever rim
/// [`linear_rim_side`] meets first) and, until it was retired,
/// `props_rim_dir_group` — are why that sentence is here: a margin for
/// this sign taken against a reference rim would mint a third.
///
/// **Sphere-only, and that is a claim about the OTHER kinds' faces,
/// not about the geometry.** The derivation is chart-generic — it is
/// the interior-left rule at any `∂u × ∂v` — but the premise it
/// underwrites is the one whose answer the sense BIT settles: a rim of
/// a ball bounds the cap under one bit and the ball minus it under the
/// other, two valid solids sharing a whole boundary. The cone's
/// rim-only face is the shape without the ambiguity — its missing
/// extreme is the apex whichever way the rim runs
/// ([`cone_apex_level`]), so what the two traversals differ in is a
/// SIDE, read bit-free by [`linear_rim_side`] and checked against the
/// bit at tier 3's check 6. A cylinder reaches neither: its rim-only
/// face has no extent to name at all
/// (`a_cylinder_rim_only_face_is_extent_less`).
fn rim_interior_side<T: Real>(rim: &Rim<T>, sense: bool) -> Sign {
    sign_mul(
        rim.d_u_sign,
        if sense {
            Sign::Positive
        } else {
            Sign::Negative
        },
    )
}

/// **Every rim's interior side points INTO the face's level range.**
///
/// The extent `[lo, hi]` says where the face's levels reach; σ
/// ([`rim_interior_side`]) says, per rim, which way the material lies
/// from that rim. On an iso-rectangle the two agree by construction —
/// a rim at `lo` has its interior above it, a rim at `hi` below — and
/// where they disagree the rectangle premise is not established: the
/// face's material extends PAST the extent the levels folded, so the
/// closed form would integrate over a domain the face does not have.
/// That is the L-shaped complement of a half-cap, whose rim sits at
/// `lo` while its traversal says the interior lies below: it shares
/// both edges with the half-cap, parses to the same levels, and its
/// flux cancelled the half-cap's on a closed sphere.
///
/// Decided per rim through the funnel, on [`rim_offset_margin`]'s
/// comparand pointed by σ — so `Positive` is agreement and `Negative`
/// is the refusal.
///
/// **Every rim is decided before any refusal is returned**, and that
/// ordering is the point rather than a style: returning at the first
/// `Negative` would record one verdict for a face whose refusing rim
/// comes first and two for the same face anchored the other way, which
/// would make the RECORDED POPULATION a fact about cycle order even
/// though each sign in it is not. The refusal that is returned is the
/// first in traversal order, which is a choice among equals — every
/// one of them refuses the same face by the same name.
///
/// **`Zero` is subsumed, not handled.** It would be a rim in the
/// middle of its own extent, and the two premises that run before this
/// exclude it: [`require_extent`] has already answered `hi − lo`
/// definitely positive (so `hi − lo > escalate`), and `props_rim_level`
/// has already placed every rim within `zero` of `lo` or of `hi`, so
/// `|lo + hi − 2v| ≥ hi − lo − 2·zero > 0`. The arm stays total rather
/// than panicking (D9) and answers the refusal its neighbour
/// [`rim_side`] answers for the same shape.
///
/// Called only after [`linear_rim_side`], so the rims are already
/// known to sit at the extremes: a face that fails the shape premise
/// refuses by `props_rim_level`, which names the defect better.
fn require_rim_interior_sides<T: Decide>(
    b: &LinearBoundary<T>,
    (lo, hi): (T, T),
    sense: bool,
    band: Band,
) -> Result<(), PropsError> {
    let mut refusal = None;
    for rim in &b.rims {
        let sigma = rim_interior_side(rim, sense);
        let out = match classify(
            "props_rim_interior_side",
            rim_offset_margin(rim.level, lo, hi, b.arms, sigma),
            band,
        )? {
            Sign::Positive => None,
            Sign::Negative => Some(PropsError::NotIsoRectangle {
                what: "props_rim_interior_side",
            }),
            Sign::Zero => Some(PropsError::DegenerateFace),
        };
        refusal = refusal.or(out);
    }
    refusal.map_or(Ok(()), Err)
}

/// **The pole a rim-only boundary contains, pushed into its levels.**
///
/// A sphere face bounded by rims and nothing else touches exactly the
/// latitudes its rims sit at. Where those are one latitude the levels
/// hold no extent — one entry, `lo == hi`, [`require_extent`]'s
/// `DegenerateFace` — for a face that is not degenerate: a ball cut by
/// one plane is bounded by one rim circle and the pole is interior to
/// it. The extreme the levels are missing is that pole, and the fact
/// that names it is the one a rim's TRAVERSAL carries on its own,
/// which no level does: σ, the side of the rim the material lies on
/// ([`rim_interior_side`]). `σ·1` is the latitude sine of the pole on
/// that side, so the cap's extent folds to `[v₀, +1]` or `[−1, v₀]`
/// and `R²·Δu·(sin v_hi − sin v_lo)` measures it, with `Δu = 2π` from
/// the rim spans [`du_of_rims`] already sums.
///
/// **Three inputs keep their old answers, and each is a different
/// shape.** A boundary whose rim levels DO span an extent — the
/// spherical zone between two rim circles — is left alone, which is
/// what `props_rim_only_extent` decides. A boundary with a meridian is
/// never offered here: its levels speak for themselves, arc spans
/// folded in. And rims at ONE level whose traversals disagree point at
/// two opposite poles, which is no pole at all: the levels stay
/// silent and `require_extent` refuses `DegenerateFace` exactly as
/// before — the zero-extent patch M2 pinned, two rims at one level
/// joined by zero-length meridians, keeps its verdict.
///
/// Unanimity is a property of the SET of σ, not of an order: equality
/// is symmetric, so "every rim agrees with some rim" is the same
/// statement whichever rim the loop walk hands over first, and no
/// verdict is recorded for it.
fn sphere_rim_only_pole_level<T: Decide>(
    b: &mut LinearBoundary<T>,
    sense: bool,
    radius: T,
    band: Band,
) -> Result<bool, PropsError> {
    let (lo, hi) = min_max(&b.levels)?;
    if classify(
        "props_rim_only_extent",
        sphere_extent_margin(lo, hi, radius),
        band,
    )? != Sign::Zero
    {
        return Ok(false);
    }
    let mut sides = b.rims.iter().map(|rim| rim_interior_side(rim, sense));
    let Some(sigma) = sides.next() else {
        return Ok(false);
    };
    if !sides.all(|s| s == sigma) {
        return Ok(false);
    }
    b.levels.push(t_sign::<T>(sigma));
    Ok(true)
}

/// **A folded extreme is interior to a rim only if the rim CLOSES
/// around it** (`props_rim_only_closed`), decided on the `Δu` the rim
/// spans sum to — the sphere's pole ([`sphere_rim_only_pole_level`])
/// and the cone's apex ([`cone_apex_level`]) alike, since the two
/// folds share the defect exactly.
///
/// [`sphere_rim_only_pole_level`] reads a traversal DIRECTION, which
/// says which side of the rim the material is on and nothing about how
/// far the rim goes. Everything else on the arm is silent about that
/// too: the levels hold one latitude however much of it the boundary
/// states, `props_rim_level` places a rim at an extreme whatever its
/// span, and [`du_of_rims`] SUMS the spans of one (level, direction)
/// group without ever comparing the sum to a whole turn. So the fold's
/// premise — that the face is a cap and `Δu = 2π` — has to be decided,
/// and until it was, four shapes that all refused `DegenerateFace`
/// before the fold answered a definite area through the public door:
/// a lone half rim at half the cap's, a quarter rim at a quarter, the
/// same full rim stated twice at double, and a full rim with an extra
/// half arc at 1.5×.
///
/// The comparand is `(Δu − τ)` at the kind's azimuthal arm
/// ([`RimArms::azimuth`] — the sphere's `R`, the cone's own rim
/// radius), the arc the rim fails to close by: a length, like every
/// other margin here. It is asked ONLY where an extreme was folded: a
/// face whose levels carry their own extent states its `u`-domain the
/// way every other face does, and `props_du_consistent` is what bounds
/// it there.
///
/// **On the cone it is the only guard the public door has.** The
/// sphere's two traversals are two valid faces (the cap and the ball
/// minus it), which σ tells apart; the cone's other traversal bounds
/// the rest of the nappe, which runs to infinity and is no finite face
/// of any solid — and `fn cone` takes no sense bit to tell them apart
/// with, because a cone's flux needs no material side (generators run
/// through the apex, so the anchored term vanishes). What catches that
/// face is tier 3's check 6, against the `Encoded` side
/// [`boundary_material_sign`]'s cone arm reads off the same folded
/// extent.
///
/// **The rimless branch's sibling is [`require_band_opposite`]**
/// (`props_band_opposite`), and the two can never both fire: this one
/// needs a rim and no meridian, that one needs no rim at all, and the
/// parse settles which before either is asked. They are the same KIND
/// of premise on the two branches — *the loop actually goes round
/// once* — reached independently, and each catches the shape its own
/// branch's other premises are blind to: a coplanar rimless loop that
/// doubles back on one half-plane (a slit) and a rim-only loop whose
/// spans are a part or a multiple of a turn.
fn require_rim_only_closed<T: Decide>(du: T, arm: T, band: Band) -> Result<(), PropsError> {
    require_zero(
        "props_rim_only_closed",
        Margin::levered(du - T::tau(), arm),
        band,
    )
}

/// Classify a sphere face's boundary into (rims, meridian great-circle
/// axes, latitude-sine levels) — the shared parse consumed by both the
/// flux closed form and [`boundary_material_sign`].
#[allow(clippy::type_complexity)]
fn sphere_boundary<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<(LinearBoundary<T>, Vec<Vec3<T>>), PropsError> {
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut meridian_axes: Vec<Vec3<T>> = Vec::new();
    let mut levels: Vec<T> = Vec::new();
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
        )?;
        match s {
            Sign::Positive | Sign::Negative => {
                let w = c_c - center;
                require_zero(
                    "props_rim_fit",
                    Margin::of((w.norm_squared() + r_c.powi(2)).sqrt() - radius),
                    band,
                )?;
                // Incidence: the fit above only fixes ‖w‖; the offset
                // must also point ALONG the axis (w ∥ â) with the
                // carrier axis parallel — together they place the
                // circle on the sphere as the iso-v rim.
                require_rim_incidence(w, n_c, r_c, axis, band)?;
                let sin_v = w.dot(axis) / radius;
                let cos_v = r_c / radius;
                rims.push(Rim {
                    d_u_sign: rim_dir(s, e.forward),
                    dt: e.t1 - e.t0,
                    // Dimensionless latitude DIRECTION pair, both
                    // components from stored data (`w·â/R`, `r_c/R`).
                    // The chord between two of these is the geodesic
                    // separation at R everywhere on the sphere; the
                    // axial component alone shrinks by `cos v̄` toward
                    // the poles and merges distinct near-polar rims.
                    level: RimLevel::Unit(sin_v, cos_v),
                    tags: (e.start, e.end),
                });
                levels.push(sin_v);
            }
            Sign::Zero => {
                // Meridian great circle: centered at the sphere center
                // with the sphere's radius.
                require_zero(
                    "props_meridian_great",
                    Margin::of((c_c - center).norm().max((r_c - radius).abs())),
                    band,
                )?;
                meridian_axes.push(n_c);
                levels.push((e.p0() - center).dot(axis) / radius);
                levels.push((e.p1() - center).dot(axis) / radius);
                // The arc's extent is its stored span's, not its
                // endpoints': fold in the pole latitude(s) the span
                // contains (see `sphere_meridian_span_levels`); the
                // pole helper decides first that the span is one the
                // fold may read.
                sphere_meridian_span_levels(e, center, radius, axis, n_c, &mut levels, band)?;
            }
        }
    }
    // Sphere levels are latitude SINES; the rims carry the full
    // `(sin v, cos v)` direction pair, so the lift completes a scalar
    // extreme with its cosine (latitudes live in `[−π/2, π/2]`, so
    // the cosine is the nonnegative root; the `max` keeps the sqrt
    // in-domain when a folded extreme sits a rounding past ±1).
    // Metered at the sphere radius, which is also its azimuthal arm.
    Ok((
        LinearBoundary {
            rims,
            levels,
            arms: RimArms::uniform(radius),
            as_level: |s| RimLevel::Unit(s, (T::one() - s.powi(2)).max(T::zero()).sqrt()),
        },
        meridian_axes,
    ))
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
/// meridian arc ([`sphere_meridian_span_levels`]). One home, because
/// the flux lane and [`boundary_material_sign`] both need it and a
/// face's extremes are not a thing two callers may each decide.
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

/// Torus face: rims are circles with axis ∥ the torus axis at minor
/// angle `v` (`sin v = ((C − c)·axis)/r`, `cos v = (r_c − R)/r`);
/// meridians are minor circles (radius `r`, center on the tube center
/// circle, carrier axis ⊥ the torus axis). The minor angle is
/// periodic, so the face's `v`-interval comes from a meridian's
/// **stored parameter span** plus its orientation relative to the
/// chart (`dv/dt = −sign(n_c·τ̂)` with `τ̂ = axis × ρ̂` at the minor
/// center) — never from endpoint `atan2`. With `[v0, v1]` the
/// increasing interval (`Δv = v1 − v0`, `s_i = sin v_i`,
/// `c_i = cos v_i`):
///
/// ```text
/// Area = r·Δu·[R·Δv + r·(s1 − s0)]
/// ∮(p−c)·n_chart dA
///   = r·Δu·[(R²+r²)(s1−s0) + R·r·Δv + (R·r/2)(Δv + s1·c1 − s0·c0)]
/// ```
///
/// (from `(p−c)·n_chart = R·cos v + r` and the Jacobian
/// `r·(R + r·cos v)`). `s_f` uses the rim topologically adjacent to
/// the anchor meridian's `t0` endpoint: the interior lies from that
/// rim in the direction `dv/dt` sweeps.
fn torus<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let p = torus_parse(center, axis, major, minor, edges, band)?;
    require_extent(Margin::levered(p.anchor.dt, minor), band)?;
    let dv = p.anchor.dt;
    // The iso-rectangle premise (S58/#649) — the SAME predicate the
    // other three kinds run, which is where it came from: this arm was
    // the only one #649's adversarial probe could not break, and
    // generalising it is the fix.
    let arms = torus_arms(major, minor);
    let (s0, c0, s1, c1) = torus_rims_at_extremes(&p, center, axis, major, minor, band)?;
    let du = du_of_rims(&p.rims, arms, band)?;
    // s_f: the rim topologically adjacent to the anchor endpoint; the
    // interior sweeps from it in the `dv/dt = −orient` direction.
    let rim_a = torus_anchor_rim(&p.rims, &p.anchor)?;
    let s_f = t_sign::<T>(sign_mul(rim_a.d_u_sign, p.orient.flip()));
    let half = T::from_f64(0.5);
    let area = minor * du * (major * dv + minor * (s1 - s0));
    let k = minor
        * du
        * ((major.powi(2) + minor.powi(2)) * (s1 - s0)
            + major * minor * dv
            + (major * minor * half) * (dv + s1 * c1 - s0 * c0));
    let va = loop_vector_area(edges, center)?;
    let flux = s_f * k + (center - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
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
    anchor_tag: u32,
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
                // carrying one routes to the quadrature lane first,
                // which refuses at its chart gate (`props.rs`,
                // `cut_face_rounds`: only the cylinder chart has a
                // lane) — and a stored pcurve cache does not change
                // that. The spiric quadrature lane is the spiric unit's
                // props PR.
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
                )?;
                require_rim_incidence(c_c - center, n_c, r_c, axis, band)?;
                classified.push(TorusEdge::Rim(Rim {
                    d_u_sign: rim_dir(s, e.forward),
                    dt: e.t1 - e.t0,
                    // Dimensionless minor-angle direction pair.
                    level: RimLevel::Unit(sin_v, cos_v),
                    tags: (e.start, e.end),
                }));
            }
            Sign::Zero => {
                let w = c_c - center;
                let h = w.dot(axis);
                let rho = (w - axis * h).norm();
                require_zero(
                    "props_meridian_fit",
                    Margin::of((rho - major).abs().max(h.abs()).max((r_c - minor).abs())),
                    band,
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
                )?;
                classified.push(TorusEdge::Arc(TorusArc { edge: e, n_c, c_c }));
            }
        }
    }
    fold_torus_meridians(classified, minor, band)
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
/// admitted. A loop of arcs that all continue one another (one closed
/// minor circle in pieces, no rim) has no chain boundary and refuses
/// by one name whatever rotation it arrives in; every other loop is
/// walked from an edge no chain continues into, so no chain is cut.
///
/// The folded interval is `[lowest t0, highest t1]` over the chain —
/// on one parametrisation, the original edge's own stored interval,
/// bitwise, whatever the split fractions — and the anchor is the arc
/// at its `t0` end, so a meridian carried by one edge folds to exactly
/// the record that edge produces alone.
fn fold_torus_meridians<T: Decide>(
    mut edges: Vec<TorusEdge<'_, T>>,
    minor: T,
    band: Band,
) -> Result<TorusParts<T>, PropsError> {
    fn same_edge<T: Real>(a: &TorusArc<'_, T>, b: &TorusArc<'_, T>) -> bool {
        a.edge.forward == b.edge.forward
            && matches!((a.edge.carrier_id, b.edge.carrier_id), (Some(x), Some(y)) if x == y)
    }
    let n = edges.len();
    if n == 0 {
        return Ok((Vec::new(), Vec::new()));
    }
    let Some(start) = (0..n).find(|&i| match (&edges[(i + n - 1) % n], &edges[i]) {
        (TorusEdge::Arc(a), TorusEdge::Arc(b)) => !same_edge(a, b),
        _ => true,
    }) else {
        return Err(PropsError::NotIsoRectangle {
            what: "torus meridian pieces close a loop with no rim",
        });
    };
    edges.rotate_left(start);
    let mut rims = Vec::new();
    let mut meridians = Vec::new();
    let mut chain: Vec<TorusArc<'_, T>> = Vec::new();
    for e in edges {
        match e {
            TorusEdge::Rim(r) => {
                if !chain.is_empty() {
                    meridians.push(fold_chain(core::mem::take(&mut chain), minor, band)?);
                }
                rims.push(r);
            }
            TorusEdge::Arc(a) => {
                if chain.last().is_some_and(|last| !same_edge(last, &a)) {
                    meridians.push(fold_chain(core::mem::take(&mut chain), minor, band)?);
                }
                chain.push(a);
            }
        }
    }
    if !chain.is_empty() {
        meridians.push(fold_chain(chain, minor, band)?);
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
    match Band::new(f64::from_bits(1), f64::from_bits(2)) {
        Ok(band) => band,
        // Two finite, ordered, positive constants: a `BandError` from
        // them is a kernel bug, not a state (D2 addendum row 4).
        Err(_) => unreachable!("the exact-order band's constants are valid by construction"),
    }
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
/// arc inside its pole helper ([`require_meridian_span_within_period`]),
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
/// A single edge is its own certified interval and re-decides nothing.
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
        if classify(
            "props_meridian_pieces_forward",
            Margin::levered(dt, minor),
            band,
        )? != Sign::Positive
        {
            return Err(PropsError::NotIsoRectangle {
                what: "props_meridian_pieces_forward",
            });
        }
        if classify(
            "props_meridian_pieces_winding",
            Margin::levered(T::tau() - dt, minor),
            band,
        )? == Sign::Negative
        {
            return Err(PropsError::NotIsoRectangle {
                what: "props_meridian_pieces_winding",
            });
        }
    }
    Ok(TorusMeridian {
        n_c: lo.n_c,
        c_c: lo.c_c,
        dt,
        anchor: lo.edge.p0(),
        anchor_tag: lo.edge.tag_at_t0(),
    })
}

/// Chart orientation of the anchor meridian: `v` winds right-handed
/// about `−τ̂` at its minor center, so `dv/dt = −orient`. Definite by
/// construction (the `Zero` arm refuses typed).
fn torus_meridian_orient<T: Decide>(
    m0: &TorusMeridian<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    minor: T,
    band: Band,
) -> Result<Sign, PropsError> {
    let w = m0.c_c - center;
    let rho_hat = (w - axis * w.dot(axis)).normalize();
    let tau = axis.cross(rho_hat);
    let orient = classify(
        "props_meridian_orient",
        Margin::levered(m0.n_c.dot(tau), minor),
        band,
    )?;
    if orient == Sign::Zero {
        return Err(PropsError::NotIsoRectangle {
            what: "torus meridian orientation degenerate",
        });
    }
    Ok(orient)
}

/// The rim topologically adjacent to the anchor meridian's `t0`
/// endpoint — the rim the torus `s_f` derivation reads (the interior
/// sweeps from it in the `dv/dt` direction).
fn torus_anchor_rim<'a, T: Real>(
    rims: &'a [Rim<T>],
    m0: &TorusMeridian<T>,
) -> Result<&'a Rim<T>, PropsError> {
    rims.iter()
        .find(|r| r.tags.0 == m0.anchor_tag || r.tags.1 == m0.anchor_tag)
        .ok_or(PropsError::NotIsoRectangle {
            what: "torus meridian anchor not on a rim",
        })
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
        let got = level_coincides(
            "props_rim_level_group",
            RimLevel::Length(1.0_f64),
            RimLevel::Unit(0.5, 0.5),
            None,
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
            dt: 1.0,
            level: RimLevel::Unit(0.5, 0.5),
            tags: (0, 1),
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

    /// **The metric is the chord, and the two call sites share it**
    /// (S81). A `Unit` pair whose two components are each inside the
    /// coincidence band but whose CHORD is not must not be grouped —
    /// the grouping site decided the components separately and answered
    /// "same level" where the predicate answered "not at an extreme".
    ///
    /// **The chord can only ever be `√2` × a component**, and the
    /// escalation multiplier K is larger than that on any sane run, so
    /// the honest outcome here is a **refusal, not a definite
    /// disagreement**: the pair lands in the ambiguity band and
    /// `classify` escalates. Both are `!Ok(true)`, and that — *the
    /// component rule's answer is not this rule's answer* — is what the
    /// row asserts, so it holds at every ε and every K rather than at
    /// the one it was first written against.
    ///
    /// The offsets come from the run's own band, never from a literal.
    /// A literal here passed at ε = 1e-9 for the wrong reason (both
    /// components were outside the band too) and failed at ε = 1e-6.
    #[test]
    fn a_pair_the_component_rule_calls_one_level_is_not_grouped() {
        let band = Band::linear(Tol::witness()).expect("band");
        let arms = RimArms::uniform(1.0_f64);
        // Each component 0.8·zero (inside), chord 1.13·zero (outside).
        let d = band.zero() * 0.8;
        let a = RimLevel::Unit(0.0, 0.0);
        let got = level_coincides(
            "props_rim_level_group",
            a,
            RimLevel::Unit(d, d),
            None,
            arms,
            band,
        );
        assert!(
            !matches!(got, Ok(true)),
            "components inside the band but the chord outside it is not one level: {got:?}"
        );
        let rim = Rim {
            d_u_sign: Sign::Positive,
            dt: 1.0,
            level: RimLevel::Unit(d, d),
            tags: (0, 1),
        };
        assert!(
            require_rims_at_extremes(
                std::slice::from_ref(&rim),
                (a, RimLevel::Unit(1.0, 0.0)),
                arms,
                band
            )
            .is_err(),
            "and the predicate must agree with the grouping, not differ from it"
        );
    }

    /// The floor for the row above: a pair whose CHORD is inside the
    /// band **is** one level, so "share the chord rule" is not "refuse
    /// everything". `0.1·zero` per component puts the chord at
    /// `0.14·zero`, inside at every ε and every K.
    #[test]
    fn a_pair_inside_the_band_by_its_chord_is_one_level() {
        let band = Band::linear(Tol::witness()).expect("band");
        let d = band.zero() * 0.1;
        assert!(
            level_coincides(
                "props_rim_level_group",
                RimLevel::Unit(0.0, 0.0),
                RimLevel::Unit(d, d),
                None,
                RimArms::uniform(1.0_f64),
                band,
            )
            .expect("decides"),
            "a pair inside the band by its chord is one level"
        );
    }
}
