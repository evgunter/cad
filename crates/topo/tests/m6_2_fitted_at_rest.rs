//! **M6-2 acceptance: a rung-3 edge in a body AT REST** — the row M5
//! PR 9's spec asked for, the M5 exit walk carried as row 2, and the SSI
//! generic-`T` lift's own acceptance obligation.
//!
//! The cylinder×sphere fixture's small loop of the kernel's own
//! traced-and-fitted branch, restricted to an edge carrier, certified
//! into a body, and minted. Its two halves of the between-samples
//! statement live where C2 and C4 put them:
//!
//! - **the edge's certificate** holds the uniqueness tube (C2's limb 3):
//!   over a chain of boxes around the carrier the pair's crossing is one
//!   arc spanning it. `mev` runs it through the scalar's lane, and tier
//!   3 re-derives it;
//! - **the face's rows** are the projected image, the cylinder chart's
//!   inverse applied to the carrier (C4), whose envelope bounds
//!   `|S(P(t)) − C(t)|` over the whole span — the carrier's distance from
//!   the chart included — and carry no pair certificate.
//!
//! Stated at both scalars the lift was for: **`f64`** (the outer rows)
//! and **`Interval`** (the `certified` module), enclosure-style.
//!
//! **ε posture.** No ε literal appears here. Every margin is compared
//! against the run's own resolved band.
//!
//! **What this row does NOT do**: it does not wire the cyl×sphere JOIN
//! lane (banked past M6). The edge is built through the public
//! certification doors.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use geom_brep::{EnvelopeStatement, Pcurve};
use geom_core::Band;
use geom_core::Tol;

/// The full at-rest run at `f64`: build, read the minted rows, run the
/// edge's tube, validate.
#[test]
fn a_rung3_edge_at_rest_carries_its_projected_rows_and_its_edge_the_tube() {
    let built = fixture::build::<f64>();
    let band = Band::linear(Tol::witness()).unwrap();
    for he in [built.he_plus, built.he_minus] {
        let cache = built
            .body
            .pcurve(he)
            .expect("both half-edges carry a minted row");
        assert!(
            matches!(cache.pcurve(), Pcurve::Projected(_)),
            "ROW: the stored image is the projected one: {cache:?}"
        );
        let cert = cache.certificate();
        assert_eq!(cert.statement, EnvelopeStatement::MapResidualProjected);
        assert!(
            cert.ssi().is_none(),
            "ROW: the row carries no pair certificate; the tube is the edge's"
        );
        assert!(
            cert.envelope <= band.zero(),
            "ROW: certified sup bound within ε"
        );
    }
    // The tube is the edge's: it certifies on the operand pair, and the
    // same carrier against a degenerate pair (its own surface twice,
    // nowhere transverse) refuses — the limb is live, not vacuous.
    geom_brep::rung3_tube(&built.carrier, &built.cylinder, &built.sphere, band)
        .expect("TUBE: the pair's crossing is one arc spanning the carrier");
    assert!(
        geom_brep::rung3_tube(&built.carrier, &built.cylinder, &built.cylinder, band).is_err(),
        "TUBE: a pair that crosses nowhere has no tube"
    );
    let findings = topo::pcurves::validate_pcurves(&built.body, band);
    assert!(findings.is_empty(), "AT-REST: {findings:?}");
    // The edge's own re-certification with the lane in hand runs the
    // tube; with a degenerate mate swapped into its description it would
    // refuse there.
    let ek = built.body.get_half_edge(built.he_plus).unwrap().edge;
    let edge = built.body.get_edge(ek).unwrap();
    let Some(topo::CurveGeom::Certified(curve)) = built.body.get_curve_geom(edge.curve) else {
        panic!("the rung-3 edge is certified")
    };
    let point = |he| {
        let v = built.body.get_half_edge(he).unwrap().start;
        *built
            .body
            .get_point(built.body.get_vertex(v).unwrap().point)
            .unwrap()
    };
    curve
        .recertify_via(
            point(built.he_plus),
            point(built.he_minus),
            |k| built.body.get_surface(k).cloned(),
            band,
            Some(geom_brep::NurbsLane::certified()),
        )
        .expect("AT-REST: the edge re-certifies with its tube");
}

