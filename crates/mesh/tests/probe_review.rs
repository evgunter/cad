//! REVIEW PROBES (adversarial): per-triangle certificate falsification
//! on the three fixture walls at two deltas each, plus gate probes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;
use profile::RawLoop;

use geom_core::{Affine3, Point2, Point3, Vec3};
use sweep::{SketchSegment, loft_body, segment_curve, sweep_body};
use topo::Body;

mod common;
use common::quad;

const SQ: [(f64, f64); 4] = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
const TRAP: [(f64, f64); 4] = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];

fn loft_at(zs: &[f64]) -> Body<f64> {
    let sections = vec![quad(SQ), quad(TRAP), quad(SQ)];
    let places: Vec<Affine3<f64>> = zs
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    loft_body::<f64>(&sections, &places, 2)
        .expect("loft builds")
        .body
}

/// A RATIONAL-walled loft (M8-5): a pie-slice profile whose curved
/// side is a single-span arc (bulge 0.4 — safely under the quarter-turn
/// sub-arc split, so no C⁰ double knot), lofted straight up. Its wall
/// is a genuinely rational NURBS face (weights `[1, cos(θ/2), 1]`) —
/// the class M8-2's rational span meter made BUILDABLE and whose
/// Hessian/sagitta bounds M8-5 certifies (`nurbs_cert`/`chords`).
fn rational_pie() -> Body<f64> {
    let v = |x: f64, y: f64, bulge: f64| sweep::ProfileVertex {
        pos: Point2::new(x, y),
        bulge,
    };
    let lp = sweep::ProfileLoop::new(vec![v(1.0, 0.0, 0.4), v(0.0, 1.0, 0.0), v(0.0, 0.0, 0.0)]);
    let sections = vec![vec![lp.clone()], vec![lp]];
    let places: Vec<Affine3<f64>> = [0.0, 1.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    loft_body::<f64>(&sections, &places, 1)
        .expect("the rational pie lofts")
        .body
}

fn swept_elbow() -> Body<f64> {
    let (r, h) = (3.0, 0.25);
    let path = segment_curve(
        0,
        SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(r, r),
            bulge: (core::f64::consts::PI / 8.0).tan(),
        },
        Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            -FRAC_PI_2,
        ),
    )
    .expect("arc path");
    sweep_body::<f64>(
        &quad([(-h, -h), (h, -h), (h, h), (-h, h)]),
        Affine3::identity(),
        &path,
        9,
        3,
    )
    .expect("sweep builds")
    .body
}

/// Z1: per-triangle |S - Pi| vs cert on every NURBS triangle of all
/// three fixtures at two deltas — the probe assertions live inside
/// trimmed.rs (env-gated); here we drive them and print headroom.
#[test]
fn z1_per_triangle_certificate_falsification() {
    // MIN-1 adoption: the probe is the SUITE'S guard now — it arms
    // itself (mesh::probe_stats::arm) instead of demanding an env
    // var, so the hosted gate runs it unconditionally. The planted
    // 0.25 -> 0.05 cert bug the review used dies HERE, empirically,
    // not in a formula mirror.
    mesh::probe_stats::arm(true);
    for (name, body) in [
        ("loft_prism", loft_at(&[0.0, 1.0, 2.0])),
        ("nonuniform_loft", loft_at(&[0.0, 1.0, 3.0])),
        ("swept_elbow", swept_elbow()),
        // Promoted from the Z1R frontier pin at M8-3: the rational
        // wall's arc cap rim now mints a stored pcurve
        // (`Pcurve::IsoArc`), so the rational pie tessellates and the
        // armed per-triangle falsifier covers the RATIONAL lane end to
        // end — which is what its retirement condition asked for.
        ("rational_pie", rational_pie()),
    ] {
        for delta in [3e-2, 6e-3] {
            let _ = mesh::probe_stats::take();
            let m = mesh::tessellate(&body, delta).expect("tessellates");
            let (worst_d, its_cert, max_ratio, count) = mesh::probe_stats::take();
            println!(
                "{name} delta={delta:.0e}: tris={} samples={count} worst|S-Pi|={worst_d:.3e} \
                 (its cert {its_cert:.3e}) max d/cert={max_ratio:.4}",
                m.patches.iter().map(|p| p.triangles.len()).sum::<usize>(),
            );
            assert!(
                max_ratio <= 1.0,
                "{name}: a triangle's samples exceeded its certificate"
            );
        }
    }
    mesh::probe_stats::arm(false);
}

/// Z2 (d'): a NURBS-face half-edge with NO stored pcurve must refuse
/// typed at the chord pass, before any Mesh exists.
#[test]
fn z2_detached_pcurve_refuses_typed() {
    let mut body = loft_at(&[0.0, 1.0, 2.0]);
    let hek = body
        .pcurves()
        .map(|(h, _)| h)
        .next()
        .expect("body has pcurve caches");
    body.detach_pcurve(hek);
    match mesh::tessellate(&body, 1e-2) {
        Err(e) => {
            let s = format!("{e:?}");
            assert!(
                s.contains("UnsupportedCurve") || s.contains("pcurve"),
                "refusal names the pcurve gap: {s}"
            );
        }
        Ok(_) => panic!("tessellated with a detached pcurve cache"),
    }
}

/// Z5: cross-process bitwise determinism — print a hash of all
/// position bits; compare across two separate cargo invocations.
#[test]
fn z5_positions_hash_stamp() {
    let body = swept_elbow();
    let m = mesh::tessellate(&body, 1e-2).expect("tessellates");
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for p in &m.positions {
        for b in [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()] {
            h ^= b;
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    }
    for pa in &m.patches {
        for t in &pa.triangles {
            for &i in t {
                h ^= u64::from(i);
                h = h.wrapping_mul(0x1000_0000_01b3);
            }
        }
    }
    println!("Z5 HASH {h:016x} positions={} ", m.positions.len());
}

/// Z3: the worst-case boundary — very fine NURBS walls against
/// one-chord-sufficient planar caps; watertightness must hold by
/// shared chord ids (a T-junction would surface as a boundary edge).
#[test]
fn z3_fine_nurbs_vs_coarse_planar_neighbor_watertight() {
    for delta in [5e-4, 2e-4] {
        let mesh = mesh::tessellate(&loft_at(&[0.0, 1.0, 2.0]), delta).expect("tessellates");
        mesh::validate::check_mesh(&mesh).expect("watertight at fine delta");
    }
}
