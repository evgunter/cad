//! CERT-5 review lane R1 import-door probes (blinded adversarial
//! review of PR 1314, frozen head 3fc450d6).
//!
//! **Adopted into the unit by merge, authorship kept.** The dm1 row
//! was written to fail against the PR's claimed residual; its window
//! now carries the re-measured figure, and the note on that row says
//! what the discrepancy was.
//!
//! Two rows: (1) dm1 imported by the reviewer — the PR's residual
//! claim (a refusal with no floor under it) and the
//! wall-time claim, re-measured rather than believed; (2) the
//! reviewer's OWN rational wall (a 60-degree-arc loft at six
//! stations, off-grid interior v knots) — its enclosure through the
//! props door, and the same enclosure round-tripped through STEP —
//! what an import CONSUMER sees post-fix on a wall the unit's
//! fixtures never shaped.
//!
//! # One balloon, one certificate, every claim on it
//!
//! Row (2) is the single home for this reviewer's balloon. Its
//! native-props half — the off-grid-knot hypothesis, tiers 1/2, the
//! analytic-extrusion oracle and the pad ceiling — was a separate
//! `sweep` row building a character-identical body and running a
//! second rational quadrature over it. Under nextest's
//! process-per-test isolation nothing is shared between two rows, and
//! a rational patch-flux certificate is the expensive thing in this
//! class: at ε = 1e-12, where the schedule runs to the budget, it is
//! the whole cost of both rows. So the two are one row, and every
//! assertion below NAMES its property — `FIXTURE`, `TIER-1/2`,
//! `ORACLE`, `E2E ACCURACY`, `PAD CEILING`, `ROUND TRIP` — so the
//! failing property is unambiguous from the message alone. Keep that
//! discipline when adding assertions here.
//!
//! # Measured limits of the buildable family (found while probing)
//!
//! These bound what a probe of this class can reach at all, and they
//! are pre-existing kernel facts rather than anything a review PR
//! introduced:
//!
//! - a 3-sub-arc profile arc (> 180 degrees, the only native route to
//!   OFF-GRID interior *u* knots) never lofts: the build refuses at
//!   pcurve seam certification, "the seam carrier is not the chart's
//!   own boundary row";
//! - the same refusal fires for identical-section stacks at
//!   `(stations, degree, size, height)` = (7, 2, 1, 2), (8, 3, 1, 2),
//!   (9, 3, 1, 2), (6, 2, 0.6, 1.75) — and for the unit's own blade
//!   profile at height 1.75 instead of 2.0. **The buildable family is
//!   a narrow pocket around the shipped fixtures, and the quadrature
//!   is unreachable outside it**, which is why the balloon below is
//!   the shape it is rather than a bigger or a more twisted wall.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use std::path::PathBuf;
use step_import::{ImportOptions, StepImportError, import_step};

#[test]
fn dm1_residual_and_wall_time_remeasured() {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "wild",
        "stepcode",
        "dm1-id-214.stp",
    ]
    .iter()
    .collect();
    let text = std::fs::read_to_string(&path).unwrap();
    let t0 = std::time::Instant::now();
    let out = import_step(&text, &ImportOptions::default(), Tol::witness());
    let dt = t0.elapsed();
    match out {
        Err(StepImportError::TierInvalid { solid, errors }) => {
            eprintln!("CERT5-R1 dm1: TierInvalid solid {solid:?} in {dt:?}: {errors:?}");
            let text = format!("{errors:?}");
            // Band-honest on adoption: at a COARSE ambient band this
            // file's enclosure lands just under the loose target and
            // the convergence predicate escalates instead of refusing
            // on budget. Both are the same lane; only one of them is
            // reachable at a given ε.
            let coarse = Tol::witness().get().eps > 1e-9;
            assert!(
                text.contains("QuadratureBudget") || (coarse && text.contains("Escalated")),
                "dm1's refusal must still be the rational patch-flux lane: {text}"
            );
            if coarse {
                eprintln!("CERT5-R1 dm1: coarse band escalates rather than refusing on budget");
                return;
            }
            // Extract the width from the debug text.
            let w = text
                .split("width_len:")
                .nth(1)
                .and_then(|s| s.trim().split([',', ' ']).next())
                .and_then(|s| s.parse::<f64>().ok())
                .expect("the refusal must carry a width");
            eprintln!("CERT5-R1 dm1: width_len {w:e}");
            // **The re-measured residual.** The PR originally claimed
            // 1.5435e-6. That figure was taken mid-development, after
            // the hull blocks were knot-aligned but BEFORE the shared
            // area rule was — and the meter is
            // `flux.width() / (3·area_mid)`, so it moved when the
            // DENOMINATOR did. The flux enclosure is bit-identical
            // across that change (1.960408001025648e-9); what changed
            // is that the area stopped being inflated by the hull rule
            // its straddling cells used to take, and then tightened
            // again when the rule began intersecting both bounds.
            //
            // The window is wide because this row's claim is the
            // DISPOSITION — dm1 still refuses, and nowhere near the
            // 2.7e-4 floor it used to sit on — not the digit. The digit
            // lives in the PR description, where it can be argued.
            assert!(
                (1.5e-6..2.5e-6).contains(&w),
                "dm1 must still refuse, at a width that is the schedule running \
                 out rather than the retired 2.7e-4 floor: {w:e}"
            );
        }
        other => panic!("dm1 must still refuse at the at-rest gate, got {other:?}"),
    }
}

