//! Review probes for PR 5956 (a plane through a cone's apex), reviewer R2.
//!
//! Poses the pins do not hold: half-angles 10° and 80°, a cone with its
//! apex down, partial revolves whose apex sector is under or over a
//! half-turn (with the wedge caps fused at the apex), a cone on a
//! cylinder, secant planes through a conical pit's apex, and a frustum
//! cut through its virtual apex. Planes through the apex are sampled
//! over the sphere of normals, including planes holding the axis (and
//! with it both seam meridians) and planes a hair off it. Each side
//! that lands is checked against an independent closed form (the cone
//! over the base region on that side, or the slice integral of the
//! solid of revolution) and tier 3′; a refusal is recorded by its
//! variant.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use topo::ContactRecords;
use topo::splitting::split;

/// Adaptive Simpson over `[a, b]` to absolute tolerance `eps`.
fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64, eps: f64) -> f64 {
    #[allow(clippy::too_many_arguments)]
    fn rec(
        f: &dyn Fn(f64) -> f64,
        a: f64,
        b: f64,
        fa: f64,
        fm: f64,
        fb: f64,
        whole: f64,
        eps: f64,
        depth: u32,
    ) -> f64 {
        let m = 0.5 * (a + b);
        let (lm, rm) = (0.5 * (a + m), 0.5 * (m + b));
        let (flm, frm) = (f(lm), f(rm));
        let left = (m - a) / 6.0 * (fa + 4.0 * flm + fm);
        let right = (b - m) / 6.0 * (fm + 4.0 * frm + fb);
        if depth == 0 || (left + right - whole).abs() <= 15.0 * eps.max(1e-17 * (b - a)) {
            return left + right + (left + right - whole) / 15.0;
        }
        rec(f, a, m, fa, flm, fm, left, eps / 2.0, depth - 1)
            + rec(f, m, b, fm, frm, fb, right, eps / 2.0, depth - 1)
    }
    // Pre-split into panels so a kink cannot hide between samples.
    let panels = 64;
    let h = (b - a) / panels as f64;
    (0..panels)
        .map(|i| {
            let (x0, x1) = (a + h * i as f64, a + h * (i + 1) as f64);
            let (f0, fm, f1) = (f(x0), f(0.5 * (x0 + x1)), f(x1));
            rec(
                f,
                x0,
                x1,
                f0,
                fm,
                f1,
                (x1 - x0) / 6.0 * (f0 + 4.0 * fm + f1),
                eps / panels as f64,
                30,
            )
        })
        .sum()
}

/// The area of `{ρ ∈ [r_in, r_out], φ ∈ window} ∩ {nx·x + nz·z > d}`,
/// the point at `(ρ cos φ, −ρ sin φ)` (a right-hand turn about +y).
/// `window` `None` is the whole turn.
fn slice_area(r_in: f64, r_out: f64, window: Option<f64>, nx: f64, nz: f64, d: f64) -> f64 {
    if r_out <= r_in {
        return 0.0;
    }
    let m = nx.hypot(nz);
    match window {
        None => {
            if m < 1e-300 {
                return if d < 0.0 {
                    PI * (r_out * r_out - r_in * r_in)
                } else {
                    0.0
                };
            }
            let delta = d / m;
            let seg = |r: f64| {
                let dd = delta.clamp(-r, r);
                r * r * (dd / r).acos() - dd * (r * r - dd * dd).sqrt()
            };
            seg(r_out) - if r_in > 0.0 { seg(r_in) } else { 0.0 }
        }
        Some(w) => {
            let f = |phi: f64| {
                let c = nx * phi.cos() - nz * phi.sin();
                let (lo, hi) = if c > 0.0 {
                    ((d / c).max(r_in), r_out)
                } else if c < 0.0 {
                    (r_in, (d / c).min(r_out))
                } else if d < 0.0 {
                    (r_in, r_out)
                } else {
                    (0.0, 0.0)
                };
                if hi > lo {
                    0.5 * (hi * hi - lo * lo)
                } else {
                    0.0
                }
            };
            simpson(&f, 0.0, w, 1e-15)
        }
    }
}

/// A solid of revolution about +y: `y ∈ [y0, y1]`, the radii at `y`.
struct Solid {
    name: String,
    body: topo::AtRestBody<f64>,
    y_range: (f64, f64),
    radii: Box<dyn Fn(f64) -> (f64, f64)>,
    window: Option<f64>,
    /// The apex (or virtual apex) the planes are posed through.
    apex: Point3<f64>,
    /// A cone over a base disc at height `base_y` with radius `base_r`:
    /// read the side's volume as `h·A/3` instead of the slice integral.
    cone: Option<(f64, f64)>,
}

