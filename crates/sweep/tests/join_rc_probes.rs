//! The reflex-corner probe's poses: `a` the 315° reflex prism `prism_z`
//! over `(0,0) (2,2) (-2,2) (-2,-2) (2,-2) (2,0)`, z ∈ [0, 1], and `b` a
//! `prism_ops` prism over z ∈ (1, 3) sheared `z' = z + sx·x + sy·y`,
//! whose bottom cap passes through `a`'s reflex corner `(0, 0, 1)`;
//! flush-declared. Twelve profiles put a vertex or an edge of `b` on the
//! corner; `join1_r1_probes::join1_r1_reflex_battery` runs them all.
//! `rc_detail` runs the one pose `RC_CASE="<profile> <sx> <sy> <op>"`
//! names (`cargo test -p sweep --test all rc_detail -- --ignored
//! --nocapture`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::Body;
use topo::test_support::{
    FaceGeometry, describe_as_intersections, flush_declarations, prism_ops, prism_z,
};

fn tol() -> Tol {
    Tol::witness()
}

fn area(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    (0..n)
        .map(|i| {
            let (a, b) = (p[i], p[(i + 1) % n]);
            a.0 * b.1 - a.1 * b.0
        })
        .sum::<f64>()
        / 2.0
}

fn ccw(mut p: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    if area(&p) < 0.0 {
        p.reverse();
    }
    p
}

/// `∫ x dA`, `∫ y dA` over a simple polygon (signed by its winding).
fn moments(p: &[(f64, f64)]) -> (f64, f64) {
    let n = p.len();
    let (mut mx, mut my) = (0.0, 0.0);
    for i in 0..n {
        let (a, b) = (p[i], p[(i + 1) % n]);
        let c = a.0 * b.1 - b.0 * a.1;
        mx += (a.0 + b.0) * c;
        my += (a.1 + b.1) * c;
    }
    (mx / 6.0, my / 6.0)
}

/// Sutherland–Hodgman of `poly` against the convex CCW polygon `clipper`.
fn clip_convex(poly: &[(f64, f64)], clipper: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = poly.to_vec();
    let m = clipper.len();
    for k in 0..m {
        let (a, b) = (clipper[k], clipper[(k + 1) % m]);
        let side = |p: (f64, f64)| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        let inp = std::mem::take(&mut out);
        let n = inp.len();
        for i in 0..n {
            let (s, e) = (inp[i], inp[(i + 1) % n]);
            let (ds, de) = (side(s), side(e));
            let isect = || {
                let t = ds / (ds - de);
                (s.0 + t * (e.0 - s.0), s.1 + t * (e.1 - s.1))
            };
            match (ds >= 0.0, de >= 0.0) {
                (true, true) => out.push(e),
                (true, false) => out.push(isect()),
                (false, true) => {
                    out.push(isect());
                    out.push(e);
                }
                (false, false) => {}
            }
        }
    }
    out
}

const A_PROFILE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (2.0, 2.0),
    (-2.0, 2.0),
    (-2.0, -2.0),
    (2.0, -2.0),
    (2.0, 0.0),
];

/// The probe's twelve `b` profiles, by name.
const PROFILES: [(&str, [(f64, f64); 4]); 12] = [
    ("sqQ1", [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]),
    ("sqQ2", [(-1.0, 0.0), (0.0, 0.0), (0.0, 1.0), (-1.0, 1.0)]),
    ("sqQ3", [(-1.0, -1.0), (0.0, -1.0), (0.0, 0.0), (-1.0, 0.0)]),
    ("sqQ4", [(0.0, -1.0), (1.0, -1.0), (1.0, 0.0), (0.0, 0.0)]),
    ("dUp", [(0.0, 0.0), (0.5, 0.5), (0.0, 1.0), (-0.5, 0.5)]),
    (
        "dLeft",
        [(0.0, 0.0), (-0.5, 0.5), (-1.0, 0.0), (-0.5, -0.5)],
    ),
    (
        "dDown",
        [(0.0, 0.0), (-0.5, -0.5), (0.0, -1.0), (0.5, -0.5)],
    ),
    ("dRight", [(0.0, 0.0), (0.5, -0.5), (1.0, 0.0), (0.5, 0.5)]),
    ("eBot", [(-0.5, 0.0), (0.5, 0.0), (0.5, 1.0), (-0.5, 1.0)]),
    ("eTop", [(-0.5, -1.0), (0.5, -1.0), (0.5, 0.0), (-0.5, 0.0)]),
    ("eLeft", [(0.0, -0.5), (1.0, -0.5), (1.0, 0.5), (0.0, 0.5)]),
    (
        "eRight",
        [(-1.0, -0.5), (0.0, -0.5), (0.0, 0.5), (-1.0, 0.5)],
    ),
];

/// One pose: the operands, their flush declarations, and the closed-form
/// volumes `(∩, ∪, a ∖ b)`.
struct Pose {
    a: Body<f64>,
    b: Body<f64>,
    d: topo::BooleanDeclarations,
    want: [f64; 3],
}

