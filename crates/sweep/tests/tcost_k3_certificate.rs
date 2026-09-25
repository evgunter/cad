//! **The tier-3 doors' returned certificate** — one certified
//! quadrature per body per gate.
//!
//! A body that is gated at rest and then measured used to pay two
//! certified quadratures for one number: check 7 computes a full
//! `MassProperties` to decide the +V invariant and drops it, so a
//! caller that also wants the enclosure runs the identical computation
//! again. `topo::validate_geometric_certificate` and
//! `topo::validate_pseudomanifold_certificate` return what the gate
//! computed — a `SignCertificate`, refined as far as each solid's sign
//! needed — and the caller who wants the number continues it.
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
//! `ONE READ PER FACE`, `ROUND SPLIT`, `PLANTED`.
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

use crate::common::{arc_section, quad_verdicts, stacked};
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

/// The arc prism at scale `s`, translated `dx·s` along `+x` — a solid
/// that can share a body with one at the origin without touching it.
fn arc_prism_at(s: f64, dx: f64) -> Body<f64> {
    loft_body::<f64>(
        &[arc_section(s), arc_section(s)],
        &[0.0, 1.0]
            .map(|h| geom_core::Affine3::translation(geom_core::Vec3::new(dx * s, 0.0, h * s))),
        1,
        Tol::witness(),
    )
    .expect("the translated arc prism lofts")
    .body
}

/// `solids` grafted into one body, in the order given — each its own
/// solid, so check 7 walks them apart and the certificate assembles.
fn grafted(solids: &[Body<f64>]) -> Body<f64> {
    let mut body = Body::new();
    for solid in solids {
        topo::graft_disjoint(&mut body, solid, Tol::witness()).expect("a disjoint graft");
    }
    body
}

