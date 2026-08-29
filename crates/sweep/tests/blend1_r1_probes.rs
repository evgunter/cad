//! BLEND-1 review probes (r1) — unique-signal attacks on PR #1222's
//! load-bearing claims, beside the suite the PR ships. Each probe is
//! designed to be evidence the PR's own rows cannot already be.
//!
//! - **The recourse's reach** (claim 8): the tag fires from the
//!   battery's corner classifier, which runs before any convexity door,
//!   so a chain stopping at a CONCAVE rim's seam vertex meets it too.
//!   That was the review's MAJOR while the sentence promised the carve
//!   unconditionally; it is now the premise the conditioned sentence
//!   rests on, measured here rather than argued.
//! - **An independent closed form** (claim 3): the suite's volume
//!   oracle is a differential against the one-edge twin, which could
//!   share a defect with the door under test. The lip rim's removal is
//!   re-derived here by hand — Pappus on the kite-minus-sector region —
//!   with no kernel quantity but the measured volumes.
//! - **The routing boundary** (claim 2): what a multi-link closed chain
//!   that is NOT a seam-split rim meets, now that the two refusals the
//!   issue measured became the routing test.
//! - **Resolution violations** (claim 4): a planted ring on a half-band
//!   support; a request that adds a seam meridian to the rim's arcs.
//!
//! Authored in the review lane and ADOPTED into the unit at its fix
//! pass, so the findings below are re-taken by the suite on every run
//! rather than living only in a review thread.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::SQRT_2;

