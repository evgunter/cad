//! **Plane × cone's apex outcomes stand in for a section only inside
//! the band.** A plane a hair off the apex (its gap Zero) cuts the conic
//! its plane through the apex degenerates from, and that conic stands
//! an extent-scale distance from the apex when the discriminant `D` or
//! the cone's aperture is small:
//!
//! - where the plane through the apex clears the cone (`D < 0`), the
//!   ellipse reaches `|δ|/|D|` from the apex, and the apex point is
//!   served only inside the band of it;
//! - where it holds generators, each nappe's section starts where the
//!   plane is steepest against that nappe's generators, and the line
//!   pair (through the apex on both nappes) or the tangent generator is
//!   served only where the section starts inside the band of the apex.
//!
//! The oracles here never read `D`: they walk the generators, find the
//! shallowest (for the reach) or the steepest on each side of the plane
//! (for where each nappe's section starts), sampled and then refined by
//! ternary search, and divide the gap by that slope.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/intersect.rs",
    "crates/geom-brep/tests/shared/"
];

use crate::shared::surf::at_apex;
use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::intersect::{OpenConic, PlaneConeSection, SectionError, plane_cone_section};
use geom_core::{Band, Point3, Tol, Vec3};
use test_utils::fuzz;

/// A cone and the plane `gap` off its apex along `normal`.
struct Pose {
    apex: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    alpha: f64,
    normal: Vec3<f64>,
    gap: f64,
}

impl Pose {
    fn cone(&self) -> Surface<f64> {
        Surface::Cone {
            apex: self.apex,
            axis: self.axis,
            half_angle: self.alpha,
            u_ref: self.u_ref,
        }
    }

    /// The plane with `(apex − origin)·normal = gap`.
    fn plane(&self) -> Surface<f64> {
        let n = self.normal;
        let u = if n.x.abs() < 0.9 {
            Vec3::unit_x()
        } else {
            Vec3::unit_y()
        };
        Surface::Plane {
            origin: self.apex - n * self.gap,
            normal: n,
            u_ref: (u - n * u.dot(n)).normalize(),
        }
    }

    /// The gap the built plane realizes: forming its origin rounds, and
    /// `apex − origin` is then exact, so this reads it back.
    fn realized_gap(&self) -> f64 {
        let Surface::Plane { origin, .. } = self.plane() else {
            unreachable!()
        };
        (self.apex - origin).dot(self.normal)
    }

    fn served(&self, extent: f64, band: Band) -> Result<PlaneConeSection<f64>, SectionError> {
        plane_cone_section(
            &self.plane(),
            &self.cone(),
            &at_apex(&self.cone(), extent),
            band,
        )
    }

    /// The unit generator at azimuth `u` from a frame built off the axis
    /// alone: the cone is its apex, axis and half-angle, and its stored
    /// `u_ref` (whose tilt off the axis nothing checks) never enters the
    /// oracle.
    fn generator(&self, u: f64) -> Vec3<f64> {
        let a = self.axis;
        let across = if a.x.abs() < 0.5 {
            Vec3::unit_x()
        } else {
            Vec3::unit_y()
        };
        let e1 = a.cross(across).normalize();
        let e2 = a.cross(e1);
        let (su, cu) = u.sin_cos();
        a * self.alpha.cos() + (e1 * cu + e2 * su) * self.alpha.sin()
    }

    /// The generator the plane grazes: of the two where `g·n` is
    /// extremal over the azimuth (radial along `±` the normal's part
    /// square to the axis), the one whose slope is nearer zero.
    fn grazed(&self) -> Vec3<f64> {
        let a = self.axis;
        let across = (self.normal - a * self.normal.dot(a)).normalize();
        let (s, c) = self.alpha.sin_cos();
        let (up, down) = (a * c + across * s, a * c - across * s);
        if up.dot(self.normal).abs() <= down.dot(self.normal).abs() {
            up
        } else {
            down
        }
    }

    /// The plane's slope to the generator at azimuth `u`, `g(u)·n`.
    fn slope(&self, u: f64) -> f64 {
        self.generator(u).dot(self.normal)
    }

    /// The slope at its extreme under `key` (the generator maximizing
    /// `key(g·n)`): the best of 1024 samples, refined by ternary search
    /// about it.
    fn extreme_slope(&self, key: impl Fn(f64) -> f64) -> f64 {
        const N: usize = 1024;
        let step = core::f64::consts::TAU / N as f64;
        let at = |u: f64| key(self.slope(u));
        let i = (0..N)
            .max_by(|&i, &j| at(i as f64 * step).total_cmp(&at(j as f64 * step)))
            .unwrap();
        let (mut lo, mut hi) = ((i as f64 - 1.0) * step, (i as f64 + 1.0) * step);
        for _ in 0..200 {
            let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
            if at(m1) > at(m2) {
                hi = m2;
            } else {
                lo = m1;
            }
        }
        self.slope(0.5 * (lo + hi))
    }

    /// The oracle: the farthest the section reaches from the apex, or
    /// `None` where some generator runs parallel to the plane or crosses
    /// it on both nappes' sides (no closed section). The plane meets the
    /// line through generator `g` at `−gap/(g·n)`; the ellipse's farthest
    /// point is on the generator whose `|g·n|` is least.
    fn reach(&self) -> Option<f64> {
        let sign = self.slope(0.0).signum();
        let crosses = (0..1024).any(|i| {
            let s = self.slope(f64::from(i) * core::f64::consts::TAU / 1024.0);
            s.signum() != sign || s == 0.0
        });
        (!crosses).then(|| self.realized_gap().abs() / self.extreme_slope(|x| -x.abs()).abs())
    }

