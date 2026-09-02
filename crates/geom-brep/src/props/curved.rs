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
//! measure a pole-crossing arc EXACTLY (four rows of
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
use geom_core::spline::SpanLocate;
use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use super::{FaceContribution, LoopEdge, PropsError, loop_vector_area};
use crate::dihedral::decide;

/// The flux and area of a curved face from its **outer** loop (curved
/// M2 faces carry no rings — the owning body refuses ringed curved
/// faces before calling). Dispatches on the surface kind; `band` is
/// the run's linear band, built once at operation entry.
///
/// `sense_sign` is the face's `±1` orientation sense (M5 S10,
/// `topo::Face::sense_sign`). It is **deliberately not applied to
/// every term**: `A⃗` and the rim-derived `s_f` are recovered from the
/// face's STORED LOOP TRAVERSAL, which the interior-left rule already
/// ties to the outward normal — `revert` reverses loops and flips
/// `sense` together, so multiplying those terms by the sense would
/// double-count and negate the volume twice. `sense_sign` is consumed
/// at exactly one place: the **rimless** sphere band, whose boundary
/// carries no rim to derive `s_f` from and which previously hardcoded
/// `s_f = +1` on the assumption that sweeps emit outward shells only.
/// That is the one orientation fact in this module the boundary does
/// not encode, so it is the one the bit must supply.
///
/// # Errors
///
/// [`PropsError`] — unimplemented kinds, out-of-inventory boundary
/// shapes, definite consistency failures, escalated classifications.
pub fn curved_face<T: Decide>(
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    sense_sign: T,
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
        } => sphere(center, radius, axis, outer, sense_sign, band),
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
    /// The boundary does not encode the side: the **rimless sphere
    /// band** (M2 PR 5's axis-touching full revolve), the one analytic
    /// face whose flux sign the boundary cannot supply — its `s_f` IS
    /// `Face::sense`, a single encoding with nothing to cross-check
    /// against (the documented residual of the curved sense gate).
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
            let b = cone_boundary(apex, axis, sin_a, cos_a, outer, band)?;
            let (lo, hi) = min_max(&b.levels)?;
            Ok(MaterialSign::Encoded(linear_rim_side(&b, (lo, hi), band)?))
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => {
            let (b, _meridian_axes) = sphere_boundary(center, radius, axis, outer, band)?;
            if b.rims.is_empty() {
                return Ok(MaterialSign::Unencoded);
            }
            let (lo, hi) = min_max(&b.levels)?;
            Ok(MaterialSign::Encoded(linear_rim_side(&b, (lo, hi), band)?))
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
/// on top of the same per-kind boundary classification the flux lane
/// and [`boundary_material_sign`] parse with, and by nothing else: no
/// flux, no area, no material side. A consumer whose own lane rests on
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
/// **Carrier membership is not arc membership, and this door does not
/// decide the latter** (issue 1571). A certified meridian CARRIER can
/// carry an arc that leaves one chart meridian: a great circle
/// contains both poles, so a sphere meridian arc may cross a pole
/// mid-edge, where the chart's `u` jumps by π. The parse folds that
/// pole into the face's extent (the closed form is right about the
/// area) and says nothing about the arc's chart image; a consumer
/// whose walk assumes each edge stays on one iso curve — `mesh`'s —
/// still inherits that premise rather than receiving it from here.
///
/// **It inherits the extent derivations, and says so.** The door
/// decides shape from rim structure against the extremes each kind
/// derives; where a derivation mis-read the extent, the answer would
/// be a false "not at an extreme", not a shape verdict. Each kind
/// derives its extremes from spans certification bounded per edge:
/// the linear kinds' are `min_max` over endpoint levels, the sphere's
/// fold each arc's pole extremes in, and the torus's is its anchor
/// meridian's whole stored span — the pieces of a split edge folded
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
/// this door says so. [`curved_face`] refuses the same face unless its
/// meridians are coplanar (`props_band_coplanar`) — that is the closed
/// form's own premise, `Δu = π`, which the flux arm needs in order to
/// integrate and which says nothing about the shape. Two questions,
/// two homes: a consumer of this door meshes a partial sphere wedge,
/// the flux lane finds its volume uncomputable, and both are right.
///
/// **A zero-extent face PASSES too.** Two rims at one level joined by
/// zero-length meridians have every rim at an extreme (`lo == hi`);
/// the flux lane refuses `DegenerateFace` at its own `require_extent`
/// first. Extent is not a shape question — a degenerate rectangle is a
/// rectangle — so this door does not ask it: a consumer that cannot
/// mesh a zero-area face refuses it on its own terms (the walk's and
/// the CDT's), and one that can is not told otherwise by a shape
/// predicate. Pinned beside the lune in `tests/r2_mesh7_door_probes`.
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
/// [`PropsError::Unimplemented`] for a NURBS carrier or surface. The
/// recourse for a genuinely notched iso domain is the
/// certified-quadrature lane (D2 addendum row 2: valid input, lane not
/// built).
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
/// the_half_cap_exactly`, `the_rimless_hemisphere_split_off_its_poles_
/// still_measures` and `a_multi_wrap_span_covers_both_poles` are three
/// faces whose meridian arcs contain a pole in their interior and
/// whose closed form is asserted EXACT. A refusal placed in the shared
/// parse, or in [`curved_face`], would retract that. What such a face
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
/// Each arm nonetheless repeats ONE of the parse's classifications —
/// `props_circle_axis_class` on the sphere, `props_meridian_apex` on
/// the cone — because it must know WHICH edges the per-kind question
/// is about, and a public door may not rest on a caller having run
/// another one. That costs one duplicate funnel sample per circle or
/// line edge at `mesh`'s door. It is a duplicate of a sample already
/// in the stream, never a new margin VALUE, so the large-K lint (which
/// lints margins against K, not counts) cannot move on it. The
/// alternative a reviewer proposed — route this door through
/// `sphere_boundary` and read its per-edge kinds — is strictly worse
/// on exactly this axis: it would repeat EVERY decide of the parse
/// (`props_rim_fit`, `props_meridian_great`, `props_meridian_pole`,
/// the rim incidences) instead of one, since the shape door has
/// already run that parse by the time this one is asked.
///
/// # Errors
///
/// [`PropsError::NotOneChartBranch`] naming the offending edge and the
/// branch boundary its span crosses (valid input, unbuilt lane — D2
/// addendum row 2: the recourse is to state the side as two edges
/// meeting at the singularity, which every consumer reads);
/// [`PropsError::Escalated`] when the rim/meridian classification
/// lands in the ambiguity band (escalate, never guess);
/// [`PropsError::Unimplemented`] for a NURBS surface;
/// [`PropsError::NotIsoRectangle`] on a plane.
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
                for (m, _) in sphere_meridian_pole_margins(e, center, radius, axis, n_c) {
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

/// The predicate on a linearly-leveled parse: the face's extremes from
/// `min_max` over every level the boundary touches, lifted into the
/// rims' own representation. Vacuous on a rimless parse (the sphere
/// band), which is the door's stated answer for it.
fn linear_rims_at_extremes<T: Decide>(b: &LinearBoundary<T>, band: Band) -> Result<(), PropsError> {
    let (lo, hi) = min_max(&b.levels)?;
    require_rims_at_extremes(&b.rims, ((b.as_level)(lo), (b.as_level)(hi)), b.arms, band)
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

/// A classified rim: signed `u`-traversal direction (`d_u`), parameter
/// span (`dt`, the face's `Δu` candidate), and its iso-level payload.
struct Rim<T: Real> {
    /// ±1: traversal direction in `u` = sign(carrier axis · surface
    /// axis) × traversal direction. The scalar image of `d_u_sign`
    /// (`t_sign`), kept as `T` for the margin arithmetic it feeds
    /// (the `du_of_rims` group keys).
    d_u: T,
    /// The same traversal direction as a **discrete** definite sign —
    /// the combinatorial channel [`boundary_material_sign`] reads, so
    /// the material-side cross-check compares two exact ±1s without
    /// comparing scalars (both factors are discrete at origin: a
    /// definite `classify` outcome × the stored traversal bool).
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
/// the torus's `minor`). A Δu angle or a ±1 traversal-direction
/// difference is azimuthal, and the point deviation it induces is at
/// the **azimuthal** arm (the sphere's `R`, the torus's `major`). The
/// two coincide on every kind but the torus, which is why one scalar
/// was enough until it was not.
#[derive(Clone, Copy)]
struct RimArms<T> {
    /// The lever a [`RimLevel::Unit`] difference turns about.
    /// **Never consumed for a [`RimLevel::Length`] kind** — an axial or
    /// slant level difference is already the point deviation and
    /// reaches the funnel bare.
    level: T,
    /// The lever an azimuthal difference (Δu, ±1 traversal direction)
    /// turns about.
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
///   `Ok(false)` would let a caller that only groups carry on, and
///   [`require_zero`] would let a poisoned `Zero` through outright.
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
        return Err(mixed_levels::<T>(name, band));
    };
    if let Some(second) = or {
        let Some(d) = level_gap(level, second) else {
            return Err(mixed_levels::<T>(name, band));
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

/// The refusal a mixed-representation level pair gets. The escalation
/// is attempted first, so the ordinary outcome is a typed
/// [`PropsError::Escalated`] rather than a panic (D9); a `Zero` from
/// the poisoned margin still refuses, which is why this is not routed
/// through [`require_zero`].
fn mixed_levels<T: Decide>(name: &'static str, band: Band) -> PropsError {
    match classify::<T>(name, Margin::of(T::from_f64(f64::NAN)), band) {
        Ok(_) => PropsError::NotIsoRectangle { what: name },
        Err(escalated) => escalated,
    }
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
/// Three margins are metered here and they do not share a lever
/// ([`RimArms`]): the rim LEVEL difference that keys the grouping
/// turns about `arms.level`, the ±1 traversal-direction difference and
/// the `Δu` angle difference about `arms.azimuth`. On the torus those
/// are `minor` and `major`.
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
    let mut groups: Vec<(RimLevel<T>, T, T)> = Vec::new(); // (level, d_u, dt sum)
    for rim in rims {
        let mut placed = false;
        for g in &mut groups {
            let same = level_coincides("props_rim_level_group", rim.level, g.0, None, arms, band)?;
            let same_dir = classify(
                "props_rim_dir_group",
                Margin::levered(rim.d_u - g.1, arms.azimuth),
                band,
            )? == Sign::Zero;
            if same && same_dir {
                g.2 = g.2 + rim.dt;
                placed = true;
                break;
            }
        }
        if !placed {
            groups.push((rim.level, rim.d_u, rim.dt));
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
fn linear_rim_side<T: Decide>(
    b: &LinearBoundary<T>,
    (lo, hi): (T, T),
    band: Band,
) -> Result<Sign, PropsError> {
    /// `s_f` from `rim`'s level and the face's level range: the
    /// interior lies toward the opposite extreme. Metered by the
    /// level's own dimension ([`RimLevel`]) — bare for `Length`
    /// (meters already), `× arms.level` for the dimensionless `Unit`
    /// primary component.
    ///
    /// Returns the **discrete** sign, definite by construction: a
    /// definite `props_rim_side` outcome × the rim's definite traversal
    /// direction. Flux callers scalarize through `t_sign`; the
    /// material-side gate consumes it combinatorially.
    fn side<T: Decide>(
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
        match classify("props_rim_side", margin, band)? {
            Sign::Positive => Ok(rim.d_u_sign),
            Sign::Negative => Ok(rim.d_u_sign.flip()),
            Sign::Zero => Err(PropsError::DegenerateFace),
        }
    }

    require_rims_at_extremes(&b.rims, ((b.as_level)(lo), (b.as_level)(hi)), b.arms, band)?;
    let rim = b.rims.first().ok_or(PropsError::NotIsoRectangle {
        what: "curved face without a rim (non-sphere)",
    })?;
    side(rim, lo, hi, b.arms, band)
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
                    d_u: t_sign::<T>(rim_dir(s, e.forward)),
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
            Curve3::Nurbs(_) => return Err(PropsError::Unimplemented),
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
    let b = cone_boundary(apex, axis, sin_a, cos_a, edges, band)?;
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
/// the shared parse consumed by both the flux closed form and
/// [`boundary_material_sign`].
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
    for e in edges {
        match e.carrier {
            Curve3::Line { dir, .. } => {
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
                    d_u: t_sign::<T>(rim_dir(s, e.forward)),
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
            Curve3::Nurbs(_) => return Err(PropsError::Unimplemented),
        }
    }
    // Cone levels are the signed SLANT arc length — `Length`, bare —
    // and the arm is the first rim's own radius ([`cone_arm`]), which
    // meters only the dimensionless margins in `du_of_rims`.
    let arms = RimArms::uniform(cone_arm(&rims, sin_a));
    Ok(LinearBoundary {
        rims,
        levels,
        arms,
        as_level: RimLevel::Length,
    })
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
/// A face with **no rims** is the two-band construction (M2 PR 5's
/// axis-touching full revolve). What the arm below establishes, and
/// what it does not, stated exactly — it is the ONE domain the
/// iso-rectangle predicate is exempt from, and an exemption is a claim
/// about the arm, not a fact the arm checks:
///
/// * **Established.** Every boundary edge classified `Zero` by
///   `props_circle_axis_class` is a meridian great circle centred on
///   the sphere centre at the sphere's radius (`props_meridian_great`),
///   and all of their carrier axes are parallel
///   (`props_band_coplanar`) — so every meridian of the boundary lies
///   on ONE great circle, which cuts the sphere into two halves of
///   azimuthal width π. That is where `Δu = π` comes from, and it is
///   load-bearing: a lune bounded by meridians on two DIFFERENT great
///   circles would take `Δu = π` for a domain of another width and
///   measure by that factor (a quarter lune, twice over).
/// * **Established separately: the `v`-extent.** `(lo, hi)` is
///   `min_max` over the meridians' endpoint latitudes AND each arc's
///   span-derived pole extremes ([`sphere_meridian_span_levels`]), so
///   the arcs need not run pole to pole for the extent to be theirs:
///   the same hemisphere split at two ordinary points instead of at
///   its poles still folds to `[−1, 1]`. The extent derivation is a
///   fact about the levels, not about this exemption — do not read
///   the exemption as "the domain is verified a rectangle".
///
/// Its `s_f` is the **only** flux
/// sign in this module that the boundary does not encode — with no rim
/// there is no traversal to read it off, and a rimless band's two
/// meridians are traversed the same way whichever side is material.
/// Before M5 S10 it was hardcoded `+1`, justified by "M2 sweeps emit
/// single outward shells only, so a rimless band with inward
/// orientation is unrepresentable at rest". S10 makes that
/// representable — `Face::sense` is exactly the missing bit — so the
/// hardcode becomes `s_f = sense_sign`. Identical for every face this
/// build mints (all `sense: true`); the difference is that an inward
/// rimless band is no longer silently metered as outward.
fn sphere<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    edges: &[LoopEdge<T>],
    sense_sign: T,
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let (b, meridian_axes) = sphere_boundary(center, radius, axis, edges, band)?;
    let (du, s_f);
    let (lo, hi) = min_max(&b.levels)?;
    require_extent(Margin::levered(hi - lo, radius), band)?;
    if b.rims.is_empty() {
        // Two-band face (module docs above): meridians coplanar, Δu = π.
        let Some((&first, rest)) = meridian_axes.split_first() else {
            return Err(PropsError::NotIsoRectangle {
                what: "sphere face with an empty boundary",
            });
        };
        for &n in rest {
            require_zero(
                "props_band_coplanar",
                Margin::levered(n.cross(first).norm(), radius),
                band,
            )?;
        }
        du = T::pi();
        // The one orientation fact no rim encodes (see the fn docs):
        // the face's sense IS `s_f` here, not a cross-check of it.
        s_f = sense_sign;
    } else {
        // The iso-rectangle premise (S58/#649). Sphere rims carry the
        // `(sin v, cos v)` direction pair, so the scalar extremes are
        // lifted into the same representation (`as_level`) and metered
        // at the sphere radius.
        s_f = t_sign::<T>(linear_rim_side(&b, (lo, hi), band)?);
        du = du_of_rims(&b.rims, b.arms, band)?;
    }
    let area = radius.powi(2) * du * (hi - lo);
    let va = loop_vector_area(edges, center)?;
    let flux = s_f * (radius * area) + (center - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
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
/// **Saturated spans (`dt ≥ 2π`) are NOT total, and the earlier claim
/// that they were is retracted here** (issue 1601, pre-existing on
/// this unit's base and NOT fixed by it — the fix is the flux lane's).
/// The clamp holds `c_edge` at `−1`, which leaves the membership test
/// as `f = ⟨P, M⟩ + 1` — nonnegative, but with a zero set that is not
/// empty: it vanishes at the ONE direction antipodal to the span's
/// midpoint `M`. "A full turn covers every direction, so the edge
/// cosine excludes nothing" is the claim that needed an EMPTY zero
/// set. At and near that direction `f` is a rounding residual, and
/// `copysign` transfers its SIGN onto the chord, so a `2π + 2δ` span
/// whose pole lands there can read `Negative` and fold short — by
/// `(1 − cos δ)/2` in the dot value — rather than folding as the
/// paragraph above promises.
///
/// **The branch door is unaffected**, which is why this is a note and
/// not a blocker for it: [`require_one_chart_branch`] refuses only a
/// definite `Positive`, so a residual `Negative` at that direction
/// ADMITS — the direction this door is already permissive in, and the
/// same answer it gives for the arc that merely ends at a pole.
///
/// The pole is located relative to the STORED span, as directions —
/// no chart inversion at all, so this is not the wedge-unwrap trap
/// the module docs forbid (two endpoint inversions differenced,
/// which loses the winding); the interval stays the stored
/// `t1 − t0`.
fn sphere_meridian_pole_margins<T: SpanLocate>(
    e: &LoopEdge<T>,
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    n_c: Vec3<T>,
) -> [(T, T); 2] {
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
    // the scalar lane does not have.
    let half = T::from_f64(0.5);
    let (sd2, cd2) = (dt * half).sin_cos();
    // The membership EDGE saturates at a half-turn: a span of 2π or
    // more covers every direction of the parameter circle, so its
    // edge cosine is −1 — while raw `cos(dt/2)` swings back positive
    // past `dt = 2π` and would EXCLUDE directions a multi-wrap span
    // covers (executed: a 3π span read the north pole `Negative` and
    // the face measured half its area). The clamp makes the test
    // total over every positive stored span — and it is a clamp only
    // because this is ONE edge's span, which certification bounded; a
    // span reconstructed across edges is decided, not clamped
    // (`fold_chain`, the class statement).
    let (_, c_edge) = (dt * half).min(T::pi()).sin_cos();
    let (sdt, cdt) = dt.sin_cos();
    [(sa, ca, r0), (-sa, -ca, -r0)].map(|(ps, pc, extreme)| {
        // Sign: the pole lies in the closed span iff its direction is
        // within `min(dt/2, π)` of the span's midpoint direction —
        // one dot test, `⟨P, M⟩ − c_edge`, whose zero set on the
        // circle is exactly the two span endpoints (empty for a
        // full-period span, which contains everything).
        let f = ps * cd2 + pc * sd2 - c_edge;
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
    })
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
) {
    for (m, extreme) in sphere_meridian_pole_margins(e, center, radius, axis, n_c) {
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
                Curve3::Nurbs(_) => PropsError::Unimplemented,
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
                    d_u: t_sign::<T>(rim_dir(s, e.forward)),
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
                // contains (see `sphere_meridian_span_levels`).
                sphere_meridian_span_levels(e, center, radius, axis, n_c, &mut levels, band);
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
                    d_u: t_sign::<T>(rim_dir(s, e.forward)),
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
/// on that bound — the sphere arm's pole fold saturates its membership
/// edge at a half-turn, a clamp that is total only because no
/// certified edge exceeds one period. A span this fold reconstructs
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

    /// The structurally-impossible mixed-kind arm must escalate typed
    /// (poisoned classify), never panic and never answer false.
    #[test]
    fn mixed_kind_levels_escalate_typed() {
        let band = Band::linear(Tol::witness()).expect("band");
        let got = level_coincides(
            "props_rim_level_group",
            RimLevel::Length(1.0_f64),
            RimLevel::Unit(0.5, 0.5),
            None,
            RimArms::uniform(1.0),
            band,
        );
        assert!(got.is_err(), "mixed kinds must poison typed: {got:?}");
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
            d_u: 1.0_f64,
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
            d_u: 1.0,
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
