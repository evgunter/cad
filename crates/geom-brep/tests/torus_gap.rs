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

use crate::shared::tol::{band, eps};
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
/// a grid of starts, and its least value over every iterate, the global
/// minimum (always a critical value), which Newton reaches only linearly
/// where two cores run side by side and its step then never settles.
fn circle_values(a: Core, b: Core) -> Vec<f64> {
    const N: usize = 48;
    let mut out: Vec<f64> = Vec::new();
    let mut least = f64::INFINITY;
    for i in 0..N {
        for j in 0..N {
            let (mut s, mut t) = (TAU * i as f64 / N as f64, TAU * j as f64 / N as f64);
            let mut converged = false;
            for _ in 0..60 {
                let w = a.at(s) - b.at(t);
                least = least.min(w.norm());
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
    out.push(least);
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
/// Zero, inside the ambiguity band it escalates, a few escalation widths
/// out it never reads tangent, and `0.1` mm out it is transverse at every
/// band. Mutants: the least-distance reading
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
        (1e-4, Some(false)),
    ] {
        for (what, partner) in [("bend", bend(0.5 + delta)), ("pipe", pipe(0.5 + delta))] {
            for (x, y) in [(&first, &partner), (&partner, &first)] {
                assert_eq!(gap_of(x, y), want, "{what} at δ = {delta:e}");
            }
        }
    }
    // A few escalation widths out the joint is transverse or escalates:
    // where `D` passes the level the cores run side by side, `(D²)′` is
    // second order in the offset, and at a tight band it stands inside
    // its own rounding (at `ε = 1e-12`, `4·escalate` reads no slope).
    for partner in [
        bend(0.5 + 4.0 * b.escalate()),
        pipe(0.5 + 4.0 * b.escalate()),
    ] {
        for (x, y) in [(&first, &partner), (&partner, &first)] {
            assert_ne!(gap_of(x, y), Some(true), "4·escalate out read tangent");
        }
    }
    // The tangent pipe stands square to the torus axis: the section value
    // lists one tangent component, classification only.
    match geom_brep::pair_section(&first, &pipe(0.5), None, None, b) {
        Ok(SectionRead::Built(s)) => {
            assert!(s.tangency().is_some(), "the pipe's tangency is decided");
            assert!(
                matches!(s.counted().expect("a counted section"), [c] if matches!(c.carrier, ClassCarrier::Unminted(_))),
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
        match geom_brep::pair_section(x, y, None, None, band()) {
            Ok(SectionRead::Built(sec)) => {
                assert!(sec.tangency().is_some(), "tangent");
                assert!(
                    matches!(sec.counted().expect("a counted section"), [c] if c.shape == ComponentShape::Loop
                        && c.witness.is_none()),
                    "one tangent loop: {:?}",
                    sec.components()
                );
            }
            other => panic!("{other:?}"),
        }
    }
}

/// **A G1 pipe joint read in a generic frame** (review R1): the cores
/// touch with one tangent, so the least distance is `0` and the gap is
/// `|r₁ − r₂|`. In the band (`δ` of `3` and `5` zero bands) it never reads
/// transverse, in either order; a joint two escalation widths off at a 600 m scale,
/// definitely transverse, never reads tangent. The joint is built on the
/// first torus's core and checked against the oracle: the review's own
/// first frame, read literally, stands `4.06e-6` apart (its cores do not
/// touch), and reads transverse rightly. Mutant: a sign of `G′` read
/// past its rounding (the rounding of `G′`, about `1e-15` of its scale,
/// past the band once divided by `D ≈ 1e-8`).
#[test]
fn a_generic_frame_pipe_joint_reads_no_definite_verdict_in_band() {
    let t = |c: [f64; 3], a: [f64; 3], u: [f64; 3], big: f64, r: f64| Surface::Torus {
        center: p(c[0], c[1], c[2]),
        axis: Vec3::new(a[0], a[1], a[2]),
        major_radius: big,
        minor_radius: r,
        u_ref: Vec3::new(u[0], u[1], u[2]),
    };
    let z = band().zero();
    let r1 = 12.656385726945569;
    let big1 = 23.171879988522775;
    let axis1 = Vec3::new(1.3815487195293774, -0.8823486091242255, 2.512525436198265);
    let a = t(
        [0.0; 3],
        [axis1.x, axis1.y, axis1.z],
        [0.6541898890162481, 6.186765858262199, 3.5091235336082125],
        big1,
        r1,
    );
    // The joint at the review's nearest approach, `φ = 1.6271` on the
    // first core: the second core runs along it there.
    let core = Core::of(p(0.0, 0.0, 0.0), axis1, big1);
    let phi = 1.6271;
    let (q, along) = (core.at(phi), core.d1(phi).normalize());
    let tilt = Vec3::new(0.8669846131600993, 0.21765944051911806, -0.4482879080418493);
    let axis2 = (tilt - along * tilt.dot(along)).normalize();
    let big2 = 58.95540493709222;
    let center2 = q + axis2.cross(along) * big2;
    let b = |r2| torus(center2, axis2, big2, r2);
    for d in [3.0 * z, 5.0 * z] {
        let gap = oracle_gap(&values_of(&a, &b(r1 - d)), [r1, r1 - d]);
        assert!((gap - d).abs() < 1e-11, "the joint's gap is δ: {gap:e}");
        assert_ne!(gap_of(&a, &b(r1 - d)), Some(false), "in band at δ = {d:e}");
        assert_ne!(gap_of(&b(r1 - d), &a), Some(false), "swapped, δ = {d:e}");
    }
    let a = t(
        [0.0; 3],
        [0.6220201617964208, -0.30817188285414643, 0.7198062301320298],
        [-0.38147759920165625, 0.1043928358361628, 0.5132738218804678],
        600.0,
        150.0,
    );
    let b = t(
        [163.60592379805317, -49.92485268642143, 374.23820747966545],
        [0.1249805341084149, 0.08022231456261628, 0.9889106361750754],
        [
            -0.2573625282109311,
            -0.007393921782424581,
            0.3581603935179129,
        ],
        600.0,
        150.0 - 2.0 * band().escalate(),
    );
    let gap = oracle_gap(&values_of(&a, &b), [150.0, 150.0 - 2.0 * band().escalate()]);
    assert!(
        gap >= band().escalate(),
        "the far joint is transverse: {gap:e}"
    );
    for (x, y) in [(&a, &b), (&b, &a)] {
        assert_ne!(
            gap_of(x, y),
            Some(true),
            "a definitely transverse joint read tangent"
        );
    }
}

/// **A critical point on the first cut's grid** (review R2): the
/// sphere, cylinder and torus partners each with a critical value `0`
/// at a core angle `2πk/32`, a cut of the sweep's first cells, and the
/// level `s·ε` in the band. None reads transverse. Mutant: the slope of
/// a cell decided where it has none (the turn lies on the cell's end).
#[test]
fn no_in_band_gap_reads_transverse_on_a_grid_angle() {
    let e = eps();
    let a = torus(p(0.0, 0.0, 0.0), Vec3::unit_z(), 2.0, 0.5);
    for k in 0..32 {
        let al = TAU * f64::from(k) / 32.0;
        let p0 = p(2.0 * al.cos(), 2.0 * al.sin(), 0.0);
        let tang = Vec3::new(-al.sin(), al.cos(), 0.0);
        for s in [1.5, 3.0, 5.0, 9.5] {
            let rr = 0.5 + s * e;
            for b in [
                sphere(p0, rr),
                cylinder(p0, tang + Vec3::unit_z(), rr),
                torus(p0 + Vec3::new(0.0, 0.0, 1.5), tang, 1.5, rr),
            ] {
                for (x, y) in [(&a, &b), (&b, &a)] {
                    assert_ne!(gap_of(x, y), Some(false), "k = {k}, s = {s}: {y:?}");
                }
            }
        }
    }
}

/// **A sphere on the core, in the band, escalates at the march door**
/// (review R2; main's closed form did, the sweep marched it): its gap is
/// `3ε` from a tangency along a whole tube circle.
#[test]
fn a_sphere_on_the_core_in_band_escalates_at_the_ssi_door() {
    use geom_brep::SsiDomain;
    use geom_brep::ssi::{SsiError, TraceDecision};
    let t = torus(p(0.0, 0.0, 0.0), Vec3::unit_z(), 2.0, 0.5);
    let dom = SsiDomain {
        center: Point3::origin(),
        half_extent: 3.0,
        extent: 5.0,
        floor_scale: 1.0,
    };
    let al = core::f64::consts::FRAC_PI_8;
    let s = sphere(p(2.0 * al.cos(), 2.0 * al.sin(), 0.0), 0.5 + 3.0 * eps());
    let got = geom_brep::ssi::sphere_torus_ssi(&s, &t, dom, band());
    assert!(
        matches!(
            got,
            Err(SsiError::Escalated {
                decision: TraceDecision::SphereTorusTangency,
                ..
            })
        ),
        "{got:?}"
    );
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

/// A unit vector square to `n`, otherwise random.
fn square_to(g: &mut fuzz::Rng, n: Vec3<f64>) -> Vec3<f64> {
    loop {
        let v = unit_vector(g);
        let w = v - n * v.dot(n);
        if w.norm() > 0.3 {
            break w.normalize();
        }
    }
}

/// **The counterexample search at scale, on the tube circles** (review
/// R1, R2): a torus in a generic frame, up to a few hundred metres across
/// and as far from the origin, against a partner whose spine meets its
/// core, or passes it within a few bands, at a core point that is
/// random or exactly on a cut of the sweep's first cells: a sphere
/// centred there, a cylinder whose axis crosses the core there, a torus
/// whose core crosses it or runs along it (a pipe joint). The partner's
/// radius sets the level `|ρ₁ − ρ₂|` on the least distance, within the
/// zero band, across the ambiguity band, a few escalation widths off, or
/// anywhere. As the first search: a tangent reading has an oracle gap
/// within the zero band, a transverse one at least the escalation width.
/// Two tori read alike in either order. Mutant: a sign of `G′` read past
/// its rounding (a joint in the band reads transverse).
#[test]
fn no_tube_circle_pose_at_scale_reads_the_wrong_verdict() {
    let mut g = fuzz::start("torus_gap::tube_circles_at_scale");
    let b = band();
    let mut tally = [0usize; 3];
    for k in 0..fuzz::scaled(120) {
        let scale = (g.range(0.0, 1.0) * 500f64.ln()).exp();
        let big = scale * g.range(1.0, 2.0);
        let r = big * g.range(0.1, 0.6);
        let far = scale * g.range(0.0, 2.0);
        let axis = unit_vector(&mut g);
        let u_ref = square_to(&mut g, axis);
        let center = p(0.0, 0.0, 0.0) + unit_vector(&mut g) * far;
        let t = Surface::Torus {
            center,
            axis,
            major_radius: big,
            minor_radius: r,
            u_ref,
        };
        // The core point, in the sweep's own frame: on a first cut or
        // anywhere.
        let phi = if g.unit() < 0.5 {
            TAU / 32.0 * f64::from(u32::try_from(g.below(32)).unwrap())
        } else {
            g.range(0.0, TAU)
        };
        let v = axis.cross(u_ref);
        let q = center + (u_ref * phi.cos() + v * phi.sin()) * big;
        let tangent = v * phi.cos() - u_ref * phi.sin();
        let off = match g.below(3) {
            0 => 0.0,
            1 => g.range(0.0, 3.0) * b.zero(),
            _ => g.range(0.0, 3.0) * b.escalate(),
        };
        let q = q + unit_vector(&mut g) * off;
        let delta = match g.below(5) {
            0 => g.range(-0.5, 0.5) * b.zero(),
            1 => g.range(1.0, 10.0) * b.zero(),
            2 => g.range(-1.0, 1.0) * b.escalate(),
            3 => g.range(2.0, 6.0) * b.escalate(),
            _ => g.range(0.0, 0.3) * r,
        };
        let kind = k % 4;
        let partner = match kind {
            0 => sphere(q, r + delta),
            1 => cylinder(q, unit_vector(&mut g), r + delta),
            _ => {
                let big2 = scale * g.range(0.8, 2.0);
                // A joint runs along the core (one tangent), a crossing
                // meets it at a generic angle.
                let along = kind == 2;
                let axis2 = if along {
                    square_to(&mut g, tangent)
                } else {
                    unit_vector(&mut g)
                };
                let radial = if along {
                    axis2.cross(tangent)
                } else {
                    square_to(&mut g, axis2)
                };
                let rho = r - delta;
                if !(rho > 0.02 * big2 && rho < 0.9 * big2) {
                    continue;
                }
                Surface::Torus {
                    center: q + radial * big2,
                    axis: axis2,
                    major_radius: big2,
                    minor_radius: rho,
                    u_ref: square_to(&mut g, axis2),
                }
            }
        };
        let rho = match partner {
            Surface::Sphere { radius, .. } | Surface::Cylinder { radius, .. } => radius,
            Surface::Torus { minor_radius, .. } => minor_radius,
            _ => unreachable!(),
        };
        if rho <= 0.0 {
            continue;
        }
        let gap = oracle_gap(&values_of(&t, &partner), [r, rho]);
        let slack = 1e-11 + 1e-14 * (scale + far);
        let mut reads = Vec::new();
        for (x, y) in [(&t, &partner), (&partner, &t)] {
            let read = gap_of(x, y);
            let what = format!(
                "pose {k} (kind {kind}, scale {scale:.1}, gap {gap:e}): {t:?}, {partner:?}"
            );
            match read {
                Some(true) => assert!(
                    gap <= b.zero() + slack,
                    "{what}: read tangent; {}",
                    fuzz::replay()
                ),
                Some(false) => assert!(
                    gap >= b.escalate() - slack,
                    "{what}: read transverse; {}",
                    fuzz::replay()
                ),
                None => {}
            }
            tally[match read {
                Some(true) => 0,
                Some(false) => 1,
                None => 2,
            }] += 1;
            reads.push(read);
        }
        if kind >= 2 {
            assert_eq!(
                reads[0],
                reads[1],
                "pose {k}: two tori read apart by their order: {t:?}, {partner:?}; {}",
                fuzz::replay()
            );
        }
    }
    println!("[torus gap at scale] tangent, transverse, escalated: {tally:?}");
}

/// **Two tori read alike in either order** (review): a general pose
/// clearly transverse, one tangent at a crossing of their cores in the
/// band's zero, one a pipe joint in the ambiguity band, the designer's
/// tilt of the donut, and two poses of the search at scale whose orders
/// read apart when one core alone was swept (a joint `1.03e-9` off, read
/// escalated one way and transverse the other, and a crossing `3.7e-9`
/// off, escalated one way and tangent the other), and one the search
/// found reading apart with the rounding charged (a crossing whose first
/// sweep escalates). Each order reads the same verdict, and none
/// contradicts the oracle's gap. Mutant: the first core alone swept (the
/// last pose reads apart).
#[test]
fn two_tori_read_alike_in_either_order() {
    let b = band();
    let z = b.zero();
    let ring = |c: [f64; 3], a: [f64; 3], u: [f64; 3], big: f64, r: f64| Surface::Torus {
        center: p(c[0], c[1], c[2]),
        axis: Vec3::new(a[0], a[1], a[2]),
        major_radius: big,
        minor_radius: r,
        u_ref: Vec3::new(u[0], u[1], u[2]),
    };
    let donut = torus(p(0.0, 0.0, 0.0), Vec3::unit_z(), 2.0, 0.5);
    let pairs = [
        (
            donut.clone(),
            torus(p(3.0, 0.5, 0.2), Vec3::new(0.3, 1.0, 0.2), 1.2, 0.3),
        ),
        (
            donut.clone(),
            torus(p(2.0, 0.0, 1.0), Vec3::unit_y(), 1.0, 0.5 + 0.3 * z),
        ),
        (
            donut.clone(),
            torus(
                p(2.0, -1.0, 0.0),
                Vec3::new(0.6, 0.0, 0.8),
                1.0,
                0.5 - 4.0 * z,
            ),
        ),
        (
            donut.clone(),
            torus(p(0.4, -0.3, 0.1), Vec3::new(0.05, 0.02, 1.0), 2.1, 0.45),
        ),
        (
            ring(
                [0.7371994969736253, 1.4071667225033466, 1.4449639323225372],
                [
                    -0.41241404402180415,
                    0.030492610618506687,
                    -0.9104860553524416,
                ],
                [-0.7653690279720402, 0.5304606594209075, 0.36444717014660744],
                2.8256798722829823,
                1.572088476216037,
            ),
            ring(
                [-1.7930129175338143, 0.4987117323437381, 0.35164523138721826],
                [
                    -0.9049781751821206,
                    -0.42501517535980887,
                    0.019406265944530144,
                ],
                [0.1353314011127205, -0.24431587734531132, 0.9602057924996386],
                2.1408012268769703,
                1.5720884758097045,
            ),
        ),
        (
            ring(
                [-186.04799181930048, -73.14100957154437, 12.843183465053569],
                [0.5335272141162449, 0.3120727421397563, 0.7861038833454085],
                [
                    -0.8099515859890302,
                    -0.07914494365431496,
                    0.5811320901893217,
                ],
                200.72883307104772,
                22.92456949431801,
            ),
            ring(
                [-301.5981510112282, -134.84928956325714, -35.18115106385696],
                [0.8756148088107871, 0.4575502315307861, 0.15474654185917303],
                [0.3026883017074579, -0.2701320080609617, -0.914006832704443],
                174.05431515601282,
                22.92456948965355,
            ),
        ),
        (
            ring(
                [6.165966375727142, 4.5263691008133975, 1.3027087643139867],
                [0.2830071901442608, -0.7804203627697167, -0.5575401220549366],
                [0.9117458973847763, 0.399337197098489, -0.09617287359516445],
                121.66018205674065,
                65.792628432848,
            ),
            ring(
                [33.57617817131589, 0.8525779418705071, -65.00940475456076],
                [
                    0.38043382389596564,
                    -0.28168272117574406,
                    -0.8808660228587101,
                ],
                [-0.560676405109135, -0.8277281130958148, 0.02254195077492089],
                82.87039731372161,
                65.7926284329852,
            ),
        ),
    ];
    for (i, (x, y)) in pairs.iter().enumerate() {
        let read = gap_of(x, y);
        assert_eq!(read, gap_of(y, x), "pose {i}");
        let (
            Surface::Torus {
                minor_radius: r1, ..
            },
            Surface::Torus {
                minor_radius: r2, ..
            },
        ) = (x, y)
        else {
            unreachable!()
        };
        let gap = oracle_gap(&values_of(x, y), [*r1, *r2]);
        match read {
            Some(true) => assert!(gap <= z + 1e-11, "pose {i}: tangent at {gap:e}"),
            Some(false) => assert!(
                gap >= b.escalate() - 1e-11,
                "pose {i}: transverse at {gap:e}"
            ),
            None => {}
        }
    }
}
