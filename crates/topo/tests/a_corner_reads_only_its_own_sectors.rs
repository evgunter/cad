//! **A corner reads only a glued pair as one surface.** Two plane faces
//! meeting at a shared vertex, which the glue door did not glue, read
//! parallel at the corner's arm (`bool_faces_parallel`, the normals'
//! cross levered at the shorter of the two corners' arms) refuse there:
//! `Coincidence(CornerCrossing)`, the Zero rejected by the question that
//! only a definite tilt passes, its margin `θ·arm` and its offer the
//! tolerance that decides that tilt. An unglued Zero acts over no extent,
//! and a coplanar lump would speak for both faces' whole spans: here a
//! far bound stands hundreds of bands off the other plane.
//!
//! The poses: the designers' tool on a block's corner
//! (`test_support::tool_on_a_block_corner`, its corner's arm 1 mm),
//! generic and hinged along `b`, and a parallelepiped with a 1 mm and a
//! 10 m edge on its bottom cornered on a unit one below it.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Band, ErrorTextReading, Point3, Sign, SizedPass, Tol, Vec3};
use test_utils::offer::{Outcome, report, run};
use topo::test_support::{HINGED_ALONG_B, finished, mapped_cube, tool_on_a_block_corner};
use topo::{AtRestBody, BooleanDecision, BooleanError, Coincide, DeclarationRead};

/// The corner's arm in every pose here: 1 mm.
const ARM: f64 = 1e-3;

/// The tool on the block's corner at tilt `theta`, finished.
fn corner_pose(theta: f64, psi: f64, tol: Tol) -> [AtRestBody<f64>; 2] {
    let (block, tool) = tool_on_a_block_corner(theta, psi, tol);
    [finished("block", block, tol), finished("tool", tool, tol)]
}

/// `A`, a parallelepiped with a 10 m and a 1 mm edge on its bottom at the
/// origin, and `B`, a unit one below it cornered there, its top dipping
/// `θ·(ta·x + tb·y)` and spanning the directions `b0`° to `b1`°.
fn needle_on_a_corner(
    theta: f64,
    (ta, tb): (f64, f64),
    (b0, b1): (f64, f64),
    tol: Tol,
) -> [AtRestBody<f64>; 2] {
    let o = Point3::new(0.0, 0.0, 0.0);
    let a = mapped_cube(
        |u, v, w| {
            o + Vec3::new(10.0, 0.0, 0.0) * u
                + Vec3::new(0.0, 1e-3, 0.0) * v
                + Vec3::new(0.0, 0.0, 1.0) * w
        },
        tol,
    );
    let dip = |deg: f64| {
        let (x, y) = (deg.to_radians().cos(), deg.to_radians().sin());
        Vec3::new(x, y, -theta * (ta * x + tb * y))
    };
    let (u, v) = (dip(b0), dip(b1));
    let b = mapped_cube(
        move |s, t, w| o + v * s + u * t + Vec3::new(0.0, 0.0, -1.0) * w,
        tol,
    );
    [finished("A", a, tol), finished("B", b, tol)]
}

