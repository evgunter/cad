//! Reviewer probes for PR #4044 (dual lane reach-dual4044-r2).
//! Mounted locally in `crates/sweep/tests/all.rs` as
//! `#[path = "../../../probes/dual4044_r2.rs"] mod dual4044_r2;`.
//! Oracle: closed-form volumes and an independent indicator function
//! per operand (signed margins: positive inside), sampled against
//! `point_in_solid`. Nothing here reads the kernel for its oracle.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use std::fmt::Write as _;
use std::panic::{AssertUnwindSafe, catch_unwind};

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, Body, BooleanOp, SolidContainment};

type Ind = std::rc::Rc<dyn Fn(Point3<f64>) -> f64>;

#[derive(Clone)]
struct Solid {
    body: AtRestBody<f64>,
    ind: Ind,
    vol: f64,
}

fn ball(r: f64, y: f64) -> Solid {
    let b = revolved_about_y(
        vec![
            (Point2::new(0.0, y - r), 1.0),
            (Point2::new(0.0, y + r), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    let c = Point3::new(0.0, y, 0.0);
    Solid {
        body: finished("ball", b, Tol::witness()),
        ind: std::rc::Rc::new(move |p| r - (p - c).norm()),
        vol: 4.0 / 3.0 * PI * r.powi(3),
    }
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64), map: Affine3<f64>) -> Solid {
    let b: Body<f64> = sweep::test_support::brick(x, y, z, Tol::witness());
    let b = topo::transform_rigid(&b, &map, Tol::witness()).unwrap();
    let inv = map.inverse();
    Solid {
        body: finished("brick", b, Tol::witness()),
        ind: std::rc::Rc::new(move |p| {
            let q = inv.transform_point(p);
            (q.x - x.0)
                .min(x.1 - q.x)
                .min(q.y - y.0)
                .min(y.1 - q.y)
                .min(q.z - z.0)
                .min(z.1 - q.z)
        }),
        vol: (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0),
    }
}

/// A slab of thickness `t` (lateral half-width `w`) whose near face is
/// the plane at distance `s` from `c` along unit `n`, material beyond.
fn slab_toward(c: Point3<f64>, n: Vec3<f64>, s: f64, t: f64, w: f64) -> Solid {
    let n = n / n.norm();
    let y = Vec3::new(0.0, 1.0, 0.0);
    let ax = y.cross(n);
    let rot = if ax.norm() < 1e-14 {
        if n.y > 0.0 {
            Affine3::translation(Vec3::new(0.0, 0.0, 0.0))
        } else {
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), PI)
        }
    } else {
        Affine3::rotation_about_axis(Point3::origin(), ax / ax.norm(), ax.norm().atan2(y.dot(n)))
    };
    let map = Affine3::translation((c - Point3::origin()) + n * s) * rot;
    brick((-w, w), (0.0, t), (-w, w), map)
}

fn dir(lat_deg: f64, az_deg: f64) -> Vec3<f64> {
    // az measured from +x toward +z (the chart's: seams at 0 and 180);
    // lat from the xz plane toward +y.
    let (l, a) = (lat_deg.to_radians(), az_deg.to_radians());
    Vec3::new(l.cos() * a.cos(), l.sin(), l.cos() * a.sin())
}

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

fn and(x: &Solid, y: &Solid, vol: f64) -> Solid {
    let (fx, fy) = (x.ind.clone(), y.ind.clone());
    Solid {
        body: run(BooleanOp::Intersect, &x.body, &y.body).expect("operand builds"),
        ind: std::rc::Rc::new(move |p| fx(p).min(fy(p))),
        vol,
    }
}

fn run(op: BooleanOp, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Result<AtRestBody<f64>, String> {
    let out = match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    };
    match out {
        Ok(o) => o
            .body()
            .map(|b| b.body.clone())
            .ok_or_else(|| "EMPTY".to_string()),
        Err(e) => Err(format!("{e:?}")),
    }
}

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[derive(Default)]
struct Tally {
    built: usize,
    refused: usize,
    wrong: Vec<String>,
    log: String,
}

