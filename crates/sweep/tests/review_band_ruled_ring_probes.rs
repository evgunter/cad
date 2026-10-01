//! Review probes for the ring-cycle ruled cut-off (PR 3243).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Sign, Tol};
use profile::{Profile, ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, Convexity, fillet_edges};
use sweep::test_support::rod_creases;
use sweep::{Extrusion, extrude};
use topo::{Body, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("closed-form props");
    p.volume
}

fn square(h: f64) -> ProfileLoop<f64> {
    bulge_loop(
        [(-h, -h), (h, -h), (h, h), (-h, h)]
            .into_iter()
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    )
}

const BR: f64 = 0.5;
const W: f64 = 0.2; // keyhole slot half-width
const XS: f64 = 0.8; // slot end

/// A keyhole through-hole: a disc of radius BR plus the slot
/// [xs0, XS] x [-W, W]. At the two disc/slot junctions the HOLE is
/// reflex, so the cylinder-plane creases there are CONVEX, and they end
/// in the caps' rings.
fn keyhole_block() -> Body<f64> {
    keyhole_block_with(Vec::new())
}

/// [`keyhole_block`] with further profile loops (bores) through it.
fn keyhole_block_with(extra: Vec<ProfileLoop<f64>>) -> Body<f64> {
    let xs0 = (BR * BR - W * W).sqrt();
    let sweep = core::f64::consts::TAU - 2.0 * W.atan2(xs0);
    let ring = bulge_loop(vec![
        (Point2::new(xs0, W), (sweep / 4.0).tan()),
        (Point2::new(xs0, -W), 0.0),
        (Point2::new(XS, -W), 0.0),
        (Point2::new(XS, W), 0.0),
    ]);
    let mut loops = vec![square(1.0), ring];
    loops.extend(extra);
    let p = Profile::new(SketchPlane::xy(), loops)
        .validate(tol())
        .expect("the keyholed profile validates");
    extrude(&p, Extrusion::Distance(1.0), tol())
        .expect("the keyholed profile extrudes")
        .body
}

/// The area the convex band removes at one keyhole junction, per unit
/// length: V = (xs0, W); ball centre c = (cx, W + r) with |c| = BR + r;
/// F_b = (cx, W) on the plane, F_a = c·BR/(BR + r) on the hole wall.
fn keyhole_cut(r: f64) -> f64 {
    let xs0 = (BR * BR - W * W).sqrt();
    let cy = W + r;
    let cx = ((BR + r).powi(2) - cy * cy).sqrt();
    let v = (xs0, W);
    let fb = (cx, W);
    let c = (cx, cy);
    let s = BR / (BR + r);
    let fa = (cx * s, cy * s);
    let quad = [v, fb, c, fa];
    let mut twice = 0.0;
    for i in 0..4 {
        let (p, q) = (quad[i], quad[(i + 1) % 4]);
        twice += p.0 * q.1 - q.0 * p.1;
    }
    let quad_area = 0.5 * twice.abs();
    // Ball sector at c between F_b (straight down) and F_a (towards O).
    let ang_b = (-1.0f64).atan2(0.0); // -pi/2
    let ang_a = (-cy).atan2(-cx);
    let mut dth = (ang_b - ang_a).abs();
    if dth > core::f64::consts::PI {
        dth = core::f64::consts::TAU - dth;
    }
    let sector = 0.5 * r * r * dth;
    // Disc segment between V and F_a on the hole wall, inside the quad.
    let phi = (fa.1.atan2(fa.0) - v.1.atan2(v.0)).abs();
    let segment = 0.5 * BR * BR * (phi - phi.sin());
    quad_area - sector - segment
}

/// **A keyhole's CONVEX creases end in the caps' rings and carve at
/// the closed form**: the disc/slot junctions are reflex in the hole,
/// so the band REMOVES `A` per unit length at each, `ΔV = −2·A·L`.
#[test]
fn a_keyhole_fillets_its_convex_ring_creases_at_the_closed_form() {
    let body = keyhole_block();
    validate_geometric(&body, tol()).expect("the keyholed block is tier-3 valid");
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 2, "the two disc/slot junctions");
    let vol0 = volume(&body);
    for r in [0.05, 0.1] {
        let out = fillet_edges(&body, &creases, r, tol())
            .unwrap_or_else(|e| panic!("r {r}: both convex ring creases carve, got {e}"));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("r {r}: tier 3, {e:?}"));
        let dv = volume(&out.body) - vol0;
        let want = -2.0 * keyhole_cut(r);
        assert!((dv - want).abs() < 1e-12, "r {r}: ΔV {dv} vs {want}");
    }
}

/// A round profile loop: two semicircles about `(x, y)`.
fn bore(x: f64, y: f64, a: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(x + a, y), 1.0),
        (Point2::new(x - a, y), 1.0),
    ])
}

/// **A bore in the sliver a keyhole crease's cut-off removes refuses**,
/// where the cut-off runs in the cap's keyhole RING and the bore is a
/// second ring of the same cap. At r = 0.1 the upper junction's ball
/// centre is `c = (√0.27, 0.3)` and its old vertex `V = (√0.21, 0.2)`,
/// so the sliver lies within `0.1 ≤ ‖p − c‖ ≤ ‖V − c‖ ≈ 0.1173`; the
/// bore at `(0.4623, 0.204)` spans `‖p − c‖ ∈ [0.1108, 0.1128]`, wholly
/// in the material the band removes.
#[test]
fn a_bore_in_a_keyhole_creases_removed_sliver_refuses_ring_clearance() {
    let body = keyhole_block_with(vec![bore(0.4623, 0.204, 0.001)]);
    validate_geometric(&body, tol()).expect("the bored keyhole block is tier-3 valid");
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 2, "the two disc/slot junctions");
    match fillet_edges(&body, &creases, 0.1, tol()).map_err(|e| {
        let text = e.error.to_string();
        (e.error, text)
    }) {
        Err((
            BlendError::RingClearance {
                face,
                chain,
                margin,
            },
            text,
        )) => {
            assert_eq!(
                chain,
                Convexity::Convex,
                "a cap meter refuses only beside a convex crease"
            );
            assert!(
                text.contains(
                    "lies in the part of a face the blend cuts away with the material it removes"
                ),
                "the sentence says the edge is cut away with the sliver: {text}"
            );
            assert_eq!(margin.sign, Sign::Negative, "definite, got {margin}");
            let f = body
                .get_face(face)
                .expect("the refusal names a source face");
            assert_eq!(
                f.rings.len(),
                2,
                "the face named is a cap: keyhole and bore"
            );
        }
        Err((other, _)) => panic!("expected RingClearance at the cap, got {other:?}"),
        Ok(_) => {
            panic!("the carve returned a body keeping the bore on a cap that no longer covers it")
        }
    }
}

/// **A bore clear of both slivers carves at the keyhole's closed form**,
/// carried through untouched: `ΔV = −2·A·L` as without it.
#[test]
fn a_bore_clear_of_a_keyhole_creases_sliver_carves_at_the_closed_form() {
    let body = keyhole_block_with(vec![bore(-0.75, 0.75, 0.1)]);
    validate_geometric(&body, tol()).expect("the bored keyhole block is tier-3 valid");
    let creases = rod_creases(&body);
    let vol0 = volume(&body);
    let out = fillet_edges(&body, &creases, 0.1, tol())
        .unwrap_or_else(|e| panic!("a clear bore carves, got {e}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier 3, {e:?}"));
    let dv = volume(&out.body) - vol0;
    let want = -2.0 * keyhole_cut(0.1);
    assert!((dv - want).abs() < 1e-12, "ΔV {dv} vs {want}");
}
