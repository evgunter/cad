//! Review probes for PR 5956 (a plane through a cone's apex), R1.
//!
//! Each probe splits a solid of revolution by planes through a cone apex
//! vertex and checks every landed side against a numerical oracle (the
//! side's volume integrated over the revolved profile, the azimuth
//! measure of each profile point on that side read in closed form) and
//! tier 3′. A refusal is recorded with its variant name; a wrong volume
//! or an invariant failure is a finding.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use topo::ContactRecords;
use topo::splitting::split;

/// A solid of revolution about `y`: its profile `(ρ, y)` and the window
/// it is revolved through (`TAU` for a full revolve).
struct Solid {
    name: String,
    profile: Vec<(f64, f64)>,
    window: f64,
    /// For a pure cone through whose apex every plane passes: the apex
    /// height, the base height and radius (the exact `h·A/3` oracle).
    exact: Option<(f64, f64, f64)>,
}

impl Solid {
    fn body(&self, tol: Tol) -> topo::AtRestBody<f64> {
        let lp = bulge_loop(
            self.profile
                .iter()
                .map(|&(x, y)| (Point2::new(x, y), 0.0))
                .collect(),
        );
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .unwrap();
        let rev = if self.window >= TAU {
            sweep::Revolution::Full
        } else {
            sweep::Revolution::Partial(self.window)
        };
        let body = sweep::revolve(
            &vp,
            sweep::RevolveAxis {
                origin: Point2::new(0.0, 0.0),
                dir: Vec2::new(0.0, 1.0),
            },
            rev,
            tol,
        )
        .unwrap()
        .body;
        sweep::test_support::finished(&self.name, body, tol)
    }

    /// The point at profile `(ρ, y)` revolved by `φ` (right hand about +y).
    fn at(rho: f64, y: f64, phi: f64) -> Vec3<f64> {
        Vec3::new(rho * phi.cos(), y, -rho * phi.sin())
    }

    /// Volume of the solid's part with `n·(p − o) > 0`.
    fn side_volume(&self, o: Point3<f64>, n: Vec3<f64>) -> f64 {
        if let Some((ya, yb, r)) = self.exact
            && (o.y - ya).abs() < 1e-15
            && o.x == 0.0
            && o.z == 0.0
        {
            return (ya - yb).abs() / 3.0
                * sector_area(r, self.window.min(TAU), n, n.y * (ya - yb));
        }
        let o = o - Point3::new(0.0, 0.0, 0.0);
        let h = n.x.hypot(n.z);
        let beta = (-n.z).atan2(n.x);
        let w = self.window.min(TAU);
        // nx cos φ − nz sin φ = h cos(φ − β)
        let measure = |rho: f64, y: f64| -> f64 {
            let b = n.dot(o) - n.y * y;
            let lhs = rho * h;
            if lhs <= 0.0 {
                return if 0.0 > b { w } else { 0.0 };
            }
            let c = b / lhs;
            if c >= 1.0 {
                return 0.0;
            }
            if c <= -1.0 {
                return w;
            }
            let half = c.acos();
            let mut total = 0.0;
            for k in -2..=2 {
                let centre = beta + TAU * f64::from(k);
                let (lo, hi) = ((centre - half).max(0.0), (centre + half).min(w));
                if hi > lo {
                    total += hi - lo;
                }
            }
            total
        };
        let f = |rho: f64, y: f64| rho * measure(rho, y);
        let p = &self.profile;
        let mut vol = 0.0;
        let n_sub = 160;
        for i in 1..p.len() - 1 {
            let (a, b, c) = (p[0], p[i], p[i + 1]);
            vol += tri_integral(a, b, c, n_sub, &f);
        }
        vol.abs()
    }

    fn whole(&self) -> f64 {
        if let Some((ya, yb, r)) = self.exact {
            return (ya - yb).abs() / 3.0 * r * r * self.window.min(TAU) / 2.0;
        }
        self.side_volume(Point3::new(0.0, -1e9, 0.0), Vec3::new(0.0, 1.0, 0.0))
    }
}

