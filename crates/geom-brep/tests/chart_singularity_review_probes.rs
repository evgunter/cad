//! Review probes for PR 6054 (`geom_brep::chart_singularity::locate`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_4, TAU};

use geom::{Curve3, Surface};
use geom_brep::chart_singularity::{Lies, Located, locate};
use geom_brep::props::LoopEdge;
use geom_core::{Band, Point3, Vec3};

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn frame(n: Vec3<f64>) -> Vec3<f64> {
    let side = if n.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let u = side.cross(n);
    u / u.norm()
}

fn section(n: Vec3<f64>, d: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0) + n * d,
        axis: n,
        radius: (1.0 - d * d).sqrt(),
        u_ref: frame(n),
    }
}

/// The same circle as a sampled carrier (an ellipse with equal semis).
fn section_sampled(n: Vec3<f64>, d: f64) -> Curve3<f64> {
    let r = (1.0 - d * d).sqrt();
    Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 0.0) + n * d,
        axis: n,
        semi_u: r,
        semi_v: r,
        u_ref: frame(n),
    }
}

fn run(
    s: &Surface<f64>,
    loops: &[Vec<LoopEdge<f64>>],
    sense: bool,
    band: Band,
) -> Vec<Located<f64>> {
    let views: Vec<&[LoopEdge<f64>]> = loops.iter().map(Vec::as_slice).collect();
    locate(s, &views, sense, band)
}

/// **C6: the `Carrier` exemption hides a decidable interior pole.** A
/// tilted cap whose rim passes 0.004 m from the north pole holds it,
/// 1e6 band-widths clear of the boundary. Stated as a circle it reads
/// `Interior`; stated as the same circle on a sampled carrier it reads
/// `Unread::Carrier`, which tier 3 exempts.
#[test]
fn review_sampled_rim_a_centimetre_from_the_pole_still_places_it() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let tilt = 0.3_f64;
    let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    // Pole angular distance to the rim: acos(d) - tilt = 0.01.
    let d = (tilt + 0.004).cos();
    let exact = run(
        &unit_sphere(),
        &[vec![LoopEdge::hand_built(
            section(n, d),
            0.0,
            TAU,
            true,
            0,
            0,
        )]],
        true,
        band,
    );
    assert!(
        matches!(exact[0].lies, Ok(Lies::Interior)),
        "{:?}",
        exact[0].lies
    );
    let sampled = run(
        &unit_sphere(),
        &[vec![LoopEdge::hand_built(
            section_sampled(n, d),
            0.0,
            TAU,
            true,
            0,
            0,
        )]],
        true,
        band,
    );
    assert!(
        matches!(sampled[0].lies, Ok(Lies::Interior)),
        "the same cap on a sampled carrier: {:?}",
        sampled[0].lies
    );
}

