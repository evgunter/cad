//! **FILLET-H5 Phase 1: what a plane-hosted closed rim actually is.**
//!
//! The unit `repaired-pole-rim-serves-no-closed-door` is about a closed
//! rim whose arcs are hosted by ONE plane face in that face's OWN OUTER
//! CYCLE — one host, no ring, and crossings the coplanar-merge repair
//! left TRIVALENT (two rim arcs plus the mate's seam, the host's seam
//! having been merged away). These rows pin what that shape is, where
//! it comes from, and what the doors say about it, so the measurement
//! the spec's Phase 1 asks for is a gate rather than a transcript.
//!
//! Two of them state NEGATIVE structural facts and are the ones that
//! earn their place: a full revolve of a pole-touching profile splits
//! EVERY wall, not only the walls that touch the axis, so the shape has
//! no native revolve instance
//! (`work/fillet/plane-hosted-rim-has-no-native-instance.md`); and a
//! repaired ANNULAR plane hosts its rim in a RING rather than in its
//! outer cycle, which routes it to the ladder and into a ring-clearance
//! refusal of a nested trim circle
//! (`work/fillet/ring-clearance-refuses-a-nested-trim-circle.md`).
//!
//! The row that FLIPS when the unit lands is
//! `the_plane_hosted_rim_refuses_at_the_ladders_ring_gate`: every
//! fixture there is the shape, and every one of them refuses today.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Point2, Sign, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{lantern, revolved_about_y, rim_arcs_at, waisted};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, VertexKey, mass_properties};

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, tol())
        .expect("mass properties compute")
        .volume
}

/// The two faces an edge separates.
fn faces_of(body: &Body<f64>, e: EdgeKey) -> (FaceKey, FaceKey) {
    let ed = body.get_edge(e).unwrap();
    let f = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    (f(ed.he_plus), f(ed.he_minus))
}

fn is_plane(body: &Body<f64>, f: FaceKey) -> bool {
    matches!(
        body.get_surface(body.get_face(f).unwrap().surface).unwrap(),
        Surface::Plane { .. }
    )
}

/// The distinct planar supports of a rim's arcs.
fn planar_supports(body: &Body<f64>, arcs: &[EdgeKey]) -> Vec<FaceKey> {
    let mut out = Vec::new();
    for &a in arcs {
        let (fa, fb) = faces_of(body, a);
        for f in [fa, fb] {
            if is_plane(body, f) && !out.contains(&f) {
                out.push(f);
            }
        }
    }
    out
}

/// A loop's edges in cycle order.
fn loop_edges(body: &Body<f64>, lp: topo::LoopKey) -> Vec<EdgeKey> {
    let LoopBoundary::Cycle { first } = body.get_loop(lp).unwrap().boundary else {
        return Vec::new();
    };
    body.loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| body.get_half_edge(he).unwrap().edge)
        .collect()
}

/// The rim's own vertices, in first-seen order.
fn rim_vertices(body: &Body<f64>, arcs: &[EdgeKey]) -> Vec<VertexKey> {
    let mut vs = Vec::new();
    for &a in arcs {
        let ed = body.get_edge(a).unwrap();
        for he in [ed.he_plus, ed.he_minus] {
            let s = body.get_half_edge(he).unwrap().start;
            if !vs.contains(&s) {
                vs.push(s);
            }
        }
    }
    vs
}

