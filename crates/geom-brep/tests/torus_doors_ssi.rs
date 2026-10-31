//! **cylinder × torus and torus × torus on the ℝ³ implicit pair**
//! (`ssi::cylinder_torus_ssi`, `ssi::torus_torus_ssi`): each pose
//! decided once, marched past a transverse critical gap over the ball
//! that holds its section (`PairPose::ball`), and every branch closed
//! and certified.
//!
//! Each branch is read against an oracle of its own: the exact distance
//! of a point from a cylinder (`| |w⊥| − r |`) and from a torus
//! (`| √((ρ − R)² + z²) − r |`), sampled along the carrier, which no
//! limb of the certificate reads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/ssi.rs",
    "crates/geom-brep/src/ssi/",
    "crates/geom-brep/src/pair_section.rs",
    "crates/geom-brep/src/pair_section/",
    "crates/geom-brep/tests/shared/"
];

use crate::shared::tol::band;
use geom::Surface;
use geom_brep::ssi::{self, SsiDomain, SsiError, TangentPair, TorusPair};
use geom_brep::{
    ClassCarrier, CylinderTorusPose, PairPose, SectionRead, SsiLane, SsiOutcome, TorusGap,
    TorusTorusPose,
};
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

fn cylinder(origin: Point3<f64>, axis: Vec3<f64>, radius: f64) -> Surface<f64> {
    let u_ref = axis.cross(Vec3::new(0.3, 0.5, 0.7)).normalize();
    Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref,
    }
}

fn torus(center: Point3<f64>, axis: Vec3<f64>, big: f64, small: f64) -> Surface<f64> {
    let u_ref = axis.cross(Vec3::new(0.3, 0.5, 0.7)).normalize();
    Surface::Torus {
        center,
        axis,
        major_radius: big,
        minor_radius: small,
        u_ref,
    }
}

/// The exact distance of `p` from `s`, a cylinder or a torus.
fn distance(s: &Surface<f64>, p: Point3<f64>) -> f64 {
    match *s {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let a = axis.normalize();
            let w = p - origin;
            ((w - a * w.dot(a)).norm() - radius).abs()
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let a = axis.normalize();
            let w = p - center;
            let z = w.dot(a);
            let rho = (w - a * z).norm();
            ((rho - major_radius).hypot(z) - minor_radius).abs()
        }
        _ => unreachable!("a cylinder or a torus"),
    }
}

/// **The pair's door** over its decided pose's domain
/// (`PairPose::domain`, the section value's own), in the order given.
fn door(a: &Surface<f64>, b: &Surface<f64>) -> Result<SsiOutcome, SsiError> {
    match (a, b) {
        (Surface::Torus { .. }, Surface::Torus { .. }) => {
            match TorusTorusPose::decide(a, b, band()).expect("two tori") {
                Ok(pose) => ssi::torus_torus_ssi(a, b, PairPose::TorusTorus(pose).domain(), band()),
                Err(e) => Err(SsiError::of_torus_pose(TorusPair::TorusTorus, e)),
            }
        }
        _ => match CylinderTorusPose::decide(a, b, band()).expect("a cylinder and a torus") {
            Ok(pose) => {
                ssi::cylinder_torus_ssi(a, b, PairPose::CylinderTorus(pose).domain(), band())
            }
            Err(e) => Err(SsiError::of_torus_pose(TorusPair::CylinderTorus, e)),
        },
    }
}