impl Solid {
    /// The volume on the side `n·(X − p) > 0`.
    fn above(&self, p: Point3<f64>, n: Vec3<f64>) -> f64 {
        let area_at = |y: f64, r_in: f64, r_out: f64| {
            // n·((x, y, z) − p) > 0  ⇔  nx x + nz z > n·p − ny y.
            let d = n.x * p.x + n.y * p.y + n.z * p.z - n.y * y;
            slice_area(r_in, r_out, self.window, n.x, n.z, d)
        };
        if let Some((base_y, base_r)) = self.cone {
            let h = (base_y - self.apex.y).abs();
            return h * area_at(base_y, 0.0, base_r) / 3.0;
        }
        let f = |y: f64| {
            let (a, b) = (self.radii)(y);
            area_at(y, a, b)
        };
        simpson(&f, self.y_range.0, self.y_range.1, 1e-14)
    }

    fn whole(&self) -> f64 {
        let f = |y: f64| {
            let (a, b) = (self.radii)(y);
            let w = self.window.unwrap_or(2.0 * PI);
            0.5 * w * (b * b - a * a)
        };
        simpson(&f, self.y_range.0, self.y_range.1, 1e-14)
    }
}

fn revolve(pts: &[(f64, f64)], rev: sweep::Revolution<f64>) -> topo::AtRestBody<f64> {
    let tol = Tol::witness();
    let lp = bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect());
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
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
    sweep::test_support::finished("probe solid", body, tol)
}

fn rev_of(window: Option<f64>) -> sweep::Revolution<f64> {
    window.map_or(sweep::Revolution::Full, sweep::Revolution::Partial)
}

/// A cone, apex up at `(0, 1, 0)`, base radius `tan α` at `y = 0`.
fn cone_up(deg: f64, window: Option<f64>) -> Solid {
    let r = deg.to_radians().tan();
    Solid {
        name: format!("cone up {deg}° window {window:?}"),
        body: revolve(&[(0.0, 0.0), (r, 0.0), (0.0, 1.0)], rev_of(window)),
        y_range: (0.0, 1.0),
        radii: Box::new(move |y| (0.0, r * (1.0 - y))),
        window,
        apex: Point3::new(0.0, 1.0, 0.0),
        cone: Some((0.0, r)),
    }
}

/// A cone, apex down at the origin, base radius `tan α` at `y = 1`.
fn cone_down(deg: f64, window: Option<f64>) -> Solid {
    let r = deg.to_radians().tan();
    Solid {
        name: format!("cone down {deg}° window {window:?}"),
        body: revolve(&[(0.0, 0.0), (r, 1.0), (0.0, 1.0)], rev_of(window)),
        y_range: (0.0, 1.0),
        radii: Box::new(move |y| (0.0, r * y)),
        window,
        apex: Point3::new(0.0, 0.0, 0.0),
        cone: Some((1.0, r)),
    }
}

/// A unit cylinder of height 1 capped by a cone of height `h` (apex at
/// `y = 1 + h`).
fn cone_on_cylinder(h: f64) -> Solid {
    Solid {
        name: format!("cone h {h} on a cylinder"),
        body: revolve(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0 + h)],
            rev_of(None),
        ),
        y_range: (0.0, 1.0 + h),
        radii: Box::new(move |y| {
            if y <= 1.0 {
                (0.0, 1.0)
            } else {
                (0.0, (1.0 - (y - 1.0) / h).max(0.0))
            }
        }),
        window: None,
        apex: Point3::new(0.0, 1.0 + h, 0.0),
        cone: None,
    }
}

/// The pins' pitted cylinder: a 30° pit sunk from the top of a unit
/// cylinder, rim radius ½.
fn pit() -> Solid {
    let (rim, depth) = (0.5, 0.5 / 30f64.to_radians().tan());
    let floor = 1.0 - depth;
    Solid {
        name: "30° pit".into(),
        body: revolve(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (rim, 1.0), (0.0, floor)],
            rev_of(None),
        ),
        y_range: (0.0, 1.0),
        radii: Box::new(move |y| {
            if y <= floor {
                (0.0, 1.0)
            } else {
                ((y - floor) / depth * rim, 1.0)
            }
        }),
        window: None,
        apex: Point3::new(0.0, floor, 0.0),
        cone: None,
    }
}

