//! **The tier-3 doors' returned certificate** — one certified
//! quadrature per body per gate.
//!
//! A body that is gated at rest and then measured used to pay two
//! certified quadratures for one number: check 7 computes a full
//! `MassProperties` to decide the +V invariant and drops it, so a
//! caller that also wants the enclosure runs the identical computation
//! again. `topo::validate_geometric_certificate` and
//! `topo::validate_pseudomanifold_certificate` return what the gate
//! computed.
//!
//! This suite lives in `sweep` because its subject is the QUADRATURE
//! lane: the identity claim is only interesting on a body whose faces
//! carry certified enclosures rather than closed forms, and the arc
//! prism's rational wall is the cheapest such body the public API
//! builds. `topo` cannot build one — `sweep` is above it. The
//! import-path half of the same claim needs a reader, so it is
//! `step-import`'s `tcost_k3_import_certificate` and is gated to the
//! same set.
//!
//! Every row NAMES its property in the assertion message, because the
//! rows share their expensive fixture (the aggregation rule in
//! `memories/test-suite-cost.md`): `IDENTITY`, `ONE CERTIFICATE`,
//! `PLANTED`.
//!
//! # ε, and why the fixture is ε-SCALED
//!
//! ε is the run's, not the row's — `Tol` is a witness, and the doors
//! under test take one. A fixture of FIXED size therefore certifies at
//! some ε rows and refuses at others, and a row that tolerates both
//! postures compares nothing on the refusing ones: the identity
//! assertions sit under an `Ok` arm that is never taken, and the row
//! degenerates into a comparison of refusal counts. That is what a
//! fixed 5 cm prism did here at ε = 1e-12, where the last-round bound
//! refuses after round 0 — and three mutants that red at 1e-9 lived
//! through it.
//!
//! So SIZE is ε's partner, in both directions, and both fixtures are
//! scaled by it:
//!
//! * [`prism`] is `1e5·ε` across, and certifies at EVERY ε row this
//!   repo runs, in the schedule's first round;
//! * [`exhausting_prism`] is `1e11·ε` across, and its schedule cannot
//!   converge at any of them.
//!
//! Each row then ASSERTS the arm it takes rather than branching on it.
//! A silent `Err` arm is not a row.
//!
//! # Cost
//!
//! Cost here is one number: a certified quadrature over this body, and
//! how many of them the suite runs. Everything else — the loft, the
//! ball, the verdict log — is under a millisecond, measured. So the
//! suite is written to run as few as its claims need: the ε-scaling
//! above puts the body in the schedule's first round at every ε, the
//! loft is the cheapest one that still has a rational wall (two
//! sections at `v`-degree 1 — half the cost of the three-section
//! degree-2 spelling, measured, and the wall is rational either way),
//! and no row calls a door whose answer another door's answer already
//! is.
//!
//! In particular `validate_geometric` is not called for its COUNT: it
//! IS `validate_geometric_certificate(..).map(|_| ())`, so counting it
//! counts one function twice and buys a quadrature's worth of nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// The doors under test (`validate.rs`), the quadrature they run
// (`topo`'s lane dispatch and `geom-brep`'s props lane), and the
// reader that consumes the tier-3′ door's certificate. `sweep`'s own
// sources are deliberately NOT named: this suite's subject is the
// doors, and the loft is a fixture builder whose breakage every other
// sweep row catches first. The helper module IS named — a marker's own
// file is implicit, a sibling helper module is not.
test_utils::gated_to![
    "crates/topo/src/validate.rs",
    "crates/topo/src/props.rs",
    "crates/geom-brep/src/props/",
    "crates/step-import/src/lib.rs",
    "crates/sweep/tests/common/",
];

use crate::common::{arc_section, stacked};
use geom_core::k_stats::{start_verdict_log, take_verdict_log};
use geom_core::{Point2, Tol};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::loft_body;
use topo::{Body, MassProperties};