fn pose(profile: &str, sx: f64, sy: f64) -> Pose {
    let a_prof = ccw(A_PROFILE.to_vec());
    let a = prism_z::<f64>(&a_prof, 0.0, 1.0, tol()).body;
    let prof = ccw(PROFILES
        .iter()
        .find(|(n, _)| *n == profile)
        .expect("a probe profile")
        .1
        .to_vec());
    let mut b = Body::<f64>::new();
    prism_ops(
        &mut b,
        &prof,
        (1.0, 3.0),
        |x, y, z| geom_core::Point3::new(x, y, z + sx * x + sy * y),
        FaceGeometry::Certified,
        tol(),
    );
    describe_as_intersections(&mut b, tol());
    // ∫ over a ∩ b ∩ {L < 0} of −L, L = sx·x + sy·y.
    let both = clip_convex(&a_prof, &prof);
    let len = (sx * sx + sy * sy).sqrt();
    let (ux, uy) = (-sy / len, sx / len);
    let (ix, iy) = (-sx / len, -sy / len);
    let big = 100.0;
    let half = ccw(vec![
        (ux * big, uy * big),
        (-ux * big, -uy * big),
        (-ux * big + ix * big, -uy * big + iy * big),
        (ux * big + ix * big, uy * big + iy * big),
    ]);
    let (mx, my) = moments(&clip_convex(&both, &half));
    let vi = -(sx * mx + sy * my);
    let (va, vb) = (area(&a_prof), area(&prof) * 2.0);
    let d = flush_declarations(&a, &b, tol());
    Pose {
        a,
        b,
        d,
        want: [vi, va + vb - vi, va - vi],
    }
}

const OPS: [&str; 3] = ["I", "U", "S_ab"];

fn run(p: &Pose, op: &str) -> Result<topo::BooleanResult<f64>, topo::BooleanError> {
    match op {
        "I" => topo::intersect_with(&p.a, &p.b, &p.d, tol()),
        "U" => topo::union_with(&p.a, &p.b, &p.d, tol()),
        _ => topo::subtract_with(&p.a, &p.b, &p.d, tol()),
    }
}

fn outcome(r: Result<topo::BooleanResult<f64>, topo::BooleanError>, want: f64) -> String {
    match r {
        Err(e) => {
            let s = format!("{e:?}");
            let cut: String = s.chars().take(110).collect();
            format!("ERR {cut}")
        }
        Ok(r) => match r.body() {
            None if want.abs() < 1e-9 => "EMPTY ok".into(),
            None => format!("EMPTY WRONG want={want}"),
            Some(bb) => {
                let t2 = topo::validate_closed(&bb.body).is_ok();
                let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).is_ok();
                let cert = topo::validate_geometric_certificate(&bb.body, tol()).is_ok();
                match topo::mass_properties(&bb.body, tol()).map(|m| m.volume) {
                    Ok(v) => {
                        let good = (v - want).abs() < 1e-7;
                        format!(
                            "OK {} t2={t2} t3p={t3} cert={cert} v={v:.9} want={want:.9}",
                            if good && t2 && t3 && cert {
                                "SOUND"
                            } else {
                                "BAD"
                            }
                        )
                    }
                    Err(e) => format!("OK-UNMEASURED t2={t2} t3p={t3} cert={cert} {e:?}"),
                }
            }
        },
    }
}

#[test]
#[ignore = "detail probe: RC_CASE=\"<profile> <sx> <sy> <op>\""]
fn rc_detail() {
    let case = std::env::var("RC_CASE").expect("RC_CASE");
    let w: Vec<&str> = case.split_whitespace().collect();
    let (sx, sy): (f64, f64) = (w[1].parse().unwrap(), w[2].parse().unwrap());
    let p = pose(w[0], sx, sy);
    let k = OPS.iter().position(|o| *o == w[3]).unwrap();
    println!("{case} => {}", outcome(run(&p, w[3]), p.want[k]));
}

/// **A strut at the reflex corner faces its germs by their true angle
/// from the arrival edge.** Each pose puts both of a strut's germs in
/// `a`'s 315° top face, at least one more than a half-turn from the
/// corner's arrival edge `+x` (measured into the face). Every op builds
/// a body that passes tiers 2 and 3′ and the at-rest certificate, has
/// the closed-form volume, and is a legal operand.
#[test]
fn reflex_corner_struts_past_a_half_turn_build_sound() {
    for (profile, sx, sy) in [
        ("sqQ1", 0.5, -0.25),
        ("sqQ2", 0.25, -0.25),
        ("sqQ2", 0.5, 0.5),
        ("dUp", 0.5, 0.0),
        ("dLeft", 0.5, -0.5),
        ("eBot", 0.25, 0.5),
        ("eLeft", 0.5, -0.25),
        ("eRight", -0.5, -0.5),
    ] {
        let p = pose(profile, sx, sy);
        for (k, op) in OPS.iter().enumerate() {
            let what = format!("{profile} (sx, sy) = ({sx}, {sy}) {op}");
            let bb = match run(&p, op) {
                Ok(topo::BooleanResult::Body(bb)) => bb,
                other => panic!("{what}: {other:?}"),
            };
            assert_eq!(topo::validate_closed(&bb.body), Ok(()), "{what}: tier 2");
            assert_eq!(
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
                Ok(()),
                "{what}: tier 3′"
            );
            assert!(
                topo::validate_geometric_certificate(&bb.body, tol()).is_ok(),
                "{what}: the at-rest certificate"
            );
            let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
            assert!(
                (v - p.want[k]).abs() < 1e-9,
                "{what}: volume {v} against the closed form {}",
                p.want[k]
            );
            sweep::test_support::assert_legal_operand(&what, &bb.body, tol());
        }
    }
}

