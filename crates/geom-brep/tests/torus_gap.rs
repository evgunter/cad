//! **The torus pairs' critical gap** (`geom_brep::pair_section::gap`):
//! the tube-circle tangencies decide `Zero`, the designer's worked example
//! is tangent exactly at its critical values, and against an independent
//! brute-force search of the spine distance's critical points no tangent
//! pose reads transverse and no transverse pose reads tangent.
//!
//! The oracle never reads the sweep's reduction to one angle: a point or
//! a line spine's distance is sampled densely round the core and each
//! extremum refined by golden section; two tori's core circles are read
//! as the two-angle squared distance, whose critical points Newton's
//! method finds from a grid of starts.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/pair_section/gap.rs",
    "crates/geom-brep/src/pair_section/torus.rs",
    "crates/geom-brep/tests/shared/"
];

use crate::shared::tol::band;
use core::f64::consts::TAU;
use geom::Surface;
use geom_brep::{
    ClassCarrier, ComponentShape, CylinderTorusPose, SectionRead, SphereTorusPose, TorusGap,
    TorusPoseRefusal, TorusQuestion, TorusTorusPose,
};
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn torus(center: Point3<f64>, axis: Vec3<f64>, big: f64, small: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Torus {
        center,
        axis,
        major_radius: big,
        minor_radius: small,
        u_ref: axis.orthonormal_basis().0,
    }
}

fn cylinder(origin: Point3<f64>, axis: Vec3<f64>, radius: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref: axis.orthonormal_basis().0,
    }
}

