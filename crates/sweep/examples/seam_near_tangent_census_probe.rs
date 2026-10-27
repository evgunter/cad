//! **What the Boolean refuses at curved seam and tangent poses
//! `δ ≈ ε` off tangency**: a measurement probe for
//! `an-in-band-certification-of-a-minted-edge-other-than-its-face-residual-reads-as-a-defect`.
//!
//! Each pose runs union, intersection and subtraction in both orders and
//! prints `BOOL <tag> <order> <op>: <outcome>`. `NT_POSE` filters by a
//! substring of the tag.
//!
//! `cargo run -p sweep --release --example seam_near_tangent_census_probe`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::{AtRestBody, Body, BooleanError, BooleanResult, mass_properties};

fn tol() -> Tol {
    Tol::witness()
}

fn finished(body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).expect("a valid operand")
}

fn circle(cx: f64, cy: f64, r: f64) -> Vec<(Point2<f64>, f64)> {
    vec![
        (Point2::new(cx + r, cy), 1.0),
        (Point2::new(cx - r, cy), 1.0),
    ]
}

fn poly(p: &[(f64, f64)]) -> Vec<(Point2<f64>, f64)> {
    p.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect()
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<(Point2<f64>, f64)> {
    poly(&[(x0, y0), (x1, y0), (x1, y1), (x0, y1)])
}

fn prism_at(verts: Vec<(Point2<f64>, f64)>, z0: f64, h: f64) -> Body<f64> {
    sweep::test_support::prism_at(verts, z0, h, tol())
}

type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, BooleanError>;

fn run(tag: &str, a: Body<f64>, b: Body<f64>) {
    if std::env::var("NT_POSE")
        .ok()
        .is_some_and(|p| !tag.contains(&p))
    {
        return;
    }
    let (a, b) = (finished(a), finished(b));
    for (order, x, y) in [("ab", &a, &b), ("ba", &b, &a)] {
        let ops: [(&str, Op); 3] = [
            ("U", topo::union),
            ("I", topo::intersect),
            ("S", topo::subtract),
        ];
        for (op, f) in ops {
            let line = match f(x, y, tol()) {
                Ok(r) => match r.body() {
                    Some(bb) => format!(
                        "ok vol={:.9}",
                        mass_properties(
                            &bb.body,
                            tol(),
                            topo::test_support::pinned_resolution(tol())
                        )
                        .map_or(f64::NAN, |m| m.enclosure.volume)
                    ),
                    None => "ok empty".to_owned(),
                },
                Err(e) => format!("REFUSED {:?} | {e:?} | {e}", e.kind()),
            };
            println!("BOOL {tag} {order} {op}: {line}");
        }
    }
}

fn main() {
    let e = tol().eps();
    let offsets: Vec<f64> = [0.5, 0.99, 1.0, 1.01, 1.5, 2.0, 5.0, 50.0, 1e3]
        .iter()
        .flat_map(|&c| [c * e, -c * e])
        .chain([0.0])
        .collect();
    for &dd in &offsets {
        // A slot: a box whose two sides the cylinder's wall meets
        // smoothly along the rulings at (±1, 0), `dd` off.
        run(
            &format!("slot dd{dd:e}"),
            prism_at(rect(-1.0, -2.0, 1.0, 0.0), 0.0, 1.0),
            prism_at(circle(0.0, 0.0, 1.0 + dd), -0.5, 2.0),
        );
        run(
            &format!("slotflush dd{dd:e}"),
            prism_at(rect(-1.0, -2.0, 1.0, 0.0), 0.0, 1.0),
            prism_at(circle(0.0, 0.0, 1.0 + dd), 0.0, 1.0),
        );
        // A box's face `dd` off the cylinder's ruling at (1, 0).
        run(
            &format!("wall dd{dd:e}"),
            prism_at(circle(0.0, 0.0, 1.0), 0.0, 1.0),
            prism_at(rect(1.0 + dd, -2.0, 3.0, 2.0), 0.25, 0.5),
        );
        // Two cylinders, external and internal tangency.
        run(
            &format!("cylext dd{dd:e}"),
            prism_at(circle(0.0, 0.0, 1.0), 0.0, 1.0),
            prism_at(circle(1.5 + dd, 0.0, 0.5), 0.25, 0.5),
        );
        run(
            &format!("cylint dd{dd:e}"),
            prism_at(circle(0.0, 0.0, 1.0), 0.0, 1.0),
            prism_at(circle(0.5 + dd, 0.0, 0.5), 0.25, 0.5),
        );
        // A ball on a box's top face.
        run(
            &format!("ball dd{dd:e}"),
            sweep::test_support::ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
            prism_at(rect(-2.0, -2.0, 2.0, 2.0), -3.0, 2.0 - dd),
        );
        // A box's face through the ruling at (1, 0), tilted `t` off it
        // about the ruling's normal, `dd` out.
        for t in [1e-3, 1e-6] {
            let slab = prism_at(rect(1.0 + dd, -2.0, 3.0, 2.0), 0.25, 0.5);
            let tilted = topo::transform_rigid(
                &slab,
                &geom_core::test_support::rigid(
                    &Affine3::rotation_about_axis(
                        Point3::new(1.0 + dd, 0.0, 0.5),
                        Vec3::new(0.0, 1.0, 0.0),
                        t,
                    ),
                    tol(),
                ),
                tol(),
            )
            .unwrap();
            run(
                &format!("walltilt t{t:e} dd{dd:e}"),
                prism_at(circle(0.0, 0.0, 1.0), 0.0, 1.0),
                tilted,
            );
        }
    }
}
