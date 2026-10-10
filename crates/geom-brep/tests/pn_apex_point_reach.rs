//! **Plane × cone's apex point stands in for a section only inside the
//! band.** A plane a hair off the apex (its gap Zero) whose plane through
//! the apex clears the cone (`D` definitely negative) still cuts an
//! ellipse, and that ellipse reaches `|δ|/|D|` from the apex — at a
//! discriminant just past the band, the scale of the extent. The oracle
//! here never reads `D`: it walks the generators, finds the shallowest
//! against the plane (sampled, then refined by ternary search), and
//! takes the section's reach as the gap over that generator's slope.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/intersect.rs",
    "crates/geom-brep/tests/shared/"
];

use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::intersect::{PlaneConeSection, SectionError, plane_cone_section};
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

/// **Counterexample search.** Planes through and near the apex of cones
/// at random half-angles, placements, scales and ε: a served apex point
/// whose oracle section reaches past the zero band, or a served conic off
/// the oracle's reach or off either surface, is a contradiction. The
/// outcome counts print on every run.
#[test]
fn apex_lane_outcomes_agree_with_the_oracle() {
    let k = Tol::witness().k();
    let mut g = fuzz::start("pn_apex_point_reach::apex_lane");
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    let mut refusals = std::collections::BTreeMap::<String, usize>::new();
    let mut contradictions = Vec::new();
    for _ in 0..fuzz::scaled(3000) {
        let e = 10f64.powf(g.range(-12.0, -5.0));
        let band = Band::new(e, k * e).unwrap();
        let scale = 10f64.powf(g.range(-1.0, 2.0));
        let unit = |g: &mut fuzz::Rng| {
            Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0)).normalize()
        };
        let axis = unit(&mut g);
        let side = unit(&mut g);
        let u_ref = (side - axis * side.dot(axis)).normalize();
        let alpha = g.range(0.02, 1.5);
        let extent = scale * g.range(0.5, 4.0);
        // The discriminant, in bands at the extent: mostly just past the
        // band, sometimes inside it or on the line-pair side.
        let d_bands = match g.below(5) {
            0 => g.range(-1.5, 1.5),
            1 => -10f64.powf(g.range(0.0, 4.0)),
            2 => 10f64.powf(g.range(0.0, 4.0)),
            _ => -g.range(9.0, 14.0),
        };
        let d = (d_bands * e / extent).clamp(-0.999_999, 0.999_999);
        let theta = lean_for(alpha, d);
        if !(0.0..=core::f64::consts::FRAC_PI_2).contains(&theta) {
            continue;
        }
        let azimuth = g.range(0.0, core::f64::consts::TAU);
        let v_ref = axis.cross(u_ref);
        let radial = u_ref * azimuth.cos() + v_ref * azimuth.sin();
        let normal = (axis * theta.cos() + radial * theta.sin()).normalize();
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
