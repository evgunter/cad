//! **R2's end-to-end probes of M10-9** — the registered-identity door,
//! re-derived on a document the unit never built, and attacks on the
//! negative result the unit reports.
//!
//! - `r2_the_door_moves_nothing_but_the_receipt_on_every_m10_fixture`
//!   (ASSERTS): the drive's serialization, door on and off, on every
//!   M10 fixture this test crate can build — claim 1 / claim 5 at
//!   document scale, at whichever ε this process was built with.
//! - `r2_circle_at_is_bit_identical_to_the_old_arm` (ASSERTS): D13's
//!   delegation against a transcription of the merge base's arm, at
//!   `f64` and at `Interval`.
//! - `r2_link_end_to_end_with_and_without_the_door` (ASSERTS + prints):
//!   R2's own arc-bearing document — a SLOTTED LINK (an obround outline
//!   whose two ends are SEMICIRCLES (tangent arcs, bulge 1, `param_end = π`)
//!   exactly, with a bore at each end) — driven at its real study door
//!   on and off; its ceiling bisected; the first refusal beyond it.
//! - `r2_evidence_the_refusal_tail` (evidence): the FULL refusal tail
//!   behind each document's ceiling, read off every decision's certified
//!   enclosure at two scales below the ceiling (the branch's probe
//!   instrument in `geom_core::sym::report`) and extrapolated linearly
//!   to the scale at which each predicate would refuse. This is what
//!   the unit's "first refusal beyond it" does not show: whether a long
//!   tail of identity residuals stands behind the first one, or a real
//!   margin.
//!
//! NOT proposed for merge: the branch carries a probe instrument
//! (`DecisionShape::enclosure`, `Decide::enclosure_probe`).

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(dead_code)]

use std::collections::BTreeMap;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use editor_core::{
    Datum, Dimension, Distribution, DocEdit, DocParam, EntityKind, Expr, GeomPred, LoopProgram,
    MeasureExpr, MeasurePrimitive, NamePat, Node, ParamName, ProfileDoc, ProfileProgram,
    ProgramStep, ProgramTarget, RecipeNodeId, Selector, SitedRef, SurfaceKindSet, UnitSym,
    select_where,
};
use geom_core::sym::report::ShapeOutcome;
use geom_core::{SymRules, Tol};

use crate::fixture::Recorder;
use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{ceiling, certifies_whole, dials};

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar")
}

fn plen(n: &str) -> Expr {
    Expr::param(ParamName::new(n), Dimension::Length)
}

/// M10-9's tier with the door shut — M10-8's. The rows here are
/// M10-9's claims, so they are pinned under M10-9's tier
/// (`SymRules::without_the_algebra`, which is that tier bit for bit);
/// with the form-level algebra on the door DOES move verdicts, because
/// rule D makes the scaffold residual's trig meet and the rim identity
/// the door states is what closes it at every sample
/// (`m10_10_pins_interval`).
fn shut() -> SymRules {
    SymRules {
        registered: false,
        ..opened()
    }
}

/// M10-9's tier exactly.
fn opened() -> SymRules {
    SymRules::without_the_algebra()
}

// ------------------------------------------------- R2's own document

/// Half the distance between the two bore centres, in metres.
const HALF_L: f64 = 3.0e-3;
/// The nominal half-width of the link — the SEMICIRCLE radius.
const HALF_W: f64 = 1.5e-3;
/// The nominal bore radius.
const BORE: f64 = 0.6e-3;

