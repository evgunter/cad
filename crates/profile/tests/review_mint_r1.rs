//! Reviewer probes for PR 7488 (the direction mint asks the
//! expression), round 1. Each row prints what it measured; the
//! assertions state what the review found on the frozen head.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp,
    clippy::print_stderr
)]

use geom_core::sym::with_session_rules;
use geom_core::{
    Bounds, Decide, Expression, Interval, ParamSymbol, Point2, Real, Sym, SymBudget, SymRules, Tol,
    Vec2,
};
use profile::{
    ConstructedLoop, ConstructedProfile, Open, PathError, SegmentDirections, SegmentKind,
    SketchPlane, Start,
};

fn tol() -> Tol {
    Tol::witness()
}

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

fn eager<R>(f: impl FnOnce() -> R) -> R {
    with_session_rules(budget(), SymRules::shipped(), f).0
}

fn deferring<R>(f: impl FnOnce() -> R) -> R {
    geom_core::sym::deferred::with_deferred_session(
        budget(),
        SymRules::shipped(),
        geom_core::SymRetry::none(),
        f,
    )
    .0
}

fn pf(name: &str, v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::new(test_utils::symbol_id(name)), v)
}

fn pb(name: &str, c: f64, h: f64) -> Sym<Interval> {
    Sym::param(
        ParamSymbol::new(test_utils::symbol_id(name)),
        Interval::from_bounds(c - h, c + h),
    )
}

fn proved<T: Real>(x: T) -> bool {
    x.expression().and_then(Expression::proves_zero).is_some()
}

fn quad<T: Decide>(x1: T, x2: T, y: T) -> Result<ConstructedLoop<T>, PathError<T>> {
    let z = T::from_f64(0.0);
    Ok(Open
        .at(Point2::new(z, z))
        .line_to(Point2::new(x1, z), tol())?
        .line_to(Point2::new(x2, y), tol())?
        .line_to(Point2::new(z, y), tol())?
        .line_to(Start, tol())?
        .loop_)
}

fn outcome<T: Decide>(r: Result<ConstructedLoop<T>, PathError<T>>) -> String {
    match r {
        Ok(lp) => match ConstructedProfile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
            Ok(_) => "built and validated".to_owned(),
            Err(e) => format!("validate: {}", head(&format!("{e:?}"))),
        },
        Err(e) => format!("verb: {}", head(&format!("{e:?}"))),
    }
}

fn head(s: &str) -> String {
    s.chars().take(110).collect()
}

/// **Claim 3: an earlier ask changes a later answer.** The sum `x + w`
/// is minted before the session is installed, so the session's table
/// does not hold it when the first ask walks `(x + w) − w − x`; the
/// question's own memo keeps the frozen form under the node's content
/// id. The session then records `x + w` itself. The SAME node, asked
/// again, is still not proved; with no earlier ask it is. Two histories,
/// one final node table, two answers.
#[test]
fn probe_an_earlier_ask_fixes_the_answer_of_a_node_recorded_afterwards() {
    let run = |ask_first: bool| {
        let minted_before = pf("x", 0.1) + pf("w", 0.3);
        eager(|| {
            let (x, w) = (pf("x", 0.1), pf("w", 0.3));
            let hop = (minted_before - w) - x;
            let first = ask_first.then(|| proved(hop));
            let recorded = x + w;
            assert_eq!(
                recorded.expression(),
                minted_before.expression(),
                "one content id"
            );
            let hop_again = (recorded - w) - x;
            assert_eq!(hop_again.expression(), hop.expression(), "one node");
            (first, proved(hop_again))
        })
    };
    let (asked, unasked) = (run(true), run(false));
    eprintln!("asked first: {asked:?}; not asked first: {unasked:?}");
    assert_eq!(unasked, (None, true), "recorded before any ask: proved");
    assert_eq!(
        asked,
        (Some(false), false),
        "FINDING: the earlier ask's frozen form answers the later ask"
    );
}