/// **Every branch closed, certified, and on both surfaces**: sampled
/// along its carrier, each point's exact distance from each operand is
/// within the certificate's limb-2 bound, and that bound within the band.
fn on_both(what: &str, out: &SsiOutcome, a: &Surface<f64>, b: &Surface<f64>) {
    for (k, branch) in out.branches.iter().enumerate() {
        assert!(
            matches!(branch.end, geom_brep::ssi::BranchEnd::Closed),
            "{what}: branch {k} closes inside the ball"
        );
        let sup = branch.certificate.hull_sup;
        assert!(sup <= band().zero(), "{what}: branch {k}'s bound {sup:e}");
        let (t0, t1) = branch.params;
        for i in 0..=512 {
            let p = branch.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 512.0);
            let slack = 16.0 * f64::EPSILON * (p - Point3::origin()).norm().max(1.0);
            for s in [a, b] {
                let d = distance(s, p);
                assert!(
                    d <= sup + slack,
                    "{what}: branch {k} sample {i} lies {d:e} from the {:?}, past its \
                     certified {sup:e}",
                    s.kind()
                );
            }
        }
    }
}

// ---------------------------------------------------------------------
// The germ row's placements
// ---------------------------------------------------------------------

/// `torus-germ-pairs-have-no-section-frame`'s six placements: B turned
/// by `angle` about `axis` through the origin, then moved by `t`.
#[allow(clippy::approx_constant)] // the row's second angle is a hair short of a right one
fn placements() -> [(f64, Vec3<f64>, Vec3<f64>); 6] {
    [
        (0.0, Vec3::unit_x(), Vec3::new(0.45, 0.1, 0.05)),
        (1.570_796_3, Vec3::unit_x(), Vec3::new(0.3, 0.2, 0.1)),
        (0.7, Vec3::unit_x(), Vec3::new(0.1, 0.35, 0.2)),
        (
            1.1,
            Vec3::new(0.3, 1.0, 0.2).normalize(),
            Vec3::new(0.42, -0.15, 0.3),
        ),
        (0.4, Vec3::unit_y(), Vec3::new(0.6, 0.0, 0.0)),
        (
            0.9,
            Vec3::new(1.0, 1.0, 0.0).normalize(),
            Vec3::new(0.05, 0.1, 0.02),
        ),
    ]
}

/// Rodrigues' turn of `v` by `angle` about the unit `k`.
fn turn(v: Vec3<f64>, (angle, k): (f64, Vec3<f64>)) -> Vec3<f64> {
    let (s, c) = angle.sin_cos();
    v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c))
}

/// The drum's wall, radius `1/2` about `z`.
fn drum(place: Option<(f64, Vec3<f64>, Vec3<f64>)>) -> Surface<f64> {
    match place {
        None => cylinder(Point3::origin(), Vec3::unit_z(), 0.5),
        Some((a, k, t)) => cylinder(Point3::origin() + t, turn(Vec3::unit_z(), (a, k)), 0.5),
    }
}

/// The donut's wall, `R = 0.6`, `r = 0.2` about `y`.
fn donut(place: Option<(f64, Vec3<f64>, Vec3<f64>)>) -> Surface<f64> {
    match place {
        None => torus(Point3::origin(), Vec3::unit_y(), 0.6, 0.2),
        Some((a, k, t)) => torus(Point3::origin() + t, turn(Vec3::unit_y(), (a, k)), 0.6, 0.2),
    }
}

/// What a door answered, in a word the rows pin.
fn verdict(got: &Result<SsiOutcome, SsiError>) -> String {
    match got {
        Ok(out) => format!("{} loops", out.branches.len()),
        Err(SsiError::PairTangent { pair, .. }) => format!("tangent ({pair:?})"),
        Err(SsiError::TubeStraddles { .. }) => "tube straddles".to_owned(),
        Err(SsiError::StepBudget { .. }) => "step budget".to_owned(),
        Err(e) => format!("refused: {e}"),
    }
}

/// The signed distance of `p` from `s` (positive outside), a cylinder or
/// a torus.
fn signed(s: &Surface<f64>, p: Point3<f64>) -> f64 {
    match *s {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let a = axis.normalize();
            let w = p - origin;
            (w - a * w.dot(a)).norm() - radius
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let a = axis.normalize();
            let w = p - center;
            let z = w.dot(a);
            ((w - a * z).norm() - major_radius).hypot(z) - minor_radius
        }
        _ => unreachable!("a cylinder or a torus"),
    }
}