/// **A slotted link**: an obround outline — two straight flanks joined
/// by two SEMICIRCULAR ends authored as tangent arcs (`TangentArcTo`,
/// derived bulge 1 = `tan(π/4)`, so each carrier's `param_end = π`) — with
/// a bore at each end. The half-width (the semicircle radius) is
/// uniform, the bore radius normal. The study measures the wall from
/// one bore to one end arc and asserts a floor.
///
/// `scale` multiplies every tolerance; `1.0` is the study a user would
/// ask for (±20 µm on the half-width, σ = 10 µm on the bores).
pub(crate) fn link(scale: f64, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    let declare = |r: &mut Recorder, n: &str, value: f64, distribution: Distribution| {
        r.push(DocEdit::SetDocParam {
            name: ParamName::new(n),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(distribution),
            },
        });
    };
    declare(
        &mut r,
        "half_w",
        HALF_W,
        Distribution::Uniform {
            lo: -2.0e-5 * scale,
            hi: 2.0e-5 * scale,
        },
    );
    declare(
        &mut r,
        "bore_r",
        BORE,
        Distribution::Normal {
            sigma: 1.0e-5 * scale,
        },
    );

    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));

    let neg_w = Expr::neg(plen("half_w"));
    // The chain's own idiom for a tangent arc between two tangent legs:
    // `.tangent()` before the arc and before the leg out of it, the
    // leg out authored as a LENGTH since it rides the inherited
    // direction (the corpus bracket's form, `m4_pr6_golden`).
    let outline = LoopProgram::Chain(vec![
        ProgramStep::At([len(-HALF_L), neg_w.clone()]),
        ProgramStep::LineTo(ProgramTarget::Point([len(HALF_L), neg_w])),
        ProgramStep::Tangent,
        ProgramStep::TangentArcTo(ProgramTarget::Point([len(HALF_L), plen("half_w")])),
        ProgramStep::Tangent,
        ProgramStep::Line(len(2.0 * HALF_L)),
        ProgramStep::Tangent,
        ProgramStep::TangentArcTo(ProgramTarget::StartArriving),
    ]);
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![outline],
    }));
    let thickness = len(1.0e-3);
    let body = r.insert(Node::Extrude {
        profile,
        distance: thickness.clone(),
    });
    let bore_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Circle {
            centre: [len(HALF_L), len(0.0)],
            radius: plen("bore_r"),
        }],
    }));
    let bore = r.insert(Node::Extrude {
        profile: bore_profile,
        distance: thickness,
    });

    let refs = {
        let ev: editor_core::Evaluation<f64> = editor_core::evaluate(
            &r.doc,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            tol,
        );
        let env = r.doc.param_env::<f64>();
        let wall = |node: RecipeNodeId, which: usize| {
            let mut faces = select_where(
                &ev,
                node,
                &Selector::of(NamePat::of_kind(EntityKind::Face)),
                &[GeomPred::SurfaceKind(SurfaceKindSet::just(
                    geom_brep::SurfaceKind::Cylinder,
                ))],
                &env,
                tol,
            )
            .expect("the surface-kind atom is exact");
            faces.sort();
            assert!(faces.len() > which, "a link has two semicircular walls");
            SitedRef::new(node, faces.remove(which))
        };
        vec![wall(body, 0), wall(bore, 0)]
    };
    let wall = MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 });
    let measure = r.insert(Node::measure(wall, refs).expect("both indices in range"));
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(0.5e-3),
        dir: editor_core::AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

// ------------------------------------------------- the asserting rows

/// Every M10 fixture this crate can build, as (name, document).
fn fixtures(tol: Tol) -> Vec<(&'static str, ProfileDoc)> {
    let eps = tol.eps();
    vec![
        ("m10_3 slab", crate::m10_3_driver_interval::slab(1.0, 0.25)),
        (
            "m10_3 sliver_axis",
            crate::m10_3_driver_interval::sliver_axis(),
        ),
        (
            "plate @ 1e2·eps",
            crate::m10_7_plate::plate(5.0e-5 * 1.0e2 * eps, 1.0e-5 * 1.0e2 * eps, tol).0,
        ),
        (
            "plate @ real study",
            crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0,
        ),
        (
            "r1 bracket @ 0.5 mm",
            crate::m10_7_r1_probes_interval::bracket_pub(0.5e-3, false, tol).0,
        ),
        (
            "r2 bracket @ 1e2·eps",
            crate::m10_7_r2_probes_interval::bracket(1.0e2 * eps, tol).0,
        ),
        (
            "r2 bracket @ real study",
            crate::m10_7_r2_probes_interval::bracket(1.0, tol).0,
        ),
        (
            "r1 annulus @ 1e2·eps",
            crate::m10_8_r1_probes_interval::annulus(1.0e2 * eps, tol).0,
        ),
        (
            "r2 pad @ 5e2·eps",
            crate::m10_8_r2_probes_interval::pad(5.0e2 * eps, tol).0,
        ),
        ("r2 link @ 1e2·eps", link(1.0e2 * eps, tol).0),
        ("r2 link @ real study", link(1.0, tol).0),
    ]
}

