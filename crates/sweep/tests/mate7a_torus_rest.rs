//! MATE-7a — torus pairs through the boolean, on torus geometry a
//! producer actually mints (`sweep::tube_along_arc`, solid and hollow).
//!
//! 1. **The operand gate lets a torus through.** The torus is on the
//!    KIND roster, so no torus pair is the gate's question; the peg
//!    seated in its socket builds through the join, and a coincident
//!    pair is one body.
//! 2. **A G1 tube chain is one bent tube**: the two walls both end on
//!    their shared meridian and glue as a junction, an edge of one
//!    segment that ends on that rim tangent to the other's wall has that
//!    end's root factored, and the seam is described where it rests. The
//!    union builds; ∖ and ∩ stop at the fallback's extent.
//! 3. **∖ and ∩ meet the doors ∪ does.** The torus is on the revert
//!    roster, so each fixture under a subtract (both orders) or an
//!    intersect answers at the door its union meets; the peg seated in
//!    the socket shares no interior with it, so its ∩ is empty and each
//!    difference its minuend whole, and a coincident pair less itself
//!    is nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::revert_ops::subtract_both_orders_and_intersect;
use topo::construction::Construct;

use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::{finished, tube_frame};
use sweep::{TubeWindow, tube_along_arc, tube_along_arc_hollow};
use topo::query;
use topo::{AtRestBody, Body, BooleanError, BooleanResult, FaceKey};

const TUBE: f64 = 0.06;
/// The lower stem arc's ring — the lily's own numbers, so the fixtures
/// are the demand signal's geometry rather than a rounder one.
const RING: f64 = 5.0;
const TURN: f64 = 22.0;

fn axis() -> Vec3<f64> {
    Vec3::new(0.0, -1.0, 0.0)
}

fn window(deg: f64) -> TubeWindow<f64> {
    TubeWindow::Arc {
        t0: 0.0,
        t1: deg.to_radians(),
    }
}

/// Segment A of the chain: a tube along the lily's lower stem arc,
/// starting at the origin heading `+z`. `None` at ε = 1e-12, where its
/// torus wall is past limb 2's reach (the kernel's limit,
/// `work/shell/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`),
/// as for segment B and the socket.
fn segment_a() -> Option<AtRestBody<f64>> {
    Some(finished(
        "segment A",
        segment_a_tube()?.body,
        Tol::witness(),
    ))
}

/// Segment A as the tube door answers it, caps and joints and all.
fn segment_a_tube() -> Option<sweep::Revolved<f64>> {
    let body = tube_along_arc(
        tube_frame(
            Point3::new(-RING, 0.0, 0.0),
            axis(),
            Vec3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        ),
        RING,
        window(TURN),
        TUBE,
        Tol::witness(),
    );
    test_utils::kernel_limit::pinned(body, Tol::witness().eps(), &[1e-12], "segment A")
}

/// Segment B: the SAME tube radius on a tighter ring, continuing from
/// A's end with A's own end tangent — so the two walls meet G1 along
/// the shared terminal meridian circle and the composed material runs
/// smoothly through it.
fn segment_b() -> Option<AtRestBody<f64>> {
    Some(finished(
        "segment B",
        segment_b_tube()?.body,
        Tol::witness(),
    ))
}

/// Segment B as the tube door answers it.
fn segment_b_tube() -> Option<sweep::Revolved<f64>> {
    let turn = TURN.to_radians();
    let end = Point3::new(-RING + RING * turn.cos(), 0.0, RING * turn.sin());
    let tangent = Vec3::new(-turn.sin(), 0.0, turn.cos());
    // The tighter ring's centre sits perpendicular to that tangent, on
    // the same side A is turning toward.
    let inward = Vec3::new(-tangent.z, 0.0, tangent.x);
    let center = end + inward * 1.1;
    let body = tube_along_arc(
        tube_frame(center, axis(), (end - center).normalize(), Tol::witness()),
        1.1,
        window(170.0),
        TUBE,
        Tol::witness(),
    );
    test_utils::kernel_limit::pinned(body, Tol::witness().eps(), &[], "segment B")
}

