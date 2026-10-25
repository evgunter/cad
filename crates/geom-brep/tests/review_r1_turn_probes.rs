//! Review R1 probes for PR 5686 (the period-turn rule). Not a gate:
//! evidence for the review, run with `--nocapture` to read the tables.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom::{Curve3, Surface};
use geom_brep::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, StatedSpan, SurfaceKey,
    SurfacePair, WindingReading,
};
use geom_core::period::{Bounded, Exact, Extent, Period, Turn, TurnRows, Within};
use geom_core::{Band, InfSpeed, Point3, Real, SupSpeed, Vec3};
use slotmap::SlotMap;

const TAU: f64 = core::f64::consts::TAU;
const TAU_LO: f64 = 2.449_293_598_294_706_4e-16;
const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;

struct Steep {
    surfaces: SlotMap<SurfaceKey, Surface<f64>>,
    pair: SurfacePair,
    ellipse: Curve3<f64>,
    a: f64,
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
    let a = 1.0 / alpha.cos();
    Steep {
        surfaces,
        pair: SurfacePair::new(plane, cyl),
        ellipse: Curve3::Ellipse {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            semi_u: a,
            semi_v: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        a,
    }
}

impl Steep {
    fn certify(&self, t0: f64, t1: f64, band: Band) -> Result<EdgeCurve<f64>, CertifyError> {
        let e = &self.ellipse;
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: self.pair,
                witness: e.eval(0.5 * (t0 + t1)),
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
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Out {
    Short,
    Whole,
    Over,
    Escalated,
    Other,
}

fn classify(got: &Result<EdgeCurve<f64>, CertifyError>) -> Out {
    match got {
        Ok(e) => match e.certificate().turn.map(|t| t.turn()) {
            Some(Within::Short) => Out::Short,
            Some(Within::Whole) => Out::Whole,
            None => panic!("a certified ellipse with no turn"),
        },
        Err(
            CertifyError::WindingExceeded
            | CertifyError::StatedInterval {
                refused: StatedSpan::Winding(WindingReading::Exceeded),
            },
        ) => Out::Over,
        Err(
            CertifyError::StatedInterval {
                refused: StatedSpan::Winding(WindingReading::InBand(_)),
            }
            | CertifyError::Escalated {
                check: geom_brep::CertCheck::ParamWinding,
                ..
            },
        ) => Out::Escalated,
        Err(_) => Out::Other,
    }
}

/// The exact parameter defect `2π − (t1 − t0)` (double-double).
fn true_defect(t0: f64, t1: f64) -> f64 {
    let s = t1 - t0;
    // two-sum error of t1 + (−t0)
    let bp = s - t1;
    let err = (t1 - (s - bp)) + (-t0 - bp);
    (TAU - s) + TAU_LO - err
}

/// Signed true length of the defect stretch on an ellipse (a, b):
/// positive a gap, negative an overlap. Composite Simpson.
fn true_length(a: f64, b: f64, t0: f64, delta: f64) -> f64 {
    let mid = t0 - 0.5 * delta;
    let w = delta.abs();
    let n = 64;
    let h = w / f64::from(n);
    let speed = |t: f64| ((a * t.sin()).powi(2) + (b * t.cos()).powi(2)).sqrt();
    let lo = mid - 0.5 * w;
    let mut s = speed(lo) + speed(lo + w);
    for i in 1..n {
        let t = lo + h * f64::from(i);
        s += speed(t) * if i % 2 == 1 { 4.0 } else { 2.0 };
    }
    (s * h / 3.0) * delta.signum()
}

/// **Claim 3, end to end at certification**: every certified Whole is
/// metrically within ε of a whole turn, every Short gap and every Over
/// overlap is definitely past 10ε — on ellipses up to a/b = 100, defects
/// centred on the slow, fast and intermediate sides, at three bands.
/// Also counts where the OLD rule (inf reading alone) admitted a defect
/// metrically past ε.
#[test]
fn r1_certify_turn_is_sound_against_the_true_defect_length() {
    let ks = [
        0.0, 0.1, 0.5, 0.9, 0.99, 1.0, 1.01, 1.1, 2.0, 5.0, 9.0, 9.9, 10.0, 10.1, 11.0, 15.0, 20.0,
        50.0,
    ];
    let mut table = std::collections::BTreeMap::new();
    let mut worst_whole = 0.0f64;
    let mut worst_whole_true = 0.0f64;
    let mut worst_short = f64::INFINITY;
    let mut worst_over = f64::INFINITY;
    let mut old_unsound = 0;
    let mut bad = Vec::new();
    let mut cases = 0;
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for ratio in [1.5, 4.0, 16.0, 20.0, 100.0] {
            let fx = steep(ratio);
            let (a, b) = (fx.a, 1.0);
            for centre in [0.0, 0.4, HALF_PI, 2.0, 1.5 * core::f64::consts::PI + 0.1] {
                let sup_c = ((a * centre.sin()).powi(2) + (b * centre.cos()).powi(2)).sqrt();
                for rate in [b, sup_c, 0.5 * (b + sup_c)] {
                    for &k in &ks {
                        for sign in [1.0, -1.0] {
                            let defect = sign * k * eps / rate; // + over-wound
                            let t0 = centre - 0.5 * defect;
                            let t1 = t0 + TAU + defect;
                            let got = fx.certify(t0, t1, band);
                            let out = classify(&got);
                            cases += 1;
                            *table.entry((ratio as u64, out)).or_insert(0) += 1;
                            // The rule's own headroom (f64 τ); the true 2π
                            // defect is reported apart.
                            let h = TAU - (t1 - t0);
                            let len = true_length(a, b, t0, h);
                            let r = len / eps;
                            let r_true = true_length(a, b, t0, true_defect(t0, t1)) / eps;
                            if out == Out::Whole {
                                worst_whole_true = worst_whole_true.max(r_true.abs());
                            }
                            match out {
                                Out::Whole => {
                                    worst_whole = worst_whole.max(r.abs());
                                    if r.abs() > 1.0 + 1e-6 {
                                        bad.push((eps, ratio, centre, k, sign, out, r));
                                    }
                                }
                                Out::Short => {
                                    if r < 1e3 {
                                        worst_short = worst_short.min(r);
                                    }
                                    if r <= 10.0 * (1.0 - 1e-6) {
                                        bad.push((eps, ratio, centre, k, sign, out, r));
                                    }
                                }
                                Out::Over => {
                                    worst_over = worst_over.min(-r);
                                    if -r <= 10.0 * (1.0 - 1e-6) {
                                        bad.push((eps, ratio, centre, k, sign, out, r));
                                    }
                                }
                                Out::Escalated | Out::Other => {}
                            }
                            // The old rule: (τ − Δt)·b decided alone.
                            let old = geom_core::k_stats::decide(
                                "r1_old",
                                geom_core::Margin::metered(TAU - (t1 - t0), InfSpeed::new(b)),
                                band,
                            );
                            if matches!(old, Ok(geom_core::Sign::Zero)) && r.abs() > 1.0 + 1e-6 {
                                old_unsound += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("cases {cases}");
    for ((ratio, out), n) in &table {
        println!("a/b {ratio:>4} {out:?}: {n}");
    }
    println!(
        "worst |L|/eps over Whole: {worst_whole:.6}; least L/eps over Short (near band): \
         {worst_short:.6}; least overlap/eps over Over: {worst_over:.6}"
    );
    println!("worst |L|/eps over Whole against the true 2π: {worst_whole_true:.6}");
    println!("old rule admitted Zero with |L| > eps in {old_unsound} cases");
    assert!(bad.is_empty(), "unsound verdicts: {bad:#?}");
}

/// ConicArc's old expression, verbatim (generic, as it was: `Real`'s
/// `sin_cos`, `powi`, `sqrt`, not `std`'s).
fn old_conic_sup<T: Real>(a: T, b: T, t0: T, gap: T) -> T {
    let (sm, cm) = (t0 - gap * T::from_f64(0.5)).sin_cos();
    ((a * sm).powi(2) + (b * cm).powi(2)).sqrt()
        + a.abs().max(b.abs()) * gap.abs() * T::from_f64(0.5)
}

/// **Claim 2**: `turn_defect_speed_hi` is the old ConicArc sup bit for
/// bit and dominates the dense-sampled speed over the defect, on
/// ellipses stored either order and sign, circles, any `t0`, gaps from
/// 1e-15 to 3 both signs.
#[test]
fn r1_turn_defect_sup_is_the_old_bits_and_bounds_the_speed() {
    let mut rng = test_utils::fuzz::start("r1_turn_defect_sup");
    let mut worst = f64::NEG_INFINITY; // max over samples of speed − sup (should be ≤ 0)
    let mut violations = 0;
    let mut cases = 0;
    for _ in 0..20_000 {
        let big = 10f64.powf(rng.range(0.0, 3.0));
        let (mut u, mut v) = (big, 1.0);
        if rng.unit() < 0.5 {
            (u, v) = (v, u);
        }
        if rng.unit() < 0.25 {
            u = -u;
        }
        let circle = rng.unit() < 0.1;
        let carrier = if circle {
            Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: big,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }
        } else {
            Curve3::Ellipse {
                center: Point3::origin(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                semi_u: u,
                semi_v: v,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }
        };
        let (a, b) = if circle { (big, big) } else { (u, v) };
        let t0 = rng.range(-10.0, 10.0);
        let mag = 10f64.powf(rng.range(-15.0, 0.5));
        let gap = if rng.unit() < 0.5 { mag } else { -mag };
        let got = geom::carrier_speed::turn_defect_speed_hi(&carrier, t0, gap).get();
        let old = old_conic_sup(a, b, t0, gap);
        assert_eq!(got.to_bits(), old.to_bits(), "a {a} b {b} t0 {t0} gap {gap:e}");
        let mid = t0 - 0.5 * gap;
        for i in 0..=256 {
            let t = mid - 0.5 * mag + mag * f64::from(i) / 256.0;
            let s = ((a * t.sin()).powi(2) + (b * t.cos()).powi(2)).sqrt();
            worst = worst.max((s - got) / got);
            if s > got {
                violations += 1;
            }
        }
        cases += 1;
    }
    println!(
        "cases {cases}; samples over the sup: {violations}; worst relative excess {worst:e}"
    );
}

/// The spiric arm: whole ovals certify `Whole` only if the period's speed
/// sup is finite. Prints the bound and the certification outcome for a
/// whole oval, at offsets approaching the tangent regime `d = R − r`.
#[test]
fn r1_spiric_whole_ovals() {
    let (big_r, r) = (0.09375, 0.0703125);
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    for d in [
        0.0,
        0.0078125,
        0.5 * (big_r - r),
        (big_r - r) * (1.0 - 1e-3),
        (big_r - r) * (1.0 - 1e-8),
        big_r - r,
    ] {
        let (s, acc) = geom::spiric_period_rate_bounds(big_r, r, d);
        let mut surfaces: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
        let torus = surfaces.insert(Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_y(),
            major_radius: big_r,
            minor_radius: r,
            u_ref: Vec3::unit_x(),
        });
        let plane = surfaces.insert(Surface::Plane {
            origin: Point3::new(d, 0.0, 0.0),
            normal: Vec3::unit_x(),
            u_ref: Vec3::unit_y().cross(Vec3::unit_x()),
        });
        let carrier = Curve3::Spiric {
            center: Point3::origin(),
            axis: Vec3::unit_y(),
            u_ref: Vec3::unit_x(),
            major_radius: big_r,
            minor_radius: r,
            offset: d,
        };
        for (t0, t1) in [(0.0, TAU), (1.3, 1.3 + TAU), (0.0, TAU + 1e-12)] {
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(plane, torus),
                    witness: carrier.eval(0.5 * (t0 + t1)),
                },
                carrier: carrier.clone(),
                param_start: t0,
                param_end: t1,
            };
            let got = EdgeCurve::certify(
                spec,
                carrier.eval(t0),
                carrier.eval(t1),
                |k| surfaces.get(k).cloned(),
                1.0,
                band,
            );
            let out = match &got {
                Ok(e) => format!("certified {:?}", e.certificate().turn.map(|t| t.turn())),
                Err(e) => format!("{e:?}"),
            };
            println!(
                "d {d:.10} sup {s:e} accel {acc:e} span ({t0}, {t1}) h {:e}: {out}",
                TAU - (t1 - t0)
            );
        }
    }
}

/// **Claim 6**: `Extent::width` and `Add` let a reversed / negative
/// extent through, and `width(0, x)` is a bare constructor.
#[test]
fn r1_extent_width_trusts_its_caller() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let one = Period::new(TAU);
    // A reversed span through `width` reads as a short arc.
    let reversed = Extent::width(1.0, 0.0);
    println!("width(1, 0) = {}", reversed.get());
    let got = one.turn(TurnRows::exact("r1"), reversed, Exact(1.0), band);
    println!("reversed width reads {got:?}");
    assert!(matches!(got, Ok(Turn::Short(_))));
    // Any value through width(0, x).
    let bare = Extent::width(0.0, -5.0);
    assert_eq!(bare.get(), -5.0);
    // A bounded rate with sup below inf is accepted.
    let odd = Bounded {
        inf: InfSpeed::new(2.0),
        sup: SupSpeed::new(1.0),
    };
    let got = one.turn(
        TurnRows::bounded("r1", "r1s"),
        Extent::width(0.0, TAU - 3e-9),
        odd,
        band,
    );
    println!("sup < inf: {got:?}");
    let _ = f64::tau();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Span {
    Wound,
    EscMargin,
    EscStraddle,
    Arc,
}

/// ConicArc::of's old span rule (pre-PR), verbatim in shape, generic.
fn old_conic(a: f64, b: f64, ellipse: bool, (t0, t1): (f64, f64), band: Band) -> Span {
    use geom_core::k_stats::decide_reported;
    use geom_core::{Decided, Margin, Sign};
    let lever = a.min(b);
    let gap = f64::tau() - (t1 - t0);
    let low = decide_reported("r1_old_span", Margin::levered(gap, lever), band);
    if let Ok(Decided {
        sign: Sign::Negative,
        ..
    }) = low
    {
        return Span::Wound;
    }
    let speed = old_conic_sup(a, b, t0, gap);
    if ellipse
        && let Ok(Decided {
            sign: Sign::Negative,
            ..
        }) = decide_reported("r1_old_span", Margin::levered(gap, speed), band)
    {
        return match low {
            Err(_) => Span::EscMargin,
            Ok(_) => Span::EscStraddle,
        };
    }
    Span::Arc
}

/// ConicArc::of's new mapping over the shipped rule.
fn new_conic(carrier: &Curve3<f64>, a: f64, b: f64, ellipse: bool, (t0, t1): (f64, f64), band: Band) -> Option<Span> {
    use geom_core::period::{TurnRow, TurnUndecided};
    let extent = Extent::forward("r1_fwd", t0, t1, InfSpeed::new(1.0), band).ok()?;
    let lever = a.min(b);
    let one = Period::new(f64::tau());
    let turn = if ellipse {
        let gap = f64::tau() - extent.get();
        let sup = geom::carrier_speed::turn_defect_speed_hi(carrier, t0, gap);
        one.turn(
            TurnRows::bounded("r1_new_span", "r1_new_straddle"),
            extent,
            Bounded { inf: InfSpeed::new(lever), sup },
            band,
        )
    } else {
        one.turn(TurnRows::exact("r1_new_span"), extent, Exact(lever), band)
    };
    Some(match turn {
        Ok(Turn::Over(_)) => Span::Wound,
        Ok(_) => Span::Arc,
        Err(TurnUndecided { on, over_at_sup: true, .. }) => match on {
            TurnRow::Turn => Span::EscMargin,
            TurnRow::Straddle => Span::EscStraddle,
        },
        Err(_) => Span::Arc,
    })
}

/// **Claim 4**: over a dense band-edge sweep, ConicArc's new mapping over
/// the rule lands every forward span in the class (wound / escalated on a
/// margin / escalated on a straddle / arc) the old rule did.
#[test]
fn r1_conic_arc_classes_do_not_move() {
    let mut moved = Vec::new();
    let mut n = 0;
    let mut hist = std::collections::BTreeMap::new();
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for (a, b, ellipse) in [
            (1.0, 1.0, false),
            (0.01, 0.01, false),
            (250.0, 250.0, false),
            (1.5, 1.0, true),
            (16.0, 1.0, true),
            (20.0, 1.0, true),
            (1.0, 20.0, true),
            (100.0, 1.0, true),
            (1000.0, 1.0, true),
        ] {
            let carrier = if ellipse {
                Curve3::Ellipse {
                    center: Point3::origin(),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    semi_u: a,
                    semi_v: b,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                }
            } else {
                Curve3::Circle {
                    center: Point3::origin(),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    radius: a,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                }
            };
            for centre in [0.0, 0.3, HALF_PI, 2.5, -1.2] {
                let sc = ((a * f64::sin(centre)).powi(2) + (b * f64::cos(centre)).powi(2)).sqrt();
                for rate in [a.min(b), sc, a.max(b)] {
                    for i in 0..=240 {
                        let k = f64::from(i) * 0.1; // 0 .. 24 in steps of 0.1
                        for k in [k, k + 0.0137] {
                            for sign in [1.0, -1.0] {
                                let defect = sign * k * eps / rate;
                                let t0 = centre - 0.5 * defect;
                                let span = (t0, t0 + TAU + defect);
                                let old = old_conic(a, b, ellipse, span, band);
                                let Some(new) = new_conic(&carrier, a, b, ellipse, span, band) else {
                                    continue;
                                };
                                n += 1;
                                *hist.entry((ellipse, old)).or_insert(0) += 1;
                                if old != new {
                                    moved.push((eps, a, b, centre, k, sign, old, new));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("cases {n}; classes {hist:?}; moved {}", moved.len());
    assert!(moved.is_empty(), "{:#?}", &moved[..moved.len().min(20)]);
}

/// **Claim 1**: certification's circle check 2 — the old spelling and
/// the new — record the same K stream (names, signs, escalation margins)
/// and decide the same, over a band-edge sweep at three bands.
#[test]
fn r1_circle_check_two_records_the_same_stream() {
    use geom_core::k_stats::{Bracket, decide, decide_reported};
    use geom_core::{Margin, Sign};
    let mut n = 0;
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for r in [1e-3, 0.01, 1.0, 7.5, 250.0] {
            for i in -260..=260 {
                for frac in [0.0, 0.37] {
                    let k = (f64::from(i) + frac) * 0.1;
                    for base in [TAU, 1.0, 1e-9 / r] {
                        let (t0, t1) = (0.3, 0.3 + base + k * eps / r);
                        let old_b = Bracket::open();
                        let old = (|| {
                            let span = t1 - t0;
                            let rate = InfSpeed::new(r);
                            let fwd = decide_reported("interval_span_forward", Margin::metered(span, rate), band);
                            let Ok(geom_core::Decided { sign: Sign::Positive, .. }) = fwd else {
                                return format!("fwd {:?}", fwd.map(|d| d.sign));
                            };
                            format!(
                                "wind {:?}",
                                decide("interval_span_winding", Margin::metered(TAU - span, rate), band)
                                    .map_err(|e| format!("{:?}", e.margin))
                            )
                        })();
                        let old_rec = old_b.finish();
                        let new_b = Bracket::open();
                        let new = (|| {
                            let rate = Exact(r);
                            let extent = match Extent::forward("interval_span_forward", t0, t1, InfSpeed::new(r), band) {
                                Ok(e) => e,
                                Err(e) => {
                                    return format!("fwd {:?}", match e {
                                        geom_core::period::NotForward::Zero(_) => Ok(Sign::Zero),
                                        geom_core::period::NotForward::Reversed(_) => Ok(Sign::Negative),
                                        geom_core::period::NotForward::Undecided(_) => Err(()),
                                    }.map_err(|()| ()).map(|s| s));
                                }
                            };
                            let got = Period::new(TAU).turn(TurnRows::exact("interval_span_winding"), extent, rate, band);
                            format!(
                                "wind {:?}",
                                got.map(|t| match t {
                                    Turn::Short(_) => Sign::Positive,
                                    Turn::Whole(_) => Sign::Zero,
                                    Turn::Over(_) => Sign::Negative,
                                })
                                .map_err(|e| format!("{:?}", e.diag.margin))
                            )
                        })();
                        let new_rec = new_b.finish();
                        n += 1;
                        let norm = |s: String| s.replace("Err(())", "Err");
                        let old_n = norm(old.replace(|c: char| c == '\0', ""));
                        let old_n = if old_n.starts_with("fwd Err") { "fwd Err".to_owned() } else { old_n };
                        let new_n = if new.starts_with("fwd Err") { "fwd Err".to_owned() } else { new };
                        assert_eq!(old_n, new_n, "r {r} eps {eps} k {k} base {base}");
                        assert_eq!(format!("{old_rec:?}"), format!("{new_rec:?}"), "r {r} eps {eps} k {k} base {base}");
                    }
                }
            }
        }
    }
    println!("circle cases {n}: identical streams");
}
