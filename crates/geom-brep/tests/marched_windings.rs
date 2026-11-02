//! **The marched windings against an independent oracle**: a fitted
//! loop's essentiality about each operand (`geom_brep::pair_section::winds`,
//! read over its uniqueness tube) checked against turns counted on dense
//! carrier samples carried onto the section by Newton.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/pair_section.rs",
    "crates/geom-brep/src/pair_section/winds.rs"
];

use geom::Surface;
use geom_brep::{SectionRead, SsiLane};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The turns about each channel of `s` (axis, then core) of the closed
/// polyline `pts`, by principal differences; `None` where a step turns
/// past a radian or a point sits on a centre.
fn oracle_turns(s: &Surface<f64>, pts: &[Point3<f64>]) -> Option<Vec<i64>> {
    type Coords = Box<dyn Fn(Point3<f64>) -> (f64, f64)>;
    let chans: Vec<Coords> = match *s {
        Surface::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        } => {
            let e2 = axis.cross(u_ref);
            vec![Box::new(move |p| {
                ((p - origin).dot(u_ref), (p - origin).dot(e2))
            })]
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            u_ref,
            ..
        } => {
            let e2 = axis.cross(u_ref);
            vec![
                Box::new(move |p| ((p - center).dot(u_ref), (p - center).dot(e2))),
                Box::new(move |p| {
                    let w = p - center;
                    let h = w.dot(axis);
                    ((w - axis * h).norm() - major_radius, h)
                }),
            ]
        }
        _ => Vec::new(),
    };
    chans
        .iter()
        .map(|c| {
            let mut total = 0.0;
            for (a, b) in pts.iter().zip(pts.iter().cycle().skip(1)) {
                let ((ax, ay), (bx, by)) = (c(*a), c(*b));
                if ax.hypot(ay) < 1e-9 || bx.hypot(by) < 1e-9 {
                    return None;
                }
                let d = by.atan2(bx) - ay.atan2(ax);
                let d = (d + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU)
                    - std::f64::consts::PI;
                if d.abs() > 1.0 {
                    return None;
                }
                total += d;
            }
            #[allow(clippy::cast_possible_truncation)]
            Some((total / std::f64::consts::TAU).round() as i64)
        })
        .collect()
}

/// `p` carried by Newton's least-norm step onto `f_a = f_b = 0`, or
/// `None` where it does not settle.
fn onto_section(a: &Surface<f64>, b: &Surface<f64>, mut p: Point3<f64>) -> Option<Point3<f64>> {
    use geom_brep::{implicit_gradient, implicit_residual};
    for _ in 0..40 {
        let (fa, fb) = (implicit_residual(a, p), implicit_residual(b, p));
        let (ga, gb) = (implicit_gradient(a, p), implicit_gradient(b, p));
        let (m11, m12, m22) = (ga.dot(ga), ga.dot(gb), gb.dot(gb));
        let det = m11 * m22 - m12 * m12;
        if det.abs() < 1e-300 {
            return None;
        }
        let l1 = (-fa * m22 + fb * m12) / det;
        let l2 = (fa * m12 - fb * m11) / det;
        let step = ga * l1 + gb * l2;
        p = p + step;
        if step.norm() < 1e-13 {
            return Some(p);
        }
    }
    None
}