/// **The oracle's count of the section's loops**: the partner's signed
/// distance sampled on an `n × n` grid of the torus's own angles, and the
/// grid edges whose ends differ in sign joined through the cells they
/// bound (marching squares on the periodic chart). Each loop of the
/// section is one class of crossed edges, at a grid fine enough that no
/// cell holds a saddle; a cell crossed four times refuses the count.
fn oracle_loops(t: &Surface<f64>, partner: &Surface<f64>, n: usize) -> usize {
    let Surface::Torus {
        center,
        axis,
        major_radius: big,
        minor_radius: small,
        u_ref,
    } = *t
    else {
        unreachable!("the torus first")
    };
    let a = axis.normalize();
    let u = (u_ref - a * u_ref.dot(a)).normalize();
    let v = a.cross(u);
    let f: Vec<f64> = (0..n * n)
        .map(|k| {
            let (i, j) = (k / n, k % n);
            let (th, ph) = (
                core::f64::consts::TAU * i as f64 / n as f64,
                core::f64::consts::TAU * j as f64 / n as f64,
            );
            let radial = u * th.cos() + v * th.sin();
            let p = center + radial * (big + small * ph.cos()) + a * (small * ph.sin());
            signed(partner, p)
        })
        .collect();
    let at = |i: usize, j: usize| f[(i % n) * n + (j % n)];
    // Edge ids: `2·(i·n + j)` the edge from (i, j) to (i+1, j), `+1` the
    // one to (i, j+1).
    let crossed = |e: usize| {
        let (c, dir) = (e / 2, e % 2);
        let (i, j) = (c / n, c % n);
        let (x, y) = if dir == 0 { (i + 1, j) } else { (i, j + 1) };
        (at(i, j) > 0.0) != (at(x, y) > 0.0)
    };
    let mut parent: Vec<usize> = (0..2 * n * n).collect();
    fn root(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    for i in 0..n {
        for j in 0..n {
            let id = |i: usize, j: usize, dir: usize| 2 * ((i % n) * n + (j % n)) + dir;
            let edges = [id(i, j, 0), id(i, j, 1), id(i + 1, j, 1), id(i, j + 1, 0)];
            let hit: Vec<usize> = edges.into_iter().filter(|&e| crossed(e)).collect();
            assert!(
                hit.len() != 4,
                "a saddle cell at ({i}, {j}): refine the grid"
            );
            if let [x, y] = hit[..] {
                let (rx, ry) = (root(&mut parent, x), root(&mut parent, y));
                parent[rx] = ry;
            }
        }
    }
    let mut roots: Vec<usize> = (0..2 * n * n)
        .filter(|&e| crossed(e))
        .map(|e| root(&mut parent, e))
        .collect();
    roots.sort_unstable();
    roots.dedup();
    roots.len()
}

/// **One germ-row pair at every placement**: `pair(place)` is the two
/// surfaces, the torus first; each pose is decided once and marched over
/// its domain, in both orders, which answer alike. Every certified pose's
/// branches lie on both surfaces and number the oracle's loops, and the
/// section value carries as many components, marched or counted. The
/// verdicts are `want`, by placement, at the compiled ε; `off` names the
/// placements whose verdict differs at another ε row, measured: at
/// `1e-6` a transverse pose may refuse on the tube, at `1e-12` on the
/// step budget
/// (`work/sectrung/the-torus-doors-tube-straddles-transverse-poses-at-a-coarse-band.md`).
fn germ_row(
    pair: impl Fn(Option<(f64, Vec3<f64>, Vec3<f64>)>) -> [Surface<f64>; 2],
    want: [&str; 6],
    off: &[(f64, usize, &str)],
) {
    let eps = crate::shared::tol::eps();
    for (i, place) in placements().into_iter().enumerate() {
        let [a, b] = pair(Some(place));
        let got = door(&a, &b);
        let what = format!("placement {i}");
        if let Ok(out) = &got {
            on_both(&what, out, &a, &b);
            assert_eq!(
                out.branches.len(),
                oracle_loops(&a, &b, 600),
                "{what}: the oracle's loop count"
            );
            let Ok(SectionRead::Built(section)) = geom_brep::pair_section(&a, &b, band()) else {
                panic!("{what}: the value builds the pair");
            };
            let carried = section
                .carry(Some(SsiLane::marched()), band())
                .unwrap_or_else(|e| panic!("{what}: {e:?}"));
            assert_eq!(
                carried.components().len(),
                out.branches.len(),
                "{what}: the carried section lists the door's loops"
            );
        }
        assert_eq!(verdict(&door(&b, &a)), verdict(&got), "{what}, reversed");
        let expected = off
            .iter()
            .find(|&&(e, k, _)| e == eps && k == i)
            .map_or(want[i], |&(_, _, v)| v);
        assert_eq!(verdict(&got), expected, "{what} at ε {eps:e}");
    }
}

/// **The germ row's donut against its moved drum**: every placement
/// certifies, at every ε row.
#[test]
fn the_germ_rows_donut_against_its_moved_drum() {
    germ_row(
        |p| [donut(None), drum(p)],
        [
            "2 loops", "2 loops", "2 loops", "2 loops", "2 loops", "1 loops",
        ],
        &[],
    );
}

/// **The germ row's moved donut against its drum**: every placement
/// certifies at the compiled ε; at `1e-6` three refuse on the tube.
#[test]
fn the_germ_rows_moved_donut_against_its_drum() {
    germ_row(
        |p| [donut(p), drum(None)],
        [
            "2 loops", "2 loops", "2 loops", "2 loops", "2 loops", "1 loops",
        ],
        &[
            (1e-6, 2, "tube straddles"),
            (1e-6, 3, "tube straddles"),
            (1e-6, 5, "tube straddles"),
        ],
    );
}

/// **The germ row's two donuts**: every placement certifies but the one
/// whose donuts share an axis line with each core through the other's
/// centre, a tangency the gap decides and the door refuses before any
/// rung, at every ε row; at `1e-6` two refuse on the tube, at `1e-12` one
/// on the step budget.
#[test]
fn the_germ_rows_two_donuts() {
    germ_row(
        |p| [donut(None), donut(p)],
        [
            "2 loops",
            "2 loops",
            "1 loops",
            "2 loops",
            "tangent (TorusTorus)",
            "1 loops",
        ],
        &[
            (1e-6, 2, "tube straddles"),
            (1e-6, 5, "tube straddles"),
            (1e-12, 5, "step budget"),
        ],
    );
}

/// **What a near-tangent transverse pose costs** (a measurement, run by
/// hand): a rod of the tube's radius whose axis passes `δ` from the
/// core, the straight pipe entering its bend parted by `δ`, marched at
/// each `δ`. Run with
/// `cargo nextest run -p geom-brep --test all --run-ignored only -E
/// 'test(near_tangent_cost)' --no-capture`.
#[test]
#[ignore = "a measurement: prints each pose's cost"]
fn near_tangent_cost() {
    let t = torus(Point3::origin(), Vec3::unit_z(), 1.0, 0.25);
    for delta in [1e-2, 1e-3, 1e-4, 1e-5, 1e-6] {
        let pipe = cylinder(Point3::new(1.0 + delta, 0.0, 0.0), Vec3::unit_y(), 0.25);
        let clock = std::time::Instant::now();
        let got = match CylinderTorusPose::decide(&t, &pipe, band()).unwrap() {
            Ok(pose) => {
                ssi::cylinder_torus_ssi(&t, &pipe, PairPose::CylinderTorus(pose).domain(), band())
                    .map(|o| (o.branches.len(), o.exhaustiveness.examined))
            }
            Err(e) => {
                eprintln!("δ = {delta:e}: the pose refuses {e:?}");
                continue;
            }
        };
        eprintln!(
            "δ = {delta:e}: {:?} in {:?}",
            got.map_err(|e| e.to_string()),
            clock.elapsed()
        );
    }
}

// ---------------------------------------------------------------------
// The radial hole
// ---------------------------------------------------------------------

/// **GERM's radial hole, marched.** The donut (`R = 2`, `r = 1/2` about
/// `y`) and a rod of radius `0.2` along `x`, through the axis and off it:
/// a square pose, whose classification counts its components in closed
/// form with the general rung's carrier. The door marches as many closed
/// loops as the classification counts, each on both surfaces, and the
/// section carries them, each loop matched to its count by its witness
/// or its winding.
#[test]
fn the_radial_hole_marches_the_square_walls_loops() {
    let t = torus(Point3::origin(), Vec3::unit_y(), 2.0, 0.5);
    for (what, h, e) in [("on the axis", 0.0, 0.0), ("off the axis", 0.1, 0.15)] {
        let rod = cylinder(Point3::new(0.0, h, e), Vec3::unit_x(), 0.2);
        let SectionRead::Built(section) = geom_brep::pair_section(&t, &rod, band()).unwrap() else {
            panic!("{what}: the value builds the pair");
        };
        assert!(
            section
                .counted()
                .expect("a counted section")
                .iter()
                .all(|c| matches!(c.carrier, ClassCarrier::General)),
            "{what}: every component is the general rung's"
        );
        let out = ssi::cylinder_torus_ssi(&t, &rod, section.pose().domain(), band())
            .unwrap_or_else(|e| panic!("{what}: {e}"));
        assert_eq!(
            out.branches.len(),
            section.counted().expect("a counted section").len(),
            "{what}: one loop per listed component"
        );
        on_both(what, &out, &t, &rod);
        let carried = section
            .carry(Some(SsiLane::marched()), band())
            .unwrap_or_else(|e| panic!("{what}: {e:?}"));
        assert_eq!(
            carried.components().len(),
            out.branches.len(),
            "{what}: the carried section lists the march's loops"
        );
    }
}

// ---------------------------------------------------------------------
// A symmetric pose carried
// ---------------------------------------------------------------------

/// **A parallel rod through the tube carries.** The torus `R = 1`,
/// `r = 1/4` about `z` and a rod of radius `0.1` parallel to it through
/// the core: two loops round the rod, one above the torus's plane and
/// one below. The carried section holds each as a certified fitted loop,
/// on both surfaces, winding about the rod and not the torus.
#[test]
fn a_rod_through_the_tube_carries_both_loops() {
    let t = torus(Point3::origin(), Vec3::unit_z(), 1.0, 0.25);
    let rod = cylinder(Point3::new(1.0, 0.05, 0.0), Vec3::unit_z(), 0.1);
    let SectionRead::Built(section) = geom_brep::pair_section(&rod, &t, band()).unwrap() else {
        panic!("the value builds the pair");
    };
    assert_eq!(
        section.counted().expect("a counted section").len(),
        2,
        "two loops round the rod"
    );
    let carried = section
        .carry(Some(SsiLane::marched()), band())
        .unwrap_or_else(|e| panic!("{e:?}"));
    let mut sides = Vec::new();
    for (k, c) in carried.components().iter().enumerate() {
        let cert = c.certificate.as_ref().expect("a fitted loop");
        assert!(c.closed, "component {k} closes");
        assert_eq!(c.winds, [true, false], "component {k} winds about the rod");
        let (t0, t1) = c.span;
        for i in 0..=256 {
            let p = c.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 256.0);
            for s in [&rod, &t] {
                assert!(
                    distance(s, p) <= cert.hull_sup + 1e-15,
                    "component {k} sample {i}"
                );
            }
        }
        sides.push(c.carrier.eval(t0).z > 0.0);
    }
    assert!(
        sides.contains(&true) && sides.contains(&false),
        "one loop on each side of the torus's plane: {sides:?}"
    );
}

