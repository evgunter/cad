//! The confirm lane for a **point touch** (C3): two cells whose
//! interiors meet at isolated points without crossing — a circle on a
//! plane, a line or a plane on a sphere, two spheres.
//!
//! The point is found on the closed-form lane of the two carriers, and
//! the record is confirmed there when three readings hold: the relative
//! curvature `κ_rel` of the two carriers across the point is definitely
//! positive (their second-order separation, levered by the curved
//! carrier's radius), the carriers coincide at it within ε, and the
//! point is strictly inside both cells. Carriers that coincide nowhere,
//! or cross, read stale. An edge–face pair outside these lanes refuses
//! typed; a face pair outside them is not this lane's
//! ([`face_face`] answers `None`). Whether the solids cross at the
//! point is the touch analysis's reading of the faces around it, not
//! this record's.
//!
//! The closed forms here are touch points, not the crossing sections
//! `boolean::section_cert` derives for the same carrier pairs (a
//! sample of the section circle, its touch a band fallback) nor the
//! ray roots `boolean::solid_contain` reads: neither answers the point
//! where the carriers come nearest.

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use super::curved::{self, decided};
use super::{CensusSubject, CensusUnsupportedCause, FaceContainment, ValidationError, face_holds};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey};

/// A face carrier these lanes read.
#[derive(Clone, Copy)]
enum Surface<T: Real> {
    Plane { origin: Point3<T>, normal: Vec3<T> },
    Sphere { center: Point3<T>, radius: T },
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

/// Where a circle and a plane touch.
enum Locus<T: Real> {
    Touch(Touch<T>),
    /// The circle lies in the plane: a curve touch, not a point one.
    Lies,
}

fn surface<T: Real>(body: &Body<T>, f: FaceKey) -> Option<Surface<T>> {
    let face = body.get_face(f)?;
    match *body.get_surface(face.surface)? {
        geom::Surface::Plane { origin, normal, .. } => Some(Surface::Plane { origin, normal }),
        geom::Surface::Sphere { center, radius, .. } => Some(Surface::Sphere { center, radius }),
        _ => None,
    }
}

/// `κ_rel` definitely positive: `Some(true)`; in band, `Some(false)`;
/// `None` escalated (pushed).
fn separates<T: Decide>(
    kappa: T,
    arm: T,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    Some(
        decided(
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
        match decided("pm_census_point_gap", Margin::of(gap - reach), band, errors)? {
            Sign::Zero => Touch::At(at),
            _ => Touch::Apart,
        },
    )
}

/// A sphere `(c, r)` against a line or a plane whose nearest point to
/// `c` is `foot` (`κ_rel = 1/r`).
fn on_sphere<T: Decide>(
    (c, r): (Point3<T>, T),
    foot: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Touch<T>> {
    if !separates(T::one() / r, r, band, errors)? {
        return Some(Touch::Apart);
    }
    let w = foot - c;
    let d = w.norm();
    // The touch point is on the sphere along `c → foot`, so a foot at
    // the centre (the carrier through it) crosses.
    if decided("pm_census_point_centre", Margin::of(d), band, errors)? != Sign::Positive {
        return Some(Touch::Apart);
    }
    coincident(d, r, c + w * (r / d), band, errors)
}

/// A circle `(c, a, r)` against a plane `(o, n)`: the circle's point
/// nearest the plane is `c ∓ r·d̂`, `d = (a × n) × a` the plane's
/// normal across the circle's plane, and `κ_rel = |d| / r`. A circle
/// parallel to the plane lies in it, or is apart.
fn circle_plane<T: Decide>(
    (c, a, r): (Point3<T>, Vec3<T>, T),
    (o, n): (Point3<T>, Vec3<T>),
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Locus<T>> {
    let d = plane_across(a, n);
    let sin = d.norm();
    let s = n.dot(c - o);
    if !separates(sin / r, r, band, errors)? {
        return Some(
            match decided("pm_census_point_plane", Margin::of(s), band, errors)? {
                Sign::Zero => Locus::Lies,
                _ => Locus::Touch(Touch::Apart),
            },
        );
    }
    let toward = d / sin;
    Some(Locus::Touch(
        match decided("pm_census_point_plane", Margin::of(s), band, errors)? {
            Sign::Positive => coincident(s, r * sin, c - toward * r, band, errors)?,
            Sign::Negative => coincident(T::zero() - s, r * sin, c + toward * r, band, errors)?,
            Sign::Zero => Touch::Apart,
        },
    ))
}

/// The plane's normal `n` across the circle's plane, `(a × n) × a`, to
/// a few ulps of its own length.
///
/// Not `n − a·(a·n)`: as the circle turns parallel to the plane that
/// difference cancels, and its component along `n` (`sin²`, which
/// places the touch point off the circle's centre plane) rounds away.
/// Nor `a × n` as it stands: near parallel its terms cancel to `sin`,
/// each off by `u`, so the direction is `u/sin` off and the touch point
/// `r·u/sin` along the circle. `a × n = a × (n ∓ a)`, and `n − a` (or
/// `n + a`, the circle facing away) is formed to `u` of its own size,
/// each coordinate a difference of two nearly equal terms or one that
/// keeps half the larger, so its cross with `a` cancels nothing. Away
/// from parallel both forms are as good, so the choice at `a·n = 0` is
/// free.
fn plane_across<T: Real>(a: Vec3<T>, n: Vec3<T>) -> Vec3<T> {
    let across = |m: Vec3<T>| a.cross(m).cross(a);
    let (facing, away) = (across(n - a), across(n + a));
    let key = a.dot(n);
    Vec3::new(
        key.select_le_zero(away.x, facing.x),
        key.select_le_zero(away.y, facing.y),
        key.select_le_zero(away.z, facing.z),
    )
}

/// Two spheres: outside each other at `|c₂ − c₁| = r₁ + r₂`
/// (`κ_rel = 1/r₁ + 1/r₂`), or one inside the other at
/// `|c₂ − c₁| = |r₁ − r₂|` (`κ_rel = 1/r_small − 1/r_big`).
fn sphere_sphere<T: Decide>(
    (c1, r1): (Point3<T>, T),
    (c2, r2): (Point3<T>, T),
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Touch<T>> {
    let v = c2 - c1;
    let dist = v.norm();
    if decided("pm_census_point_centre", Margin::of(dist), band, errors)? != Sign::Positive {
        return Some(Touch::Apart);
    }
    let u = v / dist;
    let small = r1.min(r2);
    match decided(
        "pm_census_point_gap",
        Margin::of(dist - (r1 + r2)),
        band,
        errors,
    )? {
        Sign::Zero => {
            if !separates(T::one() / r1 + T::one() / r2, small, band, errors)? {
                return Some(Touch::Apart);
            }
            return Some(Touch::At(c1 + u * r1));
        }
        Sign::Positive => return Some(Touch::Apart),
        Sign::Negative => {}
    }
    let big = r1.max(r2);
    if !separates(T::one() / small - T::one() / big, small, band, errors)? {
        return Some(Touch::Apart);
    }
    // The point is on the big sphere away from the small one's centre.
    let (cb, out) = match decided("pm_census_point_radii", Margin::of(r1 - r2), band, errors)? {
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

/// Whether `face` holds `p` strictly inside it.
fn in_face<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    match face_holds(body, face, p, band, "pm_census_point_on_plane", errors)? {
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

/// One edge–face record read as a point touch: a circle on a plane, or
/// a line on a sphere.
pub(super) fn edge_face<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    face: FaceKey,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Read {
    let subject = CensusSubject::Entity(EntityId::Edge(edge));
    let Some(curve) = curved::carrier(body, edge, band, errors) else {
        return Read::Pushed;
    };
    let touch = match (curve, surface(body, face)) {
        (
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                ..
            },
            Some(Surface::Plane { origin, normal }),
        ) => match circle_plane((center, axis, radius), (origin, normal), band, errors) {
            None => None,
            Some(Locus::Touch(touch)) => Some(touch),
            Some(Locus::Lies) => {
                errors.push(refused(
                    subject,
                    "a circle lying in a plane away from a conventional vertex: the census \
                     builds no curve-touch lane along an edge's carrier",
                ));
                return Read::Pushed;
            }
        },
        (geom::Curve3::Line { origin, dir }, Some(Surface::Sphere { center, radius })) => {
            let foot = origin + dir * dir.dot(center - origin);
            on_sphere((center, radius), foot, band, errors)
        }
        _ => {
            errors.push(refused(
                subject,
                "an edge–face point touch is read on a circle and a plane or a line and a \
                 sphere only",
            ));
            return Read::Pushed;
        }
    };
    read(
        touch,
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
        (Plane { origin, normal }, Sphere { center, radius })
        | (Sphere { center, radius }, Plane { origin, normal }) => {
            let foot = center - normal * normal.dot(center - origin);
            on_sphere((center, radius), foot, band, errors)
        }
        (
            Sphere {
                center: c1,
                radius: r1,
            },
            Sphere {
                center: c2,
                radius: r2,
            },
        ) => sphere_sphere((c1, r1), (c2, r2), band, errors),
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

    /// How far the touch point read stands off the exact one: the unit
    /// circle about the origin, its axis `3e-11` rad off the plane's
    /// normal (both off every coordinate axis) and stored times
    /// `facing`, against the plane its nearest point lies on, at
    /// ε = 1e-12 (K = 10). The exact touch direction is that of these
    /// stored vectors, rounded.
    fn touch_off(facing: f64) -> f64 {
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
        let mut errors = Vec::new();
        let at = match circle_plane((c, a, 1.0), (c - n * sin, n), band, &mut errors) {
            Some(Locus::Touch(Touch::At(p))) => p,
            _ => panic!("no touch point read: {errors:?}"),
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