/// Runs `op(x, y)`; on a build, checks tiers 1-3, volume against
/// `want`, and samples `point_in_solid` against the oracle around
/// `focus` (radius `rad`) and in the operands' box `boxc ± boxr`.
#[allow(clippy::too_many_arguments)]
fn check(
    t: &mut Tally,
    label: &str,
    op: BooleanOp,
    x: &Solid,
    y: &Solid,
    want: f64,
    focus: Point3<f64>,
    rad: f64,
    boxc: Point3<f64>,
    boxr: f64,
) -> Option<Solid> {
    eprintln!("CASE {label} {op:?}");
    let r = catch_unwind(AssertUnwindSafe(|| run(op, &x.body, &y.body)));
    let body = match r {
        Err(_) => {
            t.wrong.push(format!("{label} {op:?}: PANIC"));
            return None;
        }
        Ok(Err(e)) => {
            t.refused += 1;
            let short: String = e.chars().take(240).collect();
            let _ = writeln!(t.log, "  REFUSE {label} {op:?}: {short}");
            return None;
        }
        Ok(Ok(b)) => b,
    };
    t.built += 1;
    let (fx, fy) = (x.ind.clone(), y.ind.clone());
    let ind: Ind = match op {
        BooleanOp::Union => std::rc::Rc::new(move |p| fx(p).max(fy(p))),
        BooleanOp::Intersect => std::rc::Rc::new(move |p| fx(p).min(fy(p))),
        BooleanOp::Subtract => std::rc::Rc::new(move |p| fx(p).min(-fy(p))),
    };
    let mut bad = Vec::new();
    if let Err(e) = topo::validate(&body) {
        bad.push(format!("validate {e:?}"));
    }
    if let Err(e) = topo::validate_closed(&body) {
        bad.push(format!("closed {e:?}"));
    }
    if let Err(e) = topo::validate_geometric(&body, Tol::witness()) {
        bad.push(format!("geometric {e:?}"));
    }
    match topo::mass_properties(&body, Tol::witness()) {
        Ok(p) => {
            let rel = (p.volume - want).abs() / want.abs().max(boxr.powi(3));
            if rel > 1e-9 {
                bad.push(format!("volume {} want {want} (rel {rel:.2e})", p.volume));
            }
        }
        Err(e) => bad.push(format!("props {e:?}")),
    }
    let band = Band::linear(Tol::witness()).unwrap();
    let mut rng = Lcg(0x9e37_79b9 ^ label.len() as u64);
    let (mut agree, mut pis_ref, mut wrongpts) = (0, 0, 0);
    let mut first_wrong = String::new();
    for i in 0..160 {
        let (c, rr) = if i % 2 == 0 { (focus, rad) } else { (boxc, boxr) };
        let p = Point3::new(
            c.x + rr * (2.0 * rng.f() - 1.0),
            c.y + rr * (2.0 * rng.f() - 1.0),
            c.z + rr * (2.0 * rng.f() - 1.0),
        );
        let m = ind(p);
        if m.abs() < 1e-7 * boxr.max(rad) {
            continue;
        }
        match topo::point_in_solid(&body, p, band, Tol::witness()) {
            Ok(SolidContainment::In) if m > 0.0 => agree += 1,
            Ok(SolidContainment::Out) if m < 0.0 => agree += 1,
            Ok(got) => {
                wrongpts += 1;
                if first_wrong.is_empty() {
                    first_wrong = format!("{p:?} got {got:?} margin {m:.3e}");
                }
            }
            Err(_) => pis_ref += 1,
        }
    }
    if wrongpts > 0 {
        bad.push(format!("{wrongpts} misclassified points, e.g. {first_wrong}"));
    }
    let _ = writeln!(
        t.log,
        "  BUILT  {label} {op:?}: F{} E{} pis agree {agree}, pis refused {pis_ref}{}",
        body.faces().count(),
        body.edges().count(),
        if bad.is_empty() { String::new() } else { format!(" !! {bad:?}") }
    );
    if !bad.is_empty() {
        t.wrong.push(format!("{label} {op:?}: {bad:?}"));
    }
    Some(Solid { body, ind, vol: want })
}