/// **A producer's closing mint re-derives the row.** The mint has a
/// route to this face's rung-3 carrier (the projected image), so a
/// second mint and a rigid map — which ends with that mint — leave
/// projected rows, the same bits for the mint, and tier 3 reads the body
/// clean.
#[test]
fn a_producers_closing_mint_re_derives_the_projected_row() {
    let built = fixture::build::<f64>();
    let band = Band::linear(Tol::witness()).unwrap();
    let image = |b: &topo::Body<f64>, he| {
        b.pcurve(he)
            .map(|c| format!("{:?} {:?}", c.params(), c.pcurve()))
    };
    let mut minted = built.body.clone();
    topo::mint_pcurves(&mut minted, Tol::witness()).unwrap();
    for he in [built.he_plus, built.he_minus] {
        assert!(image(&built.body, he).is_some());
        assert_eq!(
            image(&minted, he),
            image(&built.body, he),
            "the mint is idempotent on the row"
        );
    }
    let moved = topo::transform_rigid(
        &built.body,
        &geom_core::Affine3::translation(geom_core::Vec3::new(0.25, -0.5, 1.0)),
        Tol::witness(),
    )
    .unwrap();
    for he in [built.he_plus, built.he_minus] {
        let carried = moved.pcurve(he).expect("the mapped body keeps the row");
        assert!(matches!(carried.pcurve(), Pcurve::Projected(_)));
    }
    let findings = topo::pcurves::validate_pcurves(&moved, band);
    assert!(findings.is_empty(), "{findings:?}");
}

/// **A split restricts the projected row.** Each child keeps the
/// parent's image (the same net, pieces and branch centres) over its
/// own sub-interval, re-certified through the door that minted it, and
/// tier 3 reads the split body clean.
#[test]
fn a_split_restricts_the_projected_row_to_each_child() {
    let built = fixture::build::<f64>();
    let band = Band::linear(Tol::witness()).unwrap();
    let parent = |he| {
        let c = built.body.pcurve(he).expect("the parent carries its row");
        (c.params(), format!("{:?}", c.pcurve()))
    };
    let (plus, minus) = (parent(built.he_plus), parent(built.he_minus));
    let (t0, t1) = plus.0;
    let t = 0.5 * (t0 + t1);
    let mut body = built.body.clone();
    let ek = body.get_half_edge(built.he_plus).unwrap().edge;
    let split = body
        .split_edge(ek, t, Tol::witness())
        .expect("the split carries the rows");
    for (he, image, span) in [
        (built.he_plus, &plus.1, (t0, t)),
        (split.he_plus, &plus.1, (t, t1)),
        (split.he_minus, &minus.1, (t, t1)),
        (built.he_minus, &minus.1, (t0, t)),
    ] {
        let cache = body.pcurve(he).expect("every child half carries a row");
        assert!(
            matches!(cache.pcurve(), Pcurve::Projected(_)),
            "{he:?}: {cache:?}"
        );
        assert_eq!(
            &format!("{:?}", cache.pcurve()),
            image,
            "{he:?}: the parent's image, restricted"
        );
        assert_eq!(cache.params(), span, "{he:?}: the child's own interval");
        assert!(cache.certificate().envelope <= band.zero(), "{he:?}");
    }
    let findings = topo::pcurves::validate_pcurves(&body, band);
    assert!(findings.is_empty(), "AT-REST after the split: {findings:?}");
}