/// **A parallel wall round the torus's hole winds the torus.** The torus
/// `R = 1`, `r = 1/4` about `z` and a wall of radius `1` whose axis
/// stands `0.1` off the torus's: its circle lies in the tube's shadow and
/// holds the torus axis, so each of its two loops (upper and lower) turns
/// once about the torus axis, a longitude, essential on the torus as on
/// the wall. Each carried loop's turning about the axis is counted along
/// it and must be what the classification says.
#[test]
fn a_parallel_wall_round_the_hole_winds_the_torus() {
    use core::f64::consts::{PI, TAU};
    let t = torus(Point3::origin(), Vec3::unit_z(), 1.0, 0.25);
    for (e, round) in [(0.1, true), (0.9, false)] {
        let rc = if round { 1.0 } else { 0.1 };
        let c = cylinder(Point3::new(e, 0.0, 0.0), Vec3::unit_z(), rc);
        let SectionRead::Built(s) = geom_brep::pair_section(&t, &c, band()).unwrap() else {
            panic!("the value builds the pair");
        };
        assert_eq!(
            s.counted().expect("a counted section").len(),
            2,
            "e = {e}: an upper and a lower loop"
        );
        let carried = s
            .carry(Some(SsiLane::marched()), band())
            .unwrap_or_else(|err| panic!("e = {e}: {err:?}"));
        for (k, comp) in carried.components().iter().enumerate() {
            let (t0, t1) = comp.span;
            let mut w = 0.0f64;
            let mut prev: Option<f64> = None;
            for i in 0..=2000 {
                let p = comp.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 2000.0);
                let th: f64 = p.y.atan2(p.x);
                if let Some(a) = prev {
                    w += (th - a + PI).rem_euclid(TAU) - PI;
                }
                prev = Some(th);
            }
            let turns = (w / TAU).round().abs() == 1.0;
            assert_eq!(turns, round, "e = {e}, component {k}: its turning");
            assert_eq!(
                comp.winds,
                [round, true],
                "e = {e}, component {k}: the carried winding"
            );
            assert_eq!(
                s.counted().expect("a counted section")[k].winds,
                [round, true],
                "e = {e}, component {k}"
            );
        }
    }
}