/// Area of the disc sector `ρ ≤ r, φ ∈ [0, w]` (points `(ρ cos φ, −ρ sin φ)`
/// in `(x, z)`) where `ρ (n_x cos φ − n_z sin φ) > b`, integrated in `φ`
/// between every kink with Gauss–Legendre.
fn sector_area(r: f64, w: f64, n: Vec3<f64>, b: f64) -> f64 {
    let h = n.x.hypot(n.z);
    let beta = (-n.z).atan2(n.x);
    let mut cuts = vec![0.0, w];
    let mut push = |a: f64| {
        for k in -3..=3 {
            let x = a + TAU * f64::from(k);
            if x > 0.0 && x < w {
                cuts.push(x);
            }
        }
    };
    push(beta + FRAC_PI_2);
    push(beta - FRAC_PI_2);
    if h > 0.0 && (b / (r * h)).abs() <= 1.0 {
        let a = (b / (r * h)).acos();
        push(beta + a);
        push(beta - a);
    }
    cuts.sort_by(f64::total_cmp);
    // Each piece in closed form: g is 0, r²/2, (r² − b²/c²)/2 or b²/(2c²),
    // and ∫ dφ / cos²(φ − β) = tan(φ − β).
    let k = if h > 0.0 { b * b / (2.0 * h * h) } else { 0.0 };
    let tan = |phi: f64| (phi - beta).tan();
    let mut total = 0.0;
    for pair in cuts.windows(2) {
        let (lo, hi) = (pair[0], pair[1]);
        if hi <= lo {
            continue;
        }
        let m = 0.5 * (lo + hi);
        let c = h * (m - beta).cos();
        let full = r * r / 2.0 * (hi - lo);
        let t = || k * (tan(hi) - tan(lo));
        total += if c > 0.0 {
            if b <= 0.0 {
                full
            } else if b / c >= r {
                0.0
            } else {
                full - t()
            }
        } else if c < 0.0 {
            if b >= 0.0 {
                0.0
            } else if b / c >= r {
                full
            } else {
                t()
            }
        } else if b < 0.0 {
            full
        } else {
            0.0
        };
    }
    total
}

/// Signed integral of `f` over triangle `abc`, subdivided `n`×`n`, a
/// 7-point degree-5 rule on each piece.
fn tri_integral(
    a: (f64, f64),
    b: (f64, f64),
    c: (f64, f64),
    n: usize,
    f: &impl Fn(f64, f64) -> f64,
) -> f64 {
    let area = 0.5 * ((b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1));
    let (u, v) = ((b.0 - a.0, b.1 - a.1), (c.0 - a.0, c.1 - a.1));
    let map = |s: f64, t: f64| (a.0 + u.0 * s + v.0 * t, a.1 + u.1 * s + v.1 * t);
    let (a1, b1) = (0.059_715_871_789_77, 0.470_142_064_105_115);
    let (a2, b2) = (0.797_426_985_353_087, 0.101_286_507_323_456);
    let w0 = 0.225;
    let w1 = 0.132_394_152_788_506;
    let w2 = 0.125_939_180_544_827;
    let rule = |p: [(f64, f64); 3]| {
        let bary = |l0: f64, l1: f64, l2: f64| {
            let s = l0 * p[0].0 + l1 * p[1].0 + l2 * p[2].0;
            let t = l0 * p[0].1 + l1 * p[1].1 + l2 * p[2].1;
            let (x, y) = map(s, t);
            f(x, y)
        };
        w0 * bary(1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0)
            + w1 * (bary(a1, b1, b1) + bary(b1, a1, b1) + bary(b1, b1, a1))
            + w2 * (bary(a2, b2, b2) + bary(b2, a2, b2) + bary(b2, b2, a2))
    };
    let h = 1.0 / n as f64;
    let mut sum = 0.0;
    for i in 0..n {
        for j in 0..n - i {
            let (s, t) = (i as f64 * h, j as f64 * h);
            sum += rule([(s, t), (s + h, t), (s, t + h)]);
            if i + j + 1 < n {
                sum += rule([(s + h, t), (s + h, t + h), (s, t + h)]);
            }
        }
    }
    // Each piece's area is (h²/2) of the unit triangle; the map scales by 2·area.
    sum * (h * h / 2.0) * 2.0 * area
}

/// The planes through `apex` posed against a cone of half-angle `alpha`
/// whose rulings leave the apex along `-y·nappe` (apex up: `nappe = 1`).
fn apex_planes(alpha: f64, nappe: f64, window: f64) -> Vec<(String, Vec3<f64>)> {
    let w = window.min(TAU);
    let ruling = |phi: f64| (Solid::at(alpha.sin(), -nappe * alpha.cos(), phi)).normalize();
    let mut out = Vec::new();
    let graze = FRAC_PI_2 - alpha;
    for &beta in &[
        0.0,
        1e-3,
        0.5 * graze,
        graze - 1e-3,
        graze,
        graze + 1e-3,
        graze + 0.05,
        0.5 * (graze + FRAC_PI_2),
        FRAC_PI_2 - 1e-4,
        FRAC_PI_2,
        FRAC_PI_2 + 0.3,
        PI - graze,
    ] {
        for &gamma in &[0.0, 0.3, 1.0, 2.0, PI, 3.7, 5.0, 0.5 * w, w, w - 0.2] {
            let n = Vec3::new(
                beta.sin() * gamma.cos(),
                beta.cos(),
                -beta.sin() * gamma.sin(),
            );
            for s in [1.0, -1.0] {
                out.push((format!("β {beta:.4} γ {gamma:.3} s {s}"), n * s));
            }
        }
    }
    // Planes holding a window-end ruling (or the seam), turned about it.
    for &phi in &[0.0, w] {
        let e = ruling(phi);
        let round = Vec3::new(-phi.sin(), 0.0, -phi.cos());
        let outward = round.cross(e).normalize();
        for &t in &[0.0, 1e-4, 0.05, 0.7, FRAC_PI_2, 2.5, -0.05, -0.7] {
            let n = outward * f64::cos(t) + e.cross(outward) * f64::sin(t);
            for s in [1.0, -1.0] {
                out.push((format!("holds ruling φ {phi:.3}, tilt {t}, s {s}"), n * s));
            }
        }
    }
    out
}

