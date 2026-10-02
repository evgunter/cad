//! Reviewer probe for PR #3805, second delta review (head e7f2798307).
//!
//! Mounted temporarily as a child of `crates/topo/src/boolean/ellipse_roots.rs`:
//!
//! ```text
//! #[cfg(test)]
//! #[path = "../../../../probes/reach_3805_fuzz.rs"]
//! mod reviewer_probe;
//! ```
//!
//! then `cargo nextest run -p topo reviewer_probe --no-capture`.
//!
//! It drives what `reduce::curved_face_arm` does for an ellipse edge:
//! first the conic CLEARANCE (`reduce::conic_clearance`, restated here
//! line for line because it takes an `EdgeCurve`), and only when that
//! is not definitely positive, the ellipse root door. A definitely
//! positive clearance is a certified "no event", the same claim as a
//! door `Miss`.
//!
//! Poses: an ellipse (semi-axes S·[0.5, 5], S ∈ {1 m, 1 km}, eccentricity
//! up to 40, all 8 combinations of stored order and sign), centre up to
//! 1 km out, grazed at a random one of its four vertices by a sphere, a
//! wall, a torus or a cone set off along a normal of the carrier at the
//! vertex (in-plane outward or tilted out of plane) by `gap` = ε·[−40, 40].
//! Oracle: the construction puts the carrier's least TRUE distance from
//! the surface at the vertex and equal to `gap` (closed form), and a
//! dense f64 sampling of the true distance re-checks that no other point
//! is closer (the case is skipped otherwise). Nothing the kernel computes
//! is used as the oracle.

use core::f64::consts::FRAC_PI_2;

use super::*;
use geom_core::{Point3, Vec3};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn below(&mut self, n: u32) -> u32 {
        (self.next() * f64::from(n)) as u32 % n
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
            );
            if v.norm() > 0.2 && v.norm() < 1.0 {
                return v.normalize();
            }
        }
    }
}

fn distance(s: &geom::Surface<f64>, p: Point3<f64>) -> f64 {
    match *s {
        geom::Surface::Sphere { center, radius, .. } => (p - center).norm() - radius,
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let w = p - origin;
            (w - axis * w.dot(axis)).norm() - radius
        }
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let w = p - center;
            let h = w.dot(axis);
            let rho = (w - axis * h).norm();
            ((rho - major_radius).powi(2) + h * h).sqrt() - minor_radius
        }
        geom::Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let w = p - apex;
            let h = w.dot(axis);
            let rho = (w - axis * h).norm();
            // distance to the (double) cone's generator in the meridian
            // plane, signed outward
            rho * half_angle.cos() - h.abs() * half_angle.sin()
        }
        _ => unreachable!(),
    }
}

#[derive(Default, Debug)]
struct Tally {
    cases: u32,
    skipped: u32,
    clear_certified: u32,
    door_miss: u32,
    door_certified: u32,
    door_declined: u32,
    escalated: u32,
    // defects
    clear_on_crossing: u32,
    clear_in_band: u32,
    miss_on_crossing: u32,
    miss_in_band: u32,
    root_off: u32,
    count_short: u32,
}

/// `reduce::conic_clearance`, restated (it takes an `EdgeCurve`).
fn clearance(
    s: &geom::Surface<f64>,
    conic: &geom_brep::Conic<f64>,
    t0: f64,
    t1: f64,
    band: Band,
) -> Option<Result<Sign, geom_core::Indeterminate>> {
    let (lo, hi) = geom_brep::conic_residual_extremes(s, conic)?;
    let carrier_margin = lo.max(-hi);
    let arc_margin = geom_brep::conic_arc_residual_range(s, conic, t0, t1)
        .map_or(carrier_margin, |(arc_lo, arc_hi)| arc_lo.max(-arc_hi));
    Some(crate::validate::decide(
        "bool_conic_curved_clearance",
        Margin::of(carrier_margin.max(arc_margin)),
        band,
    ))
}

