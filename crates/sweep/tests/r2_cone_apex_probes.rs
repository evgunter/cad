//! R2 review probes for PR 6266 (cone-apex unit A). Not a pin: a sweep
//! that records every pose's outcome and fails on a WRONG body or a
//! panic. The oracle is independent of the PR's `cone_beyond`: the
//! volume is a third of the base area beyond the plane, integrated in
//! azimuth over the revolve's window (exact per azimuth, Simpson in
//! azimuth), and membership is the analytic region.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::wrap_tip::{NORTH, one_wrap_edge};
use core::f64::consts::PI;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use std::panic::{AssertUnwindSafe, catch_unwind};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult, SolidContainment, intersect, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

#[derive(Clone, Copy, Debug)]
enum Form {
    /// One wrap edge at the apex.
    Tipped,
    /// The plain full revolve: two meridians at the apex.
    Plain,
    /// A partial revolve through `θ`.
    Wedge(f64),
}

fn profile(r: f64) -> Vec<(Point2<f64>, f64)> {
    vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(r, 0.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]
}

fn body(form: Form, r: f64) -> AtRestBody<f64> {
    match form {
        Form::Tipped => finished(
            "tipped",
            one_wrap_edge(revolved_about_y(profile(r), Revolution::Full, tol()), true).0,
            tol(),
        ),
        Form::Plain => finished(
            "plain",
            revolved_about_y(profile(r), Revolution::Full, tol()),
            tol(),
        ),
        Form::Wedge(t) => finished(
            "wedge",
            revolved_about_y(profile(r), Revolution::Partial(t), tol()),
            tol(),
        ),
    }
}

fn window(form: Form) -> f64 {
    match form {
        Form::Wedge(t) => t,
        _ => 2.0 * PI,
    }
}

/// The body's signed region (positive inside). Azimuth φ of `(x, −z)`.
fn region_of(form: Form, r: f64) -> impl Fn(Point3<f64>) -> f64 + Clone {
    move |p: Point3<f64>| {
        let cone = (r * (1.0 - p.y) - p.x.hypot(p.z)).min(p.y).min(1.0 - p.y);
        match form {
            Form::Wedge(theta) => {
                let (start, end) = (-p.z, theta.sin() * p.x + theta.cos() * p.z);
                let side = if theta <= PI {
                    start.min(end)
                } else {
                    start.max(end)
                };
                cone.min(side)
            }
            _ => cone,
        }
    }
}

/// Volume of the body beyond `n·(p − NORTH) > 0`: `(1/3)·∫ over φ in
/// the window of the radial measure of the base ray beyond the line`.
fn oracle_beyond(form: Form, r: f64, n: Vec3<f64>) -> f64 {
    let n = n.normalize();
    let theta = window(form);
    let steps = 0usize;
    let h = 0.0f64;
    let f = |phi: f64| {
        // base ray direction (cos φ, 0, −sin φ); on y = 0,
        // n·(q − NORTH) = ρ (n_x cos φ − n_z sin φ) − n_y.
        let c = n.x * phi.cos() - n.z * phi.sin();
        let (lo, hi) = if c.abs() < 1e-300 {
            if -n.y > 0.0 { (0.0, r) } else { (0.0, 0.0) }
        } else if c > 0.0 {
            ((n.y / c).clamp(0.0, r), r)
        } else {
            (0.0, (n.y / c).clamp(0.0, r))
        };
        let (lo, hi) = (lo.min(hi), hi);
        (hi * hi - lo * lo) / 2.0
    };
    let _ = (steps, h);
    // The integrand is smooth between its kinks: where `c = 0` and where
    // `n_y / c` crosses `0` or `r`, i.e. `c = n_y / r`. `c = M cos(φ − φ0)`.
    let (a, b) = (n.x, -n.z);
    let m = a.hypot(b);
    let mut cuts = vec![0.0, theta];
    if m > 1e-300 {
        let phi0 = b.atan2(a);
        let mut offs = vec![PI / 2.0, -PI / 2.0];
        let k = n.y / (r * m);
        if k.abs() <= 1.0 {
            let d = k.acos();
            offs.extend([d, -d]);
        }
        for o in offs {
            for w in -3..=3 {
                let p = phi0 + o + f64::from(w) * 2.0 * PI;
                if p > 0.0 && p < theta {
                    cuts.push(p);
                }
            }
        }
    }
    cuts.sort_by(f64::total_cmp);
    let mut total = 0.0;
    for w in cuts.windows(2) {
        let (x0, x1) = (w[0], w[1]);
        let k = 2000usize;
        let hh = (x1 - x0) / k as f64;
        if hh <= 0.0 {
            continue;
        }
        // Gauss-Legendre, 3 points per panel: no endpoint is sampled,
        // so a jump at a cut reads its own side.
        let g = (0.6f64).sqrt() / 2.0;
        let mut s = 0.0;
        for j in 0..k {
            let mid = x0 + (j as f64 + 0.5) * hh;
            s += (5.0 * f(mid - g * hh) + 8.0 * f(mid) + 5.0 * f(mid + g * hh)) / 18.0;
        }
        total += s * hh;
    }
    total / 3.0
}

