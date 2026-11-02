//! Review probes for PR 6310 (`apex_line_on`): the line order across
//! poses, frames, nappes and normals, and the at-apex heading read near
//! each line and near the bisector. Review-branch only.

#![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]

use super::join::apex_line_on;
use super::{Branch, BooleanError};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                2.0 * self.next() - 1.0,
                2.0 * self.next() - 1.0,
                2.0 * self.next() - 1.0,
            );
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

fn square_to(a: Vec3<f64>) -> Vec3<f64> {
    let t = if a.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    (t - a * t.dot(a)).normalize()
}

/// The two section lines, independently: directions in the plane at
/// angle `alpha` to the axis, each turned onto the axis' nappe, in the
/// order the PR states (line 0 the one the other turns from positively
/// about `n`).
fn truth(axis: Vec3<f64>, alpha: f64, n: Vec3<f64>) -> [Vec3<f64>; 2] {
    let ap = axis - n * axis.dot(n);
    let e1 = ap.normalize();
    let e2 = n.cross(e1);
    let ct = alpha.cos() / ap.norm();
    assert!(ct < 1.0, "pose has no line pair");
    let st = (1.0 - ct * ct).sqrt();
    let g = e1 * ct + e2 * st;
    let h = e1 * ct - e2 * st;
    if g.cross(h).dot(n) > 0.0 { [g, h] } else { [h, g] }
}

struct Pose {
    plane: geom::Surface<f64>,
    cone: geom::Surface<f64>,
    reach: geom_brep::Reach<f64>,
    apex: Point3<f64>,
    lines: [Vec3<f64>; 2],
    label: String,
}

fn pose(rng: &mut Lcg, alpha_deg: f64, dip: f64, flip_n: bool, flip_axis: bool) -> Pose {
    let alpha = alpha_deg.to_radians();
    let mut axis = rng.unit();
    if flip_axis {
        axis = -axis;
    }
    let apex = Point3::new(
        4.0 * rng.next() - 2.0,
        4.0 * rng.next() - 2.0,
        4.0 * rng.next() - 2.0,
    );
    // Normal at angle beta from the axis, beta in (90° − α, 90°]:
    // `dip` ∈ (0, 1] places it across that window.
    let lo = core::f64::consts::FRAC_PI_2 - alpha;
    let beta = lo + dip * alpha;
    let u = square_to(axis);
    let w = axis.cross(u);
    let phi = 2.0 * core::f64::consts::PI * rng.next();
    let side = u * phi.cos() + w * phi.sin();
    let mut n = axis * beta.cos() + side * beta.sin();
    if flip_n {
        n = -n;
    }
    let cone = geom::Surface::Cone {
        apex,
        axis,
        half_angle: alpha,
        u_ref: square_to(axis),
    };
    let plane = geom::Surface::Plane {
        origin: apex + square_to(n) * 0.7,
        normal: n,
        u_ref: square_to(n),
    };
    let reach = geom_brep::Reach::Measured {
        at: apex + axis * 0.5,
        lever: 1.5,
    };
    Pose {
        plane,
        cone,
        reach,
        apex,
        lines: truth(axis, alpha, n),
        label: format!(
            "α {alpha_deg}° dip {dip} flip_n {flip_n} flip_axis {flip_axis} axis {axis:?} n {n:?}"
        ),
    }
}

fn read(p: &Pose, site: Point3<f64>, dir: Vec3<f64>) -> Result<Branch, BooleanError> {
    apex_line_on((&p.plane, &p.cone), &p.reach, (site, dir), band())
}