/// The reviewer's OWN body: a square whose `+x` side is replaced by a
/// SIXTY-degree bulge arc — one rational sub-arc, weight `cos 30 deg`,
/// which no shipped fixture uses — lofted at six stations on a
/// quadratic skin, so the section direction carries interior knots at
/// non-dyadic parameters.
///
/// The constant is `tan(60/4 deg)`: `bulge = tan(theta/4)`, so this is
/// the 60-degree arc the header names.
const BULGE: f64 = 0.267_949_192_431_122_7;
const HEIGHT: f64 = 2.0;
const STATIONS: usize = 6;
const V_DEGREE: usize = 2;

fn balloon_section() -> Vec<ProfileLoop<f64>> {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    vec![ProfileLoop::new(vec![
        v(-1.0, -1.0, 0.0),
        v(1.0, -1.0, BULGE),
        v(1.0, 1.0, 0.0),
        v(-1.0, 1.0, 0.0),
    ])]
}

/// **The reviewer's rational wall, end to end.**
///
/// One `mass_properties` call on the balloon carries every claim
/// below: what the props door says about it, and what an import
/// CONSUMER gets back after a STEP round trip.
///
/// **ε posture.** The schedule is fixed (D9) against a `1024·ε`
/// target, so this body certifies at the default ε and honestly
/// refuses on budget at a tighter one (measured: an achieved width of
/// ~3.7e-8 against a 1.024e-9 target at ε = 1e-12). Both are pinned.
/// The round trip compares two enclosures of one solid, which is only
/// a comparison when both exist, so on the refusing row it steps aside
/// LOUDLY rather than asserting the kernel converge — but the
/// fixture, the structural tiers and the typed shape of the refusal
/// are asserted at every ε, ahead of any quadrature.
#[test]
fn own_rational_wall_roundtrips_through_the_import_door() {
    let sections: Vec<_> = (0..STATIONS).map(|_| balloon_section()).collect();
    #[allow(clippy::cast_precision_loss)]
    let places: Vec<Affine3<f64>> = (0..STATIONS)
        .map(|k| {
            Affine3::translation(Vec3::new(
                0.0,
                0.0,
                HEIGHT * k as f64 / (STATIONS - 1) as f64,
            ))
        })
        .collect();
    let lofted =
        sweep::loft_body::<f64>(&sections, &places, V_DEGREE, Tol::witness()).expect("lofts");

    // FIXTURE: the V-direction hypothesis, asserted on the fixture
    // itself and at every ε — interior knots are running means of
    // `V_DEGREE` consecutive section parameters, and at six even
    // stations two of them are off every dyadic grid the composite
    // cuts (5/12, 7/12). A fixture that stopped producing them would
    // pass by testing nothing.
    let params = &lofted.section_params;
    #[allow(clippy::cast_precision_loss)]
    let interior: Vec<f64> = (1..params.len() - V_DEGREE)
        .map(|j| params[j..j + V_DEGREE].iter().sum::<f64>() / V_DEGREE as f64)
        .collect();
    let off = interior
        .iter()
        .filter(|k| {
            let mut pieces = 8u32;
            let mut on = false;
            while pieces <= 1024 {
                let s = *k * f64::from(pieces);
                if (s - s.round()).abs() < 1e-12 {
                    on = true;
                }
                pieces *= 2;
            }
            !on
        })
        .count();
    eprintln!("CERT5-R1 balloon: interior v knots {interior:?}, off-grid {off}");
    assert!(
        off >= 2,
        "FIXTURE: the balloon must carry off-grid interior v knots (got {off})"
    );

    // TIER-1/2: structural tiers never touch quadrature, so this is
    // pinned at every ε whatever the posture below turns out to be.
    topo::validate_closed(&lofted.body).expect("TIER-1/2: tiers 1/2 admit the balloon");

    // ORACLE: the same profile EXTRUDED — the arc wall is an analytic
    // cylinder, closed form, pad exactly 0. A different surface
    // representation, a different props lane, no shared arithmetic with
    // the quadrature under test. It costs ~0.03 s and it is asserted
    // AHEAD of the certificate, unconditionally: an oracle that stopped
    // being a closed form would otherwise go unchecked on exactly the ε
    // rows where the balloon refuses and this row steps aside.
    let prof = Profile::new(SketchPlane::xy(), balloon_section())
        .validate(Tol::witness())
        .expect("the balloon profile validates");
    let oracle = sweep::extrude::<f64>(&prof, sweep::Extrusion::Distance(HEIGHT), Tol::witness())
        .expect("extrude");
    let want = topo::mass_properties(&oracle.body, Tol::witness()).expect("analytic oracle");
    assert_eq!(
        want.volume_pad, 0.0,
        "ORACLE: the extrude oracle must be a closed form"
    );

    // THE ONE CERTIFICATE. Every claim below reads it; nothing
    // recomputes it.
    let native = match topo::mass_properties(&lofted.body, Tol::witness()) {
        Ok(native) => native,
        Err(e) => {
            let refused = format!("{e}");
            assert!(
                Tol::witness().get().eps < 1e-9,
                "E2E POSTURE: the balloon must certify at the default eps through the \
                 public door (off-grid knots in both directions are exactly the retired \
                 defect): {e}"
            );
            assert!(
                refused.contains("quadrature enclosure stalled"),
                "E2E POSTURE: at a tighter eps the only honest refusal here is the \
                 budget: {e}"
            );
            // The stand-down goes through the tree's ONE in-row door
            // (`test_utils::vacuity`'s module docs: every in-row
            // stand-down in `crates/` uses it, and the whole-binary
            // `#[cfg]`-gated `interval_lane_skipped_…` rows are a
            // different idiom). It has to be the in-row one here: the
            // condition is the RUN's ε, read at run time, so no
            // `#[cfg]` and therefore no test NAME can carry it.
            test_utils::vacuity::stood_down(
                "cert5-r1 balloon round trip",
                &format!(
                    "the native balloon refuses on budget at eps={:e}, so there is no \
                     enclosure to round-trip and the ROUND TRIP assertions below are \
                     NOT made on this row; the fixture, tiers 1/2, the oracle and the \
                     typed shape of the refusal are asserted above and hold here",
                    Tol::witness().get().eps
                ),
            );
            return;
        }
    };

    eprintln!(
        "CERT5-R1 balloon: certified volume {} +- {}; oracle {}",
        native.volume, native.volume_pad, want.volume
    );
    assert!(
        (native.volume - want.volume).abs() <= native.volume_pad,
        "E2E ACCURACY: the enclosure must CONTAIN the analytic volume: \
         got {} +- {}, oracle {}",
        native.volume,
        native.volume_pad,
        want.volume
    );
    // PAD CEILING, pinned separately so a loosening enclosure cannot
    // absorb the accuracy assertion above. Keyed to ε, because the
    // schedule is fixed and what the schedule is asked for scales with
    // the run's tolerance — an absolute ceiling here would really be a
    // claim about ε = 1e-9, and would red honestly-proportionate pads
    // at a coarser band (measured: 1.27e-3 at ε = 1e-6, against a
    // 1.024e-3 target).
    let ceiling = 2.0 * 1024.0 * Tol::witness().get().eps;
    assert!(
        native.volume_pad < ceiling,
        "PAD CEILING: the pad must sit under the retired-floor ceiling: {} vs {ceiling}",
        native.volume_pad
    );

    // ROUND TRIP: what an import consumer gets back.
    let text = step_export::step_string(
        &lofted.body,
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the arc-bearing loft exports");
    let t0 = std::time::Instant::now();
    let imported = import_step(&text, &ImportOptions::default(), Tol::witness());
    let dt = t0.elapsed();
    match imported {
        Ok(step_import::StepImport::Solid { body, .. }) => {
            let m = topo::mass_properties(&body, Tol::witness())
                .expect("the imported twin certifies too");
            eprintln!(
                "CERT5-R1 roundtrip: import+gate in {dt:?}; native {} +- {}, imported {} +- {}",
                native.volume, native.volume_pad, m.volume, m.volume_pad
            );
            // The two enclosures must overlap: same solid.
            assert!(
                (m.volume - native.volume).abs() <= m.volume_pad + native.volume_pad,
                "ROUND TRIP: volume disagrees: native {} +- {}, imported {} +- {}",
                native.volume,
                native.volume_pad,
                m.volume,
                m.volume_pad
            );
        }
        other => panic!(
            "ROUND TRIP: the reviewer's rational wall must import first-class post-fix \
             (off-grid knots in both directions through the import door): {other:?}"
        ),
    }
}
