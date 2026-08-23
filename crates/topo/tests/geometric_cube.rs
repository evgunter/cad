//! M2 PR 3 acceptance: the M1 cube upgraded to full geometry — Newell
//! planes on every face, certified line carriers on every edge —
//! passing tiers 1–3; the attachment gate's teeth; the
//! prefer-intrinsic upgrade path (`Intersection` via `set_edge_curve`);
//! determinism of certification records; and the Dual value-channel
//! contract. (The Interval lane lives in `interval_body.rs`.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::{CertCheck, CertifyError, EdgeCurveSpec, EdgeGeometry};
use geom_core::{Point3, Vec3};
use topo::{
    Body, EulerOpError, FaceSurface, MefSite, MevSite, validate, validate_closed,
    validate_geometric,
};

mod common;
use common::{GeoCube, describe_as_intersections, geometric_cube};
use geom_core::Tol;

#[test]
fn geometric_cube_passes_all_three_tiers() {
    let t = geometric_cube::<f64>();
    assert_eq!(validate(&t.body), Ok(()));
    assert_eq!(validate_closed(&t.body), Ok(()));
    // 8 vertices, 12 edges, 6 faces; 6 planes + no leftover Nurbs.
    assert_eq!(t.body.vertices().count(), 8);
    assert_eq!(t.body.edges().count(), 12);
    assert_eq!(t.body.faces().count(), 6);
    assert_eq!(t.body.surfaces().count(), 6);
    assert!(
        t.body
            .surfaces()
            .all(|(_, s)| matches!(s, Surface::Plane { .. }))
    );
    // Prefer-intrinsic enforcement (D2, M2 PR 4 fix pass): every cube
    // edge is a definitely-transverse plane-pair corner, so at rest the
    // un-upgraded chords are named — all twelve, nothing else.
    let errs = validate_geometric(&t.body, Tol::witness()).unwrap_err();
    assert_eq!(errs.len(), 12, "{errs:?}");
    assert!(
        errs.iter()
            .all(|e| matches!(e, topo::ValidationError::TransverseNotIntrinsic { .. })),
        "{errs:?}"
    );
    // Upgraded (the construction-discipline the rule enforces), the
    // cube passes all three tiers.
    let mut body = t.body;
    describe_as_intersections(&mut body);
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
}

/// **M5 S10 acceptance row: tier 3 is the sense gate (check 6).**
///
/// A face's outward normal is `Face::sense_sign()` times its surface's
/// chart normal, and by the interior-left rule its outer loop winds
/// CCW about that outward normal. So `sense` and the stored winding
/// are two encodings of ONE fact, and check 6 — the loop's Newell
/// functional against the outward normal — is precisely the gate that
/// they agree. `flipped_face_sense_for_tests` inverts one bit and
/// nothing else, producing a body that is inside-out at that face;
/// tier 3 must refuse it, by name.
///
/// The row also pins why check 6 carries this alone: the +V invariant
/// (check 7) is computed from the same loop windings, which a lone
/// sense flip does not touch, so both bodies meter the identical
/// volume. Nothing else in the at-rest battery can see the defect.
///
/// Bit-identity: the unflipped cube still validates clean — planar
/// sweeps mint `sense: true` throughout (M5 S11 reverses only walls
/// whose material lies against the chart normal, all curved here), so
/// the threading multiplies by exactly `+1`.
#[test]
fn tier_three_refuses_a_hand_flipped_face_sense() {
    let t = geometric_cube::<f64>();
    let mut body = t.body;
    describe_as_intersections(&mut body);
    assert_eq!(
        validate_geometric(&body, Tol::witness()),
        Ok(()),
        "the fixture is clean"
    );

    let (face, outer) = body.faces().map(|(k, f)| (k, f.outer)).next().unwrap();
    let flipped = body.flipped_face_sense_for_tests(face).unwrap();
    let errs = validate_geometric(&flipped, Tol::witness()).unwrap_err();
    assert!(
        errs.contains(&topo::ValidationError::LoopRoleInverted {
            face,
            r#loop: outer,
        }),
        "check 6 must name the inverted face's outer loop; got {errs:?}"
    );

    // Winding-derived, hence blind to a lone sense flip: same volume.
    let volume = |b: &Body<f64>| {
        topo::props::mass_properties(b, Tol::witness())
            .unwrap()
            .volume
    };
    assert_eq!(
        volume(&body).to_bits(),
        volume(&flipped).to_bits(),
        "the +V invariant cannot see a sense flip — check 6 stands alone"
    );
}

