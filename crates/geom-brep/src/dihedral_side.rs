//! **The dihedral side**: which side of one face a second face leaves
//! a shared edge on, read from the two faces' outward normals at a point
//! of the edge, for every surface kind a face carries.
//!
//! # The reading
//!
//! At a point `p` of an edge two faces share, each face has an outward
//! normal, and a face leaves the edge into its own interior along
//! `d = n × t̂`, with `t̂` the edge's direction as that face's loop walks
//! it (outer loops run counterclockwise seen from outside, so the
//! interior lies to the left of the walk). That direction is an
//! [`EdgeWalk`], minted from the carrier's derivative and the half of
//! the edge that walks it, so the mate's walk is not a value the door
//! can be handed by mistake for the face's own. The departing face lies on
//! the side of the other face that `d` points to, which is
//! [`crate::enters_material`] of `d` against the other face's outward
//! normal: the sign of `t̂ · (n_against × n_departing)`, the dihedral's
//! sine signed by the two faces' material, metered over the folded lever
//! arm in meters (D4 ¶1, [`crate::dihedral`]'s module docs).
//!
//! The sign is a first-order fact. Where the two tangent planes coincide
//! it reads zero, and which side the face lies on is decided one order
//! up, by how the faces bend apart, which this door does not read: it
//! escalates [`SideEscalation::Tangent`] rather than guess, and an
//! in-band reading escalates through the lever's own rungs.
//!
//! # Normals on every kind
//!
//! An analytic face's normal is its implicit gradient at `p`
//! ([`crate::implicit_outward_normal`]); its lever arm is
//! [`crate::curvature_lever_arm`]. A spline face has no implicit form and
//! is given no signed residual (C5, the curve × face clause): its normal
//! is its chart's `S_u × S_v` at a foot of `p` decided on the face by
//! its distance within the band, and its arm is the radius of its
//! largest principal curvature there.
//!
//! **The chart's normal is read only where it is continuous.** Across a
//! C⁰ line of the chart — an interior knot run of multiplicity at least
//! the degree ([`geom_core::spline::KnotStructure::crease_runs`]), or a
//! closed chart's seam where its two domain ends meet — `S_u × S_v` has
//! a value on each side, and the jet at a foot on the line reads one of
//! them, which need not be the face's. A foot within the band of such a
//! line therefore escalates [`SideEscalation::Crease`]: the face's side
//! of the line is its trim's fact, which this door does not read. A foot
//! past the band from every such line lies inside one span, and reads
//! that span's normal.
//!
//! **The foot's offset is charged to first order.** The normal at the
//! foot stands for the normal at `p`, a distance `δ` inside the band
//! away, so it turns by about `κ·δ` with `κ` the curvature at the foot,
//! which over an arm of at most `1/κ` is a displacement of about `δ`.
//! That is a first-order statement at the foot: `κ` is read pointwise,
//! not bounded over the gap to `p`, so a curvature rising steeply across
//! that gap (an inflection's neighbour, a near-crease) is not covered by
//! it. The gap is inside the band, where the reading's own tolerance
//! already sits.

use geom::{NurbsSurface, Surface};
use geom_core::k_stats::Magnitude;
use geom_core::{Band, Decide, Indeterminate, Margin, MarginDiag, Point3, Real};

use crate::certify::NurbsLane;

use crate::enters::{EntersMaterial, LeverEscalation, OutwardNormal, enters_material_reported};
use crate::implicit::{curvature_lever_arm, implicit_outward_normal};

/// The edge's direction at a point, as one of its two half-edges walks
/// it: the carrier's derivative there where that half-edge is the edge's
/// `he_plus` (which walks the carrier forward, the
/// [`crate::EdgeCurve::walk_tangents`] convention), and its negation
/// where it is the edge's other half. A topo caller mints it from the
/// half-edge in the face whose side is asked, and nothing else.
#[derive(Clone, Copy, Debug)]
pub struct EdgeWalk<T: Real>(geom_core::Vec3<T>);

impl<T: Real> EdgeWalk<T> {
    /// The walk of the edge's half named by `he_plus`, at a point where
    /// the carrier's derivative is `derivative` (any nonzero length).
    #[must_use]
    pub fn of_half(derivative: geom_core::Vec3<T>, he_plus: bool) -> Self {
        Self(if he_plus { derivative } else { -derivative })
    }
}

