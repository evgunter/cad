//! **Review probes for MESH-12 (R1): does the parse's span decide
//! answer what certification answers?** (issue 1601, PR 1617.)
//!
//! The unit's claim is that `props_meridian_span_winding` re-decides
//! certification's `interval_span_winding` with the same margin, band
//! and lever, "hence the same dispositions". These rows execute both
//! decides on one ladder of spans and compare them rung for rung, at
//! whatever ε the run carries, on both scalars — a claim of the shape
//! "two sites agree" is only checked by running both sites.
//!
//! Every offset is derived from the run's own `Band`; no ε literal.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face, require_one_chart_branch};
use geom_brep::{CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec};
use geom_core::{Band, Point3, Real, Tol, Vec3};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
/// The sphere and the meridian carrier under every row: R = 10 mm.
const RS: f64 = 0.010;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn f<T: Real>(x: f64) -> T {
    T::from_f64(x)
}
fn sphere<T: Real>() -> Surface<T> {
    Surface::Sphere {
        center: Point3::new(f(0.0), f(0.0), f(0.0)),
        radius: f(RS),
        axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
        u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
    }
}
/// The meridian great circle at azimuth `u`, radius `r` (the sphere's
/// unless a row is asking what happens when it is not).
fn meridian<T: Real>(u: f64, r: f64) -> Curve3<T> {
    Curve3::Circle {
        center: Point3::new(f(0.0), f(0.0), f(0.0)),
        axis: Vec3::new(f(u.sin()), f(-u.cos()), f(0.0)),
        radius: f(r),
        u_ref: Vec3::new(f(u.cos()), f(u.sin()), f(0.0)),
    }
}
fn arc<T: Real>(carrier: Curve3<T>, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
    LoopEdge::hand_built(carrier, f(t0), f(t1), true, a, b)
}
/// A rimless pair on one meridian: the arc `[t0, t0 + dt]` and the
/// complement back to `t0 + 4π`.
fn pair<T: Real>(t0: f64, dt: f64) -> Vec<LoopEdge<T>> {
    vec![
        arc(meridian(0.0, RS), t0, t0 + dt, 0, 1),
        arc(meridian(0.0, RS), t0 + dt, t0 + 4.0 * PI, 1, 0),
    ]
}

/// The three dispositions both decides share, as one vocabulary.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rung {
    Admits,
    Escalates,
    Refuses,
}

/// Certification's answer for a circle arc of this span, through
/// `EdgeCurve::certify` — the same `run_checks` every public door
/// runs, with check 2 (`interval_span_winding`) the one under test.
fn certification(t0: f64, dt: f64, r: f64, band: Band) -> Rung {
    let carrier = meridian::<f64>(0.0, r);
    let spec = EdgeCurveSpec::arc_of_circle(carrier.clone(), t0, t0 + dt).unwrap();
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t0 + dt));
    match EdgeCurve::certify(spec, p0, p1, |_| None, band) {
        Ok(_) => Rung::Admits,
        Err(CertifyError::WindingExceeded) => Rung::Refuses,
        Err(CertifyError::Escalated {
            check: CertCheck::ParamSpan,
            ..
        }) => Rung::Escalates,
        Err(e) => panic!("dt = τ + {:e}: not a span verdict at all: {e:?}", dt - TAU),
    }
}

/// Certification's answer for the small circle the lever row uses:
/// the same check on a carrier whose radius is not the sphere's.
fn certification_on(r_c: f64, t0: f64, dt: f64, band: Band) -> Rung {
    let carrier = Curve3::Circle {
        center: Point3::new(RS * (0.75f64).sqrt(), 0.0, 0.0),
        axis: Vec3::new(1.0, 0.0, 0.0),
        radius: r_c,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let spec = EdgeCurveSpec::arc_of_circle(carrier.clone(), t0, t0 + dt).unwrap();
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t0 + dt));
    match EdgeCurve::certify(spec, p0, p1, |_| None, band) {
        Ok(_) => Rung::Admits,
        Err(CertifyError::WindingExceeded) => Rung::Refuses,
        Err(CertifyError::Escalated {
            check: CertCheck::ParamSpan,
            ..
        }) => Rung::Escalates,
        Err(e) => panic!("not a span verdict at all: {e:?}"),
    }
}

/// The parse's answer for the same span, through the flux lane.
fn parse(t0: f64, dt: f64, band: Band) -> Rung {
    disposition(&curved_face(
        &sphere::<f64>(),
        &pair::<f64>(t0, dt),
        1.0,
        band,
    ))
}

fn disposition<V: core::fmt::Debug>(r: &Result<V, PropsError>) -> Rung {
    match r {
        Ok(_) => Rung::Admits,
        Err(PropsError::NotIsoRectangle { what }) if *what == "props_meridian_span_winding" => {
            Rung::Refuses
        }
        Err(PropsError::Escalated { cause })
            if cause.predicate == Some("props_meridian_span_winding") =>
        {
            Rung::Escalates
        }
        // Any other verdict is a verdict about something else; the
        // ladder only compares the span decide, so a row that lands
        // here is a mismatch by construction and prints as one.
        Err(e) => panic!("the span decide did not answer: {e:?}"),
    }
}

