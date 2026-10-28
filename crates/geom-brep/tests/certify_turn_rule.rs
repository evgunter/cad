//! **Certification's winding check is the turn rule** (`geom_core::period`):
//! an ellipse's extent is a whole turn only where the sup rate over the
//! turn's defect reads it within the band, and short of one or over one
//! only where the inf rate over the defect reads it definitely past.
//!
//! The fixture is the plane `z = 0` cutting a unit cylinder leaning
//! `acos(1/ratio)` in xz: the ellipse `a = ratio`, `b = 1` about the
//! origin, whose speed is `b` at `θ = 0` and `a` at `θ = π/2`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, StatedSpan, SurfaceKey,
    SurfacePair, WindingReading,
};
use geom_core::{Band, Point3, Vec3};
use slotmap::SlotMap;

const TAU: f64 = core::f64::consts::TAU;
const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;

/// The plane, the leaning cylinder, and the ellipse they cut.
struct Steep {
    surfaces: SlotMap<SurfaceKey, Surface<f64>>,
    pair: SurfacePair,
    ellipse: Curve3<f64>,
}

fn steep(ratio: f64) -> Steep {
    let alpha = (1.0f64 / ratio).acos();
    let mut surfaces: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let plane = surfaces.insert(Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    });
    let cyl = surfaces.insert(Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(alpha.sin(), 0.0, alpha.cos()),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    });
    let ellipse = Curve3::Ellipse {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        semi_u: 1.0 / alpha.cos(),
        semi_v: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    Steep {
        surfaces,
        pair: SurfacePair::new(plane, cyl),
        ellipse,
    }
}

impl Steep {
    fn certify(&self, t0: f64, t1: f64, band: Band) -> Result<EdgeCurve<f64>, CertifyError> {
        let e = &self.ellipse;
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: self.pair,
                selector: geom_brep::CurveSelector::Witness(e.eval(0.5 * (t0 + t1))),
            },
            carrier: e.clone(),
            param_start: t0,
            param_end: t1,
        };
        EdgeCurve::certify(
            spec,
            e.eval(t0),
            e.eval(t1),
            |k| self.surfaces.get(k).cloned(),
            1.0,
            band,
            geom_brep::NurbsLane::certified(),
        )
    }
}

/// The escalation a winding check raised, at a door (`StatedInterval`,
/// where its enclosure is wholly on the refused side) or as it stands.
fn winding_escalation(
    got: &Result<EdgeCurve<f64>, CertifyError>,
) -> Option<geom_core::Indeterminate> {
    match got {
        Err(CertifyError::StatedInterval {
            refused: StatedSpan::Winding(WindingReading::InBand(cause)),
        })
        | Err(CertifyError::Escalated {
            check: geom_brep::CertCheck::ParamWinding,
            cause,
            ..
        }) => Some(*cause),
        _ => None,
    }
}

/// **An ellipse wound `0.9e-9` rad past τ on its fast side does not
/// certify** (`certify-admits-an-ellipse-wound-past-its-period-on-the-slow-rate`).
/// At ε = 1e-9 the doubled stretch, centred at `θ = π/2` where the edge runs
/// at speed 20, is about `1.8e-8` m long, definitely past the band; read at
/// the speed floor `b = 1` it is `0.9e-9`, within it. The inf over the
/// stretch itself is within `20·0.9e-9` of the speed at its middle, so it
/// reads the stretch definitely past the band, and the edge is refused as
/// over a turn.
#[test]
fn an_ellipse_wound_past_its_period_on_the_fast_side_does_not_certify() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let fixture = steep(20.0);
    let over = 0.9e-9;
    let t0 = HALF_PI - 0.5 * over;
    let got = fixture.certify(t0, t0 + TAU + over, band);
    assert_eq!(
        new_winding(&got),
        (Winding::Exceeded, None),
        "the over-wound edge is refused as over a turn: {:?}",
        got.map(|_| "certified")
    );
}

/// What check 2's winding reached: certified (with the turn the
/// certificate keeps), refused as over a turn, or escalated.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Winding {
    Certified,
    Exceeded,
    Escalated,
}