#[test]
fn without_the_top_cap_tier3_rejects_the_nurbs_seed() {
    // The seed face carrying the honest "no description yet" Nurbs
    // state at rest: tier 1/2 fine, tier 3 rejects it by name (put the
    // state back through the public setter).
    let t = geometric_cube::<f64>();
    let mut body = t.body;
    let seed_face = t.seed.face;
    body.set_face_surface(seed_face, FaceSurface::New(Surface::nurbs_placeholder()))
        .unwrap();
    assert_eq!(validate_closed(&body), Ok(()));
    let errs = validate_geometric(&body, Tol::witness()).unwrap_err();
    assert!(
        errs.iter().any(|e| matches!(
            e,
            topo::ValidationError::UncertifiableSurface { face } if *face == seed_face
        )),
        "{errs:?}"
    );
}

#[test]
fn wrong_cache_is_rejected_at_attachment() {
    // A carrier displaced by 100·ε (definitely beyond the escalation
    // band at every CI ε row) must be refused by the op itself, body
    // untouched.
    let eps = Tol::witness().get().eps;
    let c = |x: f64, y: f64, z: f64| Point3::new(x, y, z);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(c(0.0, 0.0, 0.0)).unwrap();
    let edges_before = body.edges().count();
    let mut spec = EdgeCurveSpec::line_between(c(0.0, 0.0, 0.0), c(1.0, 0.0, 0.0));
    spec.carrier = Curve3::Line {
        origin: c(0.0, 100.0 * eps, 0.0),
        dir: Vec3::unit_x(),
    };
    let err = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            c(1.0, 0.0, 0.0),
            spec,
            Tol::witness(),
        )
        .unwrap_err();
    assert!(
        matches!(
            err,
            EulerOpError::Certification {
                error: CertifyError::ResidualExceeded {
                    check: CertCheck::EndpointStart,
                    ..
                }
            }
        ),
        "{err:?}"
    );
    assert_eq!(body.edges().count(), edges_before, "atomic: no mutation");
    assert_eq!(validate(&body), Ok(()));
}

#[test]
fn cube_edges_upgrade_to_intersections_and_pass_tier3() {
    // The prefer-intrinsic form: re-describe every cube edge as the
    // Intersection of its two adjacent faces' planes (witness at the
    // edge midpoint), via the certified upgrade path. Tier 3 then
    // re-certifies all twelve against BOTH planes.
    let t = geometric_cube::<f64>();
    let mut body = t.body;
    describe_as_intersections(&mut body);
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
    assert!(body.curves().all(|(_, c)| matches!(
        c.certified().map(topo::EdgeCurve::description),
        Some(EdgeGeometry::Intersection { .. })
    )));

    // Teeth: an Intersection naming a NON-adjacent pair is refused by
    // the upgrade path (adjacency coherence).
    let (edge_key, edge) = body.edges().next().map(|(k, e)| (k, e.clone())).unwrap();
    let start = body.get_half_edge(edge.he_plus).unwrap().start;
    let end = body.half_edge_end(edge.he_plus).unwrap();
    let p0 = *body
        .get_point(body.get_vertex(start).unwrap().point)
        .unwrap();
    let p1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
    // Two brand-new coplanar surfaces that DO contain the edge but are
    // not the adjacent faces' arena entries.
    let foreign1 = body
        .set_face_surface(t.seed.face, FaceSurface::Inherit)
        .unwrap(); // the top face's key — not this bottom edge's face
    let mut spec = EdgeCurveSpec::line_between(p0, p1);
    spec.description = EdgeGeometry::Intersection {
        s1: foreign1,
        s2: foreign1,
        witness: p0.lerp(p1, 0.5),
    };
    let err = body
        .set_edge_curve(edge_key, spec, Tol::witness())
        .unwrap_err();
    assert!(
        matches!(err, EulerOpError::DescriptionNotAdjacent { .. }),
        "{err:?}"
    );
}

#[test]
fn certification_records_are_byte_identical_across_runs() {
    // D9: two replays of the same construction produce byte-identical
    // certification records (and parameter caches).
    let a = geometric_cube::<f64>();
    let b = geometric_cube::<f64>();
    let dump = |t: &GeoCube<f64>| {
        t.body
            .curves()
            .map(|(k, c)| {
                let c = c.certified().unwrap();
                format!("{k:?} {:?} {:?}\n", c.params(), c.certificate())
            })
            .collect::<String>()
    };
    assert_eq!(dump(&a), dump(&b));
}