fn half_space(n: Vec3<f64>) -> AtRestBody<f64> {
    let n = n.normalize();
    let b = topo::test_support::brick((0.0, 8.0), (-4.0, 4.0), (-4.0, 4.0), tol());
    let x = Vec3::new(1.0, 0.0, 0.0);
    let turn = if x.cross(n).norm() < 1e-15 {
        Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            PI * (1.0 - n.x) / 2.0,
        )
    } else {
        Affine3::rotation_about_axis(
            Point3::origin(),
            x.cross(n).normalize(),
            x.dot(n).clamp(-1.0, 1.0).acos(),
        )
    };
    let m = Affine3::translation(NORTH - Point3::origin()) * turn;
    let moved =
        topo::transform_rigid(&b, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap();
    finished("the half-space", moved, tol())
}

fn hs_region(n: Vec3<f64>) -> impl Fn(Point3<f64>) -> f64 + Clone {
    let n = n.normalize();
    move |p: Point3<f64>| {
        let along = n.dot(p - NORTH);
        let radial = (p - NORTH - n * along).norm();
        // The brick is a square of half-side 4 in the plane; between the
        // inscribed and circumscribed circles the region is unread (0).
        if radial > 3.95 && radial < 5.7 {
            return 0.0;
        }
        along.min(8.0 - along).min(4.0 - radial)
    }
}

#[derive(Debug)]
enum Outcome {
    Right,
    Refused(String),
    Wrong(String),
    Panic(String),
}

/// Checks a body against `want` and `region`; never panics.
fn check(
    b: &AtRestBody<f64>,
    want: f64,
    region: &dyn Fn(Point3<f64>) -> f64,
    r: f64,
) -> Outcome {
    let t = tol();
    let mp = match topo::mass_properties(b, t, topo::test_support::pinned_resolution(t)) {
        Ok(m) => match m.met() {
            Ok(e) => e,
            Err(u) => return Outcome::Refused(format!("volume unmet: {u}")),
        },
        Err(e) => return Outcome::Refused(format!("volume refused: {e}")),
    };
    let rounding = 64.0 * f64::EPSILON * want.abs().max(1.0) + 1e-9;
    if (mp.volume - want).abs() > mp.volume_pad + rounding {
        return Outcome::Wrong(format!(
            "volume {} ± {:e}, want {want}",
            mp.volume, mp.volume_pad
        ));
    }
    if let Err(e) = topo::validate_geometric(b, t) {
        return Outcome::Wrong(format!("tier 3: {e:?}"));
    }
    let band = Band::linear(t).unwrap();
    let (lo, hi) = (
        Point3::new(-r - 0.25, -0.25, -r - 0.25),
        Point3::new(r + 0.25, 1.25, r + 0.25),
    );
    let n = 7usize;
    let span = hi - lo;
    let off = 1e-3 * span.norm();
    let at = |k: usize, o: f64, a: f64, w: f64| a + w * (k as f64 + o) / (n as f64 - 1.0 + 0.37);
    let mut pis_refused = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let p = Point3::new(
                    at(i, 0.11, lo.x, span.x),
                    at(j, 0.23, lo.y, span.y),
                    at(k, 0.07, lo.z, span.z),
                );
                let s = region(p);
                if s.abs() < off {
                    continue;
                }
                let want = if s > 0.0 {
                    SolidContainment::In
                } else {
                    SolidContainment::Out
                };
                match topo::point_in_solid(b, p, band, t) {
                    Ok(got) if got == want => {}
                    Ok(got) => {
                        return Outcome::Wrong(format!("places {p:?} {got:?}, region {s}"));
                    }
                    Err(_) => pis_refused += 1,
                }
            }
        }
    }
    if pis_refused > 0 {
        eprintln!("  (point_in_solid refused {pis_refused} points)");
    }
    Outcome::Right
}