/// The toroidal SOCKET: a hollow elbow whose BORE is a torus of tube
/// radius [`TUBE`] — the curved spelling of the bored plate.
fn socket() -> Option<AtRestBody<f64>> {
    let body = tube_along_arc_hollow(
        tube_frame(
            Point3::new(-RING, 0.0, 0.0),
            axis(),
            Vec3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        ),
        RING,
        window(TURN),
        0.09,
        0.09 - TUBE,
        Tol::witness(),
    );
    let body =
        test_utils::kernel_limit::pinned(body, Tol::witness().eps(), &[1e-12], "the socket")?.body;
    Some(finished("the socket", body, Tol::witness()))
}

/// Two FULL tori KISSING along one circle: coaxial and coplanar, ring
/// radii `R` and `R + 2r`, so the two tube centre circles are `2r`
/// apart and the tubes touch externally along the midplane circle of
/// radius `R + r`. That circle is the outer equator seam of the inner
/// torus and the inner equator seam of the outer one, so it is a
/// boundary edge of a face on each side — a rim, not an interior
/// touch.
fn kissing_pair() -> (AtRestBody<f64>, AtRestBody<f64>) {
    (full_torus(RING), full_torus(RING + 2.0 * TUBE))
}

/// A full solid torus of ring radius `major` and tube [`TUBE`], about
/// the origin on `+z`. It carries its two wall faces and nothing else.
fn full_torus(major: f64) -> AtRestBody<f64> {
    let body = tube_along_arc(
        tube_frame(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        ),
        major,
        TubeWindow::Full,
        TUBE,
        Tol::witness(),
    )
    .expect("the full torus builds")
    .body;
    finished("the full torus", body, Tol::witness())
}

/// The torus faces of a body whose tube radius is `minor`.
fn torus_faces(body: &Body<f64>, minor: f64) -> Vec<FaceKey> {
    let hits: Vec<_> = query::all_faces(body)
        .into_iter()
        .filter(|&f| {
            matches!(
                body.face_surface(f),
                Ok(geom::Surface::Torus { minor_radius, .. })
                    if (*minor_radius - minor).abs() < 1e-12
            )
        })
        .collect();
    assert!(
        !hits.is_empty(),
        "the fixture must carry torus faces at tube radius {minor}"
    );
    hits
}

// -------------------------------------------------------------------
// 1. The operand gate.
// -------------------------------------------------------------------

/// The peg-in-socket union, held to what it promises at
/// every ε: sound at tiers 2 and 3′ and the at-rest certificate,
/// additive (the parts only touch), one shell of four faces, six edges
/// and four vertices, and a legal operand.
///
/// The chord join builds it at every ε (each strut's half beside a
/// germ's locus edge faces it).
fn peg_in_socket_union_holds(s: &AtRestBody<f64>, p: &AtRestBody<f64>) {
    let r = topo::union(s, p, Tol::witness())
        .unwrap_or_else(|e| panic!("the peg-in-socket union builds: {e:?}"));
    let bb = r.body().expect("a union of two solids is not empty");
    assert_eq!(topo::validate_closed(&bb.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
        Ok(()),
        "tier 3′"
    );
    assert!(
        topo::validate_geometric_certificate(&bb.body, Tol::witness()).is_ok(),
        "the at-rest certificate"
    );
    let vol = |b: &Body<f64>| {
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(b),
                Tol::witness(),
            ),
            Tol::witness(),
        )
        .volume
    };
    let (v, want) = (vol(&bb.body), vol(s) + vol(p));
    assert!(
        (v - want).abs() < 1e-9 * want.max(1.0),
        "the parts only touch, so the union is additive: {v} against {want}"
    );
    assert_eq!(
        (
            bb.body.faces().count(),
            bb.body.edges().count(),
            bb.body.vertices().count(),
            bb.body.shells().count()
        ),
        (4, 6, 4, 1),
        "F, E, V, shells"
    );
    sweep::test_support::assert_legal_operand("peg ∪ socket", &bb.body, Tol::witness());
}

