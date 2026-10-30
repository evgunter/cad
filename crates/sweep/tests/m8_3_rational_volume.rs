//! **M8-3 — rational walls become VOLUME-COMPUTABLE.**
//!
//! The soundness pins for the rational patch-flux enclosure reached
//! from a BODY (the arc-rim chart map is what makes the body
//! assemble), in the SKINFIT two-assertion shape
//! (`m7_skin_integral.rs`): pin the accuracy against an INDEPENDENT
//! oracle **and separately** pin the pad ceiling, so a loosening
//! enclosure can never quietly absorb the tolerance.
//!
//! The oracle is the strongest available: the *same solid* built by
//! `extrude`, whose bulged wall is an analytic `Surface::Cylinder`
//! closed form with pad exactly 0 — a different surface
//! representation, a different props lane, and no shared arithmetic
//! with the quadrature under test.
//!
//! # The arc LOFT's disposition lives in `step-import`
//!
//! The three-station loft at scales `1.0, 1.25, 1.0` — sections of
//! DIFFERENT scale, so the rational wall genuinely varies in `v` — is
//! pinned by `step-import::nurbs_import::arc_loft_natively_computes_
//! its_rational_volume`, which builds a character-identical body and
//! already pays a native rational quadrature on it for its round-trip
//! comparison. That row asserts the same posture classification, the
//! same `12.0 < volume < 13.0` band and the same `2·1024·ε` pad
//! ceiling, and adds tiers 1 and 2 on top. A second certificate here
//! would buy the same quadrature at the same ε.
//!
//! # ε posture (the PR-1 discipline, applied at the BODY level)
//!
//! The row reads at a resolution of `1024·ε`, so a body that meets it at
//! one ε honestly may not at a tighter one. The row below therefore pins
//! both honest outcomes and never widens a target: `Certified` (the
//! enclosure must CONTAIN the oracle and respect its pad ceiling) and
//! `Budget` (a bracket short of the resolution, at the floor or the work
//! wall). The resolution is compared, not decided, so no predicate can
//! escalate. Anything else panics.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{arc_section, stacked};
use geom_core::Tol;
use profile::{Profile, SketchPlane};
use sweep::ExtrudeSide;
use sweep::loft_body;
use topo::{MassMeasurement, Stop};

/// The resolution this row reads its numbers at, as a multiple of ε:
/// the pinned resolution's (`topo::test_support::pinned_resolution`).
const QUAD_TARGET_LEN_FACTOR: f64 = 1024.0;

/// The two honest outcomes of a certified quadrature at a given ε.
#[derive(Debug, PartialEq, Eq)]
enum EpsPosture {
    /// The resolution met; the caller pins accuracy and the pad ceiling.
    Certified,
    /// The faces stopped short of `1024·ε`, at the floor or the work
    /// wall, with the bracket they reached.
    Budget,
}

/// Classify a body-level measurement, asserting each posture's own
/// invariants. Anything outside the two is a real failure and panics.
fn body_posture(row: &str, out: &MassMeasurement<f64>) -> EpsPosture {
    match out.stop {
        Stop::Met => EpsPosture::Certified,
        Stop::Floor | Stop::Wall => {
            assert!(
                out.enclosure.volume_pad > 0.0,
                "{row}: a bracket short of the resolution has a width: {out:?}"
            );
            EpsPosture::Budget
        }
        stop => panic!("{row}: not an honest quadrature posture: {stop:?}"),
    }
}

