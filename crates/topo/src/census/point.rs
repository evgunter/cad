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
//! or cross, read stale; carriers outside these lanes refuse typed.
//! Whether the solids cross at the point is the touch analysis's reading
//! of the faces around it, not this record's.

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use super::{
    CensusSubject, CensusUnsupportedCause, FaceContainment, ValidationError, curved, face_holds,
};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey};

/// A carrier these lanes read.
#[derive(Clone, Copy)]
enum Carrier<T: Real> {
    Line {
        origin: Point3<T>,
        dir: Vec3<T>,
    },
    Circle {
        center: Point3<T>,
        axis: Vec3<T>,
        radius: T,
    },
    Plane {
        origin: Point3<T>,
        normal: Vec3<T>,
    },
    Sphere {
        center: Point3<T>,
        radius: T,
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
enum Locus<T: Real> {
    At(Point3<T>),
    /// The carriers coincide nowhere, or cross.
    Apart,
    /// A circle lying in a plane: a curve touch, not a point one.
    Lies,
}

fn edge_carrier<T: Real>(body: &Body<T>, e: EdgeKey) -> Option<Carrier<T>> {
    let curve = body
        .edges
        .get(e)
        .and_then(|d| body.curves.get(d.curve))
        .and_then(crate::null::CurveGeom::certified)?;
    match *curve.carrier() {
        geom::Curve3::Line { origin, dir } => Some(Carrier::Line { origin, dir }),
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => Some(Carrier::Circle {
            center,
            axis,
            radius,
        }),
        _ => None,
    }
}

fn face_carrier<T: Real>(body: &Body<T>, f: FaceKey) -> Option<Carrier<T>> {
    let face = body.get_face(f)?;
    match *body.get_surface(face.surface)? {
        geom::Surface::Plane { origin, normal, .. } => Some(Carrier::Plane { origin, normal }),
        geom::Surface::Sphere { center, radius, .. } => Some(Carrier::Sphere { center, radius }),
        _ => None,
    }
}

fn decided<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Sign> {
    geom_core::k_stats::decide(name, margin, band)
        .map_err(|cause| errors.push(ValidationError::CensusEscalated { cause }))
        .ok()
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
) -> Option<Locus<T>> {
    Some(
        match decided("pm_census_point_gap", Margin::of(gap - reach), band, errors)? {
            Sign::Zero => Locus::At(at),
            _ => Locus::Apart,
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
) -> Option<Locus<T>> {
    if !separates(T::one() / r, r, band, errors)? {
        return Some(Locus::Apart);
    }
    let w = foot - c;
    let d = w.norm();
    // The touch point is on the sphere along `c → foot`, so a foot at
    // the centre (the carrier through it) crosses.
    if decided("pm_census_point_centre", Margin::of(d), band, errors)? != Sign::Positive {
        return Some(Locus::Apart);
    }
    coincident(d, r, c + w * (r / d), band, errors)
}

/// The two carriers' touch point (module docs). `Ok(None)` where a
/// decision escalated (pushed); `Err` for a pair outside these lanes.
fn locus<T: Decide>(
    a: Carrier<T>,
    b: Carrier<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Result<Option<Locus<T>>, ()> {
    use Carrier::{Circle, Line, Plane, Sphere};
    Ok(match (a, b) {
        (
            Circle {
                center,
                axis,
                radius,
            },
            Plane { origin, normal },
        )
        | (
            Plane { origin, normal },
            Circle {
                center,
                axis,
                radius,
            },
        ) => circle_plane((center, axis, radius), (origin, normal), band, errors),
        (Line { origin, dir }, Sphere { center, radius })
        | (Sphere { center, radius }, Line { origin, dir }) => {
            let foot = origin + dir * dir.dot(center - origin);
            on_sphere((center, radius), foot, band, errors)
        }
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
        _ => return Err(()),
    })
}

/// A circle `(c, a, r)` against a plane `(o, n)`: the circle's point
/// nearest the plane is `c ∓ r·d̂`, `d` the plane's normal across the
/// circle's plane, and `κ_rel = |d| / r`. A circle parallel to the
/// plane lies in it, or is apart.
fn circle_plane<T: Decide>(
    (c, a, r): (Point3<T>, Vec3<T>, T),
    (o, n): (Point3<T>, Vec3<T>),
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Locus<T>> {
    let d = n - a * a.dot(n);
    let sin = d.norm();
    let s = n.dot(c - o);
    if !separates(sin / r, r, band, errors)? {
        return Some(
            match decided("pm_census_point_plane", Margin::of(s), band, errors)? {
                Sign::Zero => Locus::Lies,
                _ => Locus::Apart,
            },
        );
    }
    let toward = d / sin;
    match decided("pm_census_point_plane", Margin::of(s), band, errors)? {
        Sign::Positive => coincident(s, r * sin, c - toward * r, band, errors),
        Sign::Negative => coincident(T::zero() - s, r * sin, c + toward * r, band, errors),
        Sign::Zero => Some(Locus::Apart),
    }
}

/// Two spheres: outside each other at `|c₂ − c₁| = r₁ + r₂`
/// (`κ_rel = 1/r₁ + 1/r₂`), or one inside the other at
/// `|c₂ − c₁| = |r₁ − r₂|` (`κ_rel = 1/r_small − 1/r_big`).
fn sphere_sphere<T: Decide>(
    (c1, r1): (Point3<T>, T),
    (c2, r2): (Point3<T>, T),
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<Locus<T>> {
    let v = c2 - c1;
    let dist = v.norm();
    if decided("pm_census_point_centre", Margin::of(dist), band, errors)? != Sign::Positive {
        return Some(Locus::Apart);
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
                return Some(Locus::Apart);
            }
            return Some(Locus::At(c1 + u * r1));
        }
        Sign::Positive => return Some(Locus::Apart),
        Sign::Negative => {}
    }
    let big = r1.max(r2);
    if !separates(T::one() / small - T::one() / big, small, band, errors)? {
        return Some(Locus::Apart);
    }
    // The point is on the big sphere away from the small one's centre.
    let (cb, out) = match decided("pm_census_point_radii", Margin::of(r1 - r2), band, errors)? {
        Sign::Positive => (c1, u),
        Sign::Negative => (c2, -u),
        Sign::Zero => return Some(Locus::Apart),
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
    locus: Option<Locus<T>>,
    lies: impl FnOnce() -> ValidationError,
    inside: impl FnOnce(Point3<T>, &mut Vec<ValidationError>) -> Option<bool>,
    errors: &mut Vec<ValidationError>,
) -> Read {
    match locus {
        None => Read::Pushed,
        Some(Locus::Apart) => Read::Stale,
        Some(Locus::Lies) => {
            errors.push(lies());
            Read::Pushed
        }
        Some(Locus::At(p)) => match inside(p, errors) {
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
    let carriers = edge_carrier(body, edge).zip(face_carrier(body, face));
    let locus = match carriers.map(|(e, f)| locus(e, f, band, errors)) {
        Some(Ok(locus)) => locus,
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
        locus,
        || {
            refused(
                subject,
                "a circle lying in a plane away from a conventional vertex: the census builds \
                 no curve-touch lane along an edge's carrier",
            )
        },
        |p, errors| match curved::on_curved_interior(body, edge, p, band, errors)? {
            true => in_face(body, face, p, band, errors),
            false => Some(false),
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
    let (ca, cb) = face_carrier(body, a).zip(face_carrier(body, b))?;
    let locus = locus(ca, cb, band, errors).ok()?;
    Some(read(
        locus,
        || unreachable!("two faces' carriers never lie one in the other on a point lane"),
        |p, errors| match in_face(body, a, p, band, errors)? {
            true => in_face(body, b, p, band, errors),
            false => Some(false),
        },
        errors,
    ))
}
