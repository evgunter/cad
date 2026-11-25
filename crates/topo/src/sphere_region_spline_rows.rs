//! Rows for [`super::SphereFaceRegion`]'s spline arcs at their ends: the
//! unit sphere's 60° rim carried on a spline cut into arcs, read against
//! the same rim on its circle. A leg whose plane passes a vertex of the
//! arcs at the band's escalation leaves the ring composite reading that
//! end near, and the crossing beside it is the end's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::*;
use crate::entity::{EdgeKey, LoopKey};
use core::f64::consts::TAU;
use slotmap::KeyData;

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

fn sphere() -> Charted<f64> {
    Charted::record(
        Default::default(),
        &geom::Surface::Sphere {
            frame: geom::Frame::new(
                Point3::origin(),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            radius: 1.0,
        },
    )
}

/// The rim on its circle, run the circle's way round.
fn circle() -> SphereFaceRegion<f64> {
    SphereFaceRegion::on(
        sphere(),
        true,
        vec![RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(1)),
            r#loop: LoopKey::default(),
            cycle: 0,
            carrier: geom::Curve3::circle(
                Point3::new(0.0, 0.0, 0.5),
                Vec3::new(0.0, 0.0, 1.0),
                0.75f64.sqrt(),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            t0: 0.0,
            t1: TAU,
            turn: crate::region::fixture_turn(0.0, TAU),
            forward: true,
        }],
    )
}

