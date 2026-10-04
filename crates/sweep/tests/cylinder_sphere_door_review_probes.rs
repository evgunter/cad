//! Review battery for PR #4025, shaped to run on main and on head: a
//! grid of drum × ball poses (radius-0.4 drum `z ∈ [−0.6, 0.6]`, balls
//! clear of the caps, across the rim, and through a cap), every op in
//! both orders, one line per run naming the door or the built body's
//! volume and validity. Prints only; diff the two trees' lines.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair::cyl;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

fn ball(r: f64, c: [f64; 3], spin: f64) -> Body<f64> {
    let at_origin = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    let to = Affine3::translation(Vec3::new(c[0], c[1], c[2]))
        * Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(2.0, -1.0, 0.5).normalize(), spin);
    topo::transform_rigid(&at_origin, &to, Tol::witness()).unwrap()
}

#[test]
#[ignore = "review door battery"]
fn review_cs_door_battery() {
    let tol = Tol::witness();
    let a = cyl(0.4, 0.6);
    let mut n = 0;
    for &big in &[0.12, 0.25, 0.39, 0.4, 0.41, 0.55] {
        for &d in &[0.0, 0.1, 0.2, 0.3, 0.38, 0.4, 0.42, 0.5] {
            for &(z, spin) in &[(0.0, 0.0), (0.1, 0.7), (0.5, 0.0), (0.6, 1.9)] {
                let phi = 0.37 * f64::from(n % 7);
                n += 1;
                let c = [d * phi.cos(), d * phi.sin(), z];
                let b = ball(big, c, spin);
                for (ol, op, sw) in [
                    ("A∪B", BooleanOp::Union, false),
                    ("B∪A", BooleanOp::Union, true),
                    ("A∩B", BooleanOp::Intersect, false),
                    ("B∩A", BooleanOp::Intersect, true),
                    ("A∖B", BooleanOp::Subtract, false),
                    ("B∖A", BooleanOp::Subtract, true),
                ] {
                    let (x, y) = if sw { (&b, &a) } else { (&a, &b) };
                    let got = match op {
                        BooleanOp::Union => topo::union(x, y, tol),
                        BooleanOp::Intersect => topo::intersect(x, y, tol),
                        BooleanOp::Subtract => topo::subtract(x, y, tol),
                    };
                    let line = match got {
                        Ok(res) => format!("BUILT {}", format!("{res:?}").chars().take_while(|c| c.is_alphanumeric()).collect::<String>()),
                        Err(e) => {
                            let s = format!("{e:?}");
                            s.chars().take_while(|c| *c != ' ' && *c != '{' && *c != '(').collect::<String>()
                                + &match &e {
                                    topo::BooleanError::CurvedBooleanUnsupported { kind, .. } => format!("({kind:?})"),
                                    topo::BooleanError::GermFrameUnsupported { a_kind, b_kind, .. } => format!("({a_kind:?},{b_kind:?})"),
                                    _ => String::new(),
                                }
                        }
                    };
                    println!("CSDOOR R={big} d={d} z={z} spin={spin} {ol}: {line}");
                }
            }
        }
    }
}
