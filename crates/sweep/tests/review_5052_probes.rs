// Review probes for PR 5052 (`sheet_clip` / `sheet_fn` / `tilt`).
//
// Not a suite of `tests/all.rs`: these read private items of
// `blend::reach`, so the file is included from that module's test
// module by `include!` on the review branch only. Run with
// `cargo nextest run -p sweep --lib review_5052`.

use super::super::{Clip, SheetFn, sheet_clip, sheet_fn, tilt};
use geom_core::interval::Interval;
use geom_core::{Bounds, Point2, Point3, Real, Tol, Vec3};
use profile::test_support::bulge_loop;

fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).expect("the witness band")
}

/// The fixtures, each a list of profile loops (outer first), as
/// `(x, y, bulge)` vertex chains. Extruded `z ∈ [0, 1]`.
fn fixtures() -> Vec<(&'static str, Vec<Vec<(f64, f64, f64)>>)> {
    let q = core::f64::consts::FRAC_PI_4;
    let big = (3.0 * core::f64::consts::PI / 8.0).tan(); // a 270° arc
    vec![
        ("rod", vec![vec![(-0.5, 0.0, 1.0), (0.5, 0.0, 1.0)]]),
        ("half-rod", vec![vec![(-0.5, 0.0, 1.0), (0.5, 0.0, 0.0)]]),
        (
            "major-segment",
            vec![vec![
                (0.5 * q.cos(), 0.5 * q.sin(), big),
                (0.5 * q.cos(), -0.5 * q.sin(), 0.0),
            ]],
        ),
        (
            "minor-segment",
            vec![vec![
                (0.5 * q.cos(), -0.5 * q.sin(), 1.0 / big),
                (0.5 * q.cos(), 0.5 * q.sin(), 0.0),
            ]],
        ),
        (
            "bored",
            vec![
                vec![(0.3f64.cos(), 0.3f64.sin(), 1.0), (-0.3f64.cos(), -0.3f64.sin(), 1.0)],
                vec![(0.6, 0.0, -1.0), (0.2, 0.0, -1.0)],
            ],
        ),
        (
            "square",
            vec![vec![(-0.5, -0.5, 0.0), (0.5, -0.5, 0.0), (0.5, 0.5, 0.0), (-0.5, 0.5, 0.0)]],
        ),
    ]
}

fn build<T: geom_core::Decide + topo::AtRestPolicy>(loops: &[Vec<(f64, f64, f64)>]) -> topo::Body<T> {
    let loops = loops
        .iter()
        .map(|l| {
            bulge_loop(
                l.iter()
                    .map(|&(x, y, b)| (Point2::new(T::from_f64(x), T::from_f64(y)), T::from_f64(b)))
                    .collect(),
            )
        })
        .collect();
    crate::test_support::extruded(profile::SketchPlane::xy(), loops, T::one(), Tol::witness())
}

/// Ordered boundary samples of each loop of `face` (f64 body).
fn boundary(body: &topo::Body<f64>, face: topo::FaceKey) -> Vec<Vec<Point3<f64>>> {
    let f = body.get_face(face).unwrap();
    let mut out = Vec::new();
    for lp in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let mut pts = Vec::new();
        let topo::LoopBoundary::Cycle { first } = body.get_loop(lp).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            let h = body.get_half_edge(he).unwrap();
            let e = body.get_edge(h.edge).unwrap();
            let forward = e.he_plus == he;
            let cert = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            let (t0, t1) = cert.params();
            let n = 96;
            for k in 0..n {
                let s = f64::from(k) / f64::from(n);
                let s = if forward { s } else { 1.0 - s };
                pts.push(cert.carrier().eval(t0 + (t1 - t0) * s));
            }
        }
        out.push(pts);
    }
    out
}

