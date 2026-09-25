//! **Review probes for the must-carry rule over an edge** —
//! `geom_brep::must_carry_over_edge` on both sweep verbs.
//!
//! Two things the unit's own rows cannot tell apart:
//!
//! 1. A walk that READS the certification schedule's interior stations
//!    from one that reads its first station (or its midpoint) and calls
//!    the answer the edge's. Every join the two verbs mint has `κ_rel`
//!    constant along its carrier, so the verb-level rows pass either
//!    way. The rows below hand the rule a pair the certificate's lane
//!    admits whose `κ_rel` CHANGES SIGN along the carrier — two tori
//!    sharing a meridian circle — placed so the first station reads
//!    definitely positive and the second reads zero: the verdict is
//!    `UnderDetermined` only if the walk reaches the second station.
//! 2. The band-derived family, re-derived on different fixtures: a
//!    stadium extruded to a band-derived height, and a cylinder–torus
//!    ring revolved through a band-derived ANGLE so the fold's extent
//!    term, not a radius, is the arm. Both refuse typed at both ends
//!    of the band and store what the contract says on either side.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeDescription, MustCarryVerdict, must_carry_over_edge};
use geom_core::{Band, MarginDiag, Point2, Point3, Sign, Tol, Vec2, Vec3};
use profile::{Profile, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeError, Extrusion, Revolution, RevolveAxis, RevolveError, extrude, revolve};
use topo::Body;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's linear band")
}

// ---------------------------------------------------------------
// A lane-admitted pair whose κ_rel varies along the carrier.
// ---------------------------------------------------------------

/// Two tori of minor radius `r` whose spines both pass through
/// `(1, 0, 0)` with tangent `±y`: torus A (major 1, centre the
/// origin) and torus B (major 1/2, centre `(1/2, 0, 0)`), axes both
/// `z`. They share the meridian circle centred `(1, 0, 0)` in the
/// `xz`-plane and are tangent along all of it. Along that circle the
/// transverse direction is the spine tangent, and each torus's normal
/// curvature there is `cos v / (R + r·cos v)`, so
/// `κ_rel = cos v · (1/(1 + r cos v) − 1/(1/2 + r cos v))`: definitely
/// nonzero at `v = π/4`, exactly zero at `v = π/2`.
fn meridian_sharing_tori() -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let r = 0.25;
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    let a = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: z,
        major_radius: 1.0,
        minor_radius: r,
        u_ref: x,
    };
    let b = Surface::Torus {
        center: Point3::new(0.5, 0.0, 0.0),
        axis: z,
        major_radius: 0.5,
        minor_radius: r,
        u_ref: x,
    };
    // θ = 0 at the outer equator point (1 + r, 0, 0); station i of the
    // schedule sits at θ = i·π/4.
    let carrier = Curve3::Circle {
        center: Point3::new(1.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: r,
        u_ref: x,
    };
    (a, b, carrier)
}

const TAU: f64 = core::f64::consts::TAU;

/// **C5 / C3.** The lane admits a pair whose `κ_rel` is not constant
/// along the carrier, and on it the rule's verdict is decided by the
/// second station, not the first: a walk that read one station — the
/// first or the midpoint — would answer `JetDeterminate` here.
#[test]
fn kappa_rel_varies_along_a_lane_admitted_carrier_and_the_second_station_decides() {
    let (a, b, carrier) = meridian_sharing_tori();
    assert!(
        geom_brep::tangent_certificate_lane(&carrier, &a, &b),
        "a Circle carrier on a torus pair is inside the lane"
    );
    let extent = 2.0 * 0.25; // a full circle's extent is its diameter
    let at = |i: u32| {
        let t = geom_brep::sample_param(0.0, TAU, i);
        geom_brep::tangent_second_order(&a, &b, carrier.eval(t), carrier.deriv(t), extent, band())
    };
    let (s1, s2, s4) = (at(1), at(2), at(4));
    assert_eq!(
        s1.verdict,
        Ok(Sign::Positive),
        "station 1 is definite: {s1:?}"
    );
    assert_eq!(
        s2.verdict,
        Ok(Sign::Zero),
        "station 2 is the zero of κ_rel: {s2:?}"
    );
    assert_eq!(
        s4.verdict,
        Ok(Sign::Positive),
        "the midpoint station is definite: {s4:?}"
    );
    assert!(
        (s1.jet.kappa_rel - s4.jet.kappa_rel).abs() > 1.0,
        "κ_rel is not constant along this carrier: {} at station 1, {} at station 4",
        s1.jet.kappa_rel,
        s4.jet.kappa_rel
    );

    let answer = must_carry_over_edge(&a, &b, &carrier, 0.0, TAU, extent, band());
    assert_eq!(
        answer,
        MustCarryVerdict::UnderDetermined,
        "one zero-side station denies determinacy for the whole edge"
    );
    // The whole answer is the verdict. Station 1 read `Positive` here
    // (asserted above), so a rule that answered from the first station
    // alone — or from any one station — would call this edge
    // jet-determinate and store a description tier 3 refuses.
}

