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

use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::intersect::{OutsideConic, PlaneConeSection, SectionError, plane_cone_section};
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
        plane_cone_section(&self.plane(), &self.cone(), extent, band)
    }

    /// The unit generator at azimuth `u`.
    fn generator(&self, u: f64) -> Vec3<f64> {
        let v_ref = self.axis.cross(self.u_ref);
        let (su, cu) = u.sin_cos();
        self.axis * self.alpha.cos() + (self.u_ref * cu + v_ref * su) * self.alpha.sin()
    }

    /// The oracle: the farthest the section reaches from the apex, or
    /// `None` where some generator runs parallel to the plane or crosses
    /// it on both nappes' sides (no closed section). The plane meets the
    /// line through generator `g` at `−gap/(g·n)`; the ellipse's farthest
    /// point is on the generator whose `|g·n|` is least.
    fn reach(&self) -> Option<f64> {
        let slope = |u: f64| self.generator(u).dot(self.normal);
        const N: usize = 1024;
        let samples: Vec<f64> = (0..N)
            .map(|i| slope(i as f64 * core::f64::consts::TAU / N as f64))
            .collect();
        let sign = samples[0].signum();
        if samples.iter().any(|s| s.signum() != sign || *s == 0.0) {
            return None;
        }
        let step = core::f64::consts::TAU / N as f64;
        let i = (0..N)
            .min_by(|&i, &j| samples[i].abs().total_cmp(&samples[j].abs()))
            .unwrap();
        let (mut lo, mut hi) = ((i as f64 - 1.0) * step, (i as f64 + 1.0) * step);
        for _ in 0..200 {
            let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
            if slope(m1).abs() < slope(m2).abs() {
                hi = m2;
            } else {
                lo = m1;
            }
        }
        Some(self.realized_gap().abs() / slope(0.5 * (lo + hi)).abs())
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
        let slope = |u: f64| self.generator(u).dot(self.normal);
        const N: usize = 1024;
        let step = core::f64::consts::TAU / N as f64;
        // The slope's extreme on one side (`side` = +1 its maximum, −1
        // its minimum), refined about the best sample.
        let extreme = |side: f64| {
            let i = (0..N)
                .max_by(|&i, &j| {
                    (side * slope(i as f64 * step)).total_cmp(&(side * slope(j as f64 * step)))
                })
                .unwrap();
            let (mut lo, mut hi) = ((i as f64 - 1.0) * step, (i as f64 + 1.0) * step);
            for _ in 0..200 {
                let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
                if side * slope(m1) > side * slope(m2) {
                    hi = m2;
                } else {
                    lo = m1;
                }
            }
            slope(0.5 * (lo + hi))
        };
        let (most, least) = (extreme(1.0), extreme(-1.0));
        let gap = self.realized_gap();
        // The `+axis` nappe (`t > 0`) needs a slope of the sign of `−gap`.
        let start = |needed: f64| {
            let steepest = if needed > 0.0 { most } else { least };
            (steepest * needed > 0.0).then(|| gap.abs() / steepest.abs())
        };
        if gap == 0.0 {
            return (Some(0.0), Some(0.0));
        }
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

/// The poses the band edge is read at, each beside its mirror through
/// the apex's tangent plane (`axis·normal < 0`): a cone of half-angle
/// `π/6` cut by a plane leaning `θ` off its axis, and a near-flat cone
/// (`cos α = 0.9/10.5`) cut across its axis, each `gap` off the apex.
fn edge_poses(tilted: (f64, f64), flat_gap: f64) -> [(&'static str, Pose); 4] {
    let (theta, tilted_gap) = tilted;
    let flat = (0.9_f64 / 10.5).acos();
    let mirror = |mut p: Pose| {
        p.normal = Vec3::new(-p.normal.x, p.normal.y, -p.normal.z);
        p
    };
    let t = || upright(core::f64::consts::FRAC_PI_6, theta, tilted_gap);
    let f = || upright(flat, 0.0, flat_gap);
    [
        ("tilted", t()),
        ("tilted, mirrored", mirror(t())),
        ("flat", f()),
        ("flat, mirrored", mirror(f())),
    ]
}

/// The tilted pose's lean and gap putting its reach at `reach_bands`·ε.
fn tilted_at(reach_bands: f64) -> (f64, f64) {
    let gap = 0.5 * eps();
    let alpha = core::f64::consts::FRAC_PI_6;
    (lean_for(alpha, -gap / (reach_bands * eps())), gap)
}

/// The flat pose's gap putting its reach at `reach_bands`·ε.
fn flat_gap(reach_bands: f64) -> f64 {
    reach_bands * eps() * 0.9 / 10.5
}

/// **A reach inside the band escalates**: neither the apex point nor
/// the section is served where `|δ|/|D|` lies in `(ε, Kε]`.
#[test]
fn a_reach_inside_the_band_escalates() {
    let (e, k) = (eps(), Tol::witness().k());
    for reach_bands in [1.5, 0.95 * k] {
        for (name, pose) in edge_poses(tilted_at(reach_bands), flat_gap(reach_bands)) {
            let reach = pose.reach().expect("an ellipse");
            assert!(
                (reach / e - reach_bands).abs() < 1e-6 * reach_bands,
                "{name} at {reach_bands}·ε: the oracle's reach is {:e}·ε",
                reach / e
            );
            let got = pose.served(1.0, band());
            assert!(
                matches!(
                    &got,
                    Err(SectionError::Escalated(d)) if d.predicate == Some("pn_apex_point_reach")
                ),
                "{name} at {reach_bands}·ε: the reach escalates, got {got:?}"
            );
        }
    }
}

/// **A reach just past the band is the section's**: the flat cone's
/// axis-normal circle is served, on both surfaces and at the oracle's
/// reach. The tilted pose's ellipse has semi-axes inside `Kε` of each
/// other (its semi-major is at most the reach), so the carrier door
/// refuses it typed; it is never the apex point and never an escalation
/// of the reach.
#[test]
fn a_reach_just_past_the_band_is_the_section() {
    let (e, k) = (eps(), Tol::witness().k());
    let reach_bands = 1.05 * k;
    for (name, pose) in edge_poses(tilted_at(reach_bands), flat_gap(reach_bands)) {
        let reach = pose.reach().expect("an ellipse");
        let got = pose.served(1.0, band());
        if name.starts_with("flat") {
            let Ok(PlaneConeSection::AxisNormalCircle(c)) = &got else {
                panic!("{name}: the {reach:e} m circle is the section, got {got:?}");
            };
            let (on_plane, on_cone, far) = pose.residuals(c);
            assert!(
                on_plane < k * e && on_cone < k * e && (far - reach).abs() < 1e-6 * reach,
                "{name}: served {on_plane:e} off the plane, {on_cone:e} off the cone, reaching \
                 {far:e} beside the oracle's {reach:e}"
            );
        } else {
            assert!(
                matches!(got, Err(SectionError::Carrier(_))),
                "{name}: the sliver ellipse's carrier refuses, got {got:?}"
            );
        }
    }
}

/// **A reach just short of the band keeps the apex point.**
#[test]
fn a_reach_just_short_of_the_band_keeps_the_apex_point() {
    let e = eps();
    for (name, pose) in edge_poses(tilted_at(0.95), flat_gap(0.95)) {
        let reach = pose.reach().expect("an ellipse");
        assert!(
            reach > 0.9 * e && reach < e,
            "{name}: the oracle's reach is {:e}·ε",
            reach / e
        );
        let got = pose.served(1.0, band());
        assert!(
            matches!(got, Ok(PlaneConeSection::ApexPoint(_))),
            "{name}: the apex point, got {got:?}"
        );
    }
}

/// **Counterexample search.** Planes through and near the apex of cones
/// at random half-angles, placements, scales and ε: a served apex point
/// whose oracle section reaches past the zero band, or a served conic off
/// the oracle's reach or off either surface, is a contradiction. The
/// outcome counts print on every run.
#[test]
fn apex_lane_outcomes_agree_with_the_oracle() {
    let k = Tol::witness().k();
    let mut g = fuzz::start("pn_apex_reach::apex_lane");
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    let mut refusals = std::collections::BTreeMap::<String, usize>::new();
    let mut contradictions = Vec::new();
    for _ in 0..fuzz::scaled(2400) {
        let e = 10f64.powf(g.range(-12.0, -5.0));
        let band = Band::new(e, k * e).unwrap();
        let scale = 10f64.powf(g.range(-1.0, 2.0));
        let unit = |g: &mut fuzz::Rng| {
            Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0)).normalize()
        };
        let axis = unit(&mut g);
        let side = unit(&mut g);
        let u_ref = (side - axis * side.dot(axis)).normalize();
        let extent = scale * g.range(0.5, 4.0);
        // One draw in six is a near-flat cone cut within the band of
        // its axis' normal, where a gap in the band reaches `|δ|/cos α`
        // and the axis-normal circle is the section past it. The rest
        // place the discriminant, in bands at the extent: mostly just
        // past the band, sometimes inside it or on the line-pair side.
        let (alpha, theta, d) = if g.below(6) == 0 {
            let alpha = g.range(1.45, 1.56);
            let theta = g.range(0.0, 0.5) * e / extent;
            (alpha, theta, -(alpha + theta).cos())
        } else {
            let alpha = g.range(0.02, 1.5);
            let d_bands = match g.below(5) {
                0 => g.range(-1.5, 1.5),
                1 => -10f64.powf(g.range(0.0, 4.0)),
                2 => 10f64.powf(g.range(0.0, 4.0)),
                _ => -g.range(9.0, 14.0),
            };
            let d = (d_bands * e / extent).clamp(-0.999_999, 0.999_999);
            (alpha, lean_for(alpha, d), d)
        };
        if !(0.0..=core::f64::consts::FRAC_PI_2).contains(&theta) {
            continue;
        }
        let d_bands = d * extent / e;
        let azimuth = g.range(0.0, core::f64::consts::TAU);
        let v_ref = axis.cross(u_ref);
        let radial = u_ref * azimuth.cos() + v_ref * azimuth.sin();
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
        let reach = pose.reach();
        let got = pose.served(extent, band);
        let label = || {
            format!(
                "ε {e:e}, α {alpha}, extent {extent}, D·extent {d_bands}·ε, gap {:.3}·ε, \
                 oracle reach {reach:?}",
                gap / e
            )
        };
        let kind = match &got {
            Ok(PlaneConeSection::ApexPoint(_)) => {
                // Every section point within the zero band of the apex
                // (no section at all, or the plane through it).
                if reach.is_some_and(|r| r > e * (1.0 + 1e-6)) {
                    contradictions.push(format!("apex point served: {}", label()));
                }
                "apex point"
            }
            Ok(PlaneConeSection::TiltedEllipse(c) | PlaneConeSection::AxisNormalCircle(c)) => {
                let (on_plane, on_cone, far) = pose.residuals(c);
                // The oracle and the kernel each read the plane's slope to
                // a generator at f64, so each carries `f64::EPSILON/|D|`
                // of relative error in the reach.
                let rel = 1e-3 + 64.0 * f64::EPSILON / d.abs();
                let off_reach = reach.is_none_or(|r| (far - r).abs() > rel * r + k * e);
                if on_plane > k * e || on_cone > k * e || off_reach {
                    contradictions.push(format!(
                        "conic {on_plane:e} off the plane, {on_cone:e} off the cone, reaching \
                         {far:e}: {}",
                        label()
                    ));
                }
                if matches!(got, Ok(PlaneConeSection::TiltedEllipse(_))) {
                    "tilted ellipse"
                } else {
                    "axis-normal circle"
                }
            }
            Ok(PlaneConeSection::ApexLinePair { .. }) => "line pair",
            Ok(PlaneConeSection::ApexTangentLine(_)) => "tangent line",
            Err(e) => {
                let variant = format!("{e:?}");
                *refusals
                    .entry(
                        variant
                            .split(['(', ' ', '{'])
                            .next()
                            .unwrap_or("")
                            .to_owned(),
                    )
                    .or_default() += 1;
                "refused"
            }
        };
        *counts.entry(kind).or_default() += 1;
    }
    eprintln!(
        "apex lane outcomes: {counts:?}, refusals {refusals:?} ({})",
        fuzz::replay()
    );
    assert!(
        contradictions.is_empty(),
        "{} contradictions ({}), first: {}",
        contradictions.len(),
        fuzz::replay(),
        contradictions[0]
    );
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

/// Whether `got` refused naming `conic`.
fn names(got: &Result<PlaneConeSection<f64>, SectionError>, conic: OutsideConic) -> bool {
    matches!(got, Err(e) if e.outside_conic() == Some(conic))
}

/// **The item's needle poses** (`sin α = (K + 2)·ε`, unit extent, the
/// plane `0.9·ε` off the apex): along the axis, `D·extent = (K + 2)·ε`
/// and the hyperbola starts `0.9/(K + 2)` m from the apex on both
/// nappes; tangent, `D = 0`, and the section starts `0.9/(2·(K + 2))`
/// m off. Beside them the same miss at an ordinary aperture: `α = π/6`,
/// `D·extent = (K + 1)·ε`, whose hyperbola starts `ε`-near the apex on
/// one nappe and `0.9/(K + 1)` m off on the other. None is a line
/// outcome; each refuses naming its conic.
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
            OutsideConic::Hyperbola
        } else {
            OutsideConic::Parabola
        };
        assert!(
            names(&got, conic),
            "{name}: the section starts {oracle} m off the apex, so it refuses naming the \
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
/// past the band the section is refused naming its conic, or escalates.
#[test]
fn a_line_outcome_is_served_only_where_its_section_starts_in_the_band() {
    let (e, k) = (eps(), Tol::witness().k());
    let mut served = (0, 0);
    let mut refused = 0;
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
                        Err(r) if r.outside_conic().is_some() => {
                            refused += 1;
                            continue;
                        }
                        Ok(_) | Err(_) => continue,
                    };
                    let pair = lines.len() == 2;
                    let start = line_start(&pose, pair);
                    assert!(
                        start.is_some_and(|s| s <= e * (1.0 + 1e-3)),
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
        served.0 > 0 && served.1 > 0 && refused > 0,
        "the family served {served:?} (pairs, tangents) and refused {refused} by conic"
    );
}