/// **The `Dual` lane leaves the face rowless, and the refusal says
/// why.** A net's projected row reads its hull terms through the fitted
/// door, which a dual does not hold (D1, 2026-08-19: certification
/// arithmetic, C9, is not reachable from it), so the dual's mint excuses
/// the face and tier 3 reports nothing about it. At `f64`, the same row
/// offered with the door withheld refuses at check 4 naming `f64`, and
/// its text does not deny `f64` the right it has; the dual's own refusal
/// names every scalar whose fitted door answers `Some`.
#[test]
fn the_dual_leaves_the_face_rowless_and_says_why() {
    use geom_brep::{PcurveCache, PcurveCertifyError};
    use geom_core::{Dual64, Real};
    use topo::AtRestPolicy;
    let dual = fixture::build::<Dual64>();
    let band = Band::linear(Tol::witness()).unwrap();
    for he in [dual.he_plus, dual.he_minus] {
        assert!(
            dual.body.pcurve(he).is_none(),
            "DUAL: a dual mints no row whose certificate needs certification arithmetic"
        );
    }
    let findings = topo::pcurves::validate_pcurves(&dual.body, band);
    assert!(
        findings.is_empty(),
        "DUAL: the face is not owed rows: {findings:?}"
    );

    let built = fixture::build::<f64>();
    let carrier = geom::Curve3::Nurbs(std::sync::Arc::clone(&built.carrier));
    let Pcurve::Projected(image) = built.body.pcurve(built.he_plus).unwrap().pcurve().clone()
    else {
        panic!("the minted row is projected")
    };
    let (t0, t1) = built.carrier.domain();
    let err = PcurveCache::certify_projected(*image, t0, t1, &carrier, &built.cylinder, band, None)
        .expect_err("WITHHELD: a net's projected row with no door refuses");
    assert!(
        matches!(
            err,
            PcurveCertifyError::FittedLaneUnsupported { scalar: "f64" }
        ),
        "WITHHELD: the refusal is the fitted-lane one, naming the scalar: {err:?}"
    );
    let withheld = format!("{err}");
    assert!(
        withheld.contains("f64 scalar") && !withheld.contains("may not certify"),
        "TEXT: the refusal names f64 and does not deny it the right it has: {withheld}"
    );
    let msg = format!(
        "{}",
        PcurveCertifyError::FittedLaneUnsupported {
            scalar: <Dual64 as Real>::NAME
        }
    );
    assert!(
        msg.contains("dual scalar") && msg.contains("certification rights"),
        "TEXT: the refusal names the scalar and who holds the door: {msg}"
    );
    fn replay_list_names<T: AtRestPolicy>(msg: &str) {
        let name = T::NAME;
        assert!(
            T::fitted_lane().is_some(),
            "TEXT: {name} is checked against the replay list, so it must hold the door"
        );
        assert!(
            msg.contains(name),
            "TEXT: the refusal's replay list omits {name}, whose fitted door answers Some: {msg}"
        );
    }
    replay_list_names::<f64>(&msg);
    replay_list_names::<geom_core::interval::Interval>(&msg);
    replay_list_names::<geom_core::Sym<f64>>(&msg);
}

/// ε is never a literal here; this row states what the file relies on.
#[test]
fn the_band_is_the_runs_own() {
    let band = Band::linear(Tol::witness()).unwrap();
    assert_eq!(band.zero(), Tol::witness().get().eps);
}

// ==================================================================
// The interval lane — the evidence that the lift happened
// ==================================================================

mod certified {
    use super::fixture;
    use geom_brep::{EnvelopeStatement, Pcurve};
    use geom_core::Tol;
    use geom_core::{Band, Bounds, Interval};

    /// The same body at the interval scalar: the rows are minted there,
    /// their envelope is an enclosure inside the band, the edge's tube
    /// certifies on the lifted pair, and the at-rest pass re-derives it
    /// all. Each assertion names its property.
    #[test]
    fn the_projected_rows_and_the_tube_certify_at_the_interval_scalar() {
        let built = fixture::build::<Interval>();
        let band = Band::linear(Tol::witness()).unwrap();
        for he in [built.he_plus, built.he_minus] {
            let cache = built
                .body
                .pcurve(he)
                .expect("INTERVAL: the interval body carries the minted row");
            assert!(
                matches!(cache.pcurve(), Pcurve::Projected(_)),
                "INTERVAL: the stored image is the projected one"
            );
            let cert = cache.certificate();
            assert_eq!(cert.statement, EnvelopeStatement::MapResidualProjected);
            let env = cert.envelope;
            assert!(env.lo() <= env.hi(), "ENCLOSURE: a well-formed enclosure");
            assert!(
                env.hi() <= band.zero(),
                "ENCLOSURE: the whole enclosure is within ε ({:e})",
                env.hi()
            );
        }
        geom_brep::rung3_tube(&built.carrier, &built.cylinder, &built.sphere, band)
            .expect("TUBE: the lifted pair's crossing is one arc spanning the carrier");
        let findings = topo::pcurves::validate_pcurves(&built.body, band);
        assert!(findings.is_empty(), "AT-REST: {findings:?}");
    }
}
