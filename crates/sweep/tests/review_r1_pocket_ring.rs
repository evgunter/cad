//! Review r1 of PR 4008 (`join/pocket-ring-rehoming`): probe rows. See REVIEW.md
//! on `join/pocket-ring-rehoming-review-r1` for what each row showed.
//!
//! **The steep ellipse.** A tilted plane `z = k·x` cuts the cylinder
//! `x² + y² = 1` in an ellipse `P(φ) = (cos φ, sin φ, k cos φ)`. The
//! squared chord from `φ₀ = π/2` to `φ₀ + s` is
//! `2(1 − cos s) + k² sin² s`, which for `k² > 2` peaks at `s = π/2` and
//! FALLS to the half-turn — so within one half-turn (`GermArm::Ahead`)
//! the chord is not monotone in the travel, contrary to `nearer`'s doc.
//!
//! The prism's sketch cap lies in the plane and is a convex quad whose
//! two long edges cut the ellipse at φ = 90°, 260° (edge 2) and
//! φ = 190°, 195° (edge 1), so the section arcs inside the cap are
//! [90°, 190°] and [195°, 260°]. The true partners are 90↔190 and
//! 195↔260. Candidate pairs, all `Ahead`: 90→190 (chord² 11.08),
//! 90→260 (chord² 4.24, WRONG — it spans the gap's two sites) and
//! 195→260 (chord² 6.80). The greedy nearest picks 90→260 first.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;

use crate::common::differential::outcome;

fn tol() -> Tol {
    Tol::witness()
}

fn cylinder(r: f64, z0: f64, h: f64) -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(r, 0.0), 1.0),
        (Point2::new(-r, 0.0), 1.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

/// The ellipse point at azimuth `deg` in the sketch plane's own (u, v).
fn uv(k: f64, deg: f64) -> (f64, f64) {
    let p = deg.to_radians();
    ((1.0 + k * k).sqrt() * p.cos(), p.sin())
}

/// u on the line through a and b at height v.
fn at_v(a: (f64, f64), b: (f64, f64), v: f64) -> f64 {
    a.0 + (b.0 - a.0) * (v - a.1) / (b.1 - a.1)
}

/// The prism: a convex quad in the plane `z = k x` (local u along
/// `(1, 0, k)/√(1+k²)`, v along y), extruded `h` along the plane's
/// normal (`side`). `sites` = (edge-1 φ pair, edge-2 φ pair), degrees.
fn prism(k: f64, sites: [f64; 4], cap: f64, h: f64, side: ExtrudeSide) -> Body<f64> {
    prism_turned(k, sites, cap, h, side, 0.0)
}

fn prism_turned(
    k: f64,
    sites: [f64; 4],
    cap: f64,
    h: f64,
    side: ExtrudeSide,
    psi: f64,
) -> Body<f64> {
    let [e1a, e1b, e2a, e2b] = sites.map(|d| uv(k, d));
    let quad = [
        (at_v(e1a, e1b, -cap), -cap),
        (at_v(e2a, e2b, -cap), -cap),
        (at_v(e2a, e2b, cap), cap),
        (at_v(e1a, e1b, cap), cap),
    ];
    let lp = bulge_loop(
        quad.iter()
            .map(|&(u, v)| (Point2::new(u, v), 0.0))
            .collect(),
    );
    let beta = -(k.atan());
    let rot = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), beta);
    let x = rot.transform_vec(Vec3::new(1.0, 0.0, 0.0));
    assert!(
        (x.z / x.x - k).abs() < 1e-12,
        "local u runs along (1, 0, k)"
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), psi);
    let profile = Profile::new(SketchPlane::new(turn * rot), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(&profile, Extrusion::Distance { depth: h, side }, tol())
        .unwrap()
        .body
}