    /// The second oracle: where the section starts on each nappe, as its
    /// nearest point's distance from the apex (`None` where the nappe
    /// carries none), the `+axis` nappe first. The plane meets the line
    /// through generator `g` at `t = −gap/(g·n)`, on the nappe `t`'s sign
    /// names, so a nappe's nearest point is on the generator steepest
    /// against the plane among those of the slope's sign that nappe needs.
    /// A plane through the apex starts its section at the apex, which
    /// both nappes share.
    fn starts(&self) -> (Option<f64>, Option<f64>) {
        let gap = self.realized_gap();
        if gap == 0.0 {
            return (Some(0.0), Some(0.0));
        }
        let (most, least) = (self.extreme_slope(|x| x), self.extreme_slope(|x| -x));
        // The `+axis` nappe (`t > 0`) needs a slope of the sign of `−gap`.
        let start = |needed: f64| {
            let steepest = if needed > 0.0 { most } else { least };
            (steepest * needed > 0.0).then(|| gap.abs() / steepest.abs())
        };
        (start(-gap), start(gap))
    }

    /// The worst distance of a served conic from the plane and from the
    /// double cone, and its farthest point from the apex.
    fn residuals(&self, c: &Curve3<f64>) -> (f64, f64, f64) {
        let gap = self.realized_gap();
        let mut worst = (0.0_f64, 0.0_f64, 0.0_f64);
        for i in 0..1024 {
            let p = c.eval(f64::from(i) * core::f64::consts::TAU / 1024.0);
            let w = p - self.apex;
            let z = w.dot(self.axis);
            let rho = (w - self.axis * z).norm();
            worst = (
                worst.0.max((w.dot(self.normal) + gap).abs()),
                worst
                    .1
                    .max((rho * self.alpha.cos() - z.abs() * self.alpha.sin()).abs()),
                worst.2.max(w.norm()),
            );
        }
        worst
    }
}

/// The cone of half-angle `alpha` about `+z` at the origin and the plane
/// whose normal leans `theta` off the axis toward `+x`, `gap` off the
/// apex.
fn upright(alpha: f64, theta: f64, gap: f64) -> Pose {
    Pose {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        alpha,
        normal: Vec3::new(theta.sin(), 0.0, theta.cos()),
        gap,
    }
}

/// The lean that puts the discriminant `D = −cos(α + θ)` at `d`.
fn lean_for(alpha: f64, d: f64) -> f64 {
    (-d).acos() - alpha
}

/// The pose PR 4280's review measured: `D·extent = −10.7·ε` beside a
/// `0.6·ε` gap, whose ellipse reaches `0.6/10.7` of the unit extent.
/// The apex point stood in for it; the ellipse is served, on either side
/// of the apex.
#[test]
fn the_review_pose_serves_the_ellipse_its_gap_cuts() {
    let (e, k) = (eps(), Tol::witness().k());
    let alpha = core::f64::consts::FRAC_PI_6;
    for gap in [0.6 * e, -0.6 * e] {
        let pose = upright(alpha, lean_for(alpha, -10.7 * e), gap);
        let reach = pose.reach().expect("an ellipse");
        assert!(
            (reach - 0.6 / 10.7).abs() < 1e-6,
            "gap {gap:e}: the oracle's reach is {reach}"
        );
        let got = pose.served(1.0, band());
        let Ok(PlaneConeSection::TiltedEllipse(c)) = &got else {
            panic!("gap {gap:e}: the {reach} m ellipse is the section, got {got:?}");
        };
        let (on_plane, on_cone, far) = pose.residuals(c);
        assert!(
            on_plane < k * e && on_cone < k * e,
            "gap {gap:e}: the served ellipse stands {on_plane:e} off the plane, \
             {on_cone:e} off the cone"
        );
        assert!(
            (far - reach).abs() < 1e-3 * reach,
            "gap {gap:e}: the served ellipse reaches {far}, the oracle {reach}"
        );
    }
}

/// The family around the review pose: half-angles from a needle to a
/// near-flat cone, discriminants from just past the band to well clear,
/// gaps across the zero band. Wherever the oracle's section reaches past
/// the band the apex point is never served; it is the ellipse, or an
/// escalation.
#[test]
fn a_section_past_the_band_is_never_the_apex_point() {
    let (e, k) = (eps(), Tol::witness().k());
    let mut ellipses = 0;
    for alpha in [0.05, 0.3, core::f64::consts::FRAC_PI_6, 1.0, 1.4] {
        for d_bands in [10.2, 10.7, 12.0, 30.0, 1e3, 1e5] {
            for gap_bands in [0.05_f64, 0.2, 0.6, -0.6, 0.95] {
                let d = -d_bands * e;
                if (d_bands - gap_bands.abs()) <= k {
                    continue; // the floor escalates: not this row's
                }
                let pose = upright(alpha, lean_for(alpha, d), gap_bands * e);
                let reach = pose.reach().expect("an ellipse");
                let got = pose.served(1.0, band());
                let label = format!("α {alpha}, D {d_bands}·ε, gap {gap_bands}·ε, reach {reach:e}");
                if reach > k * e {
                    assert!(
                        !matches!(got, Ok(PlaneConeSection::ApexPoint(_))),
                        "{label}: the apex point stood in for the section"
                    );
                }
                if let Ok(PlaneConeSection::TiltedEllipse(c)) = &got {
                    ellipses += 1;
                    let (on_plane, on_cone, far) = pose.residuals(c);
                    assert!(
                        on_plane < k * e && on_cone < k * e && (far - reach).abs() < 1e-3 * reach,
                        "{label}: served {on_plane:e} off the plane, {on_cone:e} off the cone, \
                         reaching {far:e}"
                    );
                }
            }
        }
    }
    assert!(ellipses > 0, "the family served no ellipse");
}