/// The same history through the mint: the stored director of one
/// document's hop is the axis or the quotient by whether an earlier
/// hop asked about a node the session had not yet recorded.
#[test]
fn probe_the_mint_takes_two_branches_by_ask_history() {
    let run = |ask_first: bool| {
        let minted_before = pf("x", 0.1) + pf("w", 0.3);
        eager(|| {
            let (x, w) = (pf("x", 0.1), pf("w", 0.3));
            if ask_first {
                let _ = quad(x, minted_before - w, Sym::from_f64(1.0));
            }
            let recorded = x + w;
            let lp = quad(x, recorded - w, Sym::from_f64(1.0)).expect("builds");
            let vp = ConstructedProfile::new(SketchPlane::xy(), vec![lp])
                .validate(tol())
                .expect("validates");
            match vp.loops()[0].segments()[1].directions {
                SegmentDirections::Line(d) => [d.x.value, d.y.value],
                SegmentDirections::Arc { .. } => panic!("a line"),
            }
        })
    };
    let (asked, unasked) = (run(true), run(false));
    eprintln!("asked first: {asked:?}; not asked first: {unasked:?}");
    assert_eq!(unasked, [0.0, 1.0], "the axis");
    assert_ne!(asked, unasked, "FINDING: two branches for one document");
}

/// **Claim 7: what a chord with no length stores and who refuses it**,
/// in each lane: the hop `(x1, 0) → (x2, y)` where both components are
/// zero by identity, where one is proved and the other reads Zero, and
/// where both merely read zero.
#[test]
fn probe_a_chord_with_no_length_is_refused_by_name_in_every_lane() {
    let mut rows = Vec::new();
    rows.push(("f64 both zero", outcome(quad(1.0, 1.0, 0.0))));
    let i = |v: f64| Interval::from_bounds(v, v);
    rows.push(("Interval both zero", outcome(quad(i(1.0), i(1.0), i(0.0)))));
    for (name, session) in [("eager", true), ("deferring", false)] {
        let go = |f: &dyn Fn() -> String| if session { eager(f) } else { deferring(f) };
        rows.push((
            name,
            go(&|| {
                let w = pf("w", 1.0);
                format!(
                    "Sym<f64> both proved: {}",
                    outcome(quad(w, w, Sym::from_f64(0.0)))
                )
            }),
        ));
        rows.push((
            name,
            go(&|| {
                let w = pf("w", 1.0);
                format!(
                    "Sym<f64> x proved, y reads 0: {}",
                    outcome(quad(w, w, pf("h", 0.0)))
                )
            }),
        ));
        rows.push((
            name,
            go(&|| {
                format!(
                    "Sym<f64> both read 0: {}",
                    outcome(quad(pf("a", 1.0), pf("b", 1.0), pf("h", 0.0)))
                )
            }),
        ));
        rows.push((
            name,
            go(&|| {
                let w = pb("w", 1.0, 1.0e-11);
                format!(
                    "Sym<Interval> box both proved: {}",
                    outcome(quad(w, w, Sym::from_f64(0.0)))
                )
            }),
        ));
        rows.push((
            name,
            go(&|| {
                let w = pb("w", 1.0, 1.0e-11);
                format!(
                    "Sym<Interval> x proved, y point 0: {}",
                    outcome(quad(w, w, pb("h", 0.0, 0.0)))
                )
            }),
        ));
        rows.push((
            name,
            go(&|| {
                let w = pb("w", 1.0, 1.0e-11);
                format!(
                    "Sym<Interval> x proved, y within ±1e-12: {}",
                    outcome(quad(w, w, pb("h", 0.0, 1e-12)))
                )
            }),
        ));
        rows.push((
            name,
            go(&|| {
                let w = pb("w", 1.0, 1.0e-11);
                format!(
                    "Sym<Interval> x proved, y = 1e-12 (inside the band): {}",
                    outcome(quad(w, w, pb("h", 1e-12, 0.0)))
                )
            }),
        ));
    }
    rows.push(("f64 y = 1e-12", outcome(quad(1.0, 1.0, 1e-12))));
    for (lane, got) in &rows {
        eprintln!("{lane}: {got}");
    }
    for (lane, got) in &rows {
        assert_ne!(got.as_str(), "built and validated", "{lane}");
        assert!(!got.ends_with(": built and validated"), "{lane}: {got}");
    }
}