/// Every op both orders of `x` against cutter `y` where `y` removes
/// `common` of `x` (common = vol(x ∩ y)).
#[allow(clippy::too_many_arguments)]
fn all_ops(
    t: &mut Tally,
    label: &str,
    x: &Solid,
    y: &Solid,
    common: f64,
    focus: Point3<f64>,
    rad: f64,
    boxc: Point3<f64>,
    boxr: f64,
) {
    for (op, a, b, want, tag) in [
        (BooleanOp::Union, x, y, x.vol + y.vol - common, "x∪y"),
        (BooleanOp::Union, y, x, x.vol + y.vol - common, "y∪x"),
        (BooleanOp::Intersect, x, y, common, "x∩y"),
        (BooleanOp::Intersect, y, x, common, "y∩x"),
        (BooleanOp::Subtract, x, y, x.vol - common, "x∖y"),
        (BooleanOp::Subtract, y, x, y.vol - common, "y∖x"),
    ] {
        check(t, &format!("{label} [{tag}]"), op, a, b, want, focus, rad, boxc, boxr);
    }
}

fn lens_volume(r1: f64, r2: f64, d: f64) -> f64 {
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    cap(r1, r1 - x) + cap(r2, r2 - (d - x))
}

fn report(name: &str, t: &Tally) {
    eprintln!(
        "== {name} @ε {}: built {}, refused {}, WRONG {}\n{}",
        Tol::witness().eps(),
        t.built,
        t.refused,
        t.wrong.len(),
        t.log
    );
    for w in &t.wrong {
        eprintln!("  WRONG: {w}");
    }
}

/// Lens caps: azimuth sweep, depth sweep, pole approach, scales.
#[test]
fn probe_lens_caps() {
    for (scale, set) in [(1.0, 0), (1e-3, 1), (1e3, 1)] {
        let mut t = Tally::default();
        let k = scale;
        let lens = ball(1.0 * k, 0.0);
        let b2 = ball(0.8 * k, 1.4 * k);
        let lens = and(&lens, &b2, lens_volume(k, 0.8 * k, 1.4 * k));
        let o = Point3::origin();
        let mut poses: Vec<(f64, f64, f64)> = Vec::new(); // (polar tilt, azimuth, h)
        if set == 0 {
            for az in [90.0, 60.0, 45.0, 30.0, 15.0, 5.0, 0.0, 180.0, 175.0, 270.0, 300.0] {
                poses.push((20.0, az, 0.015));
            }
            for h in [1e-6, 1e-4, 1e-3, 0.02, 0.028, 0.0295] {
                poses.push((20.0, 90.0, h));
            }
            for tilt in [5.0, 9.0, 10.5, 12.0] {
                poses.push((tilt, 90.0, 0.015));
            }
        } else {
            poses.push((20.0, 90.0, 0.015));
            poses.push((20.0, 250.0, 0.002));
        }
        for (tilt, az, h) in poses {
            let n = dir(90.0 - tilt, az);
            let slab = slab_toward(o, n, (1.0 - h) * k, 1.0 * k, 3.0 * k);
            let label = format!("lens×{k} tilt {tilt} az {az} h {h}");
            let foc = Point3::origin() + n * k;
            all_ops(&mut t, &label, &lens, &slab, cap(k, h * k), foc, 0.3 * k, Point3::new(0.0, 0.7 * k, 0.0), 1.0 * k);
        }
        report(&format!("lens caps ×{k}"), &t);
        assert!(t.wrong.is_empty(), "wrong bodies: {:#?}", t.wrong);
    }
}