/// **Claim 1 / claim 5 at document scale, on every fixture**: a drive
/// with the door on and the same drive with it off serialize the SAME
/// leaves, verdict-vector keys, witness vector, masses and refusals —
/// only the receipt's `decisions` line may differ, and it may differ
/// only by a `registered=` field.
#[test]
fn r2_the_door_moves_nothing_but_the_receipt_on_every_m10_fixture() {
    let tol = Tol::witness();
    for (name, doc) in fixtures(tol) {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let run = |rules: SymRules| {
            drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: 4,
                    symbolic: dials(rules),
                    ..DriveConfig::default()
                },
                tol,
            )
            .map(|v| v.serialize())
            .map_err(|e| format!("{e:?}"))
        };
        let on = run(opened());
        let off = run(shut());
        let split = |s: &Result<String, String>| -> (Vec<String>, Option<String>) {
            match s {
                Ok(text) => (
                    text.lines()
                        .filter(|l| !l.starts_with("decisions "))
                        .map(str::to_owned)
                        .collect(),
                    text.lines()
                        .find(|l| l.starts_with("decisions "))
                        .map(str::to_owned),
                ),
                Err(e) => (vec![format!("ERR {e}")], None),
            }
        };
        let (on_body, on_dec) = split(&on);
        let (off_body, off_dec) = split(&off);
        println!(
            "   {name:<26} on: {:?}\n   {:<26} off: {:?}",
            on_dec.as_deref().unwrap_or("(no tier line)"),
            "",
            off_dec.as_deref().unwrap_or("(no tier line)")
        );
        println!("      {}", on_body.first().cloned().unwrap_or_default());
        assert_eq!(
            on_body, off_body,
            "{name}: every line but the receipt is byte-identical door on and off"
        );
        // The receipt may differ ONLY by `registered=`, and numeric
        // must shrink by exactly that count.
        let fields = |l: &Option<String>| -> BTreeMap<String, u64> {
            l.as_deref()
                .map(|l| {
                    l.split(' ')
                        .filter_map(|w| {
                            let (k, v) = w.split_once('=')?;
                            Some((k.to_owned(), v.parse::<u64>().ok()?))
                        })
                        .collect()
                })
                .unwrap_or_default()
        };
        let (fon, foff) = (fields(&on_dec), fields(&off_dec));
        let g = |m: &BTreeMap<String, u64>, k: &str| m.get(k).copied().unwrap_or(0);
        assert_eq!(
            g(&foff, "registered"),
            0,
            "{name}: the door-off receipt registers nothing"
        );
        for k in ["symbolic_zero", "sign_gated"] {
            assert_eq!(
                g(&fon, k),
                g(&foff, k),
                "{name}: `{k}` is the door-off count"
            );
        }
        // `frozen` is a plain-walk count and the two replays decide the
        // same population only when every leaf certifies (below).
        if g(&fon, "frozen") != g(&foff, "frozen") {
            println!(
                "      NOTE {name}: frozen moved with the door: on {} vs off {}",
                g(&fon, "frozen"),
                g(&foff, "frozen")
            );
        }
        // On a drive whose leaves all certify, the two replays decide
        // the same population and `numeric` shrinks by exactly the
        // registered count. On a REFUSING drive the door-on replay
        // gets past the discharged identity and decides more before it
        // refuses, so only the inequalities hold (the unit's own pin
        // says the same).
        let refused_leaves = on_body
            .first()
            .and_then(|l| l.split(' ').find_map(|w| w.strip_prefix("refused=")))
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);
        if refused_leaves == 0 {
            assert_eq!(
                g(&fon, "numeric") + g(&fon, "registered"),
                g(&foff, "numeric"),
                "{name}: the door moves decisions out of `numeric` and out of nothing else"
            );
        } else {
            assert!(
                g(&fon, "numeric") <= g(&foff, "numeric")
                    && g(&fon, "numeric") + g(&fon, "registered") >= g(&foff, "numeric"),
                "{name}: refusing drive — {fon:?} vs {foff:?}"
            );
        }
    }
}