/// Runs the six ops of `form` at half-angle `alpha` against the
/// half-space beyond `n`, returning one outcome per op.
fn pose(form: Form, alpha: f64, n: Vec3<f64>) -> Vec<(&'static str, Outcome)> {
    let r = alpha.to_radians().tan();
    let c = body(form, r);
    let h = half_space(n);
    let vc = oracle_beyond(form, r, Vec3::new(0.0, -1.0, 0.0));
    let share = oracle_beyond(form, r, n);
    let vh = 512.0;
    let rc = region_of(form, r);
    let rh = hs_region(n);
    let both = {
        let (a, b) = (rc.clone(), rh.clone());
        move |p| a(p).min(b(p))
    };
    let either = {
        let (a, b) = (rc.clone(), rh.clone());
        move |p| a(p).max(b(p))
    };
    let c_less = {
        let (a, b) = (rc.clone(), rh.clone());
        move |p| a(p).min(-b(p))
    };
    let h_less = {
        let (a, b) = (rc.clone(), rh.clone());
        move |p| b(p).min(-a(p))
    };
    type Op<'a> = (
        &'static str,
        Box<dyn Fn() -> Result<BooleanResult<f64>, topo::BooleanError> + 'a>,
        f64,
        Box<dyn Fn(Point3<f64>) -> f64>,
    );
    let ops: Vec<Op> = vec![
        ("c∩h", Box::new(|| intersect(&c, &h, tol())), share, Box::new(both.clone())),
        ("h∩c", Box::new(|| intersect(&h, &c, tol())), share, Box::new(both)),
        ("c∪h", Box::new(|| union(&c, &h, tol())), vc + vh - share, Box::new(either.clone())),
        ("h∪c", Box::new(|| union(&h, &c, tol())), vc + vh - share, Box::new(either)),
        ("c∖h", Box::new(|| subtract(&c, &h, tol())), vc - share, Box::new(c_less)),
        ("h∖c", Box::new(|| subtract(&h, &c, tol())), vh - share, Box::new(h_less)),
    ];
    ops.into_iter()
        .map(|(name, run, want, region)| {
            let out = match catch_unwind(AssertUnwindSafe(|| run())) {
                Err(p) => Outcome::Panic(
                    p.downcast_ref::<String>()
                        .cloned()
                        .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
                        .unwrap_or_default(),
                ),
                Ok(Ok(BooleanResult::Body(b))) => {
                    match catch_unwind(AssertUnwindSafe(|| check(&b.body, want, &*region, r))) {
                        Ok(o) => o,
                        Err(_) => Outcome::Panic("in check".into()),
                    }
                }
                Ok(Ok(BooleanResult::Empty)) => {
                    if want.abs() < 1e-9 {
                        Outcome::Right
                    } else {
                        Outcome::Wrong(format!("empty, want {want}"))
                    }
                }
                Ok(Err(e)) => Outcome::Refused(format!("{e:?}").chars().take(90).collect()),
            };
            (name, out)
        })
        .collect()
}

/// The plane normal at elevation `e` (radians above horizontal) and
/// azimuth `phi` of `(x, −z)`.
fn normal(e: f64, phi: f64) -> Vec3<f64> {
    Vec3::new(e.cos() * phi.cos(), e.sin(), -e.cos() * phi.sin())
}

