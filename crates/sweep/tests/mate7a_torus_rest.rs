//! MATE-7a — torus pairs through the boolean, on torus geometry a
//! producer actually mints (`sweep::tube_along_arc`, solid and hollow).
//!
//! 1. **The operand gate lets a torus through.** The torus is on the
//!    KIND roster, so no torus pair is the gate's question; the peg
//!    seated in its socket builds through the join, and a coincident
//!    pair refuses typed, further on.
//! 2. **A G1 tube chain refuses at the crossing layer**: an edge of one
//!    segment rides the other's wall.
//! 3. **∖ and ∩ stop where ∪ does**, except on the socket. The torus is
//!    on the revert roster, so each fixture under a subtract (both
//!    orders) or an intersect refuses at the door its union meets; the
//!    peg seated in the socket shares no interior with it, so its ∩ is
//!    empty and each difference its minuend whole.
//!
//! **What this suite also RECORDS is where the lane stops**: a
//! coincident torus pair passes the crossing layer and stops at the
//! no-crossings fallback's section pass, on the pair's tangency.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::revert_ops::subtract_both_orders_and_intersect;

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
/// starting at the origin heading `+z`.
fn segment_a() -> AtRestBody<f64> {
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
    )
    .expect("segment A builds")
    .body;
    finished("segment A", body, Tol::witness())
}

/// Segment B: the SAME tube radius on a tighter ring, continuing from
/// A's end with A's own end tangent — so the two walls meet G1 along
/// the shared terminal meridian circle and the composed material runs
/// smoothly through it.
fn segment_b() -> AtRestBody<f64> {
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
    )
    .expect("segment B builds")
    .body;
    finished("segment B", body, Tol::witness())
}

/// The toroidal SOCKET: a hollow elbow whose BORE is a torus of tube
/// radius [`TUBE`] — the curved spelling of the bored plate.
fn socket() -> AtRestBody<f64> {
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
    )
    .expect("socket builds")
    .body;
    finished("the socket", body, Tol::witness())
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
                body.get_face(f).and_then(|fd| body.get_surface(fd.surface)),
                Some(geom::Surface::Torus { minor_radius, .. })
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
    let vol = |b: &Body<f64>| topo::mass_properties(b, Tol::witness()).unwrap().volume;
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

/// **A coincident full torus pair reaches past the operand gate.** A
/// full torus carries NOTHING but its two wall faces, and the two
/// operands are the SAME torus: one carrier, one material side, which
/// the boolean decides a continuation by margin — so the operation
/// reaches the classification layer, and the gate is not what refuses.
#[test]
fn a_fully_covered_torus_pair_reaches_past_the_operand_gate() {
    let (a, b) = (full_torus(RING), full_torus(RING));
    assert_eq!(
        a.faces().count(),
        torus_faces(&a, TUBE).len(),
        "a full torus must carry nothing but wall faces"
    );
    let err = topo::union(&a, &b, Tol::witness())
        .expect_err("the lane still stops downstream of the gate");
    assert!(
        !matches!(err, BooleanError::CurvedPairUnsupported { .. }),
        "every pair is one carrier, so the gate must not be what refuses: {err:?}"
    );
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
    let (s, p) = (socket(), segment_a());
    peg_in_socket_union_holds(&s, &p);
}

/// **Where the lane stops once the gate is past, held still.** Two
/// coincident full tori, every wall pair one carrier.
///
/// The seam meridians ride the other torus's carrier, where the
/// residual is identically zero; the carrier-identity rung reads the
/// glued pair before the sampled clearance, so the edge's clearance is
/// zero by that certificate.
///
/// What stops the lane now is the no-crossings fallback's section pass:
/// no crossing cuts either torus, and a coincident coaxial pair is the
/// tangent row of the torus × torus classification (the tube circles
/// coincide, so the nesting margin is zero) — exactly the case the
/// vertex probe cannot decide (every vertex it would probe lies on the
/// other torus), so the pass refuses typed before the probe runs. The property is the one
/// this row has always held: a coincident torus pair never reaches a
/// body it cannot justify.
#[test]
fn the_admitted_torus_lane_stops_at_the_section_pass() {
    let (a, b) = (full_torus(RING), full_torus(RING));
    let err = topo::union(&a, &b, Tol::witness())
        .expect_err("a coincident torus pair still has no classification verdict");
    let BooleanError::FallbackExtentUnsupported { what, .. } = err else {
        panic!(
            "the coincident pair passes the crossing layer and stops at \
             the section pass: {err:?}"
        );
    };
    // The tangency sentence (R-tan): the coincident tube circles are the
    // torus × torus classification's zero nesting margin.
    assert!(
        what.contains("tangent or near-tangent carriers"),
        "the pass refuses the coincident pair as a tangency: {what}"
    );
}

