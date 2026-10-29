//! **Plane × spline: the section value over C3's whole reading.** The
//! section of a plane and a NURBS wall (an approximating surface read
//! as its fit) is [`crate::PlaneSection::all`] over the wall's whole
//! knot rectangle: every branch with its three-limb certificate, every
//! boundary region, and the accounting that they are all of them. Each
//! piece of it is a component:
//!
//! - a closed branch is a [`ComponentShape::Loop`], with its witness
//!   and its fitted carrier;
//! - an open branch, which runs between two crossings of the wall's
//!   rectangle, and a `Side` or `Corner` region, where the section runs
//!   along a side or leaves at a corner within the band, are
//!   [`ComponentShape::Open`], reaching the wall's chart boundary.
//!
//! A refusal maps by name: the march's transversality, a graze of a
//! side and a side held tangent are a pinch, named by the row that
//! decided it; any other refusal, an accounting that did not close
//! among them, is [`SectionError::Unaccounted`], carrying its cause.
//!
//! The reading is a march, so the arm is the march door's
//! ([`SsiLane::marched`], at `f64` alone).

use geom::Surface;
use geom_core::{Band, Point3, Real};

use super::{
    ClassCarrier, ComponentShape, FittedCarrier, PairPose, PairSection, SectionComponent, Sides,
};
use crate::ExtentBall;
use crate::intersect::SectionError;
use crate::ssi::{BranchEnd, SsiCertificate, SsiDomain, SsiError, SsiTube};

/// **A plane and a spline wall**, in the order the section was asked
/// of.
#[derive(Clone, Debug)]
pub struct PlaneSplinePose<T: Real> {
    plane: Surface<T>,
    wall: Surface<T>,
    /// The wall's net: a NURBS wall's own, an approximating surface's
    /// fit's.
    net: geom::NurbsSurface<T>,
    plane_first: bool,
}

impl<T: Real> PlaneSplinePose<T> {
    /// The pose of `a` and `b`, one a plane and the other a NURBS or an
    /// approximating surface; `None` for any other pair.
    #[must_use]
    pub fn of(a: &Surface<T>, b: &Surface<T>) -> Option<Self> {
        let (plane, wall, plane_first) = match (a, b) {
            (Surface::Plane { .. }, w) => (a, w, true),
            (w, Surface::Plane { .. }) => (b, w, false),
            _ => return None,
        };
        Some(Self {
            plane: plane.clone(),
            wall: wall.clone(),
            net: wall.spline_chart()?.clone(),
            plane_first,
        })
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        if self.plane_first {
            (&self.plane, &self.wall)
        } else {
            (&self.wall, &self.plane)
        }
    }

    /// The plane.
    pub fn plane(&self) -> &Surface<T> {
        &self.plane
    }

    /// The wall's net: a NURBS wall's own, an approximating surface's
    /// fit's.
    pub fn net(&self) -> &geom::NurbsSurface<T> {
        &self.net
    }

    /// `[on the plane, on the wall]` in the order the pose was asked of.
    pub(super) fn order<V>(&self, on_plane: V, on_wall: V) -> [V; 2] {
        if self.plane_first {
            [on_plane, on_wall]
        } else {
            [on_wall, on_plane]
        }
    }

    /// **A ball holding the whole section**: the wall lies in its net's
    /// convex hull (its weights are positive), so in the ball about the
    /// net's control points: about their mean, reaching the farthest.
    pub fn ball(&self) -> ExtentBall<T> {
        let control = self.net.control();
        let Some(&first) = control.first() else {
            return ExtentBall::new(Point3::origin(), T::zero());
        };
        let zero = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
        let mean = control.iter().fold(zero, |acc, &p| acc + (p - first))
            / T::from_f64(control.len() as f64);
        let center = first + mean;
        let radius = control
            .iter()
            .fold(T::zero(), |r, &p| r.max((p - center).norm()));
        ExtentBall::new(center, radius)
    }
}