/// **The oracle**: check 2's winding as it was spelled before the turn
/// rule — `(τ − Δt)` at the carrier's speed floor, `Zero` and `Positive`
/// certifying.
fn old_winding(floor: f64, (t0, t1): (f64, f64), band: Band) -> Winding {
    let headroom = geom_core::Margin::metered(TAU - (t1 - t0), geom_core::InfSpeed::new(floor));
    match geom_core::k_stats::decide("interval_span_winding", headroom, band) {
        Ok(geom_core::Sign::Positive | geom_core::Sign::Zero) => Winding::Certified,
        Ok(geom_core::Sign::Negative) => Winding::Exceeded,
        Err(_) => Winding::Escalated,
    }
}

/// Check 2's winding as certification now reaches it, and the turn its
/// certificate keeps.
fn new_winding(
    got: &Result<EdgeCurve<f64>, CertifyError>,
) -> (Winding, Option<geom_core::period::Within>) {
    match got {
        Ok(edge) => (
            Winding::Certified,
            edge.turn().copied().map(|turn| turn.turn()),
        ),
        Err(
            CertifyError::WindingExceeded
            | CertifyError::StatedInterval {
                refused: StatedSpan::Winding(WindingReading::Exceeded),
            },
        ) => (Winding::Exceeded, None),
        _ if winding_escalation(got).is_some() => (Winding::Escalated, None),
        Err(other) => panic!("the fixture refuses past the winding check: {other:?}"),
    }
}

/// **Certification agrees with the winding check it replaced** (the
/// charter's agreement test), on the cases it was written for — a full
/// turn, and a period over-wound or short by `{0.5, 0.9, 3, 20}·ε` of the
/// inf rate's and of the sup rate's length — on a unit circle and on the
/// `a/b = 16` ellipse (where no case lands on a band edge) with the defect
/// at its fast and its slow side; and a certified edge keeps the turn it
/// reached.
///
/// **The expected deltas**, each listed, all on the ellipse's fast side,
/// where the speed floor `1` reads the defect sixteen times short of its
/// length and the inf over the defect reads it at about its length:
/// - over by `0.9·ε` of the floor's length: the floor read it `Zero` and
///   certified; the inf over the defect reads its `14.4·ε` definitely past
///   the band, so the edge is refused as over a turn (the rider);
/// - `3·ε` of the floor's length and `20·ε` of the sup's, either way: the
///   floor read it in the band and escalated; the inf over the defect
///   reads it definitely past, so the edge certifies short of a turn or is
///   refused as over one;
/// - `0.5·ε` of the floor's length and `3·ε` of the sup's, either way: the
///   floor read it `Zero` and certified; the defect, `8·ε` and `3·ε` long,
///   is in the band at either bound over it, so the edge escalates.
///
/// Short by `0.9·ε` of the floor's length does not move: it certified
/// whole at the floor, and certifies short now.
///
/// The slow side and the circle do not move.
#[test]
fn certification_agrees_with_the_old_winding_check() {
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    let eps = band.zero();
    let fixture = steep(16.0);
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let certify_circle = |(t0, t1): (f64, f64)| {
        let spec = EdgeCurveSpec::arc_of_circle(circle.clone(), t0, t1).unwrap();
        let (start, end) = (spec.carrier.eval(t0), spec.carrier.eval(t1));
        EdgeCurve::certify(
            spec,
            start,
            end,
            |_| None,
            1.0,
            band,
            geom_brep::NurbsLane::certified(),
        )
    };
    let certify_ellipse = |(t0, t1): (f64, f64)| fixture.certify(t0, t1, band);
    type Door<'a> = &'a dyn Fn((f64, f64)) -> Result<EdgeCurve<f64>, CertifyError>;
    // (name, door, defect centre, speed floor, sup at the centre)
    let fixtures: [(&str, Door, f64, f64, f64); 3] = [
        ("circle", &certify_circle, 0.3, 1.0, 1.0),
        ("ellipse fast", &certify_ellipse, HALF_PI, 1.0, 16.0),
        ("ellipse slow", &certify_ellipse, 0.0, 1.0, 1.0),
    ];
    use Winding::{Certified, Escalated, Exceeded};
    // (case, the old winding, the new)
    let deltas = [
        (("ellipse fast", "inf", 0.5, true), (Certified, Escalated)),
        (("ellipse fast", "inf", 0.5, false), (Certified, Escalated)),
        (("ellipse fast", "inf", 0.9, true), (Certified, Exceeded)),
        (("ellipse fast", "inf", 3.0, true), (Escalated, Exceeded)),
        (("ellipse fast", "inf", 3.0, false), (Escalated, Certified)),
        (("ellipse fast", "sup", 3.0, true), (Certified, Escalated)),
        (("ellipse fast", "sup", 3.0, false), (Certified, Escalated)),
        (("ellipse fast", "sup", 20.0, true), (Escalated, Exceeded)),
        (("ellipse fast", "sup", 20.0, false), (Escalated, Certified)),
    ];
    let mut seen = 0;
    for (name, door, centre, floor, sup) in fixtures {
        let full = (centre, centre + TAU);
        let (got, turn) = new_winding(&door(full));
        assert_eq!(got, old_winding(floor, full, band), "{name}: a full turn");
        assert_eq!(
            turn,
            Some(geom_core::period::Within::Whole),
            "{name}: a full turn"
        );
        let quarter = (centre, centre + HALF_PI);
        assert_eq!(
            new_winding(&door(quarter)),
            (Winding::Certified, Some(geom_core::period::Within::Short)),
            "{name}: a quarter turn"
        );
        for (rate_name, rate) in [("inf", floor), ("sup", sup)] {
            for k in [0.5, 0.9, 3.0, 20.0] {
                for over in [true, false] {
                    let defect = if over {
                        k * eps / rate
                    } else {
                        -k * eps / rate
                    };
                    let t0 = centre - 0.5 * defect;
                    let span = (t0, t0 + TAU + defect);
                    let old = old_winding(floor, span, band);
                    let (new, turn) = new_winding(&door(span));
                    let case = (name, rate_name, k, over);
                    if let Some(&(_, moved)) = deltas.iter().find(|(c, _)| *c == case) {
                        seen += 1;
                        assert_eq!((old, new), moved, "{case:?}: the listed delta");
                    } else {
                        assert_eq!(new, old, "{case:?}: unlisted");
                    }
                    assert_eq!(
                        turn.is_some(),
                        new == Winding::Certified,
                        "{case:?}: a certified periodic edge keeps its turn"
                    );
                }
            }
        }
    }
    assert_eq!(seen, deltas.len(), "every listed delta is a case");
}