/// The line poses the band edge is read at, each with its section
/// starting `start_bands·ε` from the apex beside a gap of `±½·ε`, and
/// whether each is a pair: the plane along the axis (`sin α` the
/// slope), a plane leaning off a `π/4` cone to the discriminant that
/// puts the far nappe's start there, and the tangent plane to a cone and
/// to its near-flat complement (`sin 2α` the slope); each leaning pose
/// beside its mirror (`axis·normal < 0`).
fn edge_lines(start_bands: f64, gap_sign: f64) -> Vec<(&'static str, Pose, bool)> {
    let gap = gap_sign * 0.5 * eps();
    let slope = 0.5 / start_bands;
    let quarter = core::f64::consts::FRAC_PI_4;
    let steep = || upright(quarter, lean_for(quarter, slope), gap);
    let narrow = 0.5 * slope.asin();
    let flat = core::f64::consts::FRAC_PI_2 - narrow;
    vec![
        ("pair along the axis", along_axis(slope.asin(), gap), true),
        ("pair leaning", steep(), true),
        ("pair leaning, mirrored", mirrored(steep()), true),
        ("tangent", tangent(narrow, gap), false),
        ("tangent, mirrored", mirrored(tangent(narrow, gap)), false),
        ("tangent, near-flat", tangent(flat, gap), false),
        (
            "tangent, near-flat, mirrored",
            mirrored(tangent(flat, gap)),
            false,
        ),
    ]
}