/// One run of an op on the pair.
type Run = (&'static str, Result<topo::BooleanResult<f64>, BooleanError>);

/// The six runs: each op in both operand orders.
fn six([a, b]: &[AtRestBody<f64>; 2], tol: Tol) -> [Run; 6] {
    [
        ("A ∩ B", topo::intersect(a, b, tol)),
        ("B ∩ A", topo::intersect(b, a, tol)),
        ("A ∖ B", topo::subtract(a, b, tol)),
        ("B ∖ A", topo::subtract(b, a, tol)),
        ("A ∪ B", topo::union(a, b, tol)),
        ("B ∪ A", topo::union(b, a, tol)),
    ]
}

/// Asserts `out` is the corner's crossing refusal at `margin`, offering
/// `margin/K`.
fn assert_corner_crossing(
    label: &str,
    out: &Result<topo::BooleanResult<f64>, BooleanError>,
    margin: f64,
    band: Band,
) {
    let Err(err @ BooleanError::Escalated { decision, diag }) = out else {
        panic!("{label}: refuses on an escalation: {out:?}");
    };
    assert_eq!(
        *decision,
        BooleanDecision::Coincidence(Coincide::CornerCrossing, DeclarationRead::Moot),
        "{label}: the corner's crossing question: {diag:?}"
    );
    assert_eq!(
        diag.predicate,
        Some("bool_faces_parallel"),
        "{label}: the tilt at the arm"
    );
    assert_eq!(
        diag.margin.rejected_sign(),
        Some(Sign::Zero),
        "{label}: a Zero, rejected"
    );
    assert_eq!(
        diag.margin.rejected_passing(),
        Some(SizedPass::Positive),
        "{label}: rejected by the question only a definite tilt passes: {:?}",
        diag.margin
    );
    let ErrorTextReading::Value(m) = diag.margin.diagnostic_f64_for_error_text() else {
        panic!("{label}: a value margin: {diag:?}");
    };
    assert!(
        (m - margin).abs() <= 1e-6 * margin,
        "{label}: the tilt levered at the arm, {margin:e}, got {m:e}"
    );
    let text = err.render(geom_brep::recourse::Reading::Build);
    // The offer is the arm's tilt over K, where a run admits it.
    match (
        band.offer_deciding(margin),
        test_utils::offer::offered_below(&text),
    ) {
        (Some(want), Some(offer)) => assert!(
            (offer - want).abs() <= 1e-6 * want,
            "{label}: offers the arm's tilt over K, {want:e}: {text}"
        ),
        (None, None) => assert!(
            text.contains("no tolerance a run admits decides this rise"),
            "{label}: below the floor, says so: {text}"
        ),
        (want, got) => panic!("{label}: offers {got:?} where a run admits {want:?}: {text}"),
    }
    assert!(
        text.contains("move the parts so the two faces clearly cross at that corner"),
        "{label}: the corner's lever: {text}"
    );
}

/// The generic and hinged corner poses and the needle pose, at two tilts
/// scaled to the run's band, through every op in both orders: each the
/// corner's crossing refusal on the arm's tilt.
///
/// Above ε 1e-7 the two tool poses meet the tool's own fixture first:
/// its short edge `c` leans `1e-4` m off the block's corner, and its long
/// edge `a` turns `0.0995` rad off the block's `x` edge, so the edge
/// contact reads `≈ 9.95e-6` whatever the tilt (measured at both tilts
/// and both poses), in band at ε 1e-6. There they refuse that
/// containment (`bool_contact_edge`), pinned. The needle poses have no
/// such edge and refuse on the corner at every ε.
#[test]
fn an_unglued_pair_parallel_at_the_arm_refuses_on_the_arms_tilt() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let tool_meets_its_edge_first = tol.eps() > 1e-7;
    for k in [100.0, 300.0] {
        let theta = k * band.zero();
        for (pose, psi) in [("generic", 0.0), ("hinged", HINGED_ALONG_B)] {
            let bodies = corner_pose(theta, psi, tol);
            let margin = theta / (1.0 + theta * theta).sqrt() * ARM;
            for (op, out) in six(&bodies, tol) {
                let label = format!("{pose}, θ = {k}ε, {op}");
                if tool_meets_its_edge_first {
                    assert_edge_contact_first(&label, &out);
                } else {
                    assert_corner_crossing(&label, &out, margin, band);
                }
            }
        }
        for (tilt, span) in [((1.0, 0.0), (-45.0, 45.0)), ((0.6, -0.8), (30.0, 60.0))] {
            let bodies = needle_on_a_corner(theta, tilt, span, tol);
            let margin = theta / (1.0 + theta * theta).sqrt() * ARM;
            for (op, out) in six(&bodies, tol) {
                let label = format!("needle, tilt {tilt:?}, span {span:?}, θ = {k}ε, {op}");
                assert_corner_crossing(&label, &out, margin, band);
            }
        }
    }
}

