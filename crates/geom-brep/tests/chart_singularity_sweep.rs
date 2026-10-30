//! **Where a chart singularity lies, swept**: the rows the dual review
//! of the chart-singularity reading (PR 6054) drove it with, adopted
//! here. Hand-built loops, each vertex where its carriers end
//! ([`traversal_corners`]); ground truth from each case's own geometry
//! (a cap's plane, a polygon's edge planes, a sliver's projected area).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::chart_singularity::{FaceLoop, Located, locate, traversal_corners};
use geom_brep::props::LoopEdge;
use geom_core::{Band, Point3};

/// [`locate`] over loops built by hand.
fn locate_hand_built(
    surface: &Surface<f64>,
    loops: &[Vec<LoopEdge<f64>>],
    sense: bool,
    band: Band,
) -> Vec<Located<f64>> {
    let corners: Vec<Vec<Point3<f64>>> = loops.iter().map(|l| traversal_corners(l)).collect();
    let views: Vec<FaceLoop<'_, f64>> = loops
        .iter()
        .zip(&corners)
        .map(|(edges, corners)| FaceLoop { edges, corners })
        .collect();
    locate(surface, &views, sense, band)
}

/// Spheres and cones read in closed form and by their walk: tilted
/// caps, a sliver beside a pole, a sampled rim, the cone's sides and a
/// patch on it, a spur short of the pole.
mod closed_form_and_walk {
    use core::f64::consts::{FRAC_PI_4, TAU};