/// Every edge pose at `start_bands`, its oracle start checked, with the
/// section `plane_cone_section` serves it.
fn at_the_edge(
    start_bands: f64,
) -> Vec<(String, bool, Result<PlaneConeSection<f64>, SectionError>)> {
    let e = eps();
    let mut out = Vec::new();
    for gap_sign in [1.0, -1.0] {
        for (name, pose, pair) in edge_lines(start_bands, gap_sign) {
            let label = format!("{name}, gap {}·½ε at {start_bands}·ε", gap_sign);
            let start = line_start(&pose, pair).expect("the section starts somewhere");
            assert!(
                (start / e - start_bands).abs() < 1e-6 * start_bands,
                "{label}: the oracle's section starts {:e}·ε off",
                start / e
            );
            let got = pose.served(1.0, band());
            out.push((label, pair, got));
        }
    }
    out
}

/// **A section starting inside the band escalates**: neither a line
/// outcome nor the conic is served where the start lies in `(ε, Kε]`.
#[test]
fn a_line_start_inside_the_band_escalates() {
    let k = Tol::witness().k();
    for start_bands in [1.5, 0.95 * k] {
        for (label, pair, got) in at_the_edge(start_bands) {
            let row = if pair {
                "pn_apex_pair_reach"
            } else {
                "pn_apex_tangent_reach"
            };
            assert!(
                matches!(&got, Err(SectionError::Escalated(d)) if d.predicate == Some(row)),
                "{label}: escalates on {row}, got {got:?}"
            );
        }
    }
}