/// A true apex point keeps it: the plane through the apex, and one off
/// it by half a band's worth of the discriminant (its section within
/// half a band of the apex).
#[test]
fn a_section_inside_the_band_keeps_the_apex_point() {
    let e = eps();
    let alpha = core::f64::consts::FRAC_PI_6;
    for (theta, gap_of_d) in [(0.3, 0.0), (0.3, 0.5), (0.3, -0.5), (0.9, 0.5)] {
        let d = -(alpha + theta).cos();
        let pose = upright(alpha, theta, gap_of_d * e * d.abs());
        let reach = pose.reach().unwrap_or(0.0);
        assert!(
            reach <= 0.5 * e * (1.0 + 1e-9),
            "θ {theta}: reach {reach:e}"
        );
        let got = pose.served(1.0, band());
        let Ok(PlaneConeSection::ApexPoint(p)) = got else {
            panic!("θ {theta}, gap {gap_of_d}·ε·|D|: the apex point, got {got:?}");
        };
        assert_eq!((p.x, p.y, p.z), (0.0, 0.0, 0.0), "θ {theta}: the apex");
    }
}

/// The line outcomes' bound: where the oracle's section starts on the
/// nappe farther from the apex for the pair (it runs through the apex on
/// both), on the nearer for the tangent generator (a touching whose
/// clearing nappe carries no section). `None` where a needed nappe
/// carries no section, which no served line pair may meet.
fn line_start(pose: &Pose, pair: bool) -> Option<f64> {
    let (up, down) = pose.starts();
    if pair {
        up.zip(down).map(|(u, d)| u.max(d))
    } else {
        match (up, down) {
            (Some(u), Some(d)) => Some(u.min(d)),
            (one, other) => one.or(other),
        }
    }
}

/// The worst distance of a served line's points within `extent` of the
/// apex from the plane, and whether it runs through the apex along a
/// generator of the cone (to `tol` per unit length).
fn line_residuals(pose: &Pose, line: &Curve3<f64>, extent: f64, tol: f64) -> (f64, bool) {
    let Curve3::Line { origin, dir } = *line else {
        panic!("a line outcome carries a line, got {line:?}");
    };
    let gap = pose.realized_gap();
    let dir = dir.normalize();
    let off_plane = [-extent, 0.0, extent]
        .into_iter()
        .map(|t| ((origin + dir * t - pose.apex).dot(pose.normal) + gap).abs())
        .fold(0.0, f64::max);
    let through_apex = (origin - pose.apex).norm() <= tol;
    let on_cone = (dir.dot(pose.axis).abs() - pose.alpha.cos()).abs() <= tol;
    (off_plane, through_apex && on_cone)
}

/// The plane containing the axis direction: `D = M = sin α`.
fn along_axis(alpha: f64, gap: f64) -> Pose {
    upright(alpha, core::f64::consts::FRAC_PI_2, gap)
}

/// The plane through the apex tangent to the cone along a generator
/// (`D = 0`, `M = sin 2α`), `gap` off the apex.
fn tangent(alpha: f64, gap: f64) -> Pose {
    upright(alpha, core::f64::consts::FRAC_PI_2 - alpha, gap)
}

/// The mirror of a pose through the apex's tangent plane
/// (`axis·normal < 0`).
fn mirrored(mut p: Pose) -> Pose {
    p.normal = Vec3::new(-p.normal.x, p.normal.y, -p.normal.z);
    p
}

/// Whether `got` is served as `conic`.
fn names(got: &Result<PlaneConeSection<f64>, SectionError>, conic: OpenConic) -> bool {
    matches!(got, Ok(PlaneConeSection::OpenConic(s)) if s.conic() == conic)
}