fn valence(body: &Body<f64>, v: VertexKey) -> Vec<EdgeKey> {
    let he = body.get_vertex(v).unwrap().emanating.unwrap();
    let mut out: Vec<EdgeKey> = body
        .vertex_orbit(he)
        .unwrap()
        .into_iter()
        .map(|h| body.get_half_edge(h).unwrap().edge)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Whether an edge is a chart seam: the same SURFACE on both sides.
fn co_surface(body: &Body<f64>, e: EdgeKey) -> bool {
    let (fa, fb) = faces_of(body, e);
    body.get_face(fa).unwrap().surface == body.get_face(fb).unwrap().surface
}

// ------------------------------------------------------------------
// The fixtures.
// ------------------------------------------------------------------

/// A pole-touching hemisphere of radius 1 on a flat base disc: the
/// simplest plane×sphere instance, one segment per support.
fn hemisphere_on_flat_base() -> Body<f64> {
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(
                Point2::new(1.0, 0.0),
                (core::f64::consts::FRAC_PI_2 / 4.0).tan(),
            ),
            ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// **The bowl**: a flat floor at `y = 1` out to radius 1, then a lip
/// rising to `(1.5, 1.5)` and down the outside to the base. Its floor
/// rim is an INSIDE corner, so it is the plane-hosted shape's CONCAVE
/// instance — the material side the convex fixtures do not reach.
fn bowl() -> Body<f64> {
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.5, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.5, 1.5), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// **The boss**: a cylinder of radius 1 and height 1 whose flat top
/// runs in to radius 0.5, where a hemispherical dome of radius 0.5
/// rises to the pole. `up` false is its dimple twin — the same pocket
/// dipping to `(0, 0.5)` instead.
fn boss(up: bool) -> Body<f64> {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.5, 1.0), if up { q } else { -q }),
            ProfileVertex::new(Point2::new(0.0, if up { 1.5 } else { 0.5 }), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn repaired(mut body: Body<f64>) -> Body<f64> {
    body.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    body
}

// ------------------------------------------------------------------
// The rows.
// ------------------------------------------------------------------

/// **The shape, on every fixture that carries it.** One planar host,
/// the rim in that face's own OUTER cycle and in no ring, and every
/// crossing TRIVALENT — two rim arcs plus exactly one co-surface mate
/// seam, the host's seam having been merged away by the repair. All of
/// them refuse at the ladder's ring gate, which is the whole defect.
///
/// **This is the row the unit flips.** When the hostless-crossing
/// annulus arm lands, every fixture here carves instead, and the two
/// structural halves above still hold unchanged.
#[test]
fn the_plane_hosted_rim_refuses_at_the_ladders_ring_gate() {
    let fixtures: Vec<(&str, Body<f64>, f64, f64)> = vec![
        ("lantern neck", repaired(lantern(tol())), 1.0, 0.0),
        ("lantern lip", repaired(lantern(tol())), 0.2, 1.2),
        (
            "hemisphere equator",
            repaired(hemisphere_on_flat_base()),
            1.0,
            0.0,
        ),
        ("waisted base", repaired(waisted(tol())), 1.0, 0.0),
        ("waisted top", repaired(waisted(tol())), 1.0, 1.0),
        ("bowl floor", repaired(bowl()), 1.0, 1.0),
    ];
    for (name, body, r, y) in &fixtures {
        let arcs = rim_arcs_at(body, *r, *y);
        assert_eq!(arcs.len(), 2, "{name}: the repair leaves the rim two arcs");

        let hosts = planar_supports(body, &arcs);
        assert_eq!(hosts.len(), 1, "{name}: ONE plane face hosts every arc");
        let host = hosts[0];
        let fd = body.get_face(host).unwrap();
        assert!(
            fd.rings.is_empty(),
            "{name}: the plane host carries no ring of its own"
        );
        let outer = loop_edges(body, fd.outer);
        assert_eq!(
            outer.len(),
            2,
            "{name}: the host's outer cycle is exactly the rim"
        );
        assert!(
            arcs.iter().all(|a| outer.contains(a)),
            "{name}: the rim lies in the host's OUTER cycle, not in a ring"
        );

        for v in rim_vertices(body, &arcs) {
            let inc = valence(body, v);
            assert_eq!(inc.len(), 3, "{name}: a crossing is trivalent");
            let seams: Vec<EdgeKey> = inc.iter().copied().filter(|e| !arcs.contains(e)).collect();
            assert_eq!(seams.len(), 1, "{name}: one seam meets a crossing");
            assert!(
                co_surface(body, seams[0]),
                "{name}: that seam is a co-surface chart meridian"
            );
            assert!(
                !is_plane(body, faces_of(body, seams[0]).0),
                "{name}: the surviving seam is the MATE's, not the host's"
            );
        }

        match fillet_edges(body, &arcs, 0.05, tol()).map_err(|e| e.error) {
            Err(BlendError::UnsupportedChain { detail, .. }) => assert_eq!(
                detail, "a closed chain is not a ring of its plane support",
                "{name}: the ladder's ring gate refuses"
            ),
            other => panic!("{name}: expected the ring gate, got {other:?}"),
        }
    }
}

/// **The seam-split annulus could not take it either, and the gate that
/// stops it is the HALF-BAND one.** `resolve_seam_split_rim` requires
/// every support face to carry exactly ONE of the chain's arcs; the
/// repaired plane carries both, which is the structural fact that
/// refusal reads. Pinned here rather than by relaxing the ladder
/// discriminant, so the fact is measured on the body and not on a
/// scaffolded build.
#[test]
fn the_plane_host_carries_both_arcs_which_is_the_half_band_gates_own_fact() {
    let body = repaired(lantern(tol()));
    let arcs = rim_arcs_at(&body, 1.0, 0.0);
    let host = planar_supports(&body, &arcs)[0];
    let carried: Vec<EdgeKey> = loop_edges(&body, body.get_face(host).unwrap().outer)
        .into_iter()
        .filter(|e| arcs.contains(e))
        .collect();
    assert_eq!(
        carried.len(),
        2,
        "the repaired host carries BOTH rim arcs, so the half-band \
         discipline the seam-split resolver needs does not hold of it"
    );
    // The MATE side does hold it: one arc per face, which is why only
    // the host side needs a new way to reach its foot.
    for &a in &arcs {
        let (fa, fb) = faces_of(&body, a);
        let mate = if is_plane(&body, fa) { fb } else { fa };
        let mate_arcs: Vec<EdgeKey> = loop_edges(&body, body.get_face(mate).unwrap().outer)
            .into_iter()
            .filter(|e| arcs.contains(e))
            .collect();
        assert_eq!(
            mate_arcs,
            vec![a],
            "a mate face carries exactly its own arc"
        );
    }
}

/// **Both material sides are reachable in this shape.** The bowl's
/// floor rim is an inside corner and its carve ADDS material; the
/// lantern's neck rim is an outside one and its carve removes it. Both
/// measured on the RAW (unrepaired) bodies, where the seam-split
/// annulus already serves them — so the sides are a property of the
/// geometry and not of the door, and the repaired twins above inherit
/// them.
#[test]
fn the_plane_hosted_shape_reaches_either_material_side() {
    for (name, body, r, y, adds) in [
        ("bowl floor", bowl(), 1.0, 1.0, true),
        ("lantern neck", lantern(tol()), 1.0, 0.0, false),
    ] {
        let arcs = rim_arcs_at(&body, r, y);
        let before = volume(&body);
        let out = fillet_edges(&body, &arcs, 0.05, tol())
            .unwrap_or_else(|e| panic!("{name} carves before the repair, got {e:?}"));
        let delta = volume(&out.body) - before;
        assert_eq!(
            delta > 0.0,
            adds,
            "{name}: a concave rim's band adds material and a convex one's removes it \
             (delta {delta})"
        );
    }
}

/// **The shape has no NATIVE revolve instance.** A full revolve of a
/// pole-touching profile is the wire case, which sweeps every segment
/// of the loop in two π-bands — the split is a property of the BODY,
/// not of the segment, so a plane annulus that does not touch the axis
/// is minted as two half-annuli just the same. The spec's own "dome on
/// a wider flat top" is therefore the ordinary seam-split annulus:
/// TWO planar supports, valence-4 crossings, and it carves today.
#[test]
fn a_pole_touching_revolve_splits_the_walls_that_do_not_touch_the_axis_too() {
    for up in [true, false] {
        let name = if up { "boss" } else { "dimple" };
        let body = boss(up);
        assert_eq!(
            body.faces().count(),
            8,
            "{name}: four profile segments, every one of them split in two"
        );
        let arcs = rim_arcs_at(&body, 0.5, 1.0);
        assert_eq!(arcs.len(), 2, "{name}: the rim is two arcs");
        assert_eq!(
            planar_supports(&body, &arcs).len(),
            2,
            "{name}: the flat top is TWO half-annuli, not one face"
        );
        for v in rim_vertices(&body, &arcs) {
            assert_eq!(
                valence(&body, v).len(),
                4,
                "{name}: a crossing carries a co-surface seam per SIDE"
            );
        }
        fillet_edges(&body, &arcs, 0.1, tol())
            .unwrap_or_else(|e| panic!("{name} is the seam-split annulus and carves, got {e:?}"));
    }
}

/// **Repairing the boss does not produce the shape either — it produces
/// a ring-hosted ladder rim that refuses on ring clearance.** The
/// merged flat top is an ANNULUS, so its rim lands in a RING of it and
/// routes to the ladder; the ladder's own gates pass, and the exact
/// outer-boundary check then applies its EXTERNAL-separation form to a
/// boundary circle that CONTAINS the trim circle. The margin is
/// `−(trim radius + boundary radius)` on concentric circles — a
/// refusal of a carve that is geometrically fine
/// (`work/fillet/ring-clearance-refuses-a-nested-trim-circle.md`).
#[test]
fn a_repaired_boss_is_ring_hosted_and_refuses_on_a_nested_trim_circle() {
    for up in [true, false] {
        let name = if up { "boss" } else { "dimple" };
        let body = repaired(boss(up));
        let arcs = rim_arcs_at(&body, 0.5, 1.0);
        let hosts = planar_supports(&body, &arcs);
        assert_eq!(hosts.len(), 1, "{name}: the repair leaves one plane host");
        let fd = body.get_face(hosts[0]).unwrap();
        assert_eq!(
            fd.rings.len(),
            1,
            "{name}: the merged flat top is an annulus"
        );
        assert!(
            arcs.iter()
                .all(|a| loop_edges(&body, fd.rings[0]).contains(a)),
            "{name}: the rim lies in that RING, not in the outer cycle — a ladder rim"
        );
        match fillet_edges(&body, &arcs, 0.1, tol()).map_err(|e| e.error) {
            Err(BlendError::RingClearance { margin, .. }) => {
                assert_eq!(margin.predicate, "fillet3_ring_clearance");
                assert_eq!(margin.sign, Sign::Negative);
                // Concentric circles: the external form reads
                // `0 − trim − boundary`, while the containment margin
                // `boundary − trim` is comfortably positive.
                let read = margin.value().expect("a definite reading");
                assert!(
                    read < -1.0,
                    "{name}: the reading is minus the SUM of the two radii, got {read}"
                );
            }
            other => panic!("{name}: expected a ring-clearance refusal, got {other:?}"),
        }
    }
}