/// **The arc PRISM** (the `#288` waypoint's body): three identical
/// arc sections stacked, so the loft reproduces an extrusion exactly.
///
/// The `#288` claim this row settles is the CONSTRUCTION one — the
/// body assembles, its rational wall's arc rims mint and certify, and
/// tier 3 admits it. Whether the volume itself certifies is the ε
/// question above, pinned per posture.
///
/// # One body, one build, every property on it
///
/// The tier-3 verdict and the volume bracket were two separate tests
/// until the test-cost audit. Both built THIS prism — the same three
/// `arc_section(1.0)` sections on the same `stacked(&[0.0, 1.0, 2.0], 1.0)`,
/// the same 2 samples — and both then ran the same rational
/// quadrature over it. Under nextest's process-per-test isolation
/// there is no cache between them, so every ε row paid that
/// build-plus-quadrature twice. The oracle side — one `extrude` and
/// its closed-form props — costs ~0.03 s, so folding the bracket in
/// here is free and the duplicate build is gone.
///
/// What the split bought and a merged row cannot is failure
/// ISOLATION: a tier-3 break and an accuracy break now surface under
/// one test id. So every assertion below NAMES its property —
/// `TIER-1/2`, `TIER-3`, `ORACLE`, `ACCURACY`, `PAD CEILING` — and
/// the message alone says which one broke. Keep that discipline when
/// adding assertions here.
///
/// # The order below is load-bearing
///
/// `validate_closed` is asserted UNCONDITIONALLY, ahead of any
/// quadrature. Structural tiers never touch quadrature, so the CHART
/// map (rims mint and certify) is pinned at EVERY ε — including the ε
/// rows where the volume honestly refuses on budget and this test
/// returns early. That unconditional pin is the whole reason the
/// tier-3 row was split out originally; it must never migrate under
/// the `Certified` branch.
///
/// Tier 3 itself runs the +V invariant, which CONSUMES the quadrature,
/// and it is the SAME quadrature this row measures: the row takes
/// `validate_geometric_certificate`, which returns the certificate
/// check 7 decided on, and continues it to the reporting target — so
/// the tier-3 verdict and the volume bracket cost one certificate
/// between them instead of two.
///
/// **The gate and the posture are now two different questions**, and
/// the order below says so. Check 7 certifies this body's SIGN, which
/// is definite at every ε here, so tier 3 admits the rational wall
/// UNCONDITIONALLY — a tier-3 refusal is a break, at every ε row, and
/// no longer something a tight ε can honestly produce. The ε posture
/// is then read off the CONTINUATION, which is the caller that wants
/// the number and the only one the fixed schedule can honestly refuse.
#[test]
fn tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion() {
    let loft = loft_body::<f64>(
        &[arc_section(1.0), arc_section(1.0), arc_section(1.0)],
        &stacked(&[0.0, 1.0, 2.0], 1.0),
        2,
        Tol::witness(),
    )
    .expect("the arc prism lofts (the pcurve mint is inside the build)")
    .body;
    // INVARIANT: tiers 1 and 2 admit the rational-wall body at EVERY
    // ε. Structural tiers never touch quadrature: the body IS closed
    // and its rational-wall pcurves are minted and certified, whatever
    // the volume posture below turns out to be. Nothing may gate this
    // line on that posture.
    topo::validate_closed(&loft).expect("TIER-1/2: tiers 1/2 admit the rational-wall body");
    // ONE certificate for this row. The tier-3 gate computes a full
    // enclosure to decide its +V invariant; `validate_geometric_
    // certificate` hands that object back, so the verdict below and
    // the number this row measures are the same computation rather
    // than two runs of it.
    let gated = topo::validate_geometric_certificate(&loft, Tol::witness());
    // TIER 3: tier 3 certifies a rational-wall body (M8-3 flip of
    // #288/#276) at EVERY ε — its check 7 reads a sign, and this
    // body's sign is definite. Any verdict in the vector is a break.
    let gated = gated.unwrap_or_else(|errors| {
        panic!("TIER-3: tier 3 certifies a rational-wall body (M8-3 flip of #288/#276): {errors:?}")
    });
    // The ε posture is the CONTINUATION's: the caller that wants the
    // number is the one the fixed schedule can honestly refuse.
    let got = gated
        .measure(topo::test_support::pinned_resolution(Tol::witness()))
        .unwrap_or_else(|e| panic!("the arc prism measures: {e}"));
    let posture = body_posture("arc prism", &got);
    eprintln!(
        "EPS-ROW arc prism @ eps={:e}: {posture:?} volume {} ± {}",
        Tol::witness().get().eps,
        got.enclosure.volume,
        got.enclosure.volume_pad
    );
    if posture != EpsPosture::Certified {
        // The honest non-certifying postures still say the CHART map
        // landed: a body whose arc rims did not mint would have failed
        // to build, long before any quadrature ran — and the tier-1/2
        // pin above has already fired on this row regardless.
        return;
    }
    let got = got.enclosure;

    // The oracle: the same solid through `extrude`, whose bulged wall
    // is an analytic cylinder (closed form, pad 0).
    let prof = Profile::new(SketchPlane::xy(), arc_section(1.0))
        .validate(Tol::witness())
        .expect("the profile validates");
    let oracle = sweep::extrude::<f64>(
        &prof,
        sweep::Extrusion::Distance {
            depth: 2.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .expect("extrude");
    let want = topo::mass_properties(
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&oracle.body),
            Tol::witness(),
        ),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("analytic mass properties");
    assert_eq!(
        want.enclosure.volume_pad, 0.0,
        "ORACLE: the extrude oracle must be a closed form"
    );

    // 1. ACCURACY: the certified enclosure contains the oracle.
    assert!(
        (got.volume - want.enclosure.volume).abs() <= got.volume_pad,
        "ACCURACY: the rational enclosure must CONTAIN the analytic volume: \
         got {} ± {}, oracle {}",
        got.volume,
        got.volume_pad,
        want.enclosure.volume,
    );
    // 2. PAD CEILING, pinned separately: a loosening enclosure cannot
    // absorb assertion 1 without tripping this. The ceiling is keyed
    // to ε because the schedule is fixed and the pad is what the
    // schedule achieves. Each face is read about a vertex of the
    // shell's first quadrature face (`topo::props`' `shell_reference`),
    // so a quadrature face's lever is the extent of the shell's
    // quadrature faces: on this prism, centred on the world origin, its
    // diameter where the origin's was its radius. Measured 1.11e-6 at
    // 1e-9, 1.08 times the target: the ceiling is that measure with
    // headroom, not a bound the rule guarantees.
    let ceiling = 4.0 * QUAD_TARGET_LEN_FACTOR * Tol::witness().get().eps;
    assert!(
        got.volume_pad < ceiling,
        "PAD CEILING: volume pad {} vs {ceiling} (M8-3 measured {} at ε=1e-9)",
        got.volume_pad,
        ARC_PRISM_PAD_AT_DEFAULT_EPS,
    );
    // The measure the ceiling's comment cites, held where it was taken.
    if Tol::witness().get().eps == 1e-9 {
        assert!(
            (got.volume_pad / ARC_PRISM_PAD_AT_DEFAULT_EPS - 1.0).abs() < 0.01,
            "MEASURED: volume pad {} moved from the {} the ceiling's comment cites",
            got.volume_pad,
            ARC_PRISM_PAD_AT_DEFAULT_EPS,
        );
    }
}

/// The pad measured on the arc prism at the default ε (1e-9), each face
/// read about a vertex of the shell's first quadrature face. Kept as a
/// named number so the ceiling assertion above can say what the lane
/// actually achieves, not only what it must beat.
const ARC_PRISM_PAD_AT_DEFAULT_EPS: f64 = 1.106_6e-6;
