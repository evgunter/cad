//! Reviewer e2e probes for PR 4128 (not kernel rows). Mounted into
//! `crates/sweep/tests/all.rs` by
//! `#[path = "../../../probes/e2e_pierce_tangent.rs"] mod review_e2e;`
//! Oracles are closed forms from dimensions and a point-membership
//! predicate written here; nothing is read back from the kernel but the
//! answer under test.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::{FRAC_1_SQRT_2, PI};

use geom_core::{Affine3, Band, Mat3, Point2, Point3, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, BooleanError, BooleanOp};

fn tol() -> Tol {
    Tol::witness()
}

fn moved(b: &Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, m, tol()).unwrap()
}

fn ball(r: f64, y: f64) -> Body<f64> {
    let b = sweep::test_support::revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        tol(),
    );
    moved(&b, &Affine3::translation(Vec3::new(0.0, y, 0.0)))
}

fn boolean(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let (a, b) = (
        &sweep::test_support::finished("A", a.clone(), tol()),
        &sweep::test_support::finished("B", b.clone(), tol()),
    );
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, tol()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, tol()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, tol()),
    }
}

fn built(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Option<Body<f64>> {
    boolean(op, a, b)
        .unwrap_or_else(|e| panic!("{op:?} refused: {e:?}"))
        .body()
        .map(|b| (*b.body).clone())
}

fn volume(label: &str, body: &Body<f64>) -> f64 {
    assert_eq!(topo::validate(body), Ok(()), "{label}: validate");
    assert_eq!(topo::validate_closed(body), Ok(()), "{label}: closed");
    assert_eq!(topo::validate_geometric(body, tol()), Ok(()), "{label}: geometric");
    let p = topo::mass_properties(body, tol()).unwrap_or_else(|e| panic!("{label}: {e:?}"));
    PAD.with(|c| c.set(p.volume_pad));
    p.volume
}

std::thread_local! {
    /// The last volume's quadrature pad: a cut rod's oblique cap is not
    /// closed-form, so the oracle comparison widens by it.
    static PAD: core::cell::Cell<f64> = const { core::cell::Cell::new(0.0) };
}

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-9 * want.abs().max(1e-9) + PAD.with(|c| c.get()),
        "{label}: volume {got} against {want}"
    );
}

/// Every op, both orders, disjoint operands.
fn apart(label: &str, a: &Body<f64>, b: &Body<f64>, (va, vb): (f64, f64)) {
    for (op, x, y, want) in [
        (BooleanOp::Union, a, b, va + vb),
        (BooleanOp::Union, b, a, va + vb),
        (BooleanOp::Intersect, a, b, 0.0),
        (BooleanOp::Intersect, b, a, 0.0),
        (BooleanOp::Subtract, a, b, va),
        (BooleanOp::Subtract, b, a, vb),
    ] {
        let name = format!("{label}: {op:?}");
        match built(op, x, y) {
            Some(out) => close(&name, volume(&name, &out), want),
            None => assert!(want == 0.0, "{name}: empty against {want}"),
        }
    }
}

fn refuses(label: &str, a: &Body<f64>, b: &Body<f64>) {
    for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
        for (x, y) in [(a, b), (b, a)] {
            match boolean(op, x, y) {
                Err(BooleanError::CurvedPierceUnsupported { .. }) => {}
                other => panic!("{label} {op:?}: expected the pierce door, got {:?}", other.map(|_| "built")),
            }
        }
    }
}

/// Prints each op's outcome (both orders) without asserting.
fn report(label: &str, a: &Body<f64>, b: &Body<f64>) {
    for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
        for (o, x, y) in [("A·B", a, b), ("B·A", b, a)] {
            let r = boolean(op, x, y);
            let what = match &r {
                Ok(res) => format!("built, volume {:?}", res.body().map(|b| topo::mass_properties(&b.body, tol()).map(|p| p.volume))),
                Err(BooleanError::CurvedPierceUnsupported { .. }) => "CurvedPierceUnsupported".into(),
                Err(BooleanError::FallbackExtentUnsupported { .. }) => "FallbackExtentUnsupported".into(),
                Err(e) => format!("{e:?}").chars().take(80).collect(),
            };
            println!("REPORT {label} {op:?} {o}: {what}");
        }
    }
}

const R1: f64 = 1.0;
const R2: f64 = 0.8;
const D: f64 = 1.4;
const RIM: f64 = (D * D + R1 * R1 - R2 * R2) / (2.0 * D);

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