/// A frustum of the 45° cone apex up at `(0, 1, 0)`, cut at `y = ½`;
/// the planes pass its virtual apex.
fn frustum() -> Solid {
    Solid {
        name: "45° frustum, virtual apex".into(),
        body: revolve(
            &[(0.0, 0.0), (1.0, 0.0), (0.5, 0.5), (0.0, 0.5)],
            rev_of(None),
        ),
        y_range: (0.0, 0.5),
        radii: Box::new(|y| (0.0, 1.0 - y)),
        window: None,
        apex: Point3::new(0.0, 1.0, 0.0),
        cone: None,
    }
}

/// Unit normals over the sphere: polar angle `γ` from +y, azimuth `ψ`
/// (`(sin γ cos ψ, cos γ, −sin γ sin ψ)`, the revolve's own turn).
fn normals() -> Vec<(String, Vec3<f64>)> {
    let gammas = [
        0.05,
        0.4,
        0.9,
        1.3,
        FRAC_PI_2 - 1e-4,
        FRAC_PI_2,
        FRAC_PI_2 + 0.02,
        2.2,
        PI - 0.3,
    ];
    let psis = [0.0, 0.4, FRAC_PI_2, 2.0, PI, 3.9, 5.2];
    let mut out = Vec::new();
    for g in gammas {
        for p in psis {
            out.push((
                format!("γ {g:.4} ψ {p:.4}"),
                Vec3::new(g.sin() * p.cos(), g.cos(), -g.sin() * p.sin()),
            ));
        }
    }
    out
}

/// Splits `solid` through its apex at every normal; fails on a wrong
/// volume, an empty side with material, or a panic; prints tier-3′
/// findings; returns the refusals by variant.
fn probe(solid: &Solid, normals: &[(String, Vec3<f64>)]) -> BTreeMap<String, Vec<String>> {
    let tol = Tol::witness();
    let eps = tol.eps();
    let whole = solid.whole();
    let pinned_whole = topo::test_support::pinned_volume("whole", &solid.body, tol).volume;
    assert!(
        (pinned_whole - whole).abs() <= 1e-9,
        "{}: whole {pinned_whole} vs closed form {whole}",
        solid.name
    );
    let area = 4.0 * PI * 3.0; // a generous surface area bound
    let slack = 2048.0 * eps * area;
    let mut refusals: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut wrong = Vec::new();
    let mut landed = 0;
    for (label, n) in normals {
        let what = format!("{}: {label}", solid.name);
        let plane = topo::test_support::split_plane(solid.apex, *n, tol);
        let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            split(&solid.body, &plane, tol)
        }));
        let Ok(got) = got else {
            wrong.push(format!("{what}: PANICKED"));
            continue;
        };
        let result = match got {
            Ok(r) => r,
            Err(e) => {
                eprintln!("R2 REFUSAL {what}: {e:?}");
                let key = format!("{e:?}");
                let key = key.split(['{', '(']).take(3).collect::<Vec<_>>().join("|");
                refusals.entry(key).or_default().push(label.clone());
                continue;
            }
        };
        landed += 1;
        let want_above = solid.above(solid.apex, *n);
        for (side, part, want) in [
            ("above", &result.above, want_above),
            ("below", &result.below, whole - want_above),
        ] {
            let other = whole - want;
            match part.body() {
                None => {
                    if want > slack {
                        wrong.push(format!("{what}, {side}: empty, closed form {want}"));
                    }
                }
                Some(b) => {
                    let got = topo::test_support::pinned_volume(&what, b, tol).volume;
                    let landed_whole = (got - whole).abs() <= slack && other.min(want) <= slack;
                    if (got - want).abs() > slack && !landed_whole {
                        wrong.push(format!(
                            "{what}, {side}: volume {got}, closed form {want} (Δ {:e})",
                            got - want
                        ));
                    }
                    if let Err(e) =
                        topo::validate_pseudomanifold(b, &ContactRecords::default(), tol)
                    {
                        // The split's gate reads tier 3, not 3′ (filed:
                        // `the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime`):
                        // reported, not asserted.
                        eprintln!("R2 TIER3' {what}, {side}: {e:?}");
                    }
                }
            }
        }
    }
    eprintln!(
        "R2 {} ε {eps:e}: {landed}/{} landed; refusals {:#?}",
        solid.name,
        normals.len(),
        refusals
            .iter()
            .map(|(k, v)| (k, v.len()))
            .collect::<Vec<_>>()
    );
    for (k, v) in &refusals {
        eprintln!("R2   {k}: {v:?}");
    }
    assert!(wrong.is_empty(), "R2 WRONG:\n{}", wrong.join("\n"));
    // Measured: every solid here lands at least 60 of its 63 normals
    // (95%) at ε 1e-6, 1e-9 and 1e-12; the rest refuse typed.
    assert!(
        landed * 100 >= normals.len() * 93,
        "{}: landed {landed} of {} normals, under the measured floor of 93%",
        solid.name,
        normals.len()
    );
    refusals
}