/// **The item's needle poses** (`sin α = (K + 2)·ε`, unit extent, the
/// plane `0.9·ε` off the apex): along the axis, `D·extent = (K + 2)·ε`
/// and the hyperbola starts `0.9/(K + 2)` m from the apex on both
/// nappes; tangent, `D = 0`, and the section starts `0.9/(2·(K + 2))`
/// m off. Beside them the same miss at an ordinary aperture: `α = π/6`,
/// `D·extent = (K + 1)·ε`, whose hyperbola starts `ε`-near the apex on
/// one nappe and `0.9/(K + 1)` m off on the other. None is a line
/// outcome; each is served as its open conic.
#[test]
fn a_needle_section_off_the_apex_is_not_its_lines() {
    let (e, k) = (eps(), Tol::witness().k());
    let needle = ((k + 2.0) * e).asin();
    let ordinary = core::f64::consts::FRAC_PI_6;
    let rows = [
        (
            "needle, along the axis",
            along_axis(needle, 0.9 * e),
            true,
            0.9 / (k + 2.0),
        ),
        (
            "needle, tangent",
            tangent(needle, 0.9 * e),
            false,
            0.9 / (2.0 * (k + 2.0)),
        ),
        (
            "π/6, D just past the band",
            upright(ordinary, lean_for(ordinary, (k + 1.0) * e), 0.9 * e),
            true,
            0.9 / (k + 1.0),
        ),
    ];
    for (name, pose, pair, start) in rows {
        let oracle = line_start(&pose, pair).expect("the section starts somewhere");
        assert!(
            (oracle - start).abs() < 1e-3 * start,
            "{name}: the oracle's section starts {oracle} m off, the closed form {start}"
        );
        let got = pose.served(1.0, band());
        let conic = if pair {
            OpenConic::Hyperbola
        } else {
            OpenConic::Parabola
        };
        assert!(
            names(&got, conic),
            "{name}: the section starts {oracle} m off the apex, so it is served as the \
             {conic:?}, got {got:?}"
        );
    }
}

/// A named pose family, built at a gap.
type PoseAt = Box<dyn Fn(f64) -> Pose>;

/// The family around the item's poses: needles to near-flat cones, the
/// plane along the axis, leaning to a small or a steep discriminant, and
/// tangent, both faces, gaps across the zero band. A served line outcome
/// has its section starting inside the band of the apex, runs through
/// the apex along a generator and stays within the band of the plane;
/// past the band the section is served as its open conic, or escalates.
#[test]
fn a_line_outcome_is_served_only_where_its_section_starts_in_the_band() {
    let (e, k) = (eps(), Tol::witness().k());
    let mut served = (0, 0);
    let mut open = 0;
    for alpha in [
        ((k + 2.0) * e).asin(),
        (30.0 * e).asin(),
        1e-3,
        0.05,
        core::f64::consts::FRAC_PI_6,
        1.0,
        1.4,
        core::f64::consts::FRAC_PI_2 - 1e-3,
    ] {
        let mut poses: Vec<(String, PoseAt)> = vec![
            (
                "along the axis".into(),
                Box::new(move |g| along_axis(alpha, g)),
            ),
            ("tangent".into(), Box::new(move |g| tangent(alpha, g))),
        ];
        for d_bands in [k + 1.0, 30.0, 1e3, 1e5] {
            let d = d_bands * e;
            if d < alpha.sin() {
                poses.push((
                    format!("D {d_bands}·ε"),
                    Box::new(move |g| upright(alpha, lean_for(alpha, d), g)),
                ));
            }
        }
        for (pose_name, at) in &poses {
            for gap_bands in [0.0, 0.05, 0.3, 0.6, -0.6, 0.95] {
                for face in [false, true] {
                    let pose = if face {
                        mirrored(at(gap_bands * e))
                    } else {
                        at(gap_bands * e)
                    };
                    let label = format!(
                        "α {alpha}, {pose_name}, gap {gap_bands}·ε{}",
                        if face { ", mirrored" } else { "" }
                    );
                    let got = pose.served(1.0, band());
                    let lines: Vec<&Curve3<f64>> = match &got {
                        Ok(PlaneConeSection::ApexLinePair { l1, l2 }) => {
                            served.0 += 1;
                            vec![l1, l2]
                        }
                        Ok(PlaneConeSection::ApexTangentLine(l)) => {
                            served.1 += 1;
                            vec![l]
                        }
                        Err(SectionError::Escalated(_)) => continue,
                        Ok(PlaneConeSection::OpenConic(_)) => {
                            open += 1;
                            continue;
                        }
                        Ok(_) | Err(_) => continue,
                    };
                    let pair = lines.len() == 2;
                    let start = line_start(&pose, pair);
                    assert!(
                        start.is_some_and(|s| s <= e * (1.0 + 1e-9)),
                        "{label}: a line outcome served with its section starting {start:?} \
                         from the apex"
                    );
                    for l in lines {
                        let (off_plane, generator) = line_residuals(&pose, l, 1.0, 1e-12);
                        assert!(
                            off_plane <= e * (1.0 + 1e-6) && generator,
                            "{label}: the served line stands {off_plane:e} off the plane, \
                             along a generator through the apex: {generator}"
                        );
                    }
                }
            }
        }
    }
    assert!(
        served.0 > 0 && served.1 > 0 && open > 0,
        "the family served {served:?} (pairs, tangents) and {open} open conics"
    );
}

/// The apex outcome a pose's section stands beside, and the distance
/// that bounds it: the apex point by the ellipse's reach, the pair by
/// where its section starts on the farther nappe, the tangent generator
/// by where it starts on the nearer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Apex {
    Point,
    Pair,
    Tangent,
}

impl Apex {
    /// The oracle's bound for this outcome at `pose`.
    fn bound(self, pose: &Pose) -> Option<f64> {
        match self {
            Self::Point => pose.reach(),
            Self::Pair => line_start(pose, true),
            Self::Tangent => line_start(pose, false),
        }
    }

