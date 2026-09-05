//! **FILLET-H5 review probes (lane r1).** What the unit's own rows do
//! not reach: the `Struts` host gate's two refusing arms, a hostless
//! rim beside its neighbours in one call, and the recourse sentence the
//! refusals carry.
//!
//! The body all three rows use is the **boss** — a cylinder of radius 1
//! and height 1 whose flat top runs in to radius 0.5, where a
//! hemispherical dome rises to the pole — after
//! `merge_coplanar_faces`, which is the repair the unit is about. It
//! carries THREE closed rims of three different shapes at once, which
//! is why it is the fixture:
//!
//! - its BASE rim `(1, 0)` is the unit's own shape (one plane host,
//!   the rim in that host's outer cycle, no rings) and CARVES;
//! - its TOP OUTER rim `(1, 1)` is the same shape except that the
//!   merged flat top also carries a RING (the dome rim), so it routes
//!   to the same door and refuses there;
//! - its DOME rim `(0.5, 1)` is in that ring, so it is a LADDER rim
//!   (the unit's Phase 1 measured it refusing on a false ring
//!   clearance, filed separately).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Point2, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::FILLET3_ASSEMBLY_RECOURSE;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{revolved_about_y, rim_arcs_at};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// The boss: `(0,0) (1,0) (1,1) (0.5,1) [dome] (0,1.5)` revolved fully.
fn boss() -> Body<f64> {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let mut b = revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.5, 1.0), q),
            ProfileVertex::new(Point2::new(0.0, 1.5), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    b.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    b
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

fn is_plane(body: &Body<f64>, f: FaceKey) -> bool {
    matches!(
        body.get_surface(body.get_face(f).unwrap().surface).unwrap(),
        Surface::Plane { .. }
    )
}

fn plane_host(body: &Body<f64>, arcs: &[EdgeKey]) -> FaceKey {
    let mut out: Vec<FaceKey> = Vec::new();
    for &a in arcs {
        let (fa, fb) = faces_of(body, a);
        for f in [fa, fb] {
            if is_plane(body, f) && !out.contains(&f) {
                out.push(f);
            }
        }
    }
    assert_eq!(out.len(), 1, "one plane face hosts every arc");
    out[0]
}

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

/// **The boss's BASE rim is the unit's shape and carves** — the control
/// for the two rows below, so their refusals are not about the fixture.
#[test]
fn r1_the_bosss_base_rim_is_hostless_and_carves() {
    let body = boss();
    let arcs = rim_arcs_at(&body, 1.0, 0.0);
    assert_eq!(arcs.len(), 2, "the repaired base rim is two arcs");
    let host = plane_host(&body, &arcs);
    let fd = body.get_face(host).unwrap();
    assert!(fd.rings.is_empty(), "the base disc carries no ring");
    assert_eq!(
        loop_edges(&body, fd.outer).len(),
        2,
        "the base disc's outer cycle is exactly the rim"
    );
    let out = fillet_edges(&body, &arcs, 0.05, tol()).expect("the base rim carves");
    validate_geometric(&out.body, tol()).expect("tier-3 valid");
    assert_eq!(out.band_faces.len(), 1, "one band");
    assert_eq!(
        mass_properties(&out.body, tol()).unwrap().volume_pad,
        0.0,
        "closed-form faces only"
    );
}

/// **A hostless rim whose host face ALSO carries a ring refuses — and
/// the recourse it carries promises the carve it just refused.**
///
/// The boss's top outer rim `(1, 1)` is one plane host, the rim in that
/// host's own OUTER cycle, crossings trivalent — everything
/// `FILLET3_ASSEMBLY_RECOURSE`'s rewritten closed clause now names
/// ("whether each support face carries one arc of the rim or one face
/// carries every arc"). It refuses anyway, because the merged flat top
/// is an annulus and the `Struts` host gate requires a RING-FREE host.
/// The refusal is an `UnsupportedChain`, which carries that very
/// sentence.
#[test]
fn r1_a_hostless_rim_on_a_ringed_host_refuses_under_a_recourse_that_promises_it() {
    let body = boss();
    let arcs = rim_arcs_at(&body, 1.0, 1.0);
    assert_eq!(arcs.len(), 2, "the repaired top outer rim is two arcs");
    let host = plane_host(&body, &arcs);
    let fd = body.get_face(host).unwrap();
    // The shape the recourse's new clause names: ONE face carries EVERY
    // arc of the rim, in that face's own outer cycle.
    assert_eq!(
        loop_edges(&body, fd.outer).len(),
        2,
        "the merged flat top's outer cycle is exactly this rim"
    );
    assert!(
        arcs.iter().all(|a| loop_edges(&body, fd.outer).contains(a)),
        "the rim lies in the host's OUTER cycle"
    );
    assert_eq!(
        fd.rings.len(),
        1,
        "and that host also carries the dome ring"
    );

    match fillet_edges(&body, &arcs, 0.05, tol()).map_err(|e| e.error) {
        Err(BlendError::UnsupportedChain { detail, .. }) => {
            assert!(
                detail.contains("rings of its own"),
                "the hostless host gate's ring arm: {detail}"
            );
            assert!(
                FILLET3_ASSEMBLY_RECOURSE.contains("one face carries every arc"),
                "the recourse this refusal carries promises exactly this shape"
            );
        }
        other => panic!("expected the ring arm of the hostless host gate, got {other:?}"),
    }
}

/// **Two hostless rims of ONE body in one call** — the composition
/// `a_hostless_rim_composes_with_a_shared_wall_neighbour` does not
/// cover, since its two rims are one hostless and one seam-split.
/// Here the repaired lantern's neck `(1, 0)` and lip `(0.2, 1.2)` are
/// both hostless, on two different plane hosts, and share no wall.
#[test]
fn r1_two_hostless_rims_of_one_body_compose_in_one_call() {
    let source = {
        let mut b = sweep::test_support::lantern(tol());
        b.merge_coplanar_faces(tol()).expect("the caps repair");
        b
    };
    let mut both = rim_arcs_at(&source, 1.0, 0.0);
    both.extend(rim_arcs_at(&source, 0.2, 1.2));
    assert_eq!(both.len(), 4, "two hostless rims of two arcs each");
    let one_call =
        fillet_edges(&source, &both, 0.05, tol()).expect("two hostless rims carve in ONE call");
    validate_geometric(&one_call.body, tol()).expect("tier-3 valid");
    assert_eq!(one_call.band_faces.len(), 2, "one band per rim");
    let one = mass_properties(&one_call.body, tol()).unwrap();
    assert_eq!(one.volume_pad, 0.0, "closed-form faces only");

    for order in [[(1.0, 0.0), (0.2, 1.2)], [(0.2, 1.2), (1.0, 0.0)]] {
        let mut body = {
            let mut b = sweep::test_support::lantern(tol());
            b.merge_coplanar_faces(tol()).expect("the caps repair");
            b
        };
        for (rr, ry) in order {
            let arcs = rim_arcs_at(&body, rr, ry);
            body = fillet_edges(&body, &arcs, 0.05, tol())
                .unwrap_or_else(|e| panic!("the ({rr}, {ry}) rim carves alone, got {e:?}"))
                .body;
        }
        let seq = mass_properties(&body, tol()).unwrap();
        assert_eq!(
            one.volume.to_bits(),
            seq.volume.to_bits(),
            "the one-call result IS the sequential composition, bit for bit \
             (order {order:?})"
        );
    }
}

/// **The mixed LADDER/ANNULUS shared-support arm's "no reachable
/// fixture today" premise, re-measured.** `shared_support_gate`'s doc
/// says the shape it refuses "needs a plane face carrying both a pip
/// RING and a revolution-wall cycle, whose only public construction is
/// a boolean of a ball against a revolve — refused at the boolean
/// operand gate ... before any fillet request exists."
///
/// The boss's merged flat top IS such a face, built by revolve plus
/// `merge_coplanar_faces` and no boolean at all: its RING is the dome
/// rim (a ladder rim) and its OUTER CYCLE is the top rim (now, after
/// this unit, an annulus rim). So the doc's exclusivity clause is
/// stale. The gate still does not fire — the `Struts` host gate's ring
/// arm refuses first — which is what this row pins, so a change that
/// relaxes that arm reds here rather than reaching an arm with no
/// fixture.
#[test]
fn r1_the_mixed_shared_support_arm_is_not_what_refuses_the_bosss_two_rims() {
    let body = boss();
    let mut both = rim_arcs_at(&body, 1.0, 1.0);
    both.extend(rim_arcs_at(&body, 0.5, 1.0));
    assert_eq!(both.len(), 4, "the top rim and the dome rim, two arcs each");
    // Both rims rest on the SAME merged flat top.
    assert_eq!(
        plane_host(&body, &both[..2]),
        plane_host(&body, &both[2..]),
        "one plane face carries the ladder ring and the annulus cycle"
    );
    match fillet_edges(&body, &both, 0.05, tol()).map_err(|e| e.error) {
        Err(BlendError::UnsupportedChain { detail, .. }) => assert!(
            detail.contains("rings of its own"),
            "the hostless host gate answers before the shared-support gate: {detail}"
        ),
        other => panic!("expected the hostless host gate's ring arm, got {other:?}"),
    }
}