/// REVIEW (join/reflex-corner-review): the reflex probe widened —
/// every probe profile also rotated about the corner, eleven shears per
/// axis, all four ops — with the legal-operand gate as a column.
/// `RCW_SHARD="k/n"` runs one shard of the profiles.
#[test]
#[ignore = "review battery; run with --ignored --nocapture"]
fn review_rc_wide_battery() {
    let shard = std::env::var("RCW_SHARD").unwrap_or_else(|_| "0/1".into());
    let (k, n): (usize, usize) = {
        let w: Vec<usize> = shard.split('/').map(|x| x.parse().unwrap()).collect();
        (w[0], w[1])
    };
    let a_prof = ccw(A_PROFILE.to_vec());
    let a = prism_z::<f64>(&a_prof, 0.0, 1.0, tol()).body;
    let va = area(&a_prof);
    let shears = [
        -0.75, -0.5, -0.3, -0.25, -0.1, 0.0, 0.1, 0.25, 0.3, 0.5, 0.75,
    ];
    let rots: [f64; 7] = [0.0, 0.003, 7.0, -20.0, 33.0, 100.0, 190.0];
    std::panic::set_hook(Box::new(|_| {}));
    let mut idx = 0;
    for (name, base) in PROFILES.iter() {
        for &rot in &rots {
            idx += 1;
            if idx % n != k {
                continue;
            }
            let (c, s) = (rot.to_radians().cos(), rot.to_radians().sin());
            let prof = ccw(base
                .iter()
                .map(|&(x, y)| (c * x - s * y, s * x + c * y))
                .collect());
            let vb = area(&prof) * 2.0;
            for &sx in &shears {
                for &sy in &shears {
                    if sx == 0.0 && sy == 0.0 {
                        continue;
                    }
                    let mut b = Body::<f64>::new();
                    prism_ops(
                        &mut b,
                        &prof,
                        (1.0, 3.0),
                        |x, y, z| geom_core::Point3::new(x, y, z + sx * x + sy * y),
                        FaceGeometry::Certified,
                        tol(),
                    );
                    describe_as_intersections(&mut b, tol());
                    // The overlap's height over (x, y) in both profiles is
                    // min(1, max(0, −L)), L = sx·x + sy·y (b's cap may now
                    // pass below a's floor): ∫(0 − L)⁺ − ∫(−1 − L)⁺.
                    let both = clip_convex(&a_prof, &prof);
                    let len = (sx * sx + sy * sy).sqrt();
                    let (ux, uy) = (-sy / len, sx / len);
                    let (ix, iy) = (-sx / len, -sy / len);
                    let big = 100.0;
                    let below = |c: f64| {
                        // ∫ over both ∩ {L < c} of (c − L).
                        let (ox, oy) = (sx * c / (len * len), sy * c / (len * len));
                        let half = ccw(vec![
                            (ox + ux * big, oy + uy * big),
                            (ox - ux * big, oy - uy * big),
                            (ox - ux * big + ix * big, oy - uy * big + iy * big),
                            (ox + ux * big + ix * big, oy + uy * big + iy * big),
                        ]);
                        let low = clip_convex(&both, &half);
                        if low.len() < 3 {
                            return 0.0;
                        }
                        let (mx, my) = moments(&low);
                        c * area(&low) - (sx * mx + sy * my)
                    };
                    let vi = below(0.0) - below(-1.0);
                    let d = flush_declarations(&a, &b, tol());
                    for (op, want) in [
                        ("I", vi),
                        ("U", va + vb - vi),
                        ("S_ab", va - vi),
                        ("S_ba", vb - vi),
                    ] {
                        let res = match op {
                            "I" => topo::intersect_with(&a, &b, &d, tol()),
                            "U" => topo::union_with(&a, &b, &d, tol()),
                            "S_ab" => topo::subtract_with(&a, &b, &d, tol()),
                            _ => topo::subtract_with(&b, &a, &d, tol()),
                        };
                        let legal = match &res {
                            Ok(topo::BooleanResult::Body(bb)) => {
                                let body = bb.body.clone();
                                if std::panic::catch_unwind(move || {
                                    sweep::test_support::assert_legal_operand("rcw", &body, tol())
                                })
                                .is_ok()
                                {
                                    " legal"
                                } else {
                                    " NONOPERAND"
                                }
                            }
                            _ => "",
                        };
                        println!(
                            "RCW {name} rot={rot} sx={sx} sy={sy} {op} => {}{legal}",
                            outcome(res, want)
                        );
                    }
                }
            }
        }
    }
}