/// **C1 sweep**: tilted caps whose rim passes at distance `gap` from the
/// north pole (inside or outside), both traversals and senses, three
/// bands. Ground truth from the plane test.
#[test]
fn review_tilted_caps_match_the_plane_test() {
    let mut bad = Vec::new();
    for (zero, esc) in [(1e-9, 1e-8), (1e-6, 1e-5), (1e-12, 1e-11)] {
        let band = Band::new(zero, esc).unwrap();
        for tilt in [0.05_f64, 0.3, 1.0, 1.5] {
            for gap in [
                0.3,
                1e-2,
                1e-4,
                100.0 * esc,
                -100.0 * esc,
                -1e-4,
                -1e-2,
                -0.3,
            ] {
                // gap > 0: pole inside the cap about n (angular radius acos d).
                let rho = tilt + gap;
                if !(0.01..core::f64::consts::PI).contains(&rho) {
                    continue;
                }
                let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
                let d = rho.cos();
                for forward in [true, false] {
                    for sense in [true, false] {
                        let got = run(
                            &unit_sphere(),
                            &[vec![LoopEdge::hand_built(
                                section(n, d),
                                0.0,
                                TAU,
                                forward,
                                0,
                                0,
                            )]],
                            sense,
                            band,
                        );
                        // Cap side (n·p > d) is left of the CCW-about-n rim
                        // under the outward normal.
                        let face_is_cap = forward == sense;
                        for l in &got {
                            let inside_cap = n.dot(l.point - Point3::new(0.0, 0.0, 0.0)) > d;
                            let want = if inside_cap == face_is_cap {
                                Lies::Interior
                            } else {
                                Lies::Outside
                            };
                            match &l.lies {
                                Ok(x) if *x == want => {}
                                other => bad.push(format!(
                                    "band {zero} tilt {tilt} gap {gap} fwd {forward} sense {sense} {:?}: {other:?} want {want:?}",
                                    l.singularity
                                )),
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "{} wrong:\n{}", bad.len(), bad.join("\n"));
}

/// Great circle with unit normal `n`, parametrized from `u_ref`.
fn great(n: Vec3<f64>, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: n,
        radius: 1.0,
        u_ref,
    }
}

/// The great-circle arc from `p` to `q` (short way), as a loop edge.
fn arc(p: Point3<f64>, q: Point3<f64>, s: u32, e: u32) -> LoopEdge<f64> {
    let (a, b) = (
        p - Point3::new(0.0, 0.0, 0.0),
        q - Point3::new(0.0, 0.0, 0.0),
    );
    let n = a.cross(b);
    let n = n / n.norm();
    let ang = a.cross(b).norm().atan2(a.dot(b));
    LoopEdge::hand_built(great(n, a), 0.0, ang, true, s, e)
}

/// **C2 class (thin face near a pole)**: a sliver `w` wide between two
/// nearly parallel great-circle arcs passing `delta` from the north pole
/// (the pole is clear of the sliver by `delta`, far past the band). One
/// side is one edge, the other four: the chart polygon's chords on the
/// coarse side bulge past the fine side. Under the sense that makes the
/// face the sliver, both poles must read `Outside`.
#[test]
fn review_a_sliver_near_a_pole_reads_the_poles_outside() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let mut bad = Vec::new();
    for (delta, w) in [
        (0.1, 0.2),
        (0.01, 0.01),
        (0.01, 1e-3),
        (1e-3, 1e-4),
        (1e-3, 1e-5),
        (1e-3, 1e-6),
        (1e-3, 3e-7),
        (1e-2, 1e-5),
        (1e-4, 1e-7),
    ] {
        // Great circle at angular distance `dist` from the pole, its
        // closest point toward -x.
        let side = |dist: f64| {
            let n = Vec3::new(dist.cos(), 0.0, dist.sin()); // n·z = sin dist
            let u_ref = Vec3::new(-dist.sin(), 0.0, dist.cos()); // closest to pole
            (n, u_ref)
        };
        let half = delta * 3.0_f64.sqrt(); // ±60° seen from the pole
        let (na, ua) = side(delta);
        let (nb, ub) = side(delta + w);
        let pt = |n: Vec3<f64>, u: Vec3<f64>, t: f64| {
            Point3::new(0.0, 0.0, 0.0) + u * t.cos() + n.cross(u) * t.sin()
        };
        let a_edge = LoopEdge::hand_built(great(na, ua), -half, half, true, 0, 1);
        let mut edges = vec![a_edge, arc(pt(na, ua, half), pt(nb, ub, half), 1, 2)];
        let k = 4;
        for i in 0..k {
            let t1 = half - 2.0 * half * f64::from(i) / f64::from(k);
            let t0 = half - 2.0 * half * f64::from(i + 1) / f64::from(k);
            edges.push(LoopEdge::hand_built(
                great(nb, ub),
                t0,
                t1,
                false,
                2 + i,
                3 + i,
            ));
        }
        edges.push(arc(pt(nb, ub, -half), pt(na, ua, -half), 2 + k, 0));
        // True orientation: projected area seen from +z (outward at the pole).
        let mut proj = Vec::new();
        for e in &edges {
            let (a, b) = if e.forward {
                (e.t0, e.t1)
            } else {
                (e.t1, e.t0)
            };
            for j in 0..2000 {
                let p = e.carrier.eval(a + (b - a) * f64::from(j) / 2000.0);
                proj.push((p.x, p.y));
            }
        }
        let mut area = 0.0;
        for j in 0..proj.len() {
            let (p, q) = (proj[j], proj[(j + 1) % proj.len()]);
            area += p.0 * q.1 - q.0 * p.1;
        }
        let sliver_sense = area > 0.0;
        let got = run(&unit_sphere(), &[edges.clone()], sliver_sense, band);
        for l in &got {
            if !matches!(l.lies, Ok(Lies::Outside)) {
                bad.push(format!(
                    "delta {delta} w {w} (true area {area:e}): {:?} {:?}",
                    l.singularity, l.lies
                ));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle: FRAC_PI_4,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn parallel(z: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: z.abs(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **Cone orientation pinned** (the PR's own row sorts the two answers,
/// so it survives inverting the apex side of every winding loop): the
/// chart normal is outward on both nappes, and the left of `+u` is `+v`,
/// so a rim run east holds the apex on its left on the negative nappe
/// only.
#[test]
fn review_cone_rim_sides_are_pinned_on_both_nappes() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let read = |z: f64, forward: bool| {
        run(
            &cone(),
            &[vec![LoopEdge::hand_built(
                parallel(z),
                0.0,
                TAU,
                forward,
                0,
                0,
            )]],
            true,
            band,
        )[0]
        .lies
        .clone()
    };
    // Positive nappe, rim run east (+u): left is +v (away from apex).
    assert!(
        matches!(read(1.0, true), Ok(Lies::Outside)),
        "{:?}",
        read(1.0, true)
    );
    assert!(
        matches!(read(1.0, false), Ok(Lies::Interior)),
        "{:?}",
        read(1.0, false)
    );
    // Negative nappe: the parallel at z = -1 with u_ref +x is at u = π
    // for chart point v<0; run east about +z its chart u still increases.
    assert!(
        matches!(read(-1.0, true), Ok(Lies::Interior)),
        "{:?}",
        read(-1.0, true)
    );
    assert!(
        matches!(read(-1.0, false), Ok(Lies::Outside)),
        "{:?}",
        read(-1.0, false)
    );
}

/// **A winding-free cone patch**: two generators and two parallels on the
/// positive nappe, a quarter turn wide, both traversals. The apex is
/// outside the patch; under the reversed traversal the face is the
/// complement, which holds it.
#[test]
fn review_a_cone_patch_reads_the_apex_by_its_orientation() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let gen_ = |u: f64| Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(u.cos(), u.sin(), 1.0),
    };
    let (u0, u1) = (0.2, 0.2 + core::f64::consts::FRAC_PI_2);
    // Counter-clockwise in (u, v): lower parallel east, generator u1 up,
    // upper parallel west, generator u0 down.
    let ccw = vec![
        LoopEdge::hand_built(parallel(1.0), u0, u1, true, 0, 1),
        LoopEdge::hand_built(gen_(u1), 1.0, 2.0, true, 1, 2),
        LoopEdge::hand_built(parallel(2.0), u0, u1, false, 2, 3),
        LoopEdge::hand_built(gen_(u0), 1.0, 2.0, false, 3, 0),
    ];
    let got = run(&cone(), &[ccw.clone()], true, band);
    assert!(
        matches!(got[0].lies, Ok(Lies::Outside)),
        "patch: {:?}",
        got[0].lies
    );
    let got = run(&cone(), &[ccw], false, band);
    assert!(
        matches!(got[0].lies, Ok(Lies::Interior)),
        "complement: {:?}",
        got[0].lies
    );
}

/// **A meridian spur short of the pole**: the cap above latitude
/// asin(0.5) with a spur from its rim up the meridian at u = 0 to
/// latitude 1.5, out and back. It holds the north pole inside.
#[test]
fn review_a_spur_short_of_the_pole_leaves_it_inside() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let rim = section(Vec3::new(0.0, 0.0, 1.0), 0.5);
    let meridian = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let lat0 = 0.5_f64.asin();
    // `section`'s u_ref for n = z is x × z = -y: the rim starts at -y.
    // rim from -y round to +x (a quarter turn), spur up and back, rest of rim.
    let edges = vec![
        LoopEdge::hand_built(rim.clone(), 0.0, 0.25 * TAU, true, 0, 1),
        LoopEdge::hand_built(meridian.clone(), lat0, 1.5, true, 1, 2),
        LoopEdge::hand_built(meridian, lat0, 1.5, false, 2, 1),
        LoopEdge::hand_built(rim, 0.25 * TAU, TAU, true, 1, 0),
    ];
    let got = run(&unit_sphere(), &[edges], true, band);
    assert!(
        matches!(got[0].lies, Ok(Lies::Interior)),
        "{:?}",
        got[0].lies
    );
    assert!(
        matches!(got[1].lies, Ok(Lies::Outside)),
        "{:?}",
        got[1].lies
    );
}
