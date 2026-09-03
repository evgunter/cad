//! REVIEW PROBES (adversarial): per-triangle certificate falsification
//! on the three fixture walls at two deltas each, plus gate probes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use profile::RawLoop;

use geom_core::{Affine3, Point2, Vec3};
use sweep::loft_body;
// The corpus swept elbow is the kernel crate's fixture, not this
// suite's: the falsification rows below need the SAME solid the
// skin-integrality bracket and the STEP fixture meter.
use sweep::test_support::swept_elbow;
use topo::Body;

use crate::common;
use common::quad;
use geom_core::Tol;

const SQ: [(f64, f64); 4] = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
const TRAP: [(f64, f64); 4] = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];

fn loft_at(zs: &[f64]) -> Body<f64> {
    let sections = vec![quad(SQ), quad(TRAP), quad(SQ)];
    let places: Vec<Affine3<f64>> = zs
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    loft_body::<f64>(&sections, &places, 2, Tol::witness())
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
    let v = |x: f64, y: f64, bulge: f64| sweep::ProfileVertex::new(Point2::new(x, y), bulge);
    let lp = sweep::ProfileLoop::new(vec![v(1.0, 0.0, 0.4), v(0.0, 1.0, 0.0), v(0.0, 0.0, 0.0)]);
    let sections = vec![vec![lp.clone()], vec![lp]];
    let places: Vec<Affine3<f64>> = [0.0, 1.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    loft_body::<f64>(&sections, &places, 1, Tol::witness())
        .expect("the rational pie lofts")
        .body
}

/// The four fixtures the Z1 rows drive, and the two deltas they drive
/// them at. Shared by the armed row and its default-build counterpart
/// so the two cannot drift into "the falsifier covers a corpus the
/// default build never tessellates".
fn z1_fixtures() -> [(&'static str, Body<f64>); 4] {
    [
        ("loft_prism", loft_at(&[0.0, 1.0, 2.0])),
        ("nonuniform_loft", loft_at(&[0.0, 1.0, 3.0])),
        ("swept_elbow", swept_elbow(Tol::witness())),
        // Promoted from the Z1R frontier pin at M8-3: the rational
        // wall's arc cap rim now mints a stored pcurve
        // (`Pcurve::IsoArc`), so the rational pie tessellates and the
        // armed per-triangle falsifier covers the RATIONAL lane end to
        // end — which is what its retirement condition asked for.
        ("rational_pie", rational_pie()),
    ]
}

/// See [`z1_fixtures`].
const Z1_DELTAS: [f64; 2] = [3e-2, 6e-3];

/// Z1: per-triangle |S - Pi| vs cert on every NURBS triangle of all
/// four fixtures at two deltas, at the falsifier's own 12-samples-per-
/// edge density.
///
/// The check is `worst_ratio <= 1` — the largest `|S-Pi| / (cert + eps)`
/// any sample on any triangle of the face reached. That is the
/// per-TRIANGLE claim, not a per-face average: a single sample above
/// its own triangle's certificate pushes the face's maximum over 1.
/// The planted 0.25 -> 0.05 cert bug the M8-5 review used dies HERE,
/// empirically, not in a formula mirror.
///
/// **GATED**: `mesh::budget`'s instrument is compiled only under
/// `mesh`'s `budget` feature, so this row rides that build — with the
/// feature off there is no `arm`/`take` to call, which is the point of
/// the gate rather than a limitation of it. The hosted gate runs it in
/// ci.yml's "mesh budget meter + certificate falsifier
/// (feature = budget)" row (mirrored by local-scripts/ci-local.sh).
///
/// **FREQUENCY, corrected 2026-08-22 — this row is no longer
/// unconditional.** That step rides `k-lint`'s `dev-budget` feature
/// row, and `k-lint` now SAMPLES one of its five feature unifications
/// per run, so the falsifier runs on an expected 1 run in 5 rather than
/// on every build-triggering change. The draw is seeded from the head
/// SHA under its own salt, so a re-run of one commit draws the same row
/// and the draw is recoverable from the SHA without the logs;
/// repetition covers the matrix at this repository's ~60 runs/hour of
/// active work.
///
/// M8-5 MIN-1's intent survives the change, and the reason is specific
/// rather than reassuring: this row is a PERSISTENCE detector. A
/// certificate that stopped dominating its own samples stays broken in
/// the tree, so a later draw still finds it — the red is deferred, not
/// lost. Sampling would NOT be sound for a detector of absence (a row
/// deleted, or a gate sited where it cannot fire), because an absence
/// merges silently once and leaves no future red; that class stays
/// unconditional elsewhere in CI. What has moved, twice now, is which
/// build the row rides in and how often it is drawn — never whether
/// the claim is checked.
///
/// The ASSERTION is here and not in the tessellation lane, which is
/// what keeps `mesh::tessellate`'s typed-error contract out of reach
/// of any feature: no build of the kernel can turn a certificate
/// violation into a panic, because no build of the kernel asserts.
#[cfg(feature = "budget")]
#[test]
fn z1_per_triangle_certificate_falsification() {
    use mesh::budget::{self, Mode};
    for (name, body) in z1_fixtures() {
        for delta in Z1_DELTAS {
            budget::arm(Mode::Deviation {
                samples_per_edge: 12,
            });
            let m = mesh::tessellate(&body, delta, Tol::witness()).expect("tessellates");
            let measures = budget::take();
            assert!(
                !measures.is_empty(),
                "{name}: no NURBS face was measured — the falsifier sampled nothing"
            );
            let samples: u64 = measures.iter().map(|f| f.dev_samples).sum();
            assert!(samples > 0, "{name}: the deviation pass sampled nothing");
            // ASSERT OVER EVERY FACE, not over a maximum. Picking the
            // worst first and asserting on it makes the check only as
            // NaN-robust as the comparator: `total_cmp` is IEEE
            // totalOrder, which sorts a NEGATIVE NaN below −∞, so a
            // face carrying one would come out "smallest" and escape.
            // A loop has no comparator to get wrong, and `!(x <= 1.0)`
            // is true for every NaN of either sign.
            // MONOTONE THE EASY WAY, and this is the row hosted CI
            // runs: `worst_ratio` only shrinks as `bound` grows, so a
            // loose certificate passes this by a WIDER margin than a
            // tight one. `budget_meter.rs`'s sibling row grew a
            // measured floor beside its ceiling; this one has not.
            // **The asymmetry is known and unfixed**, here and in two
            // more instances of the same shape in `nurbs_cert.rs`'s own
            // test module: a ceiling with no floor cannot tell a tight
            // certificate from a loose one.
            for f in &measures {
                assert!(
                    f.worst_ratio <= 1.0,
                    "{name}: a triangle's samples exceeded its certificate ({:?}, \
                     worst d/(cert+eps) = {})",
                    f.face,
                    f.worst_ratio
                );
            }
            // Reporting only, after the assertion: this pair is the
            // worst SAMPLE's deviation and its own triangle's
            // certificate, which need not be the sample that produced
            // `worst_ratio` — the two maxima are taken independently.
            // Printed as headroom, never read as the violating pair.
            let loudest = measures
                .iter()
                .max_by(|a, b| a.worst_ratio.total_cmp(&b.worst_ratio))
                .expect("at least one measured face");
            println!(
                "{name} delta={delta:.0e}: tris={} samples={samples} \
                 worst|S-Pi|={:.3e} (that sample's cert {:.3e}) max d/cert={:.4}",
                m.patches.iter().map(|p| p.triangles.len()).sum::<usize>(),
                loudest.worst_dev,
                loudest.worst_dev_cert,
                loudest.worst_ratio,
            );
        }
    }
}

/// The Z1 corpus in a DEFAULT build, where the falsifier is gated out
/// — the row that keeps the default suite's coverage of these four
/// fixtures, and its count, when the armed row moves to the `budget`
/// build.
///
/// It asserts what a default build can honestly assert: the fixtures
/// still tessellate at both Z1 deltas and the results are watertight.
/// It deliberately does NOT claim to falsify anything — the
/// certificate check needs the 12-sample resampling that this build
/// does not contain, and a same-named row quietly doing less would be
/// exactly the fail-quiet the gate is supposed to avoid.
///
/// INVARIANT this row pins: gating the falsifier removed the SAMPLING,
/// not the lane. If `mesh::tessellate` starts refusing these fixtures
/// in a default build, that is a tessellation regression and it fails
/// here, in the default row, rather than only in the feature row.
#[cfg(not(feature = "budget"))]
#[test]
fn z1_fixtures_still_tessellate_with_the_falsifier_gated_out() {
    for (name, body) in z1_fixtures() {
        for delta in Z1_DELTAS {
            let m = mesh::tessellate(&body, delta, Tol::witness()).expect("tessellates");
            let tris: usize = m.patches.iter().map(|p| p.triangles.len()).sum();
            assert!(tris > 0, "{name} at delta={delta:.0e}: empty mesh");
            mesh::validate::check_mesh(&m)
                .unwrap_or_else(|e| panic!("{name} at delta={delta:.0e}: not watertight: {e:?}"));
        }
    }
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
    match mesh::tessellate(&body, 1e-2, Tol::witness()) {
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
    let body = swept_elbow(Tol::witness());
    let m = mesh::tessellate(&body, 1e-2, Tol::witness()).expect("tessellates");
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
        let mesh = mesh::tessellate(&loft_at(&[0.0, 1.0, 2.0]), delta, Tol::witness())
            .expect("tessellates");
        mesh::validate::check_mesh(&mesh).expect("watertight at fine delta");
    }
}