use geom_core::{Band, Point2, Point3, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::fillet::build::fillet_edges;
use sweep::fillet::{CornerConfig, FILLET3_SEAM_VERTEX_RECOURSE, FilletError};
use sweep::test_support::{cube, revolved_about_y, rim_arcs_at};
use topo::{Body, EdgeKey, FaceKey, SurfaceKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

// ------------------------------------------------------------------
// Fixtures (the PR suite's lantern, restated so these probes stand
// alone; same profile, same derivations).
// ------------------------------------------------------------------

const SHOULDER: (f64, f64) = (0.8, 0.6);
const TOP: f64 = 1.2;
const LIP_R: f64 = 0.2;
const BORE: f64 = 0.1;

fn lantern() -> Body<f64> {
    let bulge = (0.6f64.asin() / 4.0).tan();
    revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, bulge),
            v(SHOULDER.0, SHOULDER.1, 0.0),
            v(LIP_R, TOP, 0.0),
            v(0.0, TOP, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn bored_lantern() -> Body<f64> {
    let bulge = (0.6f64.asin() / 4.0).tan();
    revolved_about_y(
        vec![
            v(BORE, 0.0, 0.0),
            v(1.0, 0.0, bulge),
            v(SHOULDER.0, SHOULDER.1, 0.0),
            v(LIP_R, TOP, 0.0),
            v(BORE, TOP, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The waisted pole-touching revolve whose waist rim is CONCAVE and
/// seam-split (the PR suite's own concave fixture).
fn waisted() -> Body<f64> {
    revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            v(0.5, 0.5, 0.0),
            v(1.0, 1.0, 0.0),
            v(0.0, 1.0, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn surface_of(body: &Body<f64>, f: FaceKey) -> SurfaceKey {
    body.get_face(f).unwrap().surface
}

fn faces_of(body: &Body<f64>, e: EdgeKey) -> (FaceKey, FaceKey) {
    let ed = body.get_edge(e).unwrap();
    let f = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    (f(ed.he_plus), f(ed.he_minus))
}

fn volume(body: &Body<f64>) -> f64 {
    let props = mass_properties(body, tol()).expect("mass properties compute");
    assert_eq!(props.volume_pad, 0.0, "closed-form faces only");
    props.volume
}

// ------------------------------------------------------------------
// P1 — the recourse's promise, measured where the tag fires (claim 8).
// ------------------------------------------------------------------

/// **The tag's firing rule never reads convexity — which is why the
/// recourse's carve half is conditioned.**
///
/// This row began as the r1 review's MAJOR: the rewritten recourse
/// promised the carve unconditionally, and the corner classifier runs
/// BEFORE any convexity door, so a chain stopping at a CONCAVE rim's
/// seam vertex was shown a promise its own whole-rim request then
/// refused.
///
/// It is now the MECHANISM half of that finding, kept because it is
/// what the conditioning rests on: the tag is incidence-only, so the
/// site set it fires over spans both material sides, and any sentence
/// it carries must be true across that whole set. The composed
/// promise-and-answer pin lives in the r2 suite
/// (`the_seam_vertex_recourse_is_true_at_every_site_the_tag_fires`);
/// this row pins the premise it argues from.
///
/// Red if the tag ever learns convexity — at which point the sentence
/// may drop its hedge, and should.
#[test]
fn p1_the_seam_vertex_tag_fires_without_reading_convexity() {
    // A CONCAVE seam-split rim, and a CONVEX one on the same body.
    let body = waisted();
    for (name, rim_r, rim_y) in [
        ("the concave waist", 0.5, 0.5),
        ("the convex base", 1.0, 0.0),
    ] {
        let arcs = rim_arcs_at(&body, rim_r, rim_y);
        assert_eq!(arcs.len(), 2, "{name} is seam-split");
        match fillet_edges(&body, &arcs[..1], 0.05, band(), tol()) {
            Err(FilletError::FilletCornerUnsupported { corner, .. }) => assert!(
                matches!(corner, CornerConfig::SeamVertex),
                "{name}: the classifier reads incidence and tags the seam vertex, \
                 got {corner}"
            ),
            other => panic!("{name}: one arc stops at a seam vertex, got {other:?}"),
        }
    }
    // So the sentence the tag carries has to hold on both sides, and it
    // does that by conditioning its carve half rather than by promising
    // one door for two configurations.
    assert!(
        FILLET3_SEAM_VERTEX_RECOURSE.contains("CONVEX"),
        "the carve half names the side the door serves: {FILLET3_SEAM_VERTEX_RECOURSE}"
    );
    assert!(
        !FILLET3_SEAM_VERTEX_RECOURSE.contains("which the closed-rim band carves as one"),
        "the unconditional promise is gone: {FILLET3_SEAM_VERTEX_RECOURSE}"
    );
}

// ------------------------------------------------------------------
// P2 — an independent closed form for one removal (claim 3).
// ------------------------------------------------------------------

/// **The lip rim's removal, from Pappus and nothing of the kernel.**
///
/// The suite's volume oracle compares the seam-split carve against the
/// one-edge twin's — a sibling that shares the arm, the surgery's
/// trim derivations and `mass_properties`. This row re-derives the LIP
/// rim's removed volume by hand: the region between the sharp corner
/// and the fillet arc is a kite minus a circular sector, its centroid
/// is closed-form, and Pappus turns both into the removed volume. Only
/// the two measured volumes come from the kernel.
#[test]
fn p2_the_lip_rim_removal_matches_a_hand_pappus_closed_form() {
    let r = 0.05;
    let source = lantern();
    let arcs = rim_arcs_at(&source, LIP_R, TOP);
    assert_eq!(arcs.len(), 2, "the lip rim is seam-split");
    let out = fillet_edges(&source, &arcs, r, band(), tol())
        .unwrap_or_else(|e| panic!("the lip fillets whole, got {e:?}"));
    let removed = volume(&source) - volume(&out.body);

    // The hand form. Corner K, ball centre c (r below the top plane,
    // r inside the 45° cone through the shoulder), feet on each
    // support.
    let k = (LIP_R, TOP);
    let n = (1.0 / SQRT_2, 1.0 / SQRT_2);
    let cy = TOP - r;
    let cx = SHOULDER.0 + SHOULDER.1 - cy - r * SQRT_2;
    let f_plane = (cx, TOP);
    let f_cone = (cx + r * n.0, cy + r * n.1);
    // Tangent lengths from K are equal; check both to 1e-15 so the
    // form's own premise is pinned, not assumed.
    let len = |a: (f64, f64), b: (f64, f64)| ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt();
    let t_len = len(k, f_plane);
    assert!(
        (len(k, f_cone) - t_len).abs() < 1e-15,
        "equal tangent lengths"
    );
    // The sector angle between the two foot radials.
    let d1 = ((f_plane.0 - cx) / r, (f_plane.1 - cy) / r);
    let d2 = ((f_cone.0 - cx) / r, (f_cone.1 - cy) / r);
    let theta = (d1.0 * d2.0 + d1.1 * d2.1).acos();
    let kite_area = t_len * r;
    let sector_area = 0.5 * r * r * theta;
    let area = kite_area - sector_area;
    // Centroids: the kite is two equal-area triangles; the sector's
    // centroid sits `2·r·sin(θ/2)/(3·θ/2)` from c along the bisector.
    let x_kite = ((k.0 + f_plane.0 + cx) / 3.0 + (k.0 + f_cone.0 + cx) / 3.0) / 2.0;
    let alpha = theta / 2.0;
    let d = 2.0 * r * alpha.sin() / (3.0 * alpha);
    let bis_len = ((d1.0 + d2.0).powi(2) + (d1.1 + d2.1).powi(2)).sqrt();
    let x_sector = cx + d * (d1.0 + d2.0) / bis_len;
    let moment = x_kite * kite_area - x_sector * sector_area;
    let hand = core::f64::consts::TAU * moment;
    assert!(area > 0.0 && hand > 0.0, "a real removal");
    assert!(
        (removed - hand).abs() < 1e-12,
        "the carve removes {removed}, the hand form says {hand}"
    );
}

// ------------------------------------------------------------------
// P3 — the routing boundary (claim 2).
// ------------------------------------------------------------------

/// **A closed cycle that is no rim never becomes a closed CHAIN.** The
/// Petrie hexagon of a cube is a closed cycle of six links on six
/// different planes — the "links on distinct planes" shape the routing
/// change must not admit. Measured: it is stopped one gate EARLIER
/// than expected — chain assembly itself refuses `ChainNotG1` at the
/// first sharp corner, because a closed chain is a tangent-continuous
/// loop by construction. So the seam-split resolver's own checks are
/// only ever asked about G1-closed, torus-armed chains; the two
/// retired refusals were never the outer fence.
#[test]
fn p3_a_petrie_hexagon_cycle_never_assembles_into_a_closed_chain() {
    let body = cube(1.0, tol());
    let cycle = [
        ((0.0, 0.0, 0.0), (1.0, 0.0, 0.0)),
        ((1.0, 0.0, 0.0), (1.0, 1.0, 0.0)),
        ((1.0, 1.0, 0.0), (1.0, 1.0, 1.0)),
        ((1.0, 1.0, 1.0), (0.0, 1.0, 1.0)),
        ((0.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
        ((0.0, 0.0, 1.0), (0.0, 0.0, 0.0)),
    ];
    let point_of = |vk| {
        let p: Point3<f64> = *body.get_point(body.get_vertex(vk).unwrap().point).unwrap();
        p
    };
    let ends_of = |e: EdgeKey| {
        let ed = body.get_edge(e).unwrap();
        let s = body.get_half_edge(ed.he_plus).unwrap().start;
        let t = body.half_edge_end(ed.he_plus).unwrap();
        (point_of(s), point_of(t))
    };
    let matches_pt = |p: Point3<f64>, q: (f64, f64, f64)| {
        (p.x - q.0).abs() < 1e-9 && (p.y - q.1).abs() < 1e-9 && (p.z - q.2).abs() < 1e-9
    };
    let edges: Vec<EdgeKey> = cycle
        .iter()
        .map(|&(a, b)| {
            body.edges()
                .map(|(k, _)| k)
                .find(|&k| {
                    let (s, t) = ends_of(k);
                    (matches_pt(s, a) && matches_pt(t, b)) || (matches_pt(s, b) && matches_pt(t, a))
                })
                .expect("a cube edge between two named corners")
        })
        .collect();
    assert_eq!(edges.len(), 6, "the Petrie hexagon has six edges");
    match fillet_edges(&body, &edges, 0.1, band(), tol()) {
        Err(FilletError::ChainNotG1 { .. }) => {}
        other => panic!("a sharp-cornered hexagon cycle refuses at assembly, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// P4 — a planted ring on a half-band support (claim 4).
// ------------------------------------------------------------------

/// **The REPAIRED lantern's neck rim is outside BOTH closed-rim
/// doors — measured, because it is the body a consumer actually
/// holds.** A raw pole-touching revolve is `NonMaximalFaces` at every
/// boolean door, so any consumer who booleans (the tour's own lily
/// flow) repairs first with `merge_coplanar_faces`, which merges each
/// cap's two half-disks into ONE face. The neck rim is then two arcs
/// whose planar support is one face — so `resolve_rim`'s host-side
/// discriminant routes it to the LADDER, whose ring gate refuses. The
/// door this PR opens serves the UNREPAIRED shape only; a
/// plane-involving rim loses it at the repair the boolean lane
/// requires.
///
/// Also measured here: one arc of the repaired rim no longer registers
/// a `SeamVertex` (the cap's seam is gone, so the vertex is trivalent)
/// — so at least the carve-promising recourse is not shown for a
/// request this door can then not serve.
#[test]
fn p4_the_repaired_lantern_neck_rim_is_outside_both_closed_rim_doors() {
    let mut source = lantern();
    source
        .merge_coplanar_faces(tol())
        .expect("the pole-split caps repair (#1031's pole half)");
    let arcs = rim_arcs_at(&source, 1.0, 0.0);
    assert_eq!(arcs.len(), 2, "the neck rim is still two arcs");
    let (a0, b0) = faces_of(&source, arcs[0]);
    let (a1, b1) = faces_of(&source, arcs[1]);
    let planes: Vec<FaceKey> = [a0, b0, a1, b1]
        .into_iter()
        .filter(|f| {
            matches!(
                source
                    .get_surface(source.get_face(*f).unwrap().surface)
                    .unwrap(),
                geom::Surface::Plane { .. }
            )
        })
        .collect();
    assert_eq!(
        planes[0], planes[1],
        "after the repair one plane face hosts both arcs"
    );
    match fillet_edges(&source, &arcs, 0.05, band(), tol()) {
        Err(FilletError::UnsupportedChain { detail, .. }) => assert!(
            detail.contains("ring"),
            "the repaired rim routes to the ladder and its ring gate refuses: {detail}"
        ),
        other => panic!("the repaired neck rim refuses typed, got {other:?}"),
    }
    match fillet_edges(&source, &arcs[..1], 0.05, band(), tol()) {
        Err(FilletError::FilletCornerUnsupported { corner, .. }) => {
            assert!(
                !matches!(corner, CornerConfig::SeamVertex),
                "a trivalent repaired-rim end is not a seam vertex: {corner}"
            );
        }
        other => panic!("one repaired arc refuses at a corner door, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// P5 — a request wider than the rim (claim 4's other direction).
// ------------------------------------------------------------------

/// **Adding a seam meridian to the whole-rim request refuses at the
/// battery**, before any door: the seam is a co-surface tangency at
/// margin exactly zero (the wall-6 shape). So neither a subset (the
/// `SeamVertex` rows) nor a superset of the rim's arcs reaches the
/// carve — only the rim itself.
#[test]
fn p5_the_rim_arcs_plus_a_seam_meridian_refuse_at_the_battery() {
    let body = lantern();
    let mut req = rim_arcs_at(&body, SHOULDER.0, SHOULDER.1);
    assert_eq!(req.len(), 2);
    // A co-surface edge: same surface on both sides.
    let seam = body
        .edges()
        .map(|(k, _)| k)
        .find(|&k| {
            let (a, b) = faces_of(&body, k);
            a != b && surface_of(&body, a) == surface_of(&body, b)
        })
        .expect("a full revolve of a pole-touching profile has seam meridians");
    req.push(seam);
    match fillet_edges(&body, &req, 0.05, band(), tol()) {
        Err(FilletError::TangentialEdge { margin, .. }) => {
            assert_eq!(margin, 0.0, "a co-surface seam is tangential exactly");
        }
        other => panic!("a request carrying a seam meridian refuses tangential, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// P6 — the one-edge differential's raw bits (claim 1). Run this row at
// the merge base and at the head and diff the printed lines: the claim
// "the one-edge sequence is unchanged move for move" predicts
// bit-identical output.
// ------------------------------------------------------------------

#[test]
fn p6_one_edge_rims_bit_dump_for_the_merge_base_differential() {
    let source = bored_lantern();
    let rims = [
        ("neck", 1.0, 0.0),
        ("shoulder", SHOULDER.0, SHOULDER.1),
        ("lip", LIP_R, TOP),
    ];
    for (name, r, y) in rims {
        let arcs = rim_arcs_at(&source, r, y);
        assert_eq!(arcs.len(), 1, "{name} is one closed edge on the twin");
        let out = fillet_edges(&source, &arcs, 0.05, band(), tol())
            .unwrap_or_else(|e| panic!("{name} carves on the twin, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{name} tier-3 valid, got {e:?}"));
        let vol = volume(&out.body);
        let naming = out.naming.as_ref().expect("naming recorded");
        let counts = (
            out.body.vertices().count(),
            out.body.edges().count(),
            out.body.faces().count(),
        );
        println!(
            "P6 {name} vol={:016x} census={counts:?} bands={} feet={} msplits={} \
             mremnants={} trims={} slits={} dead_e={} dead_v={}",
            vol.to_bits(),
            naming.bands.len(),
            naming.rim_feet.len(),
            naming.meridian_splits.len(),
            naming.meridian_remnants.len(),
            naming.rim_trims.len(),
            naming.slits.len(),
            naming.dead.edges.len(),
            naming.dead.vertices.len(),
        );
        println!("P6 {name} feet_rows={:?}", naming.rim_feet);
        println!("P6 {name} trim_rows={:?}", naming.rim_trims);
        println!("P6 {name} slit_rows={:?}", naming.slits);
        println!(
            "P6 {name} dead={:?}/{:?}",
            naming.dead.edges, naming.dead.vertices
        );
    }
}