/// **A certified edge's turn is read off its own interval**: the stored
/// interval is `params()` and the stored extent `param_end − param_start`,
/// bit for bit, on an ellipse arc and a whole circle — and the two maps
/// that carry a certificate without a run carry them unchanged.
#[test]
fn the_stored_turn_is_the_edges_own_interval() {
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    let fixture = steep(16.0);
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let whole = EdgeCurveSpec::arc_of_circle(circle.clone(), 0.3, 0.3 + TAU).unwrap();
    let edges = [
        fixture.certify(0.2, 0.2 + HALF_PI, band).unwrap(),
        EdgeCurve::certify(
            whole,
            circle.eval(0.3),
            circle.eval(0.3 + TAU),
            |_| None,
            1.0,
            band,
            geom_brep::NurbsLane::certified(),
        )
        .unwrap(),
    ];
    for edge in edges {
        let mapped = [
            edge.clone(),
            edge.with_remapped_surfaces(Some).unwrap(),
            edge.with_chart_v_mirrored().unwrap(),
        ];
        for e in mapped {
            let (p0, p1) = e.params();
            let turn = e
                .turn()
                .copied()
                .expect("a periodic carrier keeps its turn");
            let (s0, s1) = turn.params();
            assert_eq!(
                (s0.to_bits(), s1.to_bits()),
                (p0.to_bits(), p1.to_bits()),
                "the stored interval is the edge's"
            );
            assert_eq!(
                turn.extent().get().to_bits(),
                (p1 - p0).to_bits(),
                "the stored extent is the edge's span"
            );
        }
    }
}

/// What a reader of a certified circle's span did with it.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Read {
    Proceeds,
    Refuses,
    Escalates,
}