fn lens_k(k: f64) -> Body<f64> {
    built(BooleanOp::Intersect, &ball(R1 * k, 0.0), &ball(R2 * k, D * k)).unwrap()
}

fn lens_volume_k(k: f64) -> f64 {
    k.powi(3) * (cap(R1, R1 - RIM) + cap(R2, R2 - (D - RIM)))
}

fn edge_brick(h: f64, w: f64, at: Vec3<f64>, along: Vec3<f64>, n: Vec3<f64>, m: Vec3<f64>) -> Body<f64> {
    let (s1, s2) = ((n + m) * FRAC_1_SQRT_2, (n - m) * FRAC_1_SQRT_2);
    let (s1, s2) = if along.cross(s1).dot(s2) > 0.0 { (s1, s2) } else { (s2, s1) };
    let brick = sweep::test_support::brick((-h, h), (0.0, w), (0.0, w), tol());
    moved(&brick, &Affine3::from_parts(Mat3::from_cols(along, s1, s2), at))
}

fn band_reach(r: f64) -> f64 {
    let band = Band::linear(tol()).unwrap();
    (2.0 * r * (band.zero() + band.escalate())).sqrt()
}

/// A: the item's pose and the turned brick, at ×1e-3, ×1, ×1e3.
#[test]
fn scaled_item_poses_build() {
    for k in [1e-3, 1.0, 1e3] {
        let lens = lens_k(k);
        let brick = sweep::test_support::brick((-k, k), (-2.0 * k, -k), (0.0, k), tol());
        apart(&format!("item ×{k}"), &lens, &brick, (lens_volume_k(k), 2.0 * k.powi(3)));
        let n = Vec3::new(0.0, 0.0, 1.0);
        let turned = edge_brick(0.5 * k, k, n * k, Vec3::new(1.0, 0.0, 0.0), n, Vec3::new(0.0, -1.0, 0.0));
        apart(&format!("equator ×{k}"), &lens, &turned, (lens_volume_k(k), k.powi(3)));
    }
}

fn brick_at_polar(k: f64, h: f64, theta: f64) -> Body<f64> {
    let n = Vec3::new(0.0, theta.cos(), theta.sin());
    let m = Vec3::new(0.0, -theta.sin(), theta.cos());
    edge_brick(h * k, k, n * k, Vec3::new(1.0, 0.0, 0.0), n, m)
}

/// A': near-rim poses at scale: the band is absolute, so the rim's ball
/// in metres is the same at every scale; the angle is not.
#[test]
fn scaled_near_rim_poses() {
    for k in [1e-3, 1.0, 1e3] {
        let lens = lens_k(k);
        let rim = (RIM / R1).acos();
        let ball = band_reach(k) / k; // radians
        let out = brick_at_polar(k, 0.25, rim + 3.0 * ball);
        let res: Vec<bool> = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract]
            .iter()
            .map(|&op| boolean(op, &lens, &out).is_ok())
            .collect();
        println!("×{k} three balls outside: built {res:?}");
        refuses(&format!("inside ×{k}"), &lens, &brick_at_polar(k, 0.25, rim - 3.0 * ball));
        refuses(&format!("rim ×{k}"), &lens, &brick_at_polar(k, 0.25, rim + 0.3 * ball));
    }
}

/// B: rotated poses — both operands carried by the same rigid motion.
#[test]
fn rotated_poses_build() {
    let rim = (RIM / R1).acos();
    for (i, (axis, angle)) in [
        (Vec3::new(1.0, 2.0, 3.0), 0.7),
        (Vec3::new(-2.0, 0.5, 1.0), 2.1),
        (Vec3::new(0.3, -1.0, 0.2), -1.3),
    ]
    .into_iter()
    .enumerate()
    {
        let m = Affine3::rotation_about_axis(Point3::new(0.3, -0.2, 0.1), axis / axis.norm(), angle);
        let lens = moved(&lens_k(1.0), &m);
        for (j, th) in [PI, PI / 2.0, rim + 3.0 * band_reach(1.0)].into_iter().enumerate() {
            let b = moved(&brick_at_polar(1.0, 0.25, th), &m);
            apart(&format!("rot {i} pose {j}"), &lens, &b, (lens_volume_k(1.0), 0.5));
        }
    }
}

