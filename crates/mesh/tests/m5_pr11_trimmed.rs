//! M5 PR 11 §1 acceptance: the pcurve-driven trimmed-face lane — the
//! tiltedcut halves tessellate WATERTIGHT at the standard δ schedule,
//! shared edges (including the curved-trim ellipse arcs) keep the
//! compute-chords-once-per-edge contract, the mesh volume tracks the
//! closed form from inside, and the whole pipeline is bit-deterministic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;
use sweep::ExtrudeSide;
use topo::construction::Construct;

use geom_core::Tol;
use geom_core::{Point2, Point3, Vec3};
use mesh::validate::{check_mesh, signed_volume, triangle_count};
use profile::{Profile, SketchPlane, ValidatedProfile, test_support::bulge_loop};
use sweep::{Extrusion, extrude};
use topo::Body;
use topo::splitting::{SplitPart, split};

const R: f64 = 1.0;
const H: f64 = 2.5;
const PHI: f64 = 0.3;
const DELTAS: [f64; 3] = [0.1, 0.01, 0.001];

fn disc_of(r: f64) -> ValidatedProfile<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(-r, 0.0), 1.0),
        (Point2::new(r, 0.0), 1.0),
    ]);
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap()
}

fn halves() -> (Body<f64>, Body<f64>) {
    halves_of(R)
}

/// [`halves`] at cylinder radius `r` (same height, same cut).
fn halves_of(r: f64) -> (Body<f64>, Body<f64>) {
    let cylinder = extrude(
        &disc_of(r),
        Extrusion::Distance {
            depth: H,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    let cylinder = topo::test_support::finished("the cylinder", cylinder, Tol::witness());
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, H / 2.0),
        Vec3::new(PHI.sin(), 0.0, PHI.cos()),
        geom_core::Tol::witness(),
    );
    let result = split(&cylinder, &plane, Tol::witness()).unwrap();
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    (Body::clone(above), Body::clone(below))
}

/// Every cylinder-face triangle of `mesh` winds outward: its normal
/// points off the cylinder's axis (reversed on a `sense: false` face). A
/// triangle folded over the ellipse chord beside it is a closed manifold
/// mesh `check_mesh` passes, so the winding is read here (PR 5920's
/// review found such folds on these halves).
fn assert_cylinder_outward(label: &str, body: &Body<f64>, mesh: &mesh::Mesh) {
    for patch in &mesh.patches {
        let face = body.get_face(patch.face).unwrap();
        let geom::Surface::Cylinder { origin, axis, .. } = *body.get_surface(face.surface).unwrap()
        else {
            continue;
        };
        for tri in &patch.triangles {
            let [a, b, c] = tri.map(|i| mesh.positions[i as usize]);
            let centroid = a + ((b - a) + (c - a)) / 3.0;
            let d = centroid - origin;
            let out = d - axis * d.dot(axis);
            let out = if face.sense { out } else { -out };
            assert!(
                (b - a).cross(c - a).dot(out) > 0.0,
                "{label}: face {:?} triangle {tri:?} is wound against its face",
                patch.face
            );
        }
    }
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
            let mesh = mesh::tessellate(body, delta, Tol::witness()).unwrap_or_else(|e| {
                panic!("{label}: the PR 11 trimmed lane tessellates at {delta}: {e:?}")
            });
            check_mesh(&mesh)
                .unwrap_or_else(|e| panic!("{label} at {delta}: watertightness: {e:?}"));
            assert_cylinder_outward(&format!("{label} at {delta}"), body, &mesh);
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

/// **A wide trimmed cylinder triangulates in the lattice metric**
/// (`work/tess/trimmed-cylinder-lane-triangulates-in-raw-angle-by-height`).
/// The lane's lattice is square in 3-D, `hu` radians by `r·hu`, so in
/// (θ, z) it is a strip of aspect ~1/r; triangulated there, an
/// ellipse trim point between two rows fanned a sliver across many
/// columns, and the halves refused `CertificateExceeded` at r = 5 for
/// all twenty δ below, at r = 20 for 19, at r = 100 for 7 (bound up
/// to ~128·δ). In the lattice metric (`nurbs_cert::lattice_metric`)
/// every one meshes watertight, at πr²H/2 within the chordal sag.
#[test]
fn a_wide_tilted_cylinder_meshes_at_every_delta() {
    for r in [5.0, 20.0, 100.0] {
        let (above, _) = halves_of(r);
        let exact = core::f64::consts::PI * r * r * H / 2.0;
        for k in 0..20 {
            // The review's twenty δ, log-spaced 5e-2 → 3e-4, scaled by r.
            let delta = r * 5e-2 * (3e-4f64 / 5e-2).powf(f64::from(k) / 19.0);
            let mesh = mesh::tessellate(&above, delta, Tol::witness())
                .unwrap_or_else(|e| panic!("r = {r}, δ = {delta:e}: {e:?}"));
            check_mesh(&mesh).unwrap_or_else(|e| panic!("r = {r}, δ = {delta:e}: {e:?}"));
            assert_cylinder_outward(&format!("r = {r}, δ = {delta:e}"), &above, &mesh);
            let rel = (signed_volume(&mesh) - exact).abs() / exact;
            assert!(
                rel < 3.0 * delta / r,
                "r = {r}, δ = {delta:e}: volume off πr²H/2 by {rel}"
            );
        }
    }
}

/// The compute-chords-once-per-edge pin: every consecutive chord-id
/// pair of every boundary polyline — the ellipse section arcs
/// included — is an edge of EXACTLY two triangles, one per adjacent
/// face patch (shared ids, not re-derived points).
#[test]
fn shared_edges_consume_one_chord_set() {
    let (above, _) = halves();
    let mesh = mesh::tessellate(&above, 0.01, Tol::witness()).unwrap();
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
    let a = mesh::tessellate(&above, 0.01, Tol::witness()).unwrap();
    let b = mesh::tessellate(&above, 0.01, Tol::witness()).unwrap();
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
    match mesh::tessellate(&stripped, 0.01, Tol::witness()) {
        Err(mesh::TessellateError::UnsupportedCurve { note, .. }) => {
            assert!(note.contains("pcurve cache"), "{note}");
        }
        other => panic!("expected the typed missing-cache refusal, got {other:?}"),
    }
}

/// **A trimmed wall whose `sense` disagrees with its loops refuses
/// `TrimLoopOrientation`.** The loop runs interior-left about the face's
/// outward normal, so on the cylinder chart its polygon runs
/// counter-clockwise exactly when `sense` is true. Flipping the bit of
/// the tilted half's trimmed wall alone (an inside-out face, the
/// discriminating input `flipped_face_sense_for_tests` exists for)
/// leaves the polygon counter-clockwise against a reversed face: meshed,
/// its triangles would wind inward. It refuses instead, naming the face.
#[test]
fn a_wall_wound_against_its_sense_refuses_typed() {
    let (above, _) = halves();
    let wall = above
        .faces()
        .find(|(_, f)| {
            matches!(
                above.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .expect("the trimmed wall");
    let flipped = above
        .flipped_face_sense_for_tests(wall)
        .expect("a live face");
    match mesh::tessellate(&flipped, 0.01, Tol::witness()) {
        Err(mesh::TessellateError::TrimLoopOrientation { face }) => assert_eq!(face, wall),
        other => panic!(
            "the inside-out wall refuses typed, got {:?}",
            other.map(|_| ())
        ),
    }
}