impl<T: Decide> EdgeWalk<T> {
    /// The walk of the edge's half named by `he_plus` at the carrier's
    /// parameter `t`.
    #[must_use]
    pub fn along(curve: &crate::EdgeCurve<T>, t: T, he_plus: bool) -> Self {
        Self::of_half(curve.carrier().deriv(t), he_plus)
    }
}

/// One face at the shared edge: its carrier and its `Face::sense` bit.
#[derive(Clone, Copy, Debug)]
pub struct SideFace<'a, T: Real> {
    /// The face's carrier.
    pub surface: &'a Surface<T>,
    /// The face's sense bit: its outward normal is its chart normal where
    /// `true` and the negation where `false`.
    pub sense: bool,
}

/// Which of the two faces [`dihedral_side`] reads a refusal names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SideRole {
    /// The face whose side is asked.
    Departing,
    /// The face it is asked against.
    Against,
}

/// Which side of the face it is read against the departing face leaves
/// the edge on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceSide {
    /// Against the other face's outward normal: into its material.
    Material,
    /// Along its outward normal: away from its material.
    Outside,
}

/// Why [`dihedral_side`] read no side.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SideEscalation {
    /// A spline face is the empty placeholder, which has no chart.
    Placeholder(SideRole),
    /// A spline face, at a scalar with no certified NURBS lane: the foot
    /// is a bracket read such a scalar may not mint.
    NoLane(SideRole),
    /// The projection of the point onto a spline face did not converge.
    FootInconclusive {
        /// The spline face.
        role: SideRole,
        /// The last distance the projection read, in meters.
        last_distance: f64,
    },
    /// The point's foot on a spline face lies in the band: whether the
    /// point is on that face is a size this tolerance cannot decide.
    FootUndecided {
        /// The spline face.
        role: SideRole,
        /// The foot distance's escalation.
        diag: Indeterminate,
    },
    /// The point's foot on a spline face is definitely apart from it:
    /// the point is not on that face, so it is no point of an edge the
    /// face bounds.
    OffFace {
        /// The spline face.
        role: SideRole,
        /// The foot distance, decided past the band.
        margin: MarginDiag,
    },
    /// The point's foot on a spline face lies within the band of a line
    /// across which the chart's normal is not continuous (module docs): a
    /// C⁰ knot run, or a closed chart's seam. Which side of that line the
    /// face lies on is its trim's fact, so no normal is read.
    Crease {
        /// The spline face.
        role: SideRole,
        /// The foot's distance from the line, along the chart's iso line
        /// through it, as decided: zero or in band.
        reading: Result<MarginDiag, Indeterminate>,
    },
    /// The arm, or the signed sine metered over it, did not decide.
    Lever(LeverEscalation),
    /// The tangent planes coincide at the point, so the departure has no
    /// first-order side: the side is the second order's to read.
    Tangent(MarginDiag),
}

/// **The side of `against` that `departing` leaves the shared edge on**
/// at `p` (module docs). `walk` is the edge's direction at `p` as
/// `departing`'s own half-edge walks it; `reach` is the
/// faces' extent from the edge, the arm's outer fold, as
/// [`crate::classify_dihedral`] takes it; `lane` is the scalar's certified
/// NURBS lane, which a spline face's foot is read through.
///
/// INVARIANT, enforced by the caller: `p` lies on the edge, so on both
/// faces' carriers. A spline face's foot checks it for that face
/// ([`SideEscalation::OffFace`]); an analytic face's gradient is read at
/// `p` as given.
///
/// # Errors
///
/// [`SideEscalation`]: a spline face's foot not decided onto it or
/// lying on a line its normal is not continuous across, the arm or the
/// reading undecided, or a tangent dihedral.
pub fn dihedral_side<T: Decide>(
    departing: SideFace<'_, T>,
    against: SideFace<'_, T>,
    p: Point3<T>,
    walk: EdgeWalk<T>,
    reach: T,
    lane: Option<NurbsLane<T>>,
    band: Band,
) -> Result<(FaceSide, MarginDiag), SideEscalation> {
    let normal = |face, role| normal_at(face, role, p, reach, lane, band);
    let (n_dep, arm_dep) = normal(departing, SideRole::Departing)?;
    let (n_against, arm_against) = normal(against, SideRole::Against)?;
    let arm = reach.min(arm_dep).min(arm_against);
    let departure = n_dep.vec().cross(walk.0.normalize());
    match enters_material_reported(departure, n_against, arm, band) {
        Ok((EntersMaterial::Enters, margin)) => Ok((FaceSide::Material, margin)),
        Ok((EntersMaterial::Exits, margin)) => Ok((FaceSide::Outside, margin)),
        Ok((EntersMaterial::Tangent, margin)) => Err(SideEscalation::Tangent(margin)),
        Err(e) => Err(SideEscalation::Lever(e)),
    }
}

