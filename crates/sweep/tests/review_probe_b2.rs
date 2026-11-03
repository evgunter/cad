//! REVIEW PROBE (B2): not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region};
use crate::common::wrap_tip::{NORTH, one_wrap_edge};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use std::panic::{AssertUnwindSafe, catch_unwind};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult, SolidContainment, intersect, point_in_solid, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

fn cone(alpha: f64) -> topo::Body<f64> {
    let r = alpha.to_radians().tan();
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn forms(alpha: f64) -> Vec<(&'static str, topo::Body<f64>)> {
    vec![
        ("tipped", one_wrap_edge(cone(alpha), true).0),
        ("revolved", crate::revolve_common::needle(alpha, None, tol()).into_body()),
    ]
}

fn moved(body: &topo::Body<f64>, m: Affine3<f64>) -> AtRestBody<f64> {
    finished(
        "moved",
        topo::transform_rigid(body, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap(),
        tol(),
    )
}

fn the_cone(r: f64) -> Region {
    Box::new(move |p: Point3<f64>| {
        ((r * (1.0 - p.y) - p.x.hypot(p.z)) / (1.0 + r * r).sqrt()).min(p.y).min(1.0 - p.y)
    })
}

fn moved_region(region: Region, m: Affine3<f64>) -> Region {
    let back = m.inverse();
    Box::new(move |p| region(back.transform_point(p)))
}

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Runs the four ops of (a, b), reports each outcome and checks volumes
/// for mutual consistency, against Monte Carlo, and lattice membership.
#[allow(clippy::too_many_arguments)]
fn probe(
    out: &mut Vec<String>,
    what: &str,
    a: &AtRestBody<f64>,
    ra: &Region,
    b: &AtRestBody<f64>,
    rb: &Region,
    bbox: (Point3<f64>, Point3<f64>),
    va: f64,
    vb: f64,
) {
    let (lo, hi) = bbox;
    let span = hi - lo;
    let mut rng = Lcg(12345);
    let n = 300_000;
    let mut both = 0usize;
    for _ in 0..n {
        let p = Point3::new(lo.x + span.x * rng.f(), lo.y + span.y * rng.f(), lo.z + span.z * rng.f());
        if ra(p) > 0.0 && rb(p) > 0.0 {
            both += 1;
        }
    }
    let boxv = span.x * span.y * span.z;
    let mc = boxv * both as f64 / n as f64;
    let mc_err = 4.0 * boxv * ((both.max(1) as f64) / n as f64).sqrt() / (n as f64).sqrt() + 1e-4;
    let band = Band::linear(tol()).unwrap();
    let mut vols = [None; 4];
    let ops: [(&str, u8); 4] = [("a∩b", 0), ("a∪b", 1), ("a∖b", 2), ("b∖a", 3)];
    for (op, k) in ops {
        let res = catch_unwind(AssertUnwindSafe(|| match k {
            0 => intersect(a, b, tol()),
            1 => union(a, b, tol()),
            2 => subtract(a, b, tol()),
            _ => subtract(b, a, tol()),
        }));
        let line = match res {
            Err(_) => "PANIC".to_string(),
            Ok(Err(e)) => format!("ERR {}", format!("{e:?}").chars().take(160).collect::<String>()),
            Ok(Ok(BooleanResult::Empty)) => {
                vols[k as usize] = Some((0.0, 0.0));
                "empty".into()
            }
            Ok(Ok(BooleanResult::Body(body))) => {
                let vol = catch_unwind(AssertUnwindSafe(|| {
                    topo::test_support::pinned_volume(what, &body.body, tol())
                }));
                let mut s = String::new();
                match vol {
                    Ok(mp) => {
                        vols[k as usize] = Some((mp.volume, mp.volume_pad));
                        s += &format!("vol {:.6}", mp.volume);
                    }
                    Err(_) => s += "VOLUME-PANIC",
                }
                let g = topo::validate_geometric(&body.body, tol());
                if g.is_err() {
                    s += " TIER3-FAIL";
                }
                // lattice
                let m = 9usize;
                let mut wrong = 0;
                let mut asked = 0;
                let mut errs = 0;
                for i in 0..m {
                    for j in 0..m {
                        for l in 0..m {
                            let f = |t: usize, o: f64| (t as f64 + o) / (m as f64 - 1.0 + 0.37);
                            let p = Point3::new(
                                lo.x - 0.2 + (span.x + 0.4) * f(i, 0.11),
                                lo.y - 0.2 + (span.y + 0.4) * f(j, 0.23),
                                lo.z - 0.2 + (span.z + 0.4) * f(l, 0.07),
                            );
                            let (x, y) = (ra(p), rb(p));
                            let s_ = match k {
                                0 => x.min(y),
                                1 => x.max(y),
                                2 => x.min(-y),
                                _ => y.min(-x),
                            };
                            if s_.abs() < 2e-3 {
                                continue;
                            }
                            asked += 1;
                            let want = if s_ > 0.0 { SolidContainment::In } else { SolidContainment::Out };
                            match point_in_solid(&body.body, p, band, tol()) {
                                Ok(got) if got == want => {}
                                Ok(_) => wrong += 1,
                                Err(_) => errs += 1,
                            }
                        }
                    }
                }
                if wrong > 0 {
                    s += &format!(" MEMBERSHIP-WRONG {wrong}/{asked}");
                }
                if errs > 0 {
                    s += &format!(" pis-err {errs}");
                }
                s
            }
        };
        out.push(format!("{what} | {op} | {line}"));
    }
    // consistency
    let want = [mc, va + vb - mc, va - mc, vb - mc];
    for (k, (op, _)) in ops.iter().enumerate() {
        if let Some((v, _pad)) = vols[k]
            && (v - want[k]).abs() > mc_err
        {
            out.push(format!("{what} | {op} | VOLUME-MISMATCH got {v:.6} mc-want {:.6} ± {mc_err:.1e}", want[k]));
        }
    }
    if let Some((i, pi)) = vols[0] {
        for (k, w) in [(1usize, va + vb - i), (2, va - i), (3, vb - i)] {
            if let Some((v, pv)) = vols[k]
                && (v - w).abs() > pi + pv + 1e-9
            {
                out.push(format!("{what} | {} | INCL-EXCL got {v:.9} want {w:.9}", ops[k].0));
            }
        }
    }
}

fn cone_vol(alpha: f64) -> f64 {
    core::f64::consts::PI * alpha.to_radians().tan().powi(2) / 3.0
}

fn box_probe(
    out: &mut Vec<String>,
    label: &str,
    bx: ((f64, f64), (f64, f64), (f64, f64)),
    m: Affine3<f64>,
    alphas: &[f64],
) {
    let vb = (bx.0.1 - bx.0.0) * (bx.1.1 - bx.1.0) * (bx.2.1 - bx.2.0);
    let b = moved(&topo::test_support::brick(bx.0, bx.1, bx.2, tol()), m);
    for &alpha in alphas {
        let r = alpha.to_radians().tan();
        for (form, body) in forms(alpha) {
            let c = finished("c", body, tol());
            let rc = the_cone(r);
            let rbx = moved_region(membership::cuboid(bx.0, bx.1, bx.2), m);
            let bbox = (Point3::new(-r, 0.0, -r), Point3::new(r, 1.0, r));
            let what = format!("{label} {alpha}° {form}");
            probe(out, &format!("{what} cone-first"), &c, &rc, &b, &rbx, bbox, cone_vol(alpha), vb);
            probe(out, &format!("{what} box-first"), &b, &rbx, &c, &rc, bbox, vb, cone_vol(alpha));
        }
    }
}

fn rot(axis: (f64, f64, f64), ang: f64) -> Affine3<f64> {
    Affine3::rotation_about_axis(NORTH, Vec3::new(axis.0, axis.1, axis.2).normalize(), ang)
}

fn dump(name: &str, out: &[String]) {
    let path = format!("{}/{name}.txt", std::env::var("PROBE_OUT").unwrap_or("/tmp".into()));
    std::fs::write(&path, out.join("\n")).unwrap();
}

/// F1: a box edge through the apex at generic directions.
#[test]
fn f1_generic_edges_through_the_apex() {
    let mut out = Vec::new();
    // local box with its edge along z at x = y = 0, the apex at the local
    // origin (z interior), moved so the origin is NORTH.
    let bx = ((0.0, 1.5), (0.0, 1.5), (-1.1, 0.9));
    let shift = Affine3::translation(NORTH - Point3::origin());
    let rots: Vec<(&str, Affine3<f64>)> = vec![
        ("r1", rot((1.0, 0.0, 0.0), 0.3)),
        ("r2", rot((1.0, 1.0, 0.0), 1.1)),
        ("r3", rot((0.3, 0.5, 0.8), 2.0)),
        ("r4-axisedge", rot((0.0, 1.0, 0.0), 0.4) * rot((1.0, 0.0, 0.0), -core::f64::consts::FRAC_PI_2)),
        ("r5", rot((0.2, -0.7, 0.4), 2.7)),
        ("r6-steep", rot((1.0, 0.0, 0.0), -1.2) * rot((0.0, 0.0, 1.0), 0.25)),
    ];
    for (name, r) in rots {
        let m = r * shift;
        let d = m.transform_vec(Vec3::new(0.0, 0.0, 1.0));
        box_probe(&mut out, &format!("F1 {name} dy={:.3}", d.y), bx, m, &[30.0, 60.0]);
    }
    dump("f1", &out);
}

/// F2: an edge ending within a few ε of the apex (and past it).
#[test]
fn f2_edges_ending_near_the_apex() {
    let mut out = Vec::new();
    let eps = tol().eps();
    for k in [0.3, 3.0, 30.0, -0.3, -3.0, -30.0, 1000.0] {
        box_probe(
            &mut out,
            &format!("F2 k={k}"),
            ((0.0, 2.0), (0.55, 1.0), (k * eps, 2.3)),
            Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 1.0, 0.0), 0.4),
            &[45.0],
        );
    }
    dump("f2", &out);
}

