//! **A vertex read on a grazing plane, against the section it ends**: a
//! measurement probe for
//! `the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane`.
//!
//! The unit cylinder (`0 ≤ z ≤ 1`) turned `φ` about its axis, so a seam
//! vertex sits at `v = (cos φ, sin φ, 0)`, against the plane through
//! `v + δ·n` with normal `n = cos ψ·w + sin ψ·(cos α·ẑ + sin α·t)` (`w`
//! the wall's normal at `v`, `t` the rim's tangent; `VS_ALPHA` lists the
//! `α`, default 0, where the plane touches the rim at `v`): `ψ` is the dihedral between the plane and the wall at `v`, and the
//! section the plane cuts on the wall passes `δ / sin ψ` from `v`.
//! It prints `SPLIT …` (the split door) and `BOOL …` (the cylinder minus a
//! slab whose face is that plane).
//!
//! `cargo run -p sweep --release --example vertex_against_section_probe`
//! (`CAD_TOLERANCE_EPS` picks the row; `VS_DOOR=split|bool` one door).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::splitting::split;
use topo::{AtRestBody, Body, mass_properties};

fn tol() -> Tol {
    Tol::witness()
}

fn finished(body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).expect("a valid operand")
}

fn turned(body: &AtRestBody<f64>, about: Point3<f64>, axis: Vec3<f64>, a: f64) -> AtRestBody<f64> {
    finished(
        topo::transform_rigid(
            body,
            &geom_core::test_support::rigid(&Affine3::rotation_about_axis(about, axis, a), tol()),
            tol(),
        )
        .unwrap(),
    )
}

fn vol(b: Option<&AtRestBody<f64>>) -> f64 {
    b.map_or(0.0, |b| {
        mass_properties(b, tol(), topo::test_support::pinned_resolution(tol()))
            .map_or(f64::NAN, |m| m.enclosure.volume)
    })
}

/// `VS_DOOR=witness`: the plane exactly through the seam vertex of the
/// cylinder turned φ = `VS_PHI` (default 2.2) at dihedral `VS_PSI`, its origin moved each
/// `VS_L` (metres) along the rim's tangent, in the plane; each body a
/// split returns is checked with `validate_geometric`.
fn witness() {
    let list = |k: &str| -> Vec<f64> {
        std::env::var(k)
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let phi: f64 = std::env::var("VS_PHI").map_or(2.2, |p| p.parse().unwrap());
    let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    let cyl = turned(
        &finished(sweep::test_support::prism(circle, 1.0, tol())),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        phi,
    );
    let (c, s) = (phi.cos(), phi.sin());
    let v = Point3::new(c, s, 0.0);
    let along = Vec3::new(-s, c, 0.0);
    for psi in list("VS_PSI") {
        let n = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin());
        for l in list("VS_L") {
            let plane = topo::test_support::split_plane(v + along * l, n, tol());
            let line = match split(&cyl, &plane, tol()) {
                Ok(r) => {
                    let check = |b: Option<&AtRestBody<f64>>| {
                        b.map_or("empty".to_owned(), |b| {
                            match topo::validate_geometric(b, tol()) {
                                Ok(_) => "valid".to_owned(),
                                Err(e) => format!("INVALID {e:?}"),
                            }
                        })
                    };
                    format!(
                        "ok above {} below {}",
                        check(r.above.body()),
                        check(r.below.body())
                    )
                }
                Err(err) => format!("REFUSED {err:?}"),
            };
            println!("WITNESS psi{psi:e} L{l}: {line}");
        }
    }
}

