//! Reviewer r1 end-to-end probe for PR #3973: the drum's tilted-cut rim
//! (two `Ellipse` edges) against tori posed by this reviewer (other radii,
//! azimuths, gaps, a far rigid placement), every op in both operand orders.
//! Mounted by appending `#[path = "../../../probes/r1_e2e.rs"] mod r1_e2e;`
//! to `crates/sweep/tests/all.rs`. Prints one line per (pose, op): the
//! refusal variant, or for a built body its volume against the closed form.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Affine3, Band, Point2, Point3, Tol, UnitVec3, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, DATUM_UNIT_NORM};

const R0: f64 = 0.5;
const TILT: f64 = 0.3;

fn drum_lower() -> Body<f64> {
    let tol = Tol::witness();
    let cylinder = sweep::test_support::prism(vec![(Point2::new(-R0, 0.0), 1.0), (Point2::new(R0, 0.0), 1.0)], 1.0, tol);
    let plane = topo::splitting::SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: UnitVec3::new(Vec3::new(TILT.sin(), 0.0, TILT.cos()), DATUM_UNIT_NORM, Band::linear(tol).unwrap()).unwrap(),
    };
    let topo::splitting::SplitPart::Body(below) = topo::splitting::split(&cylinder, &plane, tol).unwrap().below else { panic!() };
    below
}

fn donut(big: f64, small: f64, c: Vec3<f64>, axis: Vec3<f64>) -> Body<f64> {
    let at = revolved_about_y(vec![(Point2::new(big, -small), 1.0), (Point2::new(big, small), 1.0)], Revolution::Full, Tol::witness());
    let y = Vec3::new(0.0, 1.0, 0.0);
    let a = axis.normalize();
    let turned = if y.cross(a).norm() < 1e-15 { at } else {
        topo::transform_rigid(&at, &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), y.cross(a).normalize(), y.dot(a).acos()), Tol::witness()).unwrap()
    };
    topo::transform_rigid(&turned, &Affine3::translation(c), Tol::witness()).unwrap()
}

/// Tube within `gap` of the rim at azimuth `phi`, outside the corner, the
/// core circle's plane turned by `roll` about the outward bisector.
fn corner_torus(big: f64, small: f64, phi: f64, gap: f64, roll: f64) -> Body<f64> {
    let (st, ct) = TILT.sin_cos();
    let (sp, cp) = phi.sin_cos();
    let x = R0 * cp;
    let rim = Vec3::new(x, R0 * sp, 0.5 - x * TILT.tan());
    let wall = Vec3::new(cp, sp, 0.0);
    let cut = Vec3::new(st, 0.0, ct);
    let d = (wall + cut).normalize();
    let t = cut.cross(wall).normalize();
    let e = d.cross(t).normalize();
    let e = if e.dot(wall) < 0.0 { -e } else { e };
    // Roll the hub direction about d (keeps the near point on d).
    let e = e * roll.cos() + d.cross(e) * roll.sin();
    donut(big, small, rim + d * (small + gap) + e * big, d)
}

fn kind<E: core::fmt::Debug>(e: &E) -> String { let s = format!("{e:?}"); if s.starts_with("CurvedPierce") { return s; } s.split([' ', '{', '(']).next().unwrap().to_string() }

#[test]
#[ignore = "reviewer e2e probe"]
fn r1_e2e() {
    let tol = Tol::witness();
    let drum = drum_lower();
    let vdrum = core::f64::consts::PI * R0 * R0 * 0.5;
    let far = Affine3::translation(Vec3::new(700.0, -400.0, 300.0));
    let mut built = 0;
    for &(big, small) in &[(0.3, 0.05), (0.12, 0.03), (0.8, 0.2)] {
        for &phi in &[0.2, 2.5, 4.0] {
            for &gap in &[1e-3, 1e-5, 3e-8, -1e-3, -0.02] {
                for &roll in &[0.0, 0.7] {
                    for placed in [false, true] {
                        let mut a = drum.clone();
                        let mut b = corner_torus(big, small, phi, gap, roll);
                        if placed {
                            a = topo::transform_rigid(&a, &far, tol).unwrap();
                            b = topo::transform_rigid(&b, &far, tol).unwrap();
                        }
                        let vt = 2.0 * core::f64::consts::PI.powi(2) * big * small * small;
                        let ops: [(&str, &Body<f64>, &Body<f64>, fn(&Body<f64>, &Body<f64>, Tol) -> Result<topo::BooleanResult<f64>, topo::BooleanError>, f64); 8] = [
                            ("A∪B", &a, &b, topo::boolean::union, vdrum + vt),
                            ("B∪A", &b, &a, topo::boolean::union, vdrum + vt),
                            ("A∩B", &a, &b, topo::boolean::intersect, 0.0),
                            ("B∩A", &b, &a, topo::boolean::intersect, 0.0),
                            ("A∖B", &a, &b, topo::boolean::subtract, vdrum),
                            ("B∖A", &b, &a, topo::boolean::subtract, vt),
                            ("A∖B'", &a, &b, topo::boolean::subtract, vdrum),
                            ("B∖A'", &b, &a, topo::boolean::subtract, vt),
                        ];
                        for (op, x, y, f, want) in ops {
                            let line = match f(x, y, tol) {
                                Err(e) => format!("refuse {}", kind(&e)),
                                Ok(topo::BooleanResult::Body(r)) => {
                                    built += 1;
                                    let v = topo::mass_properties(&r.body, tol).map(|p| p.volume);
                                    format!("BUILT volume {v:?} closed-form-if-disjoint {want}")
                                }
                                Ok(other) => format!("BUILT-other {}", kind(&other)),
                            };
                            println!("R1E2E R={big} r={small} phi={phi} gap={gap:e} roll={roll} far={placed} {op}: {line}");
                        }
                    }
                }
            }
        }
    }
    println!("R1E2E built {built}");
}