// ---------------------------------------------------------------------
// Refusals
// ---------------------------------------------------------------------

/// **A tangent pose refuses before any rung**, as the pair's tangency:
/// a straight pipe entering a bend (a rod of the tube's radius whose axis
/// touches the core) and two equal bends joined in different planes
/// (the tube-circle tangency).
#[test]
fn a_tube_circle_tangency_refuses_at_each_door() {
    // `u_ref = +x` puts the tangency on a cell edge of the gap's sweep,
    // where it decides at every ε; elsewhere it escalates at ε = 1e-12
    // (`work/sectrung/the-torus-gaps-rounding-charge-is-first-order-and-reads-the-world-origin.md`).
    let t = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 1.0,
        minor_radius: 0.25,
        u_ref: Vec3::unit_x(),
    };
    let pipe = cylinder(Point3::new(1.0, 0.0, 0.0), Vec3::unit_y(), 0.25);
    let pose = CylinderTorusPose::decide(&t, &pipe, band())
        .unwrap()
        .unwrap();
    let got = ssi::cylinder_torus_ssi(&t, &pipe, PairPose::CylinderTorus(pose).domain(), band());
    assert!(
        matches!(
            got,
            Err(SsiError::PairTangent {
                pair: TangentPair::CylinderTorus,
                ..
            })
        ),
        "the pipe: {got:?}"
    );
    assert!(
        got.unwrap_err()
            .to_string()
            .contains("the cylinder and the torus")
    );
    let bend = torus(Point3::new(2.0, 0.0, 0.0), Vec3::unit_y(), 1.0, 0.25);
    let pose = TorusTorusPose::decide(&t, &bend, band()).unwrap().unwrap();
    assert!(matches!(pose.gap(), TorusGap::Tangent { .. }), "the bends");
    let got = ssi::torus_torus_ssi(&t, &bend, PairPose::TorusTorus(pose).domain(), band());
    assert!(
        matches!(
            got,
            Err(SsiError::PairTangent {
                pair: TangentPair::TorusTorus,
                ..
            })
        ),
        "the bends: {got:?}"
    );
}