/// Claim 1: every site on a line, either nappe, either direction, at
/// the apex or off it, names the line its independent order names.
#[test]
fn review_6310_every_germ_of_a_pose_names_its_lines_canonical_index() {
    let seed = std::env::var("REVIEW_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0x6310_u64);
    eprintln!("seed {seed:#x}");
    let mut rng = Lcg(seed);
    let mut poses = 0;
    let mut only = 0;
    for alpha in [15.0, 22.5, 30.0, 45.0, 60.0, 67.5, 75.0] {
        for dip in [0.05, 0.2, 0.5, 0.8, 1.0] {
            for flip_n in [false, true] {
                for flip_axis in [false, true] {
                    for _ in 0..4 {
                        let p = pose(&mut rng, alpha, dip, flip_n, flip_axis);
                        poses += 1;
                        for (k, d) in p.lines.iter().enumerate() {
                            for (what, site, dir) in [
                                ("up", p.apex + *d * 0.37, -*d),
                                ("up, out", p.apex + *d * 1.1, *d),
                                ("far nappe", p.apex - *d * 0.6, *d),
                                ("far nappe, out", p.apex - *d * 0.05, -*d),
                                ("apex, along", p.apex, *d),
                                ("apex, back", p.apex, -*d * 3.0),
                            ] {
                                match read(&p, site, dir) {
                                    Ok(Branch::Line(got)) => assert_eq!(
                                        got, k,
                                        "{}: {what}: line {k} named {got}",
                                        p.label
                                    ),
                                    Ok(Branch::Only) => {
                                        only += 1;
                                        eprintln!("{}: {what}: Only", p.label);
                                    }
                                    other => panic!("{}: {what}: {other:?}", p.label),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("{poses} poses, {only} reads Only");
    assert_eq!(only, 0, "a line-pair pose minted Only");
}

/// Claim 3: at the apex, a direction turned off a line by `delta`
/// toward the other line (in the plane), or out of the plane, names that
/// line or refuses typed; it never names the other line. The bisectors
/// refuse.
#[test]
fn review_6310_the_apex_heading_read_never_guesses() {
    let mut rng = Lcg(0xbeef);
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for alpha in [15.0, 30.0, 45.0, 60.0, 75.0] {
        for dip in [0.02, 0.1, 0.5, 1.0] {
            for flip_n in [false, true] {
                let p = pose(&mut rng, alpha, dip, flip_n, false);
                let n = match p.plane {
                    geom::Surface::Plane { normal, .. } => normal,
                    _ => unreachable!(),
                };
                let [l0, l1] = p.lines;
                for (k, d, other) in [(0, l0, l1), (1, l1, l0)] {
                    let toward = (other - d * other.dot(d)).normalize();
                    for delta in [
                        0.0_f64, 1e-15, 1e-13, 1e-12, 1e-11, 1e-10, 1e-9, 1e-8, 1e-7, 1e-6, 1e-5,
                        1e-3, 1e-1,
                    ] {
                        for (how, perp) in [("in plane", toward), ("out of plane", n)] {
                            for sense in [1.0, -1.0] {
                                let dir = (d * delta.cos() + perp * delta.sin()) * sense;
                                let got = read(&p, p.apex, dir);
                                let key = match &got {
                                    Ok(Branch::Line(j)) if *j == k => "own line".to_string(),
                                    Ok(Branch::Line(_)) => panic!(
                                        "{}: line {k} turned {delta:e} {how} named the other line",
                                        p.label
                                    ),
                                    Ok(b) => panic!("{}: {b:?}", p.label),
                                    Err(BooleanError::JoinDesync { .. }) => "desync".into(),
                                    Err(e) => format!("err {}", format!("{e:?}").chars().take(40).collect::<String>()),
                                };
                                *tally.entry(format!("δ {delta:e}: {key}")).or_default() += 1;
                            }
                        }
                    }
                }
                for bis in [(l0 + l1).normalize(), (l0 - l1).normalize()] {
                    let got = read(&p, p.apex, bis);
                    assert!(
                        matches!(got, Err(BooleanError::JoinDesync { .. })),
                        "{}: bisector {bis:?}: {got:?}",
                        p.label
                    );
                }
            }
        }
    }
    for (k, v) in &tally {
        eprintln!("{k}: {v}");
    }
}

/// Claim 1, frame: a rigid turn of the whole pose names the same lines
/// (both the order and the per-germ reads are frame-free).
#[test]
fn review_6310_a_turned_pose_names_the_same_lines() {
    let mut rng = Lcg(0x7u64);
    for alpha in [15.0, 40.0, 75.0] {
        for dip in [0.1, 0.6, 1.0] {
            let p = pose(&mut rng, alpha, dip, false, false);
            let axis_turn = rng.unit();
            let ang = 2.0 * core::f64::consts::PI * rng.next();
            // Rodrigues.
            let rot = |v: Vec3<f64>| {
                v * ang.cos() + axis_turn.cross(v) * ang.sin()
                    + axis_turn * axis_turn.dot(v) * (1.0 - ang.cos())
            };
            let shift = Vec3::new(3.0, -7.0, 11.0);
            let rp = |q: Point3<f64>| Point3::origin() + rot(q - Point3::origin()) + shift;
            let (geom::Surface::Plane { origin, normal, u_ref }, geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref: cu,
            }) = (p.plane.clone(), p.cone.clone())
            else {
                unreachable!()
            };
            let q = Pose {
                plane: geom::Surface::Plane {
                    origin: rp(origin),
                    normal: rot(normal),
                    u_ref: rot(u_ref),
                },
                cone: geom::Surface::Cone {
                    apex: rp(apex),
                    axis: rot(axis),
                    half_angle,
                    u_ref: rot(cu),
                },
                reach: geom_brep::Reach::Measured {
                    at: rp(apex + axis * 0.5),
                    lever: 1.5,
                },
                apex: rp(apex),
                lines: [rot(p.lines[0]), rot(p.lines[1])],
                label: format!("{} turned", p.label),
            };
            for k in 0..2 {
                let a = read(&p, p.apex + p.lines[k] * 0.4, p.lines[k]).unwrap();
                let b = read(&q, q.apex + q.lines[k] * 0.4, q.lines[k]).unwrap();
                assert_eq!(a, b, "{}: line {k}", q.label);
                assert_eq!(a, Branch::Line(k), "{}", p.label);
            }
        }
    }
}