/// C: results reused as operands, and membership sampled.
#[test]
fn results_reused_and_sampled() {
    let lens = lens_k(1.0);
    let brick = sweep::test_support::brick((-1.0, 1.0), (-2.0, -1.0), (0.0, 1.0), tol());
    let u = built(BooleanOp::Union, &lens, &brick).unwrap();
    let (vl, vb) = (lens_volume_k(1.0), 2.0);
    close("U", volume("U", &u), vl + vb);
    // U against a second brick touching the sphere's carrier at the equator.
    let n = Vec3::new(0.0, 0.0, 1.0);
    let turned = edge_brick(0.5, 1.0, n, Vec3::new(1.0, 0.0, 0.0), n, Vec3::new(0.0, -1.0, 0.0));
    apart("U, turned", &u, &turned, (vl + vb, 1.0));
    // (U − lens refuses UndeclaredCoincidence on the shared sphere: the
    // D10 coincidence door, not this PR's; not exercised.)
    let big = sweep::test_support::brick((-0.5, 0.5), (-1.5, 1.5), (0.2, 0.6), tol());
    let i = built(BooleanOp::Intersect, &u, &big).expect("U ∩ slab");
    let vi = volume("U ∩ slab", &i);
    // Oracle by Monte Carlo of the membership predicate below.
    let inside = |p: Point3<f64>| -> bool {
        let lens = p.x * p.x + p.y * p.y + p.z * p.z < 1.0 && p.x * p.x + (p.y - D).powi(2) + p.z * p.z < R2 * R2;
        let brk = p.x.abs() < 1.0 && p.y > -2.0 && p.y < -1.0 && p.z > 0.0 && p.z < 1.0;
        let slab = p.x.abs() < 0.5 && p.y.abs() < 1.5 && p.z > 0.2 && p.z < 0.6;
        (lens || brk) && slab
    };
    let mut s: u64 = 0x1234_5678_9abc_def1;
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    let (mut hits, mut wrong, n_mc) = (0usize, 0usize, 4000usize);
    let band = Band::linear(tol()).unwrap();
    for _ in 0..n_mc {
        let p = Point3::new(-0.5 + rnd(), -1.5 + 3.0 * rnd(), 0.2 + 0.4 * rnd());
        let want = inside(p);
        hits += want as usize;
        match topo::point_in_solid(&i, p, band, tol()) {
            Ok(topo::SolidContainment::In) if want => {}
            Ok(topo::SolidContainment::Out) if !want => {}
            Ok(topo::SolidContainment::OnBoundary) => {}
            other => {
                wrong += 1;
                println!("point {p:?}: want in={want}, got {other:?}");
            }
        }
    }
    let mc = hits as f64 / n_mc as f64 * (1.0 * 3.0 * 0.4);
    println!("U ∩ slab: kernel {vi}, Monte Carlo {mc}, point_in_solid disagreements {wrong}");
    assert_eq!(wrong, 0);
    assert!((vi - mc).abs() < 0.03, "volume {vi} vs MC {mc}");
}

fn donut(sweep_angle: f64, big: f64, small: f64) -> Body<f64> {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(vec![(Point2::new(big, -small), 1.0), (Point2::new(big, small), 1.0)])],
    )
    .validate(tol())
    .unwrap();
    revolve(
        &profile,
        RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) },
        Revolution::Partial(sweep_angle),
        tol(),
    )
    .unwrap()
    .body
}

/// A coin (axis along its local z) of radius `rho`, thickness `h`,
/// carried so its rim circle at z = 0 passes through `p`, lying in the
/// plane of `n` and `y`, centred `p + n·rho`, extending along `n × y`.
fn coin_touching(p: Vec3<f64>, n: Vec3<f64>, rho: f64, h: f64) -> Body<f64> {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![profile::circle(Point2::new(0.0, 0.0), rho, tol()).unwrap().into()],
    )
    .validate(tol())
    .unwrap();
    let rod = extrude(&profile, Extrusion::Distance { depth: h, side: ExtrudeSide::Along }, tol())
        .unwrap()
        .body;
    let y = Vec3::new(0.0, 1.0, 0.0);
    moved(&rod, &Affine3::from_parts(Mat3::from_cols(n, y, n.cross(y)), p + n * rho))
}

/// D: circle × torus — a coin's rim circle tangent to the 270° donut's
/// outer equator in its mouth (builds), and at the solid azimuth 135°
/// (the touch is on the face: must refuse).
#[test]
fn circle_torus_rows() {
    let (big, small) = (2.0, 0.5);
    let d = donut(1.5 * PI, big, small);
    let vd = 0.75 * 2.0 * PI * PI * big * small * small;
    let c = FRAC_1_SQRT_2;
    let n = Vec3::new(c, 0.0, c);
    let coin = coin_touching(n * (big + small), n, 0.2, 0.3);
    report("donut, coin in mouth", &d, &coin);
    let n2 = Vec3::new(-c, 0.0, -c);
    refuses("donut, coin on face", &d, &coin_touching(n2 * (big + small), n2, 0.2, 0.3));
}