/// **Each door takes its own pair only, and two tori decided one are no
/// section.**
#[test]
fn each_torus_door_refuses_any_other_pair() {
    let t = torus(Point3::origin(), Vec3::unit_z(), 1.0, 0.25);
    let c = cylinder(Point3::origin(), Vec3::unit_x(), 0.1);
    let s = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let domain = SsiDomain {
        center: Point3::origin(),
        half_extent: 1.5,
        extent: 2.5,
        floor_scale: 1.0,
    };
    for (a, b) in [(&t, &t), (&s, &t), (&c, &c)] {
        let got = ssi::cylinder_torus_ssi(a, b, domain, band());
        assert!(matches!(got, Err(SsiError::WrongLane { .. })), "{got:?}");
    }
    for (a, b) in [(&c, &t), (&s, &t)] {
        let got = ssi::torus_torus_ssi(a, b, domain, band());
        assert!(matches!(got, Err(SsiError::WrongLane { .. })), "{got:?}");
    }
    let got = ssi::torus_torus_ssi(&t, &t.clone(), domain, band());
    assert!(
        matches!(got, Err(SsiError::CoincidentTori)),
        "one torus twice: {got:?}"
    );
}

// ---------------------------------------------------------------------
// The counterexample search
// ---------------------------------------------------------------------