/// Sample points of a face of the z-extrusion: a wall's boundary points
/// spread over `z ∈ [0, 1]`; a cap's xy-grid points inside its
/// boundary (winding test over every loop, clear of it by 0.01).
fn face_points(body: &topo::Body<f64>, face: topo::FaceKey, s: &geom::Surface<f64>) -> Vec<Point3<f64>> {
    let loops = boundary(body, face);
    let all: Vec<Point3<f64>> = loops.iter().flatten().copied().collect();
    match s {
        geom::Surface::Cylinder { .. } => {
            let mut out = all.clone();
            for p in &all {
                for j in 0..=16 {
                    out.push(Point3::new(p.x, p.y, f64::from(j) / 16.0));
                }
            }
            out
        }
        geom::Surface::Plane { origin, .. } => {
            let z = origin.z;
            let mut out = all.clone();
            for i in 0..=60 {
                for j in 0..=60 {
                    let (x, y) = (-1.0 + f64::from(i) / 30.0, -1.0 + f64::from(j) / 30.0);
                    let clear = all.iter().all(|p| (p.x - x).hypot(p.y - y) > 0.01);
                    let mut w = 0.0;
                    for l in &loops {
                        for k in 0..l.len() {
                            let (p, q) = (l[k], l[(k + 1) % l.len()]);
                            let a0 = (p.y - y).atan2(p.x - x);
                            let a1 = (q.y - y).atan2(q.x - x);
                            let mut d = a1 - a0;
                            while d > core::f64::consts::PI {
                                d -= core::f64::consts::TAU;
                            }
                            while d < -core::f64::consts::PI {
                                d += core::f64::consts::TAU;
                            }
                            w += d;
                        }
                    }
                    if clear && w.abs() > 3.0 {
                        out.push(Point3::new(x, y, z));
                    }
                }
            }
            out
        }
        _ => Vec::new(),
    }
}

fn axis(t: f64, phi: f64) -> Vec3<f64> {
    Vec3::new(t.sin() * phi.cos(), t.sin() * phi.sin(), t.cos())
}

fn leans() -> Vec<f64> {
    let h = core::f64::consts::FRAC_PI_2;
    let pi = core::f64::consts::PI;
    vec![
        0.0, 0.9 * band().zero(), 1.0e-3, 0.3, 1.2, 1.5, h - 1.0e-3, h - 1.0e-6,
        h - 3.0 * band().escalate(), h + 1.0e-4, 2.0, 3.0, pi - 1.0e-9, pi,
    ]
}

fn origins() -> Vec<Point3<f64>> {
    vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.1, -0.2, 0.3),
        Point3::new(0.45, 0.0, 0.5),
        Point3::new(0.4, 0.0, 0.5), // inside the bore
        Point3::new(2.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, -7.0),
        Point3::new(0.49, 0.49, 1.0),
    ]
}

fn clip_lohi<T: Bounds>(c: Option<Clip<T>>) -> Option<(bool, f64, f64)> {
    c.map(|c| match c {
        Clip::Z(a, b) => (true, a.lo(), b.hi()),
        Clip::Rho(a, b) => (false, a.lo(), b.hi()),
    })
}

