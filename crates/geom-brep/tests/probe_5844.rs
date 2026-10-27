//! Review probes on PR 5844 (G2d): the sphere/torus kind gate reads an
//! ellipse by `semi_u - semi_v` alone, before any incidence test.
use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::{PcurveCertifyError, chart_pcurve, recourse::Reading};
use geom_core::{Point3, Vec3};

fn sphere() -> Surface<f64> {
    Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() }
}
fn torus() -> Surface<f64> {
    Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: 2.0, minor_radius: 0.5, u_ref: Vec3::unit_x() }
}
fn show(what: &str, carrier: Curve3<f64>, s: Surface<f64>) -> String {
    let got = chart_pcurve(&carrier, &s, band());
    let text = match &got {
        Ok(_) => "OK".to_string(),
        Err(e) => format!("{e:?}\n   BUILD: {}", e.render(Reading::Build)),
    };
    eprintln!("PROBE {what} (eps={:e}): {text}", eps());
    text
}

#[test]
fn probe_5844_far_round_ellipse_reads_at_a_pose() {
    let e = eps();
    // A near-round ellipse 10 m from the unit sphere: definitely off it.
    let t = show(
        "near-round ellipse 10 m off the sphere",
        Curve3::Ellipse { center: Point3::new(0.0, 0.0, 10.0), axis: Vec3::unit_z(), semi_u: 0.8, semi_v: 0.8 - 0.5 * e, u_ref: Vec3::unit_x() },
        sphere(),
    );
    let t2 = show(
        "near-round ellipse 10 m off the torus",
        Curve3::Ellipse { center: Point3::new(0.0, 0.0, 10.0), axis: Vec3::unit_z(), semi_u: 2.5, semi_v: 2.5 - 0.5 * e, u_ref: Vec3::unit_x() },
        torus(),
    );
    // Reference: the circle in the same place.
    show(
        "circle 10 m off the sphere (reference)",
        Curve3::Circle { center: Point3::new(0.0, 0.0, 10.0), axis: Vec3::unit_z(), radius: 0.8, u_ref: Vec3::unit_x() },
        sphere(),
    );
    assert!(t.contains("defect") && t2.contains("defect"), "a definite miss ends in the lever");
}

#[test]
fn probe_5844_tiny_ellipse_within_eps_reads_definitely_off() {
    let e = eps();
    // An ellipse in the tangent plane at the pole, small enough that its
    // sagitta a^2/(2R) is 0.1·ε: every point lies within ε of the sphere,
    // set at mid-sagitta. Semi-axes a and a/2: far from round.
    let a = (0.2 * e).sqrt();
    let z = 1.0 - a * a / 4.0;
    let t = show(
        "tiny ellipse within eps of the sphere",
        Curve3::Ellipse { center: Point3::new(0.0, 0.0, z), axis: Vec3::unit_z(), semi_u: a, semi_v: 0.5 * a, u_ref: Vec3::unit_x() },
        sphere(),
    );
    // Same at the torus's outer equator: plane x = 2.5 - s, axis x.
    let t2 = show(
        "tiny ellipse within eps of the torus",
        Curve3::Ellipse { center: Point3::new(2.5 - (0.05 * e), 0.0, 0.0), axis: Vec3::unit_x(), semi_u: (0.1 * e * 0.5).sqrt(), semi_v: (0.1 * e * 0.5).sqrt() * 0.5, u_ref: Vec3::unit_z() },
        torus(),
    );
    assert!(!t.contains("defect") && !t2.contains("defect"), "a carrier within ε of its face ends as a defect");
}