/// **The two decides agree rung for rung on the brief's ladder.**
/// `τ + {0.5, 0.99}·zero/R` admits at both, `1.01·zero/R … 9.9·zero/R`
/// escalates at both, `10.1·zero/R` refuses at both. Prints
/// `R1-LADDER` lines so the same ladder can be read off another tree.
#[test]
fn r1_the_span_decide_and_certification_agree_rung_for_rung() {
    let bd = band();
    let t0 = 0.3;
    let z = bd.zero() / RS;
    for k in [-1.0, -0.001, 0.0, 0.5, 0.99, 1.01, 5.0, 9.9, 10.1, 40.0] {
        let dt = TAU + k * z;
        let cert = certification(t0, dt, RS, bd);
        let props = parse(t0, dt, bd);
        println!(
            "R1-LADDER k={k} dt-tau={:e} cert={cert:?} props={props:?}",
            dt - TAU
        );
        assert_eq!(cert, props, "k = {k}: the two decides disagree");
    }
}

/// **A search for a span where they disagree**, finer than the named
/// rungs: 4001 spans across the whole coincidence-and-ambiguity
/// window and well past it. If the "same margin, same band, same
/// lever" claim is exact this finds nothing; if the two sites round
/// the margin differently it finds the first offender and names it.
#[test]
fn r1_no_span_on_a_fine_sweep_splits_the_two_decides() {
    let bd = band();
    let z = bd.zero() / RS;
    let mut split = Vec::new();
    for i in 0..=4000 {
        let k = -2.0 + 0.01 * f64::from(i);
        let dt = TAU + k * z;
        if dt <= 0.0 {
            continue;
        }
        let (cert, props) = (certification(0.3, dt, RS, bd), parse(0.3, dt, bd));
        if cert != props {
            split.push(format!("k={k} cert={cert:?} props={props:?}"));
        }
    }
    assert!(
        split.is_empty(),
        "R1-SPLIT {:?}",
        &split[..split.len().min(8)]
    );
}

/// **The lever is the SPHERE's radius, not the carrier's — and the
/// branch door has a carrier where those differ.** Certification
/// meters a circle arc's span at the CARRIER's radius; the new decide
/// meters it at the SPHERE's. The flux lane pins them equal
/// (`props_meridian_great` refuses a meridian that is not the great
/// circle), but the branch door classifies a meridian by its axis
/// alone: a SMALL circle cut from the sphere by a plane parallel to
/// the axis lies exactly on the surface, has `n_c · axis = 0`, and is
/// read as a meridian here. On it the two decides are metered a
/// factor `R / r_c` apart, and this row exhibits a span the door
/// refuses that certification only escalates on.
#[test]
fn r1_the_branch_doors_span_lever_is_not_the_carriers() {
    let bd = band();
    // The circle the plane x = R·√3/2 cuts from the sphere: on the
    // surface, radius R/2, axis x̂ — perpendicular to the sphere's.
    let r_c = RS / 2.0;
    let d = RS * (0.75f64).sqrt();
    let small = |t0: f64, t1: f64, a: u32, b: u32| {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: Point3::new(d, 0.0, 0.0),
                axis: Vec3::new(1.0, 0.0, 0.0),
                radius: r_c,
                u_ref: Vec3::new(0.0, 1.0, 0.0),
            },
            t0,
            t1,
            true,
            a,
            b,
        )
    };
    // Every point of this circle is on the sphere, so it is a carrier
    // certification would accept as lying on the surface.
    for t in [0.0, 1.0, 2.5, 4.0] {
        let p = Curve3::Circle {
            center: Point3::new(d, 0.0, 0.0),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: r_c,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        }
        .eval(t);
        assert!(((p - Point3::new(0.0, 0.0, 0.0)).norm() - RS).abs() < 1e-15);
    }
    // A span whose headroom is 1.5·escalate at the SPHERE's lever and
    // 0.75·escalate at the CARRIER's: definitely negative at one meter,
    // in the ambiguity band at the other.
    let dt = TAU + 1.5 * bd.escalate() / RS;
    let cert = certification_on(r_c, 0.3, dt, bd);
    let edges = vec![
        small(0.3, 0.3 + dt, 0, 1),
        small(0.3 + dt, 0.3 + 4.0 * PI, 1, 0),
    ];
    let door = require_one_chart_branch(&sphere::<f64>(), &edges, bd);
    println!("R1-LEVER on-sphere small circle: cert={cert:?} door={door:?}");
    assert_eq!(cert, Rung::Escalates, "certification's own meter");
    assert!(
        matches!(
            door,
            Err(PropsError::NotIsoRectangle {
                what: "props_meridian_span_winding"
            })
        ),
        "the branch door meters this span at the sphere radius: {door:?}"
    );
}