    use geom::{Curve3, Surface};
    use geom_brep::chart_singularity::{Lies, Located};
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
        super::locate_hand_built(s, loops, sense, band)
    }

    /// **C6: the `Carrier` exemption hides a decidable interior pole.** A
    /// tilted cap whose rim passes 0.004 m from the north pole holds it,
    /// 1e6 band-widths clear of the boundary. Stated as a circle it reads
    /// `Interior`; stated as the same circle on a sampled carrier it reads
    /// `Unread::Carrier`, which tier 3 exempts.
    #[test]
    fn sampled_rim_a_centimetre_from_the_pole_still_places_it() {
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
    fn tilted_caps_match_the_plane_test() {
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
    fn a_sliver_near_a_pole_reads_the_poles_outside() {
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
    fn cone_rim_sides_are_pinned_on_both_nappes() {
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
    fn a_cone_patch_reads_the_apex_by_its_orientation() {
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
        let got = run(&cone(), core::slice::from_ref(&ccw), true, band);
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
    fn a_spur_short_of_the_pole_leaves_it_inside() {
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
}

/// Sampled edges beside closed-form ones, caps and lenses near and far
/// from the poles, convex polygons fuzzed against their edge planes.
mod sampled_and_fuzzed {
    use core::f64::consts::{FRAC_PI_2, TAU};

    use geom::{Curve3, Surface};
    use geom_brep::chart_singularity::{Lies, Located, Singularity, Unread};
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

    fn section(n: Vec3<f64>, d: f64) -> Curve3<f64> {
        let side = if n.z.abs() < 0.9 {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let u_ref = side.cross(n);
        Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0) + n * d,
            axis: n,
            radius: (1.0 - d * d).sqrt(),
            u_ref: u_ref / u_ref.norm(),
        }
    }

    fn run(loops: &[Vec<LoopEdge<f64>>], sense: bool, band: Band) -> Vec<Located<f64>> {
        super::locate_hand_built(&unit_sphere(), loops, sense, band)
    }

    /// **C6: a sampled edge listed first masks a decidable "inside an
    /// edge".** The north pole lies inside the great-circle arc `A`
    /// (closed-form, decidable); the loop's other edge `B` is an ellipse
    /// that passes 1e-4 from the pole, within its own sample step, and is
    /// listed first. The reader answers `Unread::Carrier` for B, which tier
    /// 3 exempts, instead of `InsideEdge` on A, which it would refuse.
    #[test]
    fn sampled_edge_masks_a_decidable_inside_edge() {
        let a = 0.05_f64;
        // A: great circle in the xz-plane, angle t: (cos t, 0, sin t).
        let arc = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, -1.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // B: an ellipse from P2 = (-sin a, 0, cos a) to P1 = (sin a, 0, cos a)
        // whose mid point is (0, 1e-4, 1).
        let c = Point3::new(0.0, 0.0, a.cos());
        let mid = Vec3::new(0.0, 1e-4, 1.0 - a.cos());
        let semi_v = mid.norm();
        let v_dir = mid / semi_v;
        let u_ref = Vec3::new(-1.0, 0.0, 0.0);
        let axis = u_ref.cross(v_dir);
        let ellipse = Curve3::Ellipse {
            center: c,
            axis,
            semi_u: a.sin(),
            semi_v,
            u_ref,
        };
        // θ = 0 at P2, θ = π/2 at mid, θ = π at P1.
        let p_mid = ellipse.eval(FRAC_PI_2);
        assert!(
            (p_mid - Point3::new(0.0, 1e-4, 1.0)).norm() < 1e-12,
            "{p_mid:?}"
        );
        let lp = vec![
            LoopEdge::hand_built(ellipse, 0.0, core::f64::consts::PI, true, 0, 1),
            LoopEdge::hand_built(arc, FRAC_PI_2 - a, FRAC_PI_2 + a, true, 1, 0),
        ];
        let band = Band::new(1e-9, 1e-8).unwrap();
        let got = run(&[lp], true, band);
        let north = got
            .iter()
            .find(|l| l.singularity == Singularity::North)
            .unwrap();
        assert!(
            matches!(north.lies, Ok(Lies::InsideEdge { edge: 1, .. })),
            "the pole is inside the closed-form arc; got {:?}",
            north.lies
        );
    }

    /// **C2 class: a small or thin cap near a pole but clear of it reads the
    /// pole outside; one just past it reads it inside**, swept over tilt,
    /// angular radius and band, both traversals.
    #[test]
    fn caps_near_a_pole_read_by_their_region() {
        let mut bad = Vec::new();
        for (zero, esc) in [(1e-12, 1e-11), (1e-9, 1e-8), (1e-6, 1e-5)] {
            let band = Band::new(zero, esc).unwrap();
            for r in [1e-5_f64, 1e-3, 0.1, 1.0, 1.5] {
                for gap_scale in [-100.0, -10.0, 10.0, 100.0, 1e4] {
                    // angular distance from the pole to the rim: θ − r.
                    let gap = gap_scale * esc;
                    let theta = r + gap;
                    if !(0.0..core::f64::consts::PI).contains(&theta)
                        || theta - r > core::f64::consts::PI - r - r
                    {
                        continue;
                    }
                    let n = Vec3::new(theta.sin(), 0.0, theta.cos());
                    let rim = section(n, r.cos());
                    for forward in [true, false] {
                        let lp = vec![LoopEdge::hand_built(rim.clone(), 0.0, TAU, forward, 0, 0)];
                        let got = run(&[lp], forward, band);
                        let north = &got[0];
                        // The cap is on the left of the rim run forward,
                        // and the face's sense follows it: the cap itself.
                        let want_in = gap < 0.0;
                        let ok = match &north.lies {
                            Ok(Lies::Interior) => want_in,
                            Ok(Lies::Outside) => !want_in,
                            Err(Unread::Boundary { .. }) => true,
                            _ => false,
                        };
                        if !ok {
                            bad.push(format!(
                                "band {esc:e} r {r} gap {gap:e} forward {forward}: {:?}",
                                north.lies
                            ));
                        }
                    }
                }
            }
        }
        assert!(bad.is_empty(), "{bad:#?}");
    }

    /// **C2 class: a thin lens far from the poles reads both outside**,
    /// at every band, its width below the coarse band.
    #[test]
    fn thin_lens_far_from_poles() {
        let mut bad = Vec::new();
        for (zero, esc) in [(1e-12, 1e-11), (1e-9, 1e-8), (1e-6, 1e-5)] {
            let band = Band::new(zero, esc).unwrap();
            for width in [1e-9_f64, 1e-7, 1e-5, 1e-3] {
                for tilt in [0.3_f64, 1.0, FRAC_PI_2, 2.5] {
                    // Two circles through the same two points, both about
                    // axes in the plane perpendicular to the chord.
                    let n1 = Vec3::new(tilt.sin(), 0.0, tilt.cos());
                    let side = Vec3::new(0.0, 1.0, 0.0);
                    let d = 0.6_f64;
                    let c1 = section(n1, d);
                    // Rotate n1 about the chord direction (perp to n1 and side?)
                    // Simpler: a second section plane through the same chord:
                    // tilt the plane normal by `width` about the chord axis.
                    let chord = n1.cross(side);
                    let chord = chord / chord.norm();
                    let rot = |v: Vec3<f64>, ang: f64| {
                        v * ang.cos()
                            + chord.cross(v) * ang.sin()
                            + chord * chord.dot(v) * (1.0 - ang.cos())
                    };
                    let n2 = rot(n1, width);
                    // The chord lies on both planes: n·p = d for p on the chord
                    // line {d n1 + s chord}; for plane 2 we need d2 = d n1·n2.
                    let d2 = d * n1.dot(n2);
                    let c2 = section(n2, d2);
                    // Endpoints: on the chord line, at ±s, s = sqrt(1 - d²).
                    let s = (1.0 - d * d).sqrt();
                    let ends = [n1 * d + chord * s, n1 * d - chord * s];
                    let ang = |c: &Curve3<f64>, p: Vec3<f64>| {
                        let Curve3::Circle {
                            center,
                            axis,
                            u_ref,
                            ..
                        } = c
                        else {
                            unreachable!()
                        };
                        let w = Point3::new(p.x, p.y, p.z) - *center;
                        w.dot(axis.cross(*u_ref))
                            .atan2(w.dot(*u_ref))
                            .rem_euclid(TAU)
                    };
                    // Edge 1: c1 from ends[0] to ends[1] the short way? Use the
                    // arcs that are close to each other: same side.
                    let (a0, a1) = (ang(&c1, ends[0]), ang(&c1, ends[1]));
                    let (b0, b1) = (ang(&c2, ends[0]), ang(&c2, ends[1]));
                    let span = |x0: f64, x1: f64| {
                        let mut x1 = x1;
                        while x1 <= x0 {
                            x1 += TAU;
                        }
                        (x0, x1)
                    };
                    let (s1a, s1b) = span(a0, a1);
                    let (s2a, s2b) = span(b0, b1);
                    for sense in [true, false] {
                        let lp = vec![
                            LoopEdge::hand_built(c1.clone(), s1a, s1b, true, 0, 1),
                            LoopEdge::hand_built(c2.clone(), s2a, s2b, false, 1, 0),
                        ];
                        let got = run(&[lp], sense, band);
                        let states: Vec<_> = got.iter().map(|l| format!("{:?}", l.lies)).collect();
                        // One sense is the lens (no pole), the other its
                        // complement (both poles inside).
                        let outs = got
                            .iter()
                            .filter(|l| matches!(l.lies, Ok(Lies::Outside)))
                            .count();
                        let ins = got
                            .iter()
                            .filter(|l| matches!(l.lies, Ok(Lies::Interior)))
                            .count();
                        if !(outs == 2 || ins == 2) {
                            bad.push(format!(
                                "band {esc:e} width {width:e} tilt {tilt} sense {sense}: {states:?}"
                            ));
                        }
                    }
                }
            }
        }
        assert!(bad.is_empty(), "{bad:#?}");
    }

    /// A tiny deterministic generator.
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }
    }

    fn unit(v: Vec3<f64>) -> Vec3<f64> {
        v / v.norm()
    }

    /// The great-circle arc from `a` to `b` (unit vectors), as a carrier and
    /// its interval.
    fn great_arc(a: Vec3<f64>, b: Vec3<f64>) -> (Curve3<f64>, f64, f64) {
        let axis = unit(a.cross(b));
        let ang = a.cross(b).norm().atan2(a.dot(b));
        (
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, 0.0),
                axis,
                radius: 1.0,
                u_ref: a,
            },
            0.0,
            ang,
        )
    }

    /// **C1/C2 fuzz: convex spherical polygons, against the exact
    /// inside-every-edge-plane test**, both traversals and both senses, at
    /// two bands; polygons of every size, placed anywhere, many near a pole.
    #[test]
    fn convex_spherical_polygons_against_the_edge_planes() {
        let mut rng = Lcg(0x6054);
        let mut bad = Vec::new();
        let mut counts = [0usize; 4];
        for case in 0..5000 {
            let (zero, esc) = if case % 2 == 0 {
                (1e-9, 1e-8)
            } else {
                (1e-6, 1e-5)
            };
            let band = Band::new(zero, esc).unwrap();
            // The polygon's centre: half the time within 0.05 rad of a pole.
            let (theta, phi) = if case % 4 < 2 {
                (
                    rng.next() * 0.05 * if case % 8 < 4 { 1.0 } else { 1e-3 },
                    rng.next() * TAU,
                )
            } else {
                ((rng.next() * 2.0 - 1.0).acos(), rng.next() * TAU)
            };
            let c = Vec3::new(
                theta.sin() * phi.cos(),
                theta.sin() * phi.sin(),
                theta.cos(),
            );
            let rho = 10f64.powf(-5.0 + 5.1 * rng.next()).min(1.4);
            let k = 3 + (rng.next() * 5.0) as usize;
            let e1 = unit(
                if c.z.abs() < 0.9 {
                    Vec3::new(0.0, 0.0, 1.0)
                } else {
                    Vec3::new(1.0, 0.0, 0.0)
                }
                .cross(c),
            );
            let e2 = c.cross(e1);
            let mut angs: Vec<f64> = (0..k).map(|_| rng.next() * TAU).collect();
            angs.sort_by(f64::total_cmp);
            let verts: Vec<Vec3<f64>> = angs
                .iter()
                .map(|&a| unit(c * rho.cos() + (e1 * a.cos() + e2 * a.sin()) * rho.sin()))
                .collect();
            // Skip degenerate (two vertices nearly coincident).
            if verts
                .iter()
                .enumerate()
                .any(|(i, v)| (*v - verts[(i + 1) % k]).norm() < 1e-7)
            {
                continue;
            }
            // Skip polygons that are not convex (a vertex angle ≥ π; all
            // vertices on a small circle are convex unless an edge spans ≥ π
            // about c).
            if angs
                .windows(2)
                .any(|w| w[1] - w[0] >= core::f64::consts::PI)
                || TAU - angs[k - 1] + angs[0] >= core::f64::consts::PI
            {
                continue;
            }
            let ccw: Vec<LoopEdge<f64>> = (0..k)
                .map(|i| {
                    let (cv, t0, t1) = great_arc(verts[i], verts[(i + 1) % k]);
                    LoopEdge::hand_built(cv, t0, t1, true, i as u32, ((i + 1) % k) as u32)
                })
                .collect();
            let cw: Vec<LoopEdge<f64>> = ccw
                .iter()
                .rev()
                .map(|e| LoopEdge::hand_built(e.carrier.clone(), e.t0, e.t1, false, 0, 0))
                .collect();
            // Fix tags for cw (unused by the reader but keep them distinct).
            for (lp, ccw_dir) in [(ccw, true), (cw, false)] {
                for sense in [true, false] {
                    let got = run(core::slice::from_ref(&lp), sense, band);
                    for l in &got {
                        let p = Vec3::new(l.point.x, l.point.y, l.point.z);
                        // Signed distances to each edge plane (CCW order).
                        let margins: Vec<f64> = (0..k)
                            .map(|i| p.dot(unit(verts[i].cross(verts[(i + 1) % k]))))
                            .collect();
                        let min = margins.iter().copied().fold(f64::INFINITY, f64::min);
                        let near_vertex = verts.iter().any(|v| (*v - p).norm() < 100.0 * esc);
                        let in_poly = min > 0.0;
                        // The face is the polygon where the traversal's left
                        // (ccw_dir) agrees with the sense.
                        let face_is_poly = ccw_dir == sense;
                        let want_in = in_poly == face_is_poly;
                        let clear = min.abs() > 100.0 * esc && !near_vertex;
                        match &l.lies {
                            Ok(Lies::Interior) => counts[0] += 1,
                            Ok(Lies::Outside) => counts[1] += 1,
                            Err(_) => counts[2] += 1,
                            _ => counts[3] += 1,
                        }
                        if !clear {
                            continue;
                        }
                        let ok = match &l.lies {
                            Ok(Lies::Interior) => want_in,
                            Ok(Lies::Outside) => !want_in,
                            _ => false,
                        };
                        if !ok {
                            bad.push(format!(
                                "case {case} band {esc:e} rho {rho:e} k {k} theta {theta:e} ccw {ccw_dir} sense {sense} {:?}: want_in {want_in}, got {:?}",
                                l.singularity, l.lies
                            ));
                        }
                    }
                }
            }
        }
        eprintln!("counts interior/outside/unread/other {counts:?}");
        assert!(
            bad.is_empty(),
            "{} bad: {:#?}",
            bad.len(),
            &bad[..bad.len().min(30)]
        );
    }

    /// **C1: a meridian spur short of the pole still leaves it inside the
    /// cap**; a spur reaching the pole makes it a vertex.
    #[test]
    fn a_meridian_spur_short_of_the_pole() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let z = 0.5_f64;
        let rim = section(Vec3::new(0.0, 0.0, 1.0), z);
        // The rim's angle-0 point, in its frame.
        let Curve3::Circle {
            center,
            axis,
            u_ref,
            ..
        } = rim.clone()
        else {
            unreachable!()
        };
        let p0 = center + u_ref * (1.0 - z * z).sqrt();
        let az = Vec3::new(p0.x, p0.y, 0.0);
        let az = az / az.norm();
        // Meridian through p0: angle t measured from the equator toward north.
        let meridian = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: az.cross(Vec3::new(0.0, 0.0, 1.0)),
            radius: 1.0,
            u_ref: az,
        };
        let lat0 = z.asin();
        assert!(
            (meridian.eval(lat0) - p0).norm() < 1e-12,
            "{:?} {:?}",
            meridian.eval(lat0),
            p0
        );
        let _ = axis;
        for (tip, want) in [
            (1.4_f64, Lies::Interior),
            (FRAC_PI_2, Lies::AtVertex { r#loop: 0, edge: 2 }),
        ] {
            let lp = vec![
                LoopEdge::hand_built(rim.clone(), 0.0, TAU, true, 0, 0),
                LoopEdge::hand_built(meridian.clone(), lat0, tip, true, 0, 1),
                LoopEdge::hand_built(meridian.clone(), lat0, tip, false, 1, 0),
            ];
            let got = run(&[lp], true, band);
            assert!(
                got[0].lies.as_ref().is_ok_and(|l| *l == want),
                "tip {tip}: {:?}",
                got[0].lies
            );
            assert!(
                matches!(got[1].lies, Ok(Lies::Outside)),
                "south {:?}",
                got[1].lies
            );
        }
    }
}

