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
//! The convergence target is `1024·ε` against a **fixed** schedule
//! (D9), so a body that certifies at one ε honestly may not at a
//! tighter one. The row below therefore pins all three honest
//! outcomes and never widens a target: `Certified` (the enclosure must CONTAIN
//! the oracle and respect its pad ceiling), `Budget` (a typed
//! `QuadratureBudget` whose width really missed a target that really
//! is `1024·ε`), `Escalated` (only `props_quad_converged` may
//! escalate). Anything else panics.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::PropsError;
use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Section, loft_body};
use topo::{MassProperties, MassPropsError};

/// The convergence target's tolerance factor, mirrored from
/// `geom_brep::props::quad` (private there; the pin is that the
/// refusal's own `target_len` equals this times the run's ε).
const QUAD_TARGET_LEN_FACTOR: f64 = 1024.0;

/// The three honest outcomes of a certified quadrature at a given ε.
#[derive(Debug, PartialEq, Eq)]
enum EpsPosture {
    /// Bounds returned; the caller pins accuracy and the pad ceiling.
    Certified,
    /// The fixed schedule ran out before `1024·ε` — typed, with the
    /// measured width.
    Budget,
    /// The convergence predicate could not be decided in-band.
    Escalated,
}

/// Classify a body-level mass-properties outcome, asserting each
/// posture's own invariants. Anything outside the three is a real
/// failure and panics.
fn body_posture(row: &str, out: &Result<MassProperties<f64>, MassPropsError>) -> EpsPosture {
    let target = QUAD_TARGET_LEN_FACTOR * Tol::witness().get().eps;
    match out {
        Ok(_) => EpsPosture::Certified,
        Err(MassPropsError::Face {
            source:
                PropsError::QuadratureBudget {
                    width_len,
                    target_len,
                },
            ..
        }) => {
            assert!(
                width_len.is_finite() && width_len > target_len,
                "{row}: a budget refusal must carry a width that really missed: \
                 {width_len:e} vs {target_len:e}"
            );
            assert!(
                (target_len - target).abs() <= target * 1e-12,
                "{row}: the refused target must BE 1024·ε for this run: \
                 {target_len:e} vs {target:e}"
            );
            EpsPosture::Budget
        }
        Err(MassPropsError::Face {
            source: PropsError::Escalated { cause },
            ..
        }) => {
            assert_eq!(
                cause.predicate,
                Some("props_quad_converged"),
                "{row}: only the convergence predicate may escalate here: {cause:?}"
            );
            EpsPosture::Escalated
        }
        Err(other) => panic!("{row}: not an honest quadrature posture: {other}"),
    }
}

/// A unit square with a quarter-circle bulge on the `+x` side — the
/// arc-bearing profile whose lofted wall is RATIONAL (weights
/// `1, cos 22.5°, 1` over two 45° sub-arcs).
fn arc_section(s: f64) -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    vec![ProfileLoop::new(vec![
        v(-s, -s, 0.0),
        // tan(π/8): a quarter-circle bulge-out.
        v(s, -s, 0.4142135623730951),
        v(s, s, 0.0),
        v(-s, s, 0.0),
    ])]
}

fn stack(z: [f64; 3]) -> Vec<Affine3<f64>> {
    z.map(|h| Affine3::translation(Vec3::new(0.0, 0.0, h)))
        .into()
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
/// `arc_section(1.0)` sections on the same `stack([0.0, 1.0, 2.0])`,
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
/// the `Certified` branch. Tier 3 itself runs the +V invariant, which
/// CONSUMES the quadrature (a budget refusal there is an honest
/// tier-3 refusal), so `validate_geometric` stays inside the
/// `Certified` branch, exactly where it always was.
#[test]
fn tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion() {
    let loft = loft_body::<f64>(
        &[arc_section(1.0), arc_section(1.0), arc_section(1.0)],
        &stack([0.0, 1.0, 2.0]),
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
    let got = topo::mass_properties(&loft, Tol::witness());
    let posture = body_posture("arc prism", &got);
    eprintln!(
        "EPS-ROW arc prism @ eps={:e}: {posture:?}{}",
        Tol::witness().get().eps,
        match &got {
            Ok(m) => format!(" volume {} ± {}", m.volume, m.volume_pad),
            Err(e) => format!(" ({e})"),
        }
    );
    if posture != EpsPosture::Certified {
        // The honest non-certifying postures still say the CHART map
        // landed: a body whose arc rims did not mint would have failed
        // to build, long before any quadrature ran — and the tier-1/2
        // pin above has already fired on this row regardless.
        return;
    }
    let got = got.expect("certified");

    // TIER 3: the +V invariant consumes the quadrature, so the verdict
    // is pinned exactly where the quadrature certifies (a budget
    // refusal here would be an honest tier-3 refusal, not a break).
    topo::validate_geometric(&loft, Tol::witness())
        .expect("TIER-3: tier 3 certifies a rational-wall body (M8-3 flip of #288/#276)");

    // The oracle: the same solid through `extrude`, whose bulged wall
    // is an analytic cylinder (closed form, pad 0).
    let prof = Profile::new(SketchPlane::xy(), arc_section(1.0))
        .validate(Tol::witness())
        .expect("the profile validates");
    let oracle = sweep::extrude::<f64>(&prof, sweep::Extrusion::Distance(2.0), Tol::witness())
        .expect("extrude");
    let want =
        topo::mass_properties(&oracle.body, Tol::witness()).expect("analytic mass properties");
    assert_eq!(
        want.volume_pad, 0.0,
        "ORACLE: the extrude oracle must be a closed form"
    );

    // 1. ACCURACY: the certified enclosure contains the oracle.
    assert!(
        (got.volume - want.volume).abs() <= got.volume_pad,
        "ACCURACY: the rational enclosure must CONTAIN the analytic volume: \
         got {} ± {}, oracle {}",
        got.volume,
        got.volume_pad,
        want.volume,
    );
    // 2. PAD CEILING, pinned separately: a loosening enclosure cannot
    // absorb assertion 1 without tripping this. The ceiling is keyed
    // to ε because the schedule is fixed and the pad is what the
    // schedule achieves — never widened past the run's own target.
    let ceiling = 2.0 * QUAD_TARGET_LEN_FACTOR * Tol::witness().get().eps;
    assert!(
        got.volume_pad < ceiling,
        "PAD CEILING: volume pad {} vs {ceiling} (M8-3 measured {} at ε=1e-9)",
        got.volume_pad,
        ARC_PRISM_PAD_AT_DEFAULT_EPS,
    );
}

/// The pad measured on the arc prism at the default ε (1e-9), on the
/// POST-#313 hulls (the shared midpoint-plus-Taylor area rule). Kept
/// as a named number so the ceiling assertion above can say what the
/// lane actually achieves, not only what it must beat.
///
/// It is the SAME number the pre-#313 hulls gave, and that is not a
/// coincidence: `volume_pad` is flux-side, and #313 healed the AREA
/// rule. The area pad moved; the volume pad could not.
const ARC_PRISM_PAD_AT_DEFAULT_EPS: f64 = 9.816_714_3e-7;