/// The arc PRISM at scale `s`: two identical arc sections stacked and
/// skinned at `v`-degree 1. Its `+x` wall is RATIONAL, which is what
/// puts every enclosure below on the quadrature lane rather than on a
/// closed form.
///
/// Two sections and not three: the wall is rational either way, and the
/// extra station doubles the quadrature's cost for a property no row
/// here reads (the extrusion oracle is `m8_3_rational_volume`'s claim,
/// and that row keeps the three-section spelling for it).
fn arc_prism(s: f64) -> Body<f64> {
    loft_body::<f64>(
        &[arc_section(s), arc_section(s)],
        &stacked(&[0.0, 1.0], s),
        1,
        Tol::witness(),
    )
    .expect("the arc prism lofts")
    .body
}

/// **The body the certifying row measures** — the arc prism at `1e5·ε`.
///
/// SIZE is ε's partner. The quadrature's convergence target is
/// `1024·ε` on a LENGTH (the flux width over three times the area), and
/// that length scales as the body does while the target does not, so a
/// prism scaled by `k·ε` sits a FIXED factor from its target at every
/// ε — one posture at every ε row, which is what the module docs'
/// vacuity argument needs.
///
/// `k = 1e5` is four to five orders below the exhaustion threshold
/// (`9.3e9·ε`, [`exhausting_prism`]) and five orders above ε itself, so
/// neither end of the window is near. It converges in the schedule's
/// FIRST round at every ε row — measured: the same 8 quadrature
/// verdicts and the same ~0.43 s a call at 1e-9, 1e-6 and 1e-12, where
/// `k = 5e7` costs 13 verdicts and ~0.86 s for no extra claim.
fn prism() -> Body<f64> {
    arc_prism(1.0e5 * Tol::witness().get().eps)
}

/// **The prism whose schedule cannot converge, at whichever ε the run
/// committed to** — the planted quadrature refusal.
///
/// The same scaling argument as [`prism`], read from the other side.
/// The window is two-sided and both ends are real:
///
/// * from BELOW, the unit prism's own end-of-schedule length is
///   ~1.1e-7 m, so the schedule exhausts once `s > 9.3e9·ε`;
/// * from ABOVE, the loft's carrier certification is a RELATIVE
///   residual, so a body more than ~1e15·ε across cannot be built at
///   all (it refuses `ResidualExceeded` before any quadrature runs).
///
/// `1e11·ε` sits an order above the first bound and four below the
/// second, at every ε row this repo runs.
fn exhausting_prism() -> Body<f64> {
    arc_prism(1.0e11 * Tol::witness().get().eps)
}