    /// The reach predicate that decides the bound.
    fn row(self) -> &'static str {
        match self {
            Self::Point => "pn_apex_point_reach",
            Self::Pair => "pn_apex_pair_reach",
            Self::Tangent => "pn_apex_tangent_reach",
        }
    }
}

/// The poses the band edge is read at, each with its bound at
/// `bands·ε` beside a gap of `±½·ε`, and each leaning pose beside its
/// mirror (`axis·normal < 0`):
/// - the apex point: a `π/6` cone cut by a plane leaning to the
///   discriminant that puts the reach there, and a near-flat cone
///   (`cos α = 0.9/10.5`) cut across its axis at the gap that does;
/// - the pair: the plane along the axis (`sin α` the slope), and a plane
///   leaning off a `π/4` cone to the discriminant that puts the far
///   nappe's start there;
/// - the tangent: the tangent plane to a cone and to its near-flat
///   complement (`sin 2α` the slope).
fn edge_poses(bands: f64, gap_sign: f64) -> Vec<(&'static str, Apex, Pose)> {
    let gap = gap_sign * 0.5 * eps();
    let slope = 0.5 / bands;
    let sixth = core::f64::consts::FRAC_PI_6;
    let quarter = core::f64::consts::FRAC_PI_4;
    let tilted = || upright(sixth, lean_for(sixth, -slope), gap);
    let flat_cone = (0.9_f64 / 10.5).acos();
    let flat = || upright(flat_cone, 0.0, gap_sign * bands * eps() * 0.9 / 10.5);
    let steep = || upright(quarter, lean_for(quarter, slope), gap);
    let narrow = 0.5 * slope.asin();
    let wide = core::f64::consts::FRAC_PI_2 - narrow;
    vec![
        ("point, tilted", Apex::Point, tilted()),
        ("point, tilted, mirrored", Apex::Point, mirrored(tilted())),
        ("point, flat", Apex::Point, flat()),
        ("point, flat, mirrored", Apex::Point, mirrored(flat())),
        (
            "pair along the axis",
            Apex::Pair,
            along_axis(slope.asin(), gap),
        ),
        ("pair leaning", Apex::Pair, steep()),
        ("pair leaning, mirrored", Apex::Pair, mirrored(steep())),
        ("tangent", Apex::Tangent, tangent(narrow, gap)),
        (
            "tangent, mirrored",
            Apex::Tangent,
            mirrored(tangent(narrow, gap)),
        ),
        ("tangent, near-flat", Apex::Tangent, tangent(wide, gap)),
        (
            "tangent, near-flat, mirrored",
            Apex::Tangent,
            mirrored(tangent(wide, gap)),
        ),
    ]
}

/// An edge pose's label, its apex outcome, the pose and what
/// `plane_cone_section` serves it.
type EdgeRow = (
    String,
    Apex,
    Pose,
    Result<PlaneConeSection<f64>, SectionError>,
);

/// Every edge pose at `bands`, its oracle bound checked, with the pose
/// and the section `plane_cone_section` serves it.
fn at_the_edge(bands: f64) -> Vec<EdgeRow> {
    let e = eps();
    let mut out = Vec::new();
    for gap_sign in [1.0, -1.0] {
        for (name, apex, pose) in edge_poses(bands, gap_sign) {
            let label = format!("{name}, gap {gap_sign}·½ε at {bands}·ε");
            let bound = apex.bound(&pose).expect("the section has a bound");
            assert!(
                (bound / e - bands).abs() < 1e-6 * bands,
                "{label}: the oracle's bound is {:e}·ε",
                bound / e
            );
            let got = pose.served(1.0, band());
            out.push((label, apex, pose, got));
        }
    }
    out
}

/// **A bound inside the band escalates**: neither the apex outcome nor
/// the section is served where the reach or the start lies in
/// `(ε, Kε]`.
#[test]
fn a_bound_inside_the_band_escalates() {
    let k = Tol::witness().k();
    for bands in [1.5, 0.95 * k] {
        for (label, apex, _, got) in at_the_edge(bands) {
            assert!(
                matches!(&got, Err(SectionError::Escalated(d)) if d.predicate == Some(apex.row())),
                "{label}: escalates on {}, got {got:?}",
                apex.row()
            );
        }
    }
}

/// **A bound just past the band is the section's.**
/// - The flat cone's axis-normal circle is served, on both surfaces and
///   at the oracle's reach.
/// - The tilted pose's ellipse has semi-axes inside `Kε` of each other
///   (its semi-major is at most the reach), so the carrier door refuses
///   it typed: never the apex point, and never an escalation of the
///   reach.
/// - The pair's hyperbola and the tangent's near-parabola are each
///   served as their open conic.
#[test]
fn a_bound_just_past_the_band_is_the_section() {
    let (e, k) = (eps(), Tol::witness().k());
    for (label, apex, pose, got) in at_the_edge(1.05 * k) {
        match apex {
            Apex::Point if label.starts_with("point, flat") => {
                let reach = pose.reach().expect("an ellipse");
                let Ok(PlaneConeSection::AxisNormalCircle(c)) = &got else {
                    panic!("{label}: the {reach:e} m circle is the section, got {got:?}");
                };
                let (on_plane, on_cone, far) = pose.residuals(c);
                assert!(
                    on_plane < k * e && on_cone < k * e && (far - reach).abs() < 1e-6 * reach,
                    "{label}: served {on_plane:e} off the plane, {on_cone:e} off the cone, \
                     reaching {far:e} beside the oracle's {reach:e}"
                );
            }
            Apex::Point => assert!(
                matches!(got, Err(SectionError::Carrier(_))),
                "{label}: the sliver ellipse's carrier refuses, got {got:?}"
            ),
            Apex::Pair | Apex::Tangent => {
                let conic = if apex == Apex::Pair {
                    OpenConic::Hyperbola
                } else {
                    OpenConic::Parabola
                };
                assert!(
                    names(&got, conic),
                    "{label}: is served as the {conic:?}, got {got:?}"
                );
            }
        }
    }
}