impl PlaneSplinePose<f64> {
    /// **The reading's domain**: the plane's window, centred on its
    /// origin, reaching twice the farthest control point from it plus
    /// the feature extent, so it holds the wall and a march ends only on
    /// the wall's rectangle; the feature extent the net ball's diameter,
    /// the standard accounting floor.
    #[must_use]
    pub fn domain(&self) -> SsiDomain {
        let Surface::Plane { origin, .. } = self.plane else {
            unreachable!("the pose holds a plane")
        };
        let extent = self.ball().radius() * 2.0;
        let reach = self
            .net()
            .control()
            .iter()
            .map(|p| p.distance(origin))
            .fold(0.0, f64::max);
        SsiDomain {
            center: origin,
            half_extent: reach * 2.0 + extent,
            extent,
            floor_scale: 1.0,
        }
    }
}

/// The note a region's component carries.
const REGION: &str = "a Side or Corner region: the section runs along a side of the wall's \
                      rectangle, or leaves it at a corner, within the band, and no carrier is \
                      minted for it";

/// **How far a branch's witness may stand from the section**: its
/// carrier stands within the certificate's sup-norm bound of both
/// surfaces (limb 2), and the section within the tube's rung of the
/// carrier's foot (limb 3), so a point of the section lies within their
/// sum of the witness. A metre bound, at `f64`.
fn witness_reach(c: &SsiCertificate<f64>) -> f64 {
    let rung = match c.tube {
        SsiTube::Spatial { radius } => radius,
        SsiTube::Chart { rung, .. } | SsiTube::TwoChart { rung, .. } => rung,
    };
    c.hull_sup + rung
}

/// The whole reading's refusal as the section value's (module docs).
fn refused(e: SsiError) -> SectionError {
    let pinch = |predicate| SectionError::Pinched { predicate };
    match e {
        SsiError::TransversalityBand { .. } => pinch("ssi_transversality"),
        SsiError::BoundaryGraze { .. } => pinch("ssi_boundary_crossing"),
        SsiError::BoundaryTangent { .. } => pinch("ssi_boundary_strip"),
        other => SectionError::Unaccounted {
            cause: other.to_string(),
        },
    }
}

/// **The section of a plane and a spline wall** (module docs), the
/// march door's arm.
///
/// # Errors
///
/// [`SectionError::WrongLane`] for a pair that is not a plane and a
/// spline; the whole reading's refusal, mapped (module docs).
pub(super) fn section(
    a: &Surface<f64>,
    b: &Surface<f64>,
    band: Band,
) -> Result<PairSection<f64>, SectionError> {
    let pose = PlaneSplinePose::of(a, b).ok_or(SectionError::WrongLane {
        expected: "a plane and a NURBS or approximating surface",
    })?;
    let out = crate::ssi::plane_nurbs_ssi(pose.plane(), pose.net(), pose.domain(), band)
        .map_err(refused)?;
    let open = ComponentShape::Open {
        reaches: pose.order(false, true),
    };
    let mut components: Vec<SectionComponent<f64>> = out
        .branches
        .into_iter()
        .map(|b| {
            let closed = matches!(b.end, BranchEnd::Closed);
            SectionComponent {
                shape: if closed { ComponentShape::Loop } else { open },
                winds: [false, false],
                witness: Some(b.witness),
                residual: witness_reach(&b.certificate),
                carrier: ClassCarrier::Fitted(FittedCarrier {
                    carrier: b.carrier,
                    span: b.params,
                    closed,
                    certificate: b.certificate,
                    witness: b.witness,
                }),
            }
        })
        .collect();
    components.extend(out.boundary.iter().map(|_| SectionComponent {
        shape: open,
        winds: [false, false],
        witness: None::<Point3<f64>>,
        residual: 0.0,
        carrier: ClassCarrier::Unminted(REGION),
    }));
    let lever = pose.ball().radius();
    Ok(PairSection {
        pose: PairPose::PlaneSpline(Box::new(pose)),
        decided: Vec::new(),
        components,
        sides: Sides::Untold(
            "a spline section's components have no closed-form reading that tells them apart",
        ),
        lever,
    })
}