/// **D13, checked**: `Curve3::circle_at` computes what the merge base's
/// `eval` arm computed, bit for bit, at `f64` and at `Interval`. The old
/// arm (`crates/geom/src/curves.rs` at d1149b07d) was
/// `let radial = azimuth::frame(*axis, *u_ref, t).radial.0; *center +
/// radial * *radius`, with `frame` = `(s, c) = t.sin_cos(); v_ref =
/// axis × u_ref; radial = u_ref·c + v_ref·s` — transcribed here.
#[test]
fn r2_circle_at_is_bit_identical_to_the_old_arm() {
    use geom::Curve3;
    use geom_core::{Bounds, Interval, Point3, Real, Vec3};

    fn old_arm<T: Real>(
        center: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        u_ref: Vec3<T>,
        t: T,
    ) -> Point3<T> {
        let (s, c) = t.sin_cos();
        let v_ref = axis.cross(u_ref);
        let radial = u_ref * c + v_ref * s;
        center + radial * radius
    }
    let axis = Vec3::new(0.3_f64, -0.4, 0.866).normalize();
    let u0 = Vec3::new(0.8_f64, 0.6, 0.0);
    let u_ref = (u0 - axis * u0.dot(axis)).normalize();
    let center = Point3::new(1.0e-3_f64, -2.0e-3, 0.5e-3);
    for t in [
        0.0_f64,
        0.7,
        1.9,
        core::f64::consts::PI,
        4.0 * (2.5_f64).atan(),
    ] {
        let a = Curve3::circle_at(center, axis, 0.8e-3, u_ref, t);
        let b = old_arm(center, axis, 0.8e-3, u_ref, t);
        assert_eq!(
            [a.x.to_bits(), a.y.to_bits(), a.z.to_bits()],
            [b.x.to_bits(), b.y.to_bits(), b.z.to_bits()],
            "f64 at t = {t}"
        );
        // And through the enum, which is what the certifier calls.
        let c = Curve3::Circle {
            center,
            axis,
            radius: 0.8e-3,
            u_ref,
        }
        .eval(t);
        assert_eq!(
            [c.x.to_bits(), c.y.to_bits(), c.z.to_bits()],
            [b.x.to_bits(), b.y.to_bits(), b.z.to_bits()]
        );
    }
    let iv = |x: f64| Interval::from_bounds(x - 1e-7, x + 1e-7);
    let axis = Vec3::new(iv(0.3), iv(-0.4), iv(0.866)).normalize();
    let u_ref = Vec3::new(iv(0.8), iv(0.6), iv(0.0)).normalize();
    let center = Point3::new(iv(1.0e-3), iv(-2.0e-3), iv(0.5e-3));
    let bits = |v: Interval| (v.lo().to_bits(), v.hi().to_bits());
    for t in [0.0_f64, 0.7, 1.9, 4.0 * (2.5_f64).atan()] {
        let t = iv(t);
        let a = Curve3::circle_at(center, axis, iv(0.8e-3), u_ref, t);
        let b = old_arm(center, axis, iv(0.8e-3), u_ref, t);
        assert_eq!(
            [bits(a.x), bits(a.y), bits(a.z)],
            [bits(b.x), bits(b.y), bits(b.z)],
            "Interval at t = {t:?}"
        );
    }
}

