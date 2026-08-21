//! M5 PR 11 §1 acceptance: the pcurve-driven trimmed-face lane — the
//! tiltedcut halves tessellate WATERTIGHT at the standard δ schedule,
//! shared edges (including the curved-trim ellipse arcs) keep the
//! compute-chords-once-per-edge contract, the mesh volume tracks the
//! closed form from inside, and the whole pipeline is bit-deterministic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use profile::RawLoop;
use std::collections::HashMap;

use geom_core::{Point2, Point3, Tolerance, Vec3};
use mesh::validate::{check_mesh, signed_volume, triangle_count};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::Body;
use topo::splitting::{SplitPart, SplitPlane, split};
use geom_core::Tol;

const R: f64 = 1.0;
const H: f64 = 2.5;
const PHI: f64 = 0.3;
const DELTAS: [f64; 3] = [0.1, 0.01, 0.001];

fn disc() -> ValidatedProfile<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-R, 0.0), 1.0),
        ProfileVertex::new(Point2::new(R, 0.0), 1.0),
    ]);
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap()
}

fn halves() -> (Body<f64>, Body<f64>) {
    let cylinder = extrude(&disc(), Extrusion::Distance(H)).unwrap().body;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, H / 2.0),
        normal: Vec3::new(PHI.sin(), 0.0, PHI.cos()),
    };
    let result = split(&cylinder, &plane).unwrap();
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    (above.clone(), below.clone())
}

/// Watertight at every δ of the standard schedule, positive signed
/// volume, and the chordal (inscribed) volume converges to πr²H/2.
#[test]
fn tilted_halves_tessellate_watertight_across_the_delta_schedule() {
    let (above, below) = halves();
    let exact = core::f64::consts::PI * R * R * H / 2.0;
    for (label, body) in [("above", &above), ("below", &below)] {
        let mut last_err = f64::INFINITY;
        for delta in DELTAS {
            let mesh = mesh::tessellate(body, delta).unwrap_or_else(|e| {
                panic!("{label}: the PR 11 trimmed lane tessellates at {delta}: {e:?}")
            });
            check_mesh(&mesh)
                .unwrap_or_else(|e| panic!("{label} at {delta}: watertightness: {e:?}"));
            let v = signed_volume(&mesh);
            assert!(v > 0.0, "{label} at {delta}: outward orientation");
            let rel = (v - exact).abs() / exact;
            assert!(
                rel < last_err + 1e-12,
                "{label}: refinement must not regress ({rel} after {last_err})"
            );
            last_err = rel;
            assert!(
                triangle_count(&mesh) > 0,
                "{label} at {delta}: nonempty mesh"
            );
        }
        assert!(
            last_err < 1.5e-3,
            "{label}: finest mesh within 0.15% of πr²H/2 (inscribed, δ = 1e-3 ⇒ \
             mean sag ~ δ), got {last_err}"
        );
    }
}

/// The compute-chords-once-per-edge pin: every consecutive chord-id
/// pair of every boundary polyline — the ellipse section arcs
/// included — is an edge of EXACTLY two triangles, one per adjacent
/// face patch (shared ids, not re-derived points).
#[test]
fn shared_edges_consume_one_chord_set() {
    let (above, _) = halves();
    let mesh = mesh::tessellate(&above, 0.01).unwrap();
    let mut uses: HashMap<(u32, u32), u32> = HashMap::new();
    for patch in &mesh.patches {
        for t in &patch.triangles {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
    }
    for poly in &mesh.boundaries {
        for w in poly.points.windows(2) {
            let key = (w[0].min(w[1]), w[0].max(w[1]));
            assert_eq!(
                uses.get(&key).copied().unwrap_or(0),
                2,
                "boundary segment {key:?} of edge {:?} must be shared by exactly two \
                 triangles (compute-chords-once-per-edge)",
                poly.edge
            );
        }
    }
}

/// D9: byte-identical meshes for identical (body, δ) through the
/// trimmed lane.
#[test]
fn trimmed_lane_is_bit_deterministic() {
    let (above, _) = halves();
    let a = mesh::tessellate(&above, 0.01).unwrap();
    let b = mesh::tessellate(&above, 0.01).unwrap();
    assert_eq!(a.positions.len(), b.positions.len());
    for (p, q) in a.positions.iter().zip(&b.positions) {
        assert_eq!(p.x.to_bits(), q.x.to_bits());
        assert_eq!(p.y.to_bits(), q.y.to_bits());
        assert_eq!(p.z.to_bits(), q.z.to_bits());
    }
    for (pa, pb) in a.patches.iter().zip(&b.patches) {
        assert_eq!(pa.triangles, pb.triangles);
    }
}

/// The honest door: a trimmed face whose stored pcurve caches are
/// missing refuses TYPED, naming where caches mint — never a silently
/// re-derived branch guess.
#[test]
fn missing_pcurve_caches_refuse_typed() {
    let (above, _) = halves();
    let mut stripped = above.clone();
    let hes: Vec<_> = stripped.pcurves().map(|(he, _)| he).collect();
    for he in hes {
        stripped.detach_pcurve(he);
    }
    match mesh::tessellate(&stripped, 0.01) {
        Err(mesh::TessellateError::UnsupportedCurve { note, .. }) => {
            assert!(note.contains("pcurve cache"), "{note}");
        }
        other => panic!("expected the typed missing-cache refusal, got {other:?}"),
    }
}
