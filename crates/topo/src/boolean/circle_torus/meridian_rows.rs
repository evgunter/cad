//! **The circle × torus door's meridian rung**
//! ([`super::meridian_deviation`], `bool_circle_torus_meridian`), held
//! to an independent oracle: each carrier is sampled densely and every
//! sample's distance from the torus is read in the torus's own frame,
//! `|√((ρ_axis − R)² + h²) − r|`, without the door's residual or its
//! bound.
//!
//! The rows: a meridian at any pose and scale lies on the torus; a
//! near-meridian off by one condition reads at its deviation (on within
//! the band, an escalation in it, the ladder past it); a circle with no
//! zero of `F` stays off; and a circle whose axis is the torus's is
//! never a meridian.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

test_utils::gated_to![
    "crates/topo/src/boolean/circle_roots.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/linalg/",
];

use super::*;
use geom_core::Tol;

/// A torus as the door reads it, and the frame the oracle reads it in.
#[derive(Clone, Copy)]
struct Torus {
    c: Point3<f64>,
    a: Vec3<f64>,
    big: f64,
    small: f64,
}

impl Torus {
    fn surface(self) -> geom::Surface<f64> {
        geom::Surface::Torus {
            center: self.c,
            axis: self.a,
            major_radius: self.big,
            minor_radius: self.small,
            u_ref: unit_perp(self.a),
        }
    }

    /// The oracle: a point's distance from the torus.
    fn distance(self, p: Point3<f64>) -> f64 {
        let w = p - self.c;
        let h = w.dot(self.a);
        let rho = (w - self.a * h).norm();
        ((rho - self.big).powi(2) + h * h).sqrt() - self.small
    }

    /// The meridian at azimuth direction `e` (unit, ⟂ the axis), turned
    /// by `phase` in its own plane.
    fn meridian(self, e: Vec3<f64>, phase: f64) -> Circle {
        let t = self.a.cross(e);
        let (s, c) = phase.sin_cos();
        Circle {
            c: self.c + e * self.big,
            n: t,
            rho: self.small,
            u: e * c + self.a * s,
        }
    }
}

/// A circle carrier: centre, unit axis, radius, unit `u ⟂ n`.
#[derive(Clone, Copy)]
struct Circle {
    c: Point3<f64>,
    n: Vec3<f64>,
    rho: f64,
    u: Vec3<f64>,
}

impl Circle {
    fn carrier(self) -> geom::Curve3<f64> {
        geom::Curve3::Circle {
            center: self.c,
            axis: self.n,
            radius: self.rho,
            u_ref: self.u,
        }
    }

    /// The oracle's reading: the largest and smallest distance from the
    /// torus over a dense sample of the whole circle.
    fn reach(self, torus: Torus) -> (f64, f64) {
        let v = self.n.cross(self.u);
        let samples = 4096;
        (0..samples).fold((0.0_f64, f64::INFINITY), |(hi, lo), i| {
            let th = core::f64::consts::TAU * f64::from(i) / f64::from(samples);
            let p = self.c + (self.u * th.cos() + v * th.sin()) * self.rho;
            let d = torus.distance(p).abs();
            (hi.max(d), lo.min(d))
        })
    }
}

fn unit_perp(a: Vec3<f64>) -> Vec3<f64> {
    let k = if a.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    a.cross(k).normalize()
}

fn door(circle: Circle, torus: Torus, band: Band) -> Result<CircleRoots<f64>, BooleanError> {
    circle_torus_roots(&circle.carrier(), 0.3, 2.1, &torus.surface(), band)
}

fn meridian_escalation(got: &Result<CircleRoots<f64>, BooleanError>) -> bool {
    matches!(
        got,
        Err(BooleanError::Escalated { diag, .. })
            if diag.predicate == Some("bool_circle_torus_meridian")
    )
}

/// The unit torus the enumerations read, at the band they are claims
/// about.
fn unit_torus() -> Torus {
    Torus {
        c: Point3::new(0.0, 0.0, 0.0),
        a: Vec3::new(0.0, 0.0, 1.0),
        big: 1.0,
        small: 0.25,
    }
}