/// **Two coincident full tori are one body.** A full torus carries
/// nothing but its two wall faces, and the two operands are the same
/// torus: every wall pair is one carrier with one material side, which
/// the boolean decides a continuation by margin, so the gate is not the
/// question. No crossing cuts either torus, and the section pass defers
/// each coincident pair to the shells' `On` verdict: each torus lies on
/// the other. `∪` and `∩` build the torus, at Pappus' `2π²·R·r²` within
/// its pad and through tier 3 and 3′; each difference is nothing.
#[test]
fn two_coincident_full_tori_are_one_body() {
    let (a, b) = (full_torus(RING), full_torus(RING));
    assert_eq!(
        a.faces().count(),
        torus_faces(&a, TUBE).len(),
        "a full torus must carry nothing but wall faces"
    );
    let want = 2.0 * core::f64::consts::PI.powi(2) * RING * TUBE * TUBE;
    for (op, r) in [("A ∪ B", topo::union(&a, &b, Tol::witness()))]
        .into_iter()
        .chain(subtract_both_orders_and_intersect(&a, &b))
    {
        if op.contains('∖') {
            assert!(
                matches!(r, Ok(BooleanResult::Empty)),
                "{op}: one body less itself is nothing: {r:?}"
            );
            continue;
        }
        let Ok(BooleanResult::Body(bb)) = r else {
            panic!("{op}: one body: {r:?}");
        };
        assert_eq!(
            topo::validate_geometric(&bb.body, Tol::witness()),
            Ok(()),
            "{op}: tier 3"
        );
        assert_eq!(
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
            Ok(()),
            "{op}: tier 3′"
        );
        let m = topo::test_support::pinned_volume("the measured body", &bb.body, Tol::witness());
        assert!(
            (m.volume - want).abs() <= m.volume_pad + 1e-9 * want,
            "{op}: volume {} ± {} against Pappus {want}",
            m.volume,
            m.volume_pad
        );
    }
}

/// **The peg seated in its socket builds.** The socket carries an
/// OUTER wall as well as its bore; the outer wall stands 0.03 m clear
/// of the peg, and the two faces' windows are one rectangle about one
/// spine circle, so no sound box separates them. With the torus on the
/// KIND roster the pair's boxes decide nothing, the bore and the peg's
/// wall glue as the one carrier they are, and the op builds the union,
/// sound and additive ([`peg_in_socket_union_holds`]).
#[test]
fn a_partly_covered_torus_pair_is_no_longer_a_gate_question() {
    let (Some(s), Some(p)) = (socket(), segment_a()) else {
        return;
    };
    peg_in_socket_union_holds(&s, &p);
}

// -------------------------------------------------------------------
// 2. The G1 chain.
// -------------------------------------------------------------------

/// **The G1 tube chain is one bent tube**: the two segments both end on
/// their terminal meridian circle, a junction the glue door decides on
/// the rim's identity and its π wedge, so its neutral points (`κ_rel`
/// exactly zero) are no question there; an edge of one segment ending on
/// that rim tangent to the other's wall has that end's root factored
/// (`geom_brep::curve_face::end_root`); and the seam is described where
/// it rests, its bend under-determined. The two walls on the meridian
/// glue a junction (the routing's π seam with the walls leaving on
/// opposite sides, each segment's own rim edge a convex crease), and the
/// union is one solid at the two segments' Pappus volumes' sum, sound at
/// tier 3 and 3′, in both orders.
#[test]
fn the_g1_tube_chain_is_one_bent_tube() {
    let (Some(a), Some(b)) = (segment_a(), segment_b()) else {
        return;
    };
    let walls: Vec<_> = topo::boolean::rim_door(&a, &b, Tol::witness())
        .into_iter()
        .filter(|(kinds, _)| *kinds == (geom::SurfaceKind::Torus, geom::SurfaceKind::Torus))
        .map(|(_, door)| door)
        .collect();
    assert!(
        !walls.is_empty() && walls.iter().all(|d| *d == topo::boolean::RimDoor::Junction),
        "the two walls on the meridian glue a junction: {walls:?}"
    );
    let tube = core::f64::consts::PI * TUBE * TUBE;
    let want = tube * RING * TURN.to_radians() + tube * 1.1 * 170.0_f64.to_radians();
    for (order, x, y) in [("A, B", &a, &b), ("B, A", &b, &a)] {
        let Ok(BooleanResult::Body(bb)) = topo::union(x, y, Tol::witness()) else {
            panic!("{order}: the chain unions to one body");
        };
        assert_eq!(bb.body.solids().count(), 1, "{order}: one solid");
        assert_eq!(
            topo::validate_geometric(&bb.body, Tol::witness()),
            Ok(()),
            "{order}: tier 3"
        );
        assert_eq!(
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
            Ok(()),
            "{order}: tier 3′"
        );
        let v = topo::test_support::pinned_volume("the chain", &bb.body, Tol::witness()).volume;
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "{order}: volume {v} against Pappus {want}"
        );
    }
}