/// **Claim 9: a hop's recorded director against its stored segment,
/// where a fillet trims the hop.** Random convex quadrilaterals with a
/// fillet at every corner, both windings, at `f64`.
#[test]
fn probe_a_filleted_hops_director_is_its_stored_segments_direction() {
    let mut worst = 0.0_f64;
    let mut lines = 0;
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state >> 11) as f64 / (1u64 << 53) as f64
    };
    for case in 0..400 {
        let (a, b, c, d) = (
            2.0 + 2.0 * next(),
            2.0 + 2.0 * next(),
            0.5 * next(),
            0.5 * next(),
        );
        let mut pts = [(c, -d), (a, d), (a - c, b), (-d, b - c)];
        if case % 2 == 1 {
            pts.reverse();
        }
        let r = 0.05 + 0.3 * next();
        let t = tol();
        let at = |i: usize, f: f64| {
            let j = (i + 1) % 4;
            Point2::new(
                pts[i].0 + f * (pts[j].0 - pts[i].0),
                pts[i].1 + f * (pts[j].1 - pts[i].1),
            )
        };
        // Each side `i` is drawn as the hop from 0.6 to 0.7 of the way
        // along it; the fillet at the corner before it extends the
        // stored segment back to the tangent point.
        let built = (|| {
            Open.at(at(0, 0.0))
                .line_to(at(0, 0.7), t)?
                .fillet(r, t)?
                .at(at(1, 0.6), t)?
                .line_to(at(1, 0.7), t)?
                .fillet(r, t)?
                .at(at(2, 0.6), t)?
                .line_to(at(2, 0.7), t)?
                .fillet(r, t)?
                .at(at(3, 0.6), t)?
                .line_to(Start, t)
        })();
        let Ok(built) = built else {
            if case < 2 {
                eprintln!("case {case}: verb refused: {:?}", built.err());
            }
            continue;
        };
        let vp = ConstructedProfile::new(SketchPlane::xy(), vec![built.loop_])
            .validate(t)
            .expect("validates");
        for s in vp.loops()[0].segments() {
            if let (SegmentKind::Line, SegmentDirections::Line(dir)) = (s.kind, s.directions) {
                let q: Vec2<f64> = (s.end - s.start).normalize();
                let miss = (dir - q).norm();
                worst = worst.max(miss);
                lines += 1;
                assert!(
                    miss < 1e-9,
                    "case {case}: recorded {dir:?} against the stored segment's {q:?}"
                );
            }
        }
    }
    eprintln!("{lines} filleted hops; worst |recorded − stored quotient| = {worst:e}");
    assert!(lines > 1000);
}

/// **Claim 5: the stored director is bit-identical on every thread and
/// in every order**, at `Sym<Interval>` over a box.
#[test]
fn probe_the_stored_director_is_the_same_on_every_thread() {
    let build = |flip: bool| {
        eager(|| {
            let docs: [&dyn Fn() -> Vec<[(f64, f64); 2]>; 2] = [
                &|| {
                    directors(quad(
                        pb("W", 1.0, 1.0e-11),
                        pb("W", 1.0, 1.0e-11),
                        pb("H", 1.0, 1.0e-11),
                    ))
                },
                &|| {
                    let (x, w) = (pb("x", 0.1, 1.0e-11), pb("w", 0.3, 1.0e-11));
                    directors(quad(x, (x + w) - w, pb("H", 2.0, 1.0e-11)))
                },
            ];
            let mut out = if flip {
                vec![docs[1](), docs[0]()]
            } else {
                vec![docs[0](), docs[1]()]
            };
            if flip {
                out.reverse();
            }
            out
        })
    };
    let here = build(false);
    let there: Vec<_> = (0..4)
        .map(|k| std::thread::spawn(move || build(k % 2 == 1)))
        .map(|h| h.join().unwrap())
        .collect();
    for t in &there {
        assert_eq!(*t, here);
    }
}