/// **C3 / C4, the K stream.** An under-determined edge spends as many
/// samples as the index of the station that decides it — here two —
/// not the one the PR's table states for the fixture where the first
/// station happens to decide.
#[cfg(feature = "probe")]
#[test]
fn an_edge_decided_at_the_second_station_spends_two_samples() {
    use geom_core::k_stats::{self, Probe};
    let pr = Probe;
    let pt = |x: f64, y: f64, z: f64| Point3::new(pr(x), pr(y), pr(z));
    let v = |x: f64, y: f64, z: f64| Vec3::new(pr(x), pr(y), pr(z));
    let a = Surface::Torus {
        center: pt(0.0, 0.0, 0.0),
        axis: v(0.0, 0.0, 1.0),
        major_radius: pr(1.0),
        minor_radius: pr(0.25),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let b = Surface::Torus {
        center: pt(0.5, 0.0, 0.0),
        axis: v(0.0, 0.0, 1.0),
        major_radius: pr(0.5),
        minor_radius: pr(0.25),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let carrier = Curve3::Circle {
        center: pt(1.0, 0.0, 0.0),
        axis: v(0.0, 1.0, 0.0),
        radius: pr(0.25),
        u_ref: v(1.0, 0.0, 0.0),
    };
    k_stats::start_recording();
    let answer = must_carry_over_edge(&a, &b, &carrier, pr(0.0), pr(TAU), pr(0.5), band());
    let spent = k_stats::take_samples()
        .iter()
        .filter(|s| s.predicate == "tangent_second_order")
        .count();
    assert_eq!(answer, MustCarryVerdict::UnderDetermined);
    assert_eq!(
        spent, 2,
        "the walk reads station 1 (Positive) and station 2 (Zero), then stops"
    );
}

// ---------------------------------------------------------------
// The band-derived family, on the reviewer's own fixtures.
// ---------------------------------------------------------------

/// The sagitta `|κ_rel|·arm²/2` with `|κ_rel| = 1/r` and the arm the
/// free length, inverted.
fn arm_for(margin: f64, r: f64) -> f64 {
    (2.0 * r * margin).sqrt()
}

/// Two margins at the two ends of the band, each strictly inside it.
fn in_band_margins() -> [f64; 2] {
    let b = band();
    [2.0 * b.zero(), b.escalate() / 2.0]
}

fn assert_in_band(source: geom_core::Indeterminate) {
    assert_eq!(source.predicate, Some("tangent_second_order"));
    let b = band();
    match source.margin {
        MarginDiag::Value(m) => assert!(
            m.abs() > b.zero() && m.abs() < b.escalate(),
            "margin {m:e} outside ({:e}, {:e})",
            b.zero(),
            b.escalate()
        ),
        other => panic!("expected a value margin, got {other:?}"),
    }
}

fn tangent_intersections(body: &Body<f64>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(geom_brep::EdgeCurve::description),
                Some(EdgeDescription::TangentIntersection { .. })
            )
        })
        .count()
}

const STADIUM_R: f64 = 0.4;

