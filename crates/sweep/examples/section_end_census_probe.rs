//! **Where a section chord's end reads in band of its vertex, and whose
//! reading that is**: a measurement probe for
//! `a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join`.
//!
//! Splits of a unit cylinder, ball and cone frustum, one line per run,
//! `SPLIT <family> <tag>: <outcome>`, the outcome bucketed as `ok`,
//! `END <reading> <margin>` (the join's typed reading of the input),
//! `EULER <check> <arm> <margin>` (the join's endpoint pin left the
//! gate's), or `REFUSED <stage> <variant>`. `NT_FAMILY` picks one family.
//!
//! - `axial`: planes nearly along the cylinder's axis, tilted `θ` off it,
//!   cutting the split's own elongated ellipses (semi-major `r/θ`), at
//!   offsets from the axis well clear of every vertex. An in-band end here
//!   carries the ellipse's own rounding, the kernel's.
//! - `rim`: planes through the seam vertex `(1, 0, 0)`, `δ` along the
//!   seam, tilted `t` off square: the vertex at the zero band's edge at
//!   `|δ| ≈ ε`, the plane read square at `t·r ≤ ε`.
//! - `ball`, `cone`: the same two readings on a sphere's circle and a
//!   cone's axis-normal circle.
//! - `bool`: the Boolean's join, each op in both orders, `BOOL` lines: a
//!   slab's top face tilted `t` about `y`, the seam ruling `dd` inside it
//!   (`walltilt`); a box whose one face across the cylinder, its top,
//!   is tilted `t` about `y` through the axis (`squarebox`); and a box
//!   whose top face passes `δ` from the cylinder's seam vertex, tilted
//!   `a` (`seamvertex`).
//!
//! `cargo run -p sweep --release --example section_end_census_probe`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, split};
use topo::{AtRestBody, Body, BooleanError, EulerOpError, SplitJoinError};

fn tol() -> Tol {
    Tol::witness()
}

fn unresolved(miss: topo::SectionPinMiss) -> String {
    match miss {
        topo::SectionPinMiss::Undecided {
            check,
            diag,
            resolution,
        } => format!("UNRESOLVED {check:?} {:?} {resolution:?}", diag.margin),
        topo::SectionPinMiss::Past { check, margin, .. } => {
            format!("UNRESOLVED-PAST {check:?} {margin:?}")
        }
    }
}

fn outcome(r: Result<topo::SplitResult<f64>, SplitError>) -> String {
    match r {
        Ok(_) => "ok".to_owned(),
        Err(SplitError::Join(SplitJoinError::SectionEndEscalated { reading, diag, .. })) => {
            format!("END {reading:?} {:?}", diag.margin)
        }
        Err(SplitError::Join(SplitJoinError::SectionUnresolved { miss, .. })) => unresolved(miss),
        Err(SplitError::Join(SplitJoinError::Euler(EulerOpError::Certification { error }))) => {
            match error {
                geom_brep::CertifyError::Escalated { check, cause, .. } => {
                    format!("EULER {check:?} escalated {:?}", cause.margin)
                }
                geom_brep::CertifyError::ResidualExceeded { check, margin, .. } => {
                    format!("EULER {check:?} exceeded {margin:?}")
                }
                other => format!("EULER other {other:?}"),
            }
        }
        Err(e) => {
            let stage = match &e {
                SplitError::Reduce(_) => "reduce",
                SplitError::Join(_) => "join",
                SplitError::Finish(_) => "finish",
                SplitError::Pcurves(_) => "pcurves",
                SplitError::Pieces(_) => "pieces",
            };
            let variant = format!("{e:?}");
            let head: String = variant.chars().take(90).collect();
            format!("REFUSED {stage} {head}")
        }
    }
}

fn run(family: &str, tag: &str, body: &AtRestBody<f64>, origin: [f64; 3], normal: [f64; 3]) {
    let n = Vec3::new(normal[0], normal[1], normal[2]);
    let plane = topo::test_support::split_plane(
        Point3::new(origin[0], origin[1], origin[2]),
        n / n.norm(),
        tol(),
    );
    println!(
        "SPLIT {family} {tag}: {}",
        outcome(split(body, &plane, tol()))
    );
}

type Op =
    fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<topo::BooleanResult<f64>, BooleanError>;