/// The lens's OTHER face (the 0.8 ball's lower cap) cut from below; an
/// unequal lens.
#[test]
fn probe_lens_lower_face_and_unequal_lens() {
    let mut t = Tally::default();
    let b1 = ball(1.0, 0.0);
    let b2 = ball(0.8, 1.4);
    let lens = and(&b1, &b2, lens_volume(1.0, 0.8, 1.4));
    let c2 = Point3::new(0.0, 1.4, 0.0);
    for az in [90.0, 200.0] {
        let n = dir(-90.0 + 25.0, az);
        let slab = slab_toward(c2, n, 0.8 * 0.985, 1.0, 3.0);
        all_ops(&mut t, &format!("lens lower az {az}"), &lens, &slab, cap(0.8, 0.8 * 0.015), c2 + n * 0.8, 0.3, Point3::new(0.0, 0.7, 0.0), 1.0);
    }
    let b3 = ball(1.3, 1.5);
    let lens2 = and(&b1, &b3, lens_volume(1.0, 1.3, 1.5));
    // Unit sphere's face: y ≥ (1.5²+1−1.69)/3 = 0.52 → polar ≤ 58.7°.
    for (tilt, az) in [(40.0, 120.0), (40.0, 300.0)] {
        let n = dir(90.0 - tilt, az);
        let slab = slab_toward(Point3::origin(), n, 0.98, 1.0, 3.0);
        all_ops(&mut t, &format!("lens2 tilt {tilt} az {az}"), &lens2, &slab, cap(1.0, 0.02), Point3::origin() + n, 0.3, Point3::new(0.0, 0.8, 0.0), 1.3);
    }
    report("lower face / unequal lens", &t);
    assert!(t.wrong.is_empty(), "wrong bodies: {:#?}", t.wrong);
}

