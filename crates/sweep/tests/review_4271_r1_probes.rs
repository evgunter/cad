//! Review probes (band-dual-4271-r1) for PR 4271's straight-edge cap
//! meter: triangular holes through the D-rod near its upper crease,
//! judged against an exact membership test for the removed sliver `S`.
//! A hole whose boundary enters `S` must refuse; a carve must be tier-3
//! and tier-3′ valid, at the closed-form ΔV, with sampled point
//! membership matching the analytic filleted-D-minus-hole.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, Tol};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, extruded, rod_chord_at, rod_creases, rod_section_cut,
};
use topo::boolean::{SolidContainment, point_in_solid};
use topo::{Body, ContactRecords, mass_properties, validate_geometric, validate_pseudomanifold};

fn tol() -> Tol {
    Tol::witness()
}
fn volume(b: &Body<f64>) -> f64 {
    mass_properties(b, tol()).expect("props").volume
}
fn poly(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}
fn d_loop() -> ProfileLoop<f64> {
    let c = rod_chord_at(ROD_FLAT);
    bulge_loop(vec![
        (Point2::new(ROD_FLAT, c.half), c.wall_bulge),
        (Point2::new(ROD_FLAT, -c.half), 0.0),
    ])
}

const R: f64 = ROD_FILLET;
fn cen(sy: f64) -> (f64, f64) {
    let cx = ROD_FLAT - R;
    (cx, sy * ((ROD_R - R).powi(2) - cx * cx).sqrt())
}
fn in_d(p: (f64, f64)) -> bool {
    p.0 <= ROD_FLAT && p.0.hypot(p.1) <= ROD_R
}
/// In the sliver at the upper (`sy = 1`) or lower corner.
fn in_s(p: (f64, f64), sy: f64) -> bool {
    let c = cen(sy);
    let (dx, dy) = (p.0 - c.0, sy * (p.1 - c.1));
    in_d(p) && dx.hypot(dy) > R && dx >= 0.0 && dy >= 0.0 && dy * 0.5 <= dx * 0.866_025_403_784_438_6 + 1e-15
        // the wedge between (1,0) and (1/2, √3/2)
        && dy.atan2(dx) <= core::f64::consts::FRAC_PI_3
}
fn s_boundary(sy: f64) -> Vec<(f64, f64)> {
    let c = cen(sy);
    let mut v = Vec::new();
    let n = 2000;
    let vy = (ROD_R.powi(2) - ROD_FLAT.powi(2)).sqrt();
    let f2 = (c.0 * ROD_R / (ROD_R - R), c.1 * ROD_R / (ROD_R - R));
    let (a0, a1) = (f2.1.atan2(f2.0), (sy * vy).atan2(ROD_FLAT));
    for k in 0..=n {
        let t = k as f64 / n as f64;
        let th = t * core::f64::consts::FRAC_PI_3;
        v.push((c.0 + R * th.cos(), c.1 + sy * R * th.sin()));
        v.push((ROD_FLAT, c.1 + (sy * vy - c.1) * t));
        let a = a0 + (a1 - a0) * t;
        v.push((ROD_R * a.cos(), ROD_R * a.sin()));
    }
    v
}

/// (depth into S of the boundary — positive when it enters, else minus the
/// distance from S), over a dense sampling of the hole's boundary.
fn penetration(tri: &[(f64, f64)], sb: &[(f64, f64)]) -> f64 {
    let mut worst = f64::NEG_INFINITY;
    for i in 0..tri.len() {
        let (a, b) = (tri[i], tri[(i + 1) % tri.len()]);
        for k in 0..=600 {
            let t = k as f64 / 600.0;
            let p = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
            let d = sb
                .iter()
                .map(|q| (q.0 - p.0).hypot(q.1 - p.1))
                .fold(f64::INFINITY, f64::min);
            let inside = in_s(p, 1.0) || in_s(p, -1.0);
            worst = worst.max(if inside { d } else { -d });
        }
    }
    worst
}