/// **A bound just short of the band keeps its apex outcome.**
#[test]
fn a_bound_just_short_of_the_band_keeps_the_apex_outcome() {
    for (label, apex, _, got) in at_the_edge(0.95) {
        let kept = match apex {
            Apex::Point => matches!(got, Ok(PlaneConeSection::ApexPoint(_))),
            Apex::Pair => matches!(got, Ok(PlaneConeSection::ApexLinePair { .. })),
            Apex::Tangent => matches!(got, Ok(PlaneConeSection::ApexTangentLine(_))),
        };
        assert!(kept, "{label}: the apex outcome, got {got:?}");
    }
}

/// **A true apex line pair and a true tangent generator still serve**:
/// the plane through the apex (on a needle too), and one `½·ε` off it on
/// a `π/4` cone, whose section starts `ε/√2` (along the axis) and `½·ε`
/// (tangent) from the apex.
#[test]
fn true_apex_lines_still_serve() {
    let (e, k) = (eps(), Tol::witness().k());
    let needle = ((k + 2.0) * e).asin();
    let quarter = core::f64::consts::FRAC_PI_4;
    let sixth = core::f64::consts::FRAC_PI_6;
    let rows = [
        ("π/6 along the axis, through", along_axis(sixth, 0.0), true),
        (
            "π/6 leaning, through",
            upright(sixth, lean_for(sixth, 0.2), 0.0),
            true,
        ),
        (
            "needle along the axis, through",
            along_axis(needle, 0.0),
            true,
        ),
        ("π/6 tangent, through", tangent(sixth, 0.0), false),
        ("needle tangent, through", tangent(needle, 0.0), false),
        (
            "π/4 along the axis, ½ε off",
            along_axis(quarter, 0.5 * e),
            true,
        ),
        ("π/4 tangent, ½ε off", tangent(quarter, 0.5 * e), false),
    ];
    for (name, pose, pair) in rows {
        let start = line_start(&pose, pair).expect("the section starts somewhere");
        assert!(
            start <= e,
            "{name}: the oracle's section starts {start:e} off"
        );
        let got = pose.served(1.0, band());
        let lines = match &got {
            Ok(PlaneConeSection::ApexLinePair { l1, l2 }) if pair => vec![l1, l2],
            Ok(PlaneConeSection::ApexTangentLine(l)) if !pair => vec![l],
            _ => panic!("{name}: the line outcome, got {got:?}"),
        };
        for l in lines {
            let (off_plane, generator) = line_residuals(&pose, l, 1.0, 1e-12);
            assert!(
                off_plane <= e * (1.0 + 1e-6) && generator,
                "{name}: the served line stands {off_plane:e} off the plane, along a \
                 generator through the apex: {generator}"
            );
        }
    }
}

/// **Two fuzz-found poses** (seeds `0xba10fe80e6ee4af9` and
/// `0x5a26154859e91c73`), copied from their draws. Each cone's seam stands off square to its axis
/// (`u_ref·axis` −7.9e-14 and −3.8e-12), its plane at `D` −1.39e-12 and
/// −3.43e-12 (`D·extent` −11.7·ε and −41.8·ε) and −0.47·ε and −0.17·ε off
/// the apex. Read off the tilted seam, an oracle's generators lean off
/// the cone by `τ·sin α`, which moves their slopes to the plane by
/// `τ·sin α·|a·n|` at most, a part of these `D` that put each ellipse's
/// reach 2% and 26% past the cut's. Each is served the ellipse, on both
/// surfaces, reaching where the frame-free oracle says.
#[test]
fn the_two_fuzz_poses_serve_the_ellipse_their_tilted_seams_cut() {
    let k = Tol::witness().k();
    let rows = [
        (
            "0xba10fe80e6ee4af9",
            Pose {
                apex: Point3::new(23.762795332329038, 20.963398914069703, 8.915513514291348),
                axis: Vec3::new(0.9719882127425585, -0.1082347229581276, 0.20862444496200339),
                u_ref: Vec3::new(
                    0.07222624563157891,
                    -0.7071433255933591,
                    -0.7033716560330899,
                ),
                alpha: 0.6514115937405218,
                normal: Vec3::new(0.4940378047976908, 0.36015717345544745, 0.791336501015378),
                gap: -4.09628312702132e-12,
            },
            73.16333515552087,
            8.720852666426868e-12,
            2.94385,
        ),
        (
            "0x5a26154859e91c73",
            Pose {
                apex: Point3::new(3.2101647197163223, -2.8721838502192716, 0.250322800061545),
                axis: Vec3::new(0.723075735720933, 0.5098849052708943, -0.4660245313162418),
                u_ref: Vec3::new(0.0858156235023383, -0.7357264881960796, -0.6718200751165216),
                alpha: 0.4783335924238275,
                normal: Vec3::new(
                    -0.018609649589998997,
                    0.9970615769217243,
                    0.07430943929610964,
                ),
                gap: -2.3195229060182773e-13,
            },
            16.7410924109384,
            1.3721426466420017e-12,
            0.0676848,
        ),
    ];
    for (seed, pose, extent, e, cut) in rows {
        let reach = pose.reach().expect("an ellipse");
        // The fuzzer's slack: each side reads a slope to a generator at
        // f64, and the ellipse's reach is `|gap|/slope`.
        let rel = 1e-5 + 64.0 * f64::EPSILON * (1.0 + cut / pose.gap.abs());
        assert!(
            (reach - cut).abs() < rel * cut,
            "{seed}: the oracle's reach is {reach}, the cut's {cut}"
        );
        let got = pose.served(extent, Band::new(e, k * e).unwrap());
        let Ok(PlaneConeSection::TiltedEllipse(c)) = &got else {
            panic!("{seed}: the {reach} m ellipse is the section, got {got:?}");
        };
        let (on_plane, on_cone, far) = pose.residuals(c);
        assert!(
            on_plane < k * e && on_cone < k * e,
            "{seed}: the served ellipse stands {on_plane:e} off the plane, {on_cone:e} off \
             the cone"
        );
        assert!(
            (far - reach).abs() < rel * reach + k * e,
            "{seed}: the served ellipse reaches {far}, the oracle {reach}"
        );
    }
}

