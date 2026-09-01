//! R2 review probes for MATE-7a (PR #1477). End-to-end measurements of
//! the shared-rim routing on inputs the unit's own fixtures do not
//! cover. Not the unit's rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use sweep::{TubeWindow, tube_along_arc};
use topo::{Body, BooleanDeclarations, BooleanError, ContactClass, FaceKey, FacePairDeclaration};

const TUBE: f64 = 0.06;
const RING: f64 = 5.0;

fn full_torus(major: f64) -> Body<f64> {
    tube_along_arc(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        major,
        TubeWindow::Full,
        TUBE,
        Tol::witness(),
    )
    .expect("the full torus builds")
    .body
}

fn torus_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Torus { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

fn decls(a: &Body<f64>, b: &Body<f64>, class: ContactClass) -> BooleanDeclarations {
    let mut d = BooleanDeclarations::none();
    for &fa in &torus_faces(a) {
        for &fb in &torus_faces(b) {
            d.coincident_faces
                .push(FacePairDeclaration::new(fa, fb, class));
        }
    }
    d
}

/// **The rim IDENTIFICATION swallows its own escalation.**
///
/// `rim_wedge::shared_rim` decides each of its three margins with
/// `matches!(decide(..), Ok(Sign::Zero))`, so an `Err(Indeterminate)`
/// — the sliver band — is indistinguishable from "these are different
/// circles". The module's own header says "anything the samples cannot
/// settle — escalates, naming the predicate that failed to decide.
/// Never a silent verdict"; that promise covers `classify_shared_rim`
/// and not the identification in front of it.
///
/// Measured: the kissing pair with the outer torus's ring radius
/// perturbed INTO the band. The rim is the same rim to within the band,
/// and the routing went silent, handing back the bare class refusal as
/// though the geometry had been examined and cleared.
///
/// **The perturbation is derived from the RUN's band, not written as a
/// literal.** R2 measured it as `5e-9`, which sits between `zero` and
/// `escalate` at the witness ε and nowhere else: at ε = 1e-6 the same
/// number is far below `zero`, the two rims read as ONE, and the row
/// asserted an escalation that correctly never came. A row that only
/// means what it says at one ε is not an ε row — so the offset is
/// `(zero + escalate) / 2`, which is in-band at every ε the matrix
/// runs. Same fixture, same claim, stated so it survives the sweep.
///
/// **INVERTED at the fix pass.** `shared_rim` now propagates the
/// `Indeterminate` instead of discarding it, so the perturbed pair
/// escalates typed. The fixture and the perturbation are R2's; the
/// expectation moved because the swallowing was fixed. The row is also
/// STRONGER than the one it replaces: it pins that the two outcomes
/// differ AND that the in-band one is an escalation, so neither a
/// re-swallowed error nor a spurious escalation on the exact pair can
/// pass it.
#[test]
fn r2_an_in_band_rim_identification_escalates_rather_than_reading_as_no_rim() {
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's linear band");
    let in_band_offset = (band.zero() + band.escalate()) * 0.5;
    let a = full_torus(RING);
    let exact = full_torus(RING + 2.0 * TUBE);
    let perturbed = full_torus(RING + 2.0 * TUBE + in_band_offset);

    let d_exact = decls(&a, &exact, ContactClass::Tangent);
    let e_exact = topo::union_with(&a, &exact, &d_exact, Tol::witness()).expect_err("refuses");
    println!("[r2] exact kiss: {e_exact:?}");
    assert!(
        matches!(e_exact, BooleanError::RimCuspArmUnbuilt { .. }),
        "the unit's own row: the exact kiss routes to the cusp family: {e_exact:?}"
    );

    let d_pert = decls(&a, &perturbed, ContactClass::Tangent);
    let e_pert = topo::union_with(&a, &perturbed, &d_pert, Tol::witness()).expect_err("refuses");
    println!("[r2] in-band-perturbed kiss: {e_pert:?}");
    match e_pert {
        BooleanError::Escalated { diag } => assert_eq!(
            diag.predicate,
            Some("rim_circle_radius"),
            "the escalation must name the datum that landed in the band"
        ),
        other => panic!(
            "an in-band rim identification must escalate typed, not read as no rim: {other:?}"
        ),
    }
}

/// **The 34-row price, re-counted from the verdict log.** The PR
/// reports 34 metered rows for one rim on the G1 chain: 27
/// classification, 5 rim identification, 2 conformal screen. Counted
/// here on the KISSING fixture (a full-turn rim the reviewer can build
/// without the chain's helper geometry) and printed per predicate, so
/// the shape of the claim is checkable even where the absolute number
/// differs with the fixture. Note the log records DEFINITE outcomes
/// only.
#[test]
fn r2_recount_the_routing_price_from_the_verdict_log() {
    let a = full_torus(RING);
    let b = full_torus(RING + 2.0 * TUBE);
    let d = decls(&a, &b, ContactClass::Tangent);
    geom_core::k_stats::start_verdict_log();
    let err = topo::union_with(&a, &b, &d, Tol::witness()).expect_err("refuses");
    let log = geom_core::k_stats::take_verdict_log();
    let mut by: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for v in &log {
        *by.entry(v.predicate).or_default() += 1;
    }
    println!(
        "[r2] verdict log for the kissing rim ({err:?}): {} rows",
        log.len()
    );
    for (p, n) in &by {
        println!("[r2]   {p}: {n}");
    }
}

/// **The other half of the contrast, and the reason the row above is
/// worth having.** R2 wrote this pair of rows to show that a DEFINITE
/// mismatch and an INDETERMINATE one reached the identical refusal, so
/// an author could not tell "there is no rim" from "I could not decide
/// whether there is a rim".
///
/// This half needed no inversion: a definitely-different rim really is
/// "no rim", and the bare class refusal is the right answer for it. It
/// is the SIBLING that moved — the in-band pair now escalates typed —
/// and that is exactly what makes the two distinguishable. Kept, and
/// re-aimed at the distinction rather than at the collision, so the
/// pair still fails if the escalation is ever swallowed again.
#[test]
fn r2_a_definitely_absent_rim_keeps_the_bare_class_refusal() {
    let a = full_torus(RING);
    let far = full_torus(RING + 1.0);
    let d = decls(&a, &far, ContactClass::Tangent);
    let e = topo::union_with(&a, &far, &d, Tol::witness()).expect_err("refuses");
    println!("[r2] definitely-apart pair: {e:?}");
    assert!(
        matches!(
            e,
            BooleanError::UnsupportedDeclarationClass {
                class: ContactClass::Tangent
            }
        ),
        "a definitely-absent rim is not an escalation — that verdict belongs to the \
         in-band case alone: {e:?}"
    );
}