/// A stadium — two semicircles of radius [`STADIUM_R`] joined by two
/// lines — extruded to height `h`: four smooth plane–cylinder struts
/// whose fold is `min(∞, r, h)`.
fn stadium(h: f64) -> Result<Body<f64>, ExtrudeError> {
    let r = STADIUM_R;
    let p2 = Point2::<f64>::new;
    // A semicircle's bulge is tan(π/4) = 1.
    let lp = bulge_loop(vec![
        (p2(0.0, -r), 0.0),
        (p2(2.0, -r), 1.0),
        (p2(2.0, r), 0.0),
        (p2(0.0, r), 1.0),
    ])
    .with_tangent_joints(vec![0, 1, 2, 3]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a valid stadium");
    extrude(&profile, Extrusion::Distance(h), Tol::witness()).map(|e| e.body)
}

/// **C2, extrude.** Both ends of the band refuse typed under the
/// rule's predicate; either definite side stores what the contract
/// says on all four struts.
#[test]
fn a_stadium_strut_refuses_at_both_ends_of_the_band_and_stores_either_definite_side() {
    for margin in in_band_margins() {
        match stadium(arm_for(margin, STADIUM_R)) {
            Err(ExtrudeError::SliverJoin { source, .. }) => assert_in_band(source),
            other => panic!("margin {margin:e} must refuse as a sliver join: {other:?}"),
        }
    }
    let positive = stadium(arm_for(STADIUM_R / 4.0, STADIUM_R)).expect("definite builds");
    assert_eq!(tangent_intersections(&positive), 4);
    let zero = stadium(arm_for(band().zero() / 50.0, STADIUM_R)).expect("under-determined builds");
    assert_eq!(tangent_intersections(&zero), 0);
}

const LIP_R: f64 = 0.4;

/// A ring — bore radius 0.2, outer cylinder radius 1, a 45° torus
/// lip of minor radius [`LIP_R`] (major 0.6) tangent to the outer
/// cylinder at the torus's OUTER equator — revolved through the angle
/// whose latitude-arc chord is `chord`. The join's fold is
/// `min(1, r, extent)` with `extent ≈ chord` for a small angle, so the
/// ANGLE, not a radius, is what puts the sagitta in the band.
fn lipped_ring(chord: f64) -> Result<Body<f64>, RevolveError> {
    let r = LIP_R;
    let p2 = Point2::<f64>::new;
    // A 45° arc (bulge tan(π/16)) from the outer equator, so its far
    // end meets the top annulus at a CORNER: one smooth join only.
    let bulge = (core::f64::consts::FRAC_PI_4 / 4.0).tan();
    let c = core::f64::consts::FRAC_1_SQRT_2;
    let lp = bulge_loop(vec![
        (p2(0.2, -0.5), 0.0),
        (p2(1.0, -0.5), 0.0),
        (p2(1.0, 0.0), bulge),
        (p2(1.0 - r + r * c, r * c), 0.0),
        (p2(0.2, r * c), 0.0),
    ])
    .with_tangent_joints(vec![2]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a valid lipped ring");
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let theta = 2.0 * (chord / 2.0).asin();
    revolve(&profile, axis, Revolution::Partial(theta), Tol::witness()).map(|r| r.body)
}

/// **C2, revolve.** The same family with the fold's EXTENT term as
/// the arm: both ends of the band refuse `RevolveError::SliverJoin`
/// under the rule's predicate; a definite angle stores the intrinsic
/// tangency on the one smooth latitude arc.
#[test]
fn a_lipped_ring_refuses_at_both_ends_of_the_band_and_stores_the_definite_side() {
    for margin in in_band_margins() {
        match lipped_ring(arm_for(margin, LIP_R)) {
            Err(RevolveError::SliverJoin { source, .. }) => assert_in_band(source),
            other => panic!("margin {margin:e} must refuse as a sliver join: {other:?}"),
        }
    }
    // A wide angle: the fold saturates at the lip radius, margin r/2.
    let positive = lipped_ring(2.0 * (0.25f64).sin()).expect("definite builds");
    assert_eq!(tangent_intersections(&positive), 1);
    let zero = lipped_ring(arm_for(band().zero() / 50.0, LIP_R)).expect("under-determined builds");
    assert_eq!(tangent_intersections(&zero), 0);
}