fn sphere(center: Point3<f64>, radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// What a pose's gap read: `Some(true)` tangent, `Some(false)`
/// transverse, `None` escalated.
fn gap_of(a: &Surface<f64>, b: &Surface<f64>) -> Option<bool> {
    let read = |gap: TorusGap<f64>| Some(matches!(gap, TorusGap::Tangent { .. }));
    let refused = |e: TorusPoseRefusal| match e {
        TorusPoseRefusal::Undecided {
            question: TorusQuestion::Gap,
            ..
        } => None,
        other => panic!("refused before the gap: {other:?}"),
    };
    match (a, b) {
        (Surface::Torus { .. }, Surface::Torus { .. }) => TorusTorusPose::decide(a, b, band())
            .expect("two tori")
            .map_or_else(refused, |pose| read(pose.gap())),
        (Surface::Cylinder { .. }, _) | (_, Surface::Cylinder { .. }) => {
            CylinderTorusPose::decide(a, b, band())
                .expect("a cylinder and a torus")
                .map_or_else(refused, |pose| read(pose.gap()))
        }
        _ => SphereTorusPose::decide(a, b, band())
            .expect("a sphere and a torus")
            .map_or_else(refused, |pose| read(pose.gap())),
    }
}

// ---------------------------------------------------------------------
// The oracle
// ---------------------------------------------------------------------

/// A torus's core circle: centre, two unit vectors spanning its plane,
/// radius.
#[derive(Clone, Copy)]
struct Core {
    c: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    big: f64,
}

impl Core {
    fn of(center: Point3<f64>, axis: Vec3<f64>, big: f64) -> Self {
        let (u, v) = axis.normalize().orthonormal_basis();
        Self {
            c: center,
            u,
            v,
            big,
        }
    }

    fn at(self, t: f64) -> Point3<f64> {
        self.c + (self.u * t.cos() + self.v * t.sin()) * self.big
    }

    fn d1(self, t: f64) -> Vec3<f64> {
        (self.v * t.cos() - self.u * t.sin()) * self.big
    }

    fn d2(self, t: f64) -> Vec3<f64> {
        (self.u * t.cos() + self.v * t.sin()) * (-self.big)
    }
}

/// The critical values of `f` round the turn: every local extremum on
/// a dense grid, refined by golden section on its bracket.
fn critical_values_1d(f: impl Fn(f64) -> f64) -> Vec<f64> {
    const N: usize = 4096;
    let h = TAU / N as f64;
    let g: Vec<f64> = (0..N).map(|k| f(k as f64 * h)).collect();
    let mut out = Vec::new();
    for k in 0..N {
        let (a, b, c) = (g[(k + N - 1) % N], g[k], g[(k + 1) % N]);
        let max = b >= a && b >= c;
        let min = b <= a && b <= c;
        if !(max || min) {
            continue;
        }
        let sign = if max { -1.0 } else { 1.0 };
        let (mut lo, mut hi) = ((k as f64 - 1.0) * h, (k as f64 + 1.0) * h);
        let phi = (5f64.sqrt() - 1.0) / 2.0;
        for _ in 0..120 {
            let x1 = hi - phi * (hi - lo);
            let x2 = lo + phi * (hi - lo);
            if sign * f(x1) < sign * f(x2) {
                hi = x2;
            } else {
                lo = x1;
            }
        }
        out.push(f(0.5 * (lo + hi)));
    }
    out
}

/// The critical values of the distance from the core `a` to the point
/// `c`.
fn point_values(a: Core, c: Point3<f64>) -> Vec<f64> {
    critical_values_1d(|t| (a.at(t) - c).norm())
}

/// The critical values of the distance from the core `a` to the line
/// `(o, d)`.
fn line_values(a: Core, o: Point3<f64>, d: Vec3<f64>) -> Vec<f64> {
    let d = d.normalize();
    critical_values_1d(|t| {
        let w = a.at(t) - o;
        (w - d * w.dot(d)).norm()
    })
}

/// The critical values of the distance between two core circles: the
/// two-angle squared distance's critical points, by Newton's method from
/// a grid of starts.
fn circle_values(a: Core, b: Core) -> Vec<f64> {
    const N: usize = 48;
    let mut out: Vec<f64> = Vec::new();
    for i in 0..N {
        for j in 0..N {
            let (mut s, mut t) = (TAU * i as f64 / N as f64, TAU * j as f64 / N as f64);
            let mut converged = false;
            for _ in 0..60 {
                let w = a.at(s) - b.at(t);
                let (a1, b1) = (a.d1(s), b.d1(t));
                let gs = 2.0 * w.dot(a1);
                let gt = -2.0 * w.dot(b1);
                let hss = 2.0 * (a1.dot(a1) + w.dot(a.d2(s)));
                let htt = 2.0 * (b1.dot(b1) - w.dot(b.d2(t)));
                let hst = -2.0 * a1.dot(b1);
                let det = hss * htt - hst * hst;
                if det.abs() < 1e-300 {
                    break;
                }
                let (ds, dt) = ((htt * gs - hst * gt) / det, (hss * gt - hst * gs) / det);
                s -= ds;
                t -= dt;
                if ds.abs() + dt.abs() < 1e-15 {
                    converged = true;
                    break;
                }
            }
            if converged {
                let v = (a.at(s) - b.at(t)).norm();
                if !out.iter().any(|&x| (x - v).abs() < 1e-11) {
                    out.push(v);
                }
            }
        }
    }
    out
}

/// The least distance of a critical value from a level.
fn oracle_gap(values: &[f64], [ra, rb]: [f64; 2]) -> f64 {
    values
        .iter()
        .flat_map(|&c| [(c - (ra + rb)).abs(), (c - (ra - rb).abs()).abs()])
        .fold(f64::INFINITY, f64::min)
}

/// The oracle's reading of a pose's spine distance, the torus `t` first.
fn values_of(t: &Surface<f64>, partner: &Surface<f64>) -> Vec<f64> {
    let &Surface::Torus {
        center,
        axis,
        major_radius,
        ..
    } = t
    else {
        panic!("the first surface is the torus")
    };
    let core = Core::of(center, axis, major_radius);
    match *partner {
        Surface::Sphere { center, .. } => point_values(core, center),
        Surface::Cylinder { origin, axis, .. } => line_values(core, origin, axis),
        Surface::Torus {
            center,
            axis,
            major_radius,
            ..
        } => circle_values(core, Core::of(center, axis, major_radius)),
        _ => panic!("a torus pair"),
    }
}

// ---------------------------------------------------------------------
// The tube-circle tangency
// ---------------------------------------------------------------------

/// **Two pipe bends joined G1 in different planes, and a straight pipe
/// entering a bend, are tangent along a tube circle: Zero.** The bends'
/// cores touch at `(2, 0, 0)` with one tangent, in planes `0.7` rad
/// apart, both tubes of radius `0.5`; the pipe's axis is the first core's
/// tangent line there, its radius the tube's. Moved off by `δ` in the
/// second tube's radius (the level `|r₁ − r₂|` stands `δ` from the
/// cores' least distance, `0`): within the zero band the gap is still
/// Zero, inside the ambiguity band it escalates, a few escalation
/// widths out it is transverse. Mutants: the least-distance reading
/// dropped (the touch reads in band, `D` not smooth there), or decided on
/// `D` alone (every `δ` reads Zero).
#[test]
fn the_tube_circle_tangency_decides_zero() {
    let b = band();
    let first = torus(p(0.0, 0.0, 0.0), Vec3::unit_z(), 2.0, 0.5);
    let alpha = 0.7f64;
    let inward = Vec3::new(-alpha.cos(), 0.0, alpha.sin());
    let bend = |r2: f64| {
        torus(
            p(2.0, 0.0, 0.0) + inward * 1.5,
            inward.cross(Vec3::unit_y()),
            1.5,
            r2,
        )
    };
    let pipe = |rc: f64| cylinder(p(2.0, 0.0, 0.0), Vec3::unit_y(), rc);
    for (delta, want) in [
        (0.0, Some(true)),
        (0.25 * b.zero(), Some(true)),
        (0.5 * (b.zero() + b.escalate()), None),
        (4.0 * b.escalate(), Some(false)),
    ] {
        for (what, partner) in [("bend", bend(0.5 + delta)), ("pipe", pipe(0.5 + delta))] {
            for (x, y) in [(&first, &partner), (&partner, &first)] {
                assert_eq!(gap_of(x, y), want, "{what} at δ = {delta:e}");
            }
        }
    }
    // The tangent pipe stands square to the torus axis: the section value
    // lists one tangent component, classification only.
    match geom_brep::pair_section(&first, &pipe(0.5), b) {
        Ok(SectionRead::Built(s)) => {
            assert!(s.tangency().is_some(), "the pipe's tangency is decided");
            assert!(
                matches!(s.components(), [c] if matches!(c.carrier, ClassCarrier::Unminted(_))),
                "one tangent component"
            );
        }
        other => panic!("the pipe's section: {other:?}"),
    }
}

/// **A sphere centred on the core touches the torus along a whole tube
/// circle, never at one point.** Its gap reads Zero on the least
/// distance, and the touch reading, whose centre stands on the tube
/// circle's centre, refuses before it divides by that offset: the section
/// is one tangent loop, never a point with a witness. Mutant: the offset
/// guard dropped (the touch divides by zero and its NaN reads no sign,
/// which is the guard's own verdict; a witness reaching the point
/// component turns the row red).
#[test]
fn a_sphere_on_the_core_is_tangent_along_a_circle() {
    let t = torus(p(0.0, 0.0, 0.0), Vec3::unit_z(), 2.0, 0.5);
    let s = sphere(p(2.0, 0.0, 0.0), 0.5);
    for (x, y) in [(&t, &s), (&s, &t)] {
        match geom_brep::pair_section(x, y, band()) {
            Ok(SectionRead::Built(sec)) => {
                assert!(sec.tangency().is_some(), "tangent");
                assert!(
                    matches!(sec.components(), [c] if c.shape == ComponentShape::Loop
                        && c.witness.is_none()),
                    "one tangent loop: {:?}",
                    sec.components()
                );
            }
            other => panic!("{other:?}"),
        }
    }
}

// ---------------------------------------------------------------------
// The designer's worked example
// ---------------------------------------------------------------------

/// The worked example's torus, `R = 2`, `r = 0.5` about `z`.
fn example_torus() -> Surface<f64> {
    torus(p(0.0, 0.0, 0.0), Vec3::unit_z(), 2.0, 0.5)
}

/// Its tilted axis line, whose critical distances from the core are
/// about `{0.061, 0.562, 1.661, 2.372}`.
fn example_line() -> (Point3<f64>, Vec3<f64>) {
    (
        p(-0.96, 0.0, -0.11),
        Vec3::new(-0.916, 0.368, 0.158).normalize(),
    )
}

/// **The designer's worked example: tangent exactly at its critical
/// values.** The core's distance from the tilted line has four critical
/// values, read by the oracle; a cylinder about the line is tangent to
/// the torus where its radius is one of them plus or less the tube's,
/// eight radii, and at each the gap is Zero. At `r_c = 0.5` the levels
/// are `1` and `0`, the nearest critical value `0.061` off, and the gap
/// reads transverse. Mutants: a cell cleared on a first-order bound of
/// `D′` through its turn (a turn reads transverse), the turn's window
/// read on the cell alone (a turn on a cut escalates).
#[test]
fn the_designers_example_is_tangent_exactly_at_its_critical_values() {
    let t = example_torus();
    let (o, d) = example_line();
    let values = line_values(Core::of(p(0.0, 0.0, 0.0), Vec3::unit_z(), 2.0), o, d);
    let mut sorted = values.clone();
    sorted.sort_by(f64::total_cmp);
    sorted.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    assert_eq!(sorted.len(), 4, "four critical values: {sorted:?}");
    for (have, about) in sorted.iter().zip([0.061, 0.562, 1.661, 2.372]) {
        assert!((have - about).abs() < 2e-3, "{sorted:?}");
    }
    let mut radii = Vec::new();
    for &c in &sorted {
        radii.push(c + 0.5);
        if (c - 0.5).abs() > 0.01 {
            radii.push((c - 0.5).abs());
        }
    }
    assert_eq!(radii.len(), 8, "eight tangent radii: {radii:?}");
    for rc in radii {
        let wall = cylinder(o, d, rc);
        assert_eq!(gap_of(&t, &wall), Some(true), "tangent at r_c = {rc}");
        assert_eq!(
            gap_of(&wall, &t),
            Some(true),
            "tangent at r_c = {rc}, swapped"
        );
    }
    let wall = cylinder(o, d, 0.5);
    assert!((oracle_gap(&values, [0.5, 0.5]) - sorted[0]).abs() < 1e-12);
    assert_eq!(gap_of(&t, &wall), Some(false), "transverse at r_c = 0.5");
}

// ---------------------------------------------------------------------
// The counterexample search
// ---------------------------------------------------------------------

fn unit_vector(g: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0));
        let n = v.norm();
        if n > 0.2 && n < 1.0 {
            break v * (1.0 / n);
        }
    }
}

