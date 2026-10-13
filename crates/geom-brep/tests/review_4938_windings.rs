//! Reviewer probes for PR 4938's fix pass (periodic-chart closure):
//! the "at most two winding loops" boundary fact on legal and illegal
//! cylinder and torus faces, and an attempt to hand the closed form a
//! loop whose total angle `W` is far from any whole turn.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::{band, eps};
use crate::shared::topo::edge;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face_loops};
use std::f64::consts::TAU;

const RR: f64 = 0.020;
const R0: f64 = 0.005;

fn cyl(radius: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn torus() -> Surface<f64> {
    Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: RR,
        minor_radius: R0,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn crim(radius: f64, z: f64, u0: f64, u1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, z),
            axis: v3(0.0, 0.0, 1.0),
            radius,
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

fn ruling(u: f64, z0: f64, z1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Line {
            origin: p3(u.cos(), u.sin(), 0.0),
            dir: v3(0.0, 0.0, 1.0),
        },
        z0,
        z1,
        0,
        0,
    )
}

fn trim(v: f64, u0: f64, u1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, R0 * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: RR + R0 * v.cos(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

fn tmer(u: f64, v0: f64, v1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(RR * u.cos(), RR * u.sin(), 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: R0,
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        v0,
        v1,
        0,
        0,
    )
}

fn whole(up: bool) -> (f64, f64) {
    if up { (0.0, TAU) } else { (TAU, 0.0) }
}

fn run(s: &Surface<f64>, loops: &[Vec<LoopEdge<f64>>]) -> Result<f64, PropsError> {
    let refs: Vec<&[LoopEdge<f64>]> = loops.iter().map(Vec::as_slice).collect();
    curved_face_loops(s, &refs, true, band()).map(|c| c.area)
}

/// A contractible chart rectangle on the unit cylinder, wound against
/// a face that lies above its lower rims.
fn cyl_hole(u0: f64, u1: f64, z0: f64, z1: f64) -> Vec<LoopEdge<f64>> {
    vec![
        crim(1.0, z0, u1, u0),
        ruling(u0, z0, z1),
        crim(1.0, z1, u0, u1),
        ruling(u1, z1, z0),
    ]
}

fn tor_hole(u0: f64, u1: f64, v0: f64, v1: f64) -> Vec<LoopEdge<f64>> {
    vec![
        trim(v0, u1, u0),
        tmer(u0, v0, v1),
        trim(v1, u0, u1),
        tmer(u1, v1, v0),
    ]
}

#[test]
fn review_4938_winding_loop_count_census() {
    let c = cyl(1.0);
    let (u, d) = (whole(true), whole(false));
    let r = |z: f64, w: (f64, f64)| vec![crim(1.0, z, w.0, w.1)];
    let mut rows: Vec<(&str, Result<f64, PropsError>, bool)> = Vec::new();
    // Legal cylinder faces: must not refuse LoopsWindChart.
    rows.push(("cyl band", run(&c, &[r(0.0, u), r(1.0, d)]), true));
    rows.push((
        "cyl band + 2 holes",
        run(
            &c,
            &[
                r(0.0, u),
                r(1.0, d),
                cyl_hole(1.0, 2.0, 0.3, 0.6),
                cyl_hole(3.0, 4.0, 0.2, 0.5),
            ],
        ),
        true,
    ));
    // Illegal: three and four winding rims.
    rows.push((
        "cyl 3 rims + - +",
        run(&c, &[r(0.0, u), r(1.0, d), r(2.0, u)]),
        false,
    ));
    for (name, s) in [
        ("cyl 4 rims + - + -", [true, false, true, false]),
        ("cyl 4 rims - + - +", [false, true, false, true]),
        ("cyl 4 rims + + - -", [true, true, false, false]),
        ("cyl 4 rims + - - +", [true, false, false, true]),
    ] {
        let loops: Vec<_> = s
            .iter()
            .enumerate()
            .map(|(i, &up)| r(i as f64, whole(up)))
            .collect();
        rows.push((name, run(&c, &loops), false));
    }
    rows.push((
        "cyl 4 rims + - + - with a hole",
        run(
            &c,
            &[
                r(0.0, u),
                r(1.0, d),
                r(2.0, u),
                r(3.0, d),
                cyl_hole(1.0, 2.0, 0.3, 0.6),
            ],
        ),
        false,
    ));
    // Torus: parallels (1,0) and meridians (0,1).
    let t = torus();
    let p = |v: f64, w: (f64, f64)| vec![trim(v, w.0, w.1)];
    let m = |uu: f64, w: (f64, f64)| vec![tmer(uu, w.0, w.1)];
    rows.push(("tor band (1,0)", run(&t, &[p(0.2, u), p(0.7, d)]), true));
    rows.push((
        "tor band (1,0) + 2 holes",
        run(
            &t,
            &[
                p(0.2, u),
                p(0.7, d),
                tor_hole(1.0, 2.0, 0.3, 0.5),
                tor_hole(3.0, 4.0, 0.3, 0.6),
            ],
        ),
        true,
    ));
    rows.push((
        "tor band across v seam",
        run(&t, &[p(-0.3, u), p(0.3, d)]),
        true,
    ));
    rows.push(("tor band (0,1)", run(&t, &[m(0.5, u), m(2.0, d)]), true));
    rows.push((
        "tor 4 parallels + - + -",
        run(&t, &[p(0.1, u), p(0.4, d), p(0.7, u), p(1.0, d)]),
        false,
    ));
    rows.push((
        "tor 4 meridians + - + -",
        run(&t, &[m(0.5, u), m(1.0, d), m(2.0, u), m(3.0, d)]),
        false,
    ));
    rows.push((
        "tor 4 parallels + + - - with a hole",
        run(
            &t,
            &[
                p(0.1, u),
                p(0.4, u),
                p(0.7, d),
                p(1.0, d),
                tor_hole(1.0, 2.0, 0.45, 0.6),
            ],
        ),
        false,
    ));
    let mut bad = 0;
    for (name, got, legal) in &rows {
        let winds = matches!(got, Err(PropsError::LoopsWindChart));
        let ok = if *legal { !winds } else { winds };
        if !ok {
            bad += 1;
        }
        eprintln!(
            "REVIEW4938 {} {name}: {got:?}",
            if ok { "ok " } else { "BAD" }
        );
    }
    assert_eq!(bad, 0);
}

/// Item 4: a contractible zig-zag loop on a thin cylinder (`R = 11ε`)
/// whose rulings are each certified axial (lateral `0.9ε < ε`) but
/// together turn `nΔ` about the axis, which the closed form's sweep
/// counts as zero. Paired with a whole rim, so `Σk = 0` if the zig-zag
/// reads `k = −1`.
#[test]
fn review_4938_w_far_from_a_whole_turn() {
    for (k, n, h) in [
        (11.0, 60usize, 1.0),
        (100.0, 400, 0.001),
        (100.0, 400, 0.01),
    ] {
        zig_case(k, n, h);
    }
}

fn zig_case(k: f64, n: usize, h: f64) {
    let e = eps();
    let radius = k * e;
    let delta = 2.0 * (0.45 * e / radius).asin();
    let at = |u: f64, z: f64| p3(radius * u.cos(), radius * u.sin(), z);
    let mut zig = Vec::new();
    let mut uu = 0.0;
    for i in 0..n {
        let (z0, z1) = if i % 2 == 0 { (0.0, h) } else { (h, 0.0) };
        let a = at(uu, z0);
        let b = at(uu + delta, z1);
        let len = (b - a).norm();
        zig.push(edge(
            Curve3::Line {
                origin: a,
                dir: (b - a) * (1.0 / len),
            },
            0.0,
            len,
            0,
            0,
        ));
        uu += delta;
    }
    zig.push(crim(radius, 0.0, uu, 0.0));
    eprintln!("REVIEW4938 zig: R={radius:e} delta={delta} n={n} W_read=-{uu} (true turn 0)");
    let c = cyl(radius);
    let alone = run(&c, std::slice::from_ref(&zig));
    eprintln!("REVIEW4938 zig alone: {alone:?}");
    for up in [true, false] {
        let w = whole(up);
        let paired = run(&c, &[zig.clone(), vec![crim(radius, 1.0, w.0, w.1)]]);
        eprintln!("REVIEW4938 zig + whole rim up={up}: {paired:?}");
    }
}

fn gcrim<T: geom_core::Decide>(radius: f64, z: f64, u0: f64, u1: f64) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, z),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(radius),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

fn gruling<T: geom_core::Decide>(radius: f64, u: f64, z0: f64, z1: f64) -> LoopEdge<T> {
    edge(
        Curve3::Line {
            origin: p3(radius * u.cos(), radius * u.sin(), 0.0),
            dir: v3(0.0, 0.0, 1.0),
        },
        z0,
        z1,
        0,
        0,
    )
}

fn gtrim<T: geom_core::Decide>(big: f64, r: f64, v: f64, u0: f64, u1: f64) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, r * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(big + r * v.cos()),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

fn gtmer<T: geom_core::Decide>(big: f64, r: f64, u: f64, v0: f64, v1: f64) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(big * u.cos(), big * u.sin(), 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: T::from_f64(r),
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        v0,
        v1,
        0,
        0,
    )
}

fn class<X>(r: &Result<X, PropsError>) -> String {
    match r {
        Ok(_) => "ok".into(),
        Err(PropsError::Escalated { cause, .. }) => {
            format!("esc[{cause:?}]").chars().take(60).collect()
        }
        Err(e) => format!("{e:?}").chars().take(60).collect(),
    }
}

fn diff_case<T: geom_core::Decide>(kind: &str, a: f64, b: f64, gu: f64, gv: f64) -> String {
    let surf: Surface<T>;
    let loops: Vec<Vec<LoopEdge<T>>>;
    if kind == "cyl" {
        let radius = a;
        surf = Surface::Cylinder {
            origin: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(radius),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        loops = vec![vec![
            gcrim(radius, 0.0, TAU - 1.0, TAU),
            gruling(radius, gu / radius, gv, 1.0),
            gcrim(radius, 1.0, gu / radius, gu / radius - 1.0),
            gruling(radius, gu / radius - 1.0, 1.0, 0.0),
        ]];
    } else if kind == "cylband" {
        let radius = a;
        surf = Surface::Cylinder {
            origin: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(radius),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        // A band whose lower rim is split in two arcs, the second
        // starting `gu` round and `gv` up from the first's end.
        loops = vec![
            vec![
                gcrim(radius, 0.0, 0.0, 3.0),
                gcrim(radius, gv, 3.0 + gu / radius, TAU),
            ],
            vec![gcrim(radius, 1.0, TAU, 0.0)],
        ];
    } else {
        let (big, r, v0) = (a, b, if kind == "tor_in" { 2.8_f64 } else { -0.3 });
        let v1 = v0 + 0.6;
        let rho = big + r * v0.cos();
        surf = Surface::Torus {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            major_radius: T::from_f64(big),
            minor_radius: T::from_f64(r),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        let us = TAU + gu / rho;
        loops = vec![vec![
            gtrim(big, r, v0, TAU - 1.0, TAU),
            gtmer(big, r, us, v0 + gv / r, v1),
            gtrim(big, r, v1, us, us - 1.0),
            gtmer(big, r, us - 1.0, v1, v0),
        ]];
    }
    let refs: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
    class(&curved_face_loops(&surf, &refs, true, band()))
}

/// Head-versus-base differential on the closed forms: junction gaps
/// `0–6ε` in each chart direction, both scalars. Prints one line per
/// case for diffing across heads.
#[test]
fn review_4938_closed_form_gap_differential() {
    let e = eps();
    let cases: [(&str, f64, f64); 7] = [
        ("cyl", 1.0, 0.0),
        ("cyl", 0.01, 0.0),
        ("cylband", 1.0, 0.0),
        ("tor_out", RR, R0),
        ("tor_in", RR, R0),
        ("tor_out", 0.01, 0.008),
        ("tor_in", 0.01, 0.008),
    ];
    for (kind, a, b) in cases {
        for g in [0.0, 1.0, 2.0, 3.0, 3.5, 4.0, 4.5, 5.0, 6.0] {
            for (dir, gu, gv) in [("u", g * e, 0.0), ("v", 0.0, g * e)] {
                eprintln!(
                    "REVIEW4938_DIFF eps={e:e} {kind}({a},{b}) {dir} g={g}: f64 {} | iv {}",
                    diff_case::<f64>(kind, a, b, gu, gv),
                    diff_case::<geom_core::Interval>(kind, a, b, gu, gv)
                );
            }
        }
    }
}
