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
//! builds. `topo` cannot build one — `sweep` is above it.
//!
//! Every row NAMES its property in the assertion message, because the
//! rows share their expensive fixture (the aggregation rule in
//! `memories/test-suite-cost.md`): `IDENTITY`, `ONE CERTIFICATE`,
//! `REFUSAL`, `PLANTED`.
//!
//! # ε
//!
//! ε is the run's, not the row's — `Tol` is a witness, and the doors
//! under test take one. So no row pins a posture: each derives what it
//! expects from the measurement door on the same body at the same
//! tolerance and asserts the GATE agrees with it, which is the claim
//! anyway. The one place a fixed posture is needed (the quadrature's
//! refusal class) is bought by SIZE instead — `exhausting_prism`
//! below — which is ε-independent.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::{start_verdict_log, take_verdict_log};
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Section, loft_body};
use topo::{Body, MassProperties};

/// A unit square with a quarter-circle bulge on the `+x` side, scaled
/// by `s` — the arc-bearing profile whose lofted wall is RATIONAL
/// (weights `1, cos 22.5°, 1` over two 45° sub-arcs).
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

fn stack(z: [f64; 3], s: f64) -> Vec<Affine3<f64>> {
    z.map(|h| Affine3::translation(Vec3::new(0.0, 0.0, h * s)))
        .into()
}

/// The arc PRISM at scale `s`: three identical arc sections stacked,
/// so the loft reproduces an extrusion exactly. Its `+x` wall is
/// rational, which is what puts every enclosure below on the
/// quadrature lane rather than a closed form.
fn arc_prism(s: f64) -> Body<f64> {
    loft_body::<f64>(
        &[arc_section(s), arc_section(s), arc_section(s)],
        &stack([0.0, 1.0, 2.0], s),
        2,
        Tol::witness(),
    )
    .expect("the arc prism lofts")
    .body
}

/// **The body the certifying rows measure** — the arc prism at 5 cm.
///
/// Scale is this suite's cost dial and nothing else; the identity and
/// counting claims are scale-free. The quadrature's convergence target
/// is a LENGTH (the flux width over three times the area), so it scales
/// as the body does, and 5 cm is where the two ε extremes are BOTH
/// cheap: the schedule converges in a few rounds at ε = 1e-9 and 1e-6
/// (~0.9 s a call), and at 1e-12 it is out of reach from the start, so
/// the last-round bound refuses after round 0 (~0.85 s a call) instead
/// of running the schedule out. Measured, because the middle is the
/// expensive place to be: at 5 mm the body CERTIFIES at 1e-12 and this
/// row costs 30 s; at 50 cm it runs more rounds at every ε.
fn prism() -> Body<f64> {
    arc_prism(0.05)
}