fn fixed_band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// **A meridian lies on the torus at any pose and scale**: tori of
/// major radius about 1e-3, 1 and 1e3, centred at the origin or 1e3
/// from it, their axes, azimuths and the carrier's phase drawn at
/// random. Each reads `OnSurface`.
///
/// The band is the run's ε, raised to the model's own rounding floor
/// (`1e-13` per metre of the farthest coordinate): a model 3e3 from the
/// origin cannot place a point closer than that, and below it the row
/// would measure the f64 grid, not the rung.
#[test]
fn a_meridian_at_any_pose_and_scale_lies_on_the_torus() {
    use test_utils::fuzz;
    let mut rng = fuzz::start("boolean::circle_torus::meridian_poses");
    let per_cell = fuzz::scaled(8);
    let unit = |rng: &mut fuzz::Rng| loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.norm();
        if (1e-3..=1.0).contains(&n) {
            break v / n;
        }
    };
    for scale in [1e-3, 1.0, 1e3] {
        for offset in [0.0, 1e3] {
            let eps = Tol::witness().eps().max(1e-13 * (offset + 3.0 * scale));
            let band = Band::linear_at(Tol::witness(), eps).unwrap();
            for i in 0..per_cell {
                let a = unit(&mut rng);
                let big = scale * rng.range(1.0, 2.0);
                let torus = Torus {
                    c: Point3::new(0.0, 0.0, 0.0) + unit(&mut rng) * offset,
                    a,
                    big,
                    small: big * rng.range(0.05, 0.8),
                };
                let e = a.cross(unit(&mut rng)).normalize();
                let m = torus.meridian(e, rng.range(0.0, core::f64::consts::TAU));
                let (hi, _) = m.reach(torus);
                let label = format!("scale {scale}, offset {offset}, draw {i}");
                assert!(
                    hi <= eps,
                    "{label}: the drawn meridian is {hi:e} off the torus"
                );
                let got = door(m, torus, band);
                assert!(
                    matches!(got, Ok(CircleRoots::OnSurface)),
                    "{label}: a meridian lies on the torus, read {got:?} — {}",
                    fuzz::replay()
                );
            }
        }
    }
}

/// One condition of a meridian failing by a deviation of `k` bands, as
/// the oracle measures it.
#[derive(Clone, Copy, Debug)]
enum Fault {
    /// The centre moved off the centre circle, radially.
    Centre,
    /// The radius off the minor radius.
    Radius,
    /// The plane turned about the carrier's diameter along the torus
    /// axis: the worst way to turn it, and a second-order cost, so the
    /// angle is `√(2·kε·(R − r)/(rR))`.
    Tilt,
}

fn near_meridian(torus: Torus, fault: Fault, deviation: f64) -> Circle {
    let e = Vec3::new(0.6, 0.8, 0.0);
    let m = torus.meridian(e, 0.4);
    match fault {
        Fault::Centre => Circle {
            c: m.c + e * deviation,
            ..m
        },
        Fault::Radius => Circle {
            rho: m.rho + deviation,
            ..m
        },
        Fault::Tilt => {
            let (big, small) = (torus.big, torus.small);
            let sigma = (2.0 * deviation * (big - small) / (small * big)).sqrt();
            let turn = |v: Vec3<f64>| {
                let along = torus.a * v.dot(torus.a);
                let rest = v - along;
                along + rest * (1.0 - sigma * sigma).sqrt() + torus.a.cross(rest) * sigma
            };
            Circle {
                n: turn(m.n),
                u: turn(m.u),
                ..m
            }
        }
    }
}