/// The cone's half-angle and the plane's lean off its axis for one fuzz
/// draw, `e` the draw's ε and `extent` its extent, from one of two
/// families.
/// - **Around the apex point** (`D < 0`): one draw in six is a
///   near-flat cone cut within the band of its axis' normal, where a gap
///   in the band reaches `|δ|/cos α` and the axis-normal circle is the
///   section past it. The rest place the discriminant, in bands at the
///   extent: mostly just past the band, sometimes inside it or on the
///   line-pair side.
/// - **Around the line outcomes** (`D ≥ 0`): a needle or a near-flat
///   cone opens by a sine or cosine from just past the aperture guard to
///   a twentieth, the rest anywhere. The plane is tangent within the
///   band, along the axis, or leaning to a discriminant on the pair
///   side, from just past the band to steep.
fn apex_draw(g: &mut fuzz::Rng, e: f64, extent: f64) -> (f64, f64) {
    if g.below(5) < 3 {
        if g.below(6) == 0 {
            let alpha = g.range(1.45, 1.56);
            return (alpha, g.range(0.0, 0.5) * e / extent);
        }
        let alpha = g.range(0.02, 1.5);
        let d_bands = match g.below(5) {
            0 => g.range(-1.5, 1.5),
            1 => -10f64.powf(g.range(0.0, 4.0)),
            2 => 10f64.powf(g.range(0.0, 4.0)),
            _ => -g.range(9.0, 14.0),
        };
        let d = (d_bands * e / extent).clamp(-0.999_999, 0.999_999);
        return (alpha, lean_for(alpha, d));
    }
    let thin = 10f64.powf(g.range((11.0 * e / extent).log10(), -1.3));
    let alpha = match g.below(6) {
        0 | 1 => thin.asin(),
        2 => thin.acos(),
        _ => g.range(0.02, 1.5),
    };
    let theta = match g.below(4) {
        0 => lean_for(alpha, (g.range(-1.5, 1.5) * e / extent).clamp(-1.0, 1.0)),
        1 => core::f64::consts::FRAC_PI_2,
        _ => {
            let d = 10f64.powf(g.range(1.0, 6.0)) * e / extent;
            lean_for(alpha, d.min(alpha.sin() * g.range(0.5, 1.0)))
        }
    };
    (alpha, theta)
}