/// **Two certifying prisms in one body** — [`prism`] and its twin
/// four widths along `+x`, grafted as two solids. The multi-solid
/// subject: check 7 decides each solid on its own faces, so the tier-3′
/// door's certificate is two walks assembled, and whether it takes any
/// FURTHER read of the arena is what `ONE READ PER FACE` counts.
fn prism_pair() -> Body<f64> {
    grafted(&[prism(), arc_prism_at(1.0e5 * Tol::witness().get().eps, 4.0)])
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
/// **IDENTITY.** The returned certificate is the object check 7 decided
/// on, CONTINUED to the reporting target — so the comparison is an
/// identity rather than an agreement: the same face walk over the same
/// face-arena order against the same `Band::linear(tol)`, dispatched
/// to the same `quad_lane::cut_face_rounds`, over the same rounds. A single
/// differing ulp would be a real finding, not a tolerance question.
/// The claim is about the QUADRATURE lane and the row proves it is
/// there — a nonzero `volume_pad` is a certified enclosure and nothing
/// else produces one.
///
/// **ONE CERTIFICATE.** Counted, not timed. `k_stats`' verdict log
/// records every classification the kernel's one funnel makes, so the
/// `props_quad_*` verdicts of a call are a deterministic function of
/// the rounds it ran: one measurement's count is the unit. The gate
/// pays only the rounds its own certification needs — never more than
/// the measurement — and the continuation pays the rest, so the two
/// together are the measurement's count exactly. A caller that gated a
/// body and then measured it used to pay twice the unit; it pays it
/// once. That the gate is sometimes STRICTLY cheaper is the claim of
/// `sign_walk_plus_v`, which rosters bodies whose schedules run
/// past round 0; this prism's do not, and a strict inequality asserted
/// here would be asserting a property of the fixture.
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
    let mut continued = None;
    let refine = quad_verdicts(|| continued = Some(gated.refine_to_target()));
    let continued = continued.expect("the closure ran").expect(
        "IDENTITY: the continuation of a certificate the measurement door certified must \
         certify too — it is the same rounds",
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
    let mut continued3 = None;
    let refine3 = quad_verdicts(|| continued3 = Some(gated3.refine_to_target()));
    let continued3 = continued3.expect("the closure ran").expect(
        "IDENTITY: the tier-3′ certificate's continuation must certify too — it is the same \
         rounds",
    );

    // ---- ONE CERTIFICATE ----
    assert!(
        one > 0,
        "ONE CERTIFICATE: the counter must see this body's quadrature at all — \
         {one} quadrature verdicts for one measurement"
    );
    assert!(
        gate <= one,
        "ONE CERTIFICATE: the gate certifies a SIGN, so it can never pay MORE rounds than \
         the measurement it stops inside of — {gate} gate verdicts against {one}"
    );
    assert_eq!(
        gate + refine,
        one,
        "ONE CERTIFICATE: the gate's {gate} verdicts plus the continuation's {refine} must \
         be the measurement's {one} — a continuation that re-ran a round the gate already \
         paid for would exceed it, and one that skipped a round would fall short"
    );
    assert_eq!(
        gate3 + refine3,
        one,
        "ONE CERTIFICATE: and so must the tier-3′ door's {gate3} plus its continuation's \
         {refine3} — the tier-3′ door is the one the import path pays"
    );

    // ---- IDENTITY ----
    assert!(
        measured.volume_pad > 0.0,
        "IDENTITY: this row's claim is about the QUADRATURE lane, so the body must carry \
         a certified enclosure: pad {}",
        measured.volume_pad
    );
    assert_eq!(
        bits(&continued),
        bits(&measured),
        "IDENTITY: tier 3's certificate, continued to the reporting target, must BE the \
         measurement, in all four fields: gate {continued:?} vs measurement {measured:?}"
    );
    assert_eq!(
        bits(&continued3),
        bits(&measured),
        "IDENTITY: tier 3′'s certificate, continued to the reporting target, must BE the \
         measurement, in all four fields: gate {continued3:?} vs measurement {measured:?}"
    );
}

/// **ONE READ PER FACE** — a multi-solid body's tier-3′ certificate
/// is its per-solid walks ASSEMBLED, and costs one measurement with its
/// continuation, not one plus a further arena-wide read.
///
/// Check 7 decides per solid, so no one solid's walk is the body's.
/// A door that wanted a whole-body NUMBER out of the gate itself would
/// have to read every face a second time; this door returns the sign
/// certificate its walks built and lets the caller continue it. The
/// count is what sees the difference — a second read agrees with the
/// first bit for bit, so no comparison of values can.
///
/// The pair's prisms each converge in the schedule's first round at
/// every ε row ([`prism`]), so the gate and its continuation together
/// must be the measurement's count exactly.
#[test]
fn a_multi_solid_certificate_reads_each_face_once() {
    let body = prism_pair();
    let tol = Tol::witness();
    assert_eq!(
        body.solids().count(),
        2,
        "ONE READ PER FACE: the subject is two solids"
    );

    let mut measured = None;
    let one = quad_verdicts(|| measured = Some(topo::mass_properties(&body, tol)));
    let measured = measured
        .expect("the closure ran")
        .expect("ONE READ PER FACE: two certifying prisms measure at every ε row");

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
        .expect("ONE READ PER FACE: two disjoint certifying prisms pass tier 3′");
    let mut continued = None;
    let refine = quad_verdicts(|| continued = Some(gated3.refine_to_target()));
    let continued = continued
        .expect("the closure ran")
        .expect("ONE READ PER FACE: the continuation of a certifying pair certifies");

    assert!(
        one > 0,
        "ONE READ PER FACE: the counter must see the pair's quadrature at all — {one}"
    );
    assert_eq!(
        gate3 + refine,
        one,
        "ONE READ PER FACE: the tier-3′ gate's {gate3} verdicts plus its continuation's \
         {refine} must be ONE measurement's {one} — more is a second read of the arena \
         behind the per-solid walks"
    );
    assert_eq!(
        bits(&continued),
        bits(&measured),
        "ONE READ PER FACE: the assembled certificate, continued, must BE the whole-body \
         measurement: {continued:?} vs {measured:?}"
    );
}

/// **ROUND SPLIT** — a multi-solid certificate whose parts stopped at
/// DIFFERENT rounds, assembled and continued, is the whole-body
/// measurement: the same four fields bit for bit, or the same typed
/// refusal naming the same face.
///
/// `ONE READ PER FACE`'s pair settles in round 0 and leaves nothing to
/// continue. Here one solid is the `1e5·ε` prism (finished at round 0)
/// and the other a `1e9·ε` prism, whose sign settles before its
/// schedule meets the target, so the continuation genuinely resumes one
/// part's faces and not the other's — asserted, so the row cannot pass
/// by having nothing to split. Both graft ORDERS, because the assembly
/// re-orders parts into arena order and an order-sensitive splice would
/// agree on one and not the other. The third subject pairs the `1e5·ε`
/// prism with the exhausting `1e11·ε` one: the gate admits it, and the
/// continuation must refuse exactly as `mass_properties` does.
///
/// Every body is ε-scaled, so each takes the same arm at every ε row.
#[test]
fn a_multi_solid_certificate_split_across_rounds_continues_to_the_measurement() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let small = || prism();
    let fine = || arc_prism_at(1.0e9 * eps, 3.0);
    let exhausting = || arc_prism_at(1.0e11 * eps, 3.0);
    for (label, body) in [
        ("small then fine", grafted(&[small(), fine()])),
        ("fine then small", grafted(&[fine(), small()])),
        ("small then exhausting", grafted(&[small(), exhausting()])),
    ] {
        assert_eq!(body.solids().count(), 2, "ROUND SPLIT {label}: two solids");
        let mut measured = None;
        let one = quad_verdicts(|| measured = Some(topo::mass_properties(&body, tol)));
        let measured = measured.expect("the closure ran");

        let mut gated = None;
        let gate = quad_verdicts(|| {
            gated = Some(topo::validate_pseudomanifold_certificate(
                &body,
                &Default::default(),
                tol,
            ));
        });
        let gated = gated.expect("the closure ran").unwrap_or_else(|errors| {
            panic!(
                "ROUND SPLIT {label}: both solids' signs are definite, so tier 3′ admits the \
                 pair: {errors:?}"
            )
        });
        let mut continued = None;
        let refine = quad_verdicts(|| continued = Some(gated.refine_to_target()));
        let continued = continued.expect("the closure ran");

        match (&continued, &measured) {
            (Ok(continued), Ok(measured)) => {
                assert!(
                    refine > 0,
                    "ROUND SPLIT {label}: the fine prism's sign settles before its target, so \
                     the continuation must resume rounds — {refine} verdicts"
                );
                assert_eq!(
                    gate + refine,
                    one,
                    "ROUND SPLIT {label}: gate {gate} + continuation {refine} must be one \
                     measurement's {one}"
                );
                assert_eq!(
                    bits(continued),
                    bits(measured),
                    "ROUND SPLIT {label}: the assembled certificate, continued, must BE the \
                     measurement: {continued:?} vs {measured:?}"
                );
            }
            (Err(continued), Err(measured)) => {
                assert!(
                    label.contains("exhausting"),
                    "ROUND SPLIT {label}: only the exhausting pair may refuse: {measured}"
                );
                assert_eq!(
                    continued, measured,
                    "ROUND SPLIT {label}: the continuation must refuse as the measurement \
                     does, naming the same face"
                );
            }
            _ => panic!(
                "ROUND SPLIT {label}: the continuation and the measurement disagree on \
                 whether the body measures: {continued:?} vs {measured:?}"
            ),
        }
        assert_eq!(
            continued.is_err(),
            label.contains("exhausting"),
            "ROUND SPLIT {label}: exactly the exhausting pair refuses"
        );
    }
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
///   quadrature cannot reach its target. No tier-3 door refuses it:
///   check 7 consumes a SIGN, and this body's sign is definite long
///   before its target, so every door certifies it and only a caller
///   continuing to the reporting target is refused — asserted below
///   through all four doors, which must AGREE;
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
/// **That a refusing arm returns no certificate.** The return type
/// carries it: `Result<SignCertificate<'_, T>, Vec<ValidationError>>` has no
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
            topo::ValidationError::NegativeVolume {
                solid: topo::SolidKey::default(),
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

    // ---- The exhausted schedule, which every door ADMITS ----
    //
    // The body is valid and its volume is hugely positive; what it
    // cannot do is reach `1024·ε` of mean boundary displacement. Check
    // 7 reads the sign of a volume enclosure, so every tier-3 door is
    // finished with it and says so, and the caller that asks for the
    // NUMBER is the one this schedule refuses — through each returning
    // door's continuation alike.
    for (door, got) in [
        (
            "tier 3, composed door",
            topo::validate_geometric(&exhausted, tol),
        ),
        (
            "tier 3′, composed door",
            topo::validate_pseudomanifold(&exhausted, &Default::default(), tol),
        ),
    ] {
        got.unwrap_or_else(|errors| {
            panic!(
                "PLANTED exhausted schedule through the {door}: check 7 decides a SIGN, \
                 and this body's sign is definite — a refusal here is the coupling to the \
                 reporting target coming back: {errors:?}"
            )
        });
    }
    for (door, certificate) in [
        (
            "tier 3, returning door",
            topo::validate_geometric_certificate(&exhausted, tol)
                .expect("the returning door agrees with the composed one"),
        ),
        (
            "tier 3′, returning door",
            topo::validate_pseudomanifold_certificate(&exhausted, &Default::default(), tol)
                .expect("the returning door agrees with the composed one"),
        ),
    ] {
        let refused = certificate.refine_to_target().expect_err(&format!(
            "PLANTED exhausted schedule through the {door}: the caller that asks for the \
             NUMBER is the one this schedule refuses"
        ));
        assert!(
            matches!(
                refused,
                topo::MassPropsError::Face {
                    source: geom_brep::PropsError::QuadratureBudget { .. },
                    ..
                }
            ),
            "PLANTED exhausted schedule through the {door}: the continuation's refusal \
             must be the schedule's own budget refusal, got {refused:?}"
        );
    }
}