/// A face's outward normal at `p` and its curvature arm (module docs).
fn normal_at<T: Decide>(
    face: SideFace<'_, T>,
    role: SideRole,
    p: Point3<T>,
    reach: T,
    lane: Option<NurbsLane<T>>,
    band: Band,
) -> Result<(OutwardNormal<T>, T), SideEscalation> {
    match face.surface.spline_chart() {
        None => Ok((
            implicit_outward_normal(face.surface, face.sense, p),
            curvature_lever_arm(face.surface, p),
        )),
        Some(chart) => {
            let lane = lane.ok_or(SideEscalation::NoLane(role))?;
            spline_normal(chart, face.sense, role, p, reach, lane, band)
        }
    }
}

/// A spline face's outward normal at the foot of `p`, and the radius of
/// its largest principal curvature there folded with `reach`.
fn spline_normal<T: Decide>(
    chart: &NurbsSurface<T>,
    sense: bool,
    role: SideRole,
    p: Point3<T>,
    reach: T,
    lane: NurbsLane<T>,
    band: Band,
) -> Result<(OutwardNormal<T>, T), SideEscalation> {
    if chart.is_placeholder() {
        return Err(SideEscalation::Placeholder(role));
    }
    let foot = lane
        .surface_foot(chart, p)
        .map_err(|e| SideEscalation::FootInconclusive {
            role,
            last_distance: e.last_distance,
        })?;
    match crate::dihedral::decide_magnitude_reported(
        "dihedral_side_foot",
        Margin::of(foot.distance),
        band,
    ) {
        Ok((Magnitude::Zero, _)) => {}
        Ok((Magnitude::Positive, margin)) => {
            return Err(SideEscalation::OffFace { role, margin });
        }
        Err(diag) => return Err(SideEscalation::FootUndecided { role, diag }),
    }
    let (u, v) = (T::from_computed(foot.u), T::from_computed(foot.v));
    off_every_crease(chart, foot.foot, (u, v), band)
        .map_err(|reading| SideEscalation::Crease { role, reading })?;
    let jet = chart.ders(u, v);
    let kappa = crate::shape_operator::max_principal_curvature(&jet);
    let (arm, _) = crate::ssi::point_arm(kappa, reach);
    Ok((
        OutwardNormal::from_chart(jet.du.cross(jet.dv).normalize(), sense),
        arm,
    ))
}

