//! Reviewer probe (PR 3843, lane reach-dual3843-r2). Not a suite row.
//!
//! Revolved bodies carrying one sphere or torus face, cut by planes
//! drawn near that face (just clear, inside the pad, just meeting, and
//! well across), at three scales and two poses. Oracle: the body's
//! radius profile `rho(y)` in closed form; each half's volume by a
//! composite Gauss slice integral of the circular-segment area; "the
//! plane meets the face" by dense sampling of the face's own
//! parametrization (sampling only under-reports a meeting, so a
//! sampled meeting is certain). Never the kernel's numbers.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::revolve_common::axis_y;
use geom_core::{Affine3, Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ArcSweep, RawLoop, bulge_from_center, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitPart, SplitPlane, split};
use topo::{
    Body, DATUM_UNIT_NORM, SolidContainment, point_in_solid, validate, validate_closed,
    validate_geometric,
};

/// One test family: profile chain at unit scale, `rho(y)` pieces, the
/// curved face's meridian `t -> (rho, y)` over `[t0, t1]`.
struct Family {
    name: String,
    chain: Vec<(Point2<f64>, f64)>,
    tangent: Vec<usize>,
    /// `(y0, y1, rho)` smooth pieces.
    pieces: Vec<(f64, f64, Box<dyn Fn(f64) -> f64>)>,
    meridian: Box<dyn Fn(f64) -> (f64, f64)>,
    t: (f64, f64),
}

fn arc(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>, s: ArcSweep) -> f64 {
    bulge_from_center(a, b, c, s)
}

/// Cylinder r=1, y in [0,1], spherical cap about (0, c).
fn cap(c: f64) -> Family {
    let r = (1.0 + (1.0 - c) * (1.0 - c)).sqrt();
    let (a, b) = (Point2::new(1.0, 1.0), Point2::new(0.0, c + r));
    Family {
        name: format!("cap c={c}"),
        chain: vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (a, arc(a, b, Point2::new(0.0, c), ArcSweep::Ccw)),
            (b, 0.0),
        ],
        tangent: vec![],
        pieces: vec![
            (0.0, 1.0, Box::new(|_| 1.0)),
            (
                1.0,
                c + r,
                Box::new(move |y: f64| (r * r - (y - c) * (y - c)).max(0.0).sqrt()),
            ),
        ],
        meridian: Box::new(move |t: f64| (r * t.cos(), c + r * t.sin())),
        t: ((1.0 - c).atan2(1.0), FRAC_PI_2),
    }
}