fn main() {
    let e = tol().eps();
    let door = std::env::var("VS_DOOR").unwrap_or_default();
    if door == "witness" {
        witness();
        return;
    }
    let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    let cyl0 = finished(sweep::test_support::prism(circle, 1.0, tol()));
    let z = Vec3::new(0.0, 0.0, 1.0);
    let o = Point3::new(0.0, 0.0, 0.0);
    let psis = [
        ("90", core::f64::consts::FRAC_PI_2),
        ("30", core::f64::consts::FRAC_PI_6),
        ("atan0.1", 0.1f64.atan()),
        ("atan0.05", 0.05f64.atan()),
        ("1e-3", 1e-3),
        ("1e-6", 1e-6),
        ("0", 0.0),
    ];
    let cs: [f64; 9] = [0.0, 0.1, 0.2, 0.5, 0.9, 0.99, 1.01, 2.0, 20.0];
    let alphas: Vec<f64> = std::env::var("VS_ALPHA").map_or(vec![0.0], |a| {
        a.split(',').map(|x| x.parse().unwrap()).collect()
    });
    for (phi, alpha) in [0.9, 2.2]
        .into_iter()
        .flat_map(|p| alphas.iter().map(move |&a| (p, a)))
    {
        let cyl = turned(&cyl0, o, z, phi);
        let (c, s) = (f64::cos(phi), f64::sin(phi));
        for (pname, psi) in psis {
            // `n` in the frame of the wall's normal at the vertex, the
            // rim's tangent and the axis, then turned `φ`.
            let local = Vec3::new(psi.cos(), psi.sin() * alpha.sin(), psi.sin() * alpha.cos());
            let n = Vec3::new(
                c * local.x - s * local.y,
                s * local.x + c * local.y,
                local.z,
            );
            for k in cs.iter().flat_map(|&k| [k, -k]) {
                if k == 0.0 && k.is_sign_negative() {
                    continue;
                }
                let d = k * e;
                let tag = format!("phi{phi}a{alpha} psi{pname} delta/eps={k}");
                if door != "bool" && door != "mirror" {
                    let at = Point3::new(c + d * n.x, s + d * n.y, d * n.z);
                    let plane = topo::test_support::split_plane(at, n, tol());
                    let line = match split(&cyl, &plane, tol()) {
                        Ok(r) => format!(
                            "ok above={:.9} below={:.9}",
                            vol(r.above.body()),
                            vol(r.below.body())
                        ),
                        Err(err) => format!("REFUSED {err:?} | {err}"),
                    };
                    println!("SPLIT {tag}: {line}");
                }
                if door != "split" && door != "mirror" {
                    // The slab x ≥ 1 (its face through (1, 0, 0), outward
                    // −x), its normal turned onto `n`, moved δ along n.
                    let slab = finished(sweep::test_support::prism_at(
                        [(1.0, -3.0), (4.0, -3.0), (4.0, 3.0), (1.0, 3.0)]
                            .iter()
                            .map(|&(x, y)| (Point2::new(x, y), 0.0))
                            .collect(),
                        -3.0,
                        6.0,
                        tol(),
                    ));
                    let x = Vec3::new(1.0, 0.0, 0.0);
                    let axis = x.cross(local);
                    let slab = if axis.norm() == 0.0 {
                        slab
                    } else {
                        turned(
                            &slab,
                            Point3::new(1.0, 0.0, 0.0),
                            axis / axis.norm(),
                            x.dot(local).acos(),
                        )
                    };
                    let slab = turned(&slab, o, z, phi);
                    let slab = finished(
                        topo::transform_rigid(
                            &slab,
                            &geom_core::test_support::rigid(&Affine3::translation(n * d), tol()),
                            tol(),
                        )
                        .unwrap(),
                    );
                    let line = match topo::subtract(&cyl, &slab, tol()) {
                        Ok(r) => format!("ok vol={:.9}", vol(r.body().map(|b| &b.body))),
                        Err(err) => format!("REFUSED {err:?} | {err}"),
                    };
                    println!("BOOL {tag}: {line}");
                }
                if door == "mirror" {
                    // The mirrored pose: a box's corner on the wall at
                    // half height, one of its faces the plane, the
                    // vertex the box's and the wall the other solid's.
                    let mid = Point3::new(1.0, 0.0, 0.5);
                    let corner = finished(sweep::test_support::prism_at(
                        [(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]
                            .iter()
                            .map(|&(x, y)| (Point2::new(x, y), 0.0))
                            .collect(),
                        0.5,
                        1.0,
                        tol(),
                    ));
                    let x = Vec3::new(1.0, 0.0, 0.0);
                    let axis = x.cross(local);
                    let corner = if axis.norm() == 0.0 {
                        corner
                    } else {
                        turned(&corner, mid, axis / axis.norm(), x.dot(local).acos())
                    };
                    let corner = turned(&corner, o, z, phi);
                    let corner = finished(
                        topo::transform_rigid(
                            &corner,
                            &geom_core::test_support::rigid(&Affine3::translation(n * d), tol()),
                            tol(),
                        )
                        .unwrap(),
                    );
                    let line = match topo::subtract(&cyl, &corner, tol()) {
                        Ok(r) => format!("ok vol={:.9}", vol(r.body().map(|b| &b.body))),
                        Err(err) => format!("REFUSED {err:?} | {err}"),
                    };
                    println!("MIRROR {tag}: {line}");
                }
            }
        }
    }
}
