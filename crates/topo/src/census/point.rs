//! The confirm lane for a **point touch** (C3): two cells whose
//! interiors meet at isolated points without crossing — an edge's span
//! touching a face's carrier, a plane on a sphere, two spheres.
//!
//! An edge against a face is read through the curve × face crossing
//! reading ([`geom_brep::curve_face::cross()`]), every pair its table has
//! built: the record is confirmed where the pose is a nondegenerate
//! touch (`Tangent`, its relative curvature definitely positive) strictly
//! inside the edge's span, and the touch point strictly inside both
//! cells. A face pair is found on the closed-form lane of the two
//! carriers, and confirmed there when three readings hold: the relative
//! curvature `κ_rel` of the two carriers across the point is definitely
//! positive (their second-order separation, levered by the curved
//! carrier's radius), the carriers coincide at it within ε, and the
//! point is strictly inside both cells. Carriers that coincide nowhere,
//! or cross, read stale. A face pair outside these lanes is not this
//! lane's ([`face_face`] answers `None`). Whether the solids cross at the
//! point is the touch analysis's reading of the faces around it, not
//! this record's.
//!
//! The closed forms here are touch points, not the crossing sections
//! `boolean::section_cert` derives for the same carrier pairs (a
//! sample of the section circle, its touch a band fallback) nor the
//! ray roots `solid_contain` reads: neither answers the point
//! where the carriers come nearest.

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use super::curved::{self, decided_as};
use super::{CensusSubject, CensusUnsupportedCause, FaceContainment, ValidationError};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey};

/// A face carrier these lanes read.
#[derive(Clone, Copy)]
enum Surface<T: Real> {
    Plane {
        origin: Point3<T>,
        normal: Vec3<T>,
    },
    Sphere {
        center: Point3<T>,
        radius: T,
        /// The sphere's curvature arm, read off the face's carrier.
        arm: geom_core::Lever<T>,
    },
}

/// What a point-touch reading answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Read {
    /// The two cells' interiors meet at the touch point.
    Met,
    /// They do not: apart, crossing, or the point outside a cell.
    Stale,
    /// A decision escalated or the lane refused (pushed).
    Pushed,
}

/// Where two carriers touch, on their closed-form lane.
enum Touch<T: Real> {
    At(Point3<T>),
    /// The carriers coincide nowhere, or cross.
    Apart,
}

fn surface<T: Real>(body: &Body<T>, f: FaceKey) -> Option<Surface<T>> {
    let carrier = body.face(f)?.surface();
    match *carrier {
        geom::Surface::Plane { frame, .. } => {
            let (origin, normal) = (frame.origin(), frame.axis());
            Some(Surface::Plane { origin, normal })
        }
        geom::Surface::Sphere { frame, radius, .. } => {
            let center = frame.origin();
            Some(Surface::Sphere {
                center,
                radius,
                arm: geom_brep::uniform_curvature_arm(carrier),
            })
        }
        _ => None,
    }
}

/// `κ_rel` definitely positive: `Some(true)`; in band, `Some(false)`;
/// `None` escalated (pushed).
fn separates<T: Decide>(
    kappa: T,
    arm: geom_core::Lever<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    Some(
        decided_as(
            "pm_census_point_kappa",
            Margin::sagitta(kappa, arm),
            band,
            errors,
        )? == Sign::Positive,
    )
}

