//! Rejection and escalation fixtures: every typed error variant hit by
//! a closed-form profile, with the exact error asserted.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{
    arc_kisses_line, bowtie, chain, circle_h, near_tangent_hole, profile, rect, tangent_hole, tol,
};
use geom_core::MarginDiag;
use geom_core::Point2;
use geom_core::Tol;
use geom_core::{Band, Indeterminate};
use profile::{
    ArcSweep, Center, ContactKind, EscalationSite, Open, PathError, ProfileError, SegmentRef,
    SketchPlane, Start,
};

fn err(p: &profile::Profile<f64>) -> ProfileError {
    p.validate(tol()).expect_err("fixture must be rejected")
}

fn sref(loop_index: usize, segment_index: usize) -> SegmentRef {
    SegmentRef {
        loop_index,
        segment_index,
    }
}

#[test]
fn empty_profile_is_rejected() {
    let p = profile::Profile::<f64>::new(SketchPlane::xy(), vec![]);
    assert_eq!(err(&p), ProfileError::EmptyProfile);
}

#[test]
fn single_vertex_loop_fails_arity() {
    let p = profile(vec![chain(&[(0.0, 0.0, 1.0)])]);
    assert_eq!(
        err(&p),
        ProfileError::TooFewVertices {
            loop_index: 0,
            count: 1,
        }
    );
}

#[test]
fn repeated_vertex_is_a_degenerate_segment() {
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
    ])]);
    assert_eq!(err(&p), ProfileError::DegenerateSegment(sref(0, 1)));
}

#[test]
fn near_coincident_vertices_escalate() {
    // Vertices 5ε apart: inside the sliver band — a typed escalation
    // naming the distance predicate, not a guess either way.
    let eps = tol().eps();
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 5.0 * eps, 0.0),
        (0.0, 1.0, 0.0),
    ])]);
    match err(&p) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::Segment(sref(0, 1)));
            assert_eq!(source.predicate, Some("vertex_separation"));
        }
        other => panic!("expected escalation, got {other:?}"),
    }
}

#[test]
fn sliver_arc_bulge_escalates() {
    // A unit chord with sagitta 5ε: neither line nor sound arc.
    let eps = tol().eps();
    let p = profile(vec![chain(&[
        (0.0, 0.0, 10.0 * eps), // sagitta = L·b/2 = 5ε
        (1.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
        (0.0, 1.0, 0.0),
    ])]);
    match err(&p) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::Segment(sref(0, 0)));
            assert_eq!(source.predicate, Some("segment_straightness"));
        }
        other => panic!("expected escalation, got {other:?}"),
    }
}

/// **The re-homed fail-loud demo** (LIB-RETTAIL, Ev's ruling on
/// #413). The tour's coda used to build this loop and print the
/// refusal; a broken-on-purpose scene is not a use case, so the
/// contract moved here, where it can be asserted instead of narrated.
///
/// The contract, exactly as the demo stated it: the chain **authors
/// cleanly** — the PATHS lattice's junction checks are LOCAL, and all
/// four of the bowtie's corners are definitely sharp, so nothing
/// refuses at authoring time — and the **profile-level validator** is
/// what catches the crossing, typed, before any sweep can run. That
/// split is the whole point: a local check cannot see a global
/// self-intersection, which is why the validate gate exists.
///
/// The tour's own coordinates (the 2x2 scaling), so the geometry that
/// was demonstrated is the geometry that is pinned.
#[test]
fn the_bowtie_authors_cleanly_and_refuses_at_validation() {
    let authored = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(2.0, 2.0), Tol::witness())
        .and_then(|t| t.line_to(Point2::new(2.0, 0.0), Tol::witness()))
        .and_then(|t| t.line_to(Point2::new(0.0, 2.0), Tol::witness()))
        .and_then(|t| t.line_to(Start, Tol::witness()))
        .expect("every corner is sharp: the lattice's LOCAL checks pass a bowtie");

    let p = profile(vec![authored.into()]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 2),
            kind: ContactKind::Crossing,
        },
        "the validate gate must refuse the crossing typed — never Ok, never a panic"
    );
}

#[test]
fn bowtie_is_a_crossing() {
    let p = profile(vec![bowtie()]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 2),
            kind: ContactKind::Crossing,
        }
    );
}