/// **A rim's span is not decided by this key, past the period or
/// not.** The unit's control row uses a full-period rim; this one
/// takes a rim of a span-and-a-half, which is the case issue 1618
/// says is still read raw. The parse must not report the meridian
/// name on it — and, per that issue, must not report anything.
#[test]
fn r1_a_rim_span_past_the_period_is_not_refused_by_the_meridian_decide() {
    let bd = band();
    let v = 0.3;
    let cap: Vec<LoopEdge<f64>> = vec![
        LoopEdge::hand_built(
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, RS * v.sin()),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: RS * v.cos(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            0.0,
            3.0 * PI,
            true,
            0,
            0,
        ),
        arc(meridian::<f64>(0.0, RS), v, PI / 2.0, 0, 1),
        arc(meridian::<f64>(PI, RS), PI / 2.0, v, 1, 0),
    ];
    let flux = curved_face(&sphere::<f64>(), &cap, 1.0, bd);
    let door = require_one_chart_branch(&sphere::<f64>(), &cap, bd);
    println!("R1-RIM-PAST-TAU flux={flux:?} door={door:?}");
    for r in [format!("{flux:?}"), format!("{door:?}")] {
        assert!(
            !r.contains("props_meridian_span_winding"),
            "a rim span reached the meridian decide: {r}"
        );
    }
}

/// **The admitted rungs reach the retired clamp, and the fold's
/// answer there is the hemisphere pair's.** Prints one `R1-FOLD`
/// line per rung so the identical file can be run on another tree and
/// the numbers diffed: the clamp `min(dt/2, π)` was inert for every
/// `dt ≤ τ`, so the ONLY window where its deletion can move an
/// admitted answer is `τ < dt ≤ τ + zero/R`, and the pair puts one
/// edge on each side of τ at every rung below.
#[test]
fn r1_the_admitted_window_measures_the_hemisphere_pair() {
    let bd = band();
    let exact = 2.0 * PI * RS * RS;
    let z = bd.zero() / RS;
    for k in [-0.99, -0.5, -0.1, 0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 0.99] {
        let dt = TAU + k * z;
        let fc = curved_face(&sphere::<f64>(), &pair::<f64>(0.3, dt), 1.0, bd)
            .unwrap_or_else(|e| panic!("k = {k}: the flux lane must answer: {e:?}"));
        let rel = (fc.area - exact).abs() / exact;
        println!("R1-FOLD k={k} area={:.17e} rel={rel:e}", fc.area);
        assert!(rel < 1e-12, "k = {k}: area {:e} != {exact:e}", fc.area);
    }
}

/// **The same ladder on the interval scalar.** The margin
/// `(τ − Δt)·R` is an enclosure there, so a rung could widen across a
/// band boundary; this row asserts it does not, on the rungs the
/// scalar lane admits and refuses.
#[cfg(feature = "interval")]
#[test]
fn r1_the_ladder_holds_at_the_interval_scalar() {
    use geom_core::Interval;
    let bd = band();
    let z = bd.zero() / RS;
    for (k, want) in [
        (0.5, Rung::Admits),
        (0.99, Rung::Admits),
        (40.0, Rung::Refuses),
    ] {
        let dt = TAU + k * z;
        let got = disposition(&curved_face(
            &sphere::<Interval>(),
            &pair::<Interval>(0.3, dt),
            Interval::from_f64(1.0),
            bd,
        ));
        println!("R1-LADDER-INTERVAL k={k} props={got:?}");
        assert_eq!(got, want, "k = {k}");
    }
}

/// **Every admitted span's answer, as one digest.** The clamp's
/// deletion changed the arithmetic on the window `τ < dt ≤ τ + zero/R`
/// and nowhere else; this row sweeps a grid of span starts and span
/// lengths across and around that window — including the pole
/// positions the review's 400-δ sweep varied — and prints an FNV-1a
/// digest over every answered flux and area. Run the identical file
/// on the merge base and diff the one line: a moved answer on ANY
/// admitted span shows as a changed digest, and a printed count says
/// how many spans were answered at all.
#[test]
fn r1_the_admitted_spans_answers_digest_to_one_line() {
    let bd = band();
    let z = bd.zero() / RS;
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut fnv = |bytes: [u8; 8]| {
        for b in bytes {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x100_0000_01b3);
        }
    };
    // The admitted set, stated arithmetically rather than read off
    // the result, so the same spans are digested on a tree that has
    // no refusal to read: definite-positive or in-band headroom.
    let admits = |span: f64| {
        let headroom = (TAU - span) * RS;
        headroom.abs() <= bd.zero() || headroom >= bd.escalate()
    };
    let mut answered = 0u32;
    for i in 0..40 {
        let t0 = -PI + 0.157 * f64::from(i);
        for j in -60..=60 {
            let dt = TAU + 0.02 * f64::from(j) * z;
            if !admits(dt) || !admits(4.0 * PI - dt) {
                continue;
            }
            let fc = curved_face(&sphere::<f64>(), &pair::<f64>(t0, dt), 1.0, bd)
                .unwrap_or_else(|e| panic!("t0 = {t0}, dt − τ = {:e}: {e:?}", dt - TAU));
            answered += 1;
            fnv(fc.flux.to_bits().to_le_bytes());
            fnv(fc.area.to_bits().to_le_bytes());
        }
    }
    println!("R1-DIGEST answered={answered} h={h:016x}");
    assert!(answered >= 1000, "the sweep answered only {answered} spans");
}