/// **R2's link, end to end**: what a real study gets today. The link's
/// real study is driven door on and off (the receipts printed and
/// compared), then its whole-certifying ceiling is bisected under both
/// rule sets and the first refusal beyond it named with its enclosure.
///
/// Asserts: the door registers on the link (`registered > 0` on a
/// certifying box); the two drives agree outside the receipt line; the
/// ceiling bracket is the same under both (each end asserted at 0.5×
/// and 2× of the door-on bracket's lower end).
#[test]
fn r2_link_end_to_end_with_and_without_the_door() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let at = |s: f64| link(s, tol).0;

    // The real study.
    let doc = at(1.0);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    for (label, rules) in [("door ON ", SymRules::shipped()), ("door OFF", shut())] {
        let v = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 8,
                symbolic: dials(rules),
                ..DriveConfig::default()
            },
            tol,
        )
        .expect("the link builds");
        println!("== link, real study, {label}:\n{}", v.render(&analyzed));
    }

    // The ceiling, both ways.
    let mut lows = Vec::new();
    for (label, rules) in [
        ("door OFF (M10-8)", shut()),
        ("door ON  (shipped)", SymRules::shipped()),
    ] {
        let (lo, hi, per) = ceiling(&at, rules, tol, 1.0e-1 * eps, 1.0e6 * eps, 12);
        println!(
            "   link {label}: certifies x{lo:e}, refuses x{hi:e} ({per:.2}s/probe) [= {:.3e}·eps .. {:.3e}·eps]",
            lo / eps,
            hi / eps
        );
        assert!(lo.is_finite() && lo > 0.0, "the link certifies somewhere");
        let beyond = at(lo * 2.0);
        let analyzed = analyzed_box(&beyond, &AnalysisPolicy::default());
        let (shapes, refusal, counts) = replay(&beyond, &ParamBox::of(&analyzed), rules, tol);
        println!("      beyond it: {refusal:?}\n      {counts:?}");
        for s in shapes.iter().filter(|s| {
            matches!(
                s.outcome,
                ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
            )
        }) {
            println!(
                "      [{:?}] {} enclosure {:?}",
                s.outcome, s.predicate, s.enclosure
            );
        }
        lows.push(lo);
    }
    let [off_lo, on_lo]: [f64; 2] = lows.try_into().unwrap();
    // Both ends, both ways.
    for (label, rules) in [("OFF", shut()), ("ON", SymRules::shipped())] {
        assert!(
            certifies_whole(&at(0.5 * on_lo), rules, tol),
            "{label}: 0.5x certifies"
        );
        assert!(
            !certifies_whole(&at(2.0 * on_lo), rules, tol),
            "{label}: 2x refuses"
        );
    }
    println!(
        "   link ceiling: door off {:.4e}·eps, door on {:.4e}·eps (ratio {:.6})",
        off_lo / eps,
        on_lo / eps,
        on_lo / off_lo
    );
    // The door registers on the link.
    let certifying = at(0.5 * on_lo);
    let analyzed = analyzed_box(&certifying, &AnalysisPolicy::default());
    let (_, refusal, counts) = replay(
        &certifying,
        &ParamBox::of(&analyzed),
        SymRules::shipped(),
        tol,
    );
    assert!(refusal.is_none(), "{refusal:?}");
    assert!(
        counts.registered > 0,
        "the door registers on the link: {counts:?}"
    );
    println!("   link at 0.5x its ceiling, door on: {counts:?}");
}

// ------------------------------------------------- the evidence rows

/// One predicate's enclosure envelope in a replay: the lowest `lo` and
/// the highest `hi` over its numeric decisions, with the outcome of
/// the decision that set each, and the counts.
#[derive(Clone, Copy, Debug)]
struct Envelope {
    lo: f64,
    hi: f64,
    outcome: ShapeOutcome,
    n: usize,
    numeric: usize,
}

/// The per-predicate envelopes of one replay (symbolic decisions carry
/// no enclosure and are only counted).
fn envelopes(shapes: &[geom_core::sym::report::DecisionShape]) -> BTreeMap<&'static str, Envelope> {
    let mut out: BTreeMap<&'static str, Envelope> = BTreeMap::new();
    for s in shapes {
        let e = out.entry(s.predicate).or_insert(Envelope {
            lo: f64::INFINITY,
            hi: f64::NEG_INFINITY,
            outcome: s.outcome,
            n: 0,
            numeric: 0,
        });
        e.n += 1;
        if matches!(
            s.outcome,
            ShapeOutcome::Theorem | ShapeOutcome::SignGated | ShapeOutcome::Registered
        ) {
            continue;
        }
        e.numeric += 1;
        let Some((lo, hi)) = s.enclosure else {
            continue;
        };
        if lo < e.lo {
            e.lo = lo;
        }
        if hi > e.hi {
            e.hi = hi;
            e.outcome = s.outcome;
        }
    }
    out.retain(|_, e| e.numeric > 0 && e.lo.is_finite());
    out
}