/// The rim's rational quadratic of nine control points.
fn rim_spline() -> std::sync::Arc<geom::NurbsCurve3<f64>> {
    let (r, w) = (0.75f64.sqrt(), core::f64::consts::FRAC_1_SQRT_2);
    let corners = [
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let control = corners
        .iter()
        .map(|&(x, y)| Point3::new(r * x, r * y, 0.5))
        .collect();
    let weights = (0..9).map(|k| if k % 2 == 0 { 1.0 } else { w }).collect();
    let knots = geom_core::spline::KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    std::sync::Arc::new(geom::NurbsCurve3::new(knots, control, weights).unwrap())
}

/// The rim on its spline, cut into two arcs at `t = ½`: vertices at
/// `(r, 0, ½)` and `(−r, 0, ½)`.
fn halves() -> SphereFaceRegion<f64> {
    let net = rim_spline();
    let edges = [(0.0, 0.5), (0.5, 1.0)]
        .into_iter()
        .enumerate()
        .map(|(i, (t0, t1))| RegionEdge {
            edge: EdgeKey::from(KeyData::from_ffi(1 + i as u64)),
            r#loop: LoopKey::default(),
            cycle: 0,
            carrier: geom::Curve3::Nurbs(net.clone()),
            t0,
            t1,
            turn: None,
            forward: true,
        })
        .collect();
    SphereFaceRegion::on(
        sphere(),
        true,
        edges,
    )
}

/// The normal of a plane through the centre holding `w` that passes the
/// point `v` at the signed height `h`.
fn plane_at_height(v: Vec3<f64>, w: Vec3<f64>, h: f64) -> Vec3<f64> {
    let e1 = w.normalize();
    let perp = v - e1 * v.dot(e1);
    let (m, e2) = (perp.norm(), perp.normalize());
    let e3 = e1.cross(e2);
    let cb = h / m;
    e2 * cb + e3 * (1.0 - cb * cb).sqrt()
}

/// Every landing of the great-circle leg in the plane of normal `n`,
/// started square to it from a point of the plane, with whether it
/// reads no side; or the first edge's fault.
fn landings_of(
    region: &SphereFaceRegion<f64>,
    n: Vec3<f64>,
) -> Result<Vec<(Point3<f64>, bool)>, String> {
    let n = n.normalize();
    let helper = if n.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let p = n.cross(helper).normalize();
    let t = n.cross(p).normalize();
    let foot = QueryFoot::read(region.charted.surface(), Point3::origin() + p, band());
    let leg = region.ray(foot, t);
    let mut out = Vec::new();
    for e in &region.edges {
        let met = region::landings(region, e, &leg, band()).map_err(|f| format!("{f:?}"))?;
        out.extend(met.into_iter().map(|(y, _, sideless)| (y, sideless)));
    }
    Ok(out)
}

/// **A crossing beside a spline arc's end is never lost.** Legs whose
/// planes pass the vertex `(r, 0, ½)` of [`halves`] at about the band's
/// escalation, either side: the composite reads that end near and the
/// crossing (as far as tens of bands along the rim from the vertex) is
/// the end's. Each reading either sets the leg aside or keeps an even
/// count of sided landings, or a landing at the end that reads no side:
/// a rim meets a great circle an even number of times. A reader that
/// decides the end again on a narrower margin read it off the plane,
/// kept one sided landing, and lost the other.
#[test]
fn a_crossing_beside_a_spline_arcs_end_is_never_lost() {
    let net = rim_spline();
    let s = halves();
    let v = net.eval(0.0) - Point3::origin();
    for w in [Vec3::new(0.3, -0.7, 0.2), Vec3::new(-0.5, 0.5, -0.4)] {
        for target in [band().escalate(), -band().escalate()] {
            for j in -10i32..=10 {
                let h = target * (1.0 + f64::from(j) * 1e-10);
                if let Ok(met) = landings_of(&s, plane_at_height(v, w, h)) {
                    let sided = met.iter().filter(|(_, sideless)| !sideless).count();
                    let sideless = met.iter().filter(|(_, sideless)| *sideless).count();
                    assert!(
                        sided % 2 == 0 || sideless > 0,
                        "w {w:?}, h {h:e}: {sided} sided landings and none sideless: {met:?}"
                    );
                }
            }
        }
    }
}

/// **The lost crossing does not reach the verdict.** Points a few
/// hundredths of a radian past the crossing beside [`halves`]'s vertex,
/// along the great circle whose plane passes the vertex at the band's
/// escalation and aims at the arc's second target: the spline face reads
/// each as the circle face does. Read where the end's meeting was
/// decided again off the plane, a third of them read the other side.
#[test]
fn the_crossing_beside_a_spline_arcs_end_reaches_the_verdict() {
    let net = rim_spline();
    let (c, s) = (circle(), halves());
    let v = net.eval(0.0) - Point3::origin();
    let target = net.eval(0.125) - Point3::origin();
    let mut tried = 0;
    for j in -20i32..=20 {
        for sign in [1.0, -1.0] {
            let h = sign * band().escalate() * (1.0 + f64::from(j) * 1e-10);
            let n = plane_at_height(v, target, h);
            let Ok(on_circle) = landings_of(&c, n) else {
                continue;
            };
            let Some(&(x, _)) = on_circle
                .iter()
                .find(|(y, _)| (*y - (Point3::origin() + v)).norm() < 1e-3)
            else {
                continue;
            };
            let xh = (x - Point3::origin()).normalize();
            let mut tau = n.normalize().cross(xh).normalize();
            if (target - xh).dot(tau) > 0.0 {
                tau = -tau;
            }
            let p = Point3::origin() + (xh * 0.02f64.cos() + tau * 0.02f64.sin()).normalize();
            tried += 1;
            assert_eq!(
                s.contains(p, band()),
                c.contains(p, band()),
                "h {h:e}: the spline face and the circle face at {p:?}"
            );
        }
    }
    assert!(tried >= 40, "{tried} points read");
}

/// **An end's meeting farther along a shallow approach is not placed at
/// the end.** A great circle through the rim at azimuth `3·10⁻⁸`, three
/// bands from [`halves`]'s vertex along it, meets the circle face's rim
/// there, sided. The spline arc's end reads near and its stretch reaches
/// past the band's zero of the vertex, so the leg is set aside rather
/// than read with a landing at the vertex.
#[test]
fn an_ends_meeting_along_a_shallow_approach_is_not_placed_at_the_end() {
    let r = 0.75f64.sqrt();
    let at = Vec3::new(r * 3e-8f64.cos(), r * 3e-8f64.sin(), 0.5);
    let n = at.cross(Vec3::new(0.3, -0.7, 0.2));
    let on_circle = landings_of(&circle(), n).expect("the circle reads the leg");
    assert!(
        on_circle.iter().all(|(_, sideless)| !sideless),
        "the circle's landings read a side: {on_circle:?}"
    );
    match landings_of(&halves(), n) {
        Err(_) => {}
        Ok(met) => assert!(
            met.iter().all(|(y, sideless)| {
                !sideless
                    || on_circle
                        .iter()
                        .all(|(x, _)| (*x - *y).norm() < band().zero())
            }),
            "a landing at the vertex stands for a crossing past the band from it: {met:?} \
             against the circle's {on_circle:?}"
        ),
    }
}