#[test]
fn overlapping_loops_cross() {
    let p = profile(vec![rect(0.0, 0.0, 2.0, 2.0), rect(1.0, 1.0, 2.0, 2.0)]);
    match err(&p) {
        ProfileError::NonSimple {
            kind: ContactKind::Crossing,
            first,
            second,
        } => {
            assert_eq!(first.loop_index, 0);
            assert_eq!(second.loop_index, 1);
        }
        other => panic!("expected a crossing, got {other:?}"),
    }
}

#[test]
fn hole_crossing_outer_is_non_simple() {
    let p = profile(vec![rect(0.0, 0.0, 4.0, 4.0), rect(3.0, 1.0, 3.0, 1.0)]);
    match err(&p) {
        ProfileError::NonSimple {
            kind: ContactKind::Crossing,
            ..
        } => {}
        other => panic!("expected a crossing, got {other:?}"),
    }
}

#[test]
fn loops_touching_at_a_corner_are_non_simple() {
    let p = profile(vec![rect(0.0, 0.0, 2.0, 2.0), rect(2.0, 2.0, 1.0, 1.0)]);
    match err(&p) {
        ProfileError::NonSimple {
            kind: ContactKind::Touch,
            ..
        } => {}
        other => panic!("expected an endpoint contact, got {other:?}"),
    }
}

#[test]
fn vertex_on_segment_interior_is_non_simple() {
    // (1, 0) is a loop vertex lying on segment 0's interior.
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (2.0, 2.0, 0.0),
        (1.0, 0.0, 0.0),
    ])]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 2),
            kind: ContactKind::Touch,
        }
    );
}

#[test]
fn collinear_spike_is_an_overlap() {
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
    ])]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 1),
            kind: ContactKind::Overlap,
        }
    );
}

#[test]
fn two_line_digon_is_an_overlap() {
    let p = profile(vec![chain(&[(0.0, 0.0, 0.0), (2.0, 0.0, 0.0)])]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 1),
            kind: ContactKind::Overlap,
        }
    );
}

#[test]
fn doubled_arc_digon_is_an_overlap() {
    // Out along an arc and back along the same arc: identical spans on
    // one carrier (coincident apexes).
    let p = profile(vec![chain(&[(0.0, 0.0, 1.0), (2.0, 0.0, -1.0)])]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 1),
            kind: ContactKind::Overlap,
        }
    );
}

#[test]
fn hole_outside_outer_means_two_outer_loops() {
    let p = profile(vec![rect(0.0, 0.0, 2.0, 2.0), rect(5.0, 0.0, 1.0, 1.0)]);
    assert_eq!(
        err(&p),
        ProfileError::MultipleOuterLoops {
            outer_loops: vec![0, 1],
        }
    );
}

#[test]
fn nesting_deeper_than_holes_is_rejected() {
    let p = profile(vec![
        rect(0.0, 0.0, 10.0, 10.0),
        rect(1.0, 1.0, 8.0, 8.0),
        rect(2.0, 2.0, 6.0, 6.0),
    ]);
    assert_eq!(
        err(&p),
        ProfileError::NestingTooDeep {
            loop_index: 2,
            depth: 2,
        }
    );
}

#[test]
fn internally_tangent_hole_is_a_tangential_contact() {
    assert_eq!(
        err(&tangent_hole()),
        ProfileError::TangentialContact {
            first: sref(0, 0),
            second: sref(1, 0),
        }
    );
}

#[test]
fn arc_kissing_a_line_is_a_tangential_contact() {
    assert_eq!(
        err(&arc_kisses_line()),
        ProfileError::TangentialContact {
            first: sref(0, 0),
            second: sref(0, 3),
        }
    );
}

#[test]
fn near_tangent_hole_escalates_on_the_internal_clearance() {
    match err(&near_tangent_hole(tol().eps())) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::SegmentPair(sref(0, 0), sref(1, 0)));
            assert_eq!(source.predicate, Some("carrier_circles_internal"));
            match source.margin {
                MarginDiag::Value(m) => {
                    // The clearance is −5ε (up to the cancellation ulp).
                    let eps = tol().eps();
                    assert!(
                        (m + 5.0 * eps).abs() < 0.5 * eps,
                        "margin {m:e} should be ≈ −5ε"
                    );
                }
                other => panic!("expected an in-band value margin, got {other:?}"),
            }
        }
        other => panic!("expected escalation, got {other:?}"),
    }
}