/// Sphere ∖ box: the meridian below the circle meets the box's circle
/// arc (a split) and above reaches the pole.
#[test]
fn probe_sphere_less_box() {
    let mut t = Tally::default();
    let b = ball(1.0, 0.0);
    let bx = brick((0.5, 2.0), (-2.0, 2.0), (-2.0, 2.0), Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
    let removed = cap(1.0, 0.5);
    let carved = Solid {
        body: run(BooleanOp::Subtract, &b.body, &bx.body).expect("carve"),
        ind: {
            let (f, g) = (b.ind.clone(), bx.ind.clone());
            std::rc::Rc::new(move |p| f(p).min(-g(p)))
        },
        vol: b.vol - removed,
    };
    for (lat, az, h) in [(60.0, 30.0, 0.001), (-60.0, 30.0, 0.001), (20.0, 130.0, 0.015), (0.0, 270.0, 0.015)] {
        let n = dir(lat, az);
        let slab = slab_toward(Point3::origin(), n, 1.0 - h, 1.0, 3.0);
        all_ops(&mut t, &format!("ball∖box lat {lat} az {az} h {h}"), &carved, &slab, cap(1.0, h), Point3::origin() + n, 0.2, Point3::origin(), 1.1);
    }
    report("sphere ∖ box", &t);
    assert!(t.wrong.is_empty(), "wrong bodies: {:#?}", t.wrong);
}

/// Two cut-ins on ONE half-band: the banded ball ∩ a cube turned 45°
/// about y, whose four side faces each cut a cap (two per half-band).
#[test]
fn probe_two_caps_one_face() {
    let mut t = Tally::default();
    let b = ball(1.0, 0.0);
    let bandbox = brick((-2.0, 2.0), (-0.6, 0.6), (-2.0, 2.0), Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
    let v_band = PI * (1.2 - 2.0 * 0.6f64.powi(3) / 3.0);
    let banded = and(&b, &bandbox, v_band);
    let a = 0.985;
    for turn in [45.0, 30.0] {
        let cube = brick(
            (-a, a),
            (-a, a),
            (-a, a),
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), f64::to_radians(turn)),
        );
        let caps = 4.0 * cap(1.0, 1.0 - a);
        all_ops(&mut t, &format!("banded vs cube turned {turn}"), &banded, &cube, v_band - caps, Point3::new(0.7, 0.0, 0.7), 0.3, Point3::origin(), 1.0);
    }
    // Two slabs' worth in sequence on one half-band: reuse the result.
    let s1 = slab_toward(Point3::origin(), dir(10.0, 90.0), 0.985, 1.0, 3.0);
    let s2 = slab_toward(Point3::origin(), dir(-15.0, 140.0), 0.985, 1.0, 3.0);
    let c = cap(1.0, 0.015);
    if let Some(r1) = check(&mut t, "banded∖s1", BooleanOp::Subtract, &banded, &s1, v_band - c, Point3::new(1.0, 0.17, 0.0), 0.3, Point3::origin(), 1.0) {
        all_ops(&mut t, "(banded∖s1) vs s2", &r1, &s2, c, Point3::origin() + dir(-15.0, 140.0), 0.3, Point3::origin(), 1.0);
        // s1 again against its own result: the cap hole's circle now on
        // the result's faces (coincident plane face).
        let s1b = slab_toward(Point3::origin(), dir(10.0, 90.0), 0.99, 1.0, 3.0);
        all_ops(&mut t, "(banded∖s1) vs deeper s1", &r1, &s1b, 0.0, Point3::origin() + dir(10.0, 90.0), 0.3, Point3::origin(), 1.0);
    }
    // The cap body itself: a sphere face bounded by one tilted circle.
    if let Some(capb) = check(&mut t, "banded∩s1", BooleanOp::Intersect, &banded, &s1, c, Point3::origin() + dir(10.0, 90.0), 0.2, Point3::origin(), 1.0) {
        let n = dir(10.0, 90.0);
        let deeper = slab_toward(Point3::origin(), n, 0.995, 1.0, 3.0);
        all_ops(&mut t, "cap vs deeper coaxial slab", &capb, &deeper, cap(1.0, 0.005), Point3::origin() + n, 0.2, Point3::origin() + n, 0.3);
        let n2 = dir(13.0, 92.0);
        let off = slab_toward(Point3::origin(), n2, 0.997, 1.0, 3.0);
        all_ops(&mut t, "cap vs off-axis small slab", &capb, &off, cap(1.0, 0.003), Point3::origin() + n2, 0.2, Point3::origin() + n, 0.3);
    }
    report("two caps on one face / reuse", &t);
    assert!(t.wrong.is_empty(), "wrong bodies: {:#?}", t.wrong);
}

fn union_solid(x: &Solid, y: &Solid, vol: f64) -> Solid {
    let (fx, fy) = (x.ind.clone(), y.ind.clone());
    Solid {
        body: run(BooleanOp::Union, &x.body, &y.body).expect("operand builds"),
        ind: std::rc::Rc::new(move |p| fx(p).max(fy(p))),
        vol,
    }
}

/// Two caps on one half-band in one op, on one or nearby meridians
/// (a V of two slabs), and the cube at turns that change which cut
/// comes first.
#[test]
fn probe_two_caps_one_meridian() {
    let mut t = Tally::default();
    let b = ball(1.0, 0.0);
    let bandbox = brick((-2.0, 2.0), (-0.6, 0.6), (-2.0, 2.0), Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
    let v_band = PI * (1.2 - 2.0 * 0.6f64.powi(3) / 3.0);
    let banded = and(&b, &bandbox, v_band);
    let c = cap(1.0, 0.015);
    for (az1, az2) in [(90.0, 90.0), (90.0, 92.0), (60.0, 120.0), (120.0, 60.0)] {
        let n1 = dir(20.0, az1);
        let n2 = dir(-20.0, az2);
        let s1 = slab_toward(Point3::origin(), n1, 0.985, 0.3, 0.3);
        let s2 = slab_toward(Point3::origin(), n2, 0.985, 0.25, 0.25);
        // The two small slabs must be disjoint for the oracle volume.
        let pair = union_solid(&s1, &s2, s1.vol + s2.vol);
        all_ops(&mut t, &format!("banded vs V az {az1}/{az2}"), &banded, &pair, 2.0 * c, Point3::origin() + n1, 0.4, Point3::origin(), 1.0);
    }
    let a = 0.985;
    for turn in [-45.0, 135.0, 160.0, 100.0] {
        let cube = brick(
            (-a, a),
            (-a, a),
            (-a, a),
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), f64::to_radians(turn)),
        );
        let caps = 4.0 * cap(1.0, 1.0 - a);
        all_ops(&mut t, &format!("banded vs cube turned {turn}"), &banded, &cube, v_band - caps, Point3::new(0.7, 0.0, 0.7), 0.3, Point3::origin(), 1.0);
    }
    report("two caps one meridian", &t);
    assert!(t.wrong.is_empty(), "wrong bodies: {:#?}", t.wrong);
}