/// E: ellipse × torus — an obliquely cut rod whose cut ellipse is the
/// rod's unique support point in the direction of the torus.
#[test]
fn ellipse_torus_rows() {
    let (big, small) = (2.0, 0.5);
    let d = donut(1.5 * PI, big, small);
    let vd = 0.75 * 2.0 * PI * PI * big * small * small;
    let (rho, s, c0, len) = (0.2, 0.5f64, 0.5, 1.0);
    // Rod z ∈ [−len, 2], radius rho; cut by the half-space z ≤ s·x + c0.
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![profile::circle(Point2::new(0.0, 0.0), rho, tol()).unwrap().into()],
    )
    .validate(tol())
    .unwrap();
    let rod = extrude(&profile, Extrusion::Distance { depth: 2.0 + len, side: ExtrudeSide::Along }, tol())
        .unwrap()
        .body;
    let rod = moved(&rod, &Affine3::translation(Vec3::new(0.0, 0.0, -len)));
    let slab = sweep::test_support::brick((-5.0, 5.0), (-5.0, 5.0), (-10.0, 0.0), tol());
    let slab = moved(&slab, &Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), -s.atan()));
    let slab = moved(&slab, &Affine3::translation(Vec3::new(0.0, 0.0, c0)));
    let cut = built(BooleanOp::Intersect, &rod, &slab).unwrap();
    let vcut = PI * rho * rho * (c0 + len);
    close("cut rod", volume("cut rod", &cut), vcut);
    // Support direction at the ellipse point (rho, 0, s·rho + c0).
    let top = Vec3::new(-s, 0.0, 1.0);
    let u = top / top.norm() + Vec3::new(1.0, 0.0, 0.0);
    let u = u / u.norm();
    let e = Point3::new(rho, 0.0, s * rho + c0);
    for (label, n, ok) in [
        ("mouth", Vec3::new(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2), true),
        ("face", Vec3::new(-FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2), false),
    ] {
        // Carry u → −n, local y → global y, the touch point e → P.
        let w = u.cross(Vec3::new(0.0, 1.0, 0.0));
        let to = -n;
        let to_w = to.cross(Vec3::new(0.0, 1.0, 0.0));
        let from = Mat3::from_cols(u, Vec3::new(0.0, 1.0, 0.0), w);
        let onto = Mat3::from_cols(to, Vec3::new(0.0, 1.0, 0.0), to_w);
        let rot = onto * from.transpose();
        let p = n * (big + small);
        let ev = Vec3::new(e.x, e.y, e.z);
        let m = Affine3::from_parts(rot, p - rot * ev);
        let placed = moved(&cut, &m);
        if ok {
            let _ = (vd, vcut);
            report(&format!("donut, ellipse in {label}"), &d, &placed);
        } else {
            refuses(&format!("donut, ellipse on {label}"), &d, &placed);
        }
    }
}

/// F: the torus's INNER equator, in the mouth: a brick edge along the
/// axis direction tangent to the tube from the hole, where the carrier's
/// azimuthal curvature has the other sign.
#[test]
fn inner_equator_in_the_mouth_builds() {
    let (big, small) = (2.0, 0.5);
    let d = donut(1.5 * PI, big, small);
    let vd = 0.75 * 2.0 * PI * PI * big * small * small;
    let c = FRAC_1_SQRT_2;
    let n = Vec3::new(c, 0.0, c);
    let t = Vec3::new(-c, 0.0, c);
    let b = edge_brick(0.25, 0.3, n * (big - small), Vec3::new(0.0, 1.0, 0.0), -n, t);
    apart("donut, inner equator", &d, &b, (vd, 0.5 * 0.09));
}

/// G: two touches on one span, both off the face, the span's middle on
/// the torus axis: a quarter donut and a line over its top through the
/// axis, in the gap.
#[test]
fn two_touches_over_the_axis_build() {
    let (big, small) = (2.0, 0.5);
    let d = donut(0.5 * PI, big, small);
    let vd = 0.25 * 2.0 * PI * PI * big * small * small;
    let c = FRAC_1_SQRT_2;
    let along = Vec3::new(c, 0.0, c);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let b = edge_brick(3.0, 0.5, up * small, along, up, along.cross(up));
    apart("quarter donut, line over the top", &d, &b, (vd, 6.0 * 0.25));
}