/// Asserts `out` is the tool pose's edge contact read in band
/// (`bool_contact_edge`, margin `≈ 9.95e-6`), which the tool's fixture
/// meets ahead of the corner above ε 1e-7.
fn assert_edge_contact_first(label: &str, out: &Result<topo::BooleanResult<f64>, BooleanError>) {
    let Err(BooleanError::Escalated {
        decision: BooleanDecision::Containment { .. },
        diag,
    }) = out
    else {
        panic!("{label}: the edge contact's containment: {out:?}");
    };
    assert_eq!(diag.predicate, Some("bool_contact_edge"), "{label}");
    let ErrorTextReading::Value(m) = diag.margin.diagnostic_f64_for_error_text() else {
        panic!("{label}: a value margin: {diag:?}");
    };
    assert!(
        (m - 9.95e-6).abs() < 1e-2 * 9.95e-6,
        "{label}: the fixture's edge contact, whatever the tilt: {m:e}"
    );
}

/// What the six runs of a corner pose at the design tilt `1e-7` did, at
/// the tolerance this process runs at: the first refusal, keyed by its
/// decision and predicate, or a pass.
fn six_at_the_design_tilt(psi: f64) -> Outcome {
    let tol = Tol::witness();
    let bodies = corner_pose(1e-7, psi, tol);
    six(&bodies, tol)
        .into_iter()
        .find_map(|(_, out)| out.err())
        .map_or(Outcome::Pass, |err| Outcome::Refused {
            key: match &err {
                BooleanError::Escalated { decision, diag } => {
                    format!("{decision:?} {:?}", diag.predicate)
                }
                other => format!("{other:?}"),
            },
            defect: false,
            text: err.render(geom_brep::recourse::Reading::Build),
        })
}

#[test]
#[ignore = "a child row: the_corners_offer_executes runs it"]
fn generic_corner_six_runs() {
    report("generic_corner_six_runs", &six_at_the_design_tilt(0.0));
}

#[test]
#[ignore = "a child row: the_corners_offer_executes runs it"]
fn hinged_corner_six_runs() {
    report(
        "hinged_corner_six_runs",
        &six_at_the_design_tilt(HINGED_ALONG_B),
    );
}

/// **The corner's offer executes.** At the design tolerance (1e-9) the
/// tool at tilt `1e-7` refuses on the arm's tilt, `1e-7·1 mm`, offering
/// that over K. Just under the offer the generic pose builds in all six
/// runs; the hinged pose, whose faces cross along the short edge, meets
/// the contact edge's placement in band there, a decision of its own.
#[test]
fn the_corners_offer_executes() {
    let module = module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, m)| m);
    let design = 1e-9;
    let k = geom_core::tolerance::DEFAULT_K;
    let offer = 1e-7 / (1.0 + 1e-14_f64).sqrt() * ARM / k;
    let below = test_utils::offer::BELOW * offer;
    for (row, want) in [
        ("generic_corner_six_runs", None),
        ("hinged_corner_six_runs", Some("bool_contact_edge")),
    ] {
        let first = run(&format!("{module}::{row}"), design);
        let Outcome::Refused { key, text, .. } = &first else {
            panic!("{row} at {design:e}: refuses: {first:?}");
        };
        assert!(
            key.starts_with("Coincidence(CornerCrossing, Moot)"),
            "{row} at {design:e}: the corner's crossing: {first:?}"
        );
        let quoted = test_utils::offer::offered_below(text).expect("an offer");
        assert!(
            (quoted - offer).abs() <= 1e-6 * offer,
            "{row}: offers {offer:e}, quoted {quoted:e}"
        );
        let got = run(&format!("{module}::{row}"), below);
        match (want, &got) {
            (None, Outcome::Pass) => {}
            (Some(predicate), Outcome::Refused { key, .. })
                if key.contains(predicate) && !key.contains("CornerCrossing") => {}
            _ => panic!("{row} at {below:e}, just under the offer: {got:?}"),
        }
    }
}
