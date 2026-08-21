//! S6 adversarial review probe (e2e): trigger the boolean coincidence
//! pair and the census pair through REAL API calls, render the full
//! Display messages, and assert the shared recourse carrier appears
//! exactly once per message with margin payload intact and no Debug
//! leakage. Adopted from the
//! reviewer's probe set in the S6 fix pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use geom_core::{COINCIDENCE_RECOURSE, Decide};
use topo::{
    Body, BooleanError, BooleanOp, ContactRecords, ValidationError, boolean_reduce,
    validate_pseudomanifold,
};
use geom_core::Tol;

fn brick<T: Decide + geom_core::Bounds>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// The recourse a message must carry exactly once. Contact-tier
/// findings carry the TWO-arm contact menu (SELECT-DESIGN §3d,
/// ratified: declare the named class / move the geometry); every other
/// site keeps the three-arm decidability sentence, whose "lower the
/// tolerance" arm is meaningful there and meaningless at a contact.
fn recourse_for(e: &ValidationError) -> &'static str {
    match e {
        ValidationError::UndeclaredContact { .. } | ValidationError::ContactContradicted { .. } => {
            topo::CONTACT_RECOURSE
        }
        _ => COINCIDENCE_RECOURSE,
    }
}

fn assert_unified(msg: &str, recourse: &str) {
    assert_eq!(
        msg.matches(recourse).count(),
        1,
        "carrier must appear exactly once: {msg}"
    );
    assert!(!msg.contains("MarginDiag"), "Debug leakage: {msg}");
    assert!(!msg.contains("Value("), "Debug leakage: {msg}");
    assert!(!msg.contains("Indeterminate {"), "Debug leakage: {msg}");
}

/// Pair 2 e2e: two bricks whose faces coincide exactly (flush stack,
/// independent recipe sources, NO declaration) and an in-band gap —
/// both must refuse with ONE unified message carrying the recourse
/// exactly once.
#[test]
fn probe_boolean_coincidence_pair_e2e() {
    // Exactly-on: b sits flush on a (shared plane z = 1), undeclared.
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let b = brick::<f64>((0.5, 1.5), (0.5, 1.5), (1.0, 2.0));
    let err =
        boolean_reduce(BooleanOp::Union, &a, &b, Tol::witness()).expect_err("undeclared flush contact must refuse");
    let msg = err.to_string();
    eprintln!("[probe] boolean exactly-on:\n  {msg}\n");
    assert!(
        matches!(err, BooleanError::UndeclaredCoincidence { .. }),
        "{err:?}"
    );
    assert_unified(&msg, COINCIDENCE_RECOURSE);

    // In-band: corner gap of 3 eps (inside the sliver band).
    let eps = geom_core::Tol::witness().get().eps;
    let g = 1.0 + 3.0 * eps;
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<f64>((g, 2.0), (g, 2.0), (g, 2.0));
    let err = boolean_reduce(BooleanOp::Union, &a, &b, Tol::witness()).expect_err("in-band gap must escalate");
    let msg = err.to_string();
    eprintln!("[probe] boolean in-band:\n  {msg}\n");
    assert_unified(&msg, COINCIDENCE_RECOURSE);
    assert!(
        msg.contains("ambiguity band") || msg.contains("cannot be classified"),
        "margin payload must survive: {msg}"
    );
}

/// Pair 3 e2e: census gap — a notch apex touching (delta = 0) and
/// in-band above (delta inside the band) the bottom edge, validated
/// with no declared contacts. Both arms unified, recourse once, and
/// the CensusEscalated arm must NOT show `{:?}` Debug leakage (the S6
/// census bug).
#[test]
fn probe_census_pair_e2e() {
    let scenario = |delta: f64| {
        let fx = prism_z::<f64>(
            &[
                (0.0, 0.0),
                (4.0, 0.0),
                (4.0, 2.0),
                (3.0, 2.0),
                (2.0, delta),
                (1.0, 2.0),
                (0.0, 2.0),
            ],
            0.0,
            1.0,
        );
        validate_pseudomanifold(&fx.body, &ContactRecords::default(), Tol::witness())
            .expect_err("touch/near-touch must be loud")
    };
    let mut saw_contact = false;
    let mut saw_escalated = false;
    let eps = geom_core::Tol::witness().get().eps;
    for delta in [0.0, 3.0 * eps] {
        for e in scenario(delta) {
            let msg = e.to_string();
            match e {
                ValidationError::UndeclaredContact { .. } => {
                    saw_contact = true;
                    eprintln!("[probe] census definite (delta={delta}):\n  {msg}\n");
                    assert_unified(&msg, recourse_for(&e));
                }
                ValidationError::CensusEscalated { .. } => {
                    saw_escalated = true;
                    eprintln!("[probe] census escalated (delta={delta}):\n  {msg}\n");
                    assert_unified(&msg, recourse_for(&e));
                    assert!(
                        msg.contains("ambiguity band") || msg.contains("cannot be classified"),
                        "margin payload must survive: {msg}"
                    );
                }
                other => panic!("unexpected error class: {other:?}"),
            }
        }
    }
    assert!(saw_contact, "fixtures never hit UndeclaredContact");
    assert!(saw_escalated, "fixtures never hit CensusEscalated");
}
