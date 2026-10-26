//! **What the split door refuses at near-tangent poses**: a measurement
//! probe for `an-in-band-certification-of-a-minted-edge-other-than-its-face-residual-reads-as-a-defect`.
//!
//! Three families, every pose printed as one line `SPLIT <tag>: <outcome>`:
//!
//! - `nt`: the near-tangent census's poses ([`NtPrism::near_normal`]) as a
//!   split of the prism by the cube's near face, the plane `d` rad off the
//!   corner's edge `e`.
//! - `edge`: the same planes, tilted so the far vertex of edge `e` reads
//!   within a few roundings of the zero band's edge, `|r| ≈ ε`.
//! - `curved`: a cylinder, a ball and a cone cut by planes `δ ≈ ε` off a
//!   tangency (a ruling, a pole, a rim vertex).
//!
//! A pose that refuses `EdgeOnFacesEscalated` is followed by its lever
//! told as it reads, `SPLIT LEVER <tag> -> d<tilt>` lines: the far vertex
//! moved onto the plane (`d = 0`) and off it by each `NT_OFF` multiple of
//! `Kε` (comma list, default 3), on either side.
//!
//! `NT_ONLY` one prism, `NT_FAMILY` one family.
//!
//! `cargo run -p sweep --release --example split_near_tangent_census_probe`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "../tests/common/differential.rs"]
#[allow(dead_code, unreachable_pub)]
mod differential;

use differential::{NtPrism, nt_prisms};
use geom_core::{Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, split};
use topo::{AtRestBody, Body, mass_properties};

type V3 = [f64; 3];

fn tol() -> Tol {
    Tol::witness()
}

fn eps() -> f64 {
    tol().eps()
}

fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn run(tag: &str, operand: &AtRestBody<f64>, origin: V3, normal: V3) -> bool {
    let n = Vec3::new(normal[0], normal[1], normal[2]);
    let plane = topo::test_support::split_plane(
        Point3::new(origin[0], origin[1], origin[2]),
        n / n.norm(),
        tol(),
    );
    let r = split(operand, &plane, tol());
    let on_faces = matches!(
        r,
        Err(SplitError::Finish(
            topo::splitting::SplitFinishError::EdgeOnFacesEscalated { .. }
        ))
    );
    let line = match r {
        Ok(r) => {
            let vol = |b: Option<&AtRestBody<f64>>| {
                b.map_or(0.0, |b| {
                    mass_properties(b, tol(), topo::test_support::pinned_resolution(tol()))
                        .map_or(f64::NAN, |m| m.enclosure.volume)
                })
            };
            format!(
                "ok above={:.9} below={:.9}",
                vol(r.above.body()),
                vol(r.below.body())
            )
        }
        Err(e) => {
            let kind = match &e {
                SplitError::Reduce(_) => "reduce",
                SplitError::Join(_) => "join",
                SplitError::Finish(_) => "finish",
                SplitError::Pcurves(_) => "pcurves",
                _ => "other",
            };
            format!("REFUSED {kind} | {e:?} | {e}")
        }
    };
    println!("SPLIT {tag}: {line}");
    on_faces
}

/// The lever told as it reads: the far vertex of edge `j` moved onto the
/// plane (`d = 0`), or each `NT_OFF` multiple of `Kε` off it on either
/// side.
fn moved(pr: &NtPrism, body: &AtRestBody<f64>, j: usize, a: f64, tag: &str) {
    let edge = pr.corner_edges()[j];
    let len = dot(edge, edge).sqrt();
    let muls: Vec<f64> = std::env::var("NT_OFF").map_or_else(
        |_| vec![3.0],
        |s| s.split(',').map(|x| x.parse().unwrap()).collect(),
    );
    let offs = muls.iter().map(|m| m * tol().k() * eps() / len);
    for d in core::iter::once(0.0).chain(offs.flat_map(|o| [o, -o])) {
        run(
            &format!("LEVER {tag} -> d{d:e}"),
            body,
            pr.v,
            pr.near_normal(j, a, d),
        );
    }
}

fn nt_family(pr: &NtPrism, body: &AtRestBody<f64>) {
    let ds = [
        1e-5, -1e-5, 1e-6, -1e-6, 1e-7, -1e-7, 1e-8, -1e-8, 1e-9, -1e-9, 1e-10, -1e-10, 1e-12,
        -1e-12,
    ];
    for j in 0..3 {
        for a in 0..16 {
            for &d in &ds {
                let tag = format!("{} nt e{j} a{a} d{d:e}", pr.name);
                if run(&tag, body, pr.v, pr.near_normal(j, f64::from(a), d)) {
                    moved(pr, body, j, f64::from(a), &tag);
                }
            }
        }
    }
}