// -------------------------------------------------------------------
// 2. The G1 chain.
// -------------------------------------------------------------------

/// **The G1 tube chain refuses at the crossing layer**: an edge of one
/// segment rides the other's wall. In both orders.
#[test]
fn the_g1_tube_chain_refuses_at_the_crossing_layer() {
    let (a, b) = (segment_a(), segment_b());
    for (x, y) in [(&a, &b), (&b, &a)] {
        let err = topo::union(x, y, Tol::witness()).expect_err("the chain refuses");
        assert!(
            matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
            "the crossing layer's refusal: {err:?}"
        );
    }
}

// -------------------------------------------------------------------
// 3. ∖ and ∩ on the same fixtures.
// -------------------------------------------------------------------

/// **∖ and ∩ pass the revert roster; the seated peg answers the closed
/// form and the rest stop where ∪ does.** The torus is on the roster,
/// so a torus pair under a subtract or an intersect reaches the same
/// doors as under a union, in both operand orders:
///
/// - the socket and peg share no interior: `∩` is empty and
///   each difference its minuend whole, the volumes by Pappus
///   (`πr²·Rθ` for a tube of radius `r` turned through `θ` on a ring of
///   radius `R`);
/// - the coincident pair (a continuation, both materials on one side)
///   stops at the no-crossings section pass on the tangency (R-tan);
/// - the chain and the kissing pair refuse at the crossing layer
///   (escalated where the run's band puts the sampled margin in its
///   window).
#[test]
fn subtract_and_intersect_on_the_torus_rest_fixtures() {
    let tangency = |what: &str, err: &BooleanError| {
        let BooleanError::FallbackExtentUnsupported { what: w, .. } = err else {
            panic!("{what}: the section pass refuses the tangency: {err:?}");
        };
        assert!(
            w.contains("tangent or near-tangent carriers"),
            "{what}: {w}"
        );
    };
    let (s, p) = (socket(), segment_a());
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
                let v = topo::mass_properties(&bb.body, Tol::witness())
                    .unwrap()
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
    let (a, b) = (full_torus(RING), full_torus(RING));
    for (op, r) in subtract_both_orders_and_intersect(&a, &b) {
        tangency(&format!("coincident pair, {op}"), &r.expect_err(op));
    }
    for (name, (a, b)) in [
        ("chain", (segment_a(), segment_b())),
        ("kissing pair", kissing_pair()),
    ] {
        for (op, r) in subtract_both_orders_and_intersect(&a, &b) {
            let err = r.expect_err(op);
            // The escalation is the circle rung's sampled clearance
            // landing in the band's window (the `1e-6` row); at the
            // finer rows the same edge refuses at the pierce door.
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
    let (a, b) = (segment_a(), segment_b());
    let radii = |body: &Body<f64>| {
        body.faces()
            .filter_map(|(_, f)| match body.get_surface(f.surface) {
                Some(&geom::Surface::Torus { minor_radius, .. }) => Some(minor_radius),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    for r in radii(&a).into_iter().chain(radii(&b)) {
        assert_eq!(r, TUBE, "both segments carry the same tube radius");
    }
    let plane_origins = |body: &Body<f64>| {
        body.faces()
            .filter_map(|(_, f)| match body.get_surface(f.surface) {
                Some(&geom::Surface::Plane { origin, .. }) => Some(origin),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let shared = plane_origins(&a)
        .into_iter()
        .flat_map(|oa| {
            plane_origins(&b)
                .into_iter()
                .map(move |ob| (oa - ob).norm())
        })
        .filter(|d| *d < 1e-12)
        .count();
    assert_eq!(
        shared, 1,
        "exactly one cap plane is shared between the segments"
    );
    match topo::union(&a, &b, Tol::witness()) {
        Ok(BooleanResult::Body(_) | BooleanResult::Empty) => {
            panic!("a torus chain must not silently succeed")
        }
        Err(_) => {}
    }
}

/// **A torus whose walls' sense bits are reversed is not a finished
/// body**, so it cannot carry the guard this row was: that the
/// routing reads the SECOND face's sense and not the first's twice
/// (`verify_tangent_declaration` hands `classify_shared_rim` one bit
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
    let errors = topo::AtRestBody::validate(b, Tol::witness())
        .expect_err("inward walls are not a finished body");
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