fn boolean(tag: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) {
    for (order, x, y) in [("ab", a, b), ("ba", b, a)] {
        let ops: [(&str, Op); 3] = [
            ("U", topo::union),
            ("I", topo::intersect),
            ("S", topo::subtract),
        ];
        for (op, f) in ops {
            let line = match f(x, y, tol()) {
                Ok(_) => "ok".to_owned(),
                Err(BooleanError::Escalated { decision, diag }) => {
                    format!(
                        "ESCALATED {decision:?} {:?} {:?}",
                        diag.predicate, diag.margin
                    )
                }
                Err(BooleanError::Join(e)) => match e.error() {
                    SplitJoinError::SectionEndEscalated { reading, diag, .. } => {
                        format!("END {reading:?} {:?}", diag.margin)
                    }
                    SplitJoinError::SectionUnresolved { miss, .. } => unresolved(*miss),
                    other => {
                        let text = format!("{other:?}");
                        format!("REFUSED join {}", text.chars().take(90).collect::<String>())
                    }
                },
                Err(e) => format!("REFUSED {e:?} | {e}"),
            };
            println!("BOOL {tag} {order} {op}: {line}");
        }
    }
}

fn rotated(body: &AtRestBody<f64>, about: [f64; 3], angle: f64) -> AtRestBody<f64> {
    finished(
        topo::transform_rigid(
            body,
            &geom_core::test_support::rigid(
                &Affine3::rotation_about_axis(
                    Point3::new(about[0], about[1], about[2]),
                    Vec3::new(0.0, 1.0, 0.0),
                    angle,
                ),
                tol(),
            ),
            tol(),
        )
        .unwrap(),
    )
}

fn finished(body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).expect("a valid operand")
}

fn offsets(e: f64) -> Vec<f64> {
    [0.5, 0.99, 1.0, 1.01, 2.0, 50.0, 1e3, 1e5]
        .iter()
        .flat_map(|&c| [c * e, -c * e])
        .chain([0.0])
        .collect()
}