/// A ball: two rimless spherical bands. Whole-body inversion of a
/// rimless body is the one planted inversion that reaches CHECK 7 —
/// the curved sense arm has no rim to read, so the structural half
/// stays clean and the negative volume is what refuses.
fn ball() -> Body<f64> {
    use sweep::{Revolution, RevolveAxis, revolve};
    let lp = ProfileLoop::new(vec![
        // A half-circle bulge: the meridian of the ball.
        ProfileVertex::new(Point2::new(0.0, -1.0), 1.0),
        ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the meridian profile validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the ball revolves")
    .body
}

/// Every face's sense inverted — the whole-body inversion.
fn flip_all(body: &Body<f64>) -> Body<f64> {
    let keys: Vec<_> = body.faces().map(|(k, _)| k).collect();
    keys.iter().fold(body.clone(), |b, &k| {
        b.flipped_face_sense_for_tests(k).expect("live face key")
    })
}

/// The number of quadrature-lane classifications recorded while
/// `run` executed — the certificate COUNTER.
///
/// The verdict log is the kernel's own recording channel
/// (`geom_core::k_stats`), on at every scalar and written by the one
/// classification funnel, so this counts the decisions a certified
/// quadrature actually makes rather than timing it. One certificate
/// over one body at one band contributes a fixed number of
/// `props_quad_*` verdicts; two contribute twice that.
fn quad_verdicts(run: impl FnOnce()) -> usize {
    start_verdict_log();
    run();
    take_verdict_log()
        .iter()
        .filter(|v| v.predicate.starts_with("props_quad"))
        .count()
}

/// The four fields, as raw bits — the identity currency. `volume` and
/// `surface_area` are `f64` here because the doors under test are the
/// `f64` lane's; a comparison of anything narrower would let a
/// divergence hide inside a pad.
fn bits(m: &MassProperties<f64>) -> [u64; 4] {
    [
        m.volume.to_bits(),
        m.surface_area.to_bits(),
        m.volume_pad.to_bits(),
        m.area_pad.to_bits(),
    ]
}

/// **IDENTITY / ONE CERTIFICATE** — the gate's certificate IS the
/// measurement, bit for bit, and costs exactly one certified
/// quadrature.
///
/// One body, one build, both properties on it: nextest is
/// process-per-test, so a second row here would rebuild this prism and
/// re-run its quadrature in full (`memories/test-suite-cost.md`).
/// Every assertion therefore NAMES its property — `IDENTITY`,
/// `ONE CERTIFICATE` — so the message alone says which one broke.
///
/// **The row asserts the arm it takes.** [`prism`] certifies at every ε
/// row by construction, so each door's `Ok` is `expect`ed rather than
/// matched: a refusal here is this row FAILING, never this row having
/// nothing to compare. That is the module docs' vacuity argument, in
/// three `expect`s.
///
/// **IDENTITY.** The returned properties are the object check 7 decided
/// on, so the comparison is an identity rather than an agreement: the
/// same `mass_properties_impl` over the same face-arena order against
/// the same `Band::linear(tol)`, dispatched to the same
/// `quad_lane::cut_face`. A single differing ulp would be a real
/// finding, not a tolerance question. The claim is about the QUADRATURE
/// lane and the row proves it is there — a nonzero `volume_pad` is a
/// certified enclosure and nothing else produces one.
///
/// **ONE CERTIFICATE.** Counted, not timed. `k_stats`' verdict log
/// records every classification the kernel's one funnel makes, so the
/// `props_quad_*` verdicts of a call are a deterministic function of
/// the certificates it ran: one measurement's count is the unit, and
/// each returning door costs exactly that. A caller that gated a body
/// and then measured it therefore paid twice that, and now pays once —
/// which is this unit.
#[test]
fn the_gates_certificate_is_the_measurement_and_costs_one_quadrature() {
    let body = prism();
    let tol = Tol::witness();

    let mut measured = None;
    let one = quad_verdicts(|| measured = Some(topo::mass_properties(&body, tol)));
    let measured = measured.expect("the closure ran").expect(
        "IDENTITY: the fixture is scaled to 1e5·ε so that it certifies at EVERY ε row — \
         a refusal here is a moved schedule or a moved fixture, not an ε this row may \
         quietly skip",
    );

    let mut gated = None;
    let gate = quad_verdicts(|| gated = Some(topo::validate_geometric_certificate(&body, tol)));
    let gated = gated.expect("the closure ran").expect(
        "IDENTITY: the returning tier-3 door must certify the body its own measurement \
         door just certified",
    );

    let mut gated3 = None;
    let gate3 = quad_verdicts(|| {
        gated3 = Some(topo::validate_pseudomanifold_certificate(
            &body,
            &Default::default(),
            tol,
        ));
    });
    let gated3 = gated3
        .expect("the closure ran")
        .expect("IDENTITY: and so must the tier-3′ door, which is the one the import path pays");

    // ---- ONE CERTIFICATE ----
    assert!(
        one > 0,
        "ONE CERTIFICATE: the counter must see this body's quadrature at all — \
         {one} quadrature verdicts for one measurement"
    );
    assert_eq!(
        gate, one,
        "ONE CERTIFICATE: the returning door must run ONE certificate, the same work one \
         measurement is — so gate-then-measure was {one} + {one} verdicts for a number \
         the gate already held"
    );
    assert_eq!(
        gate3, one,
        "ONE CERTIFICATE: and so must the tier-3′ door, which is the one the import path \
         pays"
    );

    // ---- IDENTITY ----
    assert!(
        measured.volume_pad > 0.0,
        "IDENTITY: this row's claim is about the QUADRATURE lane, so the body must carry \
         a certified enclosure: pad {}",
        measured.volume_pad
    );
    assert_eq!(
        bits(&gated),
        bits(&measured),
        "IDENTITY: tier 3's certificate must BE the measurement, in all four fields: \
         gate {gated:?} vs measurement {measured:?}"
    );
    assert_eq!(
        bits(&gated3),
        bits(&measured),
        "IDENTITY: tier 3′'s certificate must BE the measurement, in all four fields: \
         gate {gated3:?} vs measurement {measured:?}"
    );
}

/// **PLANTED** — no gate is weakened: the class that refuses a planted
/// body is the planted one, through every tier-3 door.
///
/// One planted body per refusal class the doors reach, asserted through
/// BOTH tiers and through BOTH the composed door and the returning one.
/// The classes:
///
/// * **check 7's own, on the value** — an inverted rimless body, whose
///   volume is genuinely negative;
/// * **check 7's own, on the derivation** — a body whose certified
///   quadrature cannot reach its target, which arrives as
///   `VolumeUncomputable`;
/// * **the structural half** — a curved face with its sense inverted,
///   which CHECK 4's material arm refuses as a lamina before check 7 is
///   ever consulted, so the returning door computes NO certificate on
///   this arm at all.
///
/// # Two things this row deliberately does NOT assert
///
/// **That the returning door's verdicts equal the composed door's.**
/// They are one function: `validate_geometric` IS
/// `validate_geometric_certificate(..).map(|_| ())`, and
/// `validate_pseudomanifold` is that same shape one tier up, so the
/// equality is `Result::map`'s and holds for every input by
/// construction. What the four calls buy is the CLASS, pinned once per
/// door — evidence about the battery, which is the thing that can
/// actually move.
///
/// **That a refusing arm returns no properties.** The return type
/// carries it: `Result<MassProperties<T>, Vec<ValidationError>>` has no
/// arm that is both an `Err` and a certificate, so asserting it is a
/// codomain assertion — a deletion, in `memories/test-suite-cost.md`'s
/// terms. The claim survives where it is a claim: in the type, and in
/// `validate_geometric_certificate`'s doc.
#[test]
fn a_refusing_arm_returns_no_properties_through_either_door() {
    let tol = Tol::witness();

    let ball = ball();
    let inverted = flip_all(&ball);
    let exhausted = exhausting_prism();
    // Half of the ball inverted: the two bands then meet material-side
    // to material-side at their shared edges, which check 4's material
    // arm refuses as a lamina — a STRUCTURAL refusal, raised before
    // check 7 is consulted at all.
    let half_flipped = {
        let band = ball.faces().next().expect("the ball has faces").0;
        ball.flipped_face_sense_for_tests(band)
            .expect("live face key")
    };

    for (label, body, wanted) in [
        (
            "inverted ball",
            &inverted,
            topo::ValidationError::NegativeVolume,
        ),
        (
            "exhausted schedule",
            &exhausted,
            topo::ValidationError::VolumeUncomputable {
                // Only the CLASS is pinned: which of budget or
                // escalation the schedule reaches is an ε question and
                // the class is not.
                source: topo::MassPropsError::Corrupt { what: "" },
            },
        ),
        (
            "half-inverted ball",
            &half_flipped,
            topo::ValidationError::LaminaWedge {
                edge: half_flipped.edges().next().expect("the ball has edges").0,
            },
        ),
    ] {
        let old = topo::validate_geometric(body, tol);
        let new = topo::validate_geometric_certificate(body, tol).map(|_| ());
        let old3 = topo::validate_pseudomanifold(body, &Default::default(), tol);
        let new3 =
            topo::validate_pseudomanifold_certificate(body, &Default::default(), tol).map(|_| ());

        for (door, got) in [
            ("tier 3, composed door", &old),
            ("tier 3, returning door", &new),
            ("tier 3′, composed door", &old3),
            ("tier 3′, returning door", &new3),
        ] {
            let errors = got.as_ref().err().unwrap_or_else(|| {
                panic!("PLANTED {label} through the {door}: the door must refuse")
            });
            let found = errors
                .iter()
                .any(|e| std::mem::discriminant(e) == std::mem::discriminant(&wanted));
            assert!(
                found,
                "PLANTED {label} through the {door}: the planted class must be the one \
                 that refuses, got {errors:?}"
            );
        }
    }
}