/// Star polygons of small-circle arcs on the unit sphere, slivers to an
/// aspect of 1e-6, centred near the north pole and anywhere, both
/// traversals and senses, at three bands: the closed form, and the same
/// arcs as equal-semi ellipses through the walk. Ground truth by a
/// dense winding about each pole. Any wrong side fails; an undecided one
/// is tallied, and the walk must decide most where its bound clears.
mod small_arc_fuzz {
    use core::f64::consts::TAU;
    use geom::{Curve3, Surface};
    use geom_brep::chart_singularity::{Lies, Located, Unread};
    use geom_brep::props::LoopEdge;
    use geom_core::{Band, Point3, Vec3};

    fn run(
        s: &Surface<f64>,
        loops: &[Vec<LoopEdge<f64>>],
        sense: bool,
        band: Band,
    ) -> Vec<Located<f64>> {
        super::locate_hand_built(s, loops, sense, band)
    }

    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }
    }
    fn unit(v: Vec3<f64>) -> Vec3<f64> {
        v / v.norm()
    }
    const O: Point3<f64> = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    fn sphere() -> Surface<f64> {
        Surface::Sphere {
            center: O,
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// Small-circle arc from a to b (unit), its plane tilted by beta about the chord.
    fn small_arc(
        a: Vec3<f64>,
        b: Vec3<f64>,
        beta: f64,
        sampled: bool,
        s: u32,
        e: u32,
    ) -> LoopEdge<f64> {
        let g = unit(a.cross(b));
        let m = unit(b - a);
        let n = unit(g * beta.cos() + m.cross(g) * beta.sin());
        let d = n.dot(a);
        let c = O + n * d;
        let r = (1.0 - d * d).sqrt();
        let u = unit((O + a) - c);
        let wv = (O + b) - c;
        let mut t1 = wv.dot(n.cross(u)).atan2(wv.dot(u));
        if t1 <= 0.0 {
            t1 += TAU;
        }
        let carrier = if sampled {
            Curve3::Ellipse {
                center: c,
                axis: n,
                semi_u: r,
                semi_v: r,
                u_ref: u,
            }
        } else {
            Curve3::Circle {
                center: c,
                axis: n,
                radius: r,
                u_ref: u,
            }
        };
        LoopEdge::hand_built(carrier, 0.0, t1, true, s, e)
    }

    fn trav(e: &LoopEdge<f64>, k: usize, n: usize) -> f64 {
        let (a, b) = if e.forward {
            (e.t0, e.t1)
        } else {
            (e.t1, e.t0)
        };
        a + (b - a) * (k as f64) / (n as f64)
    }

    /// Winding of a loop about unit p, seen from outside at p, by adaptive sampling.
    fn winding(edges: &[LoopEdge<f64>], p: Vec3<f64>) -> i64 {
        let e1 = unit(
            if p.z.abs() < 0.9 {
                Vec3::new(0.0, 0.0, 1.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            }
            .cross(p),
        );
        let e2 = p.cross(e1);
        let ang = |q: Point3<f64>| {
            let w = q - O;
            w.dot(e2).atan2(w.dot(e1))
        };
        let mut total = 0.0;
        for e in edges {
            let mut stack = vec![];
            let n = 64;
            for k in (0..n).rev() {
                stack.push((trav(e, k, n), trav(e, k + 1, n), 0u32));
            }
            while let Some((a, b, depth)) = stack.pop() {
                let (pa, pb) = (e.carrier.eval(a), e.carrier.eval(b));
                let mut d = ang(pb) - ang(pa);
                while d > core::f64::consts::PI {
                    d -= TAU;
                }
                while d < -core::f64::consts::PI {
                    d += TAU;
                }
                if d.abs() > 0.2 && depth < 60 {
                    let m = 0.5 * (a + b);
                    stack.push((m, b, depth + 1));
                    stack.push((a, m, depth + 1));
                } else {
                    total += d;
                }
            }
        }
        // junction gaps are rounding-sized; ignore
        (total / TAU).round() as i64
    }

    fn reversed(edges: &[LoopEdge<f64>]) -> Vec<LoopEdge<f64>> {
        let n = edges.len() as u32;
        edges
            .iter()
            .rev()
            .enumerate()
            .map(|(i, e)| {
                let mut f = e.clone();
                f.forward = !e.forward;
                f.start = i as u32;
                f.end = (i as u32 + 1) % n;
                f
            })
            .collect()
    }

    /// Dense gnomonic polyline about c; true if it self-intersects.
    fn self_intersects(edges: &[LoopEdge<f64>], c: Vec3<f64>) -> bool {
        let e1 = unit(
            if c.z.abs() < 0.9 {
                Vec3::new(0.0, 0.0, 1.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            }
            .cross(c),
        );
        let e2 = c.cross(e1);
        let mut pts = vec![];
        for e in edges {
            for k in 0..200 {
                let w = e.carrier.eval(trav(e, k, 200)) - O;
                let h = w.dot(c);
                if h <= 0.05 {
                    return true;
                }
                pts.push((w.dot(e1) / h, w.dot(e2) / h));
            }
        }
        let n = pts.len();
        let orient = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| {
            (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
        };
        for i in 0..n {
            for j in i + 2..n {
                if i == 0 && j == n - 1 {
                    continue;
                }
                let (a, b, c2, d) = (pts[i], pts[(i + 1) % n], pts[j], pts[(j + 1) % n]);
                let o1 = orient(a, b, c2);
                let o2 = orient(a, b, d);
                let o3 = orient(c2, d, a);
                let o4 = orient(c2, d, b);
                if o1 * o2 < 0.0 && o3 * o4 < 0.0 {
                    return true;
                }
            }
        }
        false
    }

    fn bands() -> [Band; 3] {
        [
            Band::new(1e-12, 1e-11).unwrap(),
            Band::new(1e-9, 1e-8).unwrap(),
            Band::new(1e-6, 1e-5).unwrap(),
        ]
    }

    #[derive(Default, Debug)]
    struct Tally {
        right: usize,
        wrong: usize,
        straddle: usize,
        banded: usize,
        carrier: usize,
        lp: usize,
        boundary: usize,
    }

    fn judge(
        t: &mut Tally,
        bad: &mut Vec<String>,
        got: &[Located<f64>],
        left: &dyn Fn(Vec3<f64>) -> bool,
        sense: bool,
        label: &str,
    ) {
        for l in got {
            let p = l.point - O;
            match &l.lies {
                Ok(Lies::Interior) | Ok(Lies::Outside) => {
                    let want = left(p) == sense;
                    let interior = matches!(l.lies, Ok(Lies::Interior));
                    if interior == want {
                        t.right += 1
                    } else {
                        t.wrong += 1;
                        bad.push(format!(
                            "{label} {:?}: {:?} want interior={want}",
                            l.singularity, l.lies
                        ));
                    }
                }
                Ok(_) | Err(Unread::Boundary { .. }) => t.boundary += 1,
                Err(Unread::Side { why, .. }) => match why {
                    geom_brep::chart_singularity::SideUnread::Straddle => t.straddle += 1,
                    _ => t.banded += 1,
                },
                Err(Unread::Carrier { .. }) => t.carrier += 1,
                Err(Unread::Loop { .. }) => t.lp += 1,
            }
        }
    }

    /// Star polygons of small-circle arcs, slivers included, near and far from a pole.
    fn fuzz(sampled: bool, cases: usize, seed: u64) -> Tally {
        let mut rng = Lcg(seed);
        let mut bad = vec![];
        let mut t = Tally::default();
        let mut skipped = 0;
        for case in 0..cases {
            let band = bands()[case % 3];
            let rho = 10f64.powf(-4.0 + 4.0 * rng.next()).min(1.0);
            let aspect = 10f64.powf(-6.0 * rng.next());
            // centre at distance s*rho*aspect.. from the north pole
            let near = rng.next();
            let dist = if near < 0.7 {
                rho * (aspect * 10f64.powf(2.0 * rng.next() - 1.0)).min(2.0) * rng.next() * 2.0
            } else {
                (rng.next() * 2.0 - 1.0).acos()
            };
            let phi = rng.next() * TAU;
            let c = Vec3::new(dist.sin() * phi.cos(), dist.sin() * phi.sin(), dist.cos());
            let e1 = unit(
                if c.z.abs() < 0.9 {
                    Vec3::new(0.0, 0.0, 1.0)
                } else {
                    Vec3::new(1.0, 0.0, 0.0)
                }
                .cross(c),
            );
            let e2 = c.cross(e1);
            let rot = rng.next() * TAU;
            let (f1, f2) = (
                e1 * rot.cos() + e2 * rot.sin(),
                e2 * rot.cos() - e1 * rot.sin(),
            );
            let k = 2 + (rng.next() * 6.0) as usize;
            let mut angs: Vec<f64> = (0..k).map(|_| rng.next() * TAU).collect();
            angs.sort_by(f64::total_cmp);
            let verts: Vec<Vec3<f64>> = angs
                .iter()
                .map(|&a| unit(c + (f1 * (a.cos() * rho) + f2 * (a.sin() * rho * aspect))))
                .collect();
            if verts
                .iter()
                .enumerate()
                .any(|(i, v)| (*v - verts[(i + 1) % k]).norm() < 1e-9)
            {
                skipped += 1;
                continue;
            }
            let bmax = if k == 2 { 0.3 } else { 0.3 * rng.next() };
            let edges: Vec<LoopEdge<f64>> = (0..k)
                .map(|i| {
                    let (a, b) = (verts[i], verts[(i + 1) % k]);
                    let mut beta = bmax * (rng.next() * 2.0 - 1.0);
                    if k == 2 {
                        beta = if i == 0 {
                            bmax * (0.2 + rng.next())
                        } else {
                            -bmax * (0.2 + rng.next())
                        };
                    }
                    small_arc(a, b, beta, sampled, i as u32, ((i + 1) % k) as u32)
                })
                .collect();
            if self_intersects(&edges, c) {
                skipped += 1;
                continue;
            }
            // orientation: must wind +1 about c (ccw about c)
            let wc = winding(&edges, c);
            let edges = match wc {
                1 => edges,
                -1 => reversed(&edges),
                _ => {
                    skipped += 1;
                    continue;
                }
            };
            for (lp, ccw) in [(edges.clone(), true), (reversed(&edges), false)] {
                let left = |p: Vec3<f64>| {
                    let w = winding(&edges, p);
                    if ccw { w == 1 } else { w != 1 }
                };
                for sense in [true, false] {
                    let got = run(&sphere(), core::slice::from_ref(&lp), sense, band);
                    judge(
                        &mut t,
                        &mut bad,
                        &got,
                        &left,
                        sense,
                        &format!(
                            "case {case} band {band:?} rho {rho:e} aspect {aspect:e} dist {dist:e} k {k} ccw {ccw} sense {sense}"
                        ),
                    );
                }
            }
        }
        eprintln!("sampled={sampled} {t:?} skipped {skipped}");
        assert!(
            bad.is_empty(),
            "{} wrong:\n{}",
            bad.len(),
            bad.iter().take(30).cloned().collect::<Vec<_>>().join("\n")
        );
        t
    }

    #[test]
    fn small_arc_stars_read_their_poles_in_closed_form() {
        let t = fuzz(false, 1500, 0xD1);
        // A lens whose tips are exact cusps leaves the closed form for
        // the walk, which may leave its side undecided: a few at most.
        let undecided = t.straddle + t.banded + t.lp + t.carrier;
        assert!(
            undecided * 50 < t.right,
            "the closed form decides nearly every side: {t:?}"
        );
    }

    #[test]
    fn small_arc_stars_on_sampled_carriers_read_their_poles_by_the_walk() {
        let t = fuzz(true, 300, 0xD2);
        // The walk decides where its bound clears: most readings, not
        // every sliver.
        assert!(
            t.right * 2 > t.right + t.straddle,
            "the walk decides most sides: {t:?}"
        );
    }

    /// The least distance from `p` to the loop of circle arcs: each arc's
    /// closed-form foot where it falls inside the arc, else its ends.
    fn loop_distance(edges: &[LoopEdge<f64>], p: Vec3<f64>) -> f64 {
        let x = O + p;
        edges
            .iter()
            .map(|e| {
                let ends = (e.carrier.eval(e.t0) - x)
                    .norm()
                    .min((e.carrier.eval(e.t1) - x).norm());
                match geom_brep::chart_singularity::foot(&e.carrier, x, e.t0) {
                    Some(t) if t <= e.t1 => ends.min((e.carrier.eval(t) - x).norm()),
                    _ => ends,
                }
            })
            .fold(f64::INFINITY, f64::min)
    }

    /// **A vertex just beside a pole is read in closed form, decided**:
    /// at the band `(1e-12, 1e-11)`, a polygon of small-circle arcs with
    /// one vertex `2e-11` to `1e-9` from the north pole and the others half
    /// a radian off, its corner there opening from a hair to past a half
    /// turn. The pole is past the band from the vertex, so every reading
    /// is decided and right: the stereographic image from the pole of a
    /// point that close divides by `|w − pole|²/2`, about `1e-22`, which
    /// `1 − w·pole` cannot hold. Where an arc passes the pole within twice
    /// the band's escalation, the point may read on the boundary or in
    /// band; past it, every reading is decided.
    #[test]
    fn a_vertex_just_beside_a_pole_reads_it_decided_at_a_fine_band() {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let mut rng = Lcg(0xB2);
        let mut bad = Vec::new();
        let mut read = 0;
        for delta in [2e-11_f64, 5e-11, 1e-10, 1e-9] {
            for open in [1e-3_f64, 0.5, 2.0, 3.1, 3.2, 4.5] {
                for dir in [0.0_f64, 1.0, 2.5, 4.0] {
                    for beta in [0.0_f64, 0.2, -0.2] {
                        let v0 = unit(
                            Vec3::new(0.0, 0.0, 1.0) + Vec3::new(dir.cos(), dir.sin(), 0.0) * delta,
                        );
                        let e1 = unit(Vec3::new(1.0, 0.0, 0.0) - v0 * v0.x);
                        let e2 = v0.cross(e1);
                        let far = |ang: f64| {
                            unit(
                                v0 * 0.5f64.cos()
                                    + (e1 * ang.cos() + e2 * ang.sin()) * 0.5f64.sin(),
                            )
                        };
                        let a = rng.next() * TAU;
                        let verts = if open < 3.14 {
                            vec![v0, far(a), far(a + open)]
                        } else {
                            vec![v0, far(a), far(a + open * 0.5), far(a + open)]
                        };
                        let k = verts.len();
                        let edges: Vec<LoopEdge<f64>> = (0..k)
                            .map(|i| {
                                small_arc(
                                    verts[i],
                                    verts[(i + 1) % k],
                                    beta,
                                    false,
                                    i as u32,
                                    ((i + 1) % k) as u32,
                                )
                            })
                            .collect();
                        let c = unit(verts.iter().fold(Vec3::new(0.0, 0.0, 0.0), |s, v| s + *v));
                        if self_intersects(&edges, c) {
                            continue;
                        }
                        let edges = match winding(&edges, c) {
                            1 => edges,
                            -1 => reversed(&edges),
                            _ => continue,
                        };
                        for (lp, ccw) in [(edges.clone(), true), (reversed(&edges), false)] {
                            for sense in [true, false] {
                                for l in run(&sphere(), core::slice::from_ref(&lp), sense, band) {
                                    let p = l.point - O;
                                    let left = (winding(&edges, p) == 1) == ccw;
                                    let want = if left == sense {
                                        Lies::Interior
                                    } else {
                                        Lies::Outside
                                    };
                                    let clear = loop_distance(&lp, p) > 2.0 * band.escalate();
                                    if !clear
                                        && matches!(
                                            l.lies,
                                            Err(Unread::Boundary { .. })
                                                | Ok(
                                                    Lies::InsideEdge { .. } | Lies::AtVertex { .. }
                                                )
                                        )
                                    {
                                        continue;
                                    }
                                    read += 1;
                                    if !matches!(l.lies, Ok(got) if got == want) {
                                        bad.push(format!(
                                            "delta {delta:e} open {open} dir {dir} beta {beta} ccw {ccw} sense {sense} {:?}: {:?} want {want:?}",
                                            l.singularity, l.lies
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(read > 500, "the sweep reads: {read}");
        assert!(
            bad.is_empty(),
            "{} not decided right:\n{}",
            bad.len(),
            bad[..bad.len().min(20)].join("\n")
        );
    }
}