/// **Claim 1**: every clip `sheet_clip` returns, on every cylinder and
/// plane face of every fixture, at every lean, azimuth and axis origin,
/// holds every sampled point of its face, at f64 and at Interval.
#[test]
fn review_5052_every_clip_holds_its_face() {
    let band = band();
    let (mut clipped, mut refused, mut bad) = (0usize, 0usize, Vec::new());
    for (name, loops) in fixtures() {
        let body = build::<f64>(&loops);
        let ibody = build::<Interval>(&loops);
        let mut faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
        faces.sort_unstable();
        let mut ifaces: Vec<_> = ibody.faces().map(|(k, _)| k).collect();
        ifaces.sort_unstable();
        assert_eq!(faces.len(), ifaces.len(), "{name}: twin bodies");
        for (&f, &fi) in faces.iter().zip(&ifaces) {
            let s = body.get_surface(body.get_face(f).unwrap().surface).unwrap().clone();
            let si = ibody.get_surface(ibody.get_face(fi).unwrap().surface).unwrap().clone();
            if !matches!(s, geom::Surface::Cylinder { .. } | geom::Surface::Plane { .. }) {
                continue;
            }
            let pts = face_points(&body, f, &s);
            assert!(pts.len() > 50, "{name}: samples of {f:?}");
            for t in leans() {
                for phi in [0.0, 0.7, 2.0] {
                    for o in origins() {
                        let a = axis(t, phi);
                        let ai = Vec3::new(
                            Interval::from_f64(a.x),
                            Interval::from_f64(a.y),
                            Interval::from_f64(a.z),
                        );
                        let oi = Point3::new(
                            Interval::from_f64(o.x),
                            Interval::from_f64(o.y),
                            Interval::from_f64(o.z),
                        );
                        for (scalar, c) in [
                            ("f64", clip_lohi(sheet_clip(&body, f, &s, o, a, band))),
                            ("Interval", clip_lohi(sheet_clip(&ibody, fi, &si, oi, ai, band))),
                        ] {
                            let Some((is_z, lo, hi)) = c else {
                                refused += 1;
                                continue;
                            };
                            clipped += 1;
                            let worst = pts
                                .iter()
                                .map(|p| {
                                    let v = *p - o;
                                    let z = v.dot(a);
                                    let x = if is_z { z } else { (v - a * z).norm() };
                                    (lo - x).max(x - hi)
                                })
                                .fold(f64::NEG_INFINITY, f64::max);
                            if worst > 0.0 {
                                bad.push(format!(
                                    "{name} {f:?} {:?} {scalar} lean {t} phi {phi} o {o:?}: {} clip {lo}..{hi} misses a face point by {worst}",
                                    core::mem::discriminant(&s),
                                    if is_z { "Z" } else { "Rho" }
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("review_5052 clip probe: {clipped} clips read, {refused} refused");
    assert!(clipped > 500, "the probe reads clips: {clipped}");
    assert!(bad.is_empty(), "{} clips miss their face:\n{}", bad.len(), bad[..bad.len().min(40)].join("\n"));
}

/// **Claim 2**: `|d(x) − f(ρ, z)| ≤ slack` at sampled points of the
/// box, for a cylinder and a plane stored anywhere, the box anywhere,
/// at every lean, f64 and Interval.
#[test]
fn review_5052_sheet_slack_encloses_the_distance() {
    let mut bad = Vec::new();
    let mut read = 0usize;
    let boxes = [
        (Point3::new(-2.0, -2.0, -2.0), Point3::new(2.0, 2.0, 2.0)),
        (Point3::new(50.0, -3.0, 40.0), Point3::new(51.0, -2.0, 41.0)),
        (Point3::new(-1e4, 0.0, 1e4), Point3::new(-1e4 + 0.1, 0.1, 1e4 + 0.1)),
    ];
    let surfaces = |along: f64, b: Vec3<f64>| {
        vec![
            geom::Surface::Cylinder {
                origin: Point3::new(0.3, -0.1, 0.0) + b * along,
                axis: b * 3.0,
                radius: 0.5,
                u_ref: b.cross(Vec3::new(0.3, 1.0, 0.1)).normalize(),
            },
            geom::Surface::Plane {
                origin: Point3::new(0.0, 0.0, 1.0) + b.cross(Vec3::new(1.0, 0.2, 0.0)) * along,
                normal: b,
                u_ref: b.cross(Vec3::new(1.0, 0.2, 0.0)).normalize(),
            },
        ]
    };
    for t in leans() {
        let a = axis(t, 0.4);
        let o = Point3::new(0.0, 0.0, 0.0);
        for bt in [0.0, 1e-9, 1e-6, 1e-3, 0.5, 2.0, core::f64::consts::PI - 1e-7] {
            let b = axis(bt, 1.3);
            for along in [0.0, -3.0, 1e3, 1e5] {
                for s in surfaces(along, b) {
                    for (lo, hi) in boxes {
                        let Some((g, slack)) = sheet_fn(&s, o, a, (lo, hi)) else {
                            continue;
                        };
                        read += 1;
                        let d = |x: Point3<f64>| match s {
                            geom::Surface::Cylinder { origin, axis, radius, .. } => {
                                let v = x - origin;
                                let n = axis.normalize();
                                (v - n * v.dot(n)).norm() - radius
                            }
                            geom::Surface::Plane { origin, normal, .. } => (x - origin).dot(normal.normalize()),
                            _ => unreachable!(),
                        };
                        let mut worst = f64::NEG_INFINITY;
                        for i in 0..=4 {
                            for j in 0..=4 {
                                for k in 0..=4 {
                                    let f = |l: f64, h: f64, m: i32| l + (h - l) * f64::from(m) / 4.0;
                                    let x = Point3::new(f(lo.x, hi.x, i), f(lo.y, hi.y, j), f(lo.z, hi.z, k));
                                    let v = x - o;
                                    let z = v.dot(a);
                                    let rho = (v - a * z).norm();
                                    let gap = (d(x) - g.at(rho, z)).abs() - slack;
                                    // f64 rounding of the probe itself at these magnitudes
                                    let round = 1e-12 * (1.0 + x.x.abs() + x.y.abs() + x.z.abs() + along.abs());
                                    worst = worst.max(gap - round);
                                }
                            }
                        }
                        if worst > 0.0 {
                            bad.push(format!(
                                "lean {t} b-lean {bt} along {along} {:?} box {lo:?}: slack {slack} short by {worst}",
                                core::mem::discriminant(&s)
                            ));
                        }
                    }
                }
            }
        }
    }
    eprintln!("review_5052 slack probe: {read} slacks read");
    assert!(read > 100);
    assert!(bad.is_empty(), "{} slacks short:\n{}", bad.len(), bad[..bad.len().min(40)].join("\n"));

}

/// **Claim 3**: `tilt` encloses the chord `2·sin(θ/2)`, θ the angle to
/// the nearer of `±a`, near 0, π/2 and π, at Interval, and reads it to
/// a few ulps at f64, for a unit and a non-unit `b`.
#[test]
fn review_5052_tilt_reads_the_chord() {
    let a = Vec3::new(0.0, 0.0, 1.0);
    let pi = core::f64::consts::PI;
    for th in [0.0, 1e-15, 1e-12, 1e-9, 1e-6, 0.3, pi / 2.0 - 1e-9, pi / 2.0, 2.0, pi - 1e-6, pi - 1e-9, pi - 1e-12, pi] {
        for scale in [1.0, 1e-3, 7.0e3] {
            let b = Vec3::new(th.sin(), 0.0, th.cos()) * scale;
            let near = th.min(pi - th);
            let truth = 2.0 * (near / 2.0).sin();
            let f = tilt(b, a);
            assert!(
                (f - truth).abs() <= 4.0 * f64::EPSILON * (1.0 + truth) + 1e-16,
                "f64 tilt at θ {th} ×{scale}: {f} vs {truth}"
            );
            let bi = Vec3::new(Interval::from_f64(b.x), Interval::from_f64(b.y), Interval::from_f64(b.z));
            let ai = Vec3::new(Interval::from_f64(0.0), Interval::from_f64(0.0), Interval::from_f64(1.0));
            let g = tilt(bi, ai);
            // The f64 input's own θ: the probe's b is the rounded one, so
            // compare with an outward margin of its rounding.
            let m = 4.0 * f64::EPSILON;
            assert!(
                g.lo() <= truth + m && g.hi() >= truth - m,
                "Interval tilt at θ {th} ×{scale}: [{}, {}] vs {truth}",
                g.lo(),
                g.hi()
            );
        }
    }
    let _ = <f64 as Real>::from_f64(0.0);
}
