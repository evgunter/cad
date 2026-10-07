//! Review probe (PR 4280): band-edge poses per converted row, tilt in the
//! direction that moves the datum. Prints `E <row> <datum> <tilt> <label>`.

#![allow(clippy::panic)]

use crate::shared::tol::band;
use geom::Surface;
use geom_brep::intersect::{
    RadiusEvidence, cone_cylinder_section, cylinder_cylinder_section, plane_cone_section,
    plane_cylinder_section, plane_torus_section,
};
use geom_brep::{ExtentBall, Reach, SectionError, TangentLocusError, tangent_locus};
use geom_core::{Point3, Vec3};

const R: f64 = 0.25;

fn turned(from: Vec3<f64>, toward: Vec3<f64>, theta: f64) -> Vec3<f64> {
    from * theta.cos() + toward * theta.sin()
}
fn lab<T: std::fmt::Debug>(r: &Result<T, SectionError>) -> String {
    match r {
        Ok(v) => format!("{v:?}").split([' ', '(', '{']).next().unwrap().to_string(),
        Err(SectionError::Escalated(d)) => format!("ESC:{}", d.predicate.unwrap_or("?")),
        Err(e) => format!("ERR:{}", format!("{e:?}").split([' ', '(', '{']).next().unwrap()),
    }
}

#[test]
#[ignore = "review probe"]
fn review_band_edges() {
    let b = band();
    let z = b.zero();
    let k = b.escalate() / z;
    let grid = [
        (0.45, 0.45),
        (0.55, 0.55),
        (0.0, 0.95),
        (0.95, 0.0),
        (k + 0.3, 0.6),
        (k + 0.7, 0.6),
        (-(k + 0.3), 0.6),
        (-(k + 0.7), 0.6),
        (k + 0.3, -0.6),
        (-(k + 0.3), -0.6),
    ];
    for (ds, ts) in grid {
        let (d, t) = (ds * z, ts * z);
        // pc: datum r − |gap| = −lift; tilt toward the plane normal, lever 0.75.
        {
            let plane = Surface::Plane { origin: Point3::origin(), normal: Vec3::unit_z(), u_ref: Vec3::unit_x() };
            let wall = Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, R - d),
                axis: turned(Vec3::unit_x(), Vec3::unit_z(), t / 0.75),
                radius: R,
                u_ref: Vec3::unit_y(),
            };
            let reach = Reach::Ball(ExtentBall::new(Point3::origin(), 0.5));
            println!("E pc_parallel_gap {ds:.2} {ts:.2} {}", lab(&plane_cylinder_section(&plane, &wall, &reach, b)));
        }
        // cc: equal radii, axes `2R − d` apart along y, the second tilted toward y.
        for (row, sep, r2) in [("cc_parallel_gap", 2.0 * R - d, R), ("cc_coaxial", d.abs(), R), ("internal_gap", 2.0 * R + d, 3.0 * R)] {
            let c1 = Surface::Cylinder { origin: Point3::origin(), axis: Vec3::unit_x(), radius: R, u_ref: Vec3::unit_y() };
            let c2 = Surface::Cylinder {
                origin: Point3::new(0.0, sep, 0.0),
                axis: turned(Vec3::unit_x(), Vec3::unit_y(), t / (R + 1.0)),
                radius: r2,
                u_ref: Vec3::unit_z(),
            };
            let ball = ExtentBall::new(Point3::new(0.0, R, 0.0), 1.0);
            if row == "internal_gap" {
                let w = tangent_locus(&c1, &c2, ball, b);
                let l = match &w {
                    Ok(v) => format!("{v:?}").split([' ', '(', '{']).next().unwrap().to_string(),
                    Err(TangentLocusError::Escalated(d)) => format!("ESC:{}", d.predicate.unwrap_or("?")),
                    Err(e) => format!("{e:?}").split([' ', '(', '{']).next().unwrap().to_string(),
                };
                println!("E {row} {ds:.2} {ts:.2} {l}");
            } else {
                let got = cylinder_cylinder_section(&c1, &c2, RadiusEvidence::Declared, &Reach::Ball(ball), b);
                println!("E {row} {ds:.2} {ts:.2} {}", lab(&got));
            }
        }
        // coc: cone along x, cylinder offset `|d|` along y, tilted toward y; extent 1.
        {
            let cone = Surface::Cone { apex: Point3::origin(), axis: Vec3::unit_x(), half_angle: 0.5, u_ref: Vec3::unit_y() };
            let wall = Surface::Cylinder {
                origin: Point3::new(0.0, d.abs(), 0.0),
                axis: turned(Vec3::unit_x(), Vec3::unit_y(), t),
                radius: R,
                u_ref: Vec3::unit_z(),
            };
            println!("E coc_coaxial {ds:.2} {ts:.2} {}", lab(&cone_cylinder_section(&cone, &wall, 1.0, b)));
        }
        let torus = Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: 1.0, minor_radius: R, u_ref: Vec3::unit_x() };
        // pt cap: datum r − |h| = −lift; tilt at radius 1.
        {
            let plane = Surface::Plane {
                origin: Point3::new(0.0, 0.0, R - d),
                normal: turned(Vec3::unit_z(), Vec3::unit_x(), t),
                u_ref: Vec3::unit_y(),
            };
            println!("E pt_cap_gap {ds:.2} {ts:.2} {}", lab(&plane_torus_section(&plane, &torus, 2.0, b)));
        }
        // pt meridian: stand-off d; tilt over extent 2.
        {
            let plane = Surface::Plane {
                origin: Point3::new(0.0, d, 0.0),
                normal: turned(Vec3::unit_y(), Vec3::unit_z(), t / 2.0),
                u_ref: Vec3::unit_x(),
            };
            println!("E pt_axis_plane_gap {ds:.2} {ts:.2} {}", lab(&plane_torus_section(&plane, &torus, 2.0, b)));
        }
        // pt spiric: stand-off (R − r) − d.
        {
            let plane = Surface::Plane {
                origin: Point3::new(0.0, (1.0 - R) - d, 0.0),
                normal: turned(Vec3::unit_y(), Vec3::unit_z(), t / 2.0),
                u_ref: Vec3::unit_x(),
            };
            println!("E pt_spiric_two_ovals {ds:.2} {ts:.2} {}", lab(&plane_torus_section(&plane, &torus, 2.0, b)));
        }
        // pn apex: discriminant d over a unit extent, apex gap t.
        {
            let alpha: f64 = 0.5;
            let cone = Surface::Cone { apex: Point3::origin(), axis: Vec3::unit_z(), half_angle: alpha, u_ref: Vec3::unit_x() };
            let beta = core::f64::consts::FRAC_PI_2 - alpha + d;
            let normal = turned(Vec3::unit_z(), Vec3::unit_x(), beta);
            let plane = Surface::Plane { origin: Point3::origin() + normal * t, normal, u_ref: Vec3::unit_y() };
            println!("E pn_apex_section {ds:.2} {ts:.2} {}", lab(&plane_cone_section(&plane, &cone, 1.0, b)));
        }
    }
}