/// A random unit vector.
fn direction(g: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0));
        let n = v.norm();
        if (0.1..1.0).contains(&n) {
            return v / n;
        }
    }
}

/// **No marched point leaves either surface by more than its certified
/// bound** (a counterexample search). Random cylinder × torus and torus
/// × torus poses about a torus `R = 1`, `r ∈ [0.1, 0.4]`: each pose the
/// door marches, it marches over its domain, and every branch it returns
/// is closed and lies, sampled, within its limb-2 bound of both surfaces
/// by the exact distance; the branches number the oracle's loops, and
/// the reversed order answers alike. A pose the door refuses is not a
/// counterexample: the search is for a certified point off a surface.
#[test]
fn no_certified_point_leaves_either_surface() {
    let mut g = fuzz::start("torus_doors_ssi::on_both");
    let mut marched = 0;
    for _ in 0..fuzz::scaled(6) {
        let a = torus(Point3::origin(), Vec3::unit_z(), 1.0, g.range(0.1, 0.4));
        let at = Point3::new(g.range(-1.2, 1.2), g.range(-1.2, 1.2), g.range(-0.3, 0.3));
        let c = cylinder(at, direction(&mut g), g.range(0.05, 0.4));
        let b = torus(at, direction(&mut g), g.range(0.5, 1.2), g.range(0.1, 0.4));
        for partner in [c, b] {
            let what = format!("{:?} × torus ({})", partner.kind(), fuzz::replay());
            let Ok(out) = door(&a, &partner) else {
                continue;
            };
            on_both(&what, &out, &a, &partner);
            assert_eq!(
                out.branches.len(),
                oracle_loops(&a, &partner, 600),
                "{what}: the oracle"
            );
            assert_eq!(
                verdict(&door(&partner, &a)),
                verdict(&Ok(out.clone())),
                "{what}: the order"
            );
            marched += 1;
        }
    }
    assert!(
        marched > 0,
        "the search marched nothing ({})",
        fuzz::replay()
    );
}