/// **The counterexample search.** Random ring tori against a sphere, a
/// cylinder and another ring torus in general poses, each partner's
/// radius set so that a level stands a chosen distance from one of the
/// oracle's critical values: on it, within the zero band, across the
/// ambiguity band, a few escalation widths off, or anywhere. Every pose
/// read tangent has an oracle gap within the zero band, and every pose
/// read transverse one at least the escalation width; an escalation is
/// never wrong.
#[test]
fn no_tangent_pose_reads_transverse_and_no_transverse_pose_reads_tangent() {
    let mut g = fuzz::start("torus_gap::critical_values");
    let b = band();
    let slack = 1e-11;
    let mut tally = [0usize; 3];
    for k in 0..fuzz::scaled(200) {
        let big = g.range(1.0, 2.0);
        let r = big * g.range(0.1, 0.6);
        let t = torus(
            p(g.range(-0.5, 0.5), g.range(-0.5, 0.5), g.range(-0.5, 0.5)),
            unit_vector(&mut g),
            big,
            r,
        );
        let at = p(
            g.range(-1.5, 1.5) * big,
            g.range(-1.5, 1.5) * big,
            g.range(-1.5, 1.5) * big,
        );
        let axis = unit_vector(&mut g);
        let kind = k % 3;
        let big2 = g.range(0.8, 2.0);
        let shell = |rho: f64| match kind {
            0 => sphere(at, rho),
            1 => cylinder(at, axis, rho),
            _ => torus(at, axis, big2, rho),
        };
        let values = values_of(&t, &shell(0.1));
        let Some(&c) = values.get(g.below(values.len().max(1))) else {
            continue;
        };
        let delta = match g.below(5) {
            0 => 0.0,
            1 => g.range(-0.5, 0.5) * b.zero(),
            2 => g.range(-1.0, 1.0) * b.escalate(),
            3 => g.range(2.0, 6.0) * b.escalate() * if g.unit() < 0.5 { 1.0 } else { -1.0 },
            _ => g.range(-0.5, 0.5),
        };
        let level = c + delta;
        let rho = match g.below(3) {
            0 => level - r,
            1 => r + level,
            _ => r - level,
        };
        let ceiling = if kind == 2 { 0.9 * big2 } else { f64::INFINITY };
        if !(0.02..ceiling).contains(&rho) {
            continue;
        }
        let partner = shell(rho);
        let gap = oracle_gap(&values, [r, rho]);
        for (x, y) in [(&t, &partner), (&partner, &t)] {
            let read = gap_of(x, y);
            match read {
                Some(true) => assert!(
                    gap <= b.zero() + slack,
                    "pose {k}: read tangent, the oracle's gap {gap:e}; {}",
                    fuzz::replay()
                ),
                Some(false) => assert!(
                    gap >= b.escalate() - slack,
                    "pose {k}: read transverse, the oracle's gap {gap:e}; {}",
                    fuzz::replay()
                ),
                None => {}
            }
            tally[match read {
                Some(true) => 0,
                Some(false) => 1,
                None => 2,
            }] += 1;
        }
    }
    println!("[torus gap] tangent, transverse, escalated: {tally:?}");
}