fn in_tri(tri: &[(f64, f64)], p: (f64, f64)) -> bool {
    let mut sgn = 0.0f64;
    for i in 0..tri.len() {
        let (a, b) = (tri[i], tri[(i + 1) % tri.len()]);
        let c = (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        if sgn == 0.0 {
            sgn = c.signum();
        } else if c * sgn < 0.0 {
            return false;
        }
    }
    true
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

fn cases() -> Vec<Vec<(f64, f64)>> {
    let mut rng = Lcg(0x4271_0001);
    let mut out = Vec::new();
    let c = cen(1.0);
    // random triangles concentrated around the corner
    while out.len() < 160 {
        let t: Vec<(f64, f64)> = (0..3)
            .map(|_| (0.12 + 0.18 * rng.next(), 0.26 + 0.2 * rng.next()))
            .collect();
        out.push(t);
    }
    // chord edges at radius R − δ from c, half-length h, at angle θ in
    // the wedge; third vertex towards c (inside the section).
    for &delta in &[2e-3, 1e-4, 1e-6] {
        for k in 1..6 {
            let th = core::f64::consts::FRAC_PI_3 * k as f64 / 6.0;
            let (nx, ny) = (th.cos(), th.sin());
            let rho = R - delta;
            let half = (R * R - rho * rho).sqrt();
            for &f in &[0.5, 0.99, 1.01, 1.5, 3.0] {
                let h = half * f;
                let m = (c.0 + nx * rho, c.1 + ny * rho);
                out.push(vec![
                    (m.0 - ny * h, m.1 + nx * h),
                    (m.0 + ny * h, m.1 - nx * h),
                    (c.0 + nx * 0.02, c.1 + ny * 0.02),
                ]);
            }
        }
    }
    // edges along the section's axes (k² = 1 against ±u, ±w), near the feet
    for &(x0, y0, x1, y1) in &[
        (0.25, 0.30, 0.2995, 0.3464), // corner just short of the flat foot
        (0.25, 0.30, 0.2995, 0.3470), // just past the foot level, near the flat
        (0.20, 0.40, 0.2495, 0.432),  // near the wall foot
        (0.20, 0.40, 0.2505, 0.4325),
        (0.15, 0.30, 0.299, 0.3465),
    ] {
        out.push(vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]);
    }
    out
}

#[test]
fn holes_near_the_d_rods_corner_refuse_iff_they_enter_the_sliver() {
    let sb: Vec<(f64, f64)> = s_boundary(1.0)
        .into_iter()
        .chain(s_boundary(-1.0))
        .collect();
    let band = Band::linear(tol()).expect("band");
    let want = -2.0 * rod_section_cut(ROD_R, ROD_FLAT, ROD_FILLET) * ROD_L;
    let (mut built, mut refused_clear, mut refused_cross, mut grey, mut skipped) = (0, 0, 0, 0, 0);
    let mut bad = Vec::new();
    for (i, hole) in cases().into_iter().enumerate() {
        // strictly inside D with margin, no slivers of triangles
        let area = {
            let n = hole.len();
            (0..n)
                .map(|k| hole[k].0 * hole[(k + 1) % n].1 - hole[(k + 1) % n].0 * hole[k].1)
                .sum::<f64>()
                / 2.0
        };
        let margin_ok = hole
            .iter()
            .all(|&p| p.0 <= ROD_FLAT - 5e-4 && p.0.hypot(p.1) <= ROD_R - 5e-4);
        if !margin_ok || area.abs() < 2e-5 {
            skipped += 1;
            continue;
        }
        let hole: Vec<(f64, f64)> = if area > 0.0 {
            hole.into_iter().rev().collect()
        } else {
            hole
        };
        let pen = penetration(&hole, &sb);
        let body = extruded(SketchPlane::xy(), vec![d_loop(), poly(&hole)], ROD_L, tol());
        if validate_geometric(&body, tol()).is_err() {
            skipped += 1;
            continue;
        }
        let creases = rod_creases(&body);
        let vol0 = volume(&body);
        match fillet_edges(&body, &creases, ROD_FILLET, tol()) {
            Err(e) => {
                if pen > 1e-7 {
                    refused_cross += 1;
                } else if pen < -1e-4 {
                    refused_clear += 1;
                    if !matches!(e.error, BlendError::RingClearance { .. }) {
                        eprintln!("  clear hole {i} pen {pen:.2e} refused {:?}", e.error);
                    } else {
                        eprintln!("  clear hole {i} pen {pen:.2e} refused RingClearance {hole:?}");
                    }
                } else {
                    grey += 1;
                }
            }
            Ok(out) => {
                built += 1;
                let mut why = Vec::new();
                if pen > 1e-7 {
                    why.push(format!("CARVED though the boundary enters S by {pen:.3e}"));
                }
                if let Err(e) = validate_geometric(&out.body, tol()) {
                    why.push(format!("tier 3: {e:?}"));
                }
                if let Err(e) =
                    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
                {
                    why.push(format!("tier 3': {e:?}"));
                }
                let dv = volume(&out.body) - vol0;
                if (dv - want).abs() > 1e-11 {
                    why.push(format!("ΔV {dv} vs {want}"));
                }
                // membership on a grid in the corner, mid-length
                for gx in 0..24 {
                    for gy in 0..24 {
                        let p = (
                            0.10 + 0.2 * (gx as f64 + 0.37) / 24.0,
                            0.25 + 0.2 * (gy as f64 + 0.41) / 24.0,
                        );
                        let expect_in = in_d(p) && !in_s(p, 1.0) && !in_tri(&hole, p);
                        let edge_near = sb
                            .iter()
                            .map(|q| (q.0 - p.0).hypot(q.1 - p.1))
                            .fold(f64::INFINITY, f64::min)
                            < 1e-4;
                        if edge_near {
                            continue;
                        }
                        match point_in_solid(
                            &out.body,
                            Point3::new(p.0, p.1, 0.5 * ROD_L),
                            band,
                            tol(),
                        ) {
                            Ok(SolidContainment::In) if !expect_in => {
                                why.push(format!("{p:?} In, expected Out"))
                            }
                            Ok(SolidContainment::Out) if expect_in => {
                                why.push(format!("{p:?} Out, expected In"))
                            }
                            Ok(_) => {}
                            // the query's own escalation near an edge: not a body defect
                            Err(_) => {}
                        }
                    }
                }
                if !why.is_empty() {
                    bad.push(format!(
                        "hole {i} {hole:?} pen {pen:.3e}: {}",
                        why.join("; ")
                    ));
                }
            }
        }
    }
    eprintln!(
        "built {built}, refused-crossing {refused_cross}, refused-clear {refused_clear}, grey {grey}, skipped {skipped}"
    );
    for b in &bad {
        eprintln!("BAD {b}");
    }
    assert!(bad.is_empty(), "{} bad", bad.len());
}

/// The PR's three rectangular-hole witnesses (and an axis-aligned
/// crossing hole) replayed at `Interval`.
#[test]
fn the_rectangle_witnesses_at_interval() {
    use geom_core::Interval;
    let iv = <Interval as geom_core::Real>::from_f64;
    let c = rod_chord_at(ROD_FLAT);
    let dl = bulge_loop(vec![
        (Point2::new(iv(ROD_FLAT), iv(c.half)), iv(c.wall_bulge)),
        (Point2::new(iv(ROD_FLAT), iv(-c.half)), iv(0.0)),
    ]);
    for (x0, y0, x1, y1, expect) in [
        (0.1, 0.35, 0.275, 0.3964, true),
        (0.12, 0.37, 0.27, 0.41, true),
        (0.24, 0.27, 0.28, 0.39, true),
        (0.2, 0.35, 0.296, 0.398, false),
    ] {
        let hole = bulge_loop(
            [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
                .into_iter()
                .map(|(x, y)| (Point2::new(iv(x), iv(y)), iv(0.0)))
                .collect(),
        );
        let body = extruded(
            SketchPlane::<Interval>::xy(),
            vec![dl.clone(), hole],
            iv(ROD_L),
            tol(),
        );
        let creases = rod_creases(&body);
        let r = fillet_edges(&body, &creases, iv(ROD_FILLET), tol());
        eprintln!(
            "Interval rect ({x0},{y0})-({x1},{y1}) expect carve {expect}: {}",
            match &r {
                Ok(_) => "carved".to_string(),
                Err(e) => format!("{:?}", e.error),
            }
        );
        if !expect {
            assert!(r.is_err(), "a crossing hole carved at Interval");
        }
    }
}

/// Claim 4: the keyhole at `r = BR` and `1.1·BR` — sampled membership
/// against the analytic plate-minus-keyhole-minus-two-slivers, and tier 3′.
#[test]
fn the_keyhole_at_the_discs_radius_matches_membership() {
    use profile::Profile;
    use sweep::{ExtrudeSide, Extrusion, extrude};
    const BR: f64 = 0.5;
    const W: f64 = 0.2;
    const XS: f64 = 0.8;
    let xs0 = (BR * BR - W * W).sqrt();
    let sweep_a = core::f64::consts::TAU - 2.0 * W.atan2(xs0);
    let ring = bulge_loop(vec![
        (Point2::new(xs0, W), (sweep_a / 4.0).tan()),
        (Point2::new(xs0, -W), 0.0),
        (Point2::new(XS, -W), 0.0),
        (Point2::new(XS, W), 0.0),
    ]);
    let p = Profile::new(
        SketchPlane::xy(),
        vec![
            poly(&[(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
            ring,
        ],
    )
    .validate(tol())
    .unwrap();
    let body = extrude(
        &p,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    let creases = rod_creases(&body);
    let band = Band::linear(tol()).unwrap();
    for r in [BR, 1.1 * BR] {
        let out = fillet_edges(&body, &creases, r, tol()).expect("carves");
        validate_pseudomanifold(&out.body, &ContactRecords::default(), tol()).expect("tier 3'");
        let in_hole = |q: (f64, f64)| {
            q.0.hypot(q.1) < BR || (q.0 >= xs0 - 1e-12 && q.0 <= XS && q.1.abs() <= W)
        };
        let in_sliver = |q: (f64, f64)| {
            let qy = q.1.abs();
            let cy = W + r;
            let cx = ((BR + r).powi(2) - cy * cy).sqrt();
            let s = BR / (BR + r);
            let quad = [(xs0, W), (cx, W), (cx, cy), (cx * s, cy * s)];
            let mut inside = true;
            let mut sg = 0.0f64;
            for i in 0..4 {
                let (a, b) = (quad[i], quad[(i + 1) % 4]);
                let cr = (b.0 - a.0) * (qy - a.1) - (b.1 - a.1) * (q.0 - a.0);
                if sg == 0.0 {
                    sg = cr.signum();
                } else if cr * sg < 0.0 {
                    inside = false;
                }
            }
            inside && (q.0 - cx).hypot(qy - cy) > r
        };
        let (mut n, mut bad) = (0, Vec::new());
        for gx in 0..60 {
            for gy in 0..60 {
                let q = (
                    -1.0 + 2.0 * (gx as f64 + 0.31) / 60.0,
                    -1.0 + 2.0 * (gy as f64 + 0.43) / 60.0,
                );
                let expect = !in_hole(q) && !in_sliver(q);
                match point_in_solid(&out.body, Point3::new(q.0, q.1, 0.37), band, tol()) {
                    Ok(SolidContainment::In) if !expect => bad.push(q),
                    Ok(SolidContainment::Out) if expect => bad.push(q),
                    Ok(_) => n += 1,
                    Err(e) => panic!("membership {q:?}: {e:?}"),
                }
            }
        }
        eprintln!(
            "keyhole r {r}: {n} agree, {} disagree {:?}",
            bad.len(),
            &bad[..bad.len().min(5)]
        );
        assert!(bad.is_empty());
    }
}
