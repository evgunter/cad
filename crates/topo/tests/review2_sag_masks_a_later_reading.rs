//! **PR 4918 second-review probe**: the table stops a pass at the first
//! sample whose only unsettled reading is the sag, so on the finest
//! schedule `NotCertifiable` is returned before the later samples are
//! read. A great circle of the meridian sphere turned about the point
//! it shares with the torus's meridian: sample 0 lies on both surfaces
//! (the sag is the only miss there), and samples further round stand
//! definitely off the torus, a measured contradiction.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::boolean::contact_verify::tangent_locus_relation;
use topo::contact::ContactRefusal;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn read(alpha: f64, start: f64) -> Result<topo::contact::ContactVerdict, ContactRefusal> {
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 1.0,
        minor_radius: 0.2,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let sphere = Surface::Sphere {
        center: Point3::new(1.0, 0.0, 0.0),
        radius: 0.2,
        axis: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    // The meridian's axis is y; turn it by alpha about x (the u_ref), so
    // the circle still passes through (1.2, 0, 0) and stays on the
    // sphere.
    let circle = Curve3::Circle {
        center: Point3::new(1.0, 0.0, 0.0),
        axis: Vec3::new(0.0, alpha.cos(), alpha.sin()),
        radius: 0.2,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    tangent_locus_relation(
        &torus,
        true,
        &sphere,
        false,
        &circle,
        start,
        start + core::f64::consts::TAU,
        true,
        band(),
    )
}

#[test]
fn r2_a_sag_only_first_sample_masks_a_later_contradiction() {
    let alpha = 0.3;
    // Starting off the shared point: the first sample is a measured miss.
    let off = read(alpha, 1.0);
    println!("start 1.0: {off:?}");
    // Starting on the shared point: sample 0 is on both surfaces.
    let on = read(alpha, 0.0);
    println!("start 0.0: {on:?}");
    assert!(
        matches!(off, Err(ContactRefusal::Contradicted { .. })),
        "the circle leaves the torus: {off:?}"
    );
    assert!(
        matches!(on, Err(ContactRefusal::Contradicted { .. })),
        "the same circle, its schedule started on the shared point: {on:?}"
    );
}

/// **The locus's escalation log carries escalations its outcome does
/// not rest on.** Near-coaxial cylinders (radii 1 and 0.5, axes
/// `3·zero` apart): the nested gap decides `NotTangent` definitely, but
/// the cross-section built eagerly beside the read decides its
/// direction on `tangent_locus_across`, the axes' offset, in band.
#[test]
fn r2_an_eager_section_logs_an_escalation_beside_a_definite_answer() {
    use geom_brep::{ExtentBall, TangentLocusError, tangent_locus};
    let b = band();
    let cyl = |y: f64, r: f64| Surface::Cylinder {
        origin: Point3::new(0.0, y, 0.0),
        axis: Vec3::new(1.0, 0.0, 0.0),
        radius: r,
        u_ref: Vec3::new(0.0, 0.0, 1.0),
    };
    let reach = ExtentBall::new(Point3::new(0.5, 0.0, 0.0), 1.5);
    let bracket = geom_core::k_stats::Bracket::open();
    let out = tangent_locus(&cyl(0.0, 1.0), &cyl(3.0 * b.zero(), 0.5), reach, b);
    let log = bracket.finish();
    println!("{out:?}");
    println!("escalations: {:?}", log.escalations);
    assert!(
        matches!(out, Err(TangentLocusError::NotTangent { .. })),
        "{out:?}"
    );
    assert!(
        log.escalations.is_empty(),
        "a definite answer, yet the log holds: {:?}",
        log.escalations
    );
}

/// The same shape in the face-clear splice: the plane face's extent
/// ends just off the ruling (its clearance in band) and the wall face
/// stands clear; `any` reads the plane face first, its escalation
/// spliced live beside `NotTangent`.
#[test]
fn r2_a_face_clear_answer_logs_the_other_faces_escalation() {
    use geom_brep::{ExtentBall, TangentLocusError, tangent_locus_of_faces};
    let b = band();
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 1.0 + 5.5 * b.zero()),
        axis: Vec3::new(1.0, 0.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 0.0, 1.0),
    };
    let reach = ExtentBall::new(Point3::new(0.5, 0.0, 0.0), 1.5);
    let flip = |dir: geom_core::UnitVec3<f64>, (lo, hi): (f64, f64)| {
        if dir.get().y > 0.0 {
            (lo, hi)
        } else {
            (-hi, -lo)
        }
    };
    let bracket = geom_core::k_stats::Bracket::open();
    let out = tangent_locus_of_faces(&plane, &cyl, reach, b, |d| {
        [Some(flip(d, (1e-6, 0.5))), Some(flip(d, (0.5, 1.0)))]
    });
    let log = bracket.finish();
    println!("{out:?}");
    println!("escalations: {:?}", log.escalations);
    assert!(
        matches!(out, Err(TangentLocusError::NotTangent { .. })),
        "{out:?}"
    );
    assert!(
        log.escalations.is_empty(),
        "a definite answer, yet the log holds: {:?}",
        log.escalations
    );
}