#[test]
fn dual_lane_decisions_match_f64_bit_for_bit() {
    // The value channel of a Dual build takes the identical certified
    // path: same construction succeeds, and every certificate's value
    // channel equals the f64 certificate bitwise (tangent data never
    // influences a decision). Both lanes get the prefer-intrinsic
    // upgrade first (M2 PR 4 fix pass: the transverse cube chords must
    // carry Intersection at rest), so the compared certificates are the
    // upgraded Intersection re-certifications.
    use geom_core::{Dual, Dual64};
    let mut f = geometric_cube::<f64>();
    let mut d = geometric_cube::<Dual64>();
    describe_as_intersections(&mut f.body);
    describe_as_intersections(&mut d.body);
    assert_eq!(validate_geometric(&d.body, Tol::witness()), Ok(()));
    let f_certs: Vec<f64> = f
        .body
        .curves()
        .map(|(_, c)| c.certified().unwrap().certificate().max_residual)
        .collect();
    let d_certs: Vec<Dual<f64>> = d
        .body
        .curves()
        .map(|(_, c)| c.certified().unwrap().certificate().max_residual)
        .collect();
    assert_eq!(f_certs.len(), d_certs.len());
    for (fv, dv) in f_certs.iter().zip(&d_certs) {
        assert_eq!(fv.to_bits(), dv.value.to_bits());
    }
}

#[test]
fn near_tangent_intersection_attachment_escalates() {
    // ε-parameterized sliver: two planes at angle 3ε (in-band at lever
    // arm 1 for every CI ε row) — an Intersection description across
    // them is refused as a sliver escalation, typed, at the upgrade
    // path. Built standalone (plane × plane through one line).
    let eps = Tol::witness().get().eps;
    let theta = 3.0 * eps;
    let c = |x: f64, y: f64, z: f64| Point3::new(x, y, z);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(c(0.0, 0.0, 0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            c(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let split = body
        .mef(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            EdgeCurveSpec::line_between(c(0.0, 0.0, 0.0), c(1.0, 0.0, 0.0)),
            FaceSurface::New(Surface::Plane {
                origin: c(0.0, 0.0, 0.0),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            }),
            Tol::witness(),
        )
        .unwrap();
    // The old face gets the near-tangent plane through the same line.
    let tilted = body
        .set_face_surface(
            seed.face,
            FaceSurface::New(Surface::Plane {
                origin: c(0.0, 0.0, 0.0),
                normal: Vec3::new(0.0, theta.sin(), theta.cos()),
                u_ref: Vec3::unit_x(),
            }),
        )
        .unwrap();
    let flat = body.get_face(split.face).unwrap().surface;
    let mut spec = EdgeCurveSpec::line_between(c(0.0, 0.0, 0.0), c(1.0, 0.0, 0.0));
    spec.description = EdgeGeometry::Intersection {
        s1: flat,
        s2: tilted,
        witness: c(0.5, 0.0, 0.0),
    };
    let err = body
        .set_edge_curve(split.edge, spec, Tol::witness())
        .unwrap_err();
    assert!(
        matches!(
            err,
            EulerOpError::Certification {
                error: CertifyError::Escalated {
                    check: CertCheck::Transversality,
                    ..
                }
            }
        ),
        "{err:?}"
    );
}

#[test]
fn totality_no_panics_on_poison_inputs() {
    // NaN coordinates, poison specs, Nurbs carriers: every failure is a
    // typed error — never a panic (D9).
    let c = |x: f64, y: f64, z: f64| Point3::new(x, y, z);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(c(0.0, 0.0, 0.0)).unwrap();
    let nan = f64::NAN;
    assert!(matches!(
        body.mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop
            },
            c(nan, 0.0, 0.0),
            Tol::witness(),
        ),
        Err(EulerOpError::Certification { .. })
    ));
    let mut spec = EdgeCurveSpec::line_between(c(0.0, 0.0, 0.0), c(1.0, 0.0, 0.0));
    spec.carrier = Curve3::nurbs_placeholder();
    assert!(matches!(
        body.mev(
            MevSite::Lone {
                r#loop: seed.r#loop
            },
            c(1.0, 0.0, 0.0),
            spec,
            Tol::witness(),
        ),
        Err(EulerOpError::Certification {
            error: CertifyError::Unimplemented
        })
    ));
    assert_eq!(validate(&body), Ok(()));
}