/// **A section starting just past the band is the conic's**: the pair's
/// hyperbola and the tangent's near-parabola each refuse naming it.
#[test]
fn a_line_start_just_past_the_band_is_the_conic() {
    let k = Tol::witness().k();
    for (label, pair, got) in at_the_edge(1.05 * k) {
        let conic = if pair {
            OutsideConic::Hyperbola
        } else {
            OutsideConic::Parabola
        };
        assert!(
            names(&got, conic),
            "{label}: refuses naming the {conic:?}, got {got:?}"
        );
    }
}

/// **A section starting just short of the band keeps its line outcome.**
#[test]
fn a_line_start_just_short_of_the_band_keeps_the_lines() {
    for (label, pair, got) in at_the_edge(0.95) {
        let kept = if pair {
            matches!(got, Ok(PlaneConeSection::ApexLinePair { .. }))
        } else {
            matches!(got, Ok(PlaneConeSection::ApexTangentLine(_)))
        };
        assert!(kept, "{label}: the line outcome, got {got:?}");
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

/// **Counterexample search for the line outcomes.** Planes through and
/// near the apex of cones from needles (`sin α` down to just past the
/// aperture guard) through ordinary to near-flat, along the axis,
/// tangent or leaning to a discriminant on the line-pair side, both
/// faces, at random placements, scales and ε. A served line outcome
/// whose oracle section starts past the zero band of the apex, or a
/// served line off the plane's band or off a generator through the
/// apex, is a contradiction. The outcome counts print on every run.
#[test]
fn apex_lines_agree_with_the_oracle() {
    let k = Tol::witness().k();
    let mut g = fuzz::start("pn_apex_reach::apex_lines");
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    let mut contradictions = Vec::new();
    for _ in 0..fuzz::scaled(1600) {
        let e = 10f64.powf(g.range(-12.0, -5.0));
        let band = Band::new(e, k * e).unwrap();
        let scale = 10f64.powf(g.range(-1.0, 2.0));
        let unit = |g: &mut fuzz::Rng| {
            Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0)).normalize()
        };
        let axis = unit(&mut g);
        let side = unit(&mut g);
        let u_ref = (side - axis * side.dot(axis)).normalize();
        let extent = scale * g.range(0.5, 4.0);
        // A needle or a near-flat cone opens by a sine or cosine from
        // just past the aperture guard to a twentieth; the rest anywhere.
        let thin = 10f64.powf(g.range((11.0 * e / extent).log10(), -1.3));
        let alpha = match g.below(6) {
            0 | 1 => thin.asin(),
            2 => thin.acos(),
            _ => g.range(0.02, 1.5),
        };
        // Tangent within the band, along the axis, or leaning to a
        // discriminant on the pair side, from just past the band to steep.
        let theta = match g.below(4) {
            0 => lean_for(alpha, (g.range(-1.5, 1.5) * e / extent).clamp(-1.0, 1.0)),
            1 => core::f64::consts::FRAC_PI_2,
            _ => {
                let d = 10f64.powf(g.range(1.0, 6.0)) * e / extent;
                lean_for(alpha, d.min(alpha.sin() * g.range(0.5, 1.0)))
            }
        };
        if !(0.0..=core::f64::consts::FRAC_PI_2).contains(&theta) {
            continue;
        }
        let azimuth = g.range(0.0, core::f64::consts::TAU);
        let v_ref = axis.cross(u_ref);
        let radial = u_ref * azimuth.cos() + v_ref * azimuth.sin();
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
                "ε {e:e}, α {alpha}, θ {theta}, extent {extent}, gap {:.3}·ε, starts {:?}",
                gap / e,
                pose.starts()
            )
        };
        let (kind, lines): (String, Vec<&Curve3<f64>>) = match &got {
            Ok(PlaneConeSection::ApexLinePair { l1, l2 }) => ("line pair".into(), vec![l1, l2]),
            Ok(PlaneConeSection::ApexTangentLine(l)) => ("tangent line".into(), vec![l]),
            Ok(other) => (
                format!("{other:?}").split('(').next().unwrap().into(),
                vec![],
            ),
            Err(r) => match r.outside_conic() {
                Some(conic) => (format!("refused: {conic:?}"), vec![]),
                None => (
                    format!(
                        "refused: {}",
                        format!("{r:?}").split(['(', ' ', '{']).next().unwrap()
                    ),
                    vec![],
                ),
            },
        };
        if !lines.is_empty() {
            let pair = lines.len() == 2;
            let start = line_start(&pose, pair);
            // The oracle and the kernel each read the steepest slope at
            // f64, so each carries `f64::EPSILON/slope` relative error,
            // `slope = |gap|/start`, in the start.
            let rel = |s: f64| 1e-3 + 64.0 * f64::EPSILON * s / gap.abs().max(f64::MIN_POSITIVE);
            if start.is_none_or(|s| s > e * (1.0 + rel(s))) {
                contradictions.push(format!("{kind} served: {}", label()));
            }
            // The points read at the extent carry the placement's
            // rounding, `scale + extent` per unit.
            let slack = 64.0 * f64::EPSILON * (3.0 * scale + extent);
            for l in lines {
                let (off_plane, generator) = line_residuals(&pose, l, extent, slack.max(1e-12));
                if off_plane > e * (1.0 + 1e-6) + slack || !generator {
                    contradictions.push(format!(
                        "{kind} {off_plane:e} off the plane, a generator through the apex: \
                         {generator}: {}",
                        label()
                    ));
                }
            }
        }
        *counts.entry(kind).or_default() += 1;
    }
    eprintln!("apex line outcomes: {counts:?} ({})", fuzz::replay());
    assert!(
        contradictions.is_empty(),
        "{} contradictions ({}), first: {}",
        contradictions.len(),
        fuzz::replay(),
        contradictions[0]
    );
}