/// The candidate point `at`, met where `gap − reach` reads zero: `gap`
/// how far one carrier's nearest feature stands from the other, `reach`
/// how far the other carrier comes toward it.
fn coincident<T: Decide>(
    gap: T,
    reach: T,
    at: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Touch<T>> {
    Some(
        match decided_as("pm_census_point_gap", Margin::of(gap - reach), band, errors)? {
            Sign::Zero => Touch::At(at),
            _ => Touch::Apart,
        },
    )
}

/// A sphere `(c, r)` against a line or a plane whose nearest point to
/// `c` is `foot` (`κ_rel = 1/r`).
fn on_sphere<T: Decide>(
    (c, r, arm): (Point3<T>, T, geom_core::Lever<T>),
    foot: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Touch<T>> {
    if !separates(T::one() / r, arm, band, errors)? {
        return Some(Touch::Apart);
    }
    let w = foot - c;
    let d = w.norm();
    // The touch point is on the sphere along `c → foot`, so a foot at
    // the centre (the carrier through it) crosses.
    if decided_as("pm_census_point_centre", Margin::of(d), band, errors)? != Sign::Positive {
        return Some(Touch::Apart);
    }
    coincident(d, r, c + w * (r / d), band, errors)
}

/// Two spheres: outside each other at `|c₂ − c₁| = r₁ + r₂`
/// (`κ_rel = 1/r₁ + 1/r₂`), or one inside the other at
/// `|c₂ − c₁| = |r₁ − r₂|` (`κ_rel = 1/r_small − 1/r_big`).
fn sphere_sphere<T: Decide>(
    (c1, r1, arm1): (Point3<T>, T, geom_core::Lever<T>),
    (c2, r2, arm2): (Point3<T>, T, geom_core::Lever<T>),
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Touch<T>> {
    let v = c2 - c1;
    let (u, dist) = v.direction_and_length();
    if decided_as("pm_census_point_centre", Margin::of(dist), band, errors)? != Sign::Positive {
        return Some(Touch::Apart);
    }
    let small = r1.min(r2);
    match decided_as(
        "pm_census_point_gap",
        Margin::of(dist - (r1 + r2)),
        band,
        errors,
    )? {
        Sign::Zero => {
            if !separates(T::one() / r1 + T::one() / r2, arm1.min(arm2), band, errors)? {
                return Some(Touch::Apart);
            }
            return Some(Touch::At(c1 + u * r1));
        }
        Sign::Positive => return Some(Touch::Apart),
        Sign::Negative => {}
    }
    let big = r1.max(r2);
    if !separates(
        T::one() / small - T::one() / big,
        arm1.min(arm2),
        band,
        errors,
    )? {
        return Some(Touch::Apart);
    }
    // The point is on the big sphere away from the small one's centre.
    let (cb, out) = match decided_as("pm_census_point_radii", Margin::of(r1 - r2), band, errors)? {
        Sign::Positive => (c1, u),
        Sign::Negative => (c2, -u),
        Sign::Zero => return Some(Touch::Apart),
    };
    coincident(dist, big - small, cb + out * big, band, errors)
}

fn refused(subject: CensusSubject, what: &'static str) -> ValidationError {
    ValidationError::CensusUnsupported {
        subject,
        cause: CensusUnsupportedCause::ContactLane(
            crate::contact::ContactRefusal::NotCertifiable { what },
        ),
    }
}

/// Whether `face` holds `p`, a touch point on its carrier, strictly
/// inside it. The touch is the closed form of a point of both carriers
/// (a plane's foot, or a sphere's point the touch's own reading decided
/// on it, [`on_sphere`], [`sphere_sphere`]), on each to within the band,
/// so it is held in band ([`crate::held::Held::in_band`]) and placed with
/// no on-carrier decision ([`super::placed`]).
fn in_face<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    match super::placed(body, face, crate::held::Held::in_band(p), band, errors)? {
        Some(at) => Some(at == FaceContainment::In),
        None => {
            errors.push(refused(
                CensusSubject::Entity(EntityId::Face(face)),
                "a point-touch record on a face whose trim the containment door does not read",
            ));
            None
        }
    }
}

fn read<T: Real>(
    touch: Option<Touch<T>>,
    inside: impl FnOnce(Point3<T>, &mut Vec<ValidationError>) -> Option<bool>,
    errors: &mut Vec<ValidationError>,
) -> Read {
    match touch {
        None => Read::Pushed,
        Some(Touch::Apart) => Read::Stale,
        Some(Touch::At(p)) => match inside(p, errors) {
            Some(true) => Read::Met,
            Some(false) => Read::Stale,
            None => Read::Pushed,
        },
    }
}

/// **One edge–face record read as a point touch**, through the curve ×
/// face crossing reading of the edge's span against the face's carrier
/// (module docs): a nondegenerate touch strictly inside the span is the
/// candidate, checked inside both cells; apart, crossings only, or a
/// touch at an end of the span is stale; a span lying on the carrier or
/// through a cone's apex, and a window, refuse typed.
pub(super) fn edge_face<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    face: FaceKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Read {
    use geom_brep::curve_face::{CrossWhy, OnSpan, Pose, PoseRefusal, cross};
    use geom_brep::curve_pair::MeetKind;
    let subject = CensusSubject::Entity(EntityId::Edge(edge));
    let Some(span) = curved::span_of(body, edge, band, errors) else {
        return Read::Pushed;
    };
    let Some(surface) = body.face(face).map(|f| f.surface().clone()) else {
        errors.push(refused(
            CensusSubject::Entity(EntityId::Face(face)),
            "a point-touch record on a face with no surface",
        ));
        return Read::Pushed;
    };
    let touch = match cross(&span.span(), &surface, band) {
        Ok(found) => match found.pose {
            Pose::Tangent(_) => match found.meetings.first() {
                Some(m) if matches!(m.on_span, OnSpan::Inside { .. }) => Touch::At(m.point),
                _ => Touch::Apart,
            },
            Pose::Apart | Pose::Isolated(_) => Touch::Apart,
            Pose::Lies(_) => {
                errors.push(refused(
                    subject,
                    "an edge lying on a face's carrier away from a conventional vertex: the \
                     census builds no curve-touch lane along an edge's carrier",
                ));
                return Read::Pushed;
            }
            Pose::Through(_) => {
                errors.push(refused(
                    subject,
                    "an edge through a cone face's apex: a point the cone has no tangent \
                     plane at, read by the apex vertex's records",
                ));
                return Read::Pushed;
            }
        },
        Err(refusal) => {
            let cause = refusal.windows.first().and_then(|w| match w.why {
                CrossWhy::Pose(PoseRefusal::InBand { diag, .. }) => Some(diag),
                CrossWhy::Method { fault, .. } => match fault.kind() {
                    MeetKind::Escalated(diag) | MeetKind::Touch(diag) => Some(diag),
                    MeetKind::Unread(_) | MeetKind::Fault { .. } => None,
                },
                CrossWhy::Pose(_) => None,
            });
            errors.push(cause.map_or_else(
                || {
                    refused(
                        subject,
                        "an edge–face point touch the curve × face reading does not read: an \
                         unbuilt or spline pair, a meter that refused, or a torus outside its \
                         convention",
                    )
                },
                |cause| ValidationError::CensusEscalated { cause },
            ));
            return Read::Pushed;
        }
    };
    read(
        Some(touch),
        |p, errors| {
            super::in_both(
                |errors| curved::on_curved_interior(body, edge, p, band, errors),
                |errors| in_face(body, face, p, band, errors),
                errors,
            )
        },
        errors,
    )
}

/// One face–face record read as a point touch, where its two carriers
/// are a sphere and a plane or two spheres; `None` for other carriers.
pub(super) fn face_face<T: Decide>(
    body: &Body<T>,
    a: FaceKey,
    b: FaceKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Read> {
    use Surface::{Plane, Sphere};
    let touch = match surface(body, a).zip(surface(body, b))? {
        (
            Plane { origin, normal },
            Sphere {
                center,
                radius,
                arm,
            },
        )
        | (
            Sphere {
                center,
                radius,
                arm,
            },
            Plane { origin, normal },
        ) => {
            let foot = center - normal * geom::plane_offset(origin, normal, center);
            on_sphere((center, radius, arm), foot, band, errors)
        }
        (
            Sphere {
                center: c1,
                radius: r1,
                arm: arm1,
            },
            Sphere {
                center: c2,
                radius: r2,
                arm: arm2,
            },
        ) => sphere_sphere((c1, r1, arm1), (c2, r2, arm2), band, errors),
        (Plane { .. }, Plane { .. }) => return None,
    };
    Some(read(
        touch,
        |p, errors| {
            super::in_both(
                |errors| in_face(body, a, p, band, errors),
                |errors| in_face(body, b, p, band, errors),
                errors,
            )
        },
        errors,
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// How far the touch point the curve × face pose reads stands off
    /// the exact one: the unit circle about the origin, its axis `3e-11`
    /// rad off the plane's normal (both off every coordinate axis) and
    /// stored times `facing`, against the plane its nearest point lies
    /// on, at ε = 1e-12 (K = 10). The exact touch direction is that of
    /// these stored vectors, rounded.
    fn touch_off(facing: f64) -> f64 {
        use geom_brep::curve_face::Pose;
        let n = Vec3::new(0.347_008_881_792_054_3, -0.937_861_842_681_227_9, 0.0);
        let a = Vec3::new(
            0.347_008_881_788_918_5,
            -0.937_861_842_682_388_1,
            -2.981_308_812_513_786_4e-11,
        ) * facing;
        let exact = Vec3::new(
            0.104_527_924_895_754_38,
            0.038_675_332_179_441_47,
            0.993_769_657_213_278_7,
        );
        let sin = 2.999_999_839_876_325_4e-11;
        let c = Point3::origin();
        let band = Band::new(1e-12, 1e-11).unwrap();
        let circle = geom::Curve3::circle(c, a, 1.0, a.orthonormal_basis().0);
        let plane = geom::Surface::Plane {
            frame: geom::Frame::new(c - n * sin, n, n.orthonormal_basis().0),
        };
        let span = geom_brep::curve_pair::Span::new(&circle, (0.0, core::f64::consts::TAU));
        let at = match geom_brep::curve_face::pose(&span, &plane, band) {
            Ok(Pose::Tangent(touch)) => touch.point,
            other => panic!("no touch read: {other:?}"),
        };
        ((c - at) - exact).norm()
    }

    /// **A circle `3e-11` rad off parallel to a plane is touched where it
    /// touches** ([`touch_off`]), to a few ulps. Formed from `a × n` as
    /// it stands it was off by 1.8e-6, and the touch point that far
    /// along the circle.
    #[test]
    fn a_circle_nearly_parallel_to_a_plane_is_touched_where_it_touches() {
        let off = touch_off(1.0);
        assert!(
            off <= 1e-15,
            "the touch point stands {off:e} off the circle's"
        );
    }

    /// **A circle facing away from the plane is touched where it
    /// touches**: the same circle with its axis stored reversed
    /// (`a·n < 0`, the `n + a` arm), read to the same few ulps.
    #[test]
    fn a_circle_facing_away_from_a_plane_is_touched_where_it_touches() {
        let off = touch_off(-1.0);
        assert!(
            off <= 1e-15,
            "the touch point stands {off:e} off the circle's"
        );
    }
}