/// **The two-point affine model**: with the envelope measured at two
/// scales `s1 < s2`, `lo(s)` and `hi(s)` are taken affine in `s` and
/// the smallest `s > s2` at which the band classification changes is
/// answered — as a multiple of the CEILING scale. A residual decided
/// `Zero` refuses when `hi(s) > zero` or `lo(s) < −zero`; a definite
/// positive one when `lo(s) < escalate`; a definite negative when
/// `hi(s) > −escalate`. `+inf` when the envelope does not grow.
fn refusal_at(
    (s1, e1): (f64, Envelope),
    (s2, e2): (f64, Envelope),
    zero: f64,
    escalate: f64,
    ceiling: f64,
) -> f64 {
    let slope = |a: f64, b: f64| (b - a) / (s2 - s1);
    let (dlo, dhi) = (slope(e1.lo, e2.lo), slope(e1.hi, e2.hi));
    let hit = |value_at_s2: f64, rate: f64, target: f64| -> f64 {
        // value(s) = value_at_s2 + rate·(s − s2) reaches `target`.
        if rate == 0.0 {
            return f64::INFINITY;
        }
        let s = s2 + (target - value_at_s2) / rate;
        if s > s2 { s } else { f64::INFINITY }
    };
    let s = match e2.outcome {
        ShapeOutcome::NumericZero => {
            let up = if dhi > 0.0 {
                hit(e2.hi, dhi, zero)
            } else {
                f64::INFINITY
            };
            let dn = if dlo < 0.0 {
                hit(e2.lo, dlo, -zero)
            } else {
                f64::INFINITY
            };
            up.min(dn)
        }
        ShapeOutcome::Definite(geom_core::Sign::Positive) => {
            if dlo < 0.0 {
                hit(e2.lo, dlo, escalate)
            } else {
                f64::INFINITY
            }
        }
        ShapeOutcome::Definite(geom_core::Sign::Negative) => {
            if dhi > 0.0 {
                hit(e2.hi, dhi, -escalate)
            } else {
                f64::INFINITY
            }
        }
        ShapeOutcome::Indeterminate | ShapeOutcome::Invalid => s2,
        _ => f64::INFINITY,
    };
    s / ceiling
}

