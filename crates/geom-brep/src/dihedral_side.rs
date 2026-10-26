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
//! interior lies to the left of the walk). The departing face lies on
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
//! is its chart's `S_u × S_v` at a certified foot of `p`, the foot
//! decided on the face by its distance within the band, and its arm is
//! the radius of its largest principal curvature there. The foot moves
//! the normal by at most `κ·δ` for a foot distance `δ` inside the band,
//! which over an arm of at most `1/κ` is a displacement of at most `δ`:
//! the reading's own tolerance.

use geom::{NurbsSurface, Surface};
use geom_core::k_stats::Magnitude;
use geom_core::{Band, Decide, Indeterminate, Margin, MarginDiag, Point3, Real};

use crate::certify::NurbsLane;

use crate::enters::{EntersMaterial, LeverEscalation, OutwardNormal, enters_material_reported};
use crate::implicit::{curvature_lever_arm, implicit_outward_normal};

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
    /// The arm, or the signed sine metered over it, did not decide.
    Lever(LeverEscalation),
    /// The tangent planes coincide at the point, so the departure has no
    /// first-order side: the side is the second order's to read.
    Tangent(MarginDiag),
}

/// **The side of `against` that `departing` leaves the shared edge on**
/// at `p` (module docs). `tangent` is the edge's direction at `p` as
/// `departing`'s loop walks it, of any nonzero length; `reach` is the
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
/// [`SideEscalation`]: a spline face's foot not certified onto it, the
/// arm or the reading undecided, or a tangent dihedral.
pub fn dihedral_side<T: Decide>(
    departing: SideFace<'_, T>,
    against: SideFace<'_, T>,
    p: Point3<T>,
    tangent: geom_core::Vec3<T>,
    reach: T,
    lane: Option<NurbsLane<T>>,
    band: Band,
) -> Result<(FaceSide, MarginDiag), SideEscalation> {
    let normal = |face, role| normal_at(face, role, p, reach, lane, band);
    let (n_dep, arm_dep) = normal(departing, SideRole::Departing)?;
    let (n_against, arm_against) = normal(against, SideRole::Against)?;
    let arm = reach.min(arm_dep).min(arm_against);
    let departure = n_dep.vec().cross(tangent.normalize());
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
    let jet = chart.ders(T::from_computed(foot.u), T::from_computed(foot.v));
    let kappa = crate::shape_operator::max_principal_curvature(&jet);
    let (arm, _) = crate::ssi::point_arm(kappa, reach);
    Ok((
        OutwardNormal::from_chart(jet.du.cross(jet.dv).normalize(), sense),
        arm,
    ))
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
        dihedral_side(departing, against, p, tangent, 1.0, lane(), band()).map(|(s, _)| s)
    }

    const ON_EDGE: Point3<f64> = Point3::new(0.3, 0.0, 0.0);
    const X: Vec3<f64> = Vec3::new(1.0, 0.0, 0.0);

    /// The analytic door reads nothing of a spline face: its gradient is
    /// poison there, so the dihedral escalates where this door reads a
    /// side.
    #[test]
    fn the_analytic_dihedral_reads_no_spline_face() {
        let w = floor_wall(1.0);
        assert!(
            crate::classify_dihedral(&w, &floor(), ON_EDGE, 1.0, band()).is_err(),
            "classify_dihedral is poison on a spline surface"
        );
        assert_eq!(
            side(face(&w, true), face(&floor(), true), ON_EDGE, X),
            Ok(FaceSide::Outside),
            "the spline arm reads the same pair"
        );
    }

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
                    X,
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
            X,
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
                X,
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
                X,
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