// -------------------------------------------------------------------
// 3. ∖ and ∩ on the same fixtures.
// -------------------------------------------------------------------

/// **∖ and ∩ reach ∪'s doors; the seated peg answers the closed form
/// and the rest stop where ∪ does.** The coincident pair's are
/// [`two_coincident_full_tori_are_one_body`]'s. The torus is on the roster,
/// so a torus pair under a subtract or an intersect reaches the same
/// doors as under a union, in both operand orders:
///
/// - the socket and peg share no interior: `∩` is empty and
///   each difference its minuend whole, the volumes by Pappus
///   (`πr²·Rθ` for a tube of radius `r` turned through `θ` on a ring of
///   radius `R`);
/// - the chain passes the crossing layer and refuses at the fallback's
///   extent; the kissing pair refuses at the crossing layer (escalated
///   where the run's band puts the sampled margin in its window).
#[test]
fn subtract_and_intersect_on_the_torus_rest_fixtures() {
    let (Some(s), Some(p)) = (socket(), segment_a()) else {
        return;
    };
    let pappus = |r_out: f64, r_in: f64| {
        core::f64::consts::PI * (r_out * r_out - r_in * r_in) * RING * TURN.to_radians()
    };
    let wants = [
        ("A ∖ B", Some(pappus(0.09, TUBE))),
        ("B ∖ A", Some(pappus(TUBE, 0.0))),
        ("A ∩ B", None),
    ];
    for ((op, r), (_, want)) in subtract_both_orders_and_intersect(&s, &p)
        .into_iter()
        .zip(wants)
    {
        let what = format!("socket and peg, {op}");
        match (r, want) {
            (Ok(BooleanResult::Empty), None) => {}
            (Ok(BooleanResult::Body(bb)), Some(want)) => {
                let v = topo::test_support::pinned_volume(
                    "the measured body",
                    &bb.body,
                    Tol::witness(),
                )
                .volume;
                assert!(
                    (v - want).abs() <= 1e-12 * want,
                    "{what}: volume {v} against Pappus {want}"
                );
                assert_eq!(bb.body.shells().count(), 1, "{what}: one shell");
                assert_eq!(
                    topo::validate_geometric(&bb.body, Tol::witness()),
                    Ok(()),
                    "{what}: tier 3"
                );
                assert_eq!(
                    topo::validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
                    Ok(()),
                    "{what}: tier 3′"
                );
            }
            (r, want) => panic!("{what}: want {want:?} (None: empty), got {r:?}"),
        }
    }
    if let (Some(sa), Some(sb)) = (segment_a(), segment_b()) {
        for (op, r) in subtract_both_orders_and_intersect(&sa, &sb) {
            let err = r.expect_err(op);
            assert!(
                matches!(err, BooleanError::FallbackExtentUnsupported { .. }),
                "chain, {op}: past the crossing layer, at the fallback's extent: {err:?}"
            );
        }
    }
    for (name, (a, b)) in [("kissing pair", kissing_pair())] {
        for (op, r) in subtract_both_orders_and_intersect(&a, &b) {
            let err = r.expect_err(op);
            // The escalation is the circle rung's sampled clearance
            // landing in the band's window (the `1e-6` row); at the
            // finer rows the same edge's touch, read in the band's gap,
            // escalates on the circle × torus roots, at the edge and the
            // torus.
            assert!(
                matches!(
                    err,
                    BooleanError::CurvedPierceUnsupported { .. }
                        | BooleanError::Escalated {
                            diag: geom_core::Indeterminate {
                                predicate: Some("bool_conic_curved_clearance"),
                                ..
                            },
                            ..
                        }
                        | BooleanError::CurvedPierceEscalated {
                            decision: topo::BooleanDecision::ArcTorusRoots,
                            ..
                        }
                ),
                "{name}, {op}: the crossing layer's refusal: {err:?}"
            );
        }
    }
}