fn run(eps: f64, n: u32, seed: u64, only_kind: Option<u32>, verbose: bool) -> Tally {
    let band = Band::new(eps, 10.0 * eps).unwrap();
    let mut rng = Rng(seed);
    let mut t = Tally::default();
    let mut shown = 0;
    for i in 0..n {
        let scale_draw = if rng.below(2) == 0 { 1.0 } else { 1000.0 };
        let scale = std::env::var("PROBE_SCALE").ok().and_then(|s| s.parse().ok()).unwrap_or(scale_draw);
        let n_ax = rng.unit();
        let u = rng.unit();
        let u_ref = (u - n_ax * u.dot(n_ax)).normalize();
        let big = scale * rng.range(0.5, 5.0);
        let small_draw = big / rng.range(1.0, 40.0);
        let circle = std::env::var("PROBE_CIRCLE").is_ok();
        let small = if circle { big } else { small_draw };
        let combo = if circle { 0 } else { i % 8 };
        let (mut major, mut minor) = if combo & 1 == 0 {
            (big, small)
        } else {
            (small, big)
        };
        if combo & 2 != 0 {
            major = -major;
        }
        if combo & 4 != 0 {
            minor = -minor;
        }
        let far = if rng.below(2) == 0 { 1.0 } else { 1000.0 };
        let center = Point3::new(
            rng.range(-far, far),
            rng.range(-far, far),
            rng.range(-far, far),
        );
        let e = if circle {
            geom::Curve3::Circle {
                center,
                axis: n_ax,
                radius: major,
                u_ref,
            }
        } else {
            geom::Curve3::Ellipse {
                center,
                axis: n_ax,
                major,
                minor,
                u_ref,
            }
        };
        let conic = geom_brep::Conic::of(&e).unwrap();
        let k = rng.below(4);
        let vertex = FRAC_PI_2 * f64::from(k);
        let p = e.eval(vertex);
        let outward = (p - center).normalize();
        let tangent = n_ax.cross(outward);
        let psi = if rng.below(2) == 0 {
            0.0
        } else {
            rng.range(-1.2, 1.2)
        };
        let nrm = outward * psi.cos() + n_ax * psi.sin();
        let gap = eps * rng.range(-40.0, 40.0);
        let r = 10f64.powf(rng.range(-6.0, 0.0));
        let kind = only_kind.unwrap_or(rng.below(4));
        let x = Vec3::new(1.0, 0.0, 0.0);
        let perp = |a: Vec3<f64>| (x - a * x.dot(a)).normalize();
        let s = match kind {
            0 => geom::Surface::Sphere {
                center: p + nrm * (r + gap),
                radius: r,
                axis: n_ax,
                u_ref: perp(n_ax),
            },
            1 => {
                // axis ⊥ nrm: along the tangent, or a random direction ⊥ nrm
                let w = if rng.below(2) == 0 {
                    tangent
                } else {
                    let v = rng.unit();
                    (v - nrm * v.dot(nrm)).normalize()
                };
                geom::Surface::Cylinder {
                    origin: p + nrm * (r + gap),
                    axis: w,
                    radius: r,
                    u_ref: nrm,
                }
            }
            2 => {
                let big_r = r * rng.range(1.5, 10.0);
                let v = rng.unit();
                let w2 = (v - nrm * v.dot(nrm)).normalize();
                let t0c = p + nrm * (r + gap);
                let axis = nrm.cross(w2).normalize();
                geom::Surface::Torus {
                    center: t0c + nrm * big_r,
                    axis,
                    major_radius: big_r,
                    minor_radius: r,
                    u_ref: perp(axis),
                }
            }
            _ => {
                // a cone whose generator passes `gap` outside the vertex,
                // the generator along a direction ⊥ nrm
                let alpha = rng.range(0.2, 1.2);
                let v = rng.unit();
                let g = (v - nrm * v.dot(nrm)).normalize();
                // the axis makes angle alpha with g, on the far side of nrm
                let axis = (g * alpha.cos() + nrm * alpha.sin()).normalize();
                let apex = p + nrm * gap - g * (10.0 * r);
                geom::Surface::Cone {
                    apex,
                    axis,
                    half_angle: alpha,
                    u_ref: perp(axis),
                }
            }
        };
        t.cases += 1;
        let at = distance(&s, p);
        // Re-check by dense sampling that the vertex is the least distance
        // (within a fraction of a band); otherwise skip the pose.
        let steps = 20_000;
        let mut least = at;
        for j in 0..steps {
            let th = core::f64::consts::TAU * f64::from(j) / f64::from(steps);
            least = least.min(distance(&s, e.eval(th)));
        }
        if least < at - 0.1 * eps || (gap - at).abs() > 0.5 * eps {
            t.skipped += 1;
            continue;
        }
        let crossing = at < -eps;
        let in_band = at.abs() <= eps;
        let (t0, t1) = (vertex - 0.5, vertex + 0.5);
        let label = || {
            format!(
                "ε {eps} case {i} kind {kind} combo {combo} scale {scale} far {far} r {r:e} gap {gap:e} at {at:e} psi {psi} vertex {k}: {e:?} vs {s:?}"
            )
        };
        match clearance(&s, &conic, t0, t1, band) {
            Some(Ok(Sign::Positive)) => {
                t.clear_certified += 1;
                if crossing {
                    t.clear_on_crossing += 1;
                    if verbose && shown < 6 {
                        shown += 1;
                        println!("CLEAR ON CROSSING: {}", label());
                    }
                } else if in_band {
                    t.clear_in_band += 1;
                }
                continue;
            }
            Some(Err(_)) => {
                t.escalated += 1;
                continue;
            }
            _ => {}
        }
        if kind >= 2 {
            // torus and cone: no root lane (`wall_crossing` answers Unsettled)
            t.door_declined += 1;
            continue;
        }
        let door = if circle {
            match s {
                geom::Surface::Sphere { .. } => {
                    crate::boolean::circle_sphere::circle_sphere_roots(&e, t0, t1, &s, band)
                }
                _ => crate::boolean::circle_cylinder::circle_cylinder_roots(&e, t0, t1, &s, band),
            }
        } else {
            ellipse_roots(&e, t0, t1, &s, band)
        };
        match door {
            Ok(CircleRoots::Miss) => {
                t.door_miss += 1;
                if crossing {
                    t.miss_on_crossing += 1;
                    if verbose && shown < 6 {
                        shown += 1;
                        println!("MISS ON CROSSING: {}", label());
                    }
                } else if in_band {
                    t.miss_in_band += 1;
                }
            }
            Ok(CircleRoots::Certified { count, thetas }) => {
                t.door_certified += 1;
                for &th in &thetas[..count] {
                    let off = distance(&s, e.eval(th)).abs();
                    if off > eps * (1.0 + eps / r) + 1e-13 {
                        t.root_off += 1;
                        if verbose && shown < 6 {
                            shown += 1;
                            println!("ROOT OFF {off:e}: {}", label());
                        }
                    }
                }
                if crossing && count < 2 {
                    t.count_short += 1;
                }
            }
            Err(_) => t.escalated += 1,
            _ => t.door_declined += 1,
        }
    }
    t
}

