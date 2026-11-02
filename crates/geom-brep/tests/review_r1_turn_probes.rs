//! **The turn rule at certification, against the true defect** (review
//! R1 of PR 5686's rows, joined to the suite): every certified `Whole` is
//! metrically within ε of a whole turn and every `Short` gap and `Over`
//! overlap definitely past the band, on ellipses up to `a/b = 100`; a
//! whole spiric oval certifies `Whole`; and certification's circle check
//! records the stream its old spelling did.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom::{Curve3, Surface};
use geom_brep::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, StatedSpan, SurfaceKey,
    SurfacePair, WindingReading,
};
use geom_core::period::{Exact, Extent, Period, Turn, TurnRows, Within};
use geom_core::{Band, InfSpeed, Point3, Vec3};
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
        Ok(e) => match e.turn().copied().map(|t| t.turn()) {
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

/// **A whole spiric oval certifies `Whole`**, at the run's band, from two
/// starts, on ovals clear of the tangent regime `d = R − r` (the sup the
/// spiric arm reads is the period's, finite there). Nearer tangency the
/// global sup loosens and a whole oval can escalate
/// (`turn-rule-reads-short-at-the-global-speed-floor`, closed; recoverable
/// by `docs/doc-ledger/pred-leaves-the-tracker.md`).
#[test]
fn r1_spiric_whole_ovals_certify_whole() {
    let (big_r, r) = (0.09375, 0.0703125);
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    for d in [0.0, 0.0078125, 0.5 * (big_r - r)] {
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
        for t0 in [0.0, 1.3] {
            let t1 = t0 + TAU;
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(plane, torus),
                    selector: geom_brep::CurveSelector::Witness(carrier.eval(0.5 * (t0 + t1))),
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
                geom_brep::NurbsLane::certified(),
            );
            let turn = got.as_ref().ok().and_then(|e| e.turn().copied());
            assert_eq!(
                turn.map(|t| t.turn()),
                Some(Within::Whole),
                "d {d}, ({t0}, {t1}): {:?}",
                got.map(|_| "certified")
            );
        }
    }
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
                            let fwd = decide_reported(
                                "interval_span_forward",
                                Margin::metered(span, rate),
                                band,
                            );
                            let Ok(geom_core::Decided {
                                sign: Sign::Positive,
                                ..
                            }) = fwd
                            else {
                                return format!("fwd {:?}", fwd.map(|d| d.sign));
                            };
                            format!(
                                "wind {:?}",
                                decide(
                                    "interval_span_winding",
                                    Margin::metered(TAU - span, rate),
                                    band
                                )
                                .map_err(|e| format!("{:?}", e.margin))
                            )
                        })();
                        let old_rec = old_b.finish();
                        let new_b = Bracket::open();
                        let new = (|| {
                            let rate = Exact::new(r);
                            let extent = match Extent::forward(
                                "interval_span_forward",
                                t0,
                                t1,
                                InfSpeed::new(r),
                                band,
                            ) {
                                Ok(e) => e,
                                Err(e) => {
                                    return format!(
                                        "fwd {:?}",
                                        match e {
                                            geom_core::period::NotForward::Zero(_) =>
                                                Ok(Sign::Zero),
                                            geom_core::period::NotForward::Reversed(_) =>
                                                Ok(Sign::Negative),
                                            geom_core::period::NotForward::Undecided(_) => Err(()),
                                        }
                                    );
                                }
                            };
                            let got = Period::new(TAU).turn(
                                TurnRows::exact("interval_span_winding"),
                                extent,
                                rate,
                                band,
                            );
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
                        let old_n = norm(old.replace('\0', ""));
                        let old_n = if old_n.starts_with("fwd Err") {
                            "fwd Err".to_owned()
                        } else {
                            old_n
                        };
                        let new_n = if new.starts_with("fwd Err") {
                            "fwd Err".to_owned()
                        } else {
                            new
                        };
                        assert_eq!(old_n, new_n, "r {r} eps {eps} k {k} base {base}");
                        assert_eq!(
                            format!("{old_rec:?}"),
                            format!("{new_rec:?}"),
                            "r {r} eps {eps} k {k} base {base}"
                        );
                    }
                }
            }
        }
    }
    println!("circle cases {n}: identical streams");
}