/// A named document with its measured ceiling, as a function of the
/// SCALE of its real study.
type TailDocument = (&'static str, f64, Box<dyn Fn(f64) -> ProfileDoc>);

/// The four documents plus the link, each with its measured ceiling.
fn tail_documents(tol: Tol) -> Vec<TailDocument> {
    let eps = tol.eps();
    vec![
        (
            "two_hole_plate",
            7.787e2 * eps,
            Box::new(move |s| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0),
        ),
        (
            "r2_filleted_bracket",
            3.865e2 * eps,
            Box::new(move |s| crate::m10_7_r2_probes_interval::bracket(s, tol).0),
        ),
        (
            "r1_annulus",
            7.787e2 * eps,
            Box::new(move |s| crate::m10_8_r1_probes_interval::annulus(s, tol).0),
        ),
        (
            "r2_rounded_pad",
            2.083e3 * eps,
            Box::new(move |s| crate::m10_8_r2_probes_interval::pad(s, tol).0),
        ),
        ("r2_link", 1.0e3 * eps, Box::new(move |s| link(s, tol).0)),
    ]
}

/// **The refusal tail behind each ceiling.** For each document, door
/// OFF and ON, the envelope of every predicate at 0.25× and 0.5× of
/// the measured ceiling (both certify whole), fitted affinely, and the
/// multiple of the ceiling at which each predicate would refuse —
/// sorted, so the reader sees what stands behind the first refusal:
/// identity residuals close behind it, or a real margin far away.
///
/// Set `CAD_R2_TAIL_DOCS` to a comma-separated subset of the names to
/// run fewer (the bracket and the pad cost minutes each under the
/// shape report).
#[test]
#[ignore = "evidence-only: prints the extrapolated refusal tail behind each ceiling"]
fn r2_evidence_the_refusal_tail() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let band = geom_core::Band::linear(tol).unwrap();
    let (zero, escalate) = (band.zero(), band.escalate());
    let only = std::env::var("CAD_R2_TAIL_DOCS").ok();
    for (name, ceil, at) in &tail_documents(tol) {
        if only
            .as_deref()
            .is_some_and(|l| !l.split(',').any(|n| n.trim() == *name))
        {
            continue;
        }
        for (label, rules) in [("door OFF", shut()), ("door ON ", SymRules::shipped())] {
            let mut tables = Vec::new();
            for frac in [0.25_f64, 0.5] {
                let doc = at(frac * ceil);
                let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
                let (shapes, refusal, counts) = replay(&doc, &ParamBox::of(&analyzed), rules, tol);
                println!(
                    "== {name} {label} at {frac}x ceiling ({:.3e}·eps): refusal {refusal:?}; {counts:?}",
                    frac * ceil / eps
                );
                tables.push((frac * ceil, envelopes(&shapes)));
            }
            let (s1, q) = &tables[0];
            let (s2, h) = &tables[1];
            let mut rows: Vec<(&&str, f64, &Envelope)> = h
                .iter()
                .map(|(p, e)| {
                    let at = q.get(p).map_or(f64::NAN, |e1| {
                        refusal_at((*s1, *e1), (*s2, *e), zero, escalate, *ceil)
                    });
                    (p, at, e)
                })
                .collect();
            rows.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(core::cmp::Ordering::Equal));
            println!(
                "   {:<34} {:>12} {:>26} {:>9} {:<10}",
                "predicate", "refuses at", "envelope @0.5x", "num/all", "outcome"
            );
            for (pred, at, e) in rows.iter().take(16) {
                println!(
                    "   {:<34} {:>10.3}x [{:>10.3e},{:>10.3e}] {:>4}/{:<4} {:?}",
                    pred, at, e.lo, e.hi, e.numeric, e.n, e.outcome
                );
            }
        }
    }
}

/// A document's ceiling with named predicates' indeterminate answers
/// PASSED, stage by stage — the branch's funnel dial
/// (`CAD_R2_PASS_INDETERMINATE`), so a ceiling can be measured AS IF
/// those identities were discharged: what bounds the document next, at
/// what multiple, and whether a real margin is ever reached.
fn staged_ceilings(name: &str, at: &dyn Fn(f64) -> ProfileDoc, stages: &[&str], hi_scale: f64) {
    let tol = Tol::witness();
    let eps = tol.eps();
    for pass in stages {
        // Single-threaded evidence run (`--test-threads 1`).
        geom_core::k_stats::identity_pass_set(pass);
        let (lo, hi, per) = ceiling(at, SymRules::shipped(), tol, 1.0e-1 * eps, hi_scale, 16);
        println!(
            "== {name}, passing [{pass}]: certifies x{lo:e}, refuses x{hi:e} ({per:.2}s/probe) [= {:.3e}·eps .. {:.3e}·eps] = {:.3e} of the real study",
            lo / eps,
            hi / eps,
            lo
        );
        if lo.is_finite() && hi.is_finite() {
            let doc = at(hi);
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let (shapes, refusal, counts) =
                replay(&doc, &ParamBox::of(&analyzed), SymRules::shipped(), tol);
            println!("   beyond it: {refusal:?}\n   {counts:?}");
            for s in shapes.iter().filter(|s| {
                matches!(
                    s.outcome,
                    ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
                )
            }) {
                println!(
                    "   [{:?}] {} enclosure {:?}",
                    s.outcome, s.predicate, s.enclosure
                );
            }
        }
    }
    geom_core::k_stats::identity_pass_set("");
}

/// **The plate's ceiling with its identity residuals passed, one at a
/// time** — evidence for review claim 3 and the "what is owed" of
/// `plate-ceiling-is-now-the-scaffold-pushforward`.
#[test]
#[ignore = "evidence-only: the plate's ceiling with identity residuals passed in turn"]
fn r2_evidence_plate_ceiling_with_identities_passed() {
    let tol = Tol::witness();
    let at = |s: f64| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0;
    staged_ceilings(
        "plate",
        &at,
        &[
            "",
            "carrier_matches_mapped_source",
            "carrier_matches_mapped_source,carrier_on_surface_2",
            "carrier_matches_mapped_source,carrier_on_surface_2,pcurve_map_residual",
            "carrier_matches_mapped_source,carrier_on_surface_2,pcurve_map_residual,witness_on_surface_2",
            "carrier_matches_mapped_source,carrier_on_surface_2,pcurve_map_residual,witness_on_surface_2,carrier_on_surface_1,witness_on_surface_1,carrier_endpoint_start,carrier_endpoint_end,witness_at_mid_parameter",
        ],
        2.0,
    );
}