/// **A near-meridian reads at its deviation.** Each condition failing
/// alone, at 0.5, 1, 2 and 20 bands as the oracle measures: within the
/// band it lies on the torus, at the band's edge it lies on it or
/// escalates, in the band it escalates as the meridian rung, and past
/// the band the rung passes it to the ladder. Whatever it reads, an
/// `OnSurface` is never more than the band off by the oracle.
#[test]
fn a_near_meridian_reads_at_its_deviation() {
    let (torus, band) = (unit_torus(), fixed_band());
    let eps = band.zero();
    for fault in [Fault::Centre, Fault::Radius, Fault::Tilt] {
        for k in [0.5, 1.0, 2.0, 20.0] {
            let circle = near_meridian(torus, fault, k * eps);
            let (hi, _) = circle.reach(torus);
            let label = format!("{fault:?} at {k} bands (the oracle reads {hi:e})");
            assert!(
                (hi - k * eps).abs() <= 0.02 * k * eps,
                "{label}: the fixture's deviation"
            );
            let got = door(circle, torus, band);
            if matches!(got, Ok(CircleRoots::OnSurface)) {
                assert!(
                    hi <= eps * (1.0 + 1e-6),
                    "{label}: on the torus, by the oracle"
                );
            }
            let fine = if k < 1.0 {
                matches!(got, Ok(CircleRoots::OnSurface))
            } else if k == 1.0 {
                matches!(got, Ok(CircleRoots::OnSurface)) || meridian_escalation(&got)
            } else if k < 10.0 {
                meridian_escalation(&got)
            } else {
                !matches!(got, Ok(CircleRoots::OnSurface)) && !meridian_escalation(&got)
            };
            assert!(fine, "{label}: read {got:?}");
        }
    }
}

/// **A circle with no zero of `F` stays off.** Off-axis, tilted, and
/// clear of the tube everywhere by the oracle; and a meridian-plane
/// circle about a centre-circle point, a tenth of the tube outside it.
/// Neither is read on the torus, and each is a certified miss.
#[test]
fn a_circle_clear_of_the_torus_stays_off() {
    let (torus, band) = (unit_torus(), fixed_band());
    let tilted = Circle {
        c: Point3::new(2.5, 0.4, 0.6),
        n: Vec3::new(0.3, -0.5, 0.8).normalize(),
        rho: 0.4,
        u: Vec3::new(0.3, -0.5, 0.8)
            .normalize()
            .cross(Vec3::new(0.0, 0.0, 1.0))
            .normalize(),
    };
    let wide = Circle {
        rho: 0.35,
        ..torus.meridian(Vec3::new(0.6, 0.8, 0.0), 0.4)
    };
    for (label, circle) in [("tilted", tilted), ("wide", wide)] {
        let (_, lo) = circle.reach(torus);
        assert!(
            lo > 1e3 * band.escalate(),
            "{label}: clear by the oracle ({lo:e})"
        );
        let got = door(circle, torus, band);
        assert!(
            matches!(got, Ok(CircleRoots::Miss)),
            "{label}: read {got:?}"
        );
    }
}

/// **A circle whose axis is the torus's is never a meridian**, wherever
/// its centre sits: centred on the axis (a coaxial carrier), the bound
/// is at least `R`; centred on the centre circle at the tube's radius
/// (the parallel pose's), its plane is a quarter turn from the
/// meridian's, which costs at least the tube radius.
#[test]
fn a_circle_on_the_torus_axis_is_never_a_meridian() {
    let t = unit_torus();
    let deviation = |c: Point3<f64>, rho: f64| {
        let w = c - t.c;
        let h0 = w.dot(t.a);
        let w_perp = w - t.a * h0;
        meridian_deviation(t.a, rho, w_perp, w_perp.norm(), h0, t.a, t.big, t.small)
    };
    for (label, c, rho, floor) in [
        (
            "coaxial rim",
            Point3::new(0.0, 0.0, 0.0),
            t.big + t.small,
            t.big,
        ),
        (
            "coaxial at the top",
            Point3::new(0.0, 0.0, t.small),
            t.big,
            t.big,
        ),
        (
            "parallel at a core point",
            Point3::new(0.6, 0.8, 0.0),
            t.small,
            t.small,
        ),
    ] {
        let got = deviation(c, rho);
        assert!(got >= floor, "{label}: deviation {got:e} under {floor:e}");
    }
}