/// Volume of cylinder (r = 1, z ∈ [zlo, zhi]) ∩ prism by a polar
/// midpoint quadrature over the disc of the vertical extent inside the
/// prism (the prism is convex: an interval per vertical line).
fn oracle_intersection(prism_planes: &[(Vec3<f64>, f64)], zlo: f64, zhi: f64, n: usize) -> f64 {
    let mut v = 0.0;
    let (nr, nt) = (n, 4 * n);
    for i in 0..nr {
        let r = (i as f64 + 0.5) / nr as f64;
        for j in 0..nt {
            let t = (j as f64 + 0.5) / nt as f64 * core::f64::consts::TAU;
            let (x, y) = (r * t.cos(), r * t.sin());
            let (mut lo, mut hi) = (zlo, zhi);
            for &(nrm, d) in prism_planes {
                // nrm · p <= d, p = (x, y, z)
                let c = nrm.x * x + nrm.y * y;
                if nrm.z.abs() < 1e-15 {
                    if c > d {
                        hi = lo - 1.0;
                    }
                } else if nrm.z > 0.0 {
                    hi = hi.min((d - c) / nrm.z);
                } else {
                    lo = lo.max((d - c) / nrm.z);
                }
            }
            if hi > lo {
                v += (hi - lo) * r;
            }
        }
    }
    v * (1.0 / nr as f64) * (core::f64::consts::TAU / nt as f64)
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r1_steep_ellipse_partner_probe() {
    let cyl_z = (-6.0, 12.0);
    let cyl = cylinder(1.0, cyl_z.0, cyl_z.1);
    let vc = topo::mass_properties(&cyl, tol()).unwrap().volume;
    for (k, sites) in [
        (3.0, [190.0, 195.0, 260.0, 90.0]),
        (3.0, [185.0, 200.0, 265.0, 95.0]),
        (2.0, [190.0, 195.0, 260.0, 90.0]),
        // Control: a shallow ellipse (k² < 2), the chord monotone.
        (1.0, [190.0, 195.0, 260.0, 90.0]),
    ] {
        for (sname, side) in [
            ("along", ExtrudeSide::Along),
            ("against", ExtrudeSide::Against),
        ] {
            let h = 1.5;
            let pr = prism(k, sites, 2.0, h, side);
            let vp = topo::mass_properties(&pr, tol()).unwrap().volume;
            // The prism's half-spaces, for the oracle.
            let n = Vec3::new(-k, 0.0, 1.0) * (1.0 / (1.0 + k * k).sqrt());
            let ux = Vec3::new(1.0, 0.0, k) * (1.0 / (1.0 + k * k).sqrt());
            let vy = Vec3::new(0.0, 1.0, 0.0);
            let [e1a, e1b, e2a, e2b] = sites.map(|d| uv(k, d));
            let quad = [
                (at_v(e1a, e1b, -2.0), -2.0),
                (at_v(e2a, e2b, -2.0), -2.0),
                (at_v(e2a, e2b, 2.0), 2.0),
                (at_v(e1a, e1b, 2.0), 2.0),
            ];
            let mut planes = Vec::new();
            let s = if matches!(side, ExtrudeSide::Along) {
                1.0
            } else {
                -1.0
            };
            planes.push((n * -s, 0.0));
            planes.push((n * s, h));
            for i in 0..4 {
                let (a, b) = (quad[i], quad[(i + 1) % 4]);
                // outward normal of a CCW edge: (dv, −du)
                let (du, dv) = (b.0 - a.0, b.1 - a.1);
                let nrm = ux * dv + vy * (-du);
                let d = nrm.x * (ux.x * a.0) + nrm.y * a.1 + nrm.z * (ux.z * a.0);
                planes.push((nrm, d));
            }
            let vi = oracle_intersection(&planes, cyl_z.0, cyl_z.0 + cyl_z.1, 1500);
            for (op, want_ab, want_ba) in [
                ("U", vc + vp - vi, vc + vp - vi),
                ("S", vc - vi, vp - vi),
                ("I", vi, vi),
            ] {
                for (order, want) in [("AB", want_ab), ("BA", want_ba)] {
                    let (l, r) = if order == "AB" {
                        (&cyl, &pr)
                    } else {
                        (&pr, &cyl)
                    };
                    let got = match op {
                        "U" => topo::union(l, r, tol()),
                        "S" => topo::subtract(l, r, tol()),
                        _ => topo::intersect(l, r, tol()),
                    };
                    println!(
                        "k={k} sites={sites:?} {sname} {op} {order} => {}",
                        outcome(got, want, tol())
                    );
                }
            }
        }
    }
}

/// The prism's half-spaces `(n, d)`, `n · p ≤ d`, for the oracle.
fn prism_planes(k: f64, sites: [f64; 4], cap: f64, h: f64, along: bool) -> Vec<(Vec3<f64>, f64)> {
    let s = 1.0 / (1.0 + k * k).sqrt();
    let n = Vec3::new(-k, 0.0, 1.0) * s;
    let ux = Vec3::new(1.0, 0.0, k) * s;
    let vy = Vec3::new(0.0, 1.0, 0.0);
    let [e1a, e1b, e2a, e2b] = sites.map(|d| uv(k, d));
    let quad = [
        (at_v(e1a, e1b, -cap), -cap),
        (at_v(e2a, e2b, -cap), -cap),
        (at_v(e2a, e2b, cap), cap),
        (at_v(e1a, e1b, cap), cap),
    ];
    let sg = if along { 1.0 } else { -1.0 };
    let mut planes = vec![(n * -sg, 0.0), (n * sg, h)];
    for i in 0..4 {
        let (a, b) = (quad[i], quad[(i + 1) % 4]);
        let (du, dv) = (b.0 - a.0, b.1 - a.1);
        let nrm = ux * dv + vy * (-du);
        let d = nrm.x * (ux.x * a.0) + nrm.y * a.1 + nrm.z * (ux.z * a.0);
        planes.push((nrm, d));
    }
    planes
}

/// **Review r1's own battery**: steep and shallow tilted quad prisms
/// against a cylinder, the cap's section an ellipse cut in two arcs,
/// turned about the cylinder's axis so the seam moves. One line per
/// run; main and head are diffed on these lines.
#[test]
#[ignore = "review battery; run with --ignored --nocapture"]
fn r1_steep_ellipse_battery() {
    let cyl_z = (-8.0, 16.0);
    let base = cylinder(1.0, cyl_z.0, cyl_z.1);
    let vc = topo::mass_properties(&base, tol()).unwrap().volume;
    let site_sets: [[f64; 4]; 4] = [
        [190.0, 195.0, 260.0, 90.0],
        [200.0, 230.0, 300.0, 120.0],
        [160.0, 175.0, 250.0, 60.0],
        [225.0, 240.0, 320.0, 110.0],
    ];
    let list = |var: &str, dflt: &[f64]| -> Vec<f64> {
        std::env::var(var).map_or(dflt.to_vec(), |s| {
            s.split(',').map(|x| x.parse().unwrap()).collect()
        })
    };
    for k in list("R1_K", &[0.5, 1.2, 2.0, 3.0, 5.0]) {
        for sites in site_sets {
            for psi in list("R1_PSI", &[0.0, 1.1, 2.5, 4.0]) {
                for (sname, side, along) in [
                    ("along", ExtrudeSide::Along, true),
                    ("against", ExtrudeSide::Against, false),
                ] {
                    let h = 1.5;
                    let turn = Affine3::rotation_about_axis(
                        Point3::origin(),
                        Vec3::new(0.0, 0.0, 1.0),
                        psi,
                    );
                    let pr = prism_turned(k, sites, 2.0, h, side, psi);
                    let vp = topo::mass_properties(&pr, tol()).unwrap().volume;
                    let planes: Vec<_> = prism_planes(k, sites, 2.0, h, along)
                        .into_iter()
                        .map(|(n, d)| (turn.transform_vec(n), d))
                        .collect();
                    let vi = oracle_intersection(&planes, cyl_z.0, cyl_z.0 + cyl_z.1, 1200);
                    for (op, want_ab, want_ba) in [
                        ("U", vc + vp - vi, vc + vp - vi),
                        ("S", vc - vi, vp - vi),
                        ("I", vi, vi),
                    ] {
                        for (order, want) in [("AB", want_ab), ("BA", want_ba)] {
                            let (l, r) = if order == "AB" {
                                (&base, &pr)
                            } else {
                                (&pr, &base)
                            };
                            let got = match op {
                                "U" => topo::union(l, r, tol()),
                                "S" => topo::subtract(l, r, tol()),
                                _ => topo::intersect(l, r, tol()),
                            };
                            let line = match got {
                                Err(e) => {
                                    let s = format!("{e:?}");
                                    format!("ERR {}", s.chars().take(110).collect::<String>())
                                }
                                Ok(res) => match res.body() {
                                    None => format!("EMPTY want={want:.6}"),
                                    Some(bb) => {
                                        let t2 = topo::validate_closed(&bb.body).is_ok();
                                        let t3 = topo::validate_pseudomanifold(
                                            &bb.body,
                                            &bb.contacts,
                                            tol(),
                                        )
                                        .is_ok();
                                        let cert =
                                            topo::validate_geometric_certificate(&bb.body, tol())
                                                .is_ok();
                                        match topo::mass_properties(&bb.body, tol()) {
                                            Ok(m) => {
                                                let good = (m.volume - want).abs() < 2e-5;
                                                format!(
                                                    "OK {} t2={t2} t3p={t3} cert={cert} v={:.9} \
                                                     want~{want:.6}",
                                                    if good && t2 && t3 && cert {
                                                        "SOUND"
                                                    } else {
                                                        "BAD"
                                                    },
                                                    m.volume
                                                )
                                            }
                                            Err(e) => format!("OK-UNMEASURED {e:?}"),
                                        }
                                    }
                                },
                            };
                            println!(
                                "R1BAT k={k} sites={sites:?} psi={psi} {sname} {op} {order} => \
                                 {line}"
                            );
                        }
                    }
                }
            }
        }
    }
}

/// The battery's head-only rows, with the tier-3′ refusal spelled out.
#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r1_steep_ellipse_regressions() {
    let cyl_z = (-8.0, 16.0);
    let base = cylinder(1.0, cyl_z.0, cyl_z.1);
    for (k, sites, psi, side, op, ab) in [
        (
            0.5,
            [190.0, 195.0, 260.0, 90.0],
            2.5,
            ExtrudeSide::Against,
            "S",
            false,
        ),
        (
            1.2,
            [200.0, 230.0, 300.0, 120.0],
            0.0,
            ExtrudeSide::Against,
            "S",
            false,
        ),
        (
            0.5,
            [200.0, 230.0, 300.0, 120.0],
            0.0,
            ExtrudeSide::Against,
            "U",
            true,
        ),
        (
            0.5,
            [200.0, 230.0, 300.0, 120.0],
            0.0,
            ExtrudeSide::Against,
            "U",
            false,
        ),
        (
            6.0,
            [200.0, 230.0, 300.0, 120.0],
            1.75,
            ExtrudeSide::Against,
            "I",
            true,
        ),
        (
            6.0,
            [225.0, 240.0, 320.0, 110.0],
            1.45,
            ExtrudeSide::Against,
            "I",
            true,
        ),
    ] {
        let pr = prism_turned(k, sites, 2.0, 1.5, side, psi);
        let (l, r) = if ab { (&base, &pr) } else { (&pr, &base) };
        let got = match op {
            "U" => topo::union(l, r, tol()),
            "S" => topo::subtract(l, r, tol()),
            _ => topo::intersect(l, r, tol()),
        };
        let tag = format!("k={k} sites={sites:?} psi={psi} {op} ab={ab}");
        match got {
            Err(e) => println!("REG {tag} => ERR {e:?}"),
            Ok(res) => match res.body() {
                None => println!("REG {tag} => EMPTY"),
                Some(bb) => {
                    let lumps = topo::validate_closed(&bb.body).is_ok();
                    let t3: String = format!(
                        "{:?}",
                        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                    )
                    .chars()
                    .take(300)
                    .collect();
                    let cert: String = format!(
                        "{:?}",
                        topo::validate_geometric_certificate(&bb.body, tol())
                    )
                    .chars()
                    .take(300)
                    .collect();
                    println!(
                        "REG {tag} => faces={} t2={lumps} t3p={t3} cert={cert}",
                        bb.body.faces().count(),
                    );
                }
            },
        }
    }
}