/// **The pad's ceiling with `carrier_line_circle` passed** — claim 5:
/// whether discharging the fillet's declared tangency would move the
/// pad, and what stands behind `line_span`.
#[test]
#[ignore = "evidence-only: the pad's ceiling with the fillet tangency passed"]
fn r2_evidence_pad_ceiling_with_the_tangency_passed() {
    let tol = Tol::witness();
    let at = |s: f64| crate::m10_8_r2_probes_interval::pad(s, tol).0;
    staged_ceilings(
        "pad",
        &at,
        &[
            "",
            "carrier_line_circle",
            "line_span",
            "line_span,carrier_line_circle",
        ],
        2.0,
    );
}

/// **The enclosure of the plate's first refusal against the box scale**
/// — is it linear in the box (pure dependency widening) or does it carry
/// a constant? Printed at six scales, door on and off.
#[test]
#[ignore = "evidence-only: prints the plate's bounding enclosure against scale"]
fn r2_evidence_plate_enclosure_vs_scale() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let ceil = 7.787e2 * eps;
    for (label, rules) in [("door OFF", shut()), ("door ON ", SymRules::shipped())] {
        for frac in [0.125_f64, 0.25, 0.5, 0.99, 2.0, 4.0] {
            let doc = crate::m10_7_plate::plate(5.0e-5 * frac * ceil, 1.0e-5 * frac * ceil, tol).0;
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let (shapes, refusal, _) = replay(&doc, &ParamBox::of(&analyzed), rules, tol);
            let table = envelopes(&shapes);
            let pick = |p: &str| {
                table
                    .get(p)
                    .map_or("-".to_owned(), |w| format!("[{:.4e},{:.4e}]", w.lo, w.hi))
            };
            println!(
                "   plate {label} {frac:>5}x: endpoint_start {} | endpoint_end {} | mapped_source {} | refusal {}",
                pick("carrier_endpoint_start"),
                pick("carrier_endpoint_end"),
                pick("carrier_matches_mapped_source"),
                refusal
                    .as_deref()
                    .map_or("none", |r| if r.len() > 90 { &r[..90] } else { r })
            );
        }
    }
}

/// **Evidence** — why `frozen` moved with the door on R1's bracket at
/// 0.5 mm (a refusing drive): one whole-box replay each way, the first
/// refusal, the counts, and the per-outcome split.
#[test]
#[ignore = "evidence-only: the r1 bracket's frozen count door on and off"]
fn r2_evidence_r1_bracket_frozen_moves_with_the_door() {
    let tol = Tol::witness();
    let doc = crate::m10_7_r1_probes_interval::bracket_pub(0.5e-3, false, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    for (label, rules) in [("door OFF", shut()), ("door ON ", SymRules::shipped())] {
        let (shapes, refusal, counts) = replay(&doc, &ParamBox::of(&analyzed), rules, tol);
        let mut split: BTreeMap<String, usize> = BTreeMap::new();
        for s in &shapes {
            *split.entry(format!("{:?}", s.outcome)).or_default() += 1;
        }
        println!("== r1 bracket 0.5mm {label}: {counts:?}\n   refusal {refusal:?}\n   {split:?}");
        for s in shapes
            .iter()
            .filter(|s| matches!(s.outcome, ShapeOutcome::Registered))
            .take(3)
        {
            println!("   registered: {}", s.predicate);
        }
        let last: Vec<String> = shapes
            .iter()
            .rev()
            .take(4)
            .map(|s| format!("{} {:?} {:?}", s.predicate, s.outcome, s.enclosure))
            .collect();
        println!("   last decisions: {last:?}");
    }
}