#[derive(Default)]
struct Tally {
    landed: usize,
    refused: std::collections::BTreeMap<String, usize>,
    wrong: Vec<String>,
}

fn variant(e: &topo::splitting::SplitError) -> String {
    let s = format!("{e:?}");
    let cut = s
        .char_indices()
        .filter(|&(_, c)| c == '{' || c == '(')
        .nth(1)
        .map_or(s.len(), |(i, _)| i);
    let mut v = s[..cut].to_string();
    for key in ["predicate: \"", "Escalated(MarginDiag"] {
        if let Some(i) = s.find(key) {
            let rest = &s[i + key.len()..];
            let end = rest.find('"').unwrap_or(rest.len().min(40));
            v.push_str(&format!(" [{}]", &rest[..end.min(60)]));
        }
    }
    v
}

fn run(solid: &Solid, apex: Point3<f64>, planes: &[(String, Vec3<f64>)]) -> Tally {
    let tol = Tol::witness();
    let eps = tol.eps();
    let body = solid.body(tol);
    let whole = solid.whole();
    let measured_whole = topo::test_support::pinned_volume("whole", &body, tol).volume;
    assert!(
        (measured_whole - whole).abs() < 1e-6 * whole.max(1.0),
        "{}: oracle {whole} vs measured {measured_whole}",
        solid.name
    );
    let slack = if solid.exact.is_some() {
        2048.0 * eps * whole.max(1.0)
    } else {
        (1e-5 * whole).max(4096.0 * eps * whole)
    };
    let mut tally = Tally::default();
    for (label, n) in planes {
        let what = format!("{}: {label}", solid.name);
        let plane = topo::test_support::split_plane(apex, *n, tol);
        let result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            split(&body, &plane, tol)
        })) {
            Err(_) => {
                tally.wrong.push(format!("{what}: PANIC in split"));
                continue;
            }
            Ok(Ok(r)) => r,
            Ok(Err(e)) => {
                *tally.refused.entry(variant(&e)).or_default() += 1;
                continue;
            }
        };
        let above = solid.side_volume(apex, *n);
        let wants = [above, whole - above];
        let mut ok = true;
        for ((side, part), want) in [("above", &result.above), ("below", &result.below)]
            .into_iter()
            .zip(wants)
        {
            let got = match part.body() {
                None => 0.0,
                Some(b) => {
                    if let Err(e) =
                        topo::validate_pseudomanifold(b, &ContactRecords::default(), tol)
                    {
                        tally.wrong.push(format!("{what} {side}: tier 3′ {e:?}"));
                        ok = false;
                    }
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        topo::test_support::pinned_volume(&what, b, tol).volume
                    })) {
                        Ok(v) => v,
                        Err(_) => {
                            tally.wrong.push(format!("{what} {side}: volume refused"));
                            ok = false;
                            continue;
                        }
                    }
                }
            };
            let other = whole - want;
            let landed_whole = (got - whole).abs() <= slack && want.min(other) <= slack;
            let empty_ok = got == 0.0 && want <= slack;
            if !((got - want).abs() <= slack || landed_whole || empty_ok) {
                tally
                    .wrong
                    .push(format!("{what} {side}: volume {got}, oracle {want}"));
                ok = false;
            }
        }
        if ok {
            tally.landed += 1;
        }
    }
    tally
}

fn report(name: &str, t: &Tally) {
    eprintln!(
        "PROBE {name} ε {:e}: landed {}, refused {:?}, WRONG {}",
        Tol::witness().eps(),
        t.landed,
        t.refused,
        t.wrong.len()
    );
    for w in t.wrong.iter().take(40) {
        eprintln!("  WRONG {w}");
    }
}

