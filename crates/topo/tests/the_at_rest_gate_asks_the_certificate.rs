//! **The at-rest gate asks the certificate** (`[topology.tiers]`): every
//! certifying scalar's finish asks the body-level injectivity question
//! after tier 3, a crossing refuses as
//! [`ValidationError::BoundaryMeetsItself`] and an engine refusal as
//! [`ValidationError::EmbeddingUncertified`], never `Ok`; the question
//! never offers a face pair its records cite
//! ([`ContactRecords::face_pairs`]).

#![allow(clippy::expect_used, clippy::panic)]
use geom_core::{Band, Tol};
use topo::clearance::{BlockingWidth, Embedding, FacePairs, GATE_BUDGETS, Side, embedded};
use topo::test_support::brick;
use topo::{AtRestPolicy, Body, ContactRecords, ValidationError};

/// Two unit bricks resting face to face, one body of two solids, and
/// their contacts as the census records them.
fn resting() -> (Body<f64>, ContactRecords) {
    let tol = Tol::witness();
    let mut body: Body<f64> = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let other: Body<f64> = brick((1.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol);
    let second = topo::graft_disjoint(&mut body, &other).expect("the bricks graft");
    let recorded = topo::record_at_rest(
        &body,
        &ContactRecords::default(),
        |records| {
            topo::validate_pseudomanifold(&body, records, tol)
                .err()
                .unwrap_or_default()
        },
        &|solid| Some(u32::from(solid == second)),
        0,
    );
    assert!(recorded.findings.is_empty(), "{:?}", recorded.findings);
    assert!(!recorded.records.is_empty(), "the bricks touch");
    (body, recorded.records)
}

/// **The engine never offers a recorded pair**: asked of the resting
/// bricks with no exclusion, the strict question finds them crossing
/// at their shared face; asked off their records' face pairs, it holds.
#[test]
fn two_resting_bricks_cross_at_their_contact_and_hold_off_it() {
    let (body, records) = resting();
    let band = Band::linear(Tol::witness()).expect("the witness tolerance has a band");
    let ask = |excluded| {
        embedded(
            &Side {
                body: &body,
                faces: body.faces().map(|(k, _)| k).collect(),
                excluded,
            },
            band,
            GATE_BUDGETS,
        )
        .verdict
    };
    let Embedding::Crosses(crossing) = ask(FacePairs::none()) else {
        panic!(
            "the bricks cross where they rest: {:?}",
            ask(FacePairs::none())
        )
    };
    let excluded = records.face_pairs(&body);
    assert!(
        excluded.holds(crossing.faces.0, crossing.faces.1),
        "the crossing's faces are a recorded pair: {crossing:?}"
    );
    assert_eq!(ask(excluded), Embedding::Embedded, "off its records");
}

/// **A crossing refuses the finish** as
/// [`ValidationError::BoundaryMeetsItself`], its witness read at the
/// body's own scalar: the resting bricks, finished with no records.
#[test]
fn a_crossing_refuses_the_finish_naming_its_faces() {
    let (body, _) = resting();
    let findings = f64::finish(body, Tol::witness()).expect_err("the bricks cross");
    assert!(
        matches!(
            findings.as_slice(),
            [ValidationError::BoundaryMeetsItself { faces: [a, b], .. }] if a != b
        ),
        "{findings:?}"
    );
}

/// **An engine refusal reads `EmbeddingUncertified`, never `Ok`**: a
/// slab `(1 + K)·ε` thick, its faces one ε past the band apart, which
/// the departure arm cannot read at a top edge.
#[test]
fn a_slab_one_epsilon_past_the_band_refuses_uncertified() {
    let tol = Tol::witness();
    let dz = (1.0 + tol.k()) * tol.eps();
    let slab: Body<f64> = brick((0.0, 1.0), (0.0, 0.5), (0.0, dz), tol);
    let findings = f64::finish(slab, tol).expect_err("the engine refuses the slab");
    assert!(
        matches!(
            findings.as_slice(),
            [ValidationError::EmbeddingUncertified {
                width: BlockingWidth::Cell,
                ..
            }]
        ),
        "{findings:?}"
    );
}