/// One operand needing BOTH a re-chart (a closed cavity sphere the
/// slab's plane cuts inside its face) and a cut-in (the lens's top
/// face): `apply_recuts` rebuilds the operand before `apply_cut_ins`
/// reads the face key the scan recorded.
#[test]
fn probe_recut_and_cut_in_on_one_operand() {
    let mut t = Tally::default();
    let b1 = ball(1.0, 0.0);
    let b2 = ball(0.8, 1.4);
    let lens = and(&b1, &b2, lens_volume(1.0, 0.8, 1.4));
    for (tilt_axis, label) in [(Vec3::new(1.0, 0.0, 0.0), "about x"), (Vec3::new(0.0, 0.0, 1.0), "about z")] {
        let rot = Affine3::rotation_about_axis(Point3::origin(), tilt_axis, 20f64.to_radians());
        let n = rot.transform_vec(Vec3::new(0.0, 1.0, 0.0));
        let rc = 0.03;
        let cc = Point3::origin() + n * 0.96;
        let cav = {
            // A ball of radius rc at cc, from the revolve then moved.
            let b = ball(rc, 0.0);
            // Pole axis to x, seam plane spanned by x and a direction
            // perpendicular to n, so the plane's circle on it misses
            // its seams and poles (a closed group's escape).
            let rz = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), -PI / 2.0);
            let m = n.cross(Vec3::new(1.0, 0.0, 0.0));
            let theta = m.z.atan2(m.y);
            let rx = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), theta);
            let map = Affine3::translation(cc - Point3::origin()) * rx * rz;
            let moved = topo::transform_rigid(&b.body, &map, Tol::witness()).unwrap();
            Solid {
                body: finished("cavity", moved, Tol::witness()),
                ind: std::rc::Rc::new(move |p| rc - (p - cc).norm()),
                vol: 4.0 / 3.0 * PI * rc.powi(3),
            }
        };
        let holed = match catch_unwind(AssertUnwindSafe(|| run(BooleanOp::Subtract, &lens.body, &cav.body))) {
            Ok(Ok(b)) => b,
            other => {
                eprintln!("lens ∖ cavity {label} did not build: {:?}", other.as_ref().map(|r| r.as_ref().err().cloned()));
                continue;
            }
        };
        let (fl, fc) = (lens.ind.clone(), cav.ind.clone());
        let holed = Solid {
            body: holed,
            ind: std::rc::Rc::new(move |p| fl(p).min(-fc(p))),
            vol: lens.vol - cav.vol,
        };
        let slab = slab_toward(Point3::origin(), n, 0.985, 1.0, 3.0);
        let common = cap(1.0, 0.015) - cap(rc, 0.96 + rc - 0.985);
        all_ops(&mut t, &format!("lens∖cavity tilted {label}"), &holed, &slab, common, Point3::origin() + n, 0.2, Point3::new(0.0, 0.8, 0.0), 0.5);
    }
    report("recut + cut-in on one operand", &t);
    assert!(t.wrong.is_empty(), "wrong bodies: {:#?}", t.wrong);
}