#[test]
fn near_full_arc_is_a_typed_rejection() {
    // Half-gap delta sized so the diameter clearance 2r(1 - cos(delta/2))
    // ~ delta^2/4 lands at 0.5 eps: within tolerance of a full circle
    // (M2 PR 2 review SHOULD-2 fix). r = 1 keeps rounding noise (~1e-15)
    // far below every CI row's eps.
    let eps = tol().eps();
    let delta = (2.0 * eps).sqrt();
    let bulge = 1.0 / (delta / 2.0).tan();
    let (s, c) = delta.sin_cos();
    let p = profile(vec![chain(&[(c, s, bulge), (c, -s, 0.0)])]);
    assert_eq!(err(&p), ProfileError::NearFullArc(sref(0, 0)));
}

#[test]
fn almost_near_full_arc_escalates_on_diameter_clearance() {
    // Same construction with clearance ~ 5 eps: inside the ambiguity
    // band, so the gate escalates naming its predicate.
    let eps = tol().eps();
    let delta = (20.0 * eps).sqrt();
    let bulge = 1.0 / (delta / 2.0).tan();
    let (s, c) = delta.sin_cos();
    let p = profile(vec![chain(&[(c, s, bulge), (c, -s, 0.0)])]);
    match err(&p) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::Segment(sref(0, 0)));
            assert_eq!(source.predicate, Some("arc_diameter_clearance"));
        }
        other => panic!("expected diameter-clearance escalation, got {other:?}"),
    }
}

#[test]
fn nan_coordinates_poison_to_a_typed_error() {
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (f64::NAN, 0.0, 0.0),
        (1.0, 1.0, 0.0),
    ])]);
    match err(&p) {
        ProfileError::Escalated { source, .. } => {
            assert_eq!(source.margin, MarginDiag::Invalid);
        }
        other => panic!("expected poison escalation, got {other:?}"),
    }
}

#[test]
fn nan_bulge_poisons_to_a_typed_error() {
    let p = profile(vec![chain(&[
        (0.0, 0.0, f64::NAN),
        (1.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
    ])]);
    match err(&p) {
        ProfileError::Escalated { site, source } => {
            assert_eq!(site, EscalationSite::Segment(sref(0, 0)));
            assert_eq!(source.margin, MarginDiag::Invalid);
            assert_eq!(source.predicate, Some("segment_straightness"));
        }
        other => panic!("expected poison escalation, got {other:?}"),
    }
}

#[test]
fn infinite_coordinates_fail_loudly_without_panicking() {
    for bad in [f64::INFINITY, f64::NEG_INFINITY] {
        let p = profile(vec![chain(&[
            (0.0, 0.0, 0.0),
            (bad, 0.0, 0.0),
            (1.0, 1.0, 0.0),
        ])]);
        // Whatever the classification path, it must be a typed error,
        // never a panic and never an accept (∞ chords poison derived
        // quantities like the unit direction).
        assert!(p.validate(tol()).is_err());
    }
}

#[test]
fn garbage_fuzz_never_panics() {
    // A deterministic little garbage battery: special values in every
    // slot, including poison bulges on degenerate chains.
    let specials = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        f64::NAN,
        f64::INFINITY,
        1e300,
        -5e-324,
    ];
    for (i, &a) in specials.iter().enumerate() {
        for (j, &b) in specials.iter().enumerate() {
            let p = profile(vec![chain(&[
                (a, b, specials[(i + j) % specials.len()]),
                (b, a, 1.0),
                (1.0, 0.5, a),
            ])]);
            let _ = p.validate(tol());
        }
    }
}

#[test]
fn error_display_is_actionable() {
    // S6 (two-tolerance, D4 ¶1 addendum): the exactly-tangent and the
    // in-band refusals are one user situation — both carry the shared
    // recourse fragment, margins ride as payload.
    let e = err(&tangent_hole());
    let msg = e.to_string();
    assert!(msg.contains("tangential contact"), "{msg}");
    assert!(msg.contains("loop 0 segment 0"), "{msg}");
    assert!(msg.contains("loop 1 segment 0"), "{msg}");
    assert_eq!(
        msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
        1,
        "{msg}"
    );

    let e = err(&near_tangent_hole(tol().eps()));
    let msg = e.to_string();
    assert!(msg.contains("carrier_circles_internal"), "{msg}");
    assert!(msg.contains("ambiguity band"), "{msg}");
    assert_eq!(
        msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
        1,
        "{msg}"
    );
}

