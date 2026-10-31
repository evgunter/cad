//! **Which side of a plane a conic edge leaves on when its departure
//! tangent lies in the plane**: the second-order reading of a conic
//! bound, certified. The boolean's vertex-on-face pass
//! (`topo::boolean::sectors::pierce_side`) and the splitting lane's
//! neighbourhood (`topo::splitting::neighborhood`) both read it here.
//!
//! [`ConicDeparture::of`] reads an edge's departure tangent, extent and
//! [`ConicBend`] (the carrier's jet where the edge leaves the vertex
//! and the bounds its curvature keeps along the carrier);
//! [`conic_bend_side`] turns them into a side of a plane.

use crate::enters::{EntersMaterial, ReferenceNormal};
use geom_core::k_stats::NonzeroSign;
use geom_core::{Band, Decide, Indeterminate, Margin, Real, Sign, Vec3};

/// **How a conic edge bends off its departure tangent** at the vertex
/// it leaves: what [`conic_bend_side`] reads one order down where the
/// tangent lies in a plane.
#[derive(Clone, Copy, Debug)]
pub struct ConicBend<T: Real> {
    /// The carrier's curvature vector at the vertex: its second
    /// derivative's part normal to the tangent, per arc length squared.
    pub curvature: Vec3<T>,
    /// What the carrier's curvature is bounded by along the edge.
    pub bounds: BendBounds<T>,
}

/// The bounds a conic's curvature keeps everywhere on its carrier.
#[derive(Clone, Copy, Debug)]
pub enum BendBounds<T: Real> {
    /// A circle or an ellipse: a plane curve whose curvature is at
    /// least `least`, positive, and whose points all lie within `reach`
    /// of the vertex and within `rise` of its tangent line there.
    Planar {
        /// The least curvature anywhere on the carrier.
        least: T,
        /// A bound on every point's distance from the vertex.
        reach: T,
        /// A bound on every point's distance from the tangent line at
        /// the vertex, toward the principal normal.
        rise: T,
        /// A lower bound on the edge's arc length.
        arc: T,
        /// The arc length from the vertex over which the carrier rises
        /// at least `2·sin²(least·s/2)/least` off its tangent line at
        /// `s` ([`conic_bend_side`]).
        lifts_to: T,
    },
    /// A spiric section or a spline: no least curvature is certified (a
    /// spiric's oval may run flat, and a spline's curvature carries no
    /// bound here), so its tie with a plane reads no side.
    Unbounded,
}

/// **Where an edge on a curved carrier leaves a vertex**, read once for
/// both vertex-neighbourhood lanes: a conic, a spiric or a spline, each
/// by its tangent, never its chord.
#[derive(Clone, Copy, Debug)]
pub struct ConicDeparture<T: Real> {
    /// The departure's unit tangent scaled to [`Self::extent`]: its
    /// `.norm()` is the sector predicates' lever arm, its
    /// `.normalize()` the direction.
    pub dir: Vec3<T>,
    /// The edge's extent from below ([`crate::edge_extent_inf`]): no
    /// longer than its arc, which [`conic_bend_side`]'s witness needs.
    pub extent: T,
    /// How the edge bends off [`Self::dir`].
    pub bend: ConicBend<T>,
}

impl<T: Decide> ConicDeparture<T> {
    /// The departure of `curve`'s walk out of its vertex (`he_plus`
    /// walks `t₀ → t₁`, [`crate::EdgeCurve::walk_tangents`]), `chord`
    /// the distance between the edge's end vertices.
    ///
    /// # Panics
    ///
    /// On a line carrier ([`ConicBend::of`]).
    #[must_use]
    pub fn of(curve: &crate::EdgeCurve<T>, he_plus: bool, chord: T) -> Self {
        let (t0, t1) = curve.params();
        let (tangent, _) = curve.walk_tangents(he_plus);
        let extent = crate::edge_extent_inf(curve.carrier(), t0, t1, chord);
        let bend = ConicBend::of(
            curve.carrier(),
            tangent,
            curve.walk_departure_deriv2(he_plus),
            t1 - t0,
        );
        Self {
            dir: tangent.normalize() * extent,
            extent,
            bend,
        }
    }
}

impl<T: Decide> ConicBend<T> {
    /// The bend of the conic `carrier` where an edge leaves it: `deriv`
    /// and `deriv2` the carrier's first and second derivatives there,
    /// `span` the edge's parameter span (`t₁ − t₀`).
    ///
    /// # Panics
    ///
    /// On a line carrier, which does not bend.
    #[must_use]
    pub fn of(carrier: &geom::Curve3<T>, deriv: Vec3<T>, deriv2: Vec3<T>, span: T) -> Self {
        let speed_sq = deriv.norm_squared();
        let t = deriv.normalize();
        let curvature = (deriv2 - t * deriv2.dot(t)) / speed_sq;
        let bounds = match *carrier {
            // The parameter is the angle, so the arc is `radius·span`
            // long, no point lies further than the diameter, and the arc
            // rises exactly `radius·(1 − cos θ) = 2·radius·sin²(θ/2)` off
            // its tangent at angle `θ`, which grows to `θ = π`; most at
            // `θ = min(span, π)` (the sine form, since `1 − cos θ`
            // cancels to nothing at a small `θ`).
            geom::Curve3::Circle { radius, .. } => BendBounds::Planar {
                least: T::one() / radius,
                reach: (radius * span).min(radius * T::from_f64(2.0)),
                rise: radius
                    * T::from_f64(2.0)
                    * (span.min(T::pi()) * T::from_f64(0.5)).sin().powi(2),
                arc: radius * span,
                lifts_to: radius * T::pi(),
            },
            // Semi-axes `a ≥ b`: the curvature runs from `b/a²` at the
            // ends of the minor axis to `a/b²` at the ends of the major,
            // and no point lies further than the major diameter. Off its
            // tangent the arc rises `y' = sin φ ≤ φ ≤ most·s`, so at most
            // `most·S²/2` over its length `S ≤ a·span` (the speed is at
            // most `a`), and never past the major diameter. The speed is
            // at least `b`, so the arc is at least `b·span` long. The
            // lift bound holds while the tangent turns at most a quarter
            // turn at the greatest curvature.
            geom::Curve3::Ellipse { semi_u, semi_v, .. } => {
                let (a, b) = geom::Curve3::semi_axes_by_size(semi_u, semi_v);
                let most = a / b.powi(2);
                BendBounds::Planar {
                    least: b / a.powi(2),
                    reach: a * T::from_f64(2.0),
                    rise: (most * (a * span).powi(2) * T::from_f64(0.5)).min(a * T::from_f64(2.0)),
                    arc: b * span,
                    lifts_to: T::pi() * T::from_f64(0.5) / most,
                }
            }
            geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => BendBounds::Unbounded,
            geom::Curve3::Line { .. } => {
                unreachable!("`ConicBend::of` reads a curved carrier, not a line")
            }
        };
        Self { curvature, bounds }
    }
}