/// **The prism whose schedule cannot converge, at whichever ε the run
/// committed to** — the planted quadrature refusal.
///
/// SIZE buys the refusal, and ε sets the size. The convergence target
/// is `1024·ε` on a LENGTH — the flux width over three times the area
/// — and that length scales as the body does while the target does
/// not, so a prism scaled by `k·ε` ends its schedule a fixed factor
/// above the target at every ε. The window is two-sided and both ends
/// are real:
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
/// **IDENTITY.** The returned properties are the object check 7
/// decided on, so the comparison is an identity rather than an
/// agreement: the same `mass_properties_impl` over the same face-arena
/// order against the same `Band::linear(tol)`, dispatched to the same
/// `quad_lane::cut_face`. A single differing ulp would be a real
/// finding, not a tolerance question. The claim is about the
/// QUADRATURE lane and the row proves it is there — a nonzero
/// `volume_pad` is a certified enclosure and nothing else produces
/// one.
///
/// **ONE CERTIFICATE.** Counted, not timed. `k_stats`' verdict log
/// records every classification the kernel's one funnel makes, so the
/// `props_quad_*` verdicts of a call are a deterministic function of
/// the certificates it ran: one measurement's count is the unit, and
/// the two returning doors and the composed door each cost exactly
/// that. A caller that gated a body and then measured it therefore
/// paid twice that, and now pays once — which is this unit.
///
/// At an ε where the schedule refuses, both doors refuse and the row
/// says so instead of pinning a posture (module docs); the counting
/// half holds either way.
#[test]
fn the_gates_certificate_is_the_measurement_and_costs_one_quadrature() {
    let body = prism();
    let tol = Tol::witness();

    let mut measured = None;
    let one = quad_verdicts(|| measured = Some(topo::mass_properties(&body, tol)));
    let measured = measured.expect("the closure ran");

    let mut gated = None;
    let gate = quad_verdicts(|| gated = Some(topo::validate_geometric_certificate(&body, tol)));
    let gated = gated.expect("the closure ran");

    let mut gated3 = None;
    let gate3 = quad_verdicts(|| {
        gated3 = Some(topo::validate_pseudomanifold_certificate(
            &body,
            &Default::default(),
            tol,
        ));
    });
    let gated3 = gated3.expect("the closure ran");

    let composed = quad_verdicts(|| {
        let _ = topo::validate_geometric(&body, tol);
    });

    // ---- ONE CERTIFICATE ----
    assert!(
        one > 0,
        "ONE CERTIFICATE: the counter must see this body's quadrature at all — \
         {one} quadrature verdicts for one measurement"
    );
    assert_eq!(
        gate, one,
        "ONE CERTIFICATE: the returning door must run ONE certificate, the same work \
         one measurement is"
    );
    assert_eq!(
        gate3, one,
        "ONE CERTIFICATE: and so must the tier-3′ door, which is the one the import \
         path pays"
    );
    assert_eq!(
        composed, one,
        "ONE CERTIFICATE: the composed door still runs one, so gate-then-measure was \
         {one} + {one} verdicts for a number the gate already held"
    );

    // ---- IDENTITY ----
    match (&measured, &gated, &gated3) {
        (Ok(m), Ok(g), Ok(g3)) => {
            assert!(
                m.volume_pad > 0.0,
                "IDENTITY: this row's claim is about the QUADRATURE lane, so the body \
                 must carry a certified enclosure: pad {}",
                m.volume_pad
            );
            assert_eq!(
                bits(g),
                bits(m),
                "IDENTITY: tier 3's certificate must BE the measurement, in all four \
                 fields: gate {g:?} vs measurement {m:?}"
            );
            assert_eq!(
                bits(g3),
                bits(m),
                "IDENTITY: tier 3′'s certificate must BE the measurement, in all four \
                 fields: gate {g3:?} vs measurement {m:?}"
            );
        }
        (Err(source), Err(errors), Err(errors3)) => {
            // The schedule's honest frontier at this ε: no certificate
            // exists to compare, and both doors say what the
            // measurement door said.
            let expected = vec![topo::ValidationError::VolumeUncomputable {
                source: source.clone(),
            }];
            assert_eq!(
                *errors, expected,
                "IDENTITY: a refusing measurement must reach the gate as its own verdict"
            );
            assert_eq!(*errors3, expected, "IDENTITY: and the same through tier 3′");
        }
        other => panic!(
            "IDENTITY: the measurement door and the two gates must agree on whether this \
             body certifies at this ε: {other:?}"
        ),
    }
}

/// **REFUSAL / PLANTED** — no gate is weakened, and a refusal carries
/// no blessed number.
///
/// One planted body per refusal class the doors reach, each asserted
/// through BOTH the old door and the new one: the verdict vectors must
/// be IDENTICAL — same rejections, same typed verdicts, same order —
/// and the returning door must hand back no properties on any of them.
/// The classes:
///
/// * **check 7's own, on the value** — an inverted rimless body, whose
///   volume is genuinely negative;
/// * **check 7's own, on the derivation** — a body whose certified
///   quadrature cannot reach its target, which arrives as
///   `VolumeUncomputable`;
/// * **the structural half** — a curved face with its sense inverted,
///   which check 6 refuses before check 7 is ever consulted, so the
///   returning door computes NO certificate on this arm at all.
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
                // Filled in per-body below; only the CLASS is pinned
                // here, because which of budget or escalation the
                // schedule reaches is an ε question and the class is
                // not.
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
        let new = topo::validate_geometric_certificate(body, tol);
        let old3 = topo::validate_pseudomanifold(body, &Default::default(), tol);
        let new3 = topo::validate_pseudomanifold_certificate(body, &Default::default(), tol);

        let errors = new
            .as_ref()
            .err()
            .unwrap_or_else(|| panic!("PLANTED {label}: the returning door must refuse"));
        assert_eq!(
            old.as_ref().err(),
            Some(errors),
            "PLANTED {label}: no gate is weakened — the returning door's verdicts must be \
             the composed door's, same classes in the same order"
        );
        assert_eq!(
            new3.as_ref().err(),
            old3.as_ref().err(),
            "PLANTED {label}: and the same through tier 3′"
        );
        assert!(
            new.is_err() && new3.is_err(),
            "REFUSAL {label}: a refusing arm returns no properties — a refusal carries \
             no blessed number"
        );
        let found = errors
            .iter()
            .any(|e| std::mem::discriminant(e) == std::mem::discriminant(&wanted));
        assert!(
            found,
            "PLANTED {label}: the planted class must be the one that refuses, got {errors:?}"
        );
    }
}