#[test]
fn cw_and_ccw_inputs_reject_identically() {
    // Winding is invisible to errors too: the bowtie reversed reports
    // the same crossing (segment indices mirror deterministically).
    let fwd = err(&profile(vec![bowtie()]));
    let rev = err(&profile(vec![bowtie().reversed()]));
    match (fwd, rev) {
        (ProfileError::NonSimple { kind: k1, .. }, ProfileError::NonSimple { kind: k2, .. }) => {
            assert_eq!(k1, ContactKind::Crossing);
            assert_eq!(k2, ContactKind::Crossing);
        }
        other => panic!("expected two crossings, got {other:?}"),
    }
}

#[test]
fn concentric_circles_of_close_radii_escalate_as_sliver_annulus() {
    // Radii 5ε apart, concentric: the loci sit inside each other's
    // sliver band everywhere — the identity margin escalates.
    let eps = tol().eps();
    let p = profile(vec![
        circle_h(0.0, 0.0, 1.0),
        circle_h(0.0, 0.0, 1.0 + 5.0 * eps),
    ]);
    match err(&p) {
        ProfileError::Escalated { source, .. } => {
            assert_eq!(source.predicate, Some("carrier_circles_identity"));
        }
        other => panic!("expected identity escalation, got {other:?}"),
    }
}

/// **The enclosing gate's IN-BAND arm, both halves.** ρ = R − σ·τ·r is a
/// length like any other gate's margin, so a fillet radius within the
/// ambiguity band of a leg's carrier radius leaves the class question
/// itself undecidable — and the kernel says so rather than picking a
/// side of it.
///
/// Two things are covered here because they are one situation rendered
/// at two doors: the authoring door escalates with the predicate named
/// (so a driver bisecting a parameter box knows which decision went
/// unconfirmed), and the fillet escalation's recourse rider carries the
/// same sentence its definite sibling
/// [`profile::PathError::FilletEnclosesLegCarrier`] carries (D4 ¶1 (iv)).
///
/// The second half constructs its error value rather than provoking it:
/// `EscalationSite::Fillet` has no producer in the kernel today (the
/// arc-carrier door reports its escalations as `PathError::Escalated`),
/// so the rider is a Display rule with no reachable input, and pinning
/// the rule is what can honestly be pinned about it.
#[test]
fn a_radius_within_the_band_of_a_carrier_radius_escalates_the_enclosing_gate() {
    let eps = tol().eps();
    // Two unit lobes whose crossing is a real corner, and a radius 5ε
    // above the lobe radius: rho = -5eps, inside (eps, K*eps).
    let tip = 0.75f64.sqrt();
    let err = Open
        .arc_fillet_arc(
            Center {
                c: Point2::new(-0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Point2::new(0.0, -tip),
            },
            1.0 + 5.0 * eps,
            Center {
                c: Point2::new(0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .expect_err("a radius inside the band of the carrier radius cannot be classified");
    match err {
        PathError::Escalated { source } => assert_eq!(
            source.predicate,
            Some("fillet_enclosing_carrier"),
            "the escalation must name the gate that could not be classified"
        ),
        other => panic!("expected an escalation, got {other:?}"),
    }
    // The rider: the in-band arm renders the same recourse as the
    // definite refusal, and — like that refusal — it endorses no radius
    // it cannot vouch for.
    let rendered = ProfileError::Escalated {
        site: EscalationSite::Fillet,
        source: Indeterminate {
            margin: MarginDiag::Value(-5.0 * eps),
            band: Band::new(eps, tol().k() * eps).expect("the run's band forms"),
            predicate: Some("fillet_enclosing_carrier"),
        },
    }
    .to_string();
    assert!(
        rendered.contains("puts that carrier INSIDE the fillet circle"),
        "the enclosing recourse is missing: {rendered}"
    );
    assert!(
        rendered.contains("expect to go well below it"),
        "the recourse must not endorse the class bound: {rendered}"
    );
}