#[test]
fn reviewer_fuzz_all_kinds() {
    let n: u32 = std::env::var("PROBE_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4000);
    let kinds: u32 = std::env::var("PROBE_KINDS").ok().and_then(|s| s.parse().ok()).unwrap_or(4);
    for eps in [1e-12, 1e-9, 1e-6] {
        for (kind, name) in [(0, "sphere"), (1, "wall"), (2, "torus"), (3, "cone")].into_iter().take(kinds as usize) {
            let t = run(eps, n, 0x9E37_79B9_7F4A_7C15 ^ (kind as u64), Some(kind), std::env::var("PROBE_QUIET").is_err());
            println!("ε {eps:e} {name}: {t:?}");
        }
    }
}

/// **Pinned: the conic clearance rung certifies a definite crossing
/// clear.** Metre-scale ellipse stored in the ordinary order and sign
/// (major 4.85 m, minor 0.21 m), a 24.7 µm ball 2.71e-11 m deep at the
/// minor vertex (60-digit oracle: `probes/reach_3805_oracle.py`), ε =
/// 1e-12. `reduce::conic_clearance` reads `conic_residual_extremes`, the
/// harmonics' `c₀ − A₁ − A₂`, with no rounding charge; at the minor
/// vertex the phases align so that bound is the true minimum, and its
/// rounding (~u·a²/r ≈ 1e-10) exceeds 10ε. RED at e7f2798307.
#[test]
fn reviewer_pinned_clearance_on_a_crossing() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let e = geom::Curve3::Ellipse {
        center: Point3::new(-0.4805481181680724, -0.4410169206690735, -0.8889582859866558),
        axis: Vec3::new(-0.9621201415661211, -0.09095689596824941, 0.2570052066955227),
        major: 4.846323757498574,
        minor: 0.20752839706206302,
        u_ref: Vec3::new(0.0877135584784833, 0.7893034347374652, 0.6077058659998946),
    };
    let s = geom::Surface::Sphere {
        center: Point3::new(-0.4269724200467884, -0.5670491346043792, -0.7329974017510368),
        radius: 2.4660736247312856e-5,
        axis: Vec3::new(-0.9621201415661211, -0.09095689596824941, 0.2570052066955227),
        u_ref: Vec3::new(0.27262581167744704, -0.3209947770056513, 0.9069936713904377),
    };
    let conic = geom_brep::Conic::of(&e).unwrap();
    let v = 3.0 * FRAC_PI_2;
    let (lo, hi) = geom_brep::conic_residual_extremes(&s, &conic).unwrap();
    println!("extremes ({lo:e}, {hi:e}); true least distance -2.711e-11");
    let got = clearance(&s, &conic, v - 0.5, v + 0.5, band);
    // the door, for contrast
    let door = ellipse_roots(&e, v - 0.5, v + 0.5, &s, band);
    println!("clearance {got:?}; door {door:?}");
    assert!(
        !matches!(got, Some(Ok(Sign::Positive))),
        "a crossing 2.71e-11 m deep certified clear"
    );
}
