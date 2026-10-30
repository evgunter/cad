//! Review probes for PR 6054 (`geom_brep::chart_singularity::locate`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, TAU};

use geom::{Curve3, Surface};
use geom_brep::chart_singularity::{Lies, Located, Singularity, Unread, locate};
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
    let views: Vec<&[LoopEdge<f64>]> = loops.iter().map(Vec::as_slice).collect();
    locate(&unit_sphere(), &views, sense, band)
}

/// **C6: a sampled edge listed first masks a decidable "inside an
/// edge".** The north pole lies inside the great-circle arc `A`
/// (closed-form, decidable); the loop's other edge `B` is an ellipse
/// that passes 1e-4 from the pole, within its own sample step, and is
/// listed first. The reader answers `Unread::Carrier` for B, which tier
/// 3 exempts, instead of `InsideEdge` on A, which it would refuse.
#[test]
fn probe_sampled_edge_masks_a_decidable_inside_edge() {
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
fn probe_caps_near_a_pole_read_by_their_region() {
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
fn probe_thin_lens_far_from_poles() {
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
fn probe_convex_spherical_polygons_against_the_edge_planes() {
    let mut rng = Lcg(0x6054);
    let mut bad = Vec::new();
    let mut counts = [0usize; 4];
    for case in 0..20000 {
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
                let got = run(&[lp.clone()], sense, band);
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
        bad.len() == 0,
        "{} bad: {:#?}",
        bad.len(),
        &bad[..bad.len().min(30)]
    );
}

/// **C1: a meridian spur short of the pole still leaves it inside the
/// cap**; a spur reaching the pole makes it a vertex.
#[test]
fn probe_a_meridian_spur_short_of_the_pole() {
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