/// The chain's two segments really are the tube chain the chain rows claim
/// — same tube radius, and their shared terminal circle is one circle,
/// not two near ones. Without this the chain rows could be passing on a
/// fixture that is not the geometry it names.
#[test]
fn the_chain_fixture_is_g1_with_one_shared_rim() {
    let (Some(a), Some(b)) = (segment_a(), segment_b()) else {
        return;
    };
    let radii = |body: &Body<f64>| {
        body.faces()
            .filter_map(|(fk, _)| match body.face_surface(fk) {
                Ok(&geom::Surface::Torus { minor_radius, .. }) => Some(minor_radius),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    for r in radii(&a).into_iter().chain(radii(&b)) {
        assert_eq!(r, TUBE, "both segments carry the same tube radius");
    }
    // A cap's plane, read as a plane: its origin is its station's frame
    // origin, which need not lie in the cap's disc.
    let planes = |body: &Body<f64>| {
        body.faces()
            .filter_map(|(fk, _)| match body.face_surface(fk) {
                Ok(&geom::Surface::Plane { frame, .. }) => {
                    let (origin, normal) = (frame.origin(), frame.axis());
                    Some((origin, normal))
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let shared = planes(&a)
        .into_iter()
        .flat_map(|(oa, na)| {
            planes(&b).into_iter().map(move |(ob, nb)| {
                na.cross(nb).norm() < 1e-12
                    && (ob - oa).dot(na).abs() < 1e-12
                    && (oa - ob).dot(nb).abs() < 1e-12
            })
        })
        .filter(|&coincide| coincide)
        .count();
    assert_eq!(
        shared, 1,
        "exactly one cap plane is shared between the segments"
    );
    // And the shared cap is centred at one point: A's end joint is B's
    // start joint, where the section's centre stands on that plane.
    let (Some(ta), Some(tb)) = (segment_a_tube(), segment_b_tube()) else {
        return;
    };
    let (ca, cb) = (
        sweep::revolved_caps(&ta).expect("a windowed tube has caps"),
        sweep::revolved_caps(&tb).expect("a windowed tube has caps"),
    );
    assert!(
        (ca.end.origin - cb.start.origin).norm() < 1e-12,
        "the shared cap is centred at one point: A ends at {:?}, B starts at {:?}",
        ca.end.origin,
        cb.start.origin
    );
}

/// **A torus whose walls' sense bits are reversed is not a finished
/// body**, so it cannot carry the guard this row was: that the
/// routing reads the SECOND face's sense and not the first's twice
/// (the glue door's `route_rim` hands `classify_shared_rim` one bit
/// per face, two adjacent `bool`s a transposed call site compiles
/// with). Reversing the outer torus's walls through `Body::set_face_sense`
/// turns their outward normals inward, and the at-rest gate refuses
/// exactly those walls. The guard waits on a finished fixture whose two
/// sides carry different bits
/// (`work/tang/the-rim-routings-sense-guard-has-no-finished-fixture.md`).
#[test]
fn the_rim_routing_reads_the_second_faces_sense_and_not_the_firsts() {
    let (_, b) = kissing_pair();
    let mut b = b.into_body();
    let mut walls = torus_faces(&b, TUBE);
    for &fb in &walls {
        let s = b.get_face(fb).expect("the wall face resolves").sense;
        b.set_face_sense(fb, !s).expect("the key is live");
    }
    let errors = topo::finish(b, Tol::witness()).expect_err("inward walls are not a finished body");
    let mut inverted: Vec<_> = errors
        .iter()
        .map(|e| match e {
            topo::ValidationError::CurvedSenseInverted { face, .. } => *face,
            other => panic!("a finding other than the reversed walls': {other:?}"),
        })
        .collect();
    inverted.sort();
    walls.sort();
    assert_eq!(inverted, walls, "the gate names each reversed wall once");
}