/// **A conic edge's side of a plane one order down**, where its
/// departure tangent `dir` read `Zero` against the plane: which side
/// the arc BENDS to. `extent` is the edge's extent
/// from below ([`crate::edge_extent_inf`]): no longer than its arc, so
/// a point at that arc length lies on the edge.
///
/// Put the arc in its own plane, in the frame of its unit tangent `t̂`
/// and principal normal `N̂` at the vertex: a point at arc length `s`
/// sits at `x·t̂ + y·N̂`, so its height over the plane is
/// `x·(t̂·n̂) + y·(N̂·n̂)` exactly. The first term is the first-order
/// residue the Zero left (`|x| ≤ s`), the second the bend.
///
/// **The bend's lower bound.** The tangent turns at the curvature
/// `κ(s) ∈ [least, most]` (`most` the greatest), so its angle `φ(s)` lies between `least·s`
/// and `most·s`. While `most·s ≤ π/2`, `sin φ ≥ sin(least·s)`, and
/// integrating, `y(s) ≥ (1 − cos(least·s))/least
/// = 2·sin²(least·s/2)/least`; on a circle that is the exact rise, out
/// to a half turn (`lifts_to`). So at `l = min(max(extent, arc),
/// lifts_to)` the arc stands at least
///
/// `|N̂·n̂|·2·sin²(least·l/2)/least − |t̂·n̂|·l`
///
/// off the plane on the side `N̂·n̂` names, and a margin that clears the
/// band is the verdict: bending against `n̂` is
/// [`EntersMaterial::Enters`], along it [`EntersMaterial::Exits`]. `l`
/// is no longer than the arc (`extent` and `arc` both bound it from
/// below), so the witness point lies on the edge.
///
/// **Tangent.** Every point of the arc lies in its own plane, within
/// `reach` of the vertex and within `rise` of the tangent line there
/// toward `N̂` ([`BendBounds::Planar`]), so its height is at most
/// `|t̂·n̂|·reach + |N̂·n̂|·rise`; where that reads `Zero` the whole edge
/// lies in the plane to the band, and the verdict is
/// [`EntersMaterial::Tangent`].
///
/// # Errors
///
/// The [`Indeterminate`] of `"conic_bend"` or `"conic_bend_side"`: a
/// bend the band does not separate from the residue, and every spiric
/// section, whose curvature carries no bound here.
pub fn conic_bend_side<T: Decide>(
    dir: Vec3<T>,
    extent: T,
    bend: ConicBend<T>,
    reference_normal: ReferenceNormal<T>,
    band: Band,
) -> Result<EntersMaterial, Indeterminate> {
    let n = reference_normal.vec();
    let residue = |arm: T| Margin::levered(dir.normalize().dot(n), arm).value();
    let normal_part = bend.curvature.dot(n) / bend.curvature.norm();
    // The bend's lower bound, signed as `N̂·n̂` is. A spiric section has
    // no bend this reading certifies: it lifts nothing, and the margin
    // below cannot clear.
    let (lift, l) = match bend.bounds {
        BendBounds::Planar {
            least,
            reach,
            rise,
            arc,
            lifts_to,
        } => {
            if let Ok(Sign::Zero) = crate::dihedral::decide(
                "conic_flat",
                Margin::of(residue(reach).abs() + normal_part.abs() * rise),
                band,
            ) {
                return Ok(EntersMaterial::Tangent);
            }
            let l = extent.max(arc).min(lifts_to);
            let half_turn = (least * l * T::from_f64(0.5)).sin();
            (
                normal_part * half_turn.powi(2) * T::from_f64(2.0) / least,
                l,
            )
        }
        BendBounds::Unbounded => (T::zero(), extent),
    };
    crate::dihedral::decide_positive(
        "conic_bend",
        Margin::of(lift.abs() - residue(l).abs()),
        band,
    )?;
    // The signed lift is the one that just cleared, so its sign is
    // decided wherever the margin was; the arm refuses rather than
    // trusting that.
    Ok(
        match crate::dihedral::decide_nonzero("conic_bend_side", Margin::of(lift), band)? {
            NonzeroSign::Negative => EntersMaterial::Enters,
            NonzeroSign::Positive => EntersMaterial::Exits,
        },
    )
}