#[test]
fn r2_partial_revolve_turn_direction() {
    // Which way does Partial turn? A plane z = 0 cut of a cone revolved
    // by 2 rad: with the right-hand turn about +y the swept points have
    // z ≤ 0, so `+z` holds nothing.
    let s = cone_up(45.0, Some(2.0));
    let tol = Tol::witness();
    let plane =
        topo::test_support::split_plane(Point3::new(0.0, 0.5, 0.0), Vec3::new(0.0, 0.0, 1.0), tol);
    let r =
        split(&s.body, &plane, tol).map(|r| (r.above.body().is_some(), r.below.body().is_some()));
    assert!(
        matches!(r, Ok((false, true))),
        "the closed forms' turn: {r:?}"
    );
}

#[test]
fn r2_cones_at_10_and_80_degrees() {
    let ns = normals();
    for deg in [10.0, 80.0] {
        probe(&cone_up(deg, None), &ns);
    }
}

#[test]
fn r2_cones_apex_down() {
    let ns = normals();
    for deg in [30.0, 60.0] {
        probe(&cone_down(deg, None), &ns);
    }
}

#[test]
fn r2_partial_cones() {
    let ns = normals();
    probe(&cone_down(30.0, Some(4.5)), &ns);
    for w in [1.0, 2.0, 3.0, 4.0, 5.5] {
        probe(&cone_up(45.0, Some(w)), &ns);
    }
}

#[test]
fn r2_cone_on_a_cylinder() {
    let ns = normals();
    probe(&cone_on_cylinder(1.0), &ns);
}

#[test]
fn r2_pit_secants() {
    let ns = normals();
    probe(&pit(), &ns);
}

#[test]
fn r2_frustum_through_its_virtual_apex() {
    let ns = normals();
    probe(&frustum(), &ns);
}

/// The planar analogue of the pit's apex-only touch: a cube with a
/// corner notch, split through the notch's inner corner by the plane
/// whose normal is the corner's diagonal. Every edge out of the corner
/// leaves above; the material lies on both sides.
#[test]
#[ignore = "measurement: the same unrecorded VertexOnFace on main and on PR 5956's head"]
fn r2_planar_analogue_a_notch_corner_touching_the_plane() {
    let tol = Tol::witness();
    let brick = |x, y, z| {
        sweep::test_support::finished(
            "brick",
            sweep::test_support::brick::<f64>(x, y, z, tol),
            tol,
        )
    };
    let cube = brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let notch = brick((1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
    let body = match topo::subtract(&cube, &notch, tol) {
        Ok(topo::boolean::BooleanResult::Body(b)) => b,
        other => panic!("cube ∖ notch: {:?}", other.map(|_| ())),
    };
    for (what, n) in [
        ("diagonal", Vec3::new(1.0, 1.0, 1.0).normalize()),
        ("tilted", Vec3::new(1.0, 1.3, 0.8).normalize()),
    ] {
        let plane = topo::test_support::split_plane(Point3::new(1.0, 1.0, 1.0), n, tol);
        match split(&body.body, &plane, tol) {
            Ok(r) => {
                for (side, part) in [("above", &r.above), ("below", &r.below)] {
                    if let Some(b) = part.body() {
                        eprintln!(
                            "R2 notch {what} {side}: tier 3′ {:?}",
                            topo::validate_pseudomanifold(b, &ContactRecords::default(), tol)
                        );
                    }
                }
            }
            Err(e) => eprintln!("R2 notch {what}: refused {e:?}"),
        }
    }
}

/// Every pose of the over-π partial cones, each split caught so a panic
/// names its pose.
#[test]
fn r2_partial_over_a_half_turn_never_panics() {
    let tol = Tol::witness();
    let mut panicked = Vec::new();
    for (solid, w) in [
        (cone_up(45.0, Some(5.5)), 5.5),
        (cone_up(45.0, Some(4.0)), 4.0),
        (cone_down(30.0, Some(4.5)), 4.5),
        (cone_up(30.0, Some(5.0)), 5.0),
    ] {
        for (label, n) in normals() {
            let plane = topo::test_support::split_plane(solid.apex, n, tol);
            let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                split(&solid.body, &plane, tol).map(|_| ())
            }));
            if got.is_err() {
                panicked.push(format!("{} w {w}: {label}", solid.name));
            }
        }
    }
    eprintln!(
        "R2 PANICS ε {:e}: {} {panicked:#?}",
        tol.eps(),
        panicked.len()
    );
    assert!(panicked.is_empty());
}