/// **The marched windings agree with an independent oracle** (shape 1,
/// a counterexample search): a torus against a sphere, a cylinder or a
/// torus, posed generally, near the torus's axis, near its core, near a
/// tangency or linked through it. Each marched component's carrier is
/// sampled densely, every sample carried onto the section by Newton,
/// and its turns about each operand's channels counted; the component's
/// `winds` must be the oracle's essentiality wherever the oracle reads
/// (each sample settles within the component's pad and no step turns
/// past a radian). Counted sections, and refusals, are skipped.
/// Marches: in the slow set.
#[test]
fn marched_windings_agree_with_a_projected_oracle() {
    use test_utils::fuzz;
    let mut g = fuzz::start("pair_section::marched_windings_oracle");
    let (mut compared, mut skipped) = (0usize, 0usize);
    let torus = |center: Point3<f64>, axis: Vec3<f64>, big: f64, r: f64| {
        let axis = axis.normalize();
        Surface::Torus {
            center,
            axis,
            major_radius: big,
            minor_radius: r,
            u_ref: axis.orthonormal_basis().0,
        }
    };
    let ball = |center: Point3<f64>, radius: f64| Surface::Sphere {
        center,
        radius,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let wall = |origin: Point3<f64>, axis: Vec3<f64>, radius: f64| {
        let axis = axis.normalize();
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref: axis.orthonormal_basis().0,
        }
    };
    for k in 0..fuzz::scaled(48) {
        let r = g.range(0.4, 0.8);
        let t = torus(Point3::origin(), Vec3::unit_z(), 2.0, r);
        let jitter =
            |g: &mut fuzz::Rng, s: f64| Vec3::new(g.range(-s, s), g.range(-s, s), g.range(-s, s));
        let family = g.below(6);
        let partner = match family {
            0 => ball(
                Point3::new(g.range(-3.0, 3.0), g.range(-3.0, 3.0), g.range(-1.0, 1.0)),
                g.range(0.3, 2.5),
            ),
            1 => ball(
                Point3::origin() + jitter(&mut g, 0.05),
                g.range(2.0 - r + 0.05, 2.0 + r - 0.05),
            ),
            2 => {
                let phi = g.range(0.0, std::f64::consts::TAU);
                ball(
                    Point3::new(2.0 * phi.cos(), 2.0 * phi.sin(), 0.0) + jitter(&mut g, 0.1),
                    g.range(0.2, r + 0.3),
                )
            }
            3 => wall(
                Point3::origin() + jitter(&mut g, 1.0),
                Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0)),
                g.range(0.3, 3.0),
            ),
            4 => {
                // A wall along z, its radius a few hundredths from
                // touching the tube's outer equator.
                let x0 = g.range(-0.5, 0.5);
                wall(
                    Point3::new(x0, 0.0, 0.0),
                    Vec3::unit_z(),
                    2.0 + r - x0.abs() + g.range(-0.05, 0.05),
                )
            }
            _ => torus(
                Point3::new(2.0, 0.0, 0.0) + jitter(&mut g, 0.1),
                Vec3::unit_y() + jitter(&mut g, 0.2),
                g.range(1.5, 2.5),
                g.range(0.3, 0.6),
            ),
        };
        for (a, b) in [(&t, &partner), (&partner, &t)] {
            let what = format!("pose {k} (family {family}): {a:?}, {b:?}");
            let Ok(SectionRead::Built(section)) =
                geom_brep::pair_section(a, b, None, Some(SsiLane::marched()), band())
            else {
                skipped += 1;
                continue;
            };
            if section.counted().is_some() {
                skipped += 1;
                continue;
            }
            let Ok(carried) = section.carry(Some(SsiLane::marched()), band()) else {
                skipped += 1;
                continue;
            };
            for (i, c) in carried.components().iter().enumerate() {
                let (lo, hi) = c.span;
                let pts: Option<Vec<_>> = (0..2048)
                    .map(|j| {
                        let q = c.carrier.eval(lo + (hi - lo) * f64::from(j) / 2048.0);
                        onto_section(a, b, q).filter(|p| (*p - q).norm() <= c.pad)
                    })
                    .collect();
                let Some(pts) = pts else {
                    skipped += 1;
                    continue;
                };
                for (side, s) in [a, b].into_iter().enumerate() {
                    let Some(turns) = oracle_turns(s, &pts) else {
                        skipped += 1;
                        continue;
                    };
                    compared += 1;
                    assert_eq!(
                        c.winds[side],
                        turns.iter().any(|&n| n != 0),
                        "{what}: component {i}, operand {side}: oracle turns {turns:?}; {}",
                        fuzz::replay()
                    );
                }
            }
        }
    }
    eprintln!("marched windings: {compared} compared, {skipped} skipped");
}