/// Whether the foot `at`, at chart parameters `(u, v)`, lies past the
/// band from every line of `chart` its normal is not continuous across
/// (module docs): each interior crease run, per direction, measured from
/// the foot along the iso line through it; and, per direction, each
/// domain end, where the other end meets the foot too (a closed chart's
/// seam). `Err` carries the first such line's reading, zero or in band.
fn off_every_crease<T: Decide>(
    chart: &NurbsSurface<T>,
    at: Point3<T>,
    (u, v): (T, T),
    band: Band,
) -> Result<(), Result<MarginDiag, Indeterminate>> {
    let near = |q: Point3<T>| -> Result<bool, Result<MarginDiag, Indeterminate>> {
        match crate::dihedral::decide_magnitude_reported(
            "dihedral_side_crease",
            Margin::of((q - at).norm()),
            band,
        ) {
            Ok((Magnitude::Positive, _)) => Ok(false),
            Ok((Magnitude::Zero, margin)) => Err(Ok(margin)),
            Err(diag) => Err(Err(diag)),
        }
    };
    let ku = chart.knot_values_u();
    let kv = chart.knot_values_v();
    for (knots, along_u) in [(ku, true), (kv, false)] {
        let at_line = |c: T| {
            if along_u {
                chart.ders(c, v).point
            } else {
                chart.ders(u, c).point
            }
        };
        let runs: Vec<T> = knots.runs().map(|(value, _)| value).collect();
        for r in knots.structure().crease_runs() {
            near(at_line(runs[r]))?;
        }
        let (lo, hi) = (runs[0], runs[runs.len() - 1]);
        // A seam is two ends meeting at the foot: the first end the foot
        // is within the band of is asked of the other only, so an open
        // chart's own boundary, which the foot of a rim point sits on,
        // reads as no line.
        for (end, other) in [(lo, hi), (hi, lo)] {
            if near(at_line(end)).is_err() {
                near(at_line(other))?;
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::spline::KnotVector;
    use geom_core::{Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A wall rising from the edge `edge(s)`, `s ∈ [−1, 1]`: linear along
    /// the edge, quadratic away from it through the rows `edge + v1` and
    /// `edge + v2`, so it leaves the edge along `v1` and bends toward
    /// `v2 − 2·v1`.
    fn wall(edge: impl Fn(f64) -> Point3<f64>, v1: Vec3<f64>, v2: Vec3<f64>) -> Surface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = [-1.0, 1.0]
            .into_iter()
            .flat_map(|s| [edge(s), edge(s) + v1, edge(s) + v2])
            .collect();
        Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap(),
        ))
    }

    /// The plane `z = 0`, its chart normal `+z`.
    fn floor() -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// A wall on the floor along the `x` axis, leaving it toward `+y`
    /// with the rise `rise` per unit run and curving up past it.
    fn floor_wall(rise: f64) -> Surface<f64> {
        wall(
            |s| Point3::new(s, 0.0, 0.0),
            Vec3::new(0.0, 0.5, 0.5 * rise),
            Vec3::new(0.0, 1.0, rise + 0.5),
        )
    }

    /// The walk of a carrier running along `t`, by its plus half.
    fn walk(t: Vec3<f64>) -> EdgeWalk<f64> {
        EdgeWalk::of_half(t, true)
    }

    fn lane() -> Option<crate::NurbsLane<f64>> {
        Some(crate::NurbsLane::certified())
    }

    fn face(surface: &Surface<f64>, sense: bool) -> SideFace<'_, f64> {
        SideFace { surface, sense }
    }

    fn side(
        departing: SideFace<'_, f64>,
        against: SideFace<'_, f64>,
        p: Point3<f64>,
        tangent: Vec3<f64>,
    ) -> Result<FaceSide, SideEscalation> {
        dihedral_side(departing, against, p, walk(tangent), 1.0, lane(), band()).map(|(s, _)| s)
    }

    const ON_EDGE: Point3<f64> = Point3::new(0.3, 0.0, 0.0);
    const X: Vec3<f64> = Vec3::new(1.0, 0.0, 0.0);

    /// A spline wall against a plane, rising and falling: the side is the
    /// wall's departure, and a wall of either sense, walked the way its
    /// own loop walks the edge, reads the same side.
    #[test]
    fn a_spline_wall_reads_its_side_of_a_plane() {
        let floor = floor();
        for (rise, want) in [(1.0, FaceSide::Outside), (-1.0, FaceSide::Material)] {
            let w = floor_wall(rise);
            // Sense true: the chart normal `S_u × S_v` is outward and the
            // interior `+v` lies to its left walking `+x`; sense false
            // walks `−x`.
            for (sense, t) in [(true, X), (false, -X)] {
                assert_eq!(
                    side(face(&w, sense), face(&floor, true), ON_EDGE, t),
                    Ok(want),
                    "rise {rise}, wall sense {sense}"
                );
            }
            // The floor's own material below flips with its sense.
            let flipped = match want {
                FaceSide::Outside => FaceSide::Material,
                FaceSide::Material => FaceSide::Outside,
            };
            assert_eq!(
                side(face(&w, true), face(&floor, false), ON_EDGE, X),
                Ok(flipped),
                "rise {rise}, floor sense false"
            );
        }
    }

    /// A plane against a spline wall: the floor strip on either bank of
    /// the edge, read against the rising wall, whose material lies on its
    /// `+y, −z` side.
    #[test]
    fn a_plane_reads_its_side_of_a_spline_wall() {
        let (floor, w) = (floor(), floor_wall(1.0));
        // The floor's interior lies left of its walk under `+z`: `+y`
        // walking `+x`, `−y` walking `−x`.
        assert_eq!(
            side(face(&floor, true), face(&w, true), ON_EDGE, X),
            Ok(FaceSide::Material),
            "the strip under the wall"
        );
        assert_eq!(
            side(face(&floor, true), face(&w, true), ON_EDGE, -X),
            Ok(FaceSide::Outside),
            "the strip behind it"
        );
    }

    /// The cylinder `x² + y² = 1` and a spline wall on its ruling
    /// `(1, 0, z)`, leaving it toward `+y` and out (`flare > 0`) or in.
    fn ruling_wall(flare: f64) -> Surface<f64> {
        wall(
            |s| Point3::new(1.0, 0.0, s),
            Vec3::new(0.5 * flare, 0.5, 0.0),
            Vec3::new(flare, 1.0, 0.0),
        )
    }

    fn cylinder() -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    const ON_RULING: Point3<f64> = Point3::new(1.0, 0.0, 0.3);
    const Z: Vec3<f64> = Vec3::new(0.0, 0.0, 1.0);

    /// A spline wall against a cylinder across a ruling, both ways out of
    /// it, and the cylinder against the wall on both banks.
    #[test]
    fn a_spline_wall_and_a_cylinder_read_each_others_side() {
        let cyl = cylinder();
        for (flare, want) in [(1.0, FaceSide::Outside), (-1.0, FaceSide::Material)] {
            let w = ruling_wall(flare);
            // Sense true: `S_u × S_v` outward, the wall's interior left
            // of a walk along `+z`.
            assert_eq!(
                side(face(&w, true), face(&cyl, true), ON_RULING, Z),
                Ok(want),
                "flare {flare}"
            );
        }
        // The outward wall's material lies on its `+x, −y` side, so the
        // cylinder's sheet leaving the ruling toward `−y` (walking `+z`
        // under its outward `+x`) enters it and the sheet toward `+y`
        // does not.
        let w = ruling_wall(1.0);
        assert_eq!(
            side(face(&cyl, true), face(&w, true), ON_RULING, Z),
            Ok(FaceSide::Material),
            "the cylinder toward −y"
        );
        assert_eq!(
            side(face(&cyl, true), face(&w, true), ON_RULING, -Z),
            Ok(FaceSide::Outside),
            "the cylinder toward +y"
        );
    }

    /// A wall leaving the floor tangent to it has no first-order side,
    /// and one leaving at an angle the band cannot tell from tangent
    /// escalates: neither guesses.
    #[test]
    fn a_tangent_or_in_band_departure_escalates() {
        let floor = floor();
        let tangent = floor_wall(0.0);
        assert!(
            matches!(
                dihedral_side(
                    face(&tangent, true),
                    face(&floor, true),
                    ON_EDGE,
                    walk(X),
                    1.0,
                    lane(),
                    band()
                ),
                Err(SideEscalation::Tangent(_))
            ),
            "a tangent wall"
        );
        // The wall's sine is about its rise and its curvature radius one
        // (`S_v ≈ ŷ`, `S_vv = ẑ` at the edge), so the margin metered over
        // the unit reach is about the rise: placed inside the band.
        let b = band();
        let rise = (b.zero() * b.escalate()).sqrt();
        let near = floor_wall(rise);
        match dihedral_side(
            face(&near, true),
            face(&floor, true),
            ON_EDGE,
            walk(X),
            1.0,
            lane(),
            b,
        ) {
            Err(SideEscalation::Lever(e)) => {
                assert_eq!(e.rung(), crate::LeverRung::Reading, "the reading escalates");
            }
            other => panic!("an in-band rise {rise} read {other:?}"),
        }
    }

    /// The arm folds the wall's curvature: past its radius of curvature
    /// the faces' reach meters nothing, so a rise in band over that radius
    /// escalates however far the faces run.
    #[test]
    fn the_arm_is_the_walls_radius_of_curvature_past_its_reach() {
        let b = band();
        // The wall bends at `κ ≈ 1` (`S_v ≈ ŷ`, `S_vv = ẑ`), so its arm
        // is one: a rise in band there, over a reach whose own arm would
        // carry it ten times past the band.
        let rise = (b.zero() * b.escalate()).sqrt();
        let reach = 10.0 * b.escalate() / rise;
        let near = floor_wall(rise);
        match dihedral_side(
            face(&near, true),
            face(&floor(), true),
            ON_EDGE,
            walk(X),
            reach,
            lane(),
            b,
        ) {
            Err(SideEscalation::Lever(e)) => {
                assert_eq!(e.rung(), crate::LeverRung::Reading, "the reading escalates");
            }
            other => panic!("rise {rise} over reach {reach} read {other:?}"),
        }
    }

    /// A point whose foot on the spline face lies in the band is a size
    /// the tolerance cannot place on the face: escalated, not read.
    #[test]
    fn a_foot_in_band_escalates() {
        let b = band();
        let w = floor_wall(1.0);
        // Beside the wall's rim, off its chart: the foot is the rim point
        // and its distance is the offset.
        let off = Point3::new(0.3, -(b.zero() * b.escalate()).sqrt(), 0.0);
        assert!(
            matches!(
                side(face(&w, true), face(&floor(), true), off, X),
                Err(SideEscalation::FootUndecided {
                    role: SideRole::Departing,
                    ..
                })
            ),
            "an in-band foot"
        );
    }

    /// A point off the spline face is no point of its edge: refused, not
    /// read at its foot.
    #[test]
    fn a_point_off_the_spline_face_is_refused() {
        let w = floor_wall(1.0);
        let off = Point3::new(0.3, -0.2, 0.0);
        assert!(
            matches!(
                side(face(&w, true), face(&floor(), true), off, X),
                Err(SideEscalation::OffFace {
                    role: SideRole::Departing,
                    ..
                })
            ),
            "the wall's foot is 0.2 m away"
        );
    }

    /// A scalar with no certified NURBS lane reads no spline face, and
    /// names which face it could not read; an analytic pair needs no lane.
    #[test]
    fn a_spline_face_without_a_lane_refuses_typed() {
        let (w, floor) = (floor_wall(-1.0), floor());
        assert_eq!(
            dihedral_side(
                face(&floor, true),
                face(&w, true),
                ON_EDGE,
                walk(X),
                1.0,
                None,
                band()
            ),
            Err(SideEscalation::NoLane(SideRole::Against))
        );
        let tilted = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, -1.0, 1.0).normalize(),
            u_ref: X,
        };
        assert_eq!(
            dihedral_side(
                face(&tilted, true),
                face(&floor, true),
                ON_EDGE,
                walk(X),
                1.0,
                None,
                band()
            )
            .map(|(s, _)| s),
            Ok(FaceSide::Outside),
            "the plane the rising wall leaves along"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod review_rows {
    use super::*;
    use geom_core::spline::KnotVector;
    use geom_core::{Tol, Vec3};

    /// A carrier folded along the `x` axis at the interior knot `v = ½`
    /// (degree 1 in `v`, so C0 there): the half `v < ½` rises toward
    /// `−y`, the half `v > ½` toward `+y`. A face trimmed to either half
    /// has the crease as its rim, which is no chart boundary.
    fn creased() -> Surface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let control = [-1.0, 1.0]
            .into_iter()
            .flat_map(|s| {
                [
                    Point3::new(s, -1.0, 1.0),
                    Point3::new(s, 0.0, 0.0),
                    Point3::new(s, 1.0, 1.0),
                ]
            })
            .collect();
        Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap(),
        ))
    }

    fn plane(normal: Vec3<f64>) -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: normal.normalize(),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn side(
        dep: &Surface<f64>,
        against: &Surface<f64>,
        p: Point3<f64>,
        t: Vec3<f64>,
    ) -> Result<FaceSide, SideEscalation> {
        dihedral_side(
            SideFace {
                surface: dep,
                sense: true,
            },
            SideFace {
                surface: against,
                sense: true,
            },
            p,
            EdgeWalk::of_half(t, true),
            1.0,
            Some(crate::NurbsLane::certified()),
            Band::linear(Tol::witness()).unwrap(),
        )
        .map(|(s, _)| s)
    }

    /// Each half of the creased carrier, as a face whose rim is the
    /// crease, has a normal of its own there that the chart's jet cannot
    /// tell from the other half's: the side escalates, on either walk.
    #[test]
    fn each_half_of_a_creased_carrier_escalates_on_the_crease() {
        let (c, floor) = (creased(), plane(Vec3::new(0.0, 0.0, 1.0)));
        let x = Vec3::new(1.0, 0.0, 0.0);
        for (half, walk) in [("lower", -x), ("upper", x)] {
            let got = side(&c, &floor, Point3::new(0.3, 0.0, 0.0), walk);
            assert!(
                matches!(
                    got,
                    Err(SideEscalation::Crease {
                        role: SideRole::Departing,
                        ..
                    })
                ),
                "the {half} half read {got:?}"
            );
        }
    }

    /// Past the band from the crease, a creased carrier reads the span
    /// the foot lies in: the lower half `(s, −a, a)` against the plane
    /// `x = 0.3` across its line there.
    #[test]
    fn a_creased_carrier_reads_its_side_off_the_crease() {
        let c = creased();
        let cut = Surface::Plane {
            origin: Point3::new(0.3, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        let p = Point3::new(0.3, -0.5, 0.5);
        // The lower half's outward normal is ∝ (0, 1, 1), so walking
        // (0, −1, 1) its interior lies toward +x.
        let t = Vec3::new(0.0, -1.0, 1.0);
        assert_eq!(side(&c, &cut, p, t), Ok(FaceSide::Outside), "toward +x");
        assert_eq!(side(&c, &cut, p, -t), Ok(FaceSide::Material), "toward −x");
    }

    /// A chart closed in `v` with a C⁰ seam: its two ends meet along the
    /// edge with opposite normals, so a foot on the seam escalates.
    #[test]
    fn a_closed_charts_seam_escalates() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = [-1.0, 1.0]
            .into_iter()
            .flat_map(|s| {
                [
                    Point3::new(s, 0.0, 0.0),
                    Point3::new(s, 1.0, 1.0),
                    Point3::new(s, 0.0, 0.0),
                ]
            })
            .collect();
        let fin = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap(),
        ));
        let floor = plane(Vec3::new(0.0, 0.0, 1.0));
        let got = side(
            &fin,
            &floor,
            Point3::new(0.3, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        assert!(
            matches!(got, Err(SideEscalation::Crease { .. })),
            "the seam read {got:?}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod review_rows_b {
    use super::*;
    use geom_core::spline::KnotVector;
    use geom_core::{Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// Two `x`-ruled rows of `(y, z)` profile points, degree 1 in `u`.
    fn ruled(kv: KnotVector, profile: &[(f64, f64)]) -> Surface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [-1.0, 1.0]
            .into_iter()
            .flat_map(|s| profile.iter().map(move |&(y, z)| Point3::new(s, y, z)))
            .collect();
        let n = 2 * profile.len();
        Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(ku, kv, control, vec![1.0; n]).unwrap(),
        ))
    }

    fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
        Surface::Plane {
            origin,
            normal: normal.normalize(),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn side(
        dep: &Surface<f64>,
        against: &Surface<f64>,
        p: Point3<f64>,
        t: Vec3<f64>,
    ) -> Result<FaceSide, SideEscalation> {
        dihedral_side(
            SideFace {
                surface: dep,
                sense: true,
            },
            SideFace {
                surface: against,
                sense: true,
            },
            p,
            EdgeWalk::of_half(t, true),
            1.0,
            Some(crate::NurbsLane::certified()),
            band(),
        )
        .map(|(s, _)| s)
    }

    const X: Vec3<f64> = Vec3::new(1.0, 0.0, 0.0);

    /// Claim 1: a C⁰ line need not be a knot run of multiplicity ≥ degree.
    /// Degree 2 in `v`, a SIMPLE knot at `½`, the middle two control rows
    /// coincident: the chart is the same fold as `review_rows::creased`
    /// (straight halves `(s, −a, a)` and `(s, a, a)`), but `crease_runs`
    /// lists nothing, so a foot within the band of the fold on the upper
    /// half is read for the lower face with the upper half's normal.
    #[test]
    fn a_kink_from_coincident_control_rows_escalates_like_a_knot_crease() {
        let kinked = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
            &[(-1.0, 1.0), (0.0, 0.0), (0.0, 0.0), (1.0, 1.0)],
        );
        let knot = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            &[(-1.0, 1.0), (0.0, 0.0), (1.0, 1.0)],
        );
        let floor = plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        // On the upper half, a tenth of the band from the fold: within
        // the band of the lower face too, so a point of its rim edge.
        let a = band().zero() / 10.0;
        let p = Point3::new(0.3, a, a);
        // The lower face, walking −x: rising off the floor, so Outside
        // if it is read at all.
        let reference = side(&knot, &floor, p, -X);
        let got = side(&kinked, &floor, p, -X);
        assert!(
            matches!(reference, Err(SideEscalation::Crease { .. })),
            "the knot-run fold read {reference:?}"
        );
        assert!(
            matches!(
                got,
                Err(SideEscalation::Crease { .. }) | Ok(FaceSide::Outside)
            ),
            "the coincident-row fold read {got:?} for the lower face"
        );
    }

    /// The same coincident-row fold: a point ON the upper half, 1.4 mm
    /// from the fold, is refused as decided off the face, because
    /// `NurbsSurface::project` accepts the zero-speed knot as a foot.
    #[test]
    fn a_point_on_a_kinked_chart_is_not_refused_off_it() {
        let kinked = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
            &[(-1.0, 1.0), (0.0, 0.0), (0.0, 0.0), (1.0, 1.0)],
        );
        let floor = plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let on = Point3::new(0.3, 1e-3, 1e-3);
        let got = side(&kinked, &floor, on, X);
        assert!(
            !matches!(got, Err(SideEscalation::OffFace { .. })),
            "a point on the chart read {got:?}"
        );
    }

    /// Claim 2: the residual is not bounded by a shallow fold. A hairpin
    /// carrier, degree 1 in `v` with its C⁰ knot at `½`: the half `v < ½`
    /// is the floor strip `y ∈ [0, 1]`, the half `v > ½` folds back over it
    /// at a rise of `h` over the unit run, `h` far inside the band. A face
    /// on the lower half, its edge the line `y = 3/7`, at a point on the
    /// upper half `3h/7` off it (inside the band): the foot is the upper
    /// half's, 0.43 m from the crease, and its normal is the lower half's reversed.
    #[test]
    fn a_hairpin_carrier_reads_the_face_half_not_the_overlying_half() {
        let h = band().zero() / 100.0;
        let hairpin = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            &[(1.0, 0.0), (0.0, 0.0), (1.0, h)],
        );
        let flat = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            &[(1.0, 0.0), (0.0, 0.0)],
        );
        // `(1/7, 3/7)` is a seed of `NurbsSurface::project`'s sweep on
        // both halves (8 per span), so the tie is broken by the distance.
        let y = 3.0 / 7.0;
        let cut = plane(Point3::new(0.0, y, 0.0), Vec3::new(0.0, 1.0, 1.0));
        let p = Point3::new(1.0 / 7.0, y, h * y);
        let reference = side(&flat, &cut, p, -X);
        let got = side(&hairpin, &cut, p, -X);
        assert!(reference.is_ok(), "the lone half read {reference:?}");
        assert!(
            got == reference || got.is_err(),
            "the hairpin read {got:?}, its face half alone {reference:?}"
        );
    }

    /// Claim 1, last bullet: a closed chart whose seam is tangent
    /// continuous has one normal there, so a foot on it should read it.
    /// Degree 2 in `v`, closed with the end tangent along the start's.
    #[test]
    fn a_c1_seam_reads_its_side() {
        let loop_ = ruled(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0], 2).unwrap(),
            &[
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (-1.0, 1.0),
                (-1.0, 0.0),
                (0.0, 0.0),
            ],
        );
        let cut = plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 1.0));
        let got = side(&loop_, &cut, Point3::new(0.3, 0.0, 0.0), X);
        assert!(got.is_ok(), "the C1 seam read {got:?}");
    }
}