/// F4: the PR's families in both orders.
#[test]
fn f4_pr_families_both_orders() {
    let mut out = Vec::new();
    for (fam, bx) in [
        ("across", ((0.0, 2.0), (0.55, 1.0), (-2.0, 2.3))),
        ("along", ((0.0, 2.0), (0.55, 1.6), (0.0, 2.1))),
    ] {
        for turn in [0.0, 0.4, 2.0, 1.0] {
            box_probe(
                &mut out,
                &format!("F4 {fam} t{turn}"),
                bx,
                Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 1.0, 0.0), turn),
                &[30.0, 45.0, 60.0],
            );
        }
    }
    dump("f4", &out);
}

/// F3: the apex in a sphere face.
#[test]
fn f3_sphere_faces() {
    let mut out = Vec::new();
    for (label, c) in [("seam", Vec3::new(0.3, 1.2, 0.0)), ("inface", Vec3::new(0.25, 1.3, 0.2))] {
        let rad = (Point3::origin() + c - NORTH).norm();
        let s = finished("s", sweep::test_support::ball_poled_y(rad, c, tol()), tol());
        let cc = Point3::origin() + c;
        let rs: Region = Box::new(move |p| rad - (p - cc).norm());
        for alpha in [30.0f64, 45.0] {
            let r = alpha.to_radians().tan();
            for (form, body) in forms(alpha) {
                let k = finished("c", body, tol());
                let rc = the_cone(r);
                let rs2: Region = Box::new(move |p| rad - (p - cc).norm());
                let bbox = (Point3::new(-r, 0.0, -r), Point3::new(r, 1.0, r));
                let vs = 4.0 / 3.0 * core::f64::consts::PI * rad.powi(3);
                probe(&mut out, &format!("F3 {label} {alpha}° {form} cone-first"), &k, &rc, &s, &rs, bbox, cone_vol(alpha), vs);
                probe(&mut out, &format!("F3 {label} {alpha}° {form} sphere-first"), &s, &rs2, &k, &the_cone(r), bbox, vs, cone_vol(alpha));
            }
        }
    }
    dump("f3", &out);
}