fn directors<T: Decide + Bounds>(
    r: Result<ConstructedLoop<T>, PathError<T>>,
) -> Vec<[(f64, f64); 2]> {
    let vp = ConstructedProfile::new(SketchPlane::xy(), vec![r.expect("builds")])
        .validate(tol())
        .expect("validates");
    vp.loops()[0]
        .segments()
        .iter()
        .map(|s| match s.directions {
            SegmentDirections::Line(d) => [(d.x.lo(), d.x.hi()), (d.y.lo(), d.y.hi())],
            SegmentDirections::Arc { .. } => panic!("lines"),
        })
        .collect()
}

/// The filleted quadrilateral of the trim probe, at any scalar.
fn filleted<T: profile::ArcCarrierScalar>(
    pts: [(f64, f64); 4],
    r: f64,
) -> Result<ConstructedLoop<T>, PathError<T>> {
    let t = tol();
    let at = |i: usize, f: f64| {
        let j = (i + 1) % 4;
        Point2::new(
            T::from_f64(pts[i].0 + f * (pts[j].0 - pts[i].0)),
            T::from_f64(pts[i].1 + f * (pts[j].1 - pts[i].1)),
        )
    };
    Ok(Open
        .at(at(0, 0.0))
        .line_to(at(0, 0.7), t)?
        .fillet(T::from_f64(r), t)?
        .at(at(1, 0.6), t)?
        .line_to(at(1, 0.7), t)?
        .fillet(T::from_f64(r), t)?
        .at(at(2, 0.6), t)?
        .line_to(at(2, 0.7), t)?
        .fillet(T::from_f64(r), t)?
        .at(at(3, 0.6), t)?
        .line_to(Start, t)?
        .loop_)
}

/// **Claim 9, the reader the body names and does not attack**: a
/// symbolic run's `f64` value channel against the `f64` run of the same
/// verbs. `value_channel` mints a line's direction from the STORED
/// ends where the symbolic run's director is not a point; the `f64`
/// run records the verb's director, from the AUTHORED chord. Where a
/// fillet moved the hop's start those are two computations.
#[test]
fn probe_the_value_channel_of_a_symbolic_run_against_the_f64_run() {
    let mut state = 0x2545_f491_4f6c_dd1d_u64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state >> 11) as f64 / (1u64 << 53) as f64
    };
    let (mut lines, mut differ, mut worst) = (0, 0, 0.0_f64);
    let mut first = None;
    for case in 0..30 {
        let (a, b, c, d) = (
            2.0 + 2.0 * next(),
            2.0 + 2.0 * next(),
            0.5 * next(),
            0.5 * next(),
        );
        let pts = [(c, -d), (a, d), (a - c, b), (-d, b - c)];
        let r = 0.05 + 0.3 * next();
        let Ok(plain) = filleted::<f64>(pts, r) else {
            continue;
        };
        let plain = ConstructedProfile::new(SketchPlane::xy(), vec![plain])
            .validate(tol())
            .expect("f64 validates");
        let channel = eager(|| {
            let lp = filleted::<Sym<f64>>(pts, r).ok()?;
            let vp = ConstructedProfile::new(SketchPlane::xy(), vec![lp])
                .validate(tol())
                .ok()?;
            vp.value_channel().ok()
        });
        let Some(channel) = channel else {
            continue;
        };
        for (k, (p, q)) in plain.loops()[0]
            .segments()
            .iter()
            .zip(channel.loops()[0].segments())
            .enumerate()
        {
            if let (SegmentDirections::Line(dp), SegmentDirections::Line(dq)) =
                (p.directions, q.directions)
            {
                lines += 1;
                if [dp.x.to_bits(), dp.y.to_bits()] != [dq.x.to_bits(), dq.y.to_bits()] {
                    differ += 1;
                    worst = worst.max((dp - dq).norm());
                    first.get_or_insert((case, k, dp, dq));
                }
            }
        }
    }
    eprintln!(
        "{lines} line directions compared; {differ} differ in bits between the f64 run and the \
         symbolic run's value channel; worst {worst:e}; first: {first:?}"
    );
    assert!(lines > 100, "the probe compared something");
    assert_eq!(differ, 0, "FINDING if red: two f64 forms of one document");
}