fn cone_solid(deg: f64, apex_up: bool, window: f64) -> (Solid, Point3<f64>, f64) {
    let r = deg.to_radians().tan();
    let (profile, apex) = if apex_up {
        (
            vec![(0.0, 0.0), (r, 0.0), (0.0, 1.0)],
            Point3::new(0.0, 1.0, 0.0),
        )
    } else {
        (
            vec![(0.0, 0.0), (r, 1.0), (0.0, 1.0)],
            Point3::new(0.0, 0.0, 0.0),
        )
    };
    (
        Solid {
            name: format!(
                "cone {deg}° {} window {window:.3}",
                if apex_up { "up" } else { "down" }
            ),
            profile,
            window,
            exact: Some(if apex_up {
                (1.0, 0.0, r)
            } else {
                (0.0, 1.0, r)
            }),
        },
        apex,
        if apex_up { 1.0 } else { -1.0 },
    )
}

fn probe_cone(deg: f64, apex_up: bool, window: f64) -> Tally {
    let (solid, apex, nappe) = cone_solid(deg, apex_up, window);
    let planes = apex_planes(deg.to_radians(), nappe, window);
    let t = run(&solid, apex, &planes);
    report(&solid.name, &t);
    t
}

#[test]
fn r1_other_half_angles() {
    let mut wrong = 0;
    for deg in [10.0, 30.0, 45.0, 80.0] {
        wrong += probe_cone(deg, true, TAU).wrong.len();
    }
    assert_eq!(wrong, 0, "a landed side is wrong");
}

#[test]
fn r1_apex_down() {
    let mut wrong = 0;
    for deg in [30.0, 60.0] {
        wrong += probe_cone(deg, false, TAU).wrong.len();
    }
    assert_eq!(wrong, 0, "a landed side is wrong");
}

#[test]
fn r1_partial_windows() {
    let mut wrong = 0;
    for window in [2.0, PI - 0.2, PI, PI + 0.3, 4.5, 5.8] {
        wrong += probe_cone(40.0, true, window).wrong.len();
    }
    assert_eq!(wrong, 0, "a landed side is wrong");
}

/// A spinning top: a cone standing on a cylinder, the apex on top.
#[test]
fn r1_a_cone_on_a_cylinder() {
    let solid = Solid {
        name: "spinning top".into(),
        profile: vec![(0.0, 0.0), (0.6, 0.0), (0.6, 1.0), (0.0, 2.0)],
        window: TAU,
        exact: None,
    };
    let apex = Point3::new(0.0, 2.0, 0.0);
    let alpha = 0.6f64.atan();
    let planes = apex_planes(alpha, 1.0, TAU);
    let t = run(&solid, apex, &planes);
    report(&solid.name, &t);
    assert_eq!(t.wrong.len(), 0, "a landed side is wrong");
}

/// A truncated cone capped flat: the apex is not a vertex. Planes through
/// the virtual apex miss the body or cut the frustum.
#[test]
fn r1_a_frustum_through_its_virtual_apex() {
    let solid = Solid {
        name: "frustum".into(),
        profile: vec![(0.0, 0.0), (1.0, 0.0), (0.4, 0.6), (0.0, 0.6)],
        window: TAU,
        exact: None,
    };
    let apex = Point3::new(0.0, 1.0, 0.0);
    let planes = apex_planes(45f64.to_radians(), 1.0, TAU);
    let t = run(&solid, apex, &planes);
    report(&solid.name, &t);
    assert_eq!(t.wrong.len(), 0, "a landed side is wrong");
}

/// **A plane through the apex of a cone revolved over a half-turn never
/// panics.** The wedge's one wall sector at the apex spans 4.5 rad, so
/// `apex_shape` subdivides it at both extremal rulings; at these poses
/// both land inside it, and `insert_null_edges`' lone-duplicate arm
/// reaches `unreachable!` (`insert.rs:501`). Red on the reviewed head
/// (86bdfc99) at every ε.
#[test]
fn r1_a_wedge_over_a_half_turn_splits_through_its_apex_without_panicking() {
    let (deg, window) = (40.0, 4.5);
    let (solid, apex, _) = cone_solid(deg, true, window);
    let graze = FRAC_PI_2 - f64::to_radians(deg);
    let at = |beta: f64, gamma: f64| {
        Vec3::new(
            beta.sin() * gamma.cos(),
            beta.cos(),
            -beta.sin() * gamma.sin(),
        )
    };
    let planes = vec![
        ("graze + 1e-3, γ 3.7".to_string(), at(graze + 1e-3, 3.7)),
        ("graze + 1e-3, γ 4.3".to_string(), at(graze + 1e-3, 4.3)),
        ("graze + 0.05, γ 3.7".to_string(), at(graze + 0.05, 3.7)),
    ];
    let t = run(&solid, apex, &planes);
    report(&solid.name, &t);
    assert!(t.wrong.is_empty(), "{:?}", t.wrong);
}