/// **The certified readers read the turn the certificate stored, and
/// agree with the decisions they retired**: `topo`'s point split
/// (`bool_split_span_period`, not over), the sphere trim's meridian
/// (`bool_sphere_trim_meridian_span`, strictly partial) and `sweep`'s
/// seam split (the raw `(τ − span).lo() <= 0` read, strictly partial).
/// Each old read is an oracle, run on every circle edge that certifies
/// at the run's band, with the span `τ ± k·e/r` for `k ∈ {0.5, 3, 20}`
/// at both band edges `e`, a full turn and a quarter, at a radius that
/// is not one; and every edge the split's old read would refuse or
/// escalate does not certify, so no edge reaches the arms it retired.
/// The readers decide at the band their edges certify at: `topo` and
/// `sweep` certify every stored edge at `Band::linear` of the run's one
/// `Tol`, and read it under the same.
///
/// **The expected delta**, the seam split's: a span short of a turn by
/// `0.5·ε` of the radius's length is a whole turn at the band, which the
/// seam split now refuses where its raw read (any positive headroom)
/// proceeded.
#[test]
fn the_certified_readers_agree_with_the_reads_they_retired() {
    use geom_core::k_stats::decide;
    use geom_core::period::Within;
    use geom_core::{Margin, Sign};
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    let eps = band.zero();
    let radius = 2.0;
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let certify = |(t0, t1): (f64, f64)| {
        let spec = EdgeCurveSpec::arc_of_circle(circle.clone(), t0, t1).unwrap();
        let (start, end) = (spec.carrier.eval(t0), spec.carrier.eval(t1));
        EdgeCurve::certify(
            spec,
            start,
            end,
            |_| None,
            1.0,
            band,
            geom_brep::NurbsLane::certified(),
        )
    };
    let headroom = |(t0, t1): (f64, f64)| Margin::levered(TAU - (t1 - t0), radius);
    let old_split = |span| match decide("bool_split_span_period", headroom(span), band) {
        Ok(Sign::Positive | Sign::Zero) => Read::Proceeds,
        Ok(Sign::Negative) => Read::Refuses,
        Err(_) => Read::Escalates,
    };
    let old_meridian = |span| match decide("bool_sphere_trim_meridian_span", headroom(span), band) {
        Ok(Sign::Positive) => Read::Proceeds,
        Ok(Sign::Zero | Sign::Negative) => Read::Refuses,
        Err(_) => Read::Escalates,
    };
    let old_seam = |(t0, t1): (f64, f64)| {
        if TAU - (t1 - t0) <= 0.0 {
            Read::Refuses
        } else {
            Read::Proceeds
        }
    };
    let partial = |turn: Within| match turn {
        Within::Short => Read::Proceeds,
        Within::Whole => Read::Refuses,
    };
    let t0 = 0.7;
    let mut spans = vec![(f64::NAN, (t0, t0 + TAU)), (f64::NAN, (t0, t0 + HALF_PI))];
    for edge in [band.zero(), band.escalate()] {
        for k in [0.5, 3.0, 20.0] {
            for sign in [1.0, -1.0] {
                let d = sign * k * edge / radius;
                spans.push((sign * k * (edge / eps).round(), (t0, t0 + TAU + d)));
            }
        }
    }
    let mut seen = 0;
    for (k, span) in spans {
        let Ok(edge) = certify(span) else {
            assert_ne!(
                old_split(span),
                Read::Proceeds,
                "k = {k}: refused, as the split's read"
            );
            continue;
        };
        let turn = edge.turn().copied().expect("a circle's turn").turn();
        assert_eq!(old_split(span), Read::Proceeds, "k = {k}: the split");
        assert_eq!(partial(turn), old_meridian(span), "k = {k}: the meridian");
        if k == -0.5 {
            seen += 1;
            assert_eq!(
                (old_seam(span), partial(turn)),
                (Read::Proceeds, Read::Refuses),
                "k = {k}: the seam split's listed delta"
            );
        } else {
            assert_eq!(partial(turn), old_seam(span), "k = {k}: the seam split");
        }
    }
    assert_eq!(seen, 1, "the listed delta is a case");
}