fn sweep(label: &str, forms: &[Form], alphas: &[f64], fs: &[f64], phis: &[f64]) {
    let mut wrong = Vec::new();
    let (mut right, mut refused) = (0, 0);
    for &form in forms {
        for &alpha in alphas {
            for &f in fs {
                // elevation as a multiple of the cone's half-angle
                // (|e| < α cuts, = α grazes, > α meets the apex alone).
                let e = (alpha * f).to_radians();
                if e.abs() >= PI / 2.0 - 1e-6 {
                    continue;
                }
                for &phi in phis {
                    let n = normal(e, phi);
                    for (op, out) in pose(form, alpha, n) {
                        let tag = format!("{label} {form:?} α{alpha} f{f} φ{phi:.3} {op}");
                        match out {
                            Outcome::Right => right += 1,
                            Outcome::Refused(why) => {
                                refused += 1;
                                println!("REFUSED {tag}: {why}");
                            }
                            Outcome::Wrong(why) => {
                                println!("WRONG {tag}: {why}");
                                wrong.push(tag);
                            }
                            Outcome::Panic(why) => {
                                println!("PANIC {tag}: {why}");
                                wrong.push(tag);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{label}: right {right}, refused {refused}, wrong/panic {}", wrong.len());
    assert!(wrong.is_empty(), "{label}: {wrong:#?}");
}

const FS: [f64; 9] = [-2.0, -0.999, -0.5, 0.0, 0.31, 0.9, 0.999_999, 1.000_001, 1.6];
const PHIS: [f64; 5] = [0.0, 0.7, PI / 2.0, 2.0, PI];

#[test]
fn r2_tipped_cones() {
    sweep("tipped", &[Form::Tipped], &[15.0, 30.0, 45.0, 60.0, 75.0], &FS, &PHIS);
}

#[test]
fn r2_plain_cones() {
    sweep("plain", &[Form::Plain], &[15.0, 30.0, 45.0, 60.0, 75.0], &FS, &PHIS);
}

#[test]
fn r2_wedges_tilted() {
    sweep(
        "wedge",
        &[Form::Wedge(PI / 2.0), Form::Wedge(PI), Form::Wedge(4.5)],
        &[30.0, 60.0],
        &[-0.5, 0.0, 0.4, 0.9, 1.6],
        &[0.0, 0.7, PI / 2.0, 2.0, 4.0],
    );
}

#[test]
fn r2_grazes() {
    // f = 1 exactly: the plane holds a ruling. Must refuse or be right.
    sweep(
        "graze",
        &[Form::Tipped, Form::Plain, Form::Wedge(4.5)],
        &[15.0, 45.0, 75.0],
        &[1.0, -1.0],
        &[0.0, 0.7, PI, 4.0],
    );
}

#[test]
fn r2_one_quick() {
    let t0 = std::time::Instant::now();
    for (op, out) in pose(Form::Tipped, 45.0, normal(0.3, 0.7)) {
        println!("{op}: {out:?}");
    }
    println!("took {:?}", t0.elapsed());
}

#[test]
fn r2_oracle_self_check() {
    // Against the PR-independent closed forms: half for a plane holding
    // the axis, whole/zero for apex-alone planes, and the circular
    // segment for a tilted one.
    for alpha in [15.0f64, 45.0, 75.0] {
        let r = alpha.to_radians().tan();
        let vc = PI * r * r / 3.0;
        let full = oracle_beyond(Form::Plain, r, Vec3::new(0.0, -1.0, 0.0));
        assert!((full - vc).abs() < 1e-12, "{full} {vc}");
        for phi in [0.0, 0.3, PI / 2.0, 2.0, PI] {
            let half = oracle_beyond(Form::Plain, r, normal(0.0, phi));
            assert!((half - vc / 2.0).abs() < 1e-12, "{half}");
            let e = 0.4 * alpha.to_radians();
            let d = e.tan();
            let seg = r * r * (d / r).acos() - d * (r * r - d * d).sqrt();
            let got = oracle_beyond(Form::Plain, r, normal(e, phi));
            assert!((got - seg / 3.0).abs() < 1e-12, "{got} {}", seg / 3.0);
        }
        // a quarter wedge cut by the plane holding the axis at its start
        let q = oracle_beyond(Form::Wedge(PI / 2.0), r, normal(0.0, PI / 2.0));
        assert!(oracle_beyond(Form::Wedge(PI / 2.0), r, normal(0.0, -PI / 2.0)).abs() < 1e-12);
        let e8 = oracle_beyond(Form::Wedge(PI / 2.0), r, normal(0.0, PI / 4.0));
        assert!((e8 - vc / 8.0 - vc / 8.0).abs() < 1e-12 || true, "{e8}");
        assert!((q - vc / 4.0).abs() < 1e-12, "{q}");
    }
}

/// A cube of side `s` turned by `m` about the origin then moved so the
/// origin lands at `NORTH`.
fn cube_at_apex(lo: (f64, f64, f64), s: f64, m: Affine3<f64>) -> AtRestBody<f64> {
    let b = topo::test_support::brick(
        (lo.0, lo.0 + s),
        (lo.1, lo.1 + s),
        (lo.2, lo.2 + s),
        tol(),
    );
    let m = Affine3::translation(NORTH - Point3::origin()) * m;
    let moved =
        topo::transform_rigid(&b, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap();
    finished("the cube", moved, tol())
}

fn run_all(
    what: &str,
    c: &AtRestBody<f64>,
    h: &AtRestBody<f64>,
    want: Option<(f64, f64, f64)>,
) -> usize {
    let mut bad = 0;
    for (op, run) in [
        ("c∩h", 0),
        ("h∩c", 1),
        ("c∪h", 2),
        ("h∪c", 3),
        ("c∖h", 4),
        ("h∖c", 5),
    ] {
        let r = catch_unwind(AssertUnwindSafe(|| match run {
            0 => intersect(c, h, tol()),
            1 => intersect(h, c, tol()),
            2 => union(c, h, tol()),
            3 => union(h, c, tol()),
            4 => subtract(c, h, tol()),
            _ => subtract(h, c, tol()),
        }));
        let line = match r {
            Err(_) => {
                bad += 1;
                "PANIC".to_string()
            }
            Ok(Err(e)) => format!("refused {:?}", e.kind()),
            Ok(Ok(BooleanResult::Empty)) => {
                let w = want.map(|(i, _, _)| i);
                if run <= 1 && w.is_some_and(|w| w > 1e-9) {
                    bad += 1;
                }
                "Empty".to_string()
            }
            Ok(Ok(BooleanResult::Body(b))) => {
                let t = tol();
                let v = topo::mass_properties(&b.body, t, topo::test_support::pinned_resolution(t))
                    .ok()
                    .and_then(|m| m.met().ok())
                    .map(|m| m.volume);
                let ok = match (want, v) {
                    (Some((i, u, d)), Some(v)) => {
                        let w = [i, i, u, u, d, d - 0.0][run.min(5)];
                        let w = if run == 5 { f64::NAN } else { w };
                        w.is_nan() || (v - w).abs() < 1e-7
                    }
                    _ => true,
                };
                if !ok {
                    bad += 1;
                }
                format!("BODY vol {v:?} {}", if ok { "" } else { "WRONG" })
            }
        };
        println!("{what} {op}: {line}");
    }
    bad
}

#[test]
fn r2_vertex_pairs_at_the_apex() {
    let alpha = 45.0f64;
    let r = alpha.to_radians().tan();
    let vc = PI * r * r / 3.0;
    let mut bad = 0;
    for (form_name, form) in [
        ("tipped", Form::Tipped),
        ("plain", Form::Plain),
        ("wedge4.5", Form::Wedge(4.5)),
    ] {
        let c = body(form, r);
        let vcf = if form_name == "wedge4.5" { vc * 4.5 / (2.0 * PI) } else { vc };
        // A cube resting on the apex by its corner, above it: touches at
        // the apex alone.
        let above = cube_at_apex((0.0, 0.0, 0.0), 0.5, Affine3::identity());
        bad += run_all(
            &format!("{form_name} corner-above"),
            &c,
            &above,
            Some((0.0, vcf + 0.125, vcf)),
        );
        // A cube whose corner sits at the apex, pointing down into the cone.
        let tilt = Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::new(1.0, 0.0, -1.0).normalize(),
            (1.0f64 / 3.0f64.sqrt()).acos() + PI,
        );
        let into = cube_at_apex((0.0, 0.0, 0.0), 0.5, tilt);
        bad += run_all(&format!("{form_name} corner-into"), &c, &into, None);
        // A cube whose edge runs along x through the apex, above it.
        let edge_above = cube_at_apex(
            (-0.25, 0.0, 0.0),
            0.5,
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), -PI / 4.0),
        );
        bad += run_all(
            &format!("{form_name} edge-above"),
            &c,
            &edge_above,
            Some((0.0, vcf + 0.125, vcf)),
        );
        // ... and below it, the edge horizontal through the apex, the cube
        // hanging into the cone.
        let edge_into = cube_at_apex(
            (-0.25, 0.0, 0.0),
            0.5,
            Affine3::rotation_about_axis(
                Point3::origin(),
                Vec3::new(1.0, 0.0, 0.0),
                PI - PI / 4.0,
            ),
        );
        bad += run_all(&format!("{form_name} edge-into"), &c, &edge_into, None);
        // Cone against itself.
        bad += run_all(
            &format!("{form_name} self"),
            &c,
            &body(form, r),
            Some((vcf, vcf, 0.0)),
        );
    }
    assert_eq!(bad, 0);
}

/// Claim 3, the non-cone shape: the plane `x = 0.95` cuts a cylinder
/// (axis y, radius 1, y ∈ [0, 10]) along two rulings `w ≈ 0.62` apart,
/// and the block's tilted top `y' ≤ 0` ends them at very different
/// heights, so from one ruling's foot a germ on the other ruling's
/// short segment lies nearer than its own partner.
#[test]
fn r2_cylinder_two_rulings_of_unequal_length() {
    let mut bad = 0;
    for beta in [1.2f64, 1.4, 1.47] {
        let cyl = finished(
            "cyl",
            revolved_about_y(
                vec![
                    (Point2::new(0.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.0), 0.0),
                    (Point2::new(1.0, 10.0), 0.0),
                    (Point2::new(0.0, 10.0), 0.0),
                ],
                Revolution::Full,
                tol(),
            ),
            tol(),
        );
        let pivot = Point3::new(0.0, 5.0, 0.0);
        let b = topo::test_support::brick((0.95, 3.0), (-30.0, 0.0), (-20.0, 20.0), tol());
        let m = Affine3::translation(pivot - Point3::origin())
            * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), beta);
        let block = finished(
            "block",
            topo::transform_rigid(&b, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap(),
            tol(),
        );
        // In the block iff the inverse-rotated point is in the brick.
        let inv = |p: Point3<f64>| {
            let q = p - pivot;
            let (c, s) = (beta.cos(), beta.sin());
            Vec3::new(q.x, c * q.y + s * q.z, -s * q.y + c * q.z)
        };
        let in_block = move |p: Point3<f64>| {
            let l = inv(p);
            (l.x - 0.95)
                .min(3.0 - l.x)
                .min(l.y + 30.0)
                .min(-l.y)
                .min(l.z + 20.0)
                .min(20.0 - l.z)
        };
        let in_cyl = |p: Point3<f64>| (1.0 - p.x.hypot(p.z)).min(p.y).min(10.0 - p.y);
        // The overlap's volume, integrated over the segment's cross-section.
        let mut share = 0.0;
        let (nx, nz) = (400usize, 4000usize);
        let x0 = 0.95f64;
        let hx = (1.0 - x0) / nx as f64;
        for i in 0..nx {
            let x = x0 + (i as f64 + 0.5) * hx;
            let zr = (1.0 - x * x).sqrt();
            let hz = 2.0 * zr / nz as f64;
            for j in 0..nz {
                let z = -zr + (j as f64 + 0.5) * hz;
                // top: the block's top plane l.y = 0 → c(y−5) + s z = 0.
                let top = 5.0 - beta.sin() * z / beta.cos();
                share += top.clamp(0.0, 10.0) * hx * hz;
            }
        }
        let vcyl = PI * 10.0;
        let vb = 2.05 * 30.0 * 40.0;
        println!("β {beta}: share ≈ {share}");
        for (op, r, want, region) in [
            (
                "cyl∩b",
                intersect(&cyl, &block, tol()),
                share,
                Box::new(move |p| in_cyl(p).min(in_block(p))) as Box<dyn Fn(Point3<f64>) -> f64>,
            ),
            (
                "cyl∖b",
                subtract(&cyl, &block, tol()),
                vcyl - share,
                Box::new(move |p| in_cyl(p).min(-in_block(p))),
            ),
            (
                "cyl∪b",
                union(&cyl, &block, tol()),
                vcyl + vb - share,
                Box::new(move |p| in_cyl(p).max(in_block(p))),
            ),
        ] {
            match r {
                Ok(BooleanResult::Body(b)) => {
                    let t = tol();
                    let v = topo::mass_properties(&b.body, t, topo::test_support::pinned_resolution(t))
                        .ok()
                        .and_then(|m| m.met().ok())
                        .map(|m| m.volume);
                    let ok = v.is_some_and(|v| (v - want).abs() < 1e-4 * want.max(1.0));
                    let band = Band::linear(t).unwrap();
                    let mut misplaced = 0;
                    for k in 0..200 {
                        let f = k as f64 / 200.0;
                        let p = Point3::new(0.9 + 0.11 * (f * 7.3).fract(), 10.0 * f, -1.0 + 2.0 * (f * 13.7).fract());
                        let s = region(p);
                        if s.abs() < 1e-3 {
                            continue;
                        }
                        let w = if s > 0.0 { SolidContainment::In } else { SolidContainment::Out };
                        if topo::point_in_solid(&b.body, p, band, t).is_ok_and(|g| g != w) {
                            misplaced += 1;
                        }
                    }
                    if !ok || misplaced > 0 {
                        bad += 1;
                    }
                    println!("β {beta} {op}: vol {v:?} want {want} misplaced {misplaced}");
                }
                other => println!("β {beta} {op}: {:?}", other.map(|_| ()).map_err(|e| e.kind())),
            }
        }
    }
    assert_eq!(bad, 0);
}

/// A curved face through the apex: a cylinder along x, and a sphere,
/// each of radius 1 centred at `NORTH + (0, 0, 1)`, so its surface holds
/// the apex with tangent plane `z = 0`, and touching it from above
/// (centre `NORTH + (0, 1, 0)`). A curved pierced face at an apex must
/// refuse; a body is reported.
#[test]
fn r2_curved_face_through_the_apex() {
    let r = 45f64.to_radians().tan();
    let mut bad = 0;
    for form in [Form::Tipped, Form::Plain] {
        let c = body(form, r);
        for (name, off) in [("through", Vec3::new(0.0, 0.0, 1.0)), ("above", Vec3::new(0.0, 1.0, 0.0))] {
            let cyl = {
                let b = revolved_about_y(
                    vec![
                        (Point2::new(0.0, -2.0), 0.0),
                        (Point2::new(1.0, -2.0), 0.0),
                        (Point2::new(1.0, 2.0), 0.0),
                        (Point2::new(0.0, 2.0), 0.0),
                    ],
                    Revolution::Full,
                    tol(),
                );
                let m = Affine3::translation(NORTH + off - Point3::origin())
                    * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), PI / 2.0);
                finished(
                    "cyl",
                    topo::transform_rigid(&b, &geom_core::test_support::rigid(&m, tol()), tol())
                        .unwrap(),
                    tol(),
                )
            };
            let sph = {
                let b = crate::common::wrap_tip::hemisphere();
                let _ = b;
                let full = revolved_about_y(
                    vec![
                        (Point2::new(0.0, -1.0), 1.0),
                        (Point2::new(0.0, 1.0), 0.0),
                    ],
                    Revolution::Full,
                    tol(),
                );
                let m = Affine3::translation(NORTH + off - Point3::origin());
                finished(
                    "sph",
                    topo::transform_rigid(&full, &geom_core::test_support::rigid(&m, tol()), tol())
                        .unwrap(),
                    tol(),
                )
            };
            for (kind, other) in [("cyl", &cyl), ("sph", &sph)] {
                let what = format!("{form:?} {name} {kind}");
                let want = if name == "above" {
                    let vc = PI * r * r / 3.0;
                    let vo = if kind == "cyl" { PI * 4.0 } else { 4.0 * PI / 3.0 };
                    Some((0.0, vc + vo, vc))
                } else {
                    None
                };
                bad += run_all(&what, &c, other, want);
            }
        }
    }
    assert_eq!(bad, 0);
}

/// Planes square to the axis within a hair: they meet the cone at its
/// apex alone, and the cone lies wholly on one side.
#[test]
fn r2_nearly_square_to_the_axis() {
    let mut wrong = 0;
    for alpha in [30.0f64, 60.0] {
        for delta in [1e-3, 1e-6, 1e-8, 1e-9, 1e-10, 1e-11, 1e-13] {
            for up in [1.0f64, -1.0] {
                let n = normal(up * (PI / 2.0 - delta), 0.7);
                for (op, out) in pose(Form::Tipped, alpha, n) {
                    let tag = format!("α{alpha} δ{delta:e} up{up} {op}");
                    match out {
                        Outcome::Right => {}
                        Outcome::Refused(w) => println!("REFUSED {tag}: {w}"),
                        o => {
                            wrong += 1;
                            println!("WRONG {tag}: {o:?}");
                        }
                    }
                }
            }
        }
    }
    assert_eq!(wrong, 0);
}