/// **Counterexample search.** Planes through and near the apex of cones
/// from needles through ordinary to near-flat, at random placements,
/// scales, faces and ε ([`apex_draw`]). A contradiction is:
/// - a served apex point whose oracle section reaches past the zero
///   band;
/// - a served conic off the oracle's reach, or off either surface;
/// - a served line outcome whose oracle section starts past the zero
///   band of the apex;
/// - a served line off the plane's band, or off a generator through the
///   apex.
///
/// The outcome counts print on every run.
#[test]
fn apex_outcomes_agree_with_the_oracle() {
    let k = Tol::witness().k();
    let mut g = fuzz::start("pn_apex_reach::apex_outcomes");
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    let mut contradictions = Vec::new();
    for _ in 0..fuzz::scaled(4000) {
        let e = 10f64.powf(g.range(-12.0, -5.0));
        let band = Band::new(e, k * e).unwrap();
        let scale = 10f64.powf(g.range(-1.0, 2.0));
        let unit = |g: &mut fuzz::Rng| {
            Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0)).normalize()
        };
        let axis = unit(&mut g);
        // A seam tilted off square to the axis by up to `1e-10` (no check
        // holds a cone's `u_ref ⊥ axis`), whose tilt the arm must not
        // read into the section.
        let tilt = if g.below(2) == 0 {
            0.0
        } else {
            g.range(-1.0, 1.0) * 10f64.powf(g.range(-16.0, -10.0))
        };
        let seam = axis.cross(unit(&mut g)).normalize();
        let u_ref = seam + axis * tilt;
        let extent = scale * g.range(0.5, 4.0);
        let (alpha, theta) = apex_draw(&mut g, e, extent);
        if !(0.0..=core::f64::consts::FRAC_PI_2).contains(&theta) {
            continue;
        }
        let azimuth = g.range(0.0, core::f64::consts::TAU);
        let radial = seam * azimuth.cos() + axis.cross(seam) * azimuth.sin();
        // Half the draws face the normal down the axis (`axis·normal < 0`).
        let facing = if g.below(2) == 0 { 1.0 } else { -1.0 };
        let normal = (axis * theta.cos() + radial * theta.sin()).normalize() * facing;
        let gap = if g.below(8) == 0 {
            0.0
        } else {
            g.range(-1.2, 1.2) * e
        };
        let pose = Pose {
            apex: Point3::new(
                g.range(-1.0, 1.0) * scale,
                g.range(-1.0, 1.0) * scale,
                g.range(-1.0, 1.0) * scale,
            ),
            axis,
            u_ref,
            alpha,
            normal,
            gap,
        };
        let got = pose.served(extent, band);
        let label = || {
            format!(
                "ε {e:e}, α {alpha}, θ {theta}, extent {extent}, gap {:.3}·ε, tilt {tilt:e}, \
                 reach {:?}, starts {:?}",
                gap / e,
                pose.reach(),
                pose.starts()
            )
        };
        // The oracle and the kernel each read a slope to a generator at
        // f64, so each carries `f64::EPSILON/slope` of relative error in a
        // distance `|gap|/slope`.
        let rel = |dist: f64| 64.0 * f64::EPSILON * (1.0 + dist / gap.abs().max(f64::MIN_POSITIVE));
        let mut lines: Vec<&Curve3<f64>> = Vec::new();
        let kind = match &got {
            Ok(PlaneConeSection::ApexPoint(_)) => {
                // Every section point within the zero band of the apex
                // (no section at all, or the plane through it).
                if pose.reach().is_some_and(|r| r > e * (1.0 + rel(r))) {
                    contradictions.push(format!("apex point served: {}", label()));
                }
                "apex point".to_owned()
            }
            Ok(PlaneConeSection::TiltedEllipse(c) | PlaneConeSection::AxisNormalCircle(c)) => {
                let (on_plane, on_cone, far) = pose.residuals(c);
                // `residuals` samples the conic at 1024 points, which
                // fall short of its farthest point by under 1e-5 of it.
                let off_reach = pose
                    .reach()
                    .is_none_or(|r| (far - r).abs() > (1e-5 + rel(r)) * r + k * e);
                if on_plane > k * e || on_cone > k * e || off_reach {
                    contradictions.push(format!(
                        "conic {on_plane:e} off the plane, {on_cone:e} off the cone, reaching \
                         {far:e}: {}",
                        label()
                    ));
                }
                if matches!(got, Ok(PlaneConeSection::TiltedEllipse(_))) {
                    "tilted ellipse".to_owned()
                } else {
                    "axis-normal circle".to_owned()
                }
            }
            Ok(PlaneConeSection::ApexLinePair { l1, l2 }) => {
                lines = vec![l1, l2];
                "line pair".to_owned()
            }
            Ok(PlaneConeSection::ApexTangentLine(l)) => {
                lines = vec![l];
                "tangent line".to_owned()
            }
            Ok(PlaneConeSection::OpenConic(s)) => format!("open: {:?}", s.conic()),
            Err(r) => format!(
                "refused: {}",
                format!("{r:?}").split(['(', ' ', '{']).next().unwrap()
            ),
        };
        if !lines.is_empty() {
            let start = line_start(&pose, lines.len() == 2);
            if start.is_none_or(|s| s > e * (1.0 + rel(s))) {
                contradictions.push(format!("{kind} served: {}", label()));
            }
            // The points read at the extent carry the placement's
            // rounding, `scale + extent` per unit.
            let slack = 64.0 * f64::EPSILON * (3.0 * scale + extent);
            for l in &lines {
                let (off_plane, generator) = line_residuals(&pose, l, extent, slack.max(1e-12));
                if off_plane > e * (1.0 + 1e-6) + slack || !generator {
                    contradictions.push(format!(
                        "{kind} {off_plane:e} off the plane, a generator through the apex: \
                         {generator}: {}",
                        label()
                    ));
                }
            }
            if let [Curve3::Line { dir, .. }] = lines[..] {
                // Both azimuths are read off the normal's part square to
                // the axis, so each carries `f64::EPSILON/s` of it.
                let s = pose.axis.cross(pose.normal).norm();
                let off = dir.normalize().cross(pose.grazed()).norm();
                if off > 1e-12 + 64.0 * f64::EPSILON / s {
                    contradictions.push(format!(
                        "{kind} {off:e} rad off the grazed generator: {}",
                        label()
                    ));
                }
            }
        }
        *counts.entry(kind).or_default() += 1;
    }
    eprintln!("apex outcomes: {counts:?} ({})", fuzz::replay());
    assert!(
        contradictions.is_empty(),
        "{} contradictions ({}), first: {}",
        contradictions.len(),
        fuzz::replay(),
        contradictions[0]
    );
}