/// The ball about (0, c) cut flat at y = 1 (its face spans more than a
/// hemisphere when c < 1).
fn truncated(c: f64) -> Family {
    let r = (1.0 + (1.0 - c) * (1.0 - c)).sqrt();
    let (a, b) = (Point2::new(0.0, c - r), Point2::new(1.0, 1.0));
    Family {
        name: format!("truncated ball c={c}"),
        chain: vec![
            (a, arc(a, b, Point2::new(0.0, c), ArcSweep::Ccw)),
            (b, 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        tangent: vec![],
        pieces: vec![(
            c - r,
            1.0,
            Box::new(move |y: f64| (r * r - (y - c) * (y - c)).max(0.0).sqrt()),
        )],
        meridian: Box::new(move |t: f64| (r * t.cos(), c + r * t.sin())),
        t: (-FRAC_PI_2, (1.0 - c).atan2(1.0)),
    }
}

/// A barrel: a sphere ZONE (no pole) between y = 0 and y = 1, bulging
/// to `R`, centre (0, 0.5), rims of radius `w`.
fn barrel(w: f64) -> Family {
    let r = (w * w + 0.25f64).sqrt();
    let (a, b) = (Point2::new(w, 0.0), Point2::new(w, 1.0));
    let t1 = (0.5f64).atan2(w);
    Family {
        name: format!("barrel w={w}"),
        chain: vec![
            (Point2::new(0.0, 0.0), 0.0),
            (a, arc(a, b, Point2::new(0.0, 0.5), ArcSweep::Ccw)),
            (b, 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        tangent: vec![],
        pieces: vec![(
            0.0,
            1.0,
            Box::new(move |y: f64| (r * r - (y - 0.5) * (y - 0.5)).max(0.0).sqrt()),
        )],
        meridian: Box::new(move |t: f64| (r * t.cos(), 0.5 + r * t.sin())),
        t: (-t1, t1),
    }
}

/// Cylinder of radius `a`, y in [0,1], top rim rounded by radius `r`.
fn rounded(a: f64, r: f64) -> Family {
    let (p, q) = (Point2::new(a, 1.0), Point2::new(a - r, 1.0 + r));
    Family {
        name: format!("rounded a={a} r={r}"),
        chain: vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(a, 0.0), 0.0),
            (p, arc(p, q, Point2::new(a - r, 1.0), ArcSweep::Ccw)),
            (q, 0.0),
            (Point2::new(0.0, 1.0 + r), 0.0),
        ],
        tangent: vec![2, 3],
        pieces: vec![
            (0.0, 1.0, Box::new(move |_| a)),
            (
                1.0,
                1.0 + r,
                Box::new(move |y: f64| a - r + (r * r - (y - 1.0) * (y - 1.0)).max(0.0).sqrt()),
            ),
        ],
        meridian: Box::new(move |t: f64| (a - r + r * t.cos(), 1.0 + r * t.sin())),
        t: (0.0, FRAC_PI_2),
    }
}

fn build(f: &Family, s: f64) -> Option<Body<f64>> {
    let chain = f
        .chain
        .iter()
        .map(|(p, b)| (Point2::new(p.x * s, p.y * s), *b))
        .collect();
    let prof = profile::Profile::new(
        profile::SketchPlane::xy(),
        vec![bulge_loop(chain).with_tangent_joints(f.tangent.clone())],
    )
    .validate(Tol::witness())
    .map_err(|e| eprintln!("BUILD {} s={s}: {e:?}", f.name))
    .ok()?;
    revolve(
        &prof,
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .map_err(|e| eprintln!("BUILD {} s={s}: {e:?}", f.name))
    .ok()
    .map(|r| r.body)
}

/// Area of {x < x0} in the disk of radius rho.
fn seg(rho: f64, x0: f64) -> f64 {
    if rho <= 0.0 || x0 <= -rho {
        return 0.0;
    }
    if x0 >= rho {
        return PI * rho * rho;
    }
    rho * rho * (-x0 / rho).acos() + x0 * (rho * rho - x0 * x0).sqrt()
}

/// Volume of the unit-scale body on the negative side of n·p < k.
fn below_volume(f: &Family, n: Vec3<f64>, k: f64) -> f64 {
    let m = (n.x * n.x + n.z * n.z).sqrt();
    let g = [
        (-(3.0f64 / 5.0).sqrt(), 5.0 / 9.0),
        (0.0, 8.0 / 9.0),
        ((3.0f64 / 5.0).sqrt(), 5.0 / 9.0),
    ];
    let mut v = 0.0;
    let mut cuts: Vec<(f64, f64, &dyn Fn(f64) -> f64)> = Vec::new();
    for (y0, y1, rho) in &f.pieces {
        // Split at the horizontal plane's height so the slice-area step
        // falls on a panel boundary.
        let ys = if n.y.abs() > 1e-300 { k / n.y } else { f64::NAN };
        if m < 1e-15 && ys > *y0 && ys < *y1 {
            cuts.push((*y0, ys, rho.as_ref()));
            cuts.push((ys, *y1, rho.as_ref()));
        } else {
            cuts.push((*y0, *y1, rho.as_ref()));
        }
    }
    for (y0, y1, rho) in cuts {
        let panels = 40_000;
        let h = (y1 - y0) / panels as f64;
        for i in 0..panels {
            let mid = y0 + (i as f64 + 0.5) * h;
            for (x, w) in g {
                let y = mid + x * h / 2.0;
                let r = rho(y);
                let a = if m < 1e-15 {
                    if n.y * y < k { PI * r * r } else { 0.0 }
                } else {
                    seg(r, (k - n.y * y) / m)
                };
                v += a * w * h / 2.0;
            }
        }
    }
    v
}

/// Sampled min and max of n·p − k over the curved face (unit scale).
fn face_range(f: &Family, n: Vec3<f64>, k: f64) -> (f64, f64) {
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for i in 0..=400 {
        let t = f.t.0 + (f.t.1 - f.t.0) * i as f64 / 400.0;
        let (rho, y) = (f.meridian)(t);
        for j in 0..720 {
            let th = 2.0 * PI * j as f64 / 720.0;
            let d = n.x * rho * th.cos() + n.y * y + n.z * rho * th.sin() - k;
            lo = lo.min(d);
            hi = hi.max(d);
        }
    }
    (lo, hi)
}

/// Exact inside test at unit scale, with a margin.
fn inside(f: &Family, p: Point3<f64>, margin: f64) -> Option<bool> {
    let rr = (p.x * p.x + p.z * p.z).sqrt();
    let mut best = None;
    for (y0, y1, rho) in &f.pieces {
        if p.y >= *y0 && p.y <= *y1 {
            best = Some(rho(p.y));
        }
    }
    let (ylo, yhi) = (f.pieces[0].0, f.pieces.last().unwrap().1);
    match best {
        None => Some(false),
        Some(r) => {
            if rr < r - margin && p.y > ylo + margin && p.y < yhi - margin {
                Some(true)
            } else if rr > r + margin || p.y < ylo - margin || p.y > yhi + margin {
                Some(false)
            } else {
                None
            }
        }
    }
}

/// A tiny deterministic generator.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    let band = Band::linear(Tol::witness()).unwrap();
    UnitVec3::new(v, DATUM_UNIT_NORM, band).unwrap()
}

#[derive(Default, Debug)]
struct Tally {
    admitted: usize,
    refused_gate: usize,
    refused_other: usize,
    admitted_while_meeting: usize,
    volume_bad: usize,
    tier_bad: usize,
    pis_bad: usize,
    pis_checked: usize,
}

fn families() -> Vec<Family> {
    vec![
        cap(0.25),
        cap(-0.5),
        cap(0.9),
        cap(1.3),
        truncated(0.25),
        truncated(0.9),
        barrel(1.0),
        barrel(0.4),
        rounded(1.0, 0.25),
        rounded(1.0, 0.5),
        rounded(1.0, 0.05),
        rounded(2.0, 0.9),
    ]
}

fn posed(f: &Family, s: f64, pose: Option<Affine3<f64>>) -> Option<Body<f64>> {
    let mut body = build(f, s)?;
    if let Some(map) = pose {
        body = topo::transform::transform_rigid(&body, &map, Tol::witness())
            .map_err(|e| eprintln!("POSE {} s={s}: {e:?}", f.name))
            .ok()?;
    }
    Some(body)
}

/// The kernel's plane for the unit-scale plane `n·p = k`.
fn kplane(n: Vec3<f64>, k: f64, s: f64, pose: Option<Affine3<f64>>) -> SplitPlane<f64> {
    let (mut o, mut nn) = (Point3::new(n.x * k * s, n.y * k * s, n.z * k * s), n);
    if let Some(map) = pose {
        o = map.transform_point(o);
        nn = map.transform_vec(nn);
    }
    SplitPlane {
        origin: o,
        normal: unit(nn),
    }
}

fn run(f: &Family, s: f64, pose: Option<Affine3<f64>>, seed: u64, tally: &mut Tally) {
    let Some(body) = posed(f, s, pose) else { return };
    let total = below_volume(f, Vec3::new(0.0, 1.0, 0.0), 1e9);
    let mut rng = Lcg(seed);
    for _ in 0..60 {
        // A random normal, and a random point on the curved face.
        let (u, w) = (rng.next(), rng.next());
        let zc = 2.0 * u - 1.0;
        let ph = 2.0 * PI * w;
        let rr = (1.0 - zc * zc).sqrt();
        let mut n = Vec3::new(rr * ph.cos(), zc, rr * ph.sin());
        if rng.next() < 0.6 {
            // Near-axial normals: the zone box is tight in y only.
            let al = 0.4 * rng.next() * rng.next();
            let sg = if rng.next() < 0.5 { 1.0 } else { -1.0 };
            n = Vec3::new(al.sin() * ph.cos(), sg * al.cos(), al.sin() * ph.sin());
        }
        let t = f.t.0 + (f.t.1 - f.t.0) * rng.next();
        let th = 2.0 * PI * rng.next();
        let (rho, y) = (f.meridian)(t);
        let p = Point3::new(rho * th.cos(), y, rho * th.sin());
        // Shift the plane off that point by a log-uniform offset, then
        // re-centre it on the face's sampled extreme so that "just
        // clear" / "in the pad" / "just meeting" all occur.
        let k0 = n.x * p.x + n.y * p.y + n.z * p.z;
        let (lo, hi) = face_range(f, n, k0);
        let mag = 10f64.powf(-12.0 + 11.0 * rng.next());
        let k = match (rng.next() * 4.0) as u32 {
            0 => k0 + lo - mag,  // just past the face's low extreme (clear)
            1 => k0 + lo + mag,  // just inside (meets)
            2 => k0 + hi + mag,  // clear on the other side
            _ => k0 + hi - mag,  // just inside
        };
        let (dlo, dhi) = face_range(f, n, k);
        let meets = dlo < -1e-12 && dhi > 1e-12;
        // Kernel plane at scale s, then posed.
        let (mut o, mut nn) = (Point3::new(n.x * k * s, n.y * k * s, n.z * k * s), n);
        if let Some(map) = pose {
            o = map.transform_point(o);
            nn = map.transform_vec(nn);
        }
        let plane = SplitPlane {
            origin: o,
            normal: unit(nn),
        };
        check(f, s, pose, &body, &plane, n, k, meets, dlo, dhi, total, &mut rng, tally);
    }
}

#[allow(clippy::too_many_arguments)]
fn check(
    f: &Family,
    s: f64,
    pose: Option<Affine3<f64>>,
    body: &Body<f64>,
    plane: &SplitPlane<f64>,
    n: Vec3<f64>,
    k: f64,
    meets: bool,
    dlo: f64,
    dhi: f64,
    total: f64,
    rng: &mut Lcg,
    tally: &mut Tally,
) -> &'static str {
    let eps = Tol::witness().get().eps;
    let band = Band::linear(Tol::witness()).unwrap();
        match split(body, plane, Tol::witness()) {
            Err(SplitError::Reduce(topo::splitting::SplitReduceError::CurvedBooleanUnsupported {
                ..
            })) => { tally.refused_gate += 1; return "gate"; }
            Err(_e) => {
                tally.refused_other += 1;
                eprintln!("OTHER {} s={s}: {_e}", f.name);
                return "other";
            }
            Ok(res) => {
                tally.admitted += 1;
                if meets {
                    tally.admitted_while_meeting += 1;
                    eprintln!(
                        "MEETING ADMITTED {} s={s} pose={} n={n:?} k={k} range=({dlo:e},{dhi:e})",
                        f.name,
                        pose.is_some()
                    );
                }
                let vb = below_volume(f, n, k);
                let va = total - vb;
                let mut vol = |part: &SplitPart<f64>| match part {
                    SplitPart::Empty => (0.0, 0.0),
                    SplitPart::Body(b) => {
                        if validate(b).is_err()
                            || validate_closed(b).is_err()
                            || validate_geometric(b, Tol::witness()).is_err()
                        {
                            tally.tier_bad += 1;
                            eprintln!("TIER FAIL {} s={s}", f.name);
                        }
                        match topo::props::mass_properties(b, Tol::witness()) {
                            Ok(m) => (m.volume, m.volume_pad),
                            Err(e) => {
                                eprintln!("PROPS {} s={s}: {e:?}", f.name);
                                (f64::NAN, f64::INFINITY)
                            }
                        }
                    }
                };
                let (ka, pa) = vol(&res.above);
                let (kb, pb) = vol(&res.below);
                let s3 = s * s * s;
                let slack = 1e-7 * s3 + 1e3 * eps * s * s;
                if (ka - va * s3).abs() > pa + slack || (kb - vb * s3).abs() > pb + slack {
                    tally.volume_bad += 1;
                    eprintln!(
                        "VOLUME {} s={s} pose={} n={n:?} k={k} meets={meets} kernel above {ka}±{pa} below {kb}±{pb} oracle above {} below {}",
                        f.name,
                        pose.is_some(),
                        va * s3,
                        vb * s3
                    );
                }
                // point_in_solid against the oracle, a few samples.
                for _ in 0..6 {
                    let q = Point3::new(
                        (rng.next() * 2.0 - 1.0) * 2.2,
                        rng.next() * 3.0 - 1.2,
                        (rng.next() * 2.0 - 1.0) * 2.2,
                    );
                    let margin = 1e-4;
                    let Some(ins) = inside(f, q, margin) else {
                        continue;
                    };
                    let d = n.x * q.x + n.y * q.y + n.z * q.z - k;
                    if d.abs() < margin {
                        continue;
                    }
                    let mut kq = Point3::new(q.x * s, q.y * s, q.z * s);
                    if let Some(map) = pose {
                        kq = map.transform_point(kq);
                    }
                    for (part, want) in [(&res.above, ins && d > 0.0), (&res.below, ins && d < 0.0)] {
                        let got = match part {
                            SplitPart::Empty => Some(false),
                            SplitPart::Body(b) => {
                                match point_in_solid(b, kq, band, Tol::witness()) {
                                    Ok(SolidContainment::In) => Some(true),
                                    Ok(SolidContainment::Out) => Some(false),
                                    _ => None,
                                }
                            }
                        };
                        if let Some(g) = got {
                            tally.pis_checked += 1;
                            if g != want {
                                tally.pis_bad += 1;
                                eprintln!("PIS {} s={s} q={q:?} want {want} got {g}", f.name);
                            }
                        }
                    }
                }
            }
        }
    "ok"
}

#[test]
fn probe_random_planes_near_the_curved_face() {
    let pose = Affine3::rotation_about_axis(
        Point3::new(0.3, -0.2, 0.1),
        Vec3::new(0.48, 0.6, 0.64),
        0.7,
    );
    let mut tally = Tally::default();
    for (i, f) in families().iter().enumerate() {
        for s in [1e-3, 1.0, 1e3] {
            for posed in [None, Some(pose)] {
                run(f, s, posed, 1000 + i as u64 * 7 + (s.log10() as i64 + 5) as u64, &mut tally);
            }
        }
    }
    eprintln!("TALLY {tally:?}");
    assert_eq!(tally.admitted_while_meeting, 0, "{tally:?}");
    assert_eq!(tally.volume_bad, 0, "{tally:?}");
    assert_eq!(tally.tier_bad, 0, "{tally:?}");
    assert_eq!(tally.pis_bad, 0, "{tally:?}");
}

/// Deterministic grid near the cap / rounding's rim: planes through the
/// cylinder whose highest point over the rim radius sits `delta` below
/// the face, at several tilts and azimuths.
#[test]
fn probe_grid_just_under_the_face() {
    let pose = Affine3::rotation_about_axis(
        Point3::new(0.3, -0.2, 0.1),
        Vec3::new(0.48, 0.6, 0.64),
        0.7,
    );
    let mut tally = Tally::default();
    let fams = vec![cap(0.25), cap(-0.5), cap(0.9), cap(1.3), rounded(1.0, 0.25), rounded(1.0, 0.05), rounded(2.0, 0.9)];
    for f in &fams {
        let a = f.pieces[0].2(0.5);
        let total = below_volume(f, Vec3::new(0.0, 1.0, 0.0), 1e9);
        for s in [1e-3, 1.0, 1e3] {
            for pz in [None, Some(pose)] {
                let Some(body) = posed(f, s, pz) else { continue };
                let mut rng = Lcg(7);
                let mut row = String::new();
                for phi in [0.0f64, 0.05, 0.2] {
                    for psi in [0.0f64, 2.5] {
                        for delta in [1e-1, 1e-3, 1e-5, 1e-7, 1e-8, -1e-8, -1e-5, -1e-2] {
                            let n = Vec3::new(phi.sin() * psi.cos(), phi.cos(), phi.sin() * psi.sin());
                            // highest point of the plane over rho <= a at y: plane n·p = k,
                            // y = (k - sin(phi)*rho_dir)/cos(phi); top at rim y=1 reached when
                            // k = cos(phi)*(1 - delta) - sin(phi)*a.
                            let k = phi.cos() * (1.0 - delta) - phi.sin() * a;
                            let (dlo, dhi) = face_range(f, n, k);
                            let meets = dlo < -1e-12 && dhi > 1e-12;
                            let plane = kplane(n, k, s, pz);
                            let r = check(f, s, pz, &body, &plane, n, k, meets, dlo, dhi, total, &mut rng, &mut tally);
                            row.push_str(&format!("{}", match r { "ok" => "A", "gate" => "g", _ => "x" }));
                        }
                        row.push(' ');
                    }
                }
                eprintln!("GRID {:24} s={s:<6} posed={:5} {row}", f.name, pz.is_some());
            }
        }
    }
    eprintln!("TALLY {tally:?}");
    assert_eq!(tally.admitted_while_meeting, 0, "{tally:?}");
    assert_eq!(tally.volume_bad, 0, "{tally:?}");
    assert_eq!(tally.tier_bad, 0, "{tally:?}");
    assert_eq!(tally.pis_bad, 0, "{tally:?}");
}

/// The PR's own capped cylinder, rigidly posed: the PR's φ = 0, y = ½
/// cut (0.5 clear of the cap) refuses in a rotated pose although it
/// splits unposed — the gate's axis-aligned box is pose-dependent.
#[test]
fn probe_posed_pr_fixture() {
    let f = cap(0.25);
    for angle in [0.0, 0.1, 0.3, 0.7, core::f64::consts::FRAC_PI_2] {
        let pose = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), angle);
        let body = posed(&f, 1.0, Some(pose)).unwrap();
        let plane = kplane(Vec3::new(0.0, 1.0, 0.0), 0.5, 1.0, Some(pose));
        let r = split(&body, &plane, Tol::witness());
        eprintln!("POSED angle {angle}: {}", match &r { Ok(_) => "split".to_string(), Err(e) => format!("refused: {e}") });
    }
}
