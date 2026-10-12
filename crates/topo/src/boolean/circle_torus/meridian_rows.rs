//! **The circle × torus door's meridian rung**
//! ([`super::meridian_deviation`], `bool_circle_torus_meridian`), held
//! to an independent oracle: each carrier is sampled densely and every
//! sample's distance from the torus is read in the torus's own frame,
//! `|√((ρ_axis − R)² + h²) − r|`, without the door's residual or its
//! bound.
//!
//! The rows:
//!
//! - at any pose, scale, band and tube, a meridian lies on the torus,
//!   and no carrier more than the band off reads on it;
//! - a near-meridian off by one condition reads at its deviation (on
//!   within the band, an escalation in it, the ladder past it);
//! - a circle with no zero of `F` stays off;
//! - a circle whose axis is the torus's is never a meridian;
//! - a spindle torus takes no meridian rung.
//!
//! Its fixtures pose the torus as well as the carrier, so they keep
//! their own vocabulary rather than the tests module's circle poses
//! against one torus at the origin.

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
        self.reach_at(torus, 4096)
    }

    fn reach_at(self, torus: Torus, samples: u32) -> (f64, f64) {
        let v = self.n.cross(self.u);
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

/// A rotation by `angle` about the unit `axis` (Rodrigues).
fn rotate(v: Vec3<f64>, axis: Vec3<f64>, angle: f64) -> Vec3<f64> {
    let (s, c) = angle.sin_cos();
    v * c + axis.cross(v) * s + axis * (axis.dot(v) * (1.0 - c))
}

/// The faults a carrier near a meridian carries, as multiples of one
/// size `t`: the centre moved radially and along the axis, the radius
/// off, and the plane turned about a diameter mixed from the axis and
/// radial directions by a second-order angle, so every fault's
/// deviation grows about linearly in `t`.
#[derive(Clone, Copy, Debug)]
struct Faults {
    radial: f64,
    vertical: f64,
    radius: f64,
    tilt: f64,
    /// The turning diameter's share along the torus axis (the rest is
    /// radial).
    tilt_axial: f64,
}

impl Faults {
    /// The faulted carrier about the meridian at azimuth `e` of the
    /// unit-axis torus at the origin, in that torus's frame.
    fn carrier(self, torus: Torus, e: Vec3<f64>, phase: f64, t: f64) -> Circle {
        let m = torus.meridian(e, phase);
        let (big, small) = (torus.big, torus.small);
        let angle = self.tilt * (2.0 * t * (big - small) / (small * big)).sqrt();
        let turn = (torus.a * self.tilt_axial + e * (1.0 - self.tilt_axial.abs())).normalize();
        Circle {
            c: m.c + e * (self.radial * t) + torus.a * (self.vertical * t),
            n: rotate(m.n, turn, angle),
            rho: m.rho + self.radius * t,
            u: rotate(m.u, turn, angle),
        }
    }
}

/// The posed copy of a torus and a carrier built in its frame: turned
/// by `(axis, angle)` and moved to `at`.
fn posed(
    torus: Torus,
    circle: Circle,
    (axis, angle): (Vec3<f64>, f64),
    at: Vec3<f64>,
) -> (Torus, Circle) {
    let place = |p: Point3<f64>| Point3::new(0.0, 0.0, 0.0) + at + rotate(p - torus.c, axis, angle);
    (
        Torus {
            c: place(torus.c),
            a: rotate(torus.a, axis, angle),
            ..torus
        },
        Circle {
            c: place(circle.c),
            n: rotate(circle.n, axis, angle),
            u: rotate(circle.u, axis, angle),
            ..circle
        },
    )
}

/// **The meridian rung at any pose, scale, band and tube**, held to the
/// oracle in both directions. For every torus of major radius 1e-3, 1
/// or 1e3, centred at the origin or 1e3 from it, tube ratio 0.05 to
/// 0.99, under bands of 1e-9, 1e-6 and 1e-12:
///
/// - an exact meridian at a random azimuth and phase reads `OnSurface`;
/// - a carrier faulted from it, one fault or all five at once, sized by
///   bisection to lie 1.02 to 1.5 bands off the torus by the oracle,
///   never reads `OnSurface`.
///
/// The second half is what goes red on an unsound rung: a term short of
/// a fault's point deviation serves such a carrier. Each carrier is
/// built and measured in the torus's frame, where the oracle's f64
/// distance is good to about `1e-16` of the extent, and then posed. The
/// band is raised to the posed model's own rounding floor (`1e-13` per
/// metre of the farthest coordinate): a model 3e3 from the origin cannot
/// place a point closer than that, and below it the row would measure
/// the f64 grid, not the rung.
#[test]
fn the_meridian_rung_at_any_pose_scale_and_band() {
    use test_utils::fuzz;
    let mut rng = fuzz::start("boolean::circle_torus::meridian_poses");
    let per_cell = fuzz::scaled(1);
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
    let mixes: [(&str, [f64; 4]); 6] = [
        ("radial centre", [1.0, 0.0, 0.0, 0.0]),
        ("vertical centre", [0.0, 1.0, 0.0, 0.0]),
        ("radius", [0.0, 0.0, 1.0, 0.0]),
        ("tilt", [0.0, 0.0, 0.0, 1.0]),
        ("all at once", [1.0, 1.0, 1.0, 1.0]),
        ("all, mixed", [0.0; 4]),
    ];
    let mut faulted = 0;
    for eps in [1e-9_f64, 1e-6, 1e-12] {
        for scale in [1e-3, 1.0, 1e3] {
            for offset in [0.0, 1e3] {
                let eps = eps.max(1e-13 * (offset + 3.0 * scale));
                let band = Band::linear_at(Tol::witness(), eps).unwrap();
                for ratio in [0.05, 0.5, 0.9, 0.99] {
                    for (name, weights) in mixes {
                        for i in 0..per_cell {
                            let local = Torus {
                                c: Point3::new(0.0, 0.0, 0.0),
                                a: Vec3::new(0.0, 0.0, 1.0),
                                big: scale,
                                small: scale * ratio,
                            };
                            let az = rng.range(0.0, core::f64::consts::TAU);
                            let e = Vec3::new(az.cos(), az.sin(), 0.0);
                            let phase = rng.range(0.0, core::f64::consts::TAU);
                            let sign =
                                |rng: &mut fuzz::Rng| if rng.unit() < 0.5 { -1.0 } else { 1.0 };
                            let faults = if name == "all, mixed" {
                                Faults {
                                    radial: rng.range(-1.0, 1.0),
                                    vertical: rng.range(-1.0, 1.0),
                                    radius: rng.range(-1.0, 1.0),
                                    tilt: rng.range(0.0, 1.0),
                                    tilt_axial: rng.range(-1.0, 1.0),
                                }
                            } else {
                                Faults {
                                    radial: weights[0] * sign(&mut rng),
                                    vertical: weights[1] * sign(&mut rng),
                                    radius: weights[2] * sign(&mut rng),
                                    tilt: weights[3],
                                    // The worst turn (about the axis) half
                                    // the time, any diameter otherwise.
                                    tilt_axial: if rng.unit() < 0.5 {
                                        1.0
                                    } else {
                                        rng.range(-1.0, 1.0)
                                    },
                                }
                            };
                            let turn = (unit(&mut rng), rng.range(0.0, core::f64::consts::TAU));
                            let at = unit(&mut rng) * offset;
                            let label = format!(
                                "ε {eps:e}, scale {scale}, offset {offset}, r/R {ratio}, {name} \
                                 {faults:?}, draw {i}"
                            );

                            let exact = faults.carrier(local, e, phase, 0.0);
                            let (torus, circle) = posed(local, exact, turn, at);
                            let got = door(circle, torus, band);
                            assert!(
                                matches!(got, Ok(CircleRoots::OnSurface)),
                                "{label}: a meridian lies on the torus, read {got:?} — {}",
                                fuzz::replay()
                            );

                            // Bisect the fault size onto a target deviation.
                            let target = eps * rng.range(1.02, 1.5);
                            let off =
                                |t: f64| faults.carrier(local, e, phase, t).reach_at(local, 256).0;
                            let mut hi = eps;
                            while off(hi) < target {
                                hi *= 2.0;
                            }
                            let mut lo = 0.0;
                            for _ in 0..28 {
                                let mid = 0.5 * (lo + hi);
                                if off(mid) < target {
                                    lo = mid;
                                } else {
                                    hi = mid;
                                }
                            }
                            let carrier = faults.carrier(local, e, phase, hi);
                            let dev = carrier.reach(local).0;
                            assert!(dev > 1.01 * eps, "{label}: the fixture's deviation {dev:e}");
                            let (torus, circle) = posed(local, carrier, turn, at);
                            let got = door(circle, torus, band);
                            assert!(
                                !matches!(got, Ok(CircleRoots::OnSurface)),
                                "{label}: {:.3} bands off the torus by the oracle, read on it — {}",
                                dev / eps,
                                fuzz::replay()
                            );
                            faulted += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{faulted} faulted carriers held off the torus");
}

/// **A torus off D3's ring convention takes the ladder**: on a spindle
/// torus (`r = 1.2R`, `1.5R`) the meridian bound does not hold
/// (`R − r` is negative, and so is its `η`), so neither the meridian nor
/// a tilted one reads `OnSurface` from the rung, and neither reaches the
/// invariant a negative deviation raises. The tilted one is off the
/// spindle torus by the oracle.
#[test]
fn a_spindle_torus_takes_no_meridian_rung() {
    let band = fixed_band();
    for ratio in [1.2, 1.5] {
        let torus = Torus {
            small: ratio,
            ..unit_torus()
        };
        let e = Vec3::new(0.6, 0.8, 0.0);
        let plain = torus.meridian(e, 0.4);
        let tilted = Circle {
            n: rotate(plain.n, torus.a, 1e-3),
            u: rotate(plain.u, torus.a, 1e-3),
            ..plain
        };
        assert!(
            tilted.reach(torus).0 > 1e3 * band.escalate(),
            "r/R {ratio}: tilted is off"
        );
        for (label, circle) in [("plain", plain), ("tilted", tilted)] {
            let got = door(circle, torus, band);
            assert!(
                !matches!(
                    got,
                    Ok(CircleRoots::OnSurface) | Err(BooleanError::ClassificationInvariant { .. })
                ),
                "r/R {ratio}, {label}: read {got:?}"
            );
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