fn main() {
    let e = tol().eps();
    let family = std::env::var("NT_FAMILY").ok();
    let wants = |f: &str| family.as_deref().is_none_or(|x| x == f);
    let cyl = finished(sweep::test_support::prism(
        vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)],
        1.0,
        tol(),
    ));
    if wants("axial") {
        let tilts = [
            1e-1, 3e-2, 1e-2, 3e-3, 1e-3, 3e-4, 1e-4, 3e-5, 1e-5, 3e-6, 1e-6,
        ];
        for w in [0.0, 0.3, 0.7, 0.95] {
            for k in 0..8 {
                let phi = 0.4 + f64::from(k) * 0.7;
                let (c, s) = (phi.cos(), phi.sin());
                for &th in &tilts {
                    for z0 in [0.5, 0.3] {
                        run(
                            "axial",
                            &format!("w{w} phi{phi:.1} th{th:e} z{z0}"),
                            &cyl,
                            [c * w, s * w, z0],
                            [c, s, th],
                        );
                    }
                }
            }
        }
    }
    if wants("m1") {
        // Planes nearly along the axis through a seam vertex, the vertex
        // `c·ε` off along the normal: the input's offset and the elongated
        // ellipse's own rounding in one pose.
        for th in [3e-5, 1e-4, 3e-4] {
            for (vx, vz) in [(1.0, 0.0), (-1.0, 1.0)] {
                for c in [
                    0.0, 0.5, -0.5, 0.8, -0.8, 0.9, -0.9, 0.99, -0.99, 50.0, -50.0,
                ] {
                    let phi = core::f64::consts::FRAC_PI_2;
                    let n = [phi.cos(), phi.sin(), th];
                    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                    let o = [
                        vx + n[0] / len * c * e,
                        n[1] / len * c * e,
                        vz + n[2] / len * c * e,
                    ];
                    run("m1", &format!("v({vx},{vz}) th{th:e} c{c}"), &cyl, o, n);
                }
            }
        }
    }
    if wants("rim") {
        let tilts = [0.0, 0.5 * e, e, 2.0 * e, 1e-6, 1e-4, 1e-3, 1e-2, 1e-1];
        for &t in &tilts {
            for psi in [0.0, 1.0, 2.5] {
                for &d in &offsets(e) {
                    run(
                        "rim",
                        &format!("t{t:e} psi{psi} d{d:e}"),
                        &cyl,
                        [1.0, 0.0, d],
                        [t * f64::cos(psi), t * f64::sin(psi), 1.0],
                    );
                }
            }
        }
    }
    if wants("bool") {
        let rect = |x0: f64, y0: f64, x1: f64, y1: f64| {
            [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
                .map(|(x, y)| (Point2::new(x, y), 0.0))
                .to_vec()
        };
        for dd in [-50.0 * e, -1e3 * e] {
            for t in [0.0, e, 1e-2, -1e-2] {
                let slab = finished(sweep::test_support::prism_at(
                    rect(1.0 + dd, -2.0, 3.0, 2.0),
                    0.25,
                    0.5,
                    tol(),
                ));
                boolean(
                    &format!("walltilt t{t:e} dd{dd:e}"),
                    &cyl,
                    &rotated(&slab, [1.0 + dd, 0.0, 0.5], t),
                );
            }
        }
        for t in [0.0, e, 1e-2, -1e-2] {
            let block = finished(sweep::test_support::prism_at(
                rect(-3.0, -3.0, 3.0, 3.0),
                -1.0,
                1.5,
                tol(),
            ));
            boolean(
                &format!("squarebox t{t:e}"),
                &cyl,
                &rotated(&block, [0.0, 0.0, 0.5], t),
            );
        }
        for a in [1e-3, 1e-2] {
            for &d in &offsets(e) {
                let block = finished(sweep::test_support::prism_at(
                    rect(-3.0, -3.0, 3.0, 3.0),
                    d - 2.0,
                    2.0,
                    tol(),
                ));
                boolean(
                    &format!("seamvertex a{a} d{d:e}"),
                    &cyl,
                    &rotated(&block, [1.0, 0.0, d], a),
                );
            }
        }
    }
    if wants("torus") {
        // The ring torus `R = 2`, `r = 0.5` about `y`, against a block
        // whose one face is read square to the axis (`y = h`, turned `t`
        // about `z`) or through it (`z = d`, turned `t` about `x`): the
        // concentric and the meridian circles the table stands in.
        let torus = finished(sweep::test_support::revolved_about_y(
            vec![(Point2::new(2.0, -0.5), 1.0), (Point2::new(2.0, 0.5), 1.0)],
            sweep::Revolution::Full,
            tol(),
        ));
        let rect = |x0: f64, y0: f64, x1: f64, y1: f64| {
            [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
                .map(|(x, y)| (Point2::new(x, y), 0.0))
                .to_vec()
        };
        let turned = |body: Body<f64>, about: Point3<f64>, axis: Vec3<f64>, t: f64| {
            finished(
                topo::transform_rigid(
                    &body,
                    &geom_core::test_support::rigid(
                        &Affine3::rotation_about_axis(about, axis, t),
                        tol(),
                    ),
                    tol(),
                )
                .unwrap(),
            )
        };
        let lever = e / 2.5;
        for t in [0.0, 0.5 * lever, lever, 2.0 * lever, 1e-6, 1e-2] {
            for h in [0.0, 0.2, 0.2 + e] {
                let block =
                    sweep::test_support::prism_at(rect(-4.0, -4.0, 4.0, h), -4.0, 8.0, tol());
                boolean(
                    &format!("torus-square t{t:e} h{h:e}"),
                    &torus,
                    &turned(block, Point3::new(0.0, h, 0.0), Vec3::new(0.0, 0.0, 1.0), t),
                );
            }
            for d in [0.0, e, 2.0 * e] {
                let block =
                    sweep::test_support::prism_at(rect(-4.0, -4.0, 4.0, 4.0), d - 4.0, 4.0, tol());
                boolean(
                    &format!("torus-through t{t:e} d{d:e}"),
                    &torus,
                    &turned(block, Point3::new(0.0, 0.0, d), Vec3::new(1.0, 0.0, 0.0), t),
                );
            }
        }
    }
    if wants("ball") {
        let ball = finished(sweep::test_support::ball_poled_z(
            1.0,
            Vec3::new(0.0, 0.0, 0.0),
            tol(),
        ));
        for a in [0.3, 1.0, 1.4] {
            for &d in &offsets(e) {
                // Through the pole `(0, 0, 1)`, `d` off it along `z`.
                let n = [f64::sin(a), 0.0, f64::cos(a)];
                run(
                    "ball",
                    &format!("a{a} d{d:e}"),
                    &ball,
                    [0.0, 0.0, 1.0 + d],
                    n,
                );
            }
        }
    }
    if wants("cone") {
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
        for &t in &[0.0, 0.5 * e, e, 2.0 * e, 1e-6, 1e-3, 1e-1] {
            for &d in &offsets(e) {
                // Through the rim vertex `(1, 0, 0)`, `d` up the axis `y`,
                // tilted `t` off square.
                run(
                    "cone",
                    &format!("t{t:e} d{d:e}"),
                    &cone,
                    [1.0, d, 0.0],
                    [t, 1.0, 0.0],
                );
            }
        }
    }
}