/// The far vertex of edge `j` read within a few roundings of `|r| = ε`:
/// `d` scanned on a grid fine enough to step over each rounding of the
/// residual at the model's extent.
fn edge_family(pr: &NtPrism, body: &AtRestBody<f64>) {
    let e = eps();
    let ulp = 4.0 * f64::EPSILON;
    for j in 0..3 {
        let edge = pr.corner_edges()[j];
        let far = [0, 1, 2].map(|k| pr.v[k] + edge[k]);
        let len = dot(edge, edge).sqrt();
        for a in [0.0, 3.0, 4.0, 7.0, 9.0, 12.0, 13.0] {
            for s in [1.0, -1.0] {
                for k in -12..=12 {
                    let d = s * (e / len) * (1.0 + f64::from(k) * 0.25 * ulp / e);
                    let m = pr.near_normal(j, a, d);
                    let r = dot([0, 1, 2].map(|i| far[i] - pr.v[i]), m);
                    let tag = format!("{} edge e{j} a{a} d{d:e} r/eps={:.12}", pr.name, r / e);
                    if run(&tag, body, pr.v, m) {
                        moved(pr, body, j, a, &tag);
                    }
                }
            }
        }
    }
}

fn finished(body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).expect("a valid operand")
}

fn circle(r: f64) -> Vec<(Point2<f64>, f64)> {
    vec![(Point2::new(r, 0.0), 1.0), (Point2::new(-r, 0.0), 1.0)]
}

fn curved_family() {
    let e = eps();
    let offsets: Vec<f64> = [0.5, 0.99, 1.0, 1.01, 1.5, 2.0, 5.0, 50.0, 1e3, 1e5]
        .iter()
        .flat_map(|&c| [c * e, -c * e])
        .chain([0.0])
        .collect();
    // A unit cylinder about z, 0 ≤ z ≤ 1, its seam vertices at (±1, 0).
    let cyl = finished(sweep::test_support::prism(circle(1.0), 1.0, tol()));
    for &dd in &offsets {
        for (name, tilt) in [
            ("par", 0.0),
            ("t1e-3", 1e-3),
            ("t1e-6", 1e-6),
            ("t1e-9", 1e-9),
        ] {
            // A plane along the ruling at azimuth φ, `dd` out of it and
            // tilted off the axis.
            for phi in [0.0, 0.5, core::f64::consts::FRAC_PI_2] {
                let (c, s) = (f64::cos(phi), f64::sin(phi));
                let n = [c, s, tilt];
                let o = [c * (1.0 + dd), s * (1.0 + dd), 0.5];
                run(&format!("cyl ruling {name} phi{phi} dd{dd:e}"), &cyl, o, n);
                // Through the seam vertex (1, 0, 0) and `dd` off a
                // plane holding the axis.
                let o2 = [1.0, 0.0, dd];
                run(
                    &format!("cyl rim {name} phi{phi} dd{dd:e}"),
                    &cyl,
                    o2,
                    [tilt, s, c],
                );
            }
        }
        // A cap's plane `dd` off.
        run(
            &format!("cyl cap dd{dd:e}"),
            &cyl,
            [0.0, 0.0, 1.0 + dd],
            [0.0, 0.0, 1.0],
        );
        run(
            &format!("cyl cap tilt dd{dd:e}"),
            &cyl,
            [0.0, 0.0, 1.0],
            [dd, 0.0, 1.0],
        );
    }
    // A unit ball, poles on z.
    let ball = finished(sweep::test_support::ball_poled_z(
        1.0,
        Vec3::new(0.0, 0.0, 0.0),
        tol(),
    ));
    for &dd in &offsets {
        for (name, n) in [
            ("pole", [0.0, 0.0, 1.0]),
            ("side", [1.0, 0.0, 0.0]),
            ("skew", [0.6, 0.0, 0.8]),
        ] {
            let o = n.map(|c| c * (1.0 - dd));
            run(&format!("ball {name} dd{dd:e}"), &ball, o, n);
            let o = n.map(|c| c * (0.5 + dd));
            run(&format!("ball mid {name} dd{dd:e}"), &ball, o, n);
        }
    }
    // A cone frustum: radius 1 at y = 0 to 0.5 at y = 1, revolved about y.
    let cone = finished(sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.5, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    ));
    // The generator from (1, 0, 0) to (0.5, 1, 0): its outward normal
    // in the xy plane is (2, 1)/√5.
    let g = [2.0 / 5f64.sqrt(), 1.0 / 5f64.sqrt(), 0.0];
    for &dd in &offsets {
        let o = [1.0 + g[0] * dd, g[1] * dd, 0.0];
        run(&format!("cone ruling dd{dd:e}"), &cone, o, g);
        run(
            &format!("cone ruling tilt dd{dd:e}"),
            &cone,
            [1.0, 0.0, 0.0],
            [g[0], g[1], dd],
        );
        run(
            &format!("cone cap dd{dd:e}"),
            &cone,
            [0.0, 1.0 + dd, 0.0],
            [0.0, 1.0, 0.0],
        );
    }
}

fn main() {
    let only = std::env::var("NT_ONLY").ok();
    let family = std::env::var("NT_FAMILY").ok();
    let wants = |f: &str| family.as_deref().is_none_or(|x| x == f);
    if wants("nt") || wants("edge") {
        for pr in nt_prisms() {
            if only.as_deref().is_some_and(|o| o != pr.name) {
                continue;
            }
            let body = pr.body(tol());
            if wants("nt") {
                nt_family(&pr, &body);
            }
            if wants("edge") {
                edge_family(&pr, &body);
            }
        }
    }
    if wants("curved") {
        curved_family();
    }
}
